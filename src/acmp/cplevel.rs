#[doc = "Reader of register CPLEVEL"]
pub type R = crate::R<u32, super::CPLEVEL>;
#[doc = "Writer for register CPLEVEL"]
pub type W = crate::W<u32, super::CPLEVEL>;
#[doc = "Register CPLEVEL `reset()`'s with value 0"]
impl crate::ResetValue for super::CPLEVEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LEVEL`"]
pub type LEVEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LEVEL`"]
pub struct LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LEVEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - LEVEL"]
    #[inline(always)]
    pub fn level(&self) -> LEVEL_R {
        LEVEL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - LEVEL"]
    #[inline(always)]
    pub fn level(&mut self) -> LEVEL_W {
        LEVEL_W { w: self }
    }
}
