#![allow(
    clippy::disallowed_methods,
    reason = "The test harness owns its Tokio runtimes."
)]

use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use kalshi_api::{Environment, RetryMiddleware};
use reqwest::{Method, Request, Response};
use reqwest_middleware::{ClientBuilder, Middleware, Next, Result};

struct Responses {
    statuses: Vec<u16>,
    calls: Arc<Mutex<Vec<Method>>>,
}

#[async_trait]
impl Middleware for Responses {
    async fn handle(
        &self,
        request: Request,
        _: &mut http::Extensions,
        _: Next<'_>,
    ) -> Result<Response> {
        let mut calls = self.calls.lock().unwrap();
        let status = self.statuses[calls.len().min(self.statuses.len() - 1)];
        calls.push(request.method().clone());
        Ok(http::Response::builder()
            .status(status)
            .header("content-type", "application/json")
            .header("x-fixture", "synthetic-response")
            .body(reqwest::Body::from(
                r#"{"exchange_active":true,"trading_active":false}"#,
            ))
            .unwrap()
            .into())
    }
}

#[tokio::test]
async fn opt_in_retries_use_python_method_and_status_rules() {
    for (method, statuses, attempts, expected_calls) in [
        (Method::GET, vec![500, 503, 200], 3, 3),
        (Method::DELETE, vec![500, 204], 2, 2),
        (Method::PUT, vec![500, 200], 2, 2),
        (Method::POST, vec![500, 200], 3, 1),
        (Method::PATCH, vec![500, 200], 3, 1),
        (Method::GET, vec![429, 200], 3, 1),
        (Method::GET, vec![500], 0, 1),
    ] {
        let calls = Arc::new(Mutex::new(Vec::new()));
        let client = ClientBuilder::new(reqwest::Client::new())
            .with(RetryMiddleware::new(attempts))
            .with(Responses {
                statuses,
                calls: Arc::clone(&calls),
            })
            .build();
        let _ = client
            .request(method, "https://example.invalid/")
            .send()
            .await
            .unwrap();
        assert_eq!(calls.lock().unwrap().len(), expected_calls);
    }
}

#[test]
fn explicit_hosts_match_the_official_environment_urls() {
    assert_eq!(
        Environment::Prod.base_url(),
        "https://external-api.kalshi.com/trade-api/v2"
    );
    assert_eq!(
        Environment::ProdShared.base_url(),
        "https://api.elections.kalshi.com/trade-api/v2"
    );
    assert_eq!(
        Environment::Demo.base_url(),
        "https://external-api.demo.kalshi.co/trade-api/v2"
    );
    assert_eq!(
        Environment::DemoShared.base_url(),
        "https://demo-api.kalshi.co/trade-api/v2"
    );
}

#[tokio::test]
async fn response_variants_keep_headers_and_allow_unread_error_responses() {
    use kalshi_api_openapi::apis::{
        configuration::Configuration,
        exchange_api::{
            get_exchange_status_with_http_info, get_exchange_status_without_preload_content,
        },
    };
    let configuration = Configuration {
        base_path: "https://example.invalid/trade-api/v2".to_owned(),
        client: ClientBuilder::new(reqwest::Client::new())
            .with(Responses {
                statuses: vec![200, 503],
                calls: Arc::new(Mutex::new(Vec::new())),
            })
            .build(),
        ..Configuration::new(
            std::time::Duration::from_secs(5),
            std::time::Duration::from_secs(30),
        )
        .unwrap()
    };
    let decoded = get_exchange_status_with_http_info(&configuration)
        .await
        .unwrap();
    assert_eq!(decoded.status, 200);
    assert_eq!(decoded.headers["x-fixture"], "synthetic-response");
    assert!(decoded.data.exchange_active);
    assert!(!decoded.data.trading_active);
    assert_eq!(
        decoded.raw_data,
        br#"{"exchange_active":true,"trading_active":false}"#
    );
    let unread = get_exchange_status_without_preload_content(&configuration)
        .await
        .unwrap();
    assert_eq!(unread.status(), 503);
    assert_eq!(unread.headers()["x-fixture"], "synthetic-response");
    assert_eq!(
        unread.bytes().await.unwrap().as_ref(),
        decoded.raw_data.as_slice()
    );
}

#[tokio::test]
async fn python_http_exception_classes_keep_their_status_in_rust() {
    use kalshi_api_openapi::apis::{
        configuration::Configuration, exchange_api::get_exchange_status, Error,
    };
    for status in [400, 401, 403, 404, 429, 500, 503] {
        let calls = Arc::new(Mutex::new(Vec::new()));
        let configuration = Configuration {
            base_path: "https://example.invalid/trade-api/v2".to_owned(),
            client: ClientBuilder::new(reqwest::Client::new())
                .with(Responses {
                    statuses: vec![status],
                    calls,
                })
                .build(),
            ..Configuration::new(
                std::time::Duration::from_secs(5),
                std::time::Duration::from_secs(30),
            )
            .unwrap()
        };
        match get_exchange_status(&configuration).await {
            Err(Error::ResponseError(error)) => assert_eq!(error.status.as_u16(), status),
            other => panic!("status {status} lost: {other:?}"),
        }
    }
}
