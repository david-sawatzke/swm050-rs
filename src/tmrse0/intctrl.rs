#[doc = "Reader of register INTCTRL"]
pub type R = crate::R<u32, super::INTCTRL>;
#[doc = "Writer for register INTCTRL"]
pub type W = crate::W<u32, super::INTCTRL>;
#[doc = "Register INTCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::INTCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ENA`"]
pub type ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENA`"]
pub struct ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA_W<'a> {
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
#[doc = "Reader of field `MASKn`"]
pub type MASKN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASKn`"]
pub struct MASKN_W<'a> {
    w: &'a mut W,
}
impl<'a> MASKN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - 1 Enable to observe sate, you should set this bit 1"]
    #[inline(always)]
    pub fn ena(&self) -> ENA_R {
        ENA_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 0 shield"]
    #[inline(always)]
    pub fn maskn(&self) -> MASKN_R {
        MASKN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1 Enable to observe sate, you should set this bit 1"]
    #[inline(always)]
    pub fn ena(&mut self) -> ENA_W {
        ENA_W { w: self }
    }
    #[doc = "Bit 1 - 0 shield"]
    #[inline(always)]
    pub fn maskn(&mut self) -> MASKN_W {
        MASKN_W { w: self }
    }
}
