#[doc = "Reader of register DAT"]
pub type R = crate::R<u32, super::DAT>;
#[doc = "Writer for register DAT"]
pub type W = crate::W<u32, super::DAT>;
#[doc = "Register DAT `reset()`'s with value 0"]
impl crate::ResetValue for super::DAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DAT_0`"]
pub type DAT_0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DAT_0`"]
pub struct DAT_0_W<'a> {
    w: &'a mut W,
}
impl<'a> DAT_0_W<'a> {
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
#[doc = "Reader of field `DAT_1`"]
pub type DAT_1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DAT_1`"]
pub struct DAT_1_W<'a> {
    w: &'a mut W,
}
impl<'a> DAT_1_W<'a> {
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
#[doc = "Reader of field `DAT_2`"]
pub type DAT_2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DAT_2`"]
pub struct DAT_2_W<'a> {
    w: &'a mut W,
}
impl<'a> DAT_2_W<'a> {
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
#[doc = "Reader of field `DAT_3`"]
pub type DAT_3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DAT_3`"]
pub struct DAT_3_W<'a> {
    w: &'a mut W,
}
impl<'a> DAT_3_W<'a> {
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
#[doc = "Reader of field `DAT_4`"]
pub type DAT_4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DAT_4`"]
pub struct DAT_4_W<'a> {
    w: &'a mut W,
}
impl<'a> DAT_4_W<'a> {
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
#[doc = "Reader of field `DAT_5`"]
pub type DAT_5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DAT_5`"]
pub struct DAT_5_W<'a> {
    w: &'a mut W,
}
impl<'a> DAT_5_W<'a> {
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
#[doc = "Reader of field `DAT_6`"]
pub type DAT_6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DAT_6`"]
pub struct DAT_6_W<'a> {
    w: &'a mut W,
}
impl<'a> DAT_6_W<'a> {
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
#[doc = "Reader of field `DAT_7`"]
pub type DAT_7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DAT_7`"]
pub struct DAT_7_W<'a> {
    w: &'a mut W,
}
impl<'a> DAT_7_W<'a> {
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
#[doc = "Reader of field `DAT_8`"]
pub type DAT_8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DAT_8`"]
pub struct DAT_8_W<'a> {
    w: &'a mut W,
}
impl<'a> DAT_8_W<'a> {
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
#[doc = "Reader of field `DAT_9`"]
pub type DAT_9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DAT_9`"]
pub struct DAT_9_W<'a> {
    w: &'a mut W,
}
impl<'a> DAT_9_W<'a> {
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
    #[doc = "Bit 0 - DAT_0"]
    #[inline(always)]
    pub fn dat_0(&self) -> DAT_0_R {
        DAT_0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DAT_1"]
    #[inline(always)]
    pub fn dat_1(&self) -> DAT_1_R {
        DAT_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DAT_2"]
    #[inline(always)]
    pub fn dat_2(&self) -> DAT_2_R {
        DAT_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DAT_3"]
    #[inline(always)]
    pub fn dat_3(&self) -> DAT_3_R {
        DAT_3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - DAT_4"]
    #[inline(always)]
    pub fn dat_4(&self) -> DAT_4_R {
        DAT_4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - DAT_5"]
    #[inline(always)]
    pub fn dat_5(&self) -> DAT_5_R {
        DAT_5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - DAT_6"]
    #[inline(always)]
    pub fn dat_6(&self) -> DAT_6_R {
        DAT_6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DAT_7"]
    #[inline(always)]
    pub fn dat_7(&self) -> DAT_7_R {
        DAT_7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DAT_8"]
    #[inline(always)]
    pub fn dat_8(&self) -> DAT_8_R {
        DAT_8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - DAT_9"]
    #[inline(always)]
    pub fn dat_9(&self) -> DAT_9_R {
        DAT_9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DAT_0"]
    #[inline(always)]
    pub fn dat_0(&mut self) -> DAT_0_W {
        DAT_0_W { w: self }
    }
    #[doc = "Bit 1 - DAT_1"]
    #[inline(always)]
    pub fn dat_1(&mut self) -> DAT_1_W {
        DAT_1_W { w: self }
    }
    #[doc = "Bit 2 - DAT_2"]
    #[inline(always)]
    pub fn dat_2(&mut self) -> DAT_2_W {
        DAT_2_W { w: self }
    }
    #[doc = "Bit 3 - DAT_3"]
    #[inline(always)]
    pub fn dat_3(&mut self) -> DAT_3_W {
        DAT_3_W { w: self }
    }
    #[doc = "Bit 4 - DAT_4"]
    #[inline(always)]
    pub fn dat_4(&mut self) -> DAT_4_W {
        DAT_4_W { w: self }
    }
    #[doc = "Bit 5 - DAT_5"]
    #[inline(always)]
    pub fn dat_5(&mut self) -> DAT_5_W {
        DAT_5_W { w: self }
    }
    #[doc = "Bit 6 - DAT_6"]
    #[inline(always)]
    pub fn dat_6(&mut self) -> DAT_6_W {
        DAT_6_W { w: self }
    }
    #[doc = "Bit 7 - DAT_7"]
    #[inline(always)]
    pub fn dat_7(&mut self) -> DAT_7_W {
        DAT_7_W { w: self }
    }
    #[doc = "Bit 8 - DAT_8"]
    #[inline(always)]
    pub fn dat_8(&mut self) -> DAT_8_W {
        DAT_8_W { w: self }
    }
    #[doc = "Bit 9 - DAT_9"]
    #[inline(always)]
    pub fn dat_9(&mut self) -> DAT_9_W {
        DAT_9_W { w: self }
    }
}
