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
#[doc = r" Value of the field"]
pub struct WMODR {
    bits: u8,
}
impl WMODR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
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
#[doc = r" Proxy"]
pub struct _WMODW<'a> {
    w: &'a mut W,
}
impl<'a> _WMODW<'a> {
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
    #[doc = "Bit 0 - \u{6a21}\u{5757}\u{4f7f}\u{80fd}"]
    #[inline]
    pub fn ena(&self) -> ENAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENAR { bits }
    }
    #[doc = "Bits 4:5 - 00 \u{5b9a}\u{65f6}\u{5668}/\u{8ba1}\u{6570}\u{5668}\u{6a21}\u{5f0f} 01 PWM\u{6a21}\u{5f0f} 10 \u{8109}\u{5bbd}\u{6355}\u{6349}\u{6a21}\u{5f0f} 11 \u{5360}\u{7a7a}\u{6bd4}\u{6355}\u{6349}\u{6a21}\u{5f0f}"]
    #[inline]
    pub fn wmod(&self) -> WMODR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        WMODR { bits }
    }
    #[doc = "Bit 8 - 0 \u{5185}\u{90e8} 1 \u{5916}\u{90e8}"]
    #[inline]
    pub fn oscmod(&self) -> OSCMODR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OSCMODR { bits }
    }
    #[doc = "Bits 12:13 - 00 \u{65e0}\u{8f93}\u{51fa} 01 \u{53d6}\u{53cd} 10 \u{7f6e}\u{9ad8} 11 \u{7f6e}\u{4f4e}"]
    #[inline]
    pub fn outmod(&self) -> OUTMODR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        OUTMODR { bits }
    }
    #[doc = "Bit 16 - 0 \u{4e0a}\u{5347}\u{6cbf} 1 \u{4e0b}\u{964d}\u{6cbf}"]
    #[inline]
    pub fn edge_f(&self) -> EDGE_FR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EDGE_FR { bits }
    }
    #[doc = "Bit 24 - \u{4fdd}\u{7559}\u{8ba1}\u{6570}\u{503c}"]
    #[inline]
    pub fn valsave(&self) -> VALSAVER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VALSAVER { bits }
    }
    #[doc = "Bit 28 - 1 \u{5355}\u{6b21}\u{6a21}\u{5f0f} 0 \u{5faa}\u{73af}\u{6a21}\u{5f0f}"]
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
    #[doc = "Bit 0 - \u{6a21}\u{5757}\u{4f7f}\u{80fd}"]
    #[inline]
    pub fn ena(&mut self) -> _ENAW {
        _ENAW { w: self }
    }
    #[doc = "Bits 4:5 - 00 \u{5b9a}\u{65f6}\u{5668}/\u{8ba1}\u{6570}\u{5668}\u{6a21}\u{5f0f} 01 PWM\u{6a21}\u{5f0f} 10 \u{8109}\u{5bbd}\u{6355}\u{6349}\u{6a21}\u{5f0f} 11 \u{5360}\u{7a7a}\u{6bd4}\u{6355}\u{6349}\u{6a21}\u{5f0f}"]
    #[inline]
    pub fn wmod(&mut self) -> _WMODW {
        _WMODW { w: self }
    }
    #[doc = "Bit 8 - 0 \u{5185}\u{90e8} 1 \u{5916}\u{90e8}"]
    #[inline]
    pub fn oscmod(&mut self) -> _OSCMODW {
        _OSCMODW { w: self }
    }
    #[doc = "Bits 12:13 - 00 \u{65e0}\u{8f93}\u{51fa} 01 \u{53d6}\u{53cd} 10 \u{7f6e}\u{9ad8} 11 \u{7f6e}\u{4f4e}"]
    #[inline]
    pub fn outmod(&mut self) -> _OUTMODW {
        _OUTMODW { w: self }
    }
    #[doc = "Bit 16 - 0 \u{4e0a}\u{5347}\u{6cbf} 1 \u{4e0b}\u{964d}\u{6cbf}"]
    #[inline]
    pub fn edge_f(&mut self) -> _EDGE_FW {
        _EDGE_FW { w: self }
    }
    #[doc = "Bit 24 - \u{4fdd}\u{7559}\u{8ba1}\u{6570}\u{503c}"]
    #[inline]
    pub fn valsave(&mut self) -> _VALSAVEW {
        _VALSAVEW { w: self }
    }
    #[doc = "Bit 28 - 1 \u{5355}\u{6b21}\u{6a21}\u{5f0f} 0 \u{5faa}\u{73af}\u{6a21}\u{5f0f}"]
    #[inline]
    pub fn single(&mut self) -> _SINGLEW {
        _SINGLEW { w: self }
    }
}
