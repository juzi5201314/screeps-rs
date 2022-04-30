use js_sys::Object;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    pub type Cpu;

    #[wasm_bindgen(method, getter)]
    pub fn limit(_: &Cpu) -> u32;

    #[wasm_bindgen(method, getter = tickLimit)]
    pub fn tick_limit(_: &Cpu) -> u32;

    #[wasm_bindgen(method, getter)]
    pub fn bucket(_: &Cpu) -> u32;

    #[wasm_bindgen(method, getter)]
    pub fn unlocked(_: &Cpu) -> bool;

    #[wasm_bindgen(method, getter = unlockedTime)]
    pub fn unlocked_time(_: &Cpu) -> u32;

    /// object<string, number>
    #[wasm_bindgen(method, getter = shardLimits)]
    pub fn shard_limits(_: &Cpu) -> Object;
}
