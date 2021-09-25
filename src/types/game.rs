use std::collections::BTreeMap;

use js_sys::{Array, Object};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};

use crate::types::room_object::room::JsRoom;
use crate::{JsString, Room};

#[wasm_bindgen]
extern "C" {
    pub type Game;

    #[wasm_bindgen(static_method_of = Game, getter = time)]
    pub fn __time() -> u32;

    #[wasm_bindgen(static_method_of = Game, getter = rooms)]
    pub fn __rooms() -> Object;
}

pub fn time() -> u32 {
    Game::__time()
}

pub fn room() -> BTreeMap<JsString, Room> {
    let mut map = BTreeMap::new();
    let obj: Object = Game::__rooms();
    Object::entries(&obj).for_each(&mut |val: JsValue, i, arr| {
        let arr: Array = val.unchecked_into::<Array>();
        map.insert(
            {
                #[cfg(feature = "rust_string")]
                {
                    arr.get(0).as_string().unwrap()
                }

                #[cfg(not(feature = "rust_string"))]
                {
                    arr.get(0).unchecked_into::<JsString>()
                }
            },
            arr.get(1).unchecked_into::<JsRoom>().into(),
        );
    });
    map
}
