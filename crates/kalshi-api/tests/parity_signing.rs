use std::sync::Arc;

use base64::Engine;
use kalshi_api::{Clock, SignMiddleware};
use rsa::pkcs8::DecodePublicKey;
use rsa::sha2::Sha256;
use rsa::signature::Verifier;
use serde_json::Value;

struct FixedClock;

impl Clock for FixedClock {
    fn unix_ms(&self) -> u64 {
        1_700_000_000_000
    }
}

#[test]
fn python_and_rust_sign_the_identical_message_for_both_key_types() {
    let cases: Vec<Value> =
        serde_json::from_str(include_str!("fixtures/parity/signing.json")).unwrap();
    assert_eq!(cases.len(), 10);
    for case in cases {
        let algorithm = case["algorithm"].as_str().unwrap();
        let key = match algorithm {
            "rsa" => include_str!("fixtures/parity/rsa_test_private.pem"),
            "ed25519" => include_str!("fixtures/parity/ed25519_test_private.pem"),
            other => panic!("unexpected algorithm {other}"),
        };
        let signer = SignMiddleware::new("synthetic-key", key, Arc::new(FixedClock)).unwrap();
        let method = case["method"].as_str().unwrap();
        let path = case["path"].as_str().unwrap();
        let (key_id, timestamp, rust_signature) = signer.headers(method, path);
        assert_eq!(key_id, case["headers"]["KALSHI-ACCESS-KEY"]);
        assert_eq!(timestamp, case["headers"]["KALSHI-ACCESS-TIMESTAMP"]);
        let message = format!(
            "{timestamp}{}{}",
            method.to_ascii_uppercase(),
            path.split('?').next().unwrap()
        );
        assert_eq!(message, case["message"]);
        for encoded in [
            case["headers"]["KALSHI-ACCESS-SIGNATURE"].as_str().unwrap(),
            &rust_signature,
        ] {
            let bytes = base64::engine::general_purpose::STANDARD
                .decode(encoded)
                .unwrap();
            match algorithm {
                "rsa" => {
                    let public = rsa::RsaPublicKey::from_public_key_pem(include_str!(
                        "fixtures/parity/rsa_test_public.pem"
                    ))
                    .unwrap();
                    let verifier = rsa::pss::VerifyingKey::<Sha256>::new_with_salt_len(public, 32);
                    let signature = rsa::pss::Signature::try_from(bytes.as_slice()).unwrap();
                    verifier.verify(message.as_bytes(), &signature).unwrap();
                    assert!(verifier.verify(b"different-message", &signature).is_err());
                }
                "ed25519" => {
                    let verifier = ed25519_dalek::VerifyingKey::from_public_key_pem(include_str!(
                        "fixtures/parity/ed25519_test_public.pem"
                    ))
                    .unwrap();
                    let signature = ed25519_dalek::Signature::from_slice(&bytes).unwrap();
                    verifier
                        .verify_strict(message.as_bytes(), &signature)
                        .unwrap();
                    assert!(verifier
                        .verify_strict(b"different-message", &signature)
                        .is_err());
                }
                _ => unreachable!(),
            }
        }
    }
}
