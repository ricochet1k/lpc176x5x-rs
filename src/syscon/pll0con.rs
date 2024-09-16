#[doc = "Register `PLL0CON` reader"]
pub type R = crate::R<Pll0conSpec>;
#[doc = "Register `PLL0CON` writer"]
pub type W = crate::W<Pll0conSpec>;
#[doc = "Field `PLLE0` reader - PLL0 Enable. When one, and after a valid PLL0 feed, this bit will activate PLL0 and allow it to lock to the requested frequency. See PLL0STAT register."]
pub type Plle0R = crate::BitReader;
#[doc = "Field `PLLE0` writer - PLL0 Enable. When one, and after a valid PLL0 feed, this bit will activate PLL0 and allow it to lock to the requested frequency. See PLL0STAT register."]
pub type Plle0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLC0` reader - PLL0 Connect. Setting PLLC0 to one after PLL0 has been enabled and locked, then followed by a valid PLL0 feed sequence causes PLL0 to become the clock source for the CPU, AHB peripherals, and used to derive the clocks for APB peripherals. The PLL0 output may potentially be used to clock the USB subsystem if the frequency is 48 MHz. See PLL0STAT register."]
pub type Pllc0R = crate::BitReader;
#[doc = "Field `PLLC0` writer - PLL0 Connect. Setting PLLC0 to one after PLL0 has been enabled and locked, then followed by a valid PLL0 feed sequence causes PLL0 to become the clock source for the CPU, AHB peripherals, and used to derive the clocks for APB peripherals. The PLL0 output may potentially be used to clock the USB subsystem if the frequency is 48 MHz. See PLL0STAT register."]
pub type Pllc0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PLL0 Enable. When one, and after a valid PLL0 feed, this bit will activate PLL0 and allow it to lock to the requested frequency. See PLL0STAT register."]
    #[inline(always)]
    pub fn plle0(&self) -> Plle0R {
        Plle0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PLL0 Connect. Setting PLLC0 to one after PLL0 has been enabled and locked, then followed by a valid PLL0 feed sequence causes PLL0 to become the clock source for the CPU, AHB peripherals, and used to derive the clocks for APB peripherals. The PLL0 output may potentially be used to clock the USB subsystem if the frequency is 48 MHz. See PLL0STAT register."]
    #[inline(always)]
    pub fn pllc0(&self) -> Pllc0R {
        Pllc0R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PLL0 Enable. When one, and after a valid PLL0 feed, this bit will activate PLL0 and allow it to lock to the requested frequency. See PLL0STAT register."]
    #[inline(always)]
    #[must_use]
    pub fn plle0(&mut self) -> Plle0W<Pll0conSpec> {
        Plle0W::new(self, 0)
    }
    #[doc = "Bit 1 - PLL0 Connect. Setting PLLC0 to one after PLL0 has been enabled and locked, then followed by a valid PLL0 feed sequence causes PLL0 to become the clock source for the CPU, AHB peripherals, and used to derive the clocks for APB peripherals. The PLL0 output may potentially be used to clock the USB subsystem if the frequency is 48 MHz. See PLL0STAT register."]
    #[inline(always)]
    #[must_use]
    pub fn pllc0(&mut self) -> Pllc0W<Pll0conSpec> {
        Pllc0W::new(self, 1)
    }
}
#[doc = "PLL0 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll0con::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll0con::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pll0conSpec;
impl crate::RegisterSpec for Pll0conSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll0con::R`](R) reader structure"]
impl crate::Readable for Pll0conSpec {}
#[doc = "`write(|w| ..)` method takes [`pll0con::W`](W) writer structure"]
impl crate::Writable for Pll0conSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLL0CON to value 0"]
impl crate::Resettable for Pll0conSpec {
    const RESET_VALUE: u32 = 0;
}
