#[doc = "Register `INTEN` reader"]
pub type R = crate::R<IntenSpec>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<IntenSpec>;
#[doc = "Field `TMR_EN` reader - 1 = enable the corresponding bit in the IntSt register."]
pub type TmrEnR = crate::BitReader;
#[doc = "Field `TMR_EN` writer - 1 = enable the corresponding bit in the IntSt register."]
pub type TmrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REMOVE_PU_EN` reader - 1 = enable the corresponding bit in the IntSt register."]
pub type RemovePuEnR = crate::BitReader;
#[doc = "Field `REMOVE_PU_EN` writer - 1 = enable the corresponding bit in the IntSt register."]
pub type RemovePuEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HNP_FAILURE_EN` reader - 1 = enable the corresponding bit in the IntSt register."]
pub type HnpFailureEnR = crate::BitReader;
#[doc = "Field `HNP_FAILURE_EN` writer - 1 = enable the corresponding bit in the IntSt register."]
pub type HnpFailureEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HNP_SUCCES_EN` reader - 1 = enable the corresponding bit in the IntSt register."]
pub type HnpSuccesEnR = crate::BitReader;
#[doc = "Field `HNP_SUCCES_EN` writer - 1 = enable the corresponding bit in the IntSt register."]
pub type HnpSuccesEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1 = enable the corresponding bit in the IntSt register."]
    #[inline(always)]
    pub fn tmr_en(&self) -> TmrEnR {
        TmrEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1 = enable the corresponding bit in the IntSt register."]
    #[inline(always)]
    pub fn remove_pu_en(&self) -> RemovePuEnR {
        RemovePuEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1 = enable the corresponding bit in the IntSt register."]
    #[inline(always)]
    pub fn hnp_failure_en(&self) -> HnpFailureEnR {
        HnpFailureEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1 = enable the corresponding bit in the IntSt register."]
    #[inline(always)]
    pub fn hnp_succes_en(&self) -> HnpSuccesEnR {
        HnpSuccesEnR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1 = enable the corresponding bit in the IntSt register."]
    #[inline(always)]
    #[must_use]
    pub fn tmr_en(&mut self) -> TmrEnW<IntenSpec> {
        TmrEnW::new(self, 0)
    }
    #[doc = "Bit 1 - 1 = enable the corresponding bit in the IntSt register."]
    #[inline(always)]
    #[must_use]
    pub fn remove_pu_en(&mut self) -> RemovePuEnW<IntenSpec> {
        RemovePuEnW::new(self, 1)
    }
    #[doc = "Bit 2 - 1 = enable the corresponding bit in the IntSt register."]
    #[inline(always)]
    #[must_use]
    pub fn hnp_failure_en(&mut self) -> HnpFailureEnW<IntenSpec> {
        HnpFailureEnW::new(self, 2)
    }
    #[doc = "Bit 3 - 1 = enable the corresponding bit in the IntSt register."]
    #[inline(always)]
    #[must_use]
    pub fn hnp_succes_en(&mut self) -> HnpSuccesEnW<IntenSpec> {
        HnpSuccesEnW::new(self, 3)
    }
}
#[doc = "OTG Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenSpec;
impl crate::RegisterSpec for IntenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for IntenSpec {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for IntenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for IntenSpec {
    const RESET_VALUE: u32 = 0;
}
