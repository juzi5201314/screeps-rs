use std::fmt::{Debug, Formatter};

use js_sys::Object;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsCast;

use crate::{get_prop, JsString};

use crate::types::def_attr;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = Room)]
    pub(crate) type JsRoom;

    #[wasm_bindgen(method, getter = energyAvailable)]
    pub fn energy_available(_: &JsRoom) -> u32;

    #[wasm_bindgen(method, getter = energyCapacityAvailable)]
    pub fn energy_capacity_available(_: &JsRoom) -> u32;

    #[wasm_bindgen(method, getter = name)]
    pub fn name(_: &JsRoom) -> JsString;
}

pub struct Room(pub(crate) JsRoom);

impl Room {
    def_attr!(energy_available, u32);
    def_attr!(energy_capacity_available, u32);
    def_attr!(name, JsString);
}

impl Debug for Room {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "room")
    }
}

impl From<JsRoom> for Room {
    fn from(j: JsRoom) -> Self {
        Room(j)
    }
}
