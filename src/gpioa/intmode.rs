#[doc = "Reader of register INTMODE"]
pub type R = crate::R<u32, super::INTMODE>;
#[doc = "Writer for register INTMODE"]
pub type W = crate::W<u32, super::INTMODE>;
#[doc = "Register INTMODE `reset()`'s with value 0"]
impl crate::ResetValue for super::INTMODE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INTMODE0`"]
pub type INTMODE0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTMODE0`"]
pub struct INTMODE0_W<'a> {
    w: &'a mut W,
}
impl<'a> INTMODE0_W<'a> {
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
#[doc = "Reader of field `INTMODE1`"]
pub type INTMODE1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTMODE1`"]
pub struct INTMODE1_W<'a> {
    w: &'a mut W,
}
impl<'a> INTMODE1_W<'a> {
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
#[doc = "Reader of field `INTMODE2`"]
pub type INTMODE2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTMODE2`"]
pub struct INTMODE2_W<'a> {
    w: &'a mut W,
}
impl<'a> INTMODE2_W<'a> {
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
#[doc = "Reader of field `INTMODE3`"]
pub type INTMODE3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTMODE3`"]
pub struct INTMODE3_W<'a> {
    w: &'a mut W,
}
impl<'a> INTMODE3_W<'a> {
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
#[doc = "Reader of field `INTMODE4`"]
pub type INTMODE4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTMODE4`"]
pub struct INTMODE4_W<'a> {
    w: &'a mut W,
}
impl<'a> INTMODE4_W<'a> {
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
#[doc = "Reader of field `INTMODE5`"]
pub type INTMODE5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTMODE5`"]
pub struct INTMODE5_W<'a> {
    w: &'a mut W,
}
impl<'a> INTMODE5_W<'a> {
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
#[doc = "Reader of field `INTMODE6`"]
pub type INTMODE6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTMODE6`"]
pub struct INTMODE6_W<'a> {
    w: &'a mut W,
}
impl<'a> INTMODE6_W<'a> {
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
#[doc = "Reader of field `INTMODE7`"]
pub type INTMODE7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTMODE7`"]
pub struct INTMODE7_W<'a> {
    w: &'a mut W,
}
impl<'a> INTMODE7_W<'a> {
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
#[doc = "Reader of field `INTMODE8`"]
pub type INTMODE8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTMODE8`"]
pub struct INTMODE8_W<'a> {
    w: &'a mut W,
}
impl<'a> INTMODE8_W<'a> {
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
#[doc = "Reader of field `INTMODE9`"]
pub type INTMODE9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTMODE9`"]
pub struct INTMODE9_W<'a> {
    w: &'a mut W,
}
impl<'a> INTMODE9_W<'a> {
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
    #[doc = "Bit 0 - INTMODE0"]
    #[inline(always)]
    pub fn intmode0(&self) -> INTMODE0_R {
        INTMODE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - INTMODE1"]
    #[inline(always)]
    pub fn intmode1(&self) -> INTMODE1_R {
        INTMODE1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - INTMODE2"]
    #[inline(always)]
    pub fn intmode2(&self) -> INTMODE2_R {
        INTMODE2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - INTMODE3"]
    #[inline(always)]
    pub fn intmode3(&self) -> INTMODE3_R {
        INTMODE3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - INTMODE4"]
    #[inline(always)]
    pub fn intmode4(&self) -> INTMODE4_R {
        INTMODE4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - INTMODE5"]
    #[inline(always)]
    pub fn intmode5(&self) -> INTMODE5_R {
        INTMODE5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - INTMODE6"]
    #[inline(always)]
    pub fn intmode6(&self) -> INTMODE6_R {
        INTMODE6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - INTMODE7"]
    #[inline(always)]
    pub fn intmode7(&self) -> INTMODE7_R {
        INTMODE7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - INTMODE8"]
    #[inline(always)]
    pub fn intmode8(&self) -> INTMODE8_R {
        INTMODE8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - INTMODE9"]
    #[inline(always)]
    pub fn intmode9(&self) -> INTMODE9_R {
        INTMODE9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - INTMODE0"]
    #[inline(always)]
    pub fn intmode0(&mut self) -> INTMODE0_W {
        INTMODE0_W { w: self }
    }
    #[doc = "Bit 1 - INTMODE1"]
    #[inline(always)]
    pub fn intmode1(&mut self) -> INTMODE1_W {
        INTMODE1_W { w: self }
    }
    #[doc = "Bit 2 - INTMODE2"]
    #[inline(always)]
    pub fn intmode2(&mut self) -> INTMODE2_W {
        INTMODE2_W { w: self }
    }
    #[doc = "Bit 3 - INTMODE3"]
    #[inline(always)]
    pub fn intmode3(&mut self) -> INTMODE3_W {
        INTMODE3_W { w: self }
    }
    #[doc = "Bit 4 - INTMODE4"]
    #[inline(always)]
    pub fn intmode4(&mut self) -> INTMODE4_W {
        INTMODE4_W { w: self }
    }
    #[doc = "Bit 5 - INTMODE5"]
    #[inline(always)]
    pub fn intmode5(&mut self) -> INTMODE5_W {
        INTMODE5_W { w: self }
    }
    #[doc = "Bit 6 - INTMODE6"]
    #[inline(always)]
    pub fn intmode6(&mut self) -> INTMODE6_W {
        INTMODE6_W { w: self }
    }
    #[doc = "Bit 7 - INTMODE7"]
    #[inline(always)]
    pub fn intmode7(&mut self) -> INTMODE7_W {
        INTMODE7_W { w: self }
    }
    #[doc = "Bit 8 - INTMODE8"]
    #[inline(always)]
    pub fn intmode8(&mut self) -> INTMODE8_W {
        INTMODE8_W { w: self }
    }
    #[doc = "Bit 9 - INTMODE9"]
    #[inline(always)]
    pub fn intmode9(&mut self) -> INTMODE9_W {
        INTMODE9_W { w: self }
    }
}
