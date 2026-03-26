use js_sys::{self, Array, Object, Reflect, Uint32Array};
use wasm_bindgen::prelude::*;

use crate::{Config, Match, MatchIndices, Scoring};

fn parse_config(val: &JsValue) -> Config {
    if val.is_undefined() || val.is_null() {
        return Config::default();
    }
    let max_typos = Reflect::get(val, &"maxTypos".into())
        .ok()
        .and_then(|v| v.as_f64())
        .map(|v| v as u16);
    let sort = Reflect::get(val, &"sort".into())
        .ok()
        .and_then(|v| v.as_bool())
        .unwrap_or(true);
    Config {
        max_typos,
        sort,
        scoring: Scoring::default(),
    }
}

fn haystacks_from_js(arr: &Array) -> Vec<String> {
    arr.iter().filter_map(|v| v.as_string()).collect()
}

fn match_to_js(m: &Match) -> JsValue {
    let obj = Object::new();
    Reflect::set(&obj, &"score".into(), &JsValue::from(m.score)).unwrap();
    Reflect::set(&obj, &"index".into(), &JsValue::from(m.index)).unwrap();
    Reflect::set(&obj, &"exact".into(), &JsValue::from(m.exact)).unwrap();
    obj.into()
}

fn match_indices_to_js(m: &MatchIndices) -> JsValue {
    let obj = Object::new();
    Reflect::set(&obj, &"score".into(), &JsValue::from(m.score)).unwrap();
    Reflect::set(&obj, &"index".into(), &JsValue::from(m.index)).unwrap();
    Reflect::set(&obj, &"exact".into(), &JsValue::from(m.exact)).unwrap();
    let indices: Vec<u32> = m.indices.iter().map(|&i| i as u32).collect();
    Reflect::set(
        &obj,
        &"indices".into(),
        &Uint32Array::from(&indices[..]).into(),
    )
    .unwrap();
    obj.into()
}

fn matches_to_js(matches: &[Match]) -> Array {
    let result = Array::new_with_length(matches.len() as u32);
    for (i, m) in matches.iter().enumerate() {
        result.set(i as u32, match_to_js(m));
    }
    result
}

fn match_indices_to_js_array(matches: &[MatchIndices]) -> Array {
    let result = Array::new_with_length(matches.len() as u32);
    for (i, m) in matches.iter().enumerate() {
        result.set(i as u32, match_indices_to_js(m));
    }
    result
}

#[wasm_bindgen(js_name = matchList)]
pub fn wasm_match_list(needle: &str, haystacks: Array, config: JsValue) -> Array {
    let haystacks = haystacks_from_js(&haystacks);
    let config = parse_config(&config);
    let matches = crate::match_list(needle, &haystacks, &config);
    matches_to_js(&matches)
}

#[wasm_bindgen(js_name = matchListIndices)]
pub fn wasm_match_list_indices(needle: &str, haystacks: Array, config: JsValue) -> Array {
    let haystacks = haystacks_from_js(&haystacks);
    let config = parse_config(&config);
    let matches = crate::match_list_indices(needle, &haystacks, &config);
    match_indices_to_js_array(&matches)
}

#[wasm_bindgen]
pub struct Matcher {
    inner: crate::one_shot::Matcher,
}

#[wasm_bindgen]
impl Matcher {
    #[wasm_bindgen(constructor)]
    pub fn new(needle: &str, config: JsValue) -> Self {
        let config = parse_config(&config);
        Self {
            inner: crate::Matcher::new(needle, &config),
        }
    }

    #[wasm_bindgen(js_name = setNeedle)]
    pub fn set_needle(&mut self, needle: &str) {
        self.inner.set_needle(needle);
    }

    #[wasm_bindgen(js_name = matchList)]
    pub fn match_list(&mut self, haystacks: Array) -> Array {
        let haystacks = haystacks_from_js(&haystacks);
        let matches = self.inner.match_list(&haystacks);
        matches_to_js(&matches)
    }

    #[wasm_bindgen(js_name = matchListIndices)]
    pub fn match_list_indices(&mut self, haystacks: Array) -> Array {
        let haystacks = haystacks_from_js(&haystacks);
        let matches = self.inner.match_list_indices(&haystacks);
        match_indices_to_js_array(&matches)
    }
}
