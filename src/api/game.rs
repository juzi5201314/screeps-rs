use ahash::AHashMap;
use js_sys::JsString;
use smallvec::SmallVec;

use crate::api::cpu::Cpu;
use crate::api::jskvobj::JsKVObj;
use crate::api::raw::FromRaw;
use crate::api::{raw, Room};

pub struct Game;

impl Game {
    #[inline]
    pub fn time() -> u32 {
        raw::game::Game::time()
    }

    #[inline]
    pub fn cpu() -> Cpu {
        Cpu::from_raw(raw::game::Game::cpu())
    }



    pub fn rooms() -> AHashMap<String, Room> {
        let obj = raw::game::Game::rooms();
        JsKVObj::<JsString, raw::room::Room>::new(obj)
            .entries::<10>()
            .into_iter()
            .map(|(str, room)| (str.as_string().unwrap(), Room::from_raw(room)))
            .collect()
    }
}
