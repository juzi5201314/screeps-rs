use crate::log;
use std::panic::set_hook;

pub fn hook_panic() {
    set_hook(Box::new(|info| log!("error: {}", info.to_string())))
}
