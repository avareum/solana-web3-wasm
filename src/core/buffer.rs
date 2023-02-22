use serde::{Deserialize, Deserializer};
use serde_json::{json, Map, Value};

use std::collections::HashMap;

pub fn buffer_or_uint8array_deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Value = Deserialize::deserialize(deserializer)?;
    match serde_json::from_value::<BufferData>(json!({ "data": value })) {
        Ok(data) => Ok(data.data),
        Err(_) => match serde_json::from_value::<Uint8Data>(json!({ "data": value })) {
            Ok(data) => Ok(data.data),
            Err(_) => Ok(vec![]),
        },
    }
}

pub fn get_u8s_from_option_hashmap_json_stringify_uint8(
    maybe_uint8: Option<Vec<HashMap<String, Value>>>,
) -> Vec<Vec<u8>> {
    let signatures = match maybe_uint8 {
        Some(uint8s) => uint8s,
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

pub fn get_u8s_from_map_json_stringify_uint8(uint8: Map<String, Value>) -> Vec<u8> {
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

#[derive(Debug, Clone, Deserialize, Eq, PartialEq)]
pub struct Uint8Data {
    #[serde(deserialize_with = "deserialize_uint8")]
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Deserialize, Eq, PartialEq)]
pub struct BufferData {
    #[serde(deserialize_with = "deserialize_buffer")]
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Deserialize)]
struct Buffer {
    #[allow(dead_code)]
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
        Err(_) => {
            return Err(serde::de::Error::custom(
                "Failed to deserialize at deserialize_buffer",
            ))
        }
    };

    Ok(result)
}

fn deserialize_uint8<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let result = Deserialize::deserialize(deserializer);
    let result = match result {
        Ok(map) => get_u8s_from_map_json_stringify_uint8(map),
        Err(_) => {
            return Err(serde::de::Error::custom(
                "Failed to deserialize at deserialize_uint8",
            ))
        }
    };

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
        assert_eq!(deserialized_data_0, Uint8Data { data: vec![] });

        let deserialized_data_1: Uint8Data = serde_json::from_str(test_data_1).unwrap();
        println!("{:#?}", deserialized_data_1);
        assert_eq!(
            deserialized_data_1,
            Uint8Data {
                data: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11,],
            }
        );

        let deserialized_data_2: BufferData = serde_json::from_str(test_data_2).unwrap();
        println!("{:#?}", deserialized_data_2);
        assert_eq!(
            deserialized_data_2,
            BufferData {
                data: vec![1, 2, 3,]
            }
        );
    }
}
