use wasm_bindgen::prelude::*;
use serde_wasm_bindgen;

use crate::{Config, Scoring};

#[wasm_bindgen]
pub struct WasmMatcher;

#[wasm_bindgen]
impl WasmMatcher {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        WasmMatcher
    }

    #[wasm_bindgen(js_name = matchList)]
    pub fn match_list(&self, needle: &str, haystacks: Vec<String>, config_js: JsValue) -> Result<JsValue, JsValue> {
        let config: Config = if config_js.is_undefined() || config_js.is_null() {
            Config::default()
        } else {
            serde_wasm_bindgen::from_value(config_js)
                .map_err(|e| JsValue::from_str(&format!("Config parse error: {:?}", e)))?
        };

        let haystacks_refs: Vec<&str> = haystacks.iter().map(|s| s.as_str()).collect();
        let matches = crate::match_list(needle, &haystacks_refs, config);
        
        serde_wasm_bindgen::to_value(&matches)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {:?}", e)))
    }

    #[wasm_bindgen(js_name = matchIndices)]
    pub fn match_indices(&self, needle: &str, haystack: &str, config_js: JsValue) -> Result<JsValue, JsValue> {
        let config: Config = if config_js.is_undefined() || config_js.is_null() {
            Config::default()
        } else {
            serde_wasm_bindgen::from_value(config_js)
                .map_err(|e| JsValue::from_str(&format!("Config parse error: {:?}", e)))?
        };

        let result = crate::match_indices(needle, haystack, config);
        
        match result {
            Some(indices) => {
                serde_wasm_bindgen::to_value(&indices)
                    .map_err(|e| JsValue::from_str(&format!("Serialization error: {:?}", e)))
            },
            None => Ok(JsValue::NULL)
        }
    }
}

#[wasm_bindgen]
pub fn create_default_config() -> Result<JsValue, JsValue> {
    let config = Config::default();
    serde_wasm_bindgen::to_value(&config)
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {:?}", e)))
}

#[wasm_bindgen]
pub fn create_default_scoring() -> Result<JsValue, JsValue> {
    let scoring = Scoring::default();
    serde_wasm_bindgen::to_value(&scoring)
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {:?}", e)))
}