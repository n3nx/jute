#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate jute;
extern crate serde_json;

use jute::jwa::*;
use jute::jws::*;
use jute::*;

fuzz_target!(|data: &[u8]| {
    let signing_secret = Secret::Bytes("secret".to_string().into_bytes());

    let expected_token = std::str::from_utf8(data);
    if expected_token.is_err() {
        return;
    }
    let expected_token = expected_token.unwrap();

    let token = JWT::<serde_json::Value, jute::Empty>::new_encoded(&expected_token);
    let _ = token.into_decoded(&signing_secret, SignatureAlgorithm::HS256);
});
