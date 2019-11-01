#[doc = r"Register block"]
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
    _reserved6: [u8; 104usize],
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
#[doc = "CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "CTRL"]
pub mod ctrl;
#[doc = "TARVAL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tarval](tarval) module"]
pub type TARVAL = crate::Reg<u32, _TARVAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TARVAL;
#[doc = "`read()` method returns [tarval::R](tarval::R) reader structure"]
impl crate::Readable for TARVAL {}
#[doc = "`write(|w| ..)` method takes [tarval::W](tarval::W) writer structure"]
impl crate::Writable for TARVAL {}
#[doc = "TARVAL"]
pub mod tarval;
#[doc = "CURVAL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [curval](curval) module"]
pub type CURVAL = crate::Reg<u32, _CURVAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CURVAL;
#[doc = "`read()` method returns [curval::R](curval::R) reader structure"]
impl crate::Readable for CURVAL {}
#[doc = "`write(|w| ..)` method takes [curval::W](curval::W) writer structure"]
impl crate::Writable for CURVAL {}
#[doc = "CURVAL"]
pub mod curval;
#[doc = "CAPVAL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [capval](capval) module"]
pub type CAPVAL = crate::Reg<u32, _CAPVAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAPVAL;
#[doc = "`read()` method returns [capval::R](capval::R) reader structure"]
impl crate::Readable for CAPVAL {}
#[doc = "`write(|w| ..)` method takes [capval::W](capval::W) writer structure"]
impl crate::Writable for CAPVAL {}
#[doc = "CAPVAL"]
pub mod capval;
#[doc = "CAPHALF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [caphalf](caphalf) module"]
pub type CAPHALF = crate::Reg<u32, _CAPHALF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAPHALF;
#[doc = "`read()` method returns [caphalf::R](caphalf::R) reader structure"]
impl crate::Readable for CAPHALF {}
#[doc = "`write(|w| ..)` method takes [caphalf::W](caphalf::W) writer structure"]
impl crate::Writable for CAPHALF {}
#[doc = "CAPHALF"]
pub mod caphalf;
#[doc = "STATE_P\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [state_p](state_p) module"]
pub type STATE_P = crate::Reg<u32, _STATE_P>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATE_P;
#[doc = "`read()` method returns [state_p::R](state_p::R) reader structure"]
impl crate::Readable for STATE_P {}
#[doc = "`write(|w| ..)` method takes [state_p::W](state_p::W) writer structure"]
impl crate::Writable for STATE_P {}
#[doc = "STATE_P"]
pub mod state_p;
#[doc = "OUT_LVL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [out_lvl](out_lvl) module"]
pub type OUT_LVL = crate::Reg<u32, _OUT_LVL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUT_LVL;
#[doc = "`read()` method returns [out_lvl::R](out_lvl::R) reader structure"]
impl crate::Readable for OUT_LVL {}
#[doc = "`write(|w| ..)` method takes [out_lvl::W](out_lvl::W) writer structure"]
impl crate::Writable for OUT_LVL {}
#[doc = "OUT_LVL"]
pub mod out_lvl;
#[doc = "INTCTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intctrl](intctrl) module"]
pub type INTCTRL = crate::Reg<u32, _INTCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTCTRL;
#[doc = "`read()` method returns [intctrl::R](intctrl::R) reader structure"]
impl crate::Readable for INTCTRL {}
#[doc = "`write(|w| ..)` method takes [intctrl::W](intctrl::W) writer structure"]
impl crate::Writable for INTCTRL {}
#[doc = "INTCTRL"]
pub mod intctrl;
#[doc = "INTRSTAT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intrstat](intrstat) module"]
pub type INTRSTAT = crate::Reg<u32, _INTRSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTRSTAT;
#[doc = "`read()` method returns [intrstat::R](intrstat::R) reader structure"]
impl crate::Readable for INTRSTAT {}
#[doc = "`write(|w| ..)` method takes [intrstat::W](intrstat::W) writer structure"]
impl crate::Writable for INTRSTAT {}
#[doc = "INTRSTAT"]
pub mod intrstat;
#[doc = "INTSTAT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intstat](intstat) module"]
pub type INTSTAT = crate::Reg<u32, _INTSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTSTAT;
#[doc = "`read()` method returns [intstat::R](intstat::R) reader structure"]
impl crate::Readable for INTSTAT {}
#[doc = "`write(|w| ..)` method takes [intstat::W](intstat::W) writer structure"]
impl crate::Writable for INTSTAT {}
#[doc = "INTSTAT"]
pub mod intstat;
#[doc = "INTOFLAG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intoflag](intoflag) module"]
pub type INTOFLAG = crate::Reg<u32, _INTOFLAG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTOFLAG;
#[doc = "`read()` method returns [intoflag::R](intoflag::R) reader structure"]
impl crate::Readable for INTOFLAG {}
#[doc = "`write(|w| ..)` method takes [intoflag::W](intoflag::W) writer structure"]
impl crate::Writable for INTOFLAG {}
#[doc = "INTOFLAG"]
pub mod intoflag;
