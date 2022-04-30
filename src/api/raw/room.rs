use js_sys::JsString;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    pub type Room;

    #[wasm_bindgen(method, getter = energyAvailable)]
    pub fn energy_available(_: &Room) -> u32;

    #[wasm_bindgen(method, getter = energyCapacityAvailable)]
    pub fn energy_capacity_available(_: &Room) -> u32;

    #[wasm_bindgen(method, getter)]
    pub fn name(_: &Room) -> JsString;
}
