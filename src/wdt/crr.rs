#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CRR {
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
}
#[doc = "Values that can be written to the field `CRR`"]
pub enum CRRW {
    #[doc = "Reset WDT counter"]
    RESET,
}
impl CRRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CRRW::RESET => 118,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CRRW<'a> {
    w: &'a mut W,
}
impl<'a> _CRRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRRW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Reset WDT counter"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(CRRW::RESET)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[doc = "Bits 0:7"]
    #[inline]
    pub fn crr(&mut self) -> _CRRW {
        _CRRW { w: self }
    }
}
