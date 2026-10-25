#![allow(clippy::disallowed_methods)]
use super::*;
use crate::ws::UnsubscribeParams;
use futures_util::FutureExt;

const DELTA: &str = r#"{"type":"orderbook_delta","sid":7,"seq":41,"msg":{"market_ticker":"SYNTHETIC","market_id":"synthetic","clock":1,"price_dollars":"0.1200","delta_fp":"2.50","side":"yes"}}"#;

/// Far more than any small fixed queue depth.
const BURST: usize = 4096;

fn session_over(
    read: impl StreamExt<Item = Result<Message, tokio_tungstenite::tungstenite::Error>>
        + Unpin
        + Send
        + 'static,
    write: impl SinkExt<Message, Error = tokio_tungstenite::tungstenite::Error> + Unpin + Send + 'static,
) -> WsSession {
    let (outbound, commands) = mpsc::unbounded_channel();
    let (event_tx, inbound) = mpsc::unbounded_channel();
    let (lost_tx, lost_rx) = oneshot::channel();
    let last_ping_rx_ns = Arc::new(AtomicU64::new(0));
    let task = tokio::spawn(run_io(
        read,
        write,
        commands,
        event_tx,
        lost_tx,
        IoShared {
            last_ping_rx_ns: Arc::clone(&last_ping_rx_ns),
            clock: Arc::new(crate::sign::SystemClock),
            send_timeout: None,
        },
    ));
    WsSession {
        outbound,
        inbound,
        lost_rx,
        stashed_lost: None,
        lost_taken: false,
        finished: false,
        last_ping_rx_ns,
        task,
    }
}

#[tokio::test]
async fn unread_frames_are_all_kept_and_the_socket_stays_open() {
    let (frame_tx, frame_rx) = mpsc::unbounded_channel::<Message>();
    for _ in 0..BURST {
        frame_tx.send(Message::Text(DELTA.into())).unwrap();
    }
    let read = Box::pin(futures_util::stream::unfold(
        frame_rx,
        |mut rx| async move { rx.recv().await.map(|message| (Ok(message), rx)) },
    ));
    let (written_tx, mut written_rx) = mpsc::unbounded_channel::<Message>();
    let write = Box::pin(futures_util::sink::unfold(
        written_tx,
        |tx, message: Message| async move {
            let _ = tx.send(message);
            Ok::<_, tokio_tungstenite::tungstenite::Error>(tx)
        },
    ));
    let mut session = session_over(read, write);
    // The Pong is written only after every earlier frame was handled, while
    // the caller has read none of them.
    frame_tx
        .send(Message::Ping(b"after-burst".to_vec().into()))
        .unwrap();
    // The test bounds its own wait so a broken socket task fails instead of hanging.
    let pong = tokio::time::timeout(Duration::from_secs(10), written_rx.recv())
        .await
        .expect("no Pong within the test's own bound")
        .expect("socket task ended before the Pong");
    assert_eq!(pong, Message::Pong(b"after-burst".to_vec().into()));
    assert!(
        !session.task.is_finished(),
        "unread frames ended the socket task"
    );
    for index in 0..BURST {
        match session.recv().now_or_never().flatten() {
            Some(WsEvent::Frame(_)) => {}
            other => panic!("frame {index} of {BURST} was not delivered: {other:?}"),
        }
    }
    assert!(
        session.recv().now_or_never().is_none(),
        "unexpected extra event"
    );
}

#[tokio::test]
async fn send_never_waits_for_queue_space() {
    let read = Box::pin(futures_util::stream::pending::<
        Result<Message, tokio_tungstenite::tungstenite::Error>,
    >());
    let write = Box::pin(futures_util::sink::unfold((), |(), _: Message| {
        futures_util::future::pending::<Result<(), tokio_tungstenite::tungstenite::Error>>()
    }));
    let session = session_over(read, write);
    for id in 0..BURST as u64 {
        let command = ClientCommand::Unsubscribe {
            id,
            params: UnsubscribeParams { sids: vec![7] },
        };
        match session.send(command).now_or_never() {
            Some(Ok(())) => {}
            other => panic!("send {id} of {BURST} waited or failed: {other:?}"),
        }
        match session.ping(vec![1]).now_or_never() {
            Some(Ok(())) => {}
            other => panic!("ping {id} of {BURST} waited or failed: {other:?}"),
        }
    }
}
