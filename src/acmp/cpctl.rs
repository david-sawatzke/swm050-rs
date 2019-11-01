#[doc = "Reader of register CPCTL"]
pub type R = crate::R<u32, super::CPCTL>;
#[doc = "Writer for register CPCTL"]
pub type W = crate::W<u32, super::CPCTL>;
#[doc = "Register CPCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::CPCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MOS_LEVEL`"]
pub type MOS_LEVEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MOS_LEVEL`"]
pub struct MOS_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MOS_LEVEL_W<'a> {
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
#[doc = "Reader of field `CEL_LEVEL`"]
pub type CEL_LEVEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CEL_LEVEL`"]
pub struct CEL_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CEL_LEVEL_W<'a> {
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
#[doc = "Reader of field `A7M_LEVEL`"]
pub type A7M_LEVEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `A7M_LEVEL`"]
pub struct A7M_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> A7M_LEVEL_W<'a> {
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
#[doc = "Reader of field `A7C_LEVEL`"]
pub type A7C_LEVEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `A7C_LEVEL`"]
pub struct A7C_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> A7C_LEVEL_W<'a> {
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
#[doc = "Reader of field `MOS_DELAY`"]
pub type MOS_DELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MOS_DELAY`"]
pub struct MOS_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> MOS_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Reader of field `CEL_DELAY`"]
pub type CEL_DELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CEL_DELAY`"]
pub struct CEL_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> CEL_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - MOS_LEVEL"]
    #[inline(always)]
    pub fn mos_level(&self) -> MOS_LEVEL_R {
        MOS_LEVEL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CEL_LEVEL"]
    #[inline(always)]
    pub fn cel_level(&self) -> CEL_LEVEL_R {
        CEL_LEVEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - A7M_LEVEL"]
    #[inline(always)]
    pub fn a7m_level(&self) -> A7M_LEVEL_R {
        A7M_LEVEL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - A7C_LEVEL"]
    #[inline(always)]
    pub fn a7c_level(&self) -> A7C_LEVEL_R {
        A7C_LEVEL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 8:13 - MOS_DELAY"]
    #[inline(always)]
    pub fn mos_delay(&self) -> MOS_DELAY_R {
        MOS_DELAY_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - CEL_DELAY"]
    #[inline(always)]
    pub fn cel_delay(&self) -> CEL_DELAY_R {
        CEL_DELAY_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - MOS_LEVEL"]
    #[inline(always)]
    pub fn mos_level(&mut self) -> MOS_LEVEL_W {
        MOS_LEVEL_W { w: self }
    }
    #[doc = "Bit 2 - CEL_LEVEL"]
    #[inline(always)]
    pub fn cel_level(&mut self) -> CEL_LEVEL_W {
        CEL_LEVEL_W { w: self }
    }
    #[doc = "Bit 3 - A7M_LEVEL"]
    #[inline(always)]
    pub fn a7m_level(&mut self) -> A7M_LEVEL_W {
        A7M_LEVEL_W { w: self }
    }
    #[doc = "Bit 4 - A7C_LEVEL"]
    #[inline(always)]
    pub fn a7c_level(&mut self) -> A7C_LEVEL_W {
        A7C_LEVEL_W { w: self }
    }
    #[doc = "Bits 8:13 - MOS_DELAY"]
    #[inline(always)]
    pub fn mos_delay(&mut self) -> MOS_DELAY_W {
        MOS_DELAY_W { w: self }
    }
    #[doc = "Bits 16:21 - CEL_DELAY"]
    #[inline(always)]
    pub fn cel_delay(&mut self) -> CEL_DELAY_W {
        CEL_DELAY_W { w: self }
    }
}
