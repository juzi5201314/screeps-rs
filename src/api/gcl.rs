use crate::api::raw;
use crate::api::raw::FromRaw;

pub struct Gcl(raw::gcl::Gcl);

impl Gcl {
    #[inline]
    pub fn level(&self) -> u32 {
        self.0.level()
    }

    #[inline]
    pub fn progress(&self) -> u32 {
        self.0.progress()
    }

    #[inline]
    pub fn progress_total(&self) -> u32 {
        self.0.progress_total()
    }
}

impl FromRaw for Gcl {
    type Raw = raw::gcl::Gcl;

    fn from_raw(raw: Self::Raw) -> Self {
        Gcl(raw)
    }
}
