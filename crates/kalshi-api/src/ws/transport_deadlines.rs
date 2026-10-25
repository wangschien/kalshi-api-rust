#![allow(clippy::disallowed_methods)]
use super::*;
use crate::ws::UnsubscribeParams;
use futures_util::FutureExt;

const DEADLINE: Duration = Duration::from_millis(20);
const OBSERVE_BY: Duration = Duration::from_millis(80);
const DELTA: &str = r#"{"type":"orderbook_delta","sid":7,"seq":41,"msg":{"market_ticker":"SYNTHETIC","market_id":"synthetic","clock":1,"price_dollars":"0.1200","delta_fp":"2.50","side":"yes"}}"#;

#[tokio::test(start_paused = true)]
async fn stalled_sends_end_session_with_ready_update_and_queued_frame() {
    for trigger in ["command", "ping", "pong", "close"] {
        for queued_frame in [false, true] {
            let (frame_tx, frame_rx) = mpsc::channel::<Message>(1);
            let read = Box::pin(futures_util::stream::unfold(
                frame_rx,
                |mut rx| async move { rx.recv().await.map(|message| (Ok(message), rx)) },
            ));
            let blocked = Arc::new(tokio::sync::Notify::new());
            let notified = Arc::clone(&blocked);
            let writes = Arc::new(AtomicU64::new(0));
            let started = Arc::clone(&writes);
            let write = Box::pin(futures_util::sink::unfold((), move |(), _: Message| {
                started.fetch_add(1, Ordering::Relaxed);
                notified.notify_one();
                futures_util::future::pending::<Result<(), tokio_tungstenite::tungstenite::Error>>()
            }));
            let (outbound, commands) = mpsc::unbounded_channel();
            let (event_tx, inbound) = mpsc::unbounded_channel();
            if queued_frame {
                event_tx
                    .send(WsEvent::Frame(Box::new(parse_incoming(DELTA).unwrap())))
                    .unwrap();
            }
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
                    send_timeout: Some(DEADLINE),
                },
            ));
            let mut session = WsSession {
                outbound,
                inbound,
                lost_rx,
                stashed_lost: None,
                lost_taken: false,
                finished: false,
                last_ping_rx_ns,
                task,
            };
            match trigger {
                "command" => session
                    .send(ClientCommand::Unsubscribe {
                        id: 1,
                        params: UnsubscribeParams { sids: vec![7] },
                    })
                    .await
                    .unwrap(),
                "ping" => session.ping(vec![1]).await.unwrap(),
                "pong" => frame_tx.send(Message::Ping(vec![1].into())).await.unwrap(),
                "close" => frame_tx.send(Message::Close(None)).await.unwrap(),
                _ => unreachable!(),
            }
            tokio::time::timeout(OBSERVE_BY, blocked.notified())
                .await
                .unwrap();
            frame_tx.send(Message::Text(DELTA.into())).await.unwrap();
            if queued_frame {
                // Leave the frame unread until the write expires and the task exits.
                tokio::time::timeout(OBSERVE_BY, &mut session.task)
                    .await
                    .expect("an unread frame must not block termination")
                    .unwrap();
                assert!(matches!(session.recv().await, Some(WsEvent::Frame(_))));
            }
            let event = tokio::time::timeout(OBSERVE_BY, session.recv())
                .await
                .expect("stalled send must expose termination before the observation bound");
            assert_eq!(
                event,
                Some(WsEvent::SendTimedOut { timeout: DEADLINE }),
                "{trigger}, queued_frame={queued_frame}"
            );
            assert!(session.recv().await.is_none());
            if !queued_frame {
                tokio::time::timeout(OBSERVE_BY, &mut session.task)
                    .await
                    .unwrap()
                    .unwrap();
            }
            assert!(matches!(session.ping(vec![]).await, Err(WsError::Closed)));
            assert_eq!(
                writes.load(Ordering::Relaxed),
                1,
                "no resend or extra close after expiry"
            );
        }
    }
}

fn harness(
    write: impl SinkExt<Message, Error = tokio_tungstenite::tungstenite::Error> + Unpin + Send + 'static,
    send_timeout: Option<Duration>,
) -> (WsSession, mpsc::Sender<Message>) {
    let (frame_tx, frame_rx) = mpsc::channel::<Message>(1);
    let read = Box::pin(futures_util::stream::unfold(
        frame_rx,
        |mut rx| async move { rx.recv().await.map(|message| (Ok(message), rx)) },
    ));
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
            send_timeout,
        },
    ));
    let session = WsSession {
        outbound,
        inbound,
        lost_rx,
        stashed_lost: None,
        lost_taken: false,
        finished: false,
        last_ping_rx_ns,
        task,
    };
    (session, frame_tx)
}

/// Longer than any deadline a caller could mean; paused time skips it at once.
const HORIZON: Duration = Duration::from_secs(365 * 24 * 60 * 60);

#[tokio::test(start_paused = true)]
async fn unset_send_timeout_never_expires_a_stalled_write() {
    for trigger in ["command", "ping", "pong", "close"] {
        let blocked = Arc::new(tokio::sync::Notify::new());
        let notified = Arc::clone(&blocked);
        let writes = Arc::new(AtomicU64::new(0));
        let started = Arc::clone(&writes);
        let write = Box::pin(futures_util::sink::unfold((), move |(), _: Message| {
            started.fetch_add(1, Ordering::Relaxed);
            notified.notify_one();
            futures_util::future::pending::<Result<(), tokio_tungstenite::tungstenite::Error>>()
        }));
        let (mut session, frame_tx) = harness(write, None);
        match trigger {
            "command" => session
                .send(ClientCommand::Unsubscribe {
                    id: 1,
                    params: UnsubscribeParams { sids: vec![7] },
                })
                .await
                .unwrap(),
            "ping" => session.ping(vec![1]).await.unwrap(),
            "pong" => frame_tx.send(Message::Ping(vec![1].into())).await.unwrap(),
            "close" => frame_tx.send(Message::Close(None)).await.unwrap(),
            _ => unreachable!(),
        }
        tokio::time::timeout(Duration::from_secs(1), blocked.notified())
            .await
            .expect("the stalled write never started");
        // Any local deadline shorter than the horizon would fire while this
        // sleep advances paused time, and recv would return SendTimedOut.
        tokio::time::sleep(HORIZON).await;
        assert!(
            session.recv().now_or_never().is_none(),
            "{trigger}: an unset send_timeout ended the session"
        );
        assert!(
            !session.task.is_finished(),
            "{trigger}: the I/O task stopped"
        );
        assert_eq!(writes.load(Ordering::Relaxed), 1, "{trigger}: no resend");
    }
}

#[tokio::test(start_paused = true)]
async fn unset_send_timeout_awaits_a_slow_write_to_completion() {
    let writes = Arc::new(AtomicU64::new(0));
    let started = Arc::clone(&writes);
    let write = Box::pin(futures_util::sink::unfold((), move |(), _: Message| {
        let first = started.fetch_add(1, Ordering::Relaxed) == 0;
        async move {
            if first {
                tokio::time::sleep(HORIZON).await;
            }
            Ok::<(), tokio_tungstenite::tungstenite::Error>(())
        }
    }));
    let (mut session, frame_tx) = harness(write, None);
    session.ping(vec![1]).await.unwrap();
    session.ping(vec![2]).await.unwrap();
    tokio::time::sleep(HORIZON * 2).await;
    assert_eq!(
        writes.load(Ordering::Relaxed),
        2,
        "the slow write was not completed and followed"
    );
    frame_tx.send(Message::Text(DELTA.into())).await.unwrap();
    assert!(matches!(session.recv().await, Some(WsEvent::Frame(_))));
}
