use serde::de::Error;
use serde::{Deserialize, Deserializer, Serializer};
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

/// Custom Pubkey deserializer to use with Serde
pub fn pubkey_deserialize<'de, D>(deserializer: D) -> Result<Pubkey, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer).unwrap();
    Pubkey::from_str(s.as_str()).map_err(D::Error::custom)
}

/// Custom Pubkey serializer to use with Serde
pub fn pubkey_serialize<S>(x: &Pubkey, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(x.to_string().as_str())
}

/// Custom multiple Pubkey deserializer to use with Serde
pub fn multiple_pubkey_deserialize<'de, D>(deserializer: D) -> Result<Vec<Pubkey>, D::Error>
where
    D: Deserializer<'de>,
{
    let ms: Result<Vec<String>, D::Error> = Deserialize::deserialize(deserializer);
    ms.and_then(|strings| {
        strings
            .iter()
            .map(|s| Pubkey::from_str(s.as_str()).map_err(D::Error::custom))
            .collect::<Result<Vec<Pubkey>, D::Error>>()
    })
}

/// Custom multiple Pubkey serializer to use with Serde
pub fn multiple_pubkey_serialize<S>(mx: &[Pubkey], s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let serialized: String = mx
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(",");
    s.serialize_str(&serialized)
}
