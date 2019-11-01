#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ENA`"]
pub type ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENA`"]
pub struct ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA_W<'a> {
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
#[doc = "Operating Mode Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WMOD_A {
    #[doc = "0: Timer/Counter Mode"]
    COUNTER,
    #[doc = "1: PWM Mode"]
    PWM,
    #[doc = "2: Pulse Capture mode"]
    PULSE,
    #[doc = "3: Duty Cycle capture mode"]
    DUTY_CYCLE,
}
impl From<WMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: WMOD_A) -> Self {
        match variant {
            WMOD_A::COUNTER => 0,
            WMOD_A::PWM => 1,
            WMOD_A::PULSE => 2,
            WMOD_A::DUTY_CYCLE => 3,
        }
    }
}
#[doc = "Reader of field `WMOD`"]
pub type WMOD_R = crate::R<u8, WMOD_A>;
impl WMOD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WMOD_A {
        match self.bits {
            0 => WMOD_A::COUNTER,
            1 => WMOD_A::PWM,
            2 => WMOD_A::PULSE,
            3 => WMOD_A::DUTY_CYCLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `COUNTER`"]
    #[inline(always)]
    pub fn is_counter(&self) -> bool {
        *self == WMOD_A::COUNTER
    }
    #[doc = "Checks if the value of the field is `PWM`"]
    #[inline(always)]
    pub fn is_pwm(&self) -> bool {
        *self == WMOD_A::PWM
    }
    #[doc = "Checks if the value of the field is `PULSE`"]
    #[inline(always)]
    pub fn is_pulse(&self) -> bool {
        *self == WMOD_A::PULSE
    }
    #[doc = "Checks if the value of the field is `DUTY_CYCLE`"]
    #[inline(always)]
    pub fn is_duty_cycle(&self) -> bool {
        *self == WMOD_A::DUTY_CYCLE
    }
}
#[doc = "Write proxy for field `WMOD`"]
pub struct WMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> WMOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WMOD_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Timer/Counter Mode"]
    #[inline(always)]
    pub fn counter(self) -> &'a mut W {
        self.variant(WMOD_A::COUNTER)
    }
    #[doc = "PWM Mode"]
    #[inline(always)]
    pub fn pwm(self) -> &'a mut W {
        self.variant(WMOD_A::PWM)
    }
    #[doc = "Pulse Capture mode"]
    #[inline(always)]
    pub fn pulse(self) -> &'a mut W {
        self.variant(WMOD_A::PULSE)
    }
    #[doc = "Duty Cycle capture mode"]
    #[inline(always)]
    pub fn duty_cycle(self) -> &'a mut W {
        self.variant(WMOD_A::DUTY_CYCLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `OSCMOD`"]
pub type OSCMOD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OSCMOD`"]
pub struct OSCMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> OSCMOD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `OUTMOD`"]
pub type OUTMOD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OUTMOD`"]
pub struct OUTMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTMOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `EDGE_F`"]
pub type EDGE_F_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EDGE_F`"]
pub struct EDGE_F_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGE_F_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `VALSAVE`"]
pub type VALSAVE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VALSAVE`"]
pub struct VALSAVE_W<'a> {
    w: &'a mut W,
}
impl<'a> VALSAVE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `SINGLE`"]
pub type SINGLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SINGLE`"]
pub struct SINGLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SINGLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable Timer"]
    #[inline(always)]
    pub fn ena(&self) -> ENA_R {
        ENA_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Operating Mode Selection"]
    #[inline(always)]
    pub fn wmod(&self) -> WMOD_R {
        WMOD_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 8 - 0 Internal 1 External"]
    #[inline(always)]
    pub fn oscmod(&self) -> OSCMOD_R {
        OSCMOD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - 00 No output 01 Invert 10 Set high 11 Set low"]
    #[inline(always)]
    pub fn outmod(&self) -> OUTMOD_R {
        OUTMOD_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 16 - 0 Rising edge 1 Falling edge"]
    #[inline(always)]
    pub fn edge_f(&self) -> EDGE_F_R {
        EDGE_F_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Keep count value"]
    #[inline(always)]
    pub fn valsave(&self) -> VALSAVE_R {
        VALSAVE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 28 - 1 single mode 0 circular mode"]
    #[inline(always)]
    pub fn single(&self) -> SINGLE_R {
        SINGLE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Timer"]
    #[inline(always)]
    pub fn ena(&mut self) -> ENA_W {
        ENA_W { w: self }
    }
    #[doc = "Bits 4:5 - Operating Mode Selection"]
    #[inline(always)]
    pub fn wmod(&mut self) -> WMOD_W {
        WMOD_W { w: self }
    }
    #[doc = "Bit 8 - 0 Internal 1 External"]
    #[inline(always)]
    pub fn oscmod(&mut self) -> OSCMOD_W {
        OSCMOD_W { w: self }
    }
    #[doc = "Bits 12:13 - 00 No output 01 Invert 10 Set high 11 Set low"]
    #[inline(always)]
    pub fn outmod(&mut self) -> OUTMOD_W {
        OUTMOD_W { w: self }
    }
    #[doc = "Bit 16 - 0 Rising edge 1 Falling edge"]
    #[inline(always)]
    pub fn edge_f(&mut self) -> EDGE_F_W {
        EDGE_F_W { w: self }
    }
    #[doc = "Bit 24 - Keep count value"]
    #[inline(always)]
    pub fn valsave(&mut self) -> VALSAVE_W {
        VALSAVE_W { w: self }
    }
    #[doc = "Bit 28 - 1 single mode 0 circular mode"]
    #[inline(always)]
    pub fn single(&mut self) -> SINGLE_W {
        SINGLE_W { w: self }
    }
}
