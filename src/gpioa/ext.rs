#[doc = "Reader of register EXT"]
pub type R = crate::R<u32, super::EXT>;
#[doc = "Reader of field `EXT0`"]
pub type EXT0_R = crate::R<bool, bool>;
#[doc = "Reader of field `EXT1`"]
pub type EXT1_R = crate::R<bool, bool>;
#[doc = "Reader of field `EXT2`"]
pub type EXT2_R = crate::R<bool, bool>;
#[doc = "Reader of field `EXT3`"]
pub type EXT3_R = crate::R<bool, bool>;
#[doc = "Reader of field `EXT4`"]
pub type EXT4_R = crate::R<bool, bool>;
#[doc = "Reader of field `EXT5`"]
pub type EXT5_R = crate::R<bool, bool>;
#[doc = "Reader of field `EXT6`"]
pub type EXT6_R = crate::R<bool, bool>;
#[doc = "Reader of field `EXT7`"]
pub type EXT7_R = crate::R<bool, bool>;
#[doc = "Reader of field `EXT8`"]
pub type EXT8_R = crate::R<bool, bool>;
#[doc = "Reader of field `EXT9`"]
pub type EXT9_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - EXT0"]
    #[inline(always)]
    pub fn ext0(&self) -> EXT0_R {
        EXT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - EXT1"]
    #[inline(always)]
    pub fn ext1(&self) -> EXT1_R {
        EXT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - EXT2"]
    #[inline(always)]
    pub fn ext2(&self) -> EXT2_R {
        EXT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - EXT3"]
    #[inline(always)]
    pub fn ext3(&self) -> EXT3_R {
        EXT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - EXT4"]
    #[inline(always)]
    pub fn ext4(&self) -> EXT4_R {
        EXT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - EXT5"]
    #[inline(always)]
    pub fn ext5(&self) -> EXT5_R {
        EXT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - EXT6"]
    #[inline(always)]
    pub fn ext6(&self) -> EXT6_R {
        EXT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - EXT7"]
    #[inline(always)]
    pub fn ext7(&self) -> EXT7_R {
        EXT7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - EXT8"]
    #[inline(always)]
    pub fn ext8(&self) -> EXT8_R {
        EXT8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - EXT9"]
    #[inline(always)]
    pub fn ext9(&self) -> EXT9_R {
        EXT9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
