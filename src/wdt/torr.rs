#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TORR {
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
pub struct TOPR {
    bits: u8,
}
impl TOPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TOP_INITR {
    bits: u8,
}
impl TOP_INITR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _TOPW<'a> {
    w: &'a mut W,
}
impl<'a> _TOPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TOP_INITW<'a> {
    w: &'a mut W,
}
impl<'a> _TOP_INITW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
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
    #[doc = "Bits 0:3 - \u{521d}\u{59cb}\u{503c}\u{ff08}\u{5728}WDT\u{4f7f}\u{80fd}\u{4e4b}\u{524d}\u{5199}\u{5165}\u{503c}\u{ff09} 2^(8+TOP)\u{ff0c}\u{5373}24\u{4f4d}"]
    #[inline]
    pub fn top(&self) -> TOPR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TOPR { bits }
    }
    #[doc = "Bits 4:7 - \u{8d85}\u{65f6}\u{540e}\u{5c06}\u{8981}\u{586b}\u{88c5}\u{7684}\u{503c}"]
    #[inline]
    pub fn top_init(&self) -> TOP_INITR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TOP_INITR { bits }
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
    #[doc = "Bits 0:3 - \u{521d}\u{59cb}\u{503c}\u{ff08}\u{5728}WDT\u{4f7f}\u{80fd}\u{4e4b}\u{524d}\u{5199}\u{5165}\u{503c}\u{ff09} 2^(8+TOP)\u{ff0c}\u{5373}24\u{4f4d}"]
    #[inline]
    pub fn top(&mut self) -> _TOPW {
        _TOPW { w: self }
    }
    #[doc = "Bits 4:7 - \u{8d85}\u{65f6}\u{540e}\u{5c06}\u{8981}\u{586b}\u{88c5}\u{7684}\u{503c}"]
    #[inline]
    pub fn top_init(&mut self) -> _TOP_INITW {
        _TOP_INITW { w: self }
    }
}
