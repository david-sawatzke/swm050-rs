#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CPEN"]
    pub cpen: CPEN,
    #[doc = "0x04 - CPLEVEL"]
    pub cplevel: CPLEVEL,
    _reserved0: [u8; 8usize],
    #[doc = "0x10 - CPOUTPUT"]
    pub cpoutput: CPOUTPUT,
    _reserved1: [u8; 12usize],
    #[doc = "0x20 - CPMOD"]
    pub cpmod: CPMOD,
    #[doc = "0x24 - CPCTL"]
    pub cpctl: CPCTL,
    #[doc = "0x28 - CLRLOCK"]
    pub clrlock: CLRLOCK,
    #[doc = "0x2c - LOCKCON"]
    pub lockcon: LOCKCON,
    _reserved2: [u8; 16usize],
    #[doc = "0x40 - CPINTCON"]
    pub cpintcon: CPINTCON,
    #[doc = "0x44 - CPINTRAW"]
    pub cpintraw: CPINTRAW,
    #[doc = "0x48 - CPINT"]
    pub cpint: CPINT,
    #[doc = "0x4c - CPINTOF"]
    pub cpintof: CPINTOF,
    #[doc = "0x50 - CPINTCLR"]
    pub cpintclr: CPINTCLR,
}
#[doc = "CPEN"]
pub struct CPEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPEN"]
pub mod cpen;
#[doc = "CPLEVEL"]
pub struct CPLEVEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPLEVEL"]
pub mod cplevel;
#[doc = "CPOUTPUT"]
pub struct CPOUTPUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPOUTPUT"]
pub mod cpoutput;
#[doc = "CPMOD"]
pub struct CPMOD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPMOD"]
pub mod cpmod;
#[doc = "CPCTL"]
pub struct CPCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPCTL"]
pub mod cpctl;
#[doc = "CLRLOCK"]
pub struct CLRLOCK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLRLOCK"]
pub mod clrlock;
#[doc = "LOCKCON"]
pub struct LOCKCON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LOCKCON"]
pub mod lockcon;
#[doc = "CPINTCON"]
pub struct CPINTCON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPINTCON"]
pub mod cpintcon;
#[doc = "CPINTRAW"]
pub struct CPINTRAW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPINTRAW"]
pub mod cpintraw;
#[doc = "CPINT"]
pub struct CPINT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPINT"]
pub mod cpint;
#[doc = "CPINTOF"]
pub struct CPINTOF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPINTOF"]
pub mod cpintof;
#[doc = "CPINTCLR"]
pub struct CPINTCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPINTCLR"]
pub mod cpintclr;
