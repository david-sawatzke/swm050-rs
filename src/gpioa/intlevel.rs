#[doc = "Reader of register INTLEVEL"]
pub type R = crate::R<u32, super::INTLEVEL>;
#[doc = "Writer for register INTLEVEL"]
pub type W = crate::W<u32, super::INTLEVEL>;
#[doc = "Register INTLEVEL `reset()`'s with value 0"]
impl crate::ResetValue for super::INTLEVEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INTLEVEL0`"]
pub type INTLEVEL0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTLEVEL0`"]
pub struct INTLEVEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> INTLEVEL0_W<'a> {
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
#[doc = "Reader of field `INTLEVEL1`"]
pub type INTLEVEL1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTLEVEL1`"]
pub struct INTLEVEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> INTLEVEL1_W<'a> {
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
#[doc = "Reader of field `INTLEVEL2`"]
pub type INTLEVEL2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTLEVEL2`"]
pub struct INTLEVEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> INTLEVEL2_W<'a> {
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
#[doc = "Reader of field `INTLEVEL3`"]
pub type INTLEVEL3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTLEVEL3`"]
pub struct INTLEVEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> INTLEVEL3_W<'a> {
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
#[doc = "Reader of field `INTLEVEL4`"]
pub type INTLEVEL4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTLEVEL4`"]
pub struct INTLEVEL4_W<'a> {
    w: &'a mut W,
}
impl<'a> INTLEVEL4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `INTLEVEL5`"]
pub type INTLEVEL5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTLEVEL5`"]
pub struct INTLEVEL5_W<'a> {
    w: &'a mut W,
}
impl<'a> INTLEVEL5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `INTLEVEL6`"]
pub type INTLEVEL6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTLEVEL6`"]
pub struct INTLEVEL6_W<'a> {
    w: &'a mut W,
}
impl<'a> INTLEVEL6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `INTLEVEL7`"]
pub type INTLEVEL7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTLEVEL7`"]
pub struct INTLEVEL7_W<'a> {
    w: &'a mut W,
}
impl<'a> INTLEVEL7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `INTLEVEL8`"]
pub type INTLEVEL8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTLEVEL8`"]
pub struct INTLEVEL8_W<'a> {
    w: &'a mut W,
}
impl<'a> INTLEVEL8_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `INTLEVEL9`"]
pub type INTLEVEL9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTLEVEL9`"]
pub struct INTLEVEL9_W<'a> {
    w: &'a mut W,
}
impl<'a> INTLEVEL9_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - INTLEVEL0"]
    #[inline(always)]
    pub fn intlevel0(&self) -> INTLEVEL0_R {
        INTLEVEL0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - INTLEVEL1"]
    #[inline(always)]
    pub fn intlevel1(&self) -> INTLEVEL1_R {
        INTLEVEL1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - INTLEVEL2"]
    #[inline(always)]
    pub fn intlevel2(&self) -> INTLEVEL2_R {
        INTLEVEL2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - INTLEVEL3"]
    #[inline(always)]
    pub fn intlevel3(&self) -> INTLEVEL3_R {
        INTLEVEL3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - INTLEVEL4"]
    #[inline(always)]
    pub fn intlevel4(&self) -> INTLEVEL4_R {
        INTLEVEL4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - INTLEVEL5"]
    #[inline(always)]
    pub fn intlevel5(&self) -> INTLEVEL5_R {
        INTLEVEL5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - INTLEVEL6"]
    #[inline(always)]
    pub fn intlevel6(&self) -> INTLEVEL6_R {
        INTLEVEL6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - INTLEVEL7"]
    #[inline(always)]
    pub fn intlevel7(&self) -> INTLEVEL7_R {
        INTLEVEL7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - INTLEVEL8"]
    #[inline(always)]
    pub fn intlevel8(&self) -> INTLEVEL8_R {
        INTLEVEL8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - INTLEVEL9"]
    #[inline(always)]
    pub fn intlevel9(&self) -> INTLEVEL9_R {
        INTLEVEL9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - INTLEVEL0"]
    #[inline(always)]
    pub fn intlevel0(&mut self) -> INTLEVEL0_W {
        INTLEVEL0_W { w: self }
    }
    #[doc = "Bit 1 - INTLEVEL1"]
    #[inline(always)]
    pub fn intlevel1(&mut self) -> INTLEVEL1_W {
        INTLEVEL1_W { w: self }
    }
    #[doc = "Bit 2 - INTLEVEL2"]
    #[inline(always)]
    pub fn intlevel2(&mut self) -> INTLEVEL2_W {
        INTLEVEL2_W { w: self }
    }
    #[doc = "Bit 3 - INTLEVEL3"]
    #[inline(always)]
    pub fn intlevel3(&mut self) -> INTLEVEL3_W {
        INTLEVEL3_W { w: self }
    }
    #[doc = "Bit 4 - INTLEVEL4"]
    #[inline(always)]
    pub fn intlevel4(&mut self) -> INTLEVEL4_W {
        INTLEVEL4_W { w: self }
    }
    #[doc = "Bit 5 - INTLEVEL5"]
    #[inline(always)]
    pub fn intlevel5(&mut self) -> INTLEVEL5_W {
        INTLEVEL5_W { w: self }
    }
    #[doc = "Bit 6 - INTLEVEL6"]
    #[inline(always)]
    pub fn intlevel6(&mut self) -> INTLEVEL6_W {
        INTLEVEL6_W { w: self }
    }
    #[doc = "Bit 7 - INTLEVEL7"]
    #[inline(always)]
    pub fn intlevel7(&mut self) -> INTLEVEL7_W {
        INTLEVEL7_W { w: self }
    }
    #[doc = "Bit 8 - INTLEVEL8"]
    #[inline(always)]
    pub fn intlevel8(&mut self) -> INTLEVEL8_W {
        INTLEVEL8_W { w: self }
    }
    #[doc = "Bit 9 - INTLEVEL9"]
    #[inline(always)]
    pub fn intlevel9(&mut self) -> INTLEVEL9_W {
        INTLEVEL9_W { w: self }
    }
}
