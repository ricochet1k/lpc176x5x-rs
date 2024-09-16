#[doc = "Register `CAP_CLR` writer"]
pub type W = crate::W<CapClrSpec>;
#[doc = "Field `CAP_CLR0` writer - Writing a 1 to this bit clears the CAP0 register."]
pub type CapClr0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP_CLR1` writer - Writing a 1 to this bit clears the CAP1 register."]
pub type CapClr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP_CLR2` writer - Writing a 1 to this bit clears the CAP2 register."]
pub type CapClr2W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Writing a 1 to this bit clears the CAP0 register."]
    #[inline(always)]
    #[must_use]
    pub fn cap_clr0(&mut self) -> CapClr0W<CapClrSpec> {
        CapClr0W::new(self, 0)
    }
    #[doc = "Bit 1 - Writing a 1 to this bit clears the CAP1 register."]
    #[inline(always)]
    #[must_use]
    pub fn cap_clr1(&mut self) -> CapClr1W<CapClrSpec> {
        CapClr1W::new(self, 1)
    }
    #[doc = "Bit 2 - Writing a 1 to this bit clears the CAP2 register."]
    #[inline(always)]
    #[must_use]
    pub fn cap_clr2(&mut self) -> CapClr2W<CapClrSpec> {
        CapClr2W::new(self, 2)
    }
}
#[doc = "Capture clear address\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CapClrSpec;
impl crate::RegisterSpec for CapClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cap_clr::W`](W) writer structure"]
impl crate::Writable for CapClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CAP_CLR to value 0"]
impl crate::Resettable for CapClrSpec {
    const RESET_VALUE: u32 = 0;
}
