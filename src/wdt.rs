#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CR"]
    pub cr: CR,
    #[doc = "0x04 - TORR"]
    pub torr: TORR,
    #[doc = "0x08 - CCVR"]
    pub ccvr: CCVR,
    #[doc = "0x0c - Counter Restart Register"]
    pub crr: CRR,
    #[doc = "0x10 - STAT"]
    pub stat: STAT,
    #[doc = "0x14 - ICLR"]
    pub iclr: ICLR,
}
#[doc = "CR"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CR"]
pub mod cr;
#[doc = "TORR"]
pub struct TORR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TORR"]
pub mod torr;
#[doc = "CCVR"]
pub struct CCVR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CCVR"]
pub mod ccvr;
#[doc = "Counter Restart Register"]
pub struct CRR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter Restart Register"]
pub mod crr;
#[doc = "STAT"]
pub struct STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "STAT"]
pub mod stat;
#[doc = "ICLR"]
pub struct ICLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ICLR"]
pub mod iclr;
