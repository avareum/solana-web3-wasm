use serde::de::Error;
use serde::{Deserialize, Deserializer, Serializer};
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

/// Custom Pubkey deserializer to use with Serde
pub fn pubkey_deserialize<'de, D>(deserializer: D) -> Result<Pubkey, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    Pubkey::from_str(s.as_str()).map_err(D::Error::custom)
}

/// Custom Pubkey serializer to use with Serde
pub fn pubkey_serialize<S>(x: &Pubkey, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(x.to_string().as_str())
}
