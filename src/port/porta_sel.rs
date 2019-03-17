#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PORTA_SEL {
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
pub struct PA00R {
    bits: u8,
}
impl PA00R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PA01R {
    bits: u8,
}
impl PA01R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PA02R {
    bits: u8,
}
impl PA02R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PA03R {
    bits: u8,
}
impl PA03R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PA04R {
    bits: u8,
}
impl PA04R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PA05R {
    bits: u8,
}
impl PA05R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PA06R {
    bits: u8,
}
impl PA06R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PA07R {
    bits: u8,
}
impl PA07R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PA08R {
    bits: u8,
}
impl PA08R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PA09R {
    bits: u8,
}
impl PA09R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _PA00W<'a> {
    w: &'a mut W,
}
impl<'a> _PA00W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PA01W<'a> {
    w: &'a mut W,
}
impl<'a> _PA01W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PA02W<'a> {
    w: &'a mut W,
}
impl<'a> _PA02W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PA03W<'a> {
    w: &'a mut W,
}
impl<'a> _PA03W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PA04W<'a> {
    w: &'a mut W,
}
impl<'a> _PA04W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PA05W<'a> {
    w: &'a mut W,
}
impl<'a> _PA05W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PA06W<'a> {
    w: &'a mut W,
}
impl<'a> _PA06W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PA07W<'a> {
    w: &'a mut W,
}
impl<'a> _PA07W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PA08W<'a> {
    w: &'a mut W,
}
impl<'a> _PA08W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PA09W<'a> {
    w: &'a mut W,
}
impl<'a> _PA09W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
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
    #[doc = "Bits 0:1 - 00 GPIO 01 TMRSE0_IN 10/11"]
    #[inline]
    pub fn pa00(&self) -> PA00R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PA00R { bits }
    }
    #[doc = "Bits 2:3 - 00 GPIO 01 TMRSE0_OUT 10/11"]
    #[inline]
    pub fn pa01(&self) -> PA01R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PA01R { bits }
    }
    #[doc = "Bits 4:5 - 00 GPIO 01 TMRSE1_IN 10/11"]
    #[inline]
    pub fn pa02(&self) -> PA02R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PA02R { bits }
    }
    #[doc = "Bits 6:7 - 00 GPIO 01 10/11"]
    #[inline]
    pub fn pa03(&self) -> PA03R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PA03R { bits }
    }
    #[doc = "Bits 8:9 - 00 GPIO 01 10/11"]
    #[inline]
    pub fn pa04(&self) -> PA04R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PA04R { bits }
    }
    #[doc = "Bits 10:11 - 00 GPIO 01 10/11"]
    #[inline]
    pub fn pa05(&self) -> PA05R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PA05R { bits }
    }
    #[doc = "Bits 12:13 - 00 GPIO 01 10/11"]
    #[inline]
    pub fn pa06(&self) -> PA06R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PA06R { bits }
    }
    #[doc = "Bits 14:15 - 00 GPIO 01 TMRSE1OUT 10/11"]
    #[inline]
    pub fn pa07(&self) -> PA07R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PA07R { bits }
    }
    #[doc = "Bits 16:17 - 00 GPIO 01 10/11"]
    #[inline]
    pub fn pa08(&self) -> PA08R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PA08R { bits }
    }
    #[doc = "Bits 18:19 - 00 GPIO 01 10/11"]
    #[inline]
    pub fn pa09(&self) -> PA09R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PA09R { bits }
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
    #[doc = "Bits 0:1 - 00 GPIO 01 TMRSE0_IN 10/11"]
    #[inline]
    pub fn pa00(&mut self) -> _PA00W {
        _PA00W { w: self }
    }
    #[doc = "Bits 2:3 - 00 GPIO 01 TMRSE0_OUT 10/11"]
    #[inline]
    pub fn pa01(&mut self) -> _PA01W {
        _PA01W { w: self }
    }
    #[doc = "Bits 4:5 - 00 GPIO 01 TMRSE1_IN 10/11"]
    #[inline]
    pub fn pa02(&mut self) -> _PA02W {
        _PA02W { w: self }
    }
    #[doc = "Bits 6:7 - 00 GPIO 01 10/11"]
    #[inline]
    pub fn pa03(&mut self) -> _PA03W {
        _PA03W { w: self }
    }
    #[doc = "Bits 8:9 - 00 GPIO 01 10/11"]
    #[inline]
    pub fn pa04(&mut self) -> _PA04W {
        _PA04W { w: self }
    }
    #[doc = "Bits 10:11 - 00 GPIO 01 10/11"]
    #[inline]
    pub fn pa05(&mut self) -> _PA05W {
        _PA05W { w: self }
    }
    #[doc = "Bits 12:13 - 00 GPIO 01 10/11"]
    #[inline]
    pub fn pa06(&mut self) -> _PA06W {
        _PA06W { w: self }
    }
    #[doc = "Bits 14:15 - 00 GPIO 01 TMRSE1OUT 10/11"]
    #[inline]
    pub fn pa07(&mut self) -> _PA07W {
        _PA07W { w: self }
    }
    #[doc = "Bits 16:17 - 00 GPIO 01 10/11"]
    #[inline]
    pub fn pa08(&mut self) -> _PA08W {
        _PA08W { w: self }
    }
    #[doc = "Bits 18:19 - 00 GPIO 01 10/11"]
    #[inline]
    pub fn pa09(&mut self) -> _PA09W {
        _PA09W { w: self }
    }
}
