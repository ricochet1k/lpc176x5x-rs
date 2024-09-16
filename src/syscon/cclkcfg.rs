#[doc = "Register `CCLKCFG` reader"]
pub type R = crate::R<CclkcfgSpec>;
#[doc = "Register `CCLKCFG` writer"]
pub type W = crate::W<CclkcfgSpec>;
#[doc = "Field `CCLKSEL` reader - Selects the divide value for creating the CPU clock (CCLK) from the PLL0 output. 0 = pllclk is divided by 1 to produce the CPU clock. This setting is not allowed when the PLL0 is connected, because the rate would always be greater than the maximum allowed CPU clock. 1 = pllclk is divided by 2 to produce the CPU clock. This setting is not allowed when the PLL0 is connected, because the rate would always be greater than the maximum allowed CPU clock. 2 = pllclk is divided by 3 to produce the CPU clock. 3 = pllclk is divided by 4 to produce the CPU clock. ... 255 = pllclk is divided by 256 to produce the CPU clock."]
pub type CclkselR = crate::FieldReader;
#[doc = "Field `CCLKSEL` writer - Selects the divide value for creating the CPU clock (CCLK) from the PLL0 output. 0 = pllclk is divided by 1 to produce the CPU clock. This setting is not allowed when the PLL0 is connected, because the rate would always be greater than the maximum allowed CPU clock. 1 = pllclk is divided by 2 to produce the CPU clock. This setting is not allowed when the PLL0 is connected, because the rate would always be greater than the maximum allowed CPU clock. 2 = pllclk is divided by 3 to produce the CPU clock. 3 = pllclk is divided by 4 to produce the CPU clock. ... 255 = pllclk is divided by 256 to produce the CPU clock."]
pub type CclkselW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Selects the divide value for creating the CPU clock (CCLK) from the PLL0 output. 0 = pllclk is divided by 1 to produce the CPU clock. This setting is not allowed when the PLL0 is connected, because the rate would always be greater than the maximum allowed CPU clock. 1 = pllclk is divided by 2 to produce the CPU clock. This setting is not allowed when the PLL0 is connected, because the rate would always be greater than the maximum allowed CPU clock. 2 = pllclk is divided by 3 to produce the CPU clock. 3 = pllclk is divided by 4 to produce the CPU clock. ... 255 = pllclk is divided by 256 to produce the CPU clock."]
    #[inline(always)]
    pub fn cclksel(&self) -> CclkselR {
        CclkselR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Selects the divide value for creating the CPU clock (CCLK) from the PLL0 output. 0 = pllclk is divided by 1 to produce the CPU clock. This setting is not allowed when the PLL0 is connected, because the rate would always be greater than the maximum allowed CPU clock. 1 = pllclk is divided by 2 to produce the CPU clock. This setting is not allowed when the PLL0 is connected, because the rate would always be greater than the maximum allowed CPU clock. 2 = pllclk is divided by 3 to produce the CPU clock. 3 = pllclk is divided by 4 to produce the CPU clock. ... 255 = pllclk is divided by 256 to produce the CPU clock."]
    #[inline(always)]
    #[must_use]
    pub fn cclksel(&mut self) -> CclkselW<CclkcfgSpec> {
        CclkselW::new(self, 0)
    }
}
#[doc = "CPU Clock Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cclkcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cclkcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CclkcfgSpec;
impl crate::RegisterSpec for CclkcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cclkcfg::R`](R) reader structure"]
impl crate::Readable for CclkcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cclkcfg::W`](W) writer structure"]
impl crate::Writable for CclkcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCLKCFG to value 0"]
impl crate::Resettable for CclkcfgSpec {
    const RESET_VALUE: u32 = 0;
}
