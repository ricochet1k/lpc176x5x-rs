#[doc = "Register `INTSET` writer"]
pub type W = crate::W<IntsetSpec>;
#[doc = "Field `TMR_SET` writer - 0 = no effect. 1 = set the corresponding bit in the IntSt register."]
pub type TmrSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REMOVE_PU_SET` writer - 0 = no effect. 1 = set the corresponding bit in the IntSt register."]
pub type RemovePuSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HNP_FAILURE_SET` writer - 0 = no effect. 1 = set the corresponding bit in the IntSt register."]
pub type HnpFailureSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HNP_SUCCES_SET` writer - 0 = no effect. 1 = set the corresponding bit in the IntSt register."]
pub type HnpSuccesSetW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - 0 = no effect. 1 = set the corresponding bit in the IntSt register."]
    #[inline(always)]
    #[must_use]
    pub fn tmr_set(&mut self) -> TmrSetW<IntsetSpec> {
        TmrSetW::new(self, 0)
    }
    #[doc = "Bit 1 - 0 = no effect. 1 = set the corresponding bit in the IntSt register."]
    #[inline(always)]
    #[must_use]
    pub fn remove_pu_set(&mut self) -> RemovePuSetW<IntsetSpec> {
        RemovePuSetW::new(self, 1)
    }
    #[doc = "Bit 2 - 0 = no effect. 1 = set the corresponding bit in the IntSt register."]
    #[inline(always)]
    #[must_use]
    pub fn hnp_failure_set(&mut self) -> HnpFailureSetW<IntsetSpec> {
        HnpFailureSetW::new(self, 2)
    }
    #[doc = "Bit 3 - 0 = no effect. 1 = set the corresponding bit in the IntSt register."]
    #[inline(always)]
    #[must_use]
    pub fn hnp_succes_set(&mut self) -> HnpSuccesSetW<IntsetSpec> {
        HnpSuccesSetW::new(self, 3)
    }
}
#[doc = "OTG Interrupt Set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntsetSpec;
impl crate::RegisterSpec for IntsetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`intset::W`](W) writer structure"]
impl crate::Writable for IntsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTSET to value 0"]
impl crate::Resettable for IntsetSpec {
    const RESET_VALUE: u32 = 0;
}
