#[doc = "Reader of register INTMSK"]
pub type R = crate::R<u32, super::INTMSK>;
#[doc = "Writer for register INTMSK"]
pub type W = crate::W<u32, super::INTMSK>;
#[doc = "Register INTMSK `reset()`'s with value 0"]
impl crate::ResetValue for super::INTMSK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INTMSK0`"]
pub type INTMSK0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTMSK0`"]
pub struct INTMSK0_W<'a> {
    w: &'a mut W,
}
impl<'a> INTMSK0_W<'a> {
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
#[doc = "Reader of field `INTMSK1`"]
pub type INTMSK1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTMSK1`"]
pub struct INTMSK1_W<'a> {
    w: &'a mut W,
}
impl<'a> INTMSK1_W<'a> {
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
#[doc = "Reader of field `INTMSK2`"]
pub type INTMSK2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTMSK2`"]
pub struct INTMSK2_W<'a> {
    w: &'a mut W,
}
impl<'a> INTMSK2_W<'a> {
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
#[doc = "Reader of field `INTMSK3`"]
pub type INTMSK3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTMSK3`"]
pub struct INTMSK3_W<'a> {
    w: &'a mut W,
}
impl<'a> INTMSK3_W<'a> {
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
#[doc = "Reader of field `INTMSK4`"]
pub type INTMSK4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTMSK4`"]
pub struct INTMSK4_W<'a> {
    w: &'a mut W,
}
impl<'a> INTMSK4_W<'a> {
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
#[doc = "Reader of field `INTMSK5`"]
pub type INTMSK5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTMSK5`"]
pub struct INTMSK5_W<'a> {
    w: &'a mut W,
}
impl<'a> INTMSK5_W<'a> {
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
#[doc = "Reader of field `INTMSK6`"]
pub type INTMSK6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTMSK6`"]
pub struct INTMSK6_W<'a> {
    w: &'a mut W,
}
impl<'a> INTMSK6_W<'a> {
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
#[doc = "Reader of field `INTMSK7`"]
pub type INTMSK7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTMSK7`"]
pub struct INTMSK7_W<'a> {
    w: &'a mut W,
}
impl<'a> INTMSK7_W<'a> {
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
#[doc = "Reader of field `INTMSK8`"]
pub type INTMSK8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTMSK8`"]
pub struct INTMSK8_W<'a> {
    w: &'a mut W,
}
impl<'a> INTMSK8_W<'a> {
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
#[doc = "Reader of field `INTMSK9`"]
pub type INTMSK9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTMSK9`"]
pub struct INTMSK9_W<'a> {
    w: &'a mut W,
}
impl<'a> INTMSK9_W<'a> {
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
    #[doc = "Bit 0 - INTMSK0"]
    #[inline(always)]
    pub fn intmsk0(&self) -> INTMSK0_R {
        INTMSK0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - INTMSK1"]
    #[inline(always)]
    pub fn intmsk1(&self) -> INTMSK1_R {
        INTMSK1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - INTMSK2"]
    #[inline(always)]
    pub fn intmsk2(&self) -> INTMSK2_R {
        INTMSK2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - INTMSK3"]
    #[inline(always)]
    pub fn intmsk3(&self) -> INTMSK3_R {
        INTMSK3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - INTMSK4"]
    #[inline(always)]
    pub fn intmsk4(&self) -> INTMSK4_R {
        INTMSK4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - INTMSK5"]
    #[inline(always)]
    pub fn intmsk5(&self) -> INTMSK5_R {
        INTMSK5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - INTMSK6"]
    #[inline(always)]
    pub fn intmsk6(&self) -> INTMSK6_R {
        INTMSK6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - INTMSK7"]
    #[inline(always)]
    pub fn intmsk7(&self) -> INTMSK7_R {
        INTMSK7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - INTMSK8"]
    #[inline(always)]
    pub fn intmsk8(&self) -> INTMSK8_R {
        INTMSK8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - INTMSK9"]
    #[inline(always)]
    pub fn intmsk9(&self) -> INTMSK9_R {
        INTMSK9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - INTMSK0"]
    #[inline(always)]
    pub fn intmsk0(&mut self) -> INTMSK0_W {
        INTMSK0_W { w: self }
    }
    #[doc = "Bit 1 - INTMSK1"]
    #[inline(always)]
    pub fn intmsk1(&mut self) -> INTMSK1_W {
        INTMSK1_W { w: self }
    }
    #[doc = "Bit 2 - INTMSK2"]
    #[inline(always)]
    pub fn intmsk2(&mut self) -> INTMSK2_W {
        INTMSK2_W { w: self }
    }
    #[doc = "Bit 3 - INTMSK3"]
    #[inline(always)]
    pub fn intmsk3(&mut self) -> INTMSK3_W {
        INTMSK3_W { w: self }
    }
    #[doc = "Bit 4 - INTMSK4"]
    #[inline(always)]
    pub fn intmsk4(&mut self) -> INTMSK4_W {
        INTMSK4_W { w: self }
    }
    #[doc = "Bit 5 - INTMSK5"]
    #[inline(always)]
    pub fn intmsk5(&mut self) -> INTMSK5_W {
        INTMSK5_W { w: self }
    }
    #[doc = "Bit 6 - INTMSK6"]
    #[inline(always)]
    pub fn intmsk6(&mut self) -> INTMSK6_W {
        INTMSK6_W { w: self }
    }
    #[doc = "Bit 7 - INTMSK7"]
    #[inline(always)]
    pub fn intmsk7(&mut self) -> INTMSK7_W {
        INTMSK7_W { w: self }
    }
    #[doc = "Bit 8 - INTMSK8"]
    #[inline(always)]
    pub fn intmsk8(&mut self) -> INTMSK8_W {
        INTMSK8_W { w: self }
    }
    #[doc = "Bit 9 - INTMSK9"]
    #[inline(always)]
    pub fn intmsk9(&mut self) -> INTMSK9_W {
        INTMSK9_W { w: self }
    }
}
