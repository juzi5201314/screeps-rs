#![feature(decl_macro)]
#![feature(extern_types)]
#![allow(unused)]

use js_sys::Object;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

pub mod api;
pub mod hook_panic;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn js_log(s: &str);
}

#[wasm_bindgen(module = "/js/utils.js")]
extern "C" {
    pub fn get_prop(obj: &Object, prop: &str) -> JsValue;
}

/// like `println!` but use `console.log`
#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        $crate::js_log(&format!($($arg)*))
    };
}

/// like rust `dbg!` macro
#[macro_export]
macro_rules! js_dbg {
    () => {
        $crate::log!("[{}:{}]", file!(), line!())
    };
    ($val:expr $(,)?) => {
        match $val {
            tmp => {
                $crate::log!("[{}:{}] {} = {:#?}",
                    file!(), line!(), stringify!($val), &tmp);
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($($crate::js_dbg!($val)),+,)
    };
}
