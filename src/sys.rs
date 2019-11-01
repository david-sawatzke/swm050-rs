#[doc = r"Register block"]
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
#[doc = "CLK_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clk_cfg](clk_cfg) module"]
pub type CLK_CFG = crate::Reg<u32, _CLK_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_CFG;
#[doc = "`read()` method returns [clk_cfg::R](clk_cfg::R) reader structure"]
impl crate::Readable for CLK_CFG {}
#[doc = "`write(|w| ..)` method takes [clk_cfg::W](clk_cfg::W) writer structure"]
impl crate::Writable for CLK_CFG {}
#[doc = "CLK_CFG"]
pub mod clk_cfg;
#[doc = "PCLK_EN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pclk_en](pclk_en) module"]
pub type PCLK_EN = crate::Reg<u32, _PCLK_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCLK_EN;
#[doc = "`read()` method returns [pclk_en::R](pclk_en::R) reader structure"]
impl crate::Readable for PCLK_EN {}
#[doc = "`write(|w| ..)` method takes [pclk_en::W](pclk_en::W) writer structure"]
impl crate::Writable for PCLK_EN {}
#[doc = "PCLK_EN"]
pub mod pclk_en;
#[doc = "DBL_IRC\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dbl_irc](dbl_irc) module"]
pub type DBL_IRC = crate::Reg<u32, _DBL_IRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DBL_IRC;
#[doc = "`read()` method returns [dbl_irc::R](dbl_irc::R) reader structure"]
impl crate::Readable for DBL_IRC {}
#[doc = "`write(|w| ..)` method takes [dbl_irc::W](dbl_irc::W) writer structure"]
impl crate::Writable for DBL_IRC {}
#[doc = "DBL_IRC"]
pub mod dbl_irc;
#[doc = "OSC_DIS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [osc_dis](osc_dis) module"]
pub type OSC_DIS = crate::Reg<u32, _OSC_DIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSC_DIS;
#[doc = "`read()` method returns [osc_dis::R](osc_dis::R) reader structure"]
impl crate::Readable for OSC_DIS {}
#[doc = "`write(|w| ..)` method takes [osc_dis::W](osc_dis::W) writer structure"]
impl crate::Writable for OSC_DIS {}
#[doc = "OSC_DIS"]
pub mod osc_dis;
