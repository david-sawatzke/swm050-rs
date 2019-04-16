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
#[doc = "Possible values of the field `PA00`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PA00R {
    #[doc = "GPIO function"]
    GPIO,
    #[doc = "Timer SE0 input"]
    TMRSE0_IN,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PA00R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PA00R::GPIO => 0,
            PA00R::TMRSE0_IN => 1,
            PA00R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PA00R {
        match value {
            0 => PA00R::GPIO,
            1 => PA00R::TMRSE0_IN,
            i => PA00R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO`"]
    #[inline]
    pub fn is_gpio(&self) -> bool {
        *self == PA00R::GPIO
    }
    #[doc = "Checks if the value of the field is `TMRSE0_IN`"]
    #[inline]
    pub fn is_tmrse0_in(&self) -> bool {
        *self == PA00R::TMRSE0_IN
    }
}
#[doc = "Possible values of the field `PA01`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PA01R {
    #[doc = "GPIO function"]
    GPIO,
    #[doc = "Timer SE0 output"]
    TMRSE0_OUT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PA01R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PA01R::GPIO => 0,
            PA01R::TMRSE0_OUT => 1,
            PA01R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PA01R {
        match value {
            0 => PA01R::GPIO,
            1 => PA01R::TMRSE0_OUT,
            i => PA01R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO`"]
    #[inline]
    pub fn is_gpio(&self) -> bool {
        *self == PA01R::GPIO
    }
    #[doc = "Checks if the value of the field is `TMRSE0_OUT`"]
    #[inline]
    pub fn is_tmrse0_out(&self) -> bool {
        *self == PA01R::TMRSE0_OUT
    }
}
#[doc = "Possible values of the field `PA02`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PA02R {
    #[doc = "GPIO function"]
    GPIO,
    #[doc = "Timer SE1 input"]
    TMRSE1_IN,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PA02R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PA02R::GPIO => 0,
            PA02R::TMRSE1_IN => 1,
            PA02R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PA02R {
        match value {
            0 => PA02R::GPIO,
            1 => PA02R::TMRSE1_IN,
            i => PA02R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO`"]
    #[inline]
    pub fn is_gpio(&self) -> bool {
        *self == PA02R::GPIO
    }
    #[doc = "Checks if the value of the field is `TMRSE1_IN`"]
    #[inline]
    pub fn is_tmrse1_in(&self) -> bool {
        *self == PA02R::TMRSE1_IN
    }
}
#[doc = "Possible values of the field `PA07`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PA07R {
    #[doc = "GPIO function"]
    GPIO,
    #[doc = "Timer SE1 output"]
    TMRSE1_OUT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PA07R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PA07R::GPIO => 0,
            PA07R::TMRSE1_OUT => 1,
            PA07R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PA07R {
        match value {
            0 => PA07R::GPIO,
            1 => PA07R::TMRSE1_OUT,
            i => PA07R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO`"]
    #[inline]
    pub fn is_gpio(&self) -> bool {
        *self == PA07R::GPIO
    }
    #[doc = "Checks if the value of the field is `TMRSE1_OUT`"]
    #[inline]
    pub fn is_tmrse1_out(&self) -> bool {
        *self == PA07R::TMRSE1_OUT
    }
}
#[doc = "Values that can be written to the field `PA00`"]
pub enum PA00W {
    #[doc = "GPIO function"]
    GPIO,
    #[doc = "Timer SE0 input"]
    TMRSE0_IN,
}
impl PA00W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PA00W::GPIO => 0,
            PA00W::TMRSE0_IN => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PA00W<'a> {
    w: &'a mut W,
}
impl<'a> _PA00W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PA00W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "GPIO function"]
    #[inline]
    pub fn gpio(self) -> &'a mut W {
        self.variant(PA00W::GPIO)
    }
    #[doc = "Timer SE0 input"]
    #[inline]
    pub fn tmrse0_in(self) -> &'a mut W {
        self.variant(PA00W::TMRSE0_IN)
    }
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
#[doc = "Values that can be written to the field `PA01`"]
pub enum PA01W {
    #[doc = "GPIO function"]
    GPIO,
    #[doc = "Timer SE0 output"]
    TMRSE0_OUT,
}
impl PA01W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PA01W::GPIO => 0,
            PA01W::TMRSE0_OUT => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PA01W<'a> {
    w: &'a mut W,
}
impl<'a> _PA01W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PA01W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "GPIO function"]
    #[inline]
    pub fn gpio(self) -> &'a mut W {
        self.variant(PA01W::GPIO)
    }
    #[doc = "Timer SE0 output"]
    #[inline]
    pub fn tmrse0_out(self) -> &'a mut W {
        self.variant(PA01W::TMRSE0_OUT)
    }
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
#[doc = "Values that can be written to the field `PA02`"]
pub enum PA02W {
    #[doc = "GPIO function"]
    GPIO,
    #[doc = "Timer SE1 input"]
    TMRSE1_IN,
}
impl PA02W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PA02W::GPIO => 0,
            PA02W::TMRSE1_IN => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PA02W<'a> {
    w: &'a mut W,
}
impl<'a> _PA02W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PA02W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "GPIO function"]
    #[inline]
    pub fn gpio(self) -> &'a mut W {
        self.variant(PA02W::GPIO)
    }
    #[doc = "Timer SE1 input"]
    #[inline]
    pub fn tmrse1_in(self) -> &'a mut W {
        self.variant(PA02W::TMRSE1_IN)
    }
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
#[doc = "Values that can be written to the field `PA07`"]
pub enum PA07W {
    #[doc = "GPIO function"]
    GPIO,
    #[doc = "Timer SE1 output"]
    TMRSE1_OUT,
}
impl PA07W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PA07W::GPIO => 0,
            PA07W::TMRSE1_OUT => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PA07W<'a> {
    w: &'a mut W,
}
impl<'a> _PA07W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PA07W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "GPIO function"]
    #[inline]
    pub fn gpio(self) -> &'a mut W {
        self.variant(PA07W::GPIO)
    }
    #[doc = "Timer SE1 output"]
    #[inline]
    pub fn tmrse1_out(self) -> &'a mut W {
        self.variant(PA07W::TMRSE1_OUT)
    }
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - GPIOA Pin 0"]
    #[inline]
    pub fn pa00(&self) -> PA00R {
        PA00R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - GPIOA Pin 1"]
    #[inline]
    pub fn pa01(&self) -> PA01R {
        PA01R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - GPIOA Pin 2"]
    #[inline]
    pub fn pa02(&self) -> PA02R {
        PA02R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 14:15 - GPIOA Pin 7"]
    #[inline]
    pub fn pa07(&self) -> PA07R {
        PA07R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
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
    #[doc = "Bits 0:1 - GPIOA Pin 0"]
    #[inline]
    pub fn pa00(&mut self) -> _PA00W {
        _PA00W { w: self }
    }
    #[doc = "Bits 2:3 - GPIOA Pin 1"]
    #[inline]
    pub fn pa01(&mut self) -> _PA01W {
        _PA01W { w: self }
    }
    #[doc = "Bits 4:5 - GPIOA Pin 2"]
    #[inline]
    pub fn pa02(&mut self) -> _PA02W {
        _PA02W { w: self }
    }
    #[doc = "Bits 14:15 - GPIOA Pin 7"]
    #[inline]
    pub fn pa07(&mut self) -> _PA07W {
        _PA07W { w: self }
    }
}
