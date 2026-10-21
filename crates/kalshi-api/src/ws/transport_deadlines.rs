#![allow(clippy::disallowed_methods)]
use super::*;
use crate::ws::UnsubscribeParams;

const DEADLINE: Duration = Duration::from_millis(20);
const OBSERVE_BY: Duration = Duration::from_millis(80);
const DELTA: &str = r#"{"type":"orderbook_delta","sid":7,"seq":41,"msg":{"market_ticker":"SYNTHETIC","market_id":"synthetic","clock":1,"price_dollars":"0.1200","delta_fp":"2.50","side":"yes"}}"#;

#[tokio::test(start_paused = true)]
async fn stalled_sends_end_session_with_ready_update_and_full_queue() {
    for trigger in ["command", "ping", "pong", "close"] {
        for full_queue in [false, true] {
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
            let (outbound, commands) = mpsc::channel(1);
            let (event_tx, inbound) = mpsc::channel(1);
            if full_queue {
                event_tx
                    .try_send(WsEvent::Frame(Box::new(parse_incoming(DELTA).unwrap())))
                    .unwrap();
            }
            let (lost_tx, lost_rx) = oneshot::channel();
            let last_ping_rx_ns = Arc::new(AtomicU64::new(0));
            let dropped_frames = Arc::new(AtomicU64::new(0));
            let task = tokio::spawn(run_io(
                read,
                write,
                commands,
                event_tx,
                lost_tx,
                IoShared {
                    last_ping_rx_ns: Arc::clone(&last_ping_rx_ns),
                    dropped_frames: Arc::clone(&dropped_frames),
                    clock: Arc::new(crate::sign::SystemClock),
                    send_timeout: DEADLINE,
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
                dropped_frames,
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
            if full_queue {
                // Keep the data channel full until the write expires and the task exits.
                tokio::time::timeout(OBSERVE_BY, &mut session.task)
                    .await
                    .expect("a full application queue must not block termination")
                    .unwrap();
                assert!(matches!(session.recv().await, Some(WsEvent::Frame(_))));
            }
            let event = tokio::time::timeout(OBSERVE_BY, session.recv())
                .await
                .expect("stalled send must expose termination before the observation bound");
            assert_eq!(
                event,
                Some(WsEvent::SendTimedOut { timeout: DEADLINE }),
                "{trigger}, full_queue={full_queue}"
            );
            assert!(session.recv().await.is_none());
            if !full_queue {
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
