#[doc = "Reader of register OSC_DIS"]
pub type R = crate::R<u32, super::OSC_DIS>;
#[doc = "Writer for register OSC_DIS"]
pub type W = crate::W<u32, super::OSC_DIS>;
#[doc = "Register OSC_DIS `reset()`'s with value 0"]
impl crate::ResetValue for super::OSC_DIS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OSC_DIS`"]
pub type OSC_DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OSC_DIS`"]
pub struct OSC_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> OSC_DIS_W<'a> {
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
#[doc = "Reader of field `PWR_DWN`"]
pub type PWR_DWN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWR_DWN`"]
pub struct PWR_DWN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_DWN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `MOS_DIS`"]
pub type MOS_DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MOS_DIS`"]
pub struct MOS_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> MOS_DIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - OSC_DIS"]
    #[inline(always)]
    pub fn osc_dis(&self) -> OSC_DIS_R {
        OSC_DIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - 1 Power-down"]
    #[inline(always)]
    pub fn pwr_dwn(&self) -> PWR_DWN_R {
        PWR_DWN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MOS_DIS"]
    #[inline(always)]
    pub fn mos_dis(&self) -> MOS_DIS_R {
        MOS_DIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OSC_DIS"]
    #[inline(always)]
    pub fn osc_dis(&mut self) -> OSC_DIS_W {
        OSC_DIS_W { w: self }
    }
    #[doc = "Bit 4 - 1 Power-down"]
    #[inline(always)]
    pub fn pwr_dwn(&mut self) -> PWR_DWN_W {
        PWR_DWN_W { w: self }
    }
    #[doc = "Bit 5 - MOS_DIS"]
    #[inline(always)]
    pub fn mos_dis(&mut self) -> MOS_DIS_W {
        MOS_DIS_W { w: self }
    }
}
