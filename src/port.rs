#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SWD_SEL"]
    pub swd_sel: SWD_SEL,
    _reserved1: [u8; 76usize],
    #[doc = "0x50 - PORTA_SEL"]
    pub porta_sel: PORTA_SEL,
    _reserved2: [u8; 12usize],
    #[doc = "0x60 - PORTA_PULLU"]
    pub porta_pullu: PORTA_PULLU,
    _reserved3: [u8; 76usize],
    #[doc = "0xb0 - PORTA_INDIS"]
    pub porta_indis: PORTA_INDIS,
}
#[doc = "SWD_SEL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [swd_sel](swd_sel) module"]
pub type SWD_SEL = crate::Reg<u32, _SWD_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWD_SEL;
#[doc = "`read()` method returns [swd_sel::R](swd_sel::R) reader structure"]
impl crate::Readable for SWD_SEL {}
#[doc = "`write(|w| ..)` method takes [swd_sel::W](swd_sel::W) writer structure"]
impl crate::Writable for SWD_SEL {}
#[doc = "SWD_SEL"]
pub mod swd_sel;
#[doc = "PORTA_SEL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [porta_sel](porta_sel) module"]
pub type PORTA_SEL = crate::Reg<u32, _PORTA_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PORTA_SEL;
#[doc = "`read()` method returns [porta_sel::R](porta_sel::R) reader structure"]
impl crate::Readable for PORTA_SEL {}
#[doc = "`write(|w| ..)` method takes [porta_sel::W](porta_sel::W) writer structure"]
impl crate::Writable for PORTA_SEL {}
#[doc = "PORTA_SEL"]
pub mod porta_sel;
#[doc = "PORTA_PULLU\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [porta_pullu](porta_pullu) module"]
pub type PORTA_PULLU = crate::Reg<u32, _PORTA_PULLU>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PORTA_PULLU;
#[doc = "`read()` method returns [porta_pullu::R](porta_pullu::R) reader structure"]
impl crate::Readable for PORTA_PULLU {}
#[doc = "`write(|w| ..)` method takes [porta_pullu::W](porta_pullu::W) writer structure"]
impl crate::Writable for PORTA_PULLU {}
#[doc = "PORTA_PULLU"]
pub mod porta_pullu;
#[doc = "PORTA_INDIS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [porta_indis](porta_indis) module"]
pub type PORTA_INDIS = crate::Reg<u32, _PORTA_INDIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PORTA_INDIS;
#[doc = "`read()` method returns [porta_indis::R](porta_indis::R) reader structure"]
impl crate::Readable for PORTA_INDIS {}
#[doc = "`write(|w| ..)` method takes [porta_indis::W](porta_indis::W) writer structure"]
impl crate::Writable for PORTA_INDIS {}
#[doc = "PORTA_INDIS"]
pub mod porta_indis;
