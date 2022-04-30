use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;

use ahash::AHashMap;
use js_sys::{Array, JsString, Object};
use smallvec::SmallVec;
use wasm_bindgen::convert::FromWasmAbi;
use wasm_bindgen::describe::WasmDescribe;
use wasm_bindgen::JsCast;

pub struct JsKVObj<K, V> {
    obj: Object,
    _marker1: PhantomData<K>,
    _marker2: PhantomData<V>,
}

impl<K, V> JsKVObj<K, V>
where
    K: JsCast,
    V: JsCast,
{
    pub fn new(obj: Object) -> Self {
        JsKVObj {
            obj,
            _marker1: PhantomData::default(),
            _marker2: PhantomData::default(),
        }
    }

    pub fn get(&self, key: K) -> V {
        Object::get_own_property_descriptor(&self.obj, key.as_ref()).unchecked_into()
    }

    pub fn set(&self, key: K, val: V) {
        Object::define_property(&self.obj, key.as_ref(), val.unchecked_ref());
    }

    pub fn entries<const N: usize>(&self) -> SmallVec<[(K, V); N]> {
        let arr = Object::entries(&self.obj);
        let mut buffer = SmallVec::with_capacity(arr.length() as usize);

        buffer.extend(arr.iter().map(|val| {
            let arr = val.unchecked_into::<Array>();
            (
                arr.get(0).unchecked_into::<K>(),
                arr.get(1).unchecked_into::<V>(),
            )
        }));

        buffer
    }

    pub fn entries_map(&self) -> AHashMap<K, V>
    where
        K: Hash + Eq,
    {
        let arr = Object::entries(&self.obj);
        let mut entries = AHashMap::with_capacity(arr.length() as usize);

        arr.iter().for_each(|val| {
            let arr = val.unchecked_into::<Array>();
            entries.insert(
                arr.get(0).unchecked_into::<K>(),
                arr.get(1).unchecked_into::<V>(),
            );
        });

        entries
    }
}

impl<K, V> WasmDescribe for JsKVObj<K, V>
where
    K: FromWasmAbi,
    V: FromWasmAbi,
{
    fn describe() {
        <Object as WasmDescribe>::describe()
    }
}

impl<K, V> FromWasmAbi for JsKVObj<K, V>
where
    K: FromWasmAbi + JsCast,
    V: FromWasmAbi + JsCast,
{
    type Abi = <Object as FromWasmAbi>::Abi;

    unsafe fn from_abi(js: Self::Abi) -> Self {
        JsKVObj::new(<Object as FromWasmAbi>::from_abi(js))
    }
}
