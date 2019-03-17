#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CLK_CFG {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct CORE_CLK_DIVR {
    bits: u16,
}
impl CORE_CLK_DIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TMRSE_CLK_DIVR {
    bits: u8,
}
impl TMRSE_CLK_DIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _CORE_CLK_DIVW<'a> {
    w: &'a mut W,
}
impl<'a> _CORE_CLK_DIVW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 1023;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TMRSE_CLK_DIVW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRSE_CLK_DIVW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:9 - =0\u{ff1a}2\u{5206}\u{9891} =\u{5947}\u{6570}\u{ff1a}\u{4e0d}\u{5206}\u{9891} =\u{5076}\u{6570} \u{5206}\u{9891}"]
    #[inline]
    pub fn core_clk_div(&self) -> CORE_CLK_DIVR {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        CORE_CLK_DIVR { bits }
    }
    #[doc = "Bits 16:21 - =0\u{ff1a}2\u{5206}\u{9891} =\u{5947}\u{6570}\u{ff1a}\u{4e0d}\u{5206}\u{9891} =\u{5076}\u{6570} \u{5206}\u{9891}\u{ff0c}\u{76f4}\u{63a5}\u{5bf9}\u{65f6}\u{949f}\u{6e90}\u{5206}\u{9891}\u{ff0c}\u{4e0d}\u{4f9d}\u{8d56}\u{4e8e}\u{5185}\u{6838}\u{65f6}\u{949f}"]
    #[inline]
    pub fn tmrse_clk_div(&self) -> TMRSE_CLK_DIVR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TMRSE_CLK_DIVR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:9 - =0\u{ff1a}2\u{5206}\u{9891} =\u{5947}\u{6570}\u{ff1a}\u{4e0d}\u{5206}\u{9891} =\u{5076}\u{6570} \u{5206}\u{9891}"]
    #[inline]
    pub fn core_clk_div(&mut self) -> _CORE_CLK_DIVW {
        _CORE_CLK_DIVW { w: self }
    }
    #[doc = "Bits 16:21 - =0\u{ff1a}2\u{5206}\u{9891} =\u{5947}\u{6570}\u{ff1a}\u{4e0d}\u{5206}\u{9891} =\u{5076}\u{6570} \u{5206}\u{9891}\u{ff0c}\u{76f4}\u{63a5}\u{5bf9}\u{65f6}\u{949f}\u{6e90}\u{5206}\u{9891}\u{ff0c}\u{4e0d}\u{4f9d}\u{8d56}\u{4e8e}\u{5185}\u{6838}\u{65f6}\u{949f}"]
    #[inline]
    pub fn tmrse_clk_div(&mut self) -> _TMRSE_CLK_DIVW {
        _TMRSE_CLK_DIVW { w: self }
    }
}
