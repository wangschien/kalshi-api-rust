use std::time::Duration;

use async_trait::async_trait;
use reqwest::{Method, Request, Response};
use reqwest_middleware::{Middleware, Next, Result};

/// Opt-in retries matching the official async SDK's configured retry policy.
///
/// `attempts` includes the first request. Only GET, HEAD, OPTIONS, PUT, DELETE,
/// and TRACE retry server errors. Transport errors and POST/PATCH are returned
/// immediately. Add this after signing to reuse the original authentication headers.
#[derive(Debug, Clone, Copy)]
pub struct RetryMiddleware {
    attempts: usize,
}

impl RetryMiddleware {
    pub fn new(attempts: usize) -> Self {
        Self {
            attempts: attempts.max(1),
        }
    }
}

#[async_trait]
impl Middleware for RetryMiddleware {
    async fn handle(
        &self,
        mut request: Request,
        extensions: &mut http::Extensions,
        next: Next<'_>,
    ) -> Result<Response> {
        let eligible = matches!(
            *request.method(),
            Method::GET
                | Method::HEAD
                | Method::OPTIONS
                | Method::PUT
                | Method::DELETE
                | Method::TRACE
        );
        if !eligible || self.attempts == 1 {
            return next.run(request, extensions).await;
        }
        let Some(template) = request.try_clone() else {
            return next.run(request, extensions).await;
        };
        let mut attempt = 1;
        loop {
            let response = next.clone().run(request, extensions).await?;
            if !response.status().is_server_error() || attempt >= self.attempts {
                return Ok(response);
            }
            drop(response);
            let delay_ms = (100_u64 * (1_u64 << attempt.min(11))).min(120_000);
            tokio::time::sleep(Duration::from_millis(delay_ms)).await;
            request = template
                .try_clone()
                .expect("buffered request remains cloneable");
            attempt += 1;
        }
    }
}
