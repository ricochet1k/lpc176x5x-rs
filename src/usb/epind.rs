#[doc = "Register `EPIND` writer"]
pub type W = crate::W<EpindSpec>;
#[doc = "Field `PHY_EP` writer - Physical endpoint number (0-31)"]
pub type PhyEpW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl W {
    #[doc = "Bits 0:4 - Physical endpoint number (0-31)"]
    #[inline(always)]
    #[must_use]
    pub fn phy_ep(&mut self) -> PhyEpW<EpindSpec> {
        PhyEpW::new(self, 0)
    }
}
#[doc = "USB Endpoint Index\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epind::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EpindSpec;
impl crate::RegisterSpec for EpindSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`epind::W`](W) writer structure"]
impl crate::Writable for EpindSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EPIND to value 0"]
impl crate::Resettable for EpindSpec {
    const RESET_VALUE: u32 = 0;
}
