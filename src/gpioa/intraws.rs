#[doc = "Reader of register INTRAWS"]
pub type R = crate::R<u32, super::INTRAWS>;
#[doc = "Reader of field `INTRAWS0`"]
pub type INTRAWS0_R = crate::R<bool, bool>;
#[doc = "Reader of field `INTRAWS1`"]
pub type INTRAWS1_R = crate::R<bool, bool>;
#[doc = "Reader of field `INTRAWS2`"]
pub type INTRAWS2_R = crate::R<bool, bool>;
#[doc = "Reader of field `INTRAWS3`"]
pub type INTRAWS3_R = crate::R<bool, bool>;
#[doc = "Reader of field `INTRAWS4`"]
pub type INTRAWS4_R = crate::R<bool, bool>;
#[doc = "Reader of field `INTRAWS5`"]
pub type INTRAWS5_R = crate::R<bool, bool>;
#[doc = "Reader of field `INTRAWS6`"]
pub type INTRAWS6_R = crate::R<bool, bool>;
#[doc = "Reader of field `INTRAWS7`"]
pub type INTRAWS7_R = crate::R<bool, bool>;
#[doc = "Reader of field `INTRAWS8`"]
pub type INTRAWS8_R = crate::R<bool, bool>;
#[doc = "Reader of field `INTRAWS9`"]
pub type INTRAWS9_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - INTRAWS0"]
    #[inline(always)]
    pub fn intraws0(&self) -> INTRAWS0_R {
        INTRAWS0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - INTRAWS1"]
    #[inline(always)]
    pub fn intraws1(&self) -> INTRAWS1_R {
        INTRAWS1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - INTRAWS2"]
    #[inline(always)]
    pub fn intraws2(&self) -> INTRAWS2_R {
        INTRAWS2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - INTRAWS3"]
    #[inline(always)]
    pub fn intraws3(&self) -> INTRAWS3_R {
        INTRAWS3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - INTRAWS4"]
    #[inline(always)]
    pub fn intraws4(&self) -> INTRAWS4_R {
        INTRAWS4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - INTRAWS5"]
    #[inline(always)]
    pub fn intraws5(&self) -> INTRAWS5_R {
        INTRAWS5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - INTRAWS6"]
    #[inline(always)]
    pub fn intraws6(&self) -> INTRAWS6_R {
        INTRAWS6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - INTRAWS7"]
    #[inline(always)]
    pub fn intraws7(&self) -> INTRAWS7_R {
        INTRAWS7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - INTRAWS8"]
    #[inline(always)]
    pub fn intraws8(&self) -> INTRAWS8_R {
        INTRAWS8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - INTRAWS9"]
    #[inline(always)]
    pub fn intraws9(&self) -> INTRAWS9_R {
        INTRAWS9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
