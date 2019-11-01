#[doc = "Reader of register DBL_IRC"]
pub type R = crate::R<u32, super::DBL_IRC>;
#[doc = "Writer for register DBL_IRC"]
pub type W = crate::W<u32, super::DBL_IRC>;
#[doc = "Register DBL_IRC `reset()`'s with value 0"]
impl crate::ResetValue for super::DBL_IRC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DBL_IRC`"]
pub type DBL_IRC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBL_IRC`"]
pub struct DBL_IRC_W<'a> {
    w: &'a mut W,
}
impl<'a> DBL_IRC_W<'a> {
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
    #[doc = "Bit 0 - 0:18MHz 1:36MHz"]
    #[inline(always)]
    pub fn dbl_irc(&self) -> DBL_IRC_R {
        DBL_IRC_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:18MHz 1:36MHz"]
    #[inline(always)]
    pub fn dbl_irc(&mut self) -> DBL_IRC_W {
        DBL_IRC_W { w: self }
    }
}
