#[doc = "Writer for register CRR"]
pub type W = crate::W<u32, super::CRR>;
#[doc = "Register CRR `reset()`'s with value 0"]
impl crate::ResetValue for super::CRR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRR_AW {
    #[doc = "118: Reset WDT counter"]
    RESET,
}
impl From<CRR_AW> for u8 {
    #[inline(always)]
    fn from(variant: CRR_AW) -> Self {
        match variant {
            CRR_AW::RESET => 118,
        }
    }
}
#[doc = "Write proxy for field `CRR`"]
pub struct CRR_W<'a> {
    w: &'a mut W,
}
impl<'a> CRR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRR_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Reset WDT counter"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CRR_AW::RESET)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn crr(&mut self) -> CRR_W {
        CRR_W { w: self }
    }
}
