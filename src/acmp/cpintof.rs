#[doc = "Reader of register CPINTOF"]
pub type R = crate::R<u32, super::CPINTOF>;
#[doc = "Writer for register CPINTOF"]
pub type W = crate::W<u32, super::CPINTOF>;
#[doc = "Register CPINTOF `reset()`'s with value 0"]
impl crate::ResetValue for super::CPINTOF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CPRAW`"]
pub type CPRAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CPRAW`"]
pub struct CPRAW_W<'a> {
    w: &'a mut W,
}
impl<'a> CPRAW_W<'a> {
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
#[doc = "Reader of field `CPMOS`"]
pub type CPMOS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CPMOS`"]
pub struct CPMOS_W<'a> {
    w: &'a mut W,
}
impl<'a> CPMOS_W<'a> {
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
#[doc = "Reader of field `CPCEL`"]
pub type CPCEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CPCEL`"]
pub struct CPCEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CPCEL_W<'a> {
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
    #[doc = "Bit 0 - CPRAW"]
    #[inline(always)]
    pub fn cpraw(&self) -> CPRAW_R {
        CPRAW_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CPMOS"]
    #[inline(always)]
    pub fn cpmos(&self) -> CPMOS_R {
        CPMOS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CPCEL"]
    #[inline(always)]
    pub fn cpcel(&self) -> CPCEL_R {
        CPCEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPRAW"]
    #[inline(always)]
    pub fn cpraw(&mut self) -> CPRAW_W {
        CPRAW_W { w: self }
    }
    #[doc = "Bit 1 - CPMOS"]
    #[inline(always)]
    pub fn cpmos(&mut self) -> CPMOS_W {
        CPMOS_W { w: self }
    }
    #[doc = "Bit 2 - CPCEL"]
    #[inline(always)]
    pub fn cpcel(&mut self) -> CPCEL_W {
        CPCEL_W { w: self }
    }
}
