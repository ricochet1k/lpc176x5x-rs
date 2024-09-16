#[doc = "Register `IE` reader"]
pub type R = crate::R<IeSpec>;
#[doc = "Field `INX_INT` reader - When 1, the INX_Int interrupt is enabled."]
pub type InxIntR = crate::BitReader;
#[doc = "Field `TIM_INT` reader - When 1, the TIN_Int interrupt is enabled."]
pub type TimIntR = crate::BitReader;
#[doc = "Field `VELC_INT` reader - When 1, the VELC_Int interrupt is enabled."]
pub type VelcIntR = crate::BitReader;
#[doc = "Field `DIR_INT` reader - When 1, the DIR_Int interrupt is enabled."]
pub type DirIntR = crate::BitReader;
#[doc = "Field `ERR_INT` reader - When 1, the ERR_Int interrupt is enabled."]
pub type ErrIntR = crate::BitReader;
#[doc = "Field `ENCLK_INT` reader - When 1, the ENCLK_Int interrupt is enabled."]
pub type EnclkIntR = crate::BitReader;
#[doc = "Field `POS0_INT` reader - When 1, the POS0_Int interrupt is enabled."]
pub type Pos0IntR = crate::BitReader;
#[doc = "Field `POS1_INT` reader - When 1, the POS1_Int interrupt is enabled."]
pub type Pos1IntR = crate::BitReader;
#[doc = "Field `POS2_INT` reader - When 1, the POS2_Int interrupt is enabled."]
pub type Pos2IntR = crate::BitReader;
#[doc = "Field `REV0_INT` reader - When 1, the REV0_Int interrupt is enabled."]
pub type Rev0IntR = crate::BitReader;
#[doc = "Field `POS0REV_INT` reader - When 1, the POS0REV_Int interrupt is enabled."]
pub type Pos0revIntR = crate::BitReader;
#[doc = "Field `POS1REV_INT` reader - When 1, the POS1REV_Int interrupt is enabled."]
pub type Pos1revIntR = crate::BitReader;
#[doc = "Field `POS2REV_INT` reader - When 1, the POS2REV_Int interrupt is enabled."]
pub type Pos2revIntR = crate::BitReader;
#[doc = "Field `REV1_INT` reader - When 1, the REV1_Int interrupt is enabled."]
pub type Rev1IntR = crate::BitReader;
#[doc = "Field `REV2_INT` reader - When 1, the REV2_Int interrupt is enabled."]
pub type Rev2IntR = crate::BitReader;
#[doc = "Field `MAXPOS_INT` reader - When 1, the MAXPOS_Int interrupt is enabled."]
pub type MaxposIntR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - When 1, the INX_Int interrupt is enabled."]
    #[inline(always)]
    pub fn inx_int(&self) -> InxIntR {
        InxIntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When 1, the TIN_Int interrupt is enabled."]
    #[inline(always)]
    pub fn tim_int(&self) -> TimIntR {
        TimIntR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When 1, the VELC_Int interrupt is enabled."]
    #[inline(always)]
    pub fn velc_int(&self) -> VelcIntR {
        VelcIntR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When 1, the DIR_Int interrupt is enabled."]
    #[inline(always)]
    pub fn dir_int(&self) -> DirIntR {
        DirIntR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When 1, the ERR_Int interrupt is enabled."]
    #[inline(always)]
    pub fn err_int(&self) -> ErrIntR {
        ErrIntR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - When 1, the ENCLK_Int interrupt is enabled."]
    #[inline(always)]
    pub fn enclk_int(&self) -> EnclkIntR {
        EnclkIntR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - When 1, the POS0_Int interrupt is enabled."]
    #[inline(always)]
    pub fn pos0_int(&self) -> Pos0IntR {
        Pos0IntR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - When 1, the POS1_Int interrupt is enabled."]
    #[inline(always)]
    pub fn pos1_int(&self) -> Pos1IntR {
        Pos1IntR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - When 1, the POS2_Int interrupt is enabled."]
    #[inline(always)]
    pub fn pos2_int(&self) -> Pos2IntR {
        Pos2IntR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - When 1, the REV0_Int interrupt is enabled."]
    #[inline(always)]
    pub fn rev0_int(&self) -> Rev0IntR {
        Rev0IntR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - When 1, the POS0REV_Int interrupt is enabled."]
    #[inline(always)]
    pub fn pos0rev_int(&self) -> Pos0revIntR {
        Pos0revIntR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - When 1, the POS1REV_Int interrupt is enabled."]
    #[inline(always)]
    pub fn pos1rev_int(&self) -> Pos1revIntR {
        Pos1revIntR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - When 1, the POS2REV_Int interrupt is enabled."]
    #[inline(always)]
    pub fn pos2rev_int(&self) -> Pos2revIntR {
        Pos2revIntR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - When 1, the REV1_Int interrupt is enabled."]
    #[inline(always)]
    pub fn rev1_int(&self) -> Rev1IntR {
        Rev1IntR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - When 1, the REV2_Int interrupt is enabled."]
    #[inline(always)]
    pub fn rev2_int(&self) -> Rev2IntR {
        Rev2IntR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - When 1, the MAXPOS_Int interrupt is enabled."]
    #[inline(always)]
    pub fn maxpos_int(&self) -> MaxposIntR {
        MaxposIntR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ie::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IeSpec;
impl crate::RegisterSpec for IeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ie::R`](R) reader structure"]
impl crate::Readable for IeSpec {}
#[doc = "`reset()` method sets IE to value 0"]
impl crate::Resettable for IeSpec {
    const RESET_VALUE: u32 = 0;
}
