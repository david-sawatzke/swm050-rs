#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - INTEN"]
    pub inten: INTEN,
    #[doc = "0x04 - INTMSK"]
    pub intmsk: INTMSK,
    #[doc = "0x08 - INTMODE"]
    pub intmode: INTMODE,
    #[doc = "0x0c - INTLEVEL"]
    pub intlevel: INTLEVEL,
    #[doc = "0x10 - INTSTAT"]
    pub intstat: INTSTAT,
    #[doc = "0x14 - INTRAWS"]
    pub intraws: INTRAWS,
    _reserved0: [u8; 4usize],
    #[doc = "0x1c - INTCLR"]
    pub intclr: INTCLR,
}
#[doc = "INTEN"]
pub struct INTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "INTEN"]
pub mod inten;
#[doc = "INTMSK"]
pub struct INTMSK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "INTMSK"]
pub mod intmsk;
#[doc = "INTMODE"]
pub struct INTMODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "INTMODE"]
pub mod intmode;
#[doc = "INTLEVEL"]
pub struct INTLEVEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "INTLEVEL"]
pub mod intlevel;
#[doc = "INTSTAT"]
pub struct INTSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "INTSTAT"]
pub mod intstat;
#[doc = "INTRAWS"]
pub struct INTRAWS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "INTRAWS"]
pub mod intraws;
#[doc = "INTCLR"]
pub struct INTCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "INTCLR"]
pub mod intclr;
