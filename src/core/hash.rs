use serde::de::Error;
use serde::{Deserialize, Deserializer, Serializer};
use solana_sdk::hash::Hash;
use std::str::FromStr;

/// Custom Hash deserializer to use with Serde
pub fn hash_deserialize<'de, D>(deserializer: D) -> Result<Hash, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer).unwrap();
    Hash::from_str(s.as_str()).map_err(D::Error::custom)
}

/// Custom Hash serializer to use with Serde
pub fn hash_serialize<S>(x: &Hash, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(x.to_string().as_str())
}
