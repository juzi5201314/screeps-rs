use serde::{Serialize, Deserialize};
use serde::de::DeserializeOwned;
use wasm_bindgen::prelude::wasm_bindgen;
use std::collections::HashMap;

#[wasm_bindgen]
extern "C" {
    pub type RawMemory;

    #[wasm_bindgen(static_method_of = RawMemory, setter = set)]
    fn set(val: &str);

    #[wasm_bindgen(static_method_of = RawMemory, getter = get)]
    pub fn get() -> String;
}

#[inline]
pub fn get_raw_memory<T>() -> T where T: DeserializeOwned {
    serde_json::from_str(&RawMemory::get()).unwrap()
}

#[inline]
pub fn set_raw_memory<T>(val: &T) where T: Serialize {
    RawMemory::set(serde_json::to_string(val).unwrap().as_str())
}

/*#[wasm_bindgen]
extern "C" {
    pub type Memory;

    #[wasm_bindgen(static_method_of = Memory, setter = creeps)]
    fn creeps() -> HashMap<>;
}
*/
