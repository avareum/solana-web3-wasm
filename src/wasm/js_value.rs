#![cfg(feature = "wasm_bindgen")]

use wasm_bindgen::{JsError, JsValue};

pub trait JsValueConverter {
    fn to_js_value(&self) -> Result<JsValue, JsError>;
}

impl JsValueConverter for Vec<String> {
    fn to_js_value(&self) -> Result<JsValue, JsError> {
        match serde_wasm_bindgen::to_value(&self) {
            Ok(js_value) => Ok(js_value),
            Err(err) => Err(JsError::new(&err.to_string())),
        }
    }
}
