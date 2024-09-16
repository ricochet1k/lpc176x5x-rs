#[doc = "Register `PLL1FEED` writer"]
pub type W = crate::W<Pll1feedSpec>;
#[doc = "Field `PLL1FEED` writer - The PLL1 feed sequence must be written to this register in order for PLL1 configuration and control register changes to take effect."]
pub type Pll1feedW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - The PLL1 feed sequence must be written to this register in order for PLL1 configuration and control register changes to take effect."]
    #[inline(always)]
    #[must_use]
    pub fn pll1feed(&mut self) -> Pll1feedW<Pll1feedSpec> {
        Pll1feedW::new(self, 0)
    }
}
#[doc = "PLL1 Feed Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1feed::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pll1feedSpec;
impl crate::RegisterSpec for Pll1feedSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pll1feed::W`](W) writer structure"]
impl crate::Writable for Pll1feedSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLL1FEED to value 0"]
impl crate::Resettable for Pll1feedSpec {
    const RESET_VALUE: u32 = 0;
}
