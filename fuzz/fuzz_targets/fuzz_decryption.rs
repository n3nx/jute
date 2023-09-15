#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate jute;
extern crate serde_json;

use jute::jwa::{ContentEncryptionAlgorithm, KeyManagementAlgorithm};
use jute::jwk::JWK;
use jute::{Empty, JWE};

fuzz_target!(|data: &[u8]| {
    let key: JWK<Empty> = JWK::new_octet_key(&vec![0; 256 / 8], Default::default());

    let token = std::str::from_utf8(data);
    if token.is_err() {
        return;
    }
    let token = token.unwrap();

    let token: JWE<serde_json::Value, jute::Empty, jute::Empty> = JWE::new_encrypted(&token);

    let _ = token.into_decrypted(
        &key,
        KeyManagementAlgorithm::A256GCMKW,
        ContentEncryptionAlgorithm::A256GCM,
    );
});
