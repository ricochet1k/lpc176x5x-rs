#[doc = "Register `INTCLR` writer"]
pub type W = crate::W<IntclrSpec>;
#[doc = "Field `TMR_CLR` writer - 0 = no effect. 1 = clear the corresponding bit in the IntSt register."]
pub type TmrClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REMOVE_PU_CLR` writer - 0 = no effect. 1 = clear the corresponding bit in the IntSt register."]
pub type RemovePuClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HNP_FAILURE_CLR` writer - 0 = no effect. 1 = clear the corresponding bit in the IntSt register."]
pub type HnpFailureClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HNP_SUCCES_CLR` writer - 0 = no effect. 1 = clear the corresponding bit in the IntSt register."]
pub type HnpSuccesClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - 0 = no effect. 1 = clear the corresponding bit in the IntSt register."]
    #[inline(always)]
    #[must_use]
    pub fn tmr_clr(&mut self) -> TmrClrW<IntclrSpec> {
        TmrClrW::new(self, 0)
    }
    #[doc = "Bit 1 - 0 = no effect. 1 = clear the corresponding bit in the IntSt register."]
    #[inline(always)]
    #[must_use]
    pub fn remove_pu_clr(&mut self) -> RemovePuClrW<IntclrSpec> {
        RemovePuClrW::new(self, 1)
    }
    #[doc = "Bit 2 - 0 = no effect. 1 = clear the corresponding bit in the IntSt register."]
    #[inline(always)]
    #[must_use]
    pub fn hnp_failure_clr(&mut self) -> HnpFailureClrW<IntclrSpec> {
        HnpFailureClrW::new(self, 2)
    }
    #[doc = "Bit 3 - 0 = no effect. 1 = clear the corresponding bit in the IntSt register."]
    #[inline(always)]
    #[must_use]
    pub fn hnp_succes_clr(&mut self) -> HnpSuccesClrW<IntclrSpec> {
        HnpSuccesClrW::new(self, 3)
    }
}
#[doc = "OTG Interrupt Clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntclrSpec;
impl crate::RegisterSpec for IntclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`intclr::W`](W) writer structure"]
impl crate::Writable for IntclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTCLR to value 0"]
impl crate::Resettable for IntclrSpec {
    const RESET_VALUE: u32 = 0;
}
