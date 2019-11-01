#[doc = "Reader of register LOCKCON"]
pub type R = crate::R<u32, super::LOCKCON>;
#[doc = "Writer for register LOCKCON"]
pub type W = crate::W<u32, super::LOCKCON>;
#[doc = "Register LOCKCON `reset()`'s with value 0"]
impl crate::ResetValue for super::LOCKCON {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LOCKEN`"]
pub type LOCKEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCKEN`"]
pub struct LOCKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKEN_W<'a> {
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
#[doc = "Reader of field `LEVELCON`"]
pub type LEVELCON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEVELCON`"]
pub struct LEVELCON_W<'a> {
    w: &'a mut W,
}
impl<'a> LEVELCON_W<'a> {
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
#[doc = "Reader of field `CLRCON`"]
pub type CLRCON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRCON`"]
pub struct CLRCON_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRCON_W<'a> {
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
#[doc = "Reader of field `LOCKSTAT`"]
pub type LOCKSTAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCKSTAT`"]
pub struct LOCKSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKSTAT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - LOCKEN"]
    #[inline(always)]
    pub fn locken(&self) -> LOCKEN_R {
        LOCKEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LEVELCON"]
    #[inline(always)]
    pub fn levelcon(&self) -> LEVELCON_R {
        LEVELCON_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CLRCON"]
    #[inline(always)]
    pub fn clrcon(&self) -> CLRCON_R {
        CLRCON_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LOCKSTAT"]
    #[inline(always)]
    pub fn lockstat(&self) -> LOCKSTAT_R {
        LOCKSTAT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LOCKEN"]
    #[inline(always)]
    pub fn locken(&mut self) -> LOCKEN_W {
        LOCKEN_W { w: self }
    }
    #[doc = "Bit 1 - LEVELCON"]
    #[inline(always)]
    pub fn levelcon(&mut self) -> LEVELCON_W {
        LEVELCON_W { w: self }
    }
    #[doc = "Bit 2 - CLRCON"]
    #[inline(always)]
    pub fn clrcon(&mut self) -> CLRCON_W {
        CLRCON_W { w: self }
    }
    #[doc = "Bit 3 - LOCKSTAT"]
    #[inline(always)]
    pub fn lockstat(&mut self) -> LOCKSTAT_W {
        LOCKSTAT_W { w: self }
    }
}
