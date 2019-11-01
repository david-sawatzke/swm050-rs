#[doc = "Reader of register DIR"]
pub type R = crate::R<u32, super::DIR>;
#[doc = "Writer for register DIR"]
pub type W = crate::W<u32, super::DIR>;
#[doc = "Register DIR `reset()`'s with value 0"]
impl crate::ResetValue for super::DIR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DIR_0`"]
pub type DIR_0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIR_0`"]
pub struct DIR_0_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR_0_W<'a> {
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
#[doc = "Reader of field `DIR_1`"]
pub type DIR_1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIR_1`"]
pub struct DIR_1_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR_1_W<'a> {
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
#[doc = "Reader of field `DIR_2`"]
pub type DIR_2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIR_2`"]
pub struct DIR_2_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR_2_W<'a> {
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
#[doc = "Reader of field `DIR_3`"]
pub type DIR_3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIR_3`"]
pub struct DIR_3_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR_3_W<'a> {
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
#[doc = "Reader of field `DIR_4`"]
pub type DIR_4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIR_4`"]
pub struct DIR_4_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR_4_W<'a> {
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
#[doc = "Reader of field `DIR_5`"]
pub type DIR_5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIR_5`"]
pub struct DIR_5_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR_5_W<'a> {
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
#[doc = "Reader of field `DIR_6`"]
pub type DIR_6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIR_6`"]
pub struct DIR_6_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR_6_W<'a> {
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
#[doc = "Reader of field `DIR_7`"]
pub type DIR_7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIR_7`"]
pub struct DIR_7_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR_7_W<'a> {
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
#[doc = "Reader of field `DIR_8`"]
pub type DIR_8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIR_8`"]
pub struct DIR_8_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR_8_W<'a> {
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
#[doc = "Reader of field `DIR_9`"]
pub type DIR_9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIR_9`"]
pub struct DIR_9_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR_9_W<'a> {
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
    #[doc = "Bit 0 - DIR_0"]
    #[inline(always)]
    pub fn dir_0(&self) -> DIR_0_R {
        DIR_0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DIR_1"]
    #[inline(always)]
    pub fn dir_1(&self) -> DIR_1_R {
        DIR_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DIR_2"]
    #[inline(always)]
    pub fn dir_2(&self) -> DIR_2_R {
        DIR_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DIR_3"]
    #[inline(always)]
    pub fn dir_3(&self) -> DIR_3_R {
        DIR_3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - DIR_4"]
    #[inline(always)]
    pub fn dir_4(&self) -> DIR_4_R {
        DIR_4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - DIR_5"]
    #[inline(always)]
    pub fn dir_5(&self) -> DIR_5_R {
        DIR_5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - DIR_6"]
    #[inline(always)]
    pub fn dir_6(&self) -> DIR_6_R {
        DIR_6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DIR_7"]
    #[inline(always)]
    pub fn dir_7(&self) -> DIR_7_R {
        DIR_7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DIR_8"]
    #[inline(always)]
    pub fn dir_8(&self) -> DIR_8_R {
        DIR_8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - DIR_9"]
    #[inline(always)]
    pub fn dir_9(&self) -> DIR_9_R {
        DIR_9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DIR_0"]
    #[inline(always)]
    pub fn dir_0(&mut self) -> DIR_0_W {
        DIR_0_W { w: self }
    }
    #[doc = "Bit 1 - DIR_1"]
    #[inline(always)]
    pub fn dir_1(&mut self) -> DIR_1_W {
        DIR_1_W { w: self }
    }
    #[doc = "Bit 2 - DIR_2"]
    #[inline(always)]
    pub fn dir_2(&mut self) -> DIR_2_W {
        DIR_2_W { w: self }
    }
    #[doc = "Bit 3 - DIR_3"]
    #[inline(always)]
    pub fn dir_3(&mut self) -> DIR_3_W {
        DIR_3_W { w: self }
    }
    #[doc = "Bit 4 - DIR_4"]
    #[inline(always)]
    pub fn dir_4(&mut self) -> DIR_4_W {
        DIR_4_W { w: self }
    }
    #[doc = "Bit 5 - DIR_5"]
    #[inline(always)]
    pub fn dir_5(&mut self) -> DIR_5_W {
        DIR_5_W { w: self }
    }
    #[doc = "Bit 6 - DIR_6"]
    #[inline(always)]
    pub fn dir_6(&mut self) -> DIR_6_W {
        DIR_6_W { w: self }
    }
    #[doc = "Bit 7 - DIR_7"]
    #[inline(always)]
    pub fn dir_7(&mut self) -> DIR_7_W {
        DIR_7_W { w: self }
    }
    #[doc = "Bit 8 - DIR_8"]
    #[inline(always)]
    pub fn dir_8(&mut self) -> DIR_8_W {
        DIR_8_W { w: self }
    }
    #[doc = "Bit 9 - DIR_9"]
    #[inline(always)]
    pub fn dir_9(&mut self) -> DIR_9_W {
        DIR_9_W { w: self }
    }
}
