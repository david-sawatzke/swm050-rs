#[doc = "Reader of register PORTA_INDIS"]
pub type R = crate::R<u32, super::PORTA_INDIS>;
#[doc = "Writer for register PORTA_INDIS"]
pub type W = crate::W<u32, super::PORTA_INDIS>;
#[doc = "Register PORTA_INDIS `reset()`'s with value 0"]
impl crate::ResetValue for super::PORTA_INDIS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PA00`"]
pub type PA00_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PA00`"]
pub struct PA00_W<'a> {
    w: &'a mut W,
}
impl<'a> PA00_W<'a> {
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
#[doc = "Reader of field `PA01`"]
pub type PA01_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PA01`"]
pub struct PA01_W<'a> {
    w: &'a mut W,
}
impl<'a> PA01_W<'a> {
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
#[doc = "Reader of field `PA02`"]
pub type PA02_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PA02`"]
pub struct PA02_W<'a> {
    w: &'a mut W,
}
impl<'a> PA02_W<'a> {
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
#[doc = "Reader of field `PA03`"]
pub type PA03_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PA03`"]
pub struct PA03_W<'a> {
    w: &'a mut W,
}
impl<'a> PA03_W<'a> {
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
#[doc = "Reader of field `PA04`"]
pub type PA04_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PA04`"]
pub struct PA04_W<'a> {
    w: &'a mut W,
}
impl<'a> PA04_W<'a> {
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
#[doc = "Reader of field `PA05`"]
pub type PA05_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PA05`"]
pub struct PA05_W<'a> {
    w: &'a mut W,
}
impl<'a> PA05_W<'a> {
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
#[doc = "Reader of field `PA06`"]
pub type PA06_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PA06`"]
pub struct PA06_W<'a> {
    w: &'a mut W,
}
impl<'a> PA06_W<'a> {
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
#[doc = "Reader of field `PA07`"]
pub type PA07_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PA07`"]
pub struct PA07_W<'a> {
    w: &'a mut W,
}
impl<'a> PA07_W<'a> {
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
#[doc = "Reader of field `PA08`"]
pub type PA08_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PA08`"]
pub struct PA08_W<'a> {
    w: &'a mut W,
}
impl<'a> PA08_W<'a> {
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
#[doc = "Reader of field `PA09`"]
pub type PA09_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PA09`"]
pub struct PA09_W<'a> {
    w: &'a mut W,
}
impl<'a> PA09_W<'a> {
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
    #[doc = "Bit 0 - 0 GPIOA.0 input function is not disabled 1 GPIOA.0 input function is disabled to reduce power consumption"]
    #[inline(always)]
    pub fn pa00(&self) -> PA00_R {
        PA00_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PA01"]
    #[inline(always)]
    pub fn pa01(&self) -> PA01_R {
        PA01_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PA02"]
    #[inline(always)]
    pub fn pa02(&self) -> PA02_R {
        PA02_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PA03"]
    #[inline(always)]
    pub fn pa03(&self) -> PA03_R {
        PA03_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PA04"]
    #[inline(always)]
    pub fn pa04(&self) -> PA04_R {
        PA04_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PA05"]
    #[inline(always)]
    pub fn pa05(&self) -> PA05_R {
        PA05_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PA06"]
    #[inline(always)]
    pub fn pa06(&self) -> PA06_R {
        PA06_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PA07"]
    #[inline(always)]
    pub fn pa07(&self) -> PA07_R {
        PA07_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - PA08"]
    #[inline(always)]
    pub fn pa08(&self) -> PA08_R {
        PA08_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - PA09"]
    #[inline(always)]
    pub fn pa09(&self) -> PA09_R {
        PA09_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0 GPIOA.0 input function is not disabled 1 GPIOA.0 input function is disabled to reduce power consumption"]
    #[inline(always)]
    pub fn pa00(&mut self) -> PA00_W {
        PA00_W { w: self }
    }
    #[doc = "Bit 1 - PA01"]
    #[inline(always)]
    pub fn pa01(&mut self) -> PA01_W {
        PA01_W { w: self }
    }
    #[doc = "Bit 2 - PA02"]
    #[inline(always)]
    pub fn pa02(&mut self) -> PA02_W {
        PA02_W { w: self }
    }
    #[doc = "Bit 3 - PA03"]
    #[inline(always)]
    pub fn pa03(&mut self) -> PA03_W {
        PA03_W { w: self }
    }
    #[doc = "Bit 4 - PA04"]
    #[inline(always)]
    pub fn pa04(&mut self) -> PA04_W {
        PA04_W { w: self }
    }
    #[doc = "Bit 5 - PA05"]
    #[inline(always)]
    pub fn pa05(&mut self) -> PA05_W {
        PA05_W { w: self }
    }
    #[doc = "Bit 6 - PA06"]
    #[inline(always)]
    pub fn pa06(&mut self) -> PA06_W {
        PA06_W { w: self }
    }
    #[doc = "Bit 7 - PA07"]
    #[inline(always)]
    pub fn pa07(&mut self) -> PA07_W {
        PA07_W { w: self }
    }
    #[doc = "Bit 8 - PA08"]
    #[inline(always)]
    pub fn pa08(&mut self) -> PA08_W {
        PA08_W { w: self }
    }
    #[doc = "Bit 9 - PA09"]
    #[inline(always)]
    pub fn pa09(&mut self) -> PA09_W {
        PA09_W { w: self }
    }
}
