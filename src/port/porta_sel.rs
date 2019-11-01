#[doc = "Reader of register PORTA_SEL"]
pub type R = crate::R<u32, super::PORTA_SEL>;
#[doc = "Writer for register PORTA_SEL"]
pub type W = crate::W<u32, super::PORTA_SEL>;
#[doc = "Register PORTA_SEL `reset()`'s with value 0"]
impl crate::ResetValue for super::PORTA_SEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "GPIOA Pin 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PA00_A {
    #[doc = "0: GPIO function"]
    GPIO,
    #[doc = "1: Timer SE0 input"]
    TMRSE0_IN,
}
impl From<PA00_A> for u8 {
    #[inline(always)]
    fn from(variant: PA00_A) -> Self {
        match variant {
            PA00_A::GPIO => 0,
            PA00_A::TMRSE0_IN => 1,
        }
    }
}
#[doc = "Reader of field `PA00`"]
pub type PA00_R = crate::R<u8, PA00_A>;
impl PA00_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PA00_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PA00_A::GPIO),
            1 => Val(PA00_A::TMRSE0_IN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO`"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == PA00_A::GPIO
    }
    #[doc = "Checks if the value of the field is `TMRSE0_IN`"]
    #[inline(always)]
    pub fn is_tmrse0_in(&self) -> bool {
        *self == PA00_A::TMRSE0_IN
    }
}
#[doc = "Write proxy for field `PA00`"]
pub struct PA00_W<'a> {
    w: &'a mut W,
}
impl<'a> PA00_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PA00_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "GPIO function"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut W {
        self.variant(PA00_A::GPIO)
    }
    #[doc = "Timer SE0 input"]
    #[inline(always)]
    pub fn tmrse0_in(self) -> &'a mut W {
        self.variant(PA00_A::TMRSE0_IN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "GPIOA Pin 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PA01_A {
    #[doc = "0: GPIO function"]
    GPIO,
    #[doc = "1: Timer SE0 output"]
    TMRSE0_OUT,
}
impl From<PA01_A> for u8 {
    #[inline(always)]
    fn from(variant: PA01_A) -> Self {
        match variant {
            PA01_A::GPIO => 0,
            PA01_A::TMRSE0_OUT => 1,
        }
    }
}
#[doc = "Reader of field `PA01`"]
pub type PA01_R = crate::R<u8, PA01_A>;
impl PA01_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PA01_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PA01_A::GPIO),
            1 => Val(PA01_A::TMRSE0_OUT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO`"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == PA01_A::GPIO
    }
    #[doc = "Checks if the value of the field is `TMRSE0_OUT`"]
    #[inline(always)]
    pub fn is_tmrse0_out(&self) -> bool {
        *self == PA01_A::TMRSE0_OUT
    }
}
#[doc = "Write proxy for field `PA01`"]
pub struct PA01_W<'a> {
    w: &'a mut W,
}
impl<'a> PA01_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PA01_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "GPIO function"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut W {
        self.variant(PA01_A::GPIO)
    }
    #[doc = "Timer SE0 output"]
    #[inline(always)]
    pub fn tmrse0_out(self) -> &'a mut W {
        self.variant(PA01_A::TMRSE0_OUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "GPIOA Pin 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PA02_A {
    #[doc = "0: GPIO function"]
    GPIO,
    #[doc = "1: Timer SE1 input"]
    TMRSE1_IN,
}
impl From<PA02_A> for u8 {
    #[inline(always)]
    fn from(variant: PA02_A) -> Self {
        match variant {
            PA02_A::GPIO => 0,
            PA02_A::TMRSE1_IN => 1,
        }
    }
}
#[doc = "Reader of field `PA02`"]
pub type PA02_R = crate::R<u8, PA02_A>;
impl PA02_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PA02_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PA02_A::GPIO),
            1 => Val(PA02_A::TMRSE1_IN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO`"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == PA02_A::GPIO
    }
    #[doc = "Checks if the value of the field is `TMRSE1_IN`"]
    #[inline(always)]
    pub fn is_tmrse1_in(&self) -> bool {
        *self == PA02_A::TMRSE1_IN
    }
}
#[doc = "Write proxy for field `PA02`"]
pub struct PA02_W<'a> {
    w: &'a mut W,
}
impl<'a> PA02_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PA02_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "GPIO function"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut W {
        self.variant(PA02_A::GPIO)
    }
    #[doc = "Timer SE1 input"]
    #[inline(always)]
    pub fn tmrse1_in(self) -> &'a mut W {
        self.variant(PA02_A::TMRSE1_IN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "GPIOA Pin 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PA07_A {
    #[doc = "0: GPIO function"]
    GPIO,
    #[doc = "1: Timer SE1 output"]
    TMRSE1_OUT,
}
impl From<PA07_A> for u8 {
    #[inline(always)]
    fn from(variant: PA07_A) -> Self {
        match variant {
            PA07_A::GPIO => 0,
            PA07_A::TMRSE1_OUT => 1,
        }
    }
}
#[doc = "Reader of field `PA07`"]
pub type PA07_R = crate::R<u8, PA07_A>;
impl PA07_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PA07_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PA07_A::GPIO),
            1 => Val(PA07_A::TMRSE1_OUT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO`"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == PA07_A::GPIO
    }
    #[doc = "Checks if the value of the field is `TMRSE1_OUT`"]
    #[inline(always)]
    pub fn is_tmrse1_out(&self) -> bool {
        *self == PA07_A::TMRSE1_OUT
    }
}
#[doc = "Write proxy for field `PA07`"]
pub struct PA07_W<'a> {
    w: &'a mut W,
}
impl<'a> PA07_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PA07_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "GPIO function"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut W {
        self.variant(PA07_A::GPIO)
    }
    #[doc = "Timer SE1 output"]
    #[inline(always)]
    pub fn tmrse1_out(self) -> &'a mut W {
        self.variant(PA07_A::TMRSE1_OUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - GPIOA Pin 0"]
    #[inline(always)]
    pub fn pa00(&self) -> PA00_R {
        PA00_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - GPIOA Pin 1"]
    #[inline(always)]
    pub fn pa01(&self) -> PA01_R {
        PA01_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - GPIOA Pin 2"]
    #[inline(always)]
    pub fn pa02(&self) -> PA02_R {
        PA02_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - GPIOA Pin 7"]
    #[inline(always)]
    pub fn pa07(&self) -> PA07_R {
        PA07_R::new(((self.bits >> 14) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPIOA Pin 0"]
    #[inline(always)]
    pub fn pa00(&mut self) -> PA00_W {
        PA00_W { w: self }
    }
    #[doc = "Bits 2:3 - GPIOA Pin 1"]
    #[inline(always)]
    pub fn pa01(&mut self) -> PA01_W {
        PA01_W { w: self }
    }
    #[doc = "Bits 4:5 - GPIOA Pin 2"]
    #[inline(always)]
    pub fn pa02(&mut self) -> PA02_W {
        PA02_W { w: self }
    }
    #[doc = "Bits 14:15 - GPIOA Pin 7"]
    #[inline(always)]
    pub fn pa07(&mut self) -> PA07_W {
        PA07_W { w: self }
    }
}
