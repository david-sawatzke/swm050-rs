#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL {
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
pub struct ENAR {
    bits: bool,
}
impl ENAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Possible values of the field `WMOD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WMODR {
    #[doc = "Timer/Counter Mode"]
    COUNTER,
    #[doc = "PWM Mode"]
    PWM,
    #[doc = "Pulse Capture mode"]
    PULSE,
    #[doc = "Duty Cycle capture mode"]
    DUTY_CYCLE,
}
impl WMODR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WMODR::COUNTER => 0,
            WMODR::PWM => 1,
            WMODR::PULSE => 2,
            WMODR::DUTY_CYCLE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WMODR {
        match value {
            0 => WMODR::COUNTER,
            1 => WMODR::PWM,
            2 => WMODR::PULSE,
            3 => WMODR::DUTY_CYCLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `COUNTER`"]
    #[inline]
    pub fn is_counter(&self) -> bool {
        *self == WMODR::COUNTER
    }
    #[doc = "Checks if the value of the field is `PWM`"]
    #[inline]
    pub fn is_pwm(&self) -> bool {
        *self == WMODR::PWM
    }
    #[doc = "Checks if the value of the field is `PULSE`"]
    #[inline]
    pub fn is_pulse(&self) -> bool {
        *self == WMODR::PULSE
    }
    #[doc = "Checks if the value of the field is `DUTY_CYCLE`"]
    #[inline]
    pub fn is_duty_cycle(&self) -> bool {
        *self == WMODR::DUTY_CYCLE
    }
}
#[doc = r" Value of the field"]
pub struct OSCMODR {
    bits: bool,
}
impl OSCMODR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct OUTMODR {
    bits: u8,
}
impl OUTMODR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EDGE_FR {
    bits: bool,
}
impl EDGE_FR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct VALSAVER {
    bits: bool,
}
impl VALSAVER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct SINGLER {
    bits: bool,
}
impl SINGLER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Proxy"]
pub struct _ENAW<'a> {
    w: &'a mut W,
}
impl<'a> _ENAW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WMOD`"]
pub enum WMODW {
    #[doc = "Timer/Counter Mode"]
    COUNTER,
    #[doc = "PWM Mode"]
    PWM,
    #[doc = "Pulse Capture mode"]
    PULSE,
    #[doc = "Duty Cycle capture mode"]
    DUTY_CYCLE,
}
impl WMODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WMODW::COUNTER => 0,
            WMODW::PWM => 1,
            WMODW::PULSE => 2,
            WMODW::DUTY_CYCLE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WMODW<'a> {
    w: &'a mut W,
}
impl<'a> _WMODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WMODW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Timer/Counter Mode"]
    #[inline]
    pub fn counter(self) -> &'a mut W {
        self.variant(WMODW::COUNTER)
    }
    #[doc = "PWM Mode"]
    #[inline]
    pub fn pwm(self) -> &'a mut W {
        self.variant(WMODW::PWM)
    }
    #[doc = "Pulse Capture mode"]
    #[inline]
    pub fn pulse(self) -> &'a mut W {
        self.variant(WMODW::PULSE)
    }
    #[doc = "Duty Cycle capture mode"]
    #[inline]
    pub fn duty_cycle(self) -> &'a mut W {
        self.variant(WMODW::DUTY_CYCLE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OSCMODW<'a> {
    w: &'a mut W,
}
impl<'a> _OSCMODW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OUTMODW<'a> {
    w: &'a mut W,
}
impl<'a> _OUTMODW<'a> {
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
pub struct _EDGE_FW<'a> {
    w: &'a mut W,
}
impl<'a> _EDGE_FW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VALSAVEW<'a> {
    w: &'a mut W,
}
impl<'a> _VALSAVEW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SINGLEW<'a> {
    w: &'a mut W,
}
impl<'a> _SINGLEW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 28;
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
    #[doc = "Bit 0 - Enable Timer"]
    #[inline]
    pub fn ena(&self) -> ENAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENAR { bits }
    }
    #[doc = "Bits 4:5 - Operating Mode Selection"]
    #[inline]
    pub fn wmod(&self) -> WMODR {
        WMODR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - 0 Internal 1 External"]
    #[inline]
    pub fn oscmod(&self) -> OSCMODR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OSCMODR { bits }
    }
    #[doc = "Bits 12:13 - 00 No output 01 Invert 10 Set high 11 Set low"]
    #[inline]
    pub fn outmod(&self) -> OUTMODR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        OUTMODR { bits }
    }
    #[doc = "Bit 16 - 0 Rising edge 1 Falling edge"]
    #[inline]
    pub fn edge_f(&self) -> EDGE_FR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EDGE_FR { bits }
    }
    #[doc = "Bit 24 - Keep count value"]
    #[inline]
    pub fn valsave(&self) -> VALSAVER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VALSAVER { bits }
    }
    #[doc = "Bit 28 - 1 single mode 0 circular mode"]
    #[inline]
    pub fn single(&self) -> SINGLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SINGLER { bits }
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
    #[doc = "Bit 0 - Enable Timer"]
    #[inline]
    pub fn ena(&mut self) -> _ENAW {
        _ENAW { w: self }
    }
    #[doc = "Bits 4:5 - Operating Mode Selection"]
    #[inline]
    pub fn wmod(&mut self) -> _WMODW {
        _WMODW { w: self }
    }
    #[doc = "Bit 8 - 0 Internal 1 External"]
    #[inline]
    pub fn oscmod(&mut self) -> _OSCMODW {
        _OSCMODW { w: self }
    }
    #[doc = "Bits 12:13 - 00 No output 01 Invert 10 Set high 11 Set low"]
    #[inline]
    pub fn outmod(&mut self) -> _OUTMODW {
        _OUTMODW { w: self }
    }
    #[doc = "Bit 16 - 0 Rising edge 1 Falling edge"]
    #[inline]
    pub fn edge_f(&mut self) -> _EDGE_FW {
        _EDGE_FW { w: self }
    }
    #[doc = "Bit 24 - Keep count value"]
    #[inline]
    pub fn valsave(&mut self) -> _VALSAVEW {
        _VALSAVEW { w: self }
    }
    #[doc = "Bit 28 - 1 single mode 0 circular mode"]
    #[inline]
    pub fn single(&mut self) -> _SINGLEW {
        _SINGLEW { w: self }
    }
}
