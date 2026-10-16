use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use async_trait::async_trait;
use base64::Engine;
use ed25519_dalek::SigningKey as Ed25519SigningKey;
use rand::rngs::OsRng;
use reqwest::{Request, Response};
use reqwest_middleware::{Error, Middleware, Next, Result};
use rsa::pkcs1::DecodeRsaPrivateKey;
use rsa::pkcs8::DecodePrivateKey;
use rsa::pss::SigningKey;
use rsa::sha2::Sha256;
use rsa::signature::{RandomizedSigner, SignatureEncoding, Signer};
use rsa::RsaPrivateKey;

use crate::error::MiddlewareFault;

pub trait Clock: Send + Sync {
    fn unix_ms(&self) -> u64;

    /// Nanoseconds since the Unix epoch. The WebSocket task stores this when a Ping arrives.
    fn unix_ns(&self) -> u64 {
        self.unix_ms().saturating_mul(1_000_000)
    }
}

pub struct SystemClock;

impl Clock for SystemClock {
    fn unix_ms(&self) -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_millis() as u64)
            .unwrap_or(0)
    }

    fn unix_ns(&self) -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_nanos() as u64)
            .unwrap_or(0)
    }
}

pub struct SignMiddleware {
    key_id: String,
    signing_key: SigningMaterial,
    clock: Arc<dyn Clock>,
}

impl SignMiddleware {
    pub fn new(
        key_id: impl Into<String>,
        private_key_pem: &str,
        clock: Arc<dyn Clock>,
    ) -> std::result::Result<Self, Error> {
        let signing_key = load_signing_key(private_key_pem)
            .map_err(|err| Error::middleware(MiddlewareFault(err.to_string())))?;
        Ok(Self {
            key_id: key_id.into(),
            signing_key,
            clock,
        })
    }

    /// `(key id, timestamp milliseconds, base64 signature)`.
    ///
    /// The signed text is `{timestamp}{METHOD}{path}`. Query text is excluded.
    pub fn headers(&self, method: &str, path: &str) -> (String, String, String) {
        let path = path.split('?').next().unwrap_or(path);
        let timestamp = self.clock.unix_ms().to_string();
        let message = format!("{timestamp}{}{path}", method.to_ascii_uppercase());
        let signature = match &self.signing_key {
            SigningMaterial::Rsa(key) => key.sign_with_rng(&mut OsRng, message.as_bytes()).to_vec(),
            SigningMaterial::Ed25519(key) => key.sign(message.as_bytes()).to_bytes().to_vec(),
        };
        let encoded = base64::engine::general_purpose::STANDARD.encode(signature);
        (self.key_id.clone(), timestamp, encoded)
    }
}

enum SigningMaterial {
    Rsa(Box<SigningKey<Sha256>>),
    Ed25519(Box<Ed25519SigningKey>),
}

fn load_signing_key(pem: &str) -> std::result::Result<SigningMaterial, MiddlewareFault> {
    if let Ok(key) = RsaPrivateKey::from_pkcs8_pem(pem)
        .or_else(|_| RsaPrivateKey::from_pkcs1_pem(pem).map_err(rsa::pkcs8::Error::from))
    {
        return Ok(SigningMaterial::Rsa(Box::new(SigningKey::<Sha256>::new(
            key,
        ))));
    }
    Ed25519SigningKey::from_pkcs8_pem(pem)
        .map(|key| SigningMaterial::Ed25519(Box::new(key)))
        .map_err(|_| MiddlewareFault("private key must be an RSA or Ed25519 PEM".to_owned()))
}

#[async_trait]
impl Middleware for SignMiddleware {
    async fn handle(
        &self,
        mut req: Request,
        extensions: &mut http::Extensions,
        next: Next<'_>,
    ) -> Result<Response> {
        let path = req.url().path().to_owned();
        let method = req.method().as_str().to_owned();
        let (key_id, timestamp, signature) = self.headers(&method, &path);
        let headers = req.headers_mut();
        headers.insert("KALSHI-ACCESS-KEY", header_value(&key_id)?);
        headers.insert("KALSHI-ACCESS-TIMESTAMP", header_value(&timestamp)?);
        headers.insert("KALSHI-ACCESS-SIGNATURE", header_value(&signature)?);
        next.run(req, extensions).await
    }
}

fn header_value(value: &str) -> Result<reqwest::header::HeaderValue> {
    reqwest::header::HeaderValue::from_str(value)
        .map_err(|err| Error::middleware(MiddlewareFault(err.to_string())))
}
