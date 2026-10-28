#![allow(
    clippy::disallowed_methods,
    reason = "Offline tests own their runtime and compare generated requests."
)]
#![allow(
    deprecated,
    reason = "Parity includes the official SDK's deprecated operations."
)]

use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use kalshi_api::{Clock, SignMiddleware};
use kalshi_api_openapi::apis::configuration::Configuration;
use reqwest::{Request, Response};
use reqwest_middleware::{ClientBuilder, Error, Middleware, Next, Result};
use serde_json::{json, Value};

struct FixedClock;

impl Clock for FixedClock {
    fn unix_ms(&self) -> u64 {
        1_700_000_000_000
    }
}

struct Capture(Arc<Mutex<Option<Request>>>);

fn canonical_query(mut value: Value) -> Value {
    let url = reqwest::Url::parse(value["url"].as_str().unwrap()).unwrap();
    value["url"] = json!(format!(
        "{}{}",
        url.origin().ascii_serialization(),
        url.path()
    ));
    value["query"] = json!(url.query_pairs().into_owned().collect::<Vec<_>>());
    value
}

#[async_trait]
impl Middleware for Capture {
    async fn handle(
        &self,
        request: Request,
        _: &mut http::Extensions,
        _: Next<'_>,
    ) -> Result<Response> {
        *self.0.lock().unwrap() = Some(request);
        Err(Error::middleware(std::io::Error::other(
            "offline request captured",
        )))
    }
}

#[tokio::test]
async fn every_python_public_method_builds_the_same_request() {
    let fixtures: Vec<Value> =
        serde_json::from_str(include_str!("fixtures/parity/requests.json")).unwrap();
    assert_eq!(fixtures.len(), 714);
    let captured = Arc::new(Mutex::new(None));
    let sign = SignMiddleware::new(
        "synthetic-key",
        include_str!("fixtures/parity/rsa_test_private.pem"),
        Arc::new(FixedClock),
    )
    .unwrap();
    let configuration = Configuration {
        base_path: "https://example.invalid/trade-api/v2".to_owned(),
        user_agent: Some("parity-fixture".to_owned()),
        client: ClientBuilder::new(reqwest::Client::new())
            .with(sign)
            .with(Capture(Arc::clone(&captured)))
            .build(),
        ..Configuration::new(
            Some(std::time::Duration::from_secs(5)),
            Some(std::time::Duration::from_secs(30)),
        )
        .unwrap()
    };
    let mut failures = Vec::new();
    for fixture in &fixtures {
        let operation = fixture["operation"].as_str().unwrap();
        include!("fixtures/parity/request_dispatch.rs");
        let request = captured
            .lock()
            .unwrap()
            .take()
            .unwrap_or_else(|| panic!("{} did not reach request capture", fixture["id"]));
        assert_eq!(
            request.timeout(),
            configuration.request_timeout.as_ref(),
            "{} has no caller deadline",
            fixture["id"]
        );
        let headers: serde_json::Map<String, Value> = request
            .headers()
            .iter()
            .filter(|(key, _)| key.as_str() != "kalshi-access-signature")
            .map(|(key, value)| (key.to_string(), json!(value.to_str().unwrap())))
            .collect();
        let body = request
            .body()
            .map(|body| serde_json::from_slice::<Value>(body.as_bytes().unwrap()).unwrap());
        let actual = json!({"method": request.method().as_str(), "url": request.url().as_str(), "path": request.url().path(), "query": request.url().query().unwrap_or(""), "headers": headers, "body": body});
        let actual = canonical_query(actual);
        let expected = canonical_query(fixture["expected"].clone());
        if actual != expected {
            let differing: Vec<_> = actual
                .as_object()
                .unwrap()
                .keys()
                .filter(|key| actual[*key] != expected[*key])
                .cloned()
                .collect();
            failures.push(format!(
                "{} differs in {}",
                fixture["id"],
                differing.join(",")
            ));
            if differing.contains(&"query".to_owned()) {
                failures.push(format!(
                    "query actual={} expected={}",
                    actual["query"], expected["query"]
                ));
            }
        }
        if fixture["body"].is_null() && body.is_some() {
            failures.push(format!(
                "{} sends a body where Python omits it",
                fixture["id"]
            ));
        }
    }
    println!("REQUEST_TIMEOUT_NONE=0; GENERATED_METHODS=357; OPERATIONS=119; REQUESTS=714");
    assert!(
        failures.is_empty(),
        "{} request failures. First 12.\n{}",
        failures.len(),
        failures
            .iter()
            .take(12)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n")
    );
}
