use crate::api::raw;
use crate::api::raw::FromRaw;
use std::fmt::{Debug, Formatter};

pub struct Room(raw::room::Room);

impl Room {
    #[inline]
    pub fn name(&self) -> String {
        self.0.name().as_string().unwrap()
    }

    #[inline]
    pub fn energy_available(&self) -> u32 {
        self.0.energy_available()
    }

    #[inline]
    pub fn energy_capacity_available(&self) -> u32 {
        self.0.energy_capacity_available()
    }
}

impl FromRaw for Room {
    type Raw = raw::room::Room;

    fn from_raw(raw: Self::Raw) -> Self {
        Room(raw)
    }
}

impl Debug for Room {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Room")
            .field("name", &self.name())
            .field("energy_available", &self.energy_available())
            .field(
                "energy_capacity_available",
                &self.energy_capacity_available(),
            )
            .finish()
    }
}
