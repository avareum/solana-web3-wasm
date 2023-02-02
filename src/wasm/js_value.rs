#![cfg(feature = "wasm_bindgen")]

use serde_wasm_bindgen::to_value;
use wasm_bindgen::{JsError, JsValue};

pub trait JsValueConverter {
    fn to_js_value(&self) -> Result<JsValue, JsError>;
    fn to_js_vec_value(&self) -> Result<Vec<JsValue>, JsError>;
}

impl JsValueConverter for Vec<String> {
    fn to_js_value(&self) -> Result<JsValue, JsError> {
        match serde_wasm_bindgen::to_value(&self) {
            Ok(js_value) => Ok(js_value),
            Err(err) => Err(JsError::new(&err.to_string())),
        }
    }

    fn to_js_vec_value(&self) -> Result<Vec<JsValue>, JsError> {
        let mut errors = vec![];
        let vec_str = self
            .iter()
            .map(to_value)
            .filter_map(|r| r.map_err(|e| errors.push(e)).ok())
            .collect::<Vec<_>>();

        if !errors.is_empty() {
            let error_message = errors
                .iter()
                .map(|error| error.to_string())
                .collect::<Vec<_>>()
                .join("\n");
            Err(JsError::new(&error_message))
        } else {
            Ok(vec_str)
        }
    }
}
