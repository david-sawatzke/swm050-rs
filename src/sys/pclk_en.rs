#[doc = "Reader of register PCLK_EN"]
pub type R = crate::R<u32, super::PCLK_EN>;
#[doc = "Writer for register PCLK_EN"]
pub type W = crate::W<u32, super::PCLK_EN>;
#[doc = "Register PCLK_EN `reset()`'s with value 0"]
impl crate::ResetValue for super::PCLK_EN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WDT_CLK`"]
pub type WDT_CLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDT_CLK`"]
pub struct WDT_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_CLK_W<'a> {
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
#[doc = "Reader of field `TMRSE0_CLK`"]
pub type TMRSE0_CLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TMRSE0_CLK`"]
pub struct TMRSE0_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRSE0_CLK_W<'a> {
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
#[doc = "Reader of field `TMRSE1_CLK`"]
pub type TMRSE1_CLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TMRSE1_CLK`"]
pub struct TMRSE1_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRSE1_CLK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bit 4 - WDT_CLK"]
    #[inline(always)]
    pub fn wdt_clk(&self) -> WDT_CLK_R {
        WDT_CLK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - TMRSE0_CLK"]
    #[inline(always)]
    pub fn tmrse0_clk(&self) -> TMRSE0_CLK_R {
        TMRSE0_CLK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 17 - TMRSE1_CLK"]
    #[inline(always)]
    pub fn tmrse1_clk(&self) -> TMRSE1_CLK_R {
        TMRSE1_CLK_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - WDT_CLK"]
    #[inline(always)]
    pub fn wdt_clk(&mut self) -> WDT_CLK_W {
        WDT_CLK_W { w: self }
    }
    #[doc = "Bit 6 - TMRSE0_CLK"]
    #[inline(always)]
    pub fn tmrse0_clk(&mut self) -> TMRSE0_CLK_W {
        TMRSE0_CLK_W { w: self }
    }
    #[doc = "Bit 17 - TMRSE1_CLK"]
    #[inline(always)]
    pub fn tmrse1_clk(&mut self) -> TMRSE1_CLK_W {
        TMRSE1_CLK_W { w: self }
    }
}
