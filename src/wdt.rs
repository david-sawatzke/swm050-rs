#[doc = r"Register block"]
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
#[doc = "CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`read()` method returns [cr::R](cr::R) reader structure"]
impl crate::Readable for CR {}
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "CR"]
pub mod cr;
#[doc = "TORR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [torr](torr) module"]
pub type TORR = crate::Reg<u32, _TORR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TORR;
#[doc = "`read()` method returns [torr::R](torr::R) reader structure"]
impl crate::Readable for TORR {}
#[doc = "`write(|w| ..)` method takes [torr::W](torr::W) writer structure"]
impl crate::Writable for TORR {}
#[doc = "TORR"]
pub mod torr;
#[doc = "CCVR\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ccvr](ccvr) module"]
pub type CCVR = crate::Reg<u32, _CCVR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCVR;
#[doc = "`read()` method returns [ccvr::R](ccvr::R) reader structure"]
impl crate::Readable for CCVR {}
#[doc = "CCVR"]
pub mod ccvr;
#[doc = "Counter Restart Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [crr](crr) module"]
pub type CRR = crate::Reg<u32, _CRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRR;
#[doc = "`write(|w| ..)` method takes [crr::W](crr::W) writer structure"]
impl crate::Writable for CRR {}
#[doc = "Counter Restart Register"]
pub mod crr;
#[doc = "STAT\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [stat](stat) module"]
pub type STAT = crate::Reg<u32, _STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT;
#[doc = "`read()` method returns [stat::R](stat::R) reader structure"]
impl crate::Readable for STAT {}
#[doc = "STAT"]
pub mod stat;
#[doc = "ICLR\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iclr](iclr) module"]
pub type ICLR = crate::Reg<u32, _ICLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICLR;
#[doc = "`read()` method returns [iclr::R](iclr::R) reader structure"]
impl crate::Readable for ICLR {}
#[doc = "ICLR"]
pub mod iclr;
