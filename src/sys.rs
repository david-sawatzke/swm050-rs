#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CLK_CFG"]
    pub clk_cfg: CLK_CFG,
    #[doc = "0x04 - PCLK_EN"]
    pub pclk_en: PCLK_EN,
    #[doc = "0x08 - DBL_IRC"]
    pub dbl_irc: DBL_IRC,
    #[doc = "0x0c - OSC_DIS"]
    pub osc_dis: OSC_DIS,
}
#[doc = "CLK_CFG"]
pub struct CLK_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_CFG"]
pub mod clk_cfg;
#[doc = "PCLK_EN"]
pub struct PCLK_EN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PCLK_EN"]
pub mod pclk_en;
#[doc = "DBL_IRC"]
pub struct DBL_IRC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DBL_IRC"]
pub mod dbl_irc;
#[doc = "OSC_DIS"]
pub struct OSC_DIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OSC_DIS"]
pub mod osc_dis;
