#[doc = "Reader of register CPOUTPUT"]
pub type R = crate::R<u32, super::CPOUTPUT>;
#[doc = "Writer for register CPOUTPUT"]
pub type W = crate::W<u32, super::CPOUTPUT>;
#[doc = "Register CPOUTPUT `reset()`'s with value 0"]
impl crate::ResetValue for super::CPOUTPUT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CPOUTRAW`"]
pub type CPOUTRAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CPOUTRAW`"]
pub struct CPOUTRAW_W<'a> {
    w: &'a mut W,
}
impl<'a> CPOUTRAW_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `CPOUTMOS`"]
pub type CPOUTMOS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CPOUTMOS`"]
pub struct CPOUTMOS_W<'a> {
    w: &'a mut W,
}
impl<'a> CPOUTMOS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `CPOUTCEL`"]
pub type CPOUTCEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CPOUTCEL`"]
pub struct CPOUTCEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CPOUTCEL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - CPOUTRAW"]
    #[inline(always)]
    pub fn cpoutraw(&self) -> CPOUTRAW_R {
        CPOUTRAW_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CPOUTMOS"]
    #[inline(always)]
    pub fn cpoutmos(&self) -> CPOUTMOS_R {
        CPOUTMOS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CPOUTCEL"]
    #[inline(always)]
    pub fn cpoutcel(&self) -> CPOUTCEL_R {
        CPOUTCEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPOUTRAW"]
    #[inline(always)]
    pub fn cpoutraw(&mut self) -> CPOUTRAW_W {
        CPOUTRAW_W { w: self }
    }
    #[doc = "Bit 1 - CPOUTMOS"]
    #[inline(always)]
    pub fn cpoutmos(&mut self) -> CPOUTMOS_W {
        CPOUTMOS_W { w: self }
    }
    #[doc = "Bit 2 - CPOUTCEL"]
    #[inline(always)]
    pub fn cpoutcel(&mut self) -> CPOUTCEL_W {
        CPOUTCEL_W { w: self }
    }
}
