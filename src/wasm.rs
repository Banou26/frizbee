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

#[wasm_bindgen]
pub fn create_custom_config(
    prefilter: bool,
    max_typos: Option<u16>,
    sort: bool,
    scoring_js: JsValue
) -> Result<JsValue, JsValue> {
    let scoring = if !scoring_js.is_undefined() && !scoring_js.is_null() {
        serde_wasm_bindgen::from_value(scoring_js)
            .map_err(|e| JsValue::from_str(&format!("Scoring parse error: {:?}", e)))?
    } else {
        Scoring::default()
    };

    let config = Config {
        prefilter,
        max_typos,
        sort,
        scoring,
    };

    serde_wasm_bindgen::to_value(&config)
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {:?}", e)))
}

#[wasm_bindgen]
pub fn create_custom_scoring(
    match_score: Option<u16>,
    mismatch_penalty: Option<u16>,
    gap_open_penalty: Option<u16>,
    gap_extend_penalty: Option<u16>,
    prefix_bonus: Option<u16>,
    offset_prefix_bonus: Option<u16>,
    capitalization_bonus: Option<u16>,
    matching_case_bonus: Option<u16>,
    exact_match_bonus: Option<u16>,
    delimiter_bonus: Option<u16>,
    delimiters: Option<String>
) -> Result<JsValue, JsValue> {
    let default = Scoring::default();

    let scoring = Scoring {
        match_score: match_score.unwrap_or(default.match_score),
        mismatch_penalty: mismatch_penalty.unwrap_or(default.mismatch_penalty),
        gap_open_penalty: gap_open_penalty.unwrap_or(default.gap_open_penalty),
        gap_extend_penalty: gap_extend_penalty.unwrap_or(default.gap_extend_penalty),
        prefix_bonus: prefix_bonus.unwrap_or(default.prefix_bonus),
        offset_prefix_bonus: offset_prefix_bonus.unwrap_or(default.offset_prefix_bonus),
        capitalization_bonus: capitalization_bonus.unwrap_or(default.capitalization_bonus),
        matching_case_bonus: matching_case_bonus.unwrap_or(default.matching_case_bonus),
        exact_match_bonus: exact_match_bonus.unwrap_or(default.exact_match_bonus),
        delimiter_bonus: delimiter_bonus.unwrap_or(default.delimiter_bonus),
        delimiters: delimiters.unwrap_or(default.delimiters),
    };

    serde_wasm_bindgen::to_value(&scoring)
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {:?}", e)))
}
