use std::sync::Arc;

use base64::Engine;
use kalshi_api::{Clock, SignMiddleware};
use rsa::pkcs8::{EncodePrivateKey, LineEnding};
use rsa::pss::{Signature, VerifyingKey};
use rsa::sha2::Sha256;
use rsa::signature::Verifier;
use rsa::RsaPrivateKey;

struct FixedClock;

impl Clock for FixedClock {
    fn unix_ms(&self) -> u64 {
        1_700_000_000_000
    }
}

#[test]
fn external_crate_signs_the_ws_path_without_the_query() {
    let mut rng = rand::rngs::OsRng;
    let private = RsaPrivateKey::new(&mut rng, 2048).unwrap();
    let pem = private.to_pkcs8_pem(LineEnding::LF).unwrap().to_string();
    let verifying = VerifyingKey::<Sha256>::new(private.to_public_key());
    let signer = SignMiddleware::new("key-synthetic", &pem, Arc::new(FixedClock)).unwrap();
    let (key_id, timestamp, signature) = signer.headers("GET", "/trade-api/ws/v2?ticker=SYNTH-1");
    assert_eq!(key_id, "key-synthetic");
    assert_eq!(timestamp, "1700000000000");
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(signature)
        .unwrap();
    let signature = Signature::try_from(bytes.as_slice()).unwrap();
    verifying
        .verify(b"1700000000000GET/trade-api/ws/v2", &signature)
        .unwrap();
    assert!(verifying
        .verify(
            b"1700000000000GET/trade-api/ws/v2?ticker=SYNTH-1",
            &signature,
        )
        .is_err());
}
