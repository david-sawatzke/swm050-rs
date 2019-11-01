#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DAT"]
    pub dat: DAT,
    #[doc = "0x04 - DIR"]
    pub dir: DIR,
    _reserved2: [u8; 40usize],
    #[doc = "0x30 - INTEN"]
    pub inten: INTEN,
    #[doc = "0x34 - INTMSK"]
    pub intmsk: INTMSK,
    #[doc = "0x38 - INTMODE"]
    pub intmode: INTMODE,
    #[doc = "0x3c - INTLEVEL"]
    pub intlevel: INTLEVEL,
    #[doc = "0x40 - INTSTAT"]
    pub intstat: INTSTAT,
    #[doc = "0x44 - INTRAWS"]
    pub intraws: INTRAWS,
    _reserved8: [u8; 4usize],
    #[doc = "0x4c - INTCLR"]
    pub intclr: INTCLR,
    #[doc = "0x50 - EXT"]
    pub ext: EXT,
}
#[doc = "DAT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dat](dat) module"]
pub type DAT = crate::Reg<u32, _DAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAT;
#[doc = "`read()` method returns [dat::R](dat::R) reader structure"]
impl crate::Readable for DAT {}
#[doc = "`write(|w| ..)` method takes [dat::W](dat::W) writer structure"]
impl crate::Writable for DAT {}
#[doc = "DAT"]
pub mod dat;
#[doc = "DIR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dir](dir) module"]
pub type DIR = crate::Reg<u32, _DIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIR;
#[doc = "`read()` method returns [dir::R](dir::R) reader structure"]
impl crate::Readable for DIR {}
#[doc = "`write(|w| ..)` method takes [dir::W](dir::W) writer structure"]
impl crate::Writable for DIR {}
#[doc = "DIR"]
pub mod dir;
#[doc = "INTEN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [inten](inten) module"]
pub type INTEN = crate::Reg<u32, _INTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTEN;
#[doc = "`read()` method returns [inten::R](inten::R) reader structure"]
impl crate::Readable for INTEN {}
#[doc = "`write(|w| ..)` method takes [inten::W](inten::W) writer structure"]
impl crate::Writable for INTEN {}
#[doc = "INTEN"]
pub mod inten;
#[doc = "INTMSK\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intmsk](intmsk) module"]
pub type INTMSK = crate::Reg<u32, _INTMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTMSK;
#[doc = "`read()` method returns [intmsk::R](intmsk::R) reader structure"]
impl crate::Readable for INTMSK {}
#[doc = "`write(|w| ..)` method takes [intmsk::W](intmsk::W) writer structure"]
impl crate::Writable for INTMSK {}
#[doc = "INTMSK"]
pub mod intmsk;
#[doc = "INTMODE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intmode](intmode) module"]
pub type INTMODE = crate::Reg<u32, _INTMODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTMODE;
#[doc = "`read()` method returns [intmode::R](intmode::R) reader structure"]
impl crate::Readable for INTMODE {}
#[doc = "`write(|w| ..)` method takes [intmode::W](intmode::W) writer structure"]
impl crate::Writable for INTMODE {}
#[doc = "INTMODE"]
pub mod intmode;
#[doc = "INTLEVEL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intlevel](intlevel) module"]
pub type INTLEVEL = crate::Reg<u32, _INTLEVEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTLEVEL;
#[doc = "`read()` method returns [intlevel::R](intlevel::R) reader structure"]
impl crate::Readable for INTLEVEL {}
#[doc = "`write(|w| ..)` method takes [intlevel::W](intlevel::W) writer structure"]
impl crate::Writable for INTLEVEL {}
#[doc = "INTLEVEL"]
pub mod intlevel;
#[doc = "INTSTAT\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intstat](intstat) module"]
pub type INTSTAT = crate::Reg<u32, _INTSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTSTAT;
#[doc = "`read()` method returns [intstat::R](intstat::R) reader structure"]
impl crate::Readable for INTSTAT {}
#[doc = "INTSTAT"]
pub mod intstat;
#[doc = "INTRAWS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intraws](intraws) module"]
pub type INTRAWS = crate::Reg<u32, _INTRAWS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTRAWS;
#[doc = "`read()` method returns [intraws::R](intraws::R) reader structure"]
impl crate::Readable for INTRAWS {}
#[doc = "INTRAWS"]
pub mod intraws;
#[doc = "INTCLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intclr](intclr) module"]
pub type INTCLR = crate::Reg<u32, _INTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTCLR;
#[doc = "`read()` method returns [intclr::R](intclr::R) reader structure"]
impl crate::Readable for INTCLR {}
#[doc = "`write(|w| ..)` method takes [intclr::W](intclr::W) writer structure"]
impl crate::Writable for INTCLR {}
#[doc = "INTCLR"]
pub mod intclr;
#[doc = "EXT\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ext](ext) module"]
pub type EXT = crate::Reg<u32, _EXT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXT;
#[doc = "`read()` method returns [ext::R](ext::R) reader structure"]
impl crate::Readable for EXT {}
#[doc = "EXT"]
pub mod ext;
