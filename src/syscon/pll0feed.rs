#[doc = "Register `PLL0FEED` writer"]
pub type W = crate::W<Pll0feedSpec>;
#[doc = "Field `PLL0FEED` writer - The PLL0 feed sequence must be written to this register in order for PLL0 configuration and control register changes to take effect."]
pub type Pll0feedW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - The PLL0 feed sequence must be written to this register in order for PLL0 configuration and control register changes to take effect."]
    #[inline(always)]
    #[must_use]
    pub fn pll0feed(&mut self) -> Pll0feedW<Pll0feedSpec> {
        Pll0feedW::new(self, 0)
    }
}
#[doc = "PLL0 Feed Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll0feed::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pll0feedSpec;
impl crate::RegisterSpec for Pll0feedSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pll0feed::W`](W) writer structure"]
impl crate::Writable for Pll0feedSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLL0FEED to value 0"]
impl crate::Resettable for Pll0feedSpec {
    const RESET_VALUE: u32 = 0;
}
