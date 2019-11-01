#[doc = "Reader of register INTEN"]
pub type R = crate::R<u32, super::INTEN>;
#[doc = "Writer for register INTEN"]
pub type W = crate::W<u32, super::INTEN>;
#[doc = "Register INTEN `reset()`'s with value 0"]
impl crate::ResetValue for super::INTEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INTEN0`"]
pub type INTEN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTEN0`"]
pub struct INTEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN0_W<'a> {
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
#[doc = "Reader of field `INTEN1`"]
pub type INTEN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTEN1`"]
pub struct INTEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN1_W<'a> {
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
#[doc = "Reader of field `INTEN2`"]
pub type INTEN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTEN2`"]
pub struct INTEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN2_W<'a> {
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
#[doc = "Reader of field `INTEN3`"]
pub type INTEN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTEN3`"]
pub struct INTEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN3_W<'a> {
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
#[doc = "Reader of field `INTEN4`"]
pub type INTEN4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTEN4`"]
pub struct INTEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN4_W<'a> {
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
#[doc = "Reader of field `INTEN5`"]
pub type INTEN5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTEN5`"]
pub struct INTEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN5_W<'a> {
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
#[doc = "Reader of field `INTEN6`"]
pub type INTEN6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTEN6`"]
pub struct INTEN6_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN6_W<'a> {
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
#[doc = "Reader of field `INTEN7`"]
pub type INTEN7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTEN7`"]
pub struct INTEN7_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN7_W<'a> {
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
#[doc = "Reader of field `INTEN8`"]
pub type INTEN8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTEN8`"]
pub struct INTEN8_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN8_W<'a> {
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
#[doc = "Reader of field `INTEN9`"]
pub type INTEN9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTEN9`"]
pub struct INTEN9_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN9_W<'a> {
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
    #[doc = "Bit 0 - INTEN0"]
    #[inline(always)]
    pub fn inten0(&self) -> INTEN0_R {
        INTEN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - INTEN1"]
    #[inline(always)]
    pub fn inten1(&self) -> INTEN1_R {
        INTEN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - INTEN2"]
    #[inline(always)]
    pub fn inten2(&self) -> INTEN2_R {
        INTEN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - INTEN3"]
    #[inline(always)]
    pub fn inten3(&self) -> INTEN3_R {
        INTEN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - INTEN4"]
    #[inline(always)]
    pub fn inten4(&self) -> INTEN4_R {
        INTEN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - INTEN5"]
    #[inline(always)]
    pub fn inten5(&self) -> INTEN5_R {
        INTEN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - INTEN6"]
    #[inline(always)]
    pub fn inten6(&self) -> INTEN6_R {
        INTEN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - INTEN7"]
    #[inline(always)]
    pub fn inten7(&self) -> INTEN7_R {
        INTEN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - INTEN8"]
    #[inline(always)]
    pub fn inten8(&self) -> INTEN8_R {
        INTEN8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - INTEN9"]
    #[inline(always)]
    pub fn inten9(&self) -> INTEN9_R {
        INTEN9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - INTEN0"]
    #[inline(always)]
    pub fn inten0(&mut self) -> INTEN0_W {
        INTEN0_W { w: self }
    }
    #[doc = "Bit 1 - INTEN1"]
    #[inline(always)]
    pub fn inten1(&mut self) -> INTEN1_W {
        INTEN1_W { w: self }
    }
    #[doc = "Bit 2 - INTEN2"]
    #[inline(always)]
    pub fn inten2(&mut self) -> INTEN2_W {
        INTEN2_W { w: self }
    }
    #[doc = "Bit 3 - INTEN3"]
    #[inline(always)]
    pub fn inten3(&mut self) -> INTEN3_W {
        INTEN3_W { w: self }
    }
    #[doc = "Bit 4 - INTEN4"]
    #[inline(always)]
    pub fn inten4(&mut self) -> INTEN4_W {
        INTEN4_W { w: self }
    }
    #[doc = "Bit 5 - INTEN5"]
    #[inline(always)]
    pub fn inten5(&mut self) -> INTEN5_W {
        INTEN5_W { w: self }
    }
    #[doc = "Bit 6 - INTEN6"]
    #[inline(always)]
    pub fn inten6(&mut self) -> INTEN6_W {
        INTEN6_W { w: self }
    }
    #[doc = "Bit 7 - INTEN7"]
    #[inline(always)]
    pub fn inten7(&mut self) -> INTEN7_W {
        INTEN7_W { w: self }
    }
    #[doc = "Bit 8 - INTEN8"]
    #[inline(always)]
    pub fn inten8(&mut self) -> INTEN8_W {
        INTEN8_W { w: self }
    }
    #[doc = "Bit 9 - INTEN9"]
    #[inline(always)]
    pub fn inten9(&mut self) -> INTEN9_W {
        INTEN9_W { w: self }
    }
}
