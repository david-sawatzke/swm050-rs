#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CPEN"]
    pub cpen: CPEN,
    #[doc = "0x04 - CPLEVEL"]
    pub cplevel: CPLEVEL,
    _reserved2: [u8; 8usize],
    #[doc = "0x10 - CPOUTPUT"]
    pub cpoutput: CPOUTPUT,
    _reserved3: [u8; 12usize],
    #[doc = "0x20 - CPMOD"]
    pub cpmod: CPMOD,
    #[doc = "0x24 - CPCTL"]
    pub cpctl: CPCTL,
    #[doc = "0x28 - CLRLOCK"]
    pub clrlock: CLRLOCK,
    #[doc = "0x2c - LOCKCON"]
    pub lockcon: LOCKCON,
    _reserved7: [u8; 16usize],
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
#[doc = "CPEN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cpen](cpen) module"]
pub type CPEN = crate::Reg<u32, _CPEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPEN;
#[doc = "`read()` method returns [cpen::R](cpen::R) reader structure"]
impl crate::Readable for CPEN {}
#[doc = "`write(|w| ..)` method takes [cpen::W](cpen::W) writer structure"]
impl crate::Writable for CPEN {}
#[doc = "CPEN"]
pub mod cpen;
#[doc = "CPLEVEL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cplevel](cplevel) module"]
pub type CPLEVEL = crate::Reg<u32, _CPLEVEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPLEVEL;
#[doc = "`read()` method returns [cplevel::R](cplevel::R) reader structure"]
impl crate::Readable for CPLEVEL {}
#[doc = "`write(|w| ..)` method takes [cplevel::W](cplevel::W) writer structure"]
impl crate::Writable for CPLEVEL {}
#[doc = "CPLEVEL"]
pub mod cplevel;
#[doc = "CPOUTPUT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cpoutput](cpoutput) module"]
pub type CPOUTPUT = crate::Reg<u32, _CPOUTPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPOUTPUT;
#[doc = "`read()` method returns [cpoutput::R](cpoutput::R) reader structure"]
impl crate::Readable for CPOUTPUT {}
#[doc = "`write(|w| ..)` method takes [cpoutput::W](cpoutput::W) writer structure"]
impl crate::Writable for CPOUTPUT {}
#[doc = "CPOUTPUT"]
pub mod cpoutput;
#[doc = "CPMOD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cpmod](cpmod) module"]
pub type CPMOD = crate::Reg<u32, _CPMOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPMOD;
#[doc = "`read()` method returns [cpmod::R](cpmod::R) reader structure"]
impl crate::Readable for CPMOD {}
#[doc = "`write(|w| ..)` method takes [cpmod::W](cpmod::W) writer structure"]
impl crate::Writable for CPMOD {}
#[doc = "CPMOD"]
pub mod cpmod;
#[doc = "CPCTL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cpctl](cpctl) module"]
pub type CPCTL = crate::Reg<u32, _CPCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPCTL;
#[doc = "`read()` method returns [cpctl::R](cpctl::R) reader structure"]
impl crate::Readable for CPCTL {}
#[doc = "`write(|w| ..)` method takes [cpctl::W](cpctl::W) writer structure"]
impl crate::Writable for CPCTL {}
#[doc = "CPCTL"]
pub mod cpctl;
#[doc = "CLRLOCK\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clrlock](clrlock) module"]
pub type CLRLOCK = crate::Reg<u32, _CLRLOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLRLOCK;
#[doc = "`read()` method returns [clrlock::R](clrlock::R) reader structure"]
impl crate::Readable for CLRLOCK {}
#[doc = "`write(|w| ..)` method takes [clrlock::W](clrlock::W) writer structure"]
impl crate::Writable for CLRLOCK {}
#[doc = "CLRLOCK"]
pub mod clrlock;
#[doc = "LOCKCON\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lockcon](lockcon) module"]
pub type LOCKCON = crate::Reg<u32, _LOCKCON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOCKCON;
#[doc = "`read()` method returns [lockcon::R](lockcon::R) reader structure"]
impl crate::Readable for LOCKCON {}
#[doc = "`write(|w| ..)` method takes [lockcon::W](lockcon::W) writer structure"]
impl crate::Writable for LOCKCON {}
#[doc = "LOCKCON"]
pub mod lockcon;
#[doc = "CPINTCON\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cpintcon](cpintcon) module"]
pub type CPINTCON = crate::Reg<u32, _CPINTCON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPINTCON;
#[doc = "`read()` method returns [cpintcon::R](cpintcon::R) reader structure"]
impl crate::Readable for CPINTCON {}
#[doc = "`write(|w| ..)` method takes [cpintcon::W](cpintcon::W) writer structure"]
impl crate::Writable for CPINTCON {}
#[doc = "CPINTCON"]
pub mod cpintcon;
#[doc = "CPINTRAW\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cpintraw](cpintraw) module"]
pub type CPINTRAW = crate::Reg<u32, _CPINTRAW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPINTRAW;
#[doc = "`read()` method returns [cpintraw::R](cpintraw::R) reader structure"]
impl crate::Readable for CPINTRAW {}
#[doc = "`write(|w| ..)` method takes [cpintraw::W](cpintraw::W) writer structure"]
impl crate::Writable for CPINTRAW {}
#[doc = "CPINTRAW"]
pub mod cpintraw;
#[doc = "CPINT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cpint](cpint) module"]
pub type CPINT = crate::Reg<u32, _CPINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPINT;
#[doc = "`read()` method returns [cpint::R](cpint::R) reader structure"]
impl crate::Readable for CPINT {}
#[doc = "`write(|w| ..)` method takes [cpint::W](cpint::W) writer structure"]
impl crate::Writable for CPINT {}
#[doc = "CPINT"]
pub mod cpint;
#[doc = "CPINTOF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cpintof](cpintof) module"]
pub type CPINTOF = crate::Reg<u32, _CPINTOF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPINTOF;
#[doc = "`read()` method returns [cpintof::R](cpintof::R) reader structure"]
impl crate::Readable for CPINTOF {}
#[doc = "`write(|w| ..)` method takes [cpintof::W](cpintof::W) writer structure"]
impl crate::Writable for CPINTOF {}
#[doc = "CPINTOF"]
pub mod cpintof;
#[doc = "CPINTCLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cpintclr](cpintclr) module"]
pub type CPINTCLR = crate::Reg<u32, _CPINTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPINTCLR;
#[doc = "`read()` method returns [cpintclr::R](cpintclr::R) reader structure"]
impl crate::Readable for CPINTCLR {}
#[doc = "`write(|w| ..)` method takes [cpintclr::W](cpintclr::W) writer structure"]
impl crate::Writable for CPINTCLR {}
#[doc = "CPINTCLR"]
pub mod cpintclr;
