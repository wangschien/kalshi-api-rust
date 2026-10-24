use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use bytes::Bytes;
use futures_util::StreamExt;
use http::Response as HttpResponse;
use reqwest::{Request, Response};
use reqwest_middleware::{Error, Middleware, Next, Result};
use tokio::task_local;

use crate::error::{BodyErrorKind, MiddlewareFault};

task_local! {
    static CAPTURE: Arc<Mutex<CaptureSlot>>;
}

#[derive(Debug, Clone)]
pub struct CaptureSlot {
    pub operation: String,
    pub status: Option<u16>,
    pub content_type: Option<String>,
    pub date: Option<String>,
    pub prefix: Vec<u8>,
    pub full: Vec<u8>,
    pub truncated: bool,
    pub total: u64,
    pub body_error: Option<BodyErrorKind>,
    pub too_large: bool,
}

impl CaptureSlot {
    fn new(operation: impl Into<String>) -> Self {
        Self {
            operation: operation.into(),
            status: None,
            content_type: None,
            date: None,
            prefix: Vec::new(),
            full: Vec::new(),
            truncated: false,
            total: 0,
            body_error: None,
            too_large: false,
        }
    }
}

pub struct CaptureMiddleware {
    max_body_bytes: Option<usize>,
    max_capture_bytes: usize,
    pass_through: Arc<AtomicU64>,
}

impl CaptureMiddleware {
    pub fn new(
        max_body_bytes: Option<usize>,
        max_capture_bytes: usize,
        pass_through: Arc<AtomicU64>,
    ) -> std::result::Result<Self, &'static str> {
        if max_body_bytes.is_some_and(|max| max_capture_bytes > max) {
            return Err("max_capture_bytes must be <= max_body_bytes");
        }
        Ok(Self {
            max_body_bytes,
            max_capture_bytes,
            pass_through,
        })
    }
}

pub async fn scope<T, F>(operation: &str, fut: F) -> (T, CaptureSlot)
where
    F: std::future::Future<Output = T>,
{
    let slot = Arc::new(Mutex::new(CaptureSlot::new(operation)));
    let result = CAPTURE.scope(Arc::clone(&slot), fut).await;
    let snapshot = slot
        .lock()
        .unwrap_or_else(|poison| poison.into_inner())
        .clone();
    (result, snapshot)
}

#[async_trait]
impl Middleware for CaptureMiddleware {
    async fn handle(
        &self,
        req: Request,
        extensions: &mut http::Extensions,
        next: Next<'_>,
    ) -> Result<Response> {
        if CAPTURE.try_with(|_| ()).is_err() {
            self.pass_through.fetch_add(1, Ordering::Relaxed);
            return next.run(req, extensions).await;
        }
        let response = match next.run(req, extensions).await {
            Ok(response) => response,
            Err(err) => return Err(err),
        };
        self.read_response(response).await
    }
}

impl CaptureMiddleware {
    async fn read_response(&self, response: Response) -> Result<Response> {
        let status = response.status();
        let headers = response.headers().clone();
        let version = response.version();
        let content_type = header_string(&headers, "content-type");
        let date = header_string(&headers, "date");
        write_slot(|slot| {
            slot.status = Some(status.as_u16());
            slot.content_type = content_type;
            slot.date = date;
        });

        let mut stream = response.bytes_stream();
        let mut full = Vec::new();
        let mut prefix = Vec::new();
        let mut total: u64 = 0;
        while let Some(chunk) = stream.next().await {
            let chunk = match chunk {
                Ok(chunk) => chunk,
                Err(err) => {
                    let kind = if err.is_timeout() {
                        BodyErrorKind::Timeout
                    } else if err.is_connect() || err.is_request() {
                        BodyErrorKind::Reset
                    } else {
                        BodyErrorKind::Other
                    };
                    write_slot(|slot| {
                        slot.prefix = prefix.clone();
                        slot.full = full.clone();
                        slot.total = total;
                        slot.body_error = Some(kind);
                    });
                    return Err(Error::middleware(MiddlewareFault(err.to_string())));
                }
            };
            total = total.saturating_add(chunk.len() as u64);
            if self.max_body_bytes.is_some_and(|max| total > max as u64) {
                take_prefix(&mut prefix, &chunk, self.max_capture_bytes);
                write_slot(|slot| {
                    slot.prefix = prefix;
                    slot.total = total;
                    slot.too_large = true;
                    slot.truncated = true;
                });
                return Err(Error::middleware(MiddlewareFault(
                    "response body exceeds max_body_bytes".to_owned(),
                )));
            }
            take_prefix(&mut prefix, &chunk, self.max_capture_bytes);
            full.extend_from_slice(&chunk);
        }
        let truncated = total > self.max_capture_bytes as u64;
        write_slot(|slot| {
            slot.prefix = prefix;
            slot.full = full.clone();
            slot.total = total;
            slot.truncated = truncated;
        });
        rebuild(status, version, &headers, Bytes::from(full))
    }
}

fn take_prefix(prefix: &mut Vec<u8>, chunk: &[u8], max_capture_bytes: usize) {
    if prefix.len() >= max_capture_bytes {
        return;
    }
    let room = max_capture_bytes - prefix.len();
    let take = room.min(chunk.len());
    prefix.extend_from_slice(&chunk[..take]);
}

fn write_slot(update: impl FnOnce(&mut CaptureSlot)) {
    let _ = CAPTURE.try_with(|slot| {
        let mut guard = slot.lock().unwrap_or_else(|poison| poison.into_inner());
        update(&mut guard);
    });
}

fn header_string(headers: &reqwest::header::HeaderMap, name: &str) -> Option<String> {
    headers
        .get(name)
        .and_then(|value| value.to_str().ok())
        .map(str::to_owned)
}

fn rebuild(
    status: reqwest::StatusCode,
    version: reqwest::Version,
    headers: &reqwest::header::HeaderMap,
    body: Bytes,
) -> Result<Response> {
    let mut builder = HttpResponse::builder().status(status).version(version);
    for (name, value) in headers.iter() {
        builder = builder.header(name, value);
    }
    let response = builder
        .body(reqwest::Body::from(body))
        .map_err(|err| Error::middleware(MiddlewareFault(err.to_string())))?;
    Ok(Response::from(response))
}
