#[doc = "Register `PLL1CON` reader"]
pub type R = crate::R<Pll1conSpec>;
#[doc = "Register `PLL1CON` writer"]
pub type W = crate::W<Pll1conSpec>;
#[doc = "Field `PLLE1` reader - PLL1 Enable. When one, and after a valid PLL1 feed, this bit will activate PLL1 and allow it to lock to the requested frequency."]
pub type Plle1R = crate::BitReader;
#[doc = "Field `PLLE1` writer - PLL1 Enable. When one, and after a valid PLL1 feed, this bit will activate PLL1 and allow it to lock to the requested frequency."]
pub type Plle1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLC1` reader - PLL1 Connect. Setting PLLC to one after PLL1 has been enabled and locked, then followed by a valid PLL1 feed sequence causes PLL1 to become the clock source for the USB subsystem via the USB clock divider. See PLL1STAT register."]
pub type Pllc1R = crate::BitReader;
#[doc = "Field `PLLC1` writer - PLL1 Connect. Setting PLLC to one after PLL1 has been enabled and locked, then followed by a valid PLL1 feed sequence causes PLL1 to become the clock source for the USB subsystem via the USB clock divider. See PLL1STAT register."]
pub type Pllc1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PLL1 Enable. When one, and after a valid PLL1 feed, this bit will activate PLL1 and allow it to lock to the requested frequency."]
    #[inline(always)]
    pub fn plle1(&self) -> Plle1R {
        Plle1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PLL1 Connect. Setting PLLC to one after PLL1 has been enabled and locked, then followed by a valid PLL1 feed sequence causes PLL1 to become the clock source for the USB subsystem via the USB clock divider. See PLL1STAT register."]
    #[inline(always)]
    pub fn pllc1(&self) -> Pllc1R {
        Pllc1R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PLL1 Enable. When one, and after a valid PLL1 feed, this bit will activate PLL1 and allow it to lock to the requested frequency."]
    #[inline(always)]
    #[must_use]
    pub fn plle1(&mut self) -> Plle1W<Pll1conSpec> {
        Plle1W::new(self, 0)
    }
    #[doc = "Bit 1 - PLL1 Connect. Setting PLLC to one after PLL1 has been enabled and locked, then followed by a valid PLL1 feed sequence causes PLL1 to become the clock source for the USB subsystem via the USB clock divider. See PLL1STAT register."]
    #[inline(always)]
    #[must_use]
    pub fn pllc1(&mut self) -> Pllc1W<Pll1conSpec> {
        Pllc1W::new(self, 1)
    }
}
#[doc = "PLL1 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll1con::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1con::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pll1conSpec;
impl crate::RegisterSpec for Pll1conSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll1con::R`](R) reader structure"]
impl crate::Readable for Pll1conSpec {}
#[doc = "`write(|w| ..)` method takes [`pll1con::W`](W) writer structure"]
impl crate::Writable for Pll1conSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLL1CON to value 0"]
impl crate::Resettable for Pll1conSpec {
    const RESET_VALUE: u32 = 0;
}
