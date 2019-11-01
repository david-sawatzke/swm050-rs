#[doc = "Reader of register INTSTAT"]
pub type R = crate::R<u32, super::INTSTAT>;
#[doc = "Reader of field `INTSTAT0`"]
pub type INTSTAT0_R = crate::R<bool, bool>;
#[doc = "Reader of field `INTSTAT1`"]
pub type INTSTAT1_R = crate::R<bool, bool>;
#[doc = "Reader of field `INTSTAT2`"]
pub type INTSTAT2_R = crate::R<bool, bool>;
#[doc = "Reader of field `INTSTAT3`"]
pub type INTSTAT3_R = crate::R<bool, bool>;
#[doc = "Reader of field `INTSTAT4`"]
pub type INTSTAT4_R = crate::R<bool, bool>;
#[doc = "Reader of field `INTSTAT5`"]
pub type INTSTAT5_R = crate::R<bool, bool>;
#[doc = "Reader of field `INTSTAT6`"]
pub type INTSTAT6_R = crate::R<bool, bool>;
#[doc = "Reader of field `INTSTAT7`"]
pub type INTSTAT7_R = crate::R<bool, bool>;
#[doc = "Reader of field `INTSTAT8`"]
pub type INTSTAT8_R = crate::R<bool, bool>;
#[doc = "Reader of field `INTSTAT9`"]
pub type INTSTAT9_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - INTSTAT0"]
    #[inline(always)]
    pub fn intstat0(&self) -> INTSTAT0_R {
        INTSTAT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - INTSTAT1"]
    #[inline(always)]
    pub fn intstat1(&self) -> INTSTAT1_R {
        INTSTAT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - INTSTAT2"]
    #[inline(always)]
    pub fn intstat2(&self) -> INTSTAT2_R {
        INTSTAT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - INTSTAT3"]
    #[inline(always)]
    pub fn intstat3(&self) -> INTSTAT3_R {
        INTSTAT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - INTSTAT4"]
    #[inline(always)]
    pub fn intstat4(&self) -> INTSTAT4_R {
        INTSTAT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - INTSTAT5"]
    #[inline(always)]
    pub fn intstat5(&self) -> INTSTAT5_R {
        INTSTAT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - INTSTAT6"]
    #[inline(always)]
    pub fn intstat6(&self) -> INTSTAT6_R {
        INTSTAT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - INTSTAT7"]
    #[inline(always)]
    pub fn intstat7(&self) -> INTSTAT7_R {
        INTSTAT7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - INTSTAT8"]
    #[inline(always)]
    pub fn intstat8(&self) -> INTSTAT8_R {
        INTSTAT8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - INTSTAT9"]
    #[inline(always)]
    pub fn intstat9(&self) -> INTSTAT9_R {
        INTSTAT9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
