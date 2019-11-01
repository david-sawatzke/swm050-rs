#[doc = "Reader of register CPINTCON"]
pub type R = crate::R<u32, super::CPINTCON>;
#[doc = "Writer for register CPINTCON"]
pub type W = crate::W<u32, super::CPINTCON>;
#[doc = "Register CPINTCON `reset()`'s with value 0"]
impl crate::ResetValue for super::CPINTCON {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INTMASKn`"]
pub type INTMASKN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTMASKn`"]
pub struct INTMASKN_W<'a> {
    w: &'a mut W,
}
impl<'a> INTMASKN_W<'a> {
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
#[doc = "Reader of field `INTUP`"]
pub type INTUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTUP`"]
pub struct INTUP_W<'a> {
    w: &'a mut W,
}
impl<'a> INTUP_W<'a> {
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
#[doc = "Reader of field `INTDOWN`"]
pub type INTDOWN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTDOWN`"]
pub struct INTDOWN_W<'a> {
    w: &'a mut W,
}
impl<'a> INTDOWN_W<'a> {
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
#[doc = "Reader of field `MOSINTMASKn`"]
pub type MOSINTMASKN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MOSINTMASKn`"]
pub struct MOSINTMASKN_W<'a> {
    w: &'a mut W,
}
impl<'a> MOSINTMASKN_W<'a> {
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
#[doc = "Reader of field `MOSINTEN`"]
pub type MOSINTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MOSINTEN`"]
pub struct MOSINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MOSINTEN_W<'a> {
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
#[doc = "Reader of field `MOSINTUP`"]
pub type MOSINTUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MOSINTUP`"]
pub struct MOSINTUP_W<'a> {
    w: &'a mut W,
}
impl<'a> MOSINTUP_W<'a> {
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
#[doc = "Reader of field `MOSINTDOWN`"]
pub type MOSINTDOWN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MOSINTDOWN`"]
pub struct MOSINTDOWN_W<'a> {
    w: &'a mut W,
}
impl<'a> MOSINTDOWN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - INTMASKn"]
    #[inline(always)]
    pub fn intmaskn(&self) -> INTMASKN_R {
        INTMASKN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - INTUP"]
    #[inline(always)]
    pub fn intup(&self) -> INTUP_R {
        INTUP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - INTDOWN"]
    #[inline(always)]
    pub fn intdown(&self) -> INTDOWN_R {
        INTDOWN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MOSINTMASKn"]
    #[inline(always)]
    pub fn mosintmaskn(&self) -> MOSINTMASKN_R {
        MOSINTMASKN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MOSINTEN"]
    #[inline(always)]
    pub fn mosinten(&self) -> MOSINTEN_R {
        MOSINTEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - MOSINTUP"]
    #[inline(always)]
    pub fn mosintup(&self) -> MOSINTUP_R {
        MOSINTUP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - MOSINTDOWN"]
    #[inline(always)]
    pub fn mosintdown(&self) -> MOSINTDOWN_R {
        MOSINTDOWN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - INTMASKn"]
    #[inline(always)]
    pub fn intmaskn(&mut self) -> INTMASKN_W {
        INTMASKN_W { w: self }
    }
    #[doc = "Bit 2 - INTUP"]
    #[inline(always)]
    pub fn intup(&mut self) -> INTUP_W {
        INTUP_W { w: self }
    }
    #[doc = "Bit 3 - INTDOWN"]
    #[inline(always)]
    pub fn intdown(&mut self) -> INTDOWN_W {
        INTDOWN_W { w: self }
    }
    #[doc = "Bit 4 - MOSINTMASKn"]
    #[inline(always)]
    pub fn mosintmaskn(&mut self) -> MOSINTMASKN_W {
        MOSINTMASKN_W { w: self }
    }
    #[doc = "Bit 5 - MOSINTEN"]
    #[inline(always)]
    pub fn mosinten(&mut self) -> MOSINTEN_W {
        MOSINTEN_W { w: self }
    }
    #[doc = "Bit 6 - MOSINTUP"]
    #[inline(always)]
    pub fn mosintup(&mut self) -> MOSINTUP_W {
        MOSINTUP_W { w: self }
    }
    #[doc = "Bit 7 - MOSINTDOWN"]
    #[inline(always)]
    pub fn mosintdown(&mut self) -> MOSINTDOWN_W {
        MOSINTDOWN_W { w: self }
    }
}
