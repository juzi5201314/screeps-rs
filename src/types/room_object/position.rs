use std::convert::TryInto;

use wasm_bindgen::prelude::wasm_bindgen;

use crate::types::def_attr;
use crate::{BuildableStructure, Color, JsString, ResCode};

#[wasm_bindgen]
extern "C" {
    pub(crate) type RoomPosition;

    #[wasm_bindgen(constructor)]
    pub fn new(x: u32, y: u32, room_name: &str) -> RoomPosition;

    #[wasm_bindgen(method, getter = roomName)]
    pub fn room_name(pos: &RoomPosition) -> JsString;

    #[wasm_bindgen(method, getter)]
    pub fn x(pos: &RoomPosition) -> u32;

    #[wasm_bindgen(method, getter)]
    pub fn y(pos: &RoomPosition) -> u32;

    #[wasm_bindgen(method, js_name = createConstructionSite)]
    pub fn create_construction_site1(pos: &RoomPosition, ty: &str) -> i8;

    #[wasm_bindgen(method, js_name = createConstructionSite)]
    pub fn create_construction_site2(pos: &RoomPosition, ty: &str, name: &str) -> i8;

    #[wasm_bindgen(method, js_name = createFlag)]
    pub fn create_flag0(pos: &RoomPosition) -> i8;

    #[wasm_bindgen(method, js_name = createFlag)]
    pub fn create_flag1(pos: &RoomPosition, name: &str) -> i8;

    #[wasm_bindgen(method, js_name = createFlag)]
    pub fn create_flag2(pos: &RoomPosition, name: &str, color: u8) -> i8;

    #[wasm_bindgen(method, js_name = createFlag)]
    pub fn create_flag3(pos: &RoomPosition, name: &str, color: u8, secondary_color: u8) -> i8;

    #[wasm_bindgen(method, js_name = createFlag)]
    pub fn create_flag4(pos: &RoomPosition, color: u8, secondary_color: u8) -> i8;

    #[wasm_bindgen(method, js_name = createFlag)]
    pub fn create_flag5(pos: &RoomPosition, name: &str, secondary_color: u8) -> i8;

    #[wasm_bindgen(method, js_name = createFlag)]
    pub fn create_flag6(pos: &RoomPosition, color: u8) -> i8;

    #[wasm_bindgen(method, js_name = createFlag)]
    pub fn create_flag7(pos: &RoomPosition, secondary_color: u8) -> i8;
}

pub struct Position(pub(crate) RoomPosition);

impl Position {
    #[inline]
    pub fn new(x: u32, y: u32, room_name: &str) -> Self {
        Position(RoomPosition::new(x, y, room_name))
    }

    def_attr!(room_name, JsString);
    def_attr!(x, u32);
    def_attr!(y, u32);

    #[inline]
    pub fn create_construction_site(&self, ty: BuildableStructure) -> ResCode {
        self.0
            .create_construction_site1(ty.into())
            .try_into()
            .unwrap()
    }

    #[inline]
    pub fn create_spawn(&self, name: &str) -> ResCode {
        self.0
            .create_construction_site2(BuildableStructure::STRUCTURE_SPAWN.into(), name)
            .try_into()
            .unwrap()
    }

    #[inline]
    pub fn create_flag(
        &self,
        name: Option<&str>,
        color: Option<Color>,
        secondary_color: Option<Color>,
    ) -> ResCode {
        match name {
            None => match color {
                None => match secondary_color {
                    None => self.0.create_flag0(),
                    Some(color) => self.0.create_flag7(color as u8),
                },
                Some(color) => match secondary_color {
                    None => self.0.create_flag6(color as u8),
                    Some(s_color) => self.0.create_flag4(color as u8, s_color as u8),
                },
            },
            Some(name) => match color {
                None => match secondary_color {
                    None => self.0.create_flag1(name),
                    Some(color) => self.0.create_flag5(name, color as u8),
                },
                Some(color) => match secondary_color {
                    None => self.0.create_flag2(name, color as u8),
                    Some(s_color) => self.0.create_flag3(name, color as u8, s_color as u8),
                },
            },
        }
        .try_into()
        .unwrap()
    }
}
