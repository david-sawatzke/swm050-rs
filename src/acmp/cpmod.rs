#[doc = "Reader of register CPMOD"]
pub type R = crate::R<u32, super::CPMOD>;
#[doc = "Writer for register CPMOD"]
pub type W = crate::W<u32, super::CPMOD>;
#[doc = "Register CPMOD `reset()`'s with value 0"]
impl crate::ResetValue for super::CPMOD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WORKSMOD`"]
pub type WORKSMOD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WORKSMOD`"]
pub struct WORKSMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> WORKSMOD_W<'a> {
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
impl R {
    #[doc = "Bit 0 - WORKSMOD"]
    #[inline(always)]
    pub fn worksmod(&self) -> WORKSMOD_R {
        WORKSMOD_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WORKSMOD"]
    #[inline(always)]
    pub fn worksmod(&mut self) -> WORKSMOD_W {
        WORKSMOD_W { w: self }
    }
}
