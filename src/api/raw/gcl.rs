use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    pub type Gcl;

    #[wasm_bindgen(method, getter)]
    pub fn level(_: &Gcl) -> u32;

    #[wasm_bindgen(method, getter)]
    pub fn progress(_: &Gcl) -> u32;

    #[wasm_bindgen(method, getter = progressTotal)]
    pub fn progress_total(_: &Gcl) -> u32;
}
