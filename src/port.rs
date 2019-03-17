#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SWD_SEL"]
    pub swd_sel: SWD_SEL,
    _reserved0: [u8; 76usize],
    #[doc = "0x50 - PORTA_SEL"]
    pub porta_sel: PORTA_SEL,
    _reserved1: [u8; 12usize],
    #[doc = "0x60 - PORTA_PULLU"]
    pub porta_pullu: PORTA_PULLU,
    _reserved2: [u8; 76usize],
    #[doc = "0xb0 - PORTA_INDIS"]
    pub porta_indis: PORTA_INDIS,
}
#[doc = "SWD_SEL"]
pub struct SWD_SEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SWD_SEL"]
pub mod swd_sel;
#[doc = "PORTA_SEL"]
pub struct PORTA_SEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PORTA_SEL"]
pub mod porta_sel;
#[doc = "PORTA_PULLU"]
pub struct PORTA_PULLU {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PORTA_PULLU"]
pub mod porta_pullu;
#[doc = "PORTA_INDIS"]
pub struct PORTA_INDIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PORTA_INDIS"]
pub mod porta_indis;
