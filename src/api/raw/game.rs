use js_sys::Object;
use wasm_bindgen::prelude::wasm_bindgen;

use super::cpu::Cpu;
use super::gcl::Gcl;

#[wasm_bindgen]
extern "C" {
    pub type Game;

    #[wasm_bindgen(static_method_of = Game, getter)]
    pub fn time() -> u32;

    /// object<string, room>
    #[wasm_bindgen(static_method_of = Game, getter)]
    pub fn rooms() -> Object;

    #[wasm_bindgen(static_method_of = Game, getter)]
    pub fn cpu() -> Cpu;

    #[wasm_bindgen(static_method_of = Game, getter)]
    pub fn gcl() -> Gcl;
}
