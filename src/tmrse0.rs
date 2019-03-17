#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CTRL"]
    pub ctrl: CTRL,
    #[doc = "0x04 - TARVAL"]
    pub tarval: TARVAL,
    #[doc = "0x08 - CURVAL"]
    pub curval: CURVAL,
    #[doc = "0x0c - CAPVAL"]
    pub capval: CAPVAL,
    #[doc = "0x10 - CAPHALF"]
    pub caphalf: CAPHALF,
    #[doc = "0x14 - STATE_P"]
    pub state_p: STATE_P,
    _reserved0: [u8; 104usize],
    #[doc = "0x80 - OUT_LVL"]
    pub out_lvl: OUT_LVL,
    #[doc = "0x84 - INTCTRL"]
    pub intctrl: INTCTRL,
    #[doc = "0x88 - INTRSTAT"]
    pub intrstat: INTRSTAT,
    #[doc = "0x8c - INTSTAT"]
    pub intstat: INTSTAT,
    #[doc = "0x90 - INTOFLAG"]
    pub intoflag: INTOFLAG,
}
#[doc = "CTRL"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CTRL"]
pub mod ctrl;
#[doc = "TARVAL"]
pub struct TARVAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TARVAL"]
pub mod tarval;
#[doc = "CURVAL"]
pub struct CURVAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CURVAL"]
pub mod curval;
#[doc = "CAPVAL"]
pub struct CAPVAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAPVAL"]
pub mod capval;
#[doc = "CAPHALF"]
pub struct CAPHALF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAPHALF"]
pub mod caphalf;
#[doc = "STATE_P"]
pub struct STATE_P {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "STATE_P"]
pub mod state_p;
#[doc = "OUT_LVL"]
pub struct OUT_LVL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OUT_LVL"]
pub mod out_lvl;
#[doc = "INTCTRL"]
pub struct INTCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "INTCTRL"]
pub mod intctrl;
#[doc = "INTRSTAT"]
pub struct INTRSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "INTRSTAT"]
pub mod intrstat;
#[doc = "INTSTAT"]
pub struct INTSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "INTSTAT"]
pub mod intstat;
#[doc = "INTOFLAG"]
pub struct INTOFLAG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "INTOFLAG"]
pub mod intoflag;
