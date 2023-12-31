//! Serialize and Deserialize `num_bigint::BigUint` into `Base64urlUInt` form as described in
//! [RFC 7518](https://tools.ietf.org/html/rfc7518).
//! The integers are first converted into bytes in big-endian form and then base64 encoded.
use std::fmt;

use base64ct::{Base64UrlUnpadded, Encoding};
use num_bigint::BigUint;
use serde::de;
use serde::{Deserializer, Serializer};

/// Serialize a `BigUint` into Base64 URL encoded big endian bytes
pub fn serialize<S>(value: &BigUint, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let bytes = value.to_bytes_be();
    let base64 = Base64UrlUnpadded::encode_string(bytes.as_slice());
    serializer.serialize_str(&base64)
}

/// Deserialize a `BigUint` from Base64 URL encoded big endian bytes
pub fn deserialize<'de, D>(deserializer: D) -> Result<BigUint, D::Error>
where
    D: Deserializer<'de>,
{
    struct BigUintVisitor;

    impl<'de> de::Visitor<'de> for BigUintVisitor {
        type Value = BigUint;

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("a Base64urlUInt string")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            let bytes = Base64UrlUnpadded::decode_vec(value).map_err(E::custom)?;
            Ok(BigUint::from_bytes_be(&bytes))
        }
    }

    deserializer.deserialize_str(BigUintVisitor)
}

pub struct Wrapper<'a>(&'a BigUint);

pub fn wrap(data: &BigUint) -> Wrapper {
    Wrapper(data)
}

impl<'a> serde::Serialize for Wrapper<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serialize(self.0, serializer)
    }
}

#[cfg(test)]
mod tests {
    use num_bigint::BigUint;
    use num_traits::cast::FromPrimitive;
    use serde::{Deserialize, Serialize};
    use serde_test::{assert_tokens, Token};

    #[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
    struct TestStruct {
        #[serde(with = "super")]
        bytes: BigUint,
    }

    #[test]
    fn serialization_round_trip() {
        let test_value = TestStruct {
            bytes: BigUint::from_u64(12345).unwrap(),
        };

        assert_tokens(
            &test_value,
            &[
                Token::Struct {
                    name: "TestStruct",
                    len: 1,
                },
                Token::Str("bytes"),
                Token::Str("MDk"),
                Token::StructEnd,
            ],
        );
    }
}
