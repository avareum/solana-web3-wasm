use serde::de::{self, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};
use serde_json::{Map, Value};

use std::collections::HashMap;
use std::fmt;

/*
TODO: test
test_data_0 = { "data": {} }
test_data_1 = { "data": { "0": 229, "1": 23, "2": 203 } }
test_data_2 = { "data": { "type": "Buffer", "data": [124, 51, 114] } }
*/

#[derive(Debug, Default, Clone, Deserialize)]
struct CompiledInstructionDataValue {
    #[serde(deserialize_with = "hashmap_or_buffer_deserialize")]
    data: Vec<u8>,
}
pub fn hashmap_or_buffer_deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    struct DataVisitor;

    impl<'de> Visitor<'de> for DataVisitor {
        type Value = Vec<u8>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a hashmap or buffer")
        }

        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: MapAccess<'de>,
        {
            let mut data = Vec::new();
            while let Some((key, value)) = map.next_entry::<String, serde_json::Value>()? {
                let index = key.parse::<u8>().expect("expected u8");
                println!("{index:?}");
                match &value {
                    Value::Number(num) => data.push(num.as_u64().ok_or(0u64).unwrap() as u8),
                    Value::Array(arr) => arr
                        .iter()
                        .for_each(|e| data.push(e.as_u64().ok_or(0u64).unwrap() as u8)),
                    _ => println!("ignore"),
                }
            }
            Ok(data)
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: de::SeqAccess<'de>,
        {
            let mut data = Vec::new();
            while let Some(value) = seq.next_element()? {
                data.push(value);
            }
            Ok(data)
        }
    }

    deserializer.deserialize_any(DataVisitor)
}

pub fn get_u8s_from_option_json_stringify_uint8(
    signatures: Option<Vec<HashMap<String, Value>>>,
) -> Vec<Vec<u8>> {
    let signatures = match signatures {
        Some(signatures) => signatures,
        None => return vec![],
    };

    signatures
        .into_iter()
        .map(|e| {
            let mut keys = e
                .keys()
                .flat_map(|e| e.parse::<usize>())
                .collect::<Vec<_>>();
            keys.sort();
            keys.into_iter()
                .filter_map(|k| e[&k.to_string()].as_u64())
                .map(|e| e as u8)
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<_>>()
}

pub fn get_u8s_from_json_stringify_uint8(uint8: Map<String, Value>) -> Vec<u8> {
    let mut keys = uint8
        .keys()
        .flat_map(|e| e.parse::<usize>())
        .collect::<Vec<_>>();
    keys.sort();
    keys.iter()
        .filter_map(|k| uint8[&k.to_string()].as_u64())
        .map(|e| e as u8)
        .collect::<Vec<_>>()
}

#[derive(Debug, Clone, Deserialize)]
pub struct Uint8Data {
    #[serde(deserialize_with = "deserialize_uint8")]
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BufferData {
    #[serde(deserialize_with = "deserialize_buffer")]
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Deserialize)]
struct Buffer {
    r#type: String,
    data: Vec<u8>,
}

fn deserialize_buffer<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let result = Buffer::deserialize(deserializer);
    let result = match result {
        Ok(u8s) => u8s.data,
        Err(_) => vec![],
    };

    Ok(result)
}

fn deserialize_uint8<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let result: Map<_, _> = Deserialize::deserialize(deserializer).unwrap();
    let result = get_u8s_from_json_stringify_uint8(result);
    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn success_deserialize() {
        let test_data_0 = r#"{"data":{}}"#;
        let test_data_1 =
            r#"{"data":{"0":1,"1":2,"2":3,"3":4,"4":5,"5":6,"6":7,"7":8,"8":9,"9":10,"10":11}}"#;
        let test_data_2 = r#"{"data":{"type":"Buffer","data":[1,2,3]}}"#;

        let deserialized_data_0: Uint8Data = serde_json::from_str(test_data_0).unwrap();
        println!("{:#?}", deserialized_data_0);

        let deserialized_data_1: Uint8Data = serde_json::from_str(test_data_1).unwrap();
        println!("{:#?}", deserialized_data_1);

        let deserialized_data_2: BufferData = serde_json::from_str(test_data_2).unwrap();
        println!("{:#?}", deserialized_data_2);
    }
}
