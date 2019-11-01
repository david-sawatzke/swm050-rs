#[doc = "Reader of register CLK_CFG"]
pub type R = crate::R<u32, super::CLK_CFG>;
#[doc = "Writer for register CLK_CFG"]
pub type W = crate::W<u32, super::CLK_CFG>;
#[doc = "Register CLK_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::CLK_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CORE_CLK_DIV`"]
pub type CORE_CLK_DIV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CORE_CLK_DIV`"]
pub struct CORE_CLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_CLK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
#[doc = "Reader of field `TMRSE_CLK_DIV`"]
pub type TMRSE_CLK_DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TMRSE_CLK_DIV`"]
pub struct TMRSE_CLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRSE_CLK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - =0:2 Divide = Odd: Undivided = Even Divided, directliy divides the clock source, independent of the core clock"]
    #[inline(always)]
    pub fn core_clk_div(&self) -> CORE_CLK_DIV_R {
        CORE_CLK_DIV_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:21 - =0:2 Divide = Odd: Divide = Even Divided"]
    #[inline(always)]
    pub fn tmrse_clk_div(&self) -> TMRSE_CLK_DIV_R {
        TMRSE_CLK_DIV_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - =0:2 Divide = Odd: Undivided = Even Divided, directliy divides the clock source, independent of the core clock"]
    #[inline(always)]
    pub fn core_clk_div(&mut self) -> CORE_CLK_DIV_W {
        CORE_CLK_DIV_W { w: self }
    }
    #[doc = "Bits 16:21 - =0:2 Divide = Odd: Divide = Even Divided"]
    #[inline(always)]
    pub fn tmrse_clk_div(&mut self) -> TMRSE_CLK_DIV_W {
        TMRSE_CLK_DIV_W { w: self }
    }
}
