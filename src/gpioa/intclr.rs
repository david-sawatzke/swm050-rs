#[doc = "Reader of register INTCLR"]
pub type R = crate::R<u32, super::INTCLR>;
#[doc = "Writer for register INTCLR"]
pub type W = crate::W<u32, super::INTCLR>;
#[doc = "Register INTCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::INTCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INTCLR0`"]
pub type INTCLR0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTCLR0`"]
pub struct INTCLR0_W<'a> {
    w: &'a mut W,
}
impl<'a> INTCLR0_W<'a> {
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
#[doc = "Reader of field `INTCLR1`"]
pub type INTCLR1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTCLR1`"]
pub struct INTCLR1_W<'a> {
    w: &'a mut W,
}
impl<'a> INTCLR1_W<'a> {
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
#[doc = "Reader of field `INTCLR2`"]
pub type INTCLR2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTCLR2`"]
pub struct INTCLR2_W<'a> {
    w: &'a mut W,
}
impl<'a> INTCLR2_W<'a> {
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
#[doc = "Reader of field `INTCLR3`"]
pub type INTCLR3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTCLR3`"]
pub struct INTCLR3_W<'a> {
    w: &'a mut W,
}
impl<'a> INTCLR3_W<'a> {
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
#[doc = "Reader of field `INTCLR4`"]
pub type INTCLR4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTCLR4`"]
pub struct INTCLR4_W<'a> {
    w: &'a mut W,
}
impl<'a> INTCLR4_W<'a> {
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
#[doc = "Reader of field `INTCLR5`"]
pub type INTCLR5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTCLR5`"]
pub struct INTCLR5_W<'a> {
    w: &'a mut W,
}
impl<'a> INTCLR5_W<'a> {
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
#[doc = "Reader of field `INTCLR6`"]
pub type INTCLR6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTCLR6`"]
pub struct INTCLR6_W<'a> {
    w: &'a mut W,
}
impl<'a> INTCLR6_W<'a> {
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
#[doc = "Reader of field `INTCLR7`"]
pub type INTCLR7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTCLR7`"]
pub struct INTCLR7_W<'a> {
    w: &'a mut W,
}
impl<'a> INTCLR7_W<'a> {
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
#[doc = "Reader of field `INTCLR8`"]
pub type INTCLR8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTCLR8`"]
pub struct INTCLR8_W<'a> {
    w: &'a mut W,
}
impl<'a> INTCLR8_W<'a> {
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
#[doc = "Reader of field `INTCLR9`"]
pub type INTCLR9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTCLR9`"]
pub struct INTCLR9_W<'a> {
    w: &'a mut W,
}
impl<'a> INTCLR9_W<'a> {
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
    #[doc = "Bit 0 - INTCLR0"]
    #[inline(always)]
    pub fn intclr0(&self) -> INTCLR0_R {
        INTCLR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - INTCLR1"]
    #[inline(always)]
    pub fn intclr1(&self) -> INTCLR1_R {
        INTCLR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - INTCLR2"]
    #[inline(always)]
    pub fn intclr2(&self) -> INTCLR2_R {
        INTCLR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - INTCLR3"]
    #[inline(always)]
    pub fn intclr3(&self) -> INTCLR3_R {
        INTCLR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - INTCLR4"]
    #[inline(always)]
    pub fn intclr4(&self) -> INTCLR4_R {
        INTCLR4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - INTCLR5"]
    #[inline(always)]
    pub fn intclr5(&self) -> INTCLR5_R {
        INTCLR5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - INTCLR6"]
    #[inline(always)]
    pub fn intclr6(&self) -> INTCLR6_R {
        INTCLR6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - INTCLR7"]
    #[inline(always)]
    pub fn intclr7(&self) -> INTCLR7_R {
        INTCLR7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - INTCLR8"]
    #[inline(always)]
    pub fn intclr8(&self) -> INTCLR8_R {
        INTCLR8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - INTCLR9"]
    #[inline(always)]
    pub fn intclr9(&self) -> INTCLR9_R {
        INTCLR9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - INTCLR0"]
    #[inline(always)]
    pub fn intclr0(&mut self) -> INTCLR0_W {
        INTCLR0_W { w: self }
    }
    #[doc = "Bit 1 - INTCLR1"]
    #[inline(always)]
    pub fn intclr1(&mut self) -> INTCLR1_W {
        INTCLR1_W { w: self }
    }
    #[doc = "Bit 2 - INTCLR2"]
    #[inline(always)]
    pub fn intclr2(&mut self) -> INTCLR2_W {
        INTCLR2_W { w: self }
    }
    #[doc = "Bit 3 - INTCLR3"]
    #[inline(always)]
    pub fn intclr3(&mut self) -> INTCLR3_W {
        INTCLR3_W { w: self }
    }
    #[doc = "Bit 4 - INTCLR4"]
    #[inline(always)]
    pub fn intclr4(&mut self) -> INTCLR4_W {
        INTCLR4_W { w: self }
    }
    #[doc = "Bit 5 - INTCLR5"]
    #[inline(always)]
    pub fn intclr5(&mut self) -> INTCLR5_W {
        INTCLR5_W { w: self }
    }
    #[doc = "Bit 6 - INTCLR6"]
    #[inline(always)]
    pub fn intclr6(&mut self) -> INTCLR6_W {
        INTCLR6_W { w: self }
    }
    #[doc = "Bit 7 - INTCLR7"]
    #[inline(always)]
    pub fn intclr7(&mut self) -> INTCLR7_W {
        INTCLR7_W { w: self }
    }
    #[doc = "Bit 8 - INTCLR8"]
    #[inline(always)]
    pub fn intclr8(&mut self) -> INTCLR8_W {
        INTCLR8_W { w: self }
    }
    #[doc = "Bit 9 - INTCLR9"]
    #[inline(always)]
    pub fn intclr9(&mut self) -> INTCLR9_W {
        INTCLR9_W { w: self }
    }
}
