pub use room_object::room::Room;

pub mod game;
mod room_object;

macro def_attr {
    ($name:ident, $rty:ty) => {
        #[inline]
        pub fn $name(&self) -> $rty {
            self.0.$name()
        }
    }
}
