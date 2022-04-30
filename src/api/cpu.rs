use ahash::AHashMap;
use js_sys::JsString;
use wasm_bindgen::JsValue;

use crate::api::jskvobj::JsKVObj;
use crate::api::raw;

pub struct Cpu(raw::cpu::Cpu);

impl Cpu {
    #[inline]
    pub fn limit(&self) -> u32 {
        self.0.limit()
    }

    #[inline]
    pub fn tick_limit(&self) -> u32 {
        self.0.tick_limit()
    }

    #[inline]
    pub fn bucket(&self) -> u32 {
        self.0.bucket()
    }

    #[inline]
    pub fn unlocked(&self) -> bool {
        self.0.unlocked()
    }

    #[inline]
    pub fn unlocked_time(&self) -> u32 {
        self.0.unlocked_time()
    }

    pub fn shard_limits(&self) -> AHashMap<String, u32> {
        JsKVObj::<JsString, JsValue>::new(self.0.shard_limits())
            .entries::<10>()
            .into_iter()
            .map(|(s, v)| (s.as_string().unwrap(), v.as_f64().unwrap() as u32))
            .collect()
    }
}

impl raw::FromRaw for Cpu {
    type Raw = raw::cpu::Cpu;

    fn from_raw(raw: Self::Raw) -> Self {
        Cpu(raw)
    }
}
