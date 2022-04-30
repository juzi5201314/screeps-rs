pub mod constants;
pub mod cpu;
pub mod game;
pub mod room;
pub mod gcl;

pub trait FromRaw {
    type Raw;

    fn from_raw(raw: Self::Raw) -> Self;
}
