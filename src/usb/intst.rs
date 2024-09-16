#[doc = "Register `INTST` reader"]
pub type R = crate::R<IntstSpec>;
#[doc = "Field `TMR` reader - Timer time-out."]
pub type TmrR = crate::BitReader;
#[doc = "Field `REMOVE_PU` reader - Remove pull-up. This bit is set by hardware to indicate that software needs to disable the D+ pull-up resistor."]
pub type RemovePuR = crate::BitReader;
#[doc = "Field `HNP_FAILURE` reader - HNP failed. This bit is set by hardware to indicate that the HNP switching has failed."]
pub type HnpFailureR = crate::BitReader;
#[doc = "Field `HNP_SUCCESS` reader - HNP succeeded. This bit is set by hardware to indicate that the HNP switching has succeeded."]
pub type HnpSuccessR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Timer time-out."]
    #[inline(always)]
    pub fn tmr(&self) -> TmrR {
        TmrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Remove pull-up. This bit is set by hardware to indicate that software needs to disable the D+ pull-up resistor."]
    #[inline(always)]
    pub fn remove_pu(&self) -> RemovePuR {
        RemovePuR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HNP failed. This bit is set by hardware to indicate that the HNP switching has failed."]
    #[inline(always)]
    pub fn hnp_failure(&self) -> HnpFailureR {
        HnpFailureR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HNP succeeded. This bit is set by hardware to indicate that the HNP switching has succeeded."]
    #[inline(always)]
    pub fn hnp_success(&self) -> HnpSuccessR {
        HnpSuccessR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "OTG Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`intst::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntstSpec;
impl crate::RegisterSpec for IntstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intst::R`](R) reader structure"]
impl crate::Readable for IntstSpec {}
#[doc = "`reset()` method sets INTST to value 0"]
impl crate::Resettable for IntstSpec {
    const RESET_VALUE: u32 = 0;
}
