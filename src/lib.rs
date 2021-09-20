mod constants;

pub use constants::*;

use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsValue, JsCast};
use js_sys::Object;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[wasm_bindgen]
extern "C" {
    pub type Game;

    #[wasm_bindgen(static_method_of = Game, getter = time)]
    pub fn time() -> u32;
}
