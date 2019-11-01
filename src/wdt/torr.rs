#[doc = "Reader of register TORR"]
pub type R = crate::R<u32, super::TORR>;
#[doc = "Writer for register TORR"]
pub type W = crate::W<u32, super::TORR>;
#[doc = "Register TORR `reset()`'s with value 0"]
impl crate::ResetValue for super::TORR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TOP`"]
pub type TOP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TOP`"]
pub struct TOP_W<'a> {
    w: &'a mut W,
}
impl<'a> TOP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `TOP_INIT`"]
pub type TOP_INIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TOP_INIT`"]
pub struct TOP_INIT_W<'a> {
    w: &'a mut W,
}
impl<'a> TOP_INIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Initial value (write value before WDT enable) 2^(8+TOP), 24 bits"]
    #[inline(always)]
    pub fn top(&self) -> TOP_R {
        TOP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - The value to be filled after timeout"]
    #[inline(always)]
    pub fn top_init(&self) -> TOP_INIT_R {
        TOP_INIT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Initial value (write value before WDT enable) 2^(8+TOP), 24 bits"]
    #[inline(always)]
    pub fn top(&mut self) -> TOP_W {
        TOP_W { w: self }
    }
    #[doc = "Bits 4:7 - The value to be filled after timeout"]
    #[inline(always)]
    pub fn top_init(&mut self) -> TOP_INIT_W {
        TOP_INIT_W { w: self }
    }
}
