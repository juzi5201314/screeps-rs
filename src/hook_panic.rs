use std::panic::set_hook;
use crate::log;

pub fn hook_panic() {
    set_hook(Box::new(|info| log!("error: {}", info.to_string())))
}
