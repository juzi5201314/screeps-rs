#![feature(decl_macro)]

use js_sys::Object;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;

pub use constants::*;
pub use types::*;

mod constants;
pub mod hook_panic;
mod types;

#[cfg(feature = "rust_string")]
pub type JsString = String;

#[cfg(not(feature = "rust_string"))]
pub type JsString = js_sys::JsString;


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
