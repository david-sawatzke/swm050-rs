#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DAT"]
    pub dat: DAT,
    #[doc = "0x04 - DIR"]
    pub dir: DIR,
    _reserved0: [u8; 40usize],
    #[doc = "0x30 - INTEN"]
    pub inten: INTEN,
    #[doc = "0x34 - INTMSK"]
    pub intmsk: INTMSK,
    #[doc = "0x38 - INTMODE"]
    pub intmode: INTMODE,
    #[doc = "0x3c - INTLEVEL"]
    pub intlevel: INTLEVEL,
    #[doc = "0x40 - INTSTAT"]
    pub intstat: INTSTAT,
    #[doc = "0x44 - INTRAWS"]
    pub intraws: INTRAWS,
    _reserved1: [u8; 4usize],
    #[doc = "0x4c - INTCLR"]
    pub intclr: INTCLR,
    #[doc = "0x50 - EXT"]
    pub ext: EXT,
}
#[doc = "DAT"]
pub struct DAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DAT"]
pub mod dat;
#[doc = "DIR"]
pub struct DIR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DIR"]
pub mod dir;
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
#[doc = "EXT"]
pub struct EXT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EXT"]
pub mod ext;
