use serde::{Deserialize, Deserializer, Serializer};

use serde_json::Value;
use solana_sdk::signature::Signature;
use std::collections::HashMap;

pub fn signature_deserialize<'de, D>(deserializer: D) -> Result<Signature, D::Error>
where
    D: Deserializer<'de>,
{
    let s: HashMap<&str, u8> = Deserialize::deserialize(deserializer).unwrap();
    let u8s = s.into_values().collect::<Vec<_>>();
    dbg!(&u8s);
    Ok(Signature::new(&u8s))
}

pub fn signature_serialize<S>(x: &Signature, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(x.to_string().as_str())
}

#[derive(Deserialize, Debug)]
struct MyStruct {
    signatures: Vec<HashMap<String, Value>>,
}

pub fn multiple_signature_deserialize<'de, D>(deserializer: D) -> Result<Vec<Signature>, D::Error>
where
    D: Deserializer<'de>,
{
    // let ms: Result<Vec<HashMap<&str, u8>>, D::Error> = Deserialize::deserialize(deserializer);
    // dbg!(&ms);
    // ms.and_then(|strings| {
    //     strings
    //         .into_iter()
    //         .map(|s| {
    //             // let s: HashMap<&str, u8> = serde_json::from_str(s).unwrap();
    //             let u8s = s.into_values().collect::<Vec<_>>();
    //             dbg!(&u8s);
    //             Ok(Signature::new(&u8s))
    //         })
    //         .collect::<Result<Vec<Signature>, D::Error>>()
    // })

    // let ms: Result<Vec<HashMap<String, Value>>, D::Error> = Deserialize::deserialize(deserializer);
    // dbg!(&ms);
    // ms.and_then(|strings| {
    //     strings
    //         .into_iter()
    //         .map(|s| {
    //             let u8s = s
    //                 .into_values()
    //                 .map(|e| e.as_u64().unwrap() as u8)
    //                 .collect::<Vec<u8>>();
    //             dbg!(&u8s);
    //             Ok(Signature::new(&u8s))
    //         })
    //         .collect::<Result<Vec<Signature>, D::Error>>()
    // })

    //     let u8s = tx_json
    //     .signatures
    //     .into_iter()
    //     .map(|s| s.into_values().collect::<Vec<Value>>()[0].clone())
    //     .collect::<Vec<_>>();
    // println!("{:#?}", u8s);

    Ok(vec![Signature::default()])
}

pub fn multiple_signature_serialize<S>(mx: &[Signature], s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // let serialized: String = mx
    //     .iter()
    //     .map(|x| x.to_string())
    //     .collect::<Vec<_>>()
    //     .join(",");
    // s.serialize_str(&serialized)
    s.serialize_str("ok")
}
