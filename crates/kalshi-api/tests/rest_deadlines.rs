#![allow(clippy::disallowed_methods)]
use kalshi_api_openapi::apis::{configuration::Configuration, exchange_api, Error};
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::test]
async fn generated_deadline_bounds_headers_and_body_with_a_replaced_client() {
    for body_started in [false, true] {
        for variant in ["data", "http_info", "raw"] {
            let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
            let address = listener.local_addr().unwrap();
            let server = tokio::spawn(async move {
                let (mut socket, _) = listener.accept().await.unwrap();
                let mut request = Vec::new();
                let mut byte = [0];
                while !request.ends_with(b"\r\n\r\n") {
                    socket.read_exact(&mut byte).await.unwrap();
                    request.push(byte[0]);
                }
                if body_started {
                    socket.write_all(b"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: 100\r\n\r\n{").await.unwrap();
                }
                std::future::pending::<()>().await;
            });
            let mut config =
                Configuration::new(Duration::from_millis(20), Duration::from_millis(50)).unwrap();
            config.base_path = format!("http://{address}");
            // Replacing middleware must not remove generated request deadlines.
            config.client = reqwest_middleware::ClientBuilder::new(reqwest::Client::new()).build();
            let result = tokio::time::timeout(Duration::from_secs(1), async {
                match variant {
                    "data" => exchange_api::get_exchange_status(&config).await.map(|_| ()),
                    "http_info" => exchange_api::get_exchange_status_with_http_info(&config)
                        .await
                        .map(|_| ()),
                    "raw" => {
                        match exchange_api::get_exchange_status_without_preload_content(&config)
                            .await
                        {
                            Ok(response) => {
                                response.bytes().await.map(|_| ()).map_err(Error::Reqwest)
                            }
                            Err(error) => Err(error),
                        }
                    }
                    _ => unreachable!(),
                }
            })
            .await;
            server.abort();
            assert!(server.await.unwrap_err().is_cancelled());
            let error = result
                .expect("generated request must finish before the observer timeout")
                .unwrap_err();
            let timed_out = match error {
                Error::Reqwest(error) => error.is_timeout(),
                Error::ReqwestMiddleware(reqwest_middleware::Error::Reqwest(error)) => {
                    error.is_timeout()
                }
                other => panic!("unexpected error: {other}"),
            };
            assert!(timed_out, "{variant}, body_started={body_started}");
        }
    }
}
