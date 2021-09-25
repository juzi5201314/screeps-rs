use effect::Effects;

use crate::Room;
use crate::types::room_object::position::Position;

pub mod room;
mod effect;
mod position;

pub trait RoomObject {
    fn effects(&self) -> Effects;
    fn pos(&self) -> Position;
    fn room(&self) -> Option<Room>;
}
