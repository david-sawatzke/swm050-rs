#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DAT"]
    pub dat: DAT,
    #[doc = "0x04 - DIR"]
    pub dir: DIR,
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
