#[doc = "Register `PLL0CFG` reader"]
pub type R = crate::R<Pll0cfgSpec>;
#[doc = "Register `PLL0CFG` writer"]
pub type W = crate::W<Pll0cfgSpec>;
#[doc = "Field `MSEL0` reader - PLL0 Multiplier value. Supplies the value M in PLL0 frequency calculations. The value stored here is M - 1. Note: Not all values of M are needed, and therefore some are not supported by hardware."]
pub type Msel0R = crate::FieldReader<u16>;
#[doc = "Field `MSEL0` writer - PLL0 Multiplier value. Supplies the value M in PLL0 frequency calculations. The value stored here is M - 1. Note: Not all values of M are needed, and therefore some are not supported by hardware."]
pub type Msel0W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `NSEL0` reader - PLL0 Pre-Divider value. Supplies the value N in PLL0 frequency calculations. The value stored here is N - 1. Supported values for N are 1 through 32."]
pub type Nsel0R = crate::FieldReader;
#[doc = "Field `NSEL0` writer - PLL0 Pre-Divider value. Supplies the value N in PLL0 frequency calculations. The value stored here is N - 1. Supported values for N are 1 through 32."]
pub type Nsel0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:14 - PLL0 Multiplier value. Supplies the value M in PLL0 frequency calculations. The value stored here is M - 1. Note: Not all values of M are needed, and therefore some are not supported by hardware."]
    #[inline(always)]
    pub fn msel0(&self) -> Msel0R {
        Msel0R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:23 - PLL0 Pre-Divider value. Supplies the value N in PLL0 frequency calculations. The value stored here is N - 1. Supported values for N are 1 through 32."]
    #[inline(always)]
    pub fn nsel0(&self) -> Nsel0R {
        Nsel0R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:14 - PLL0 Multiplier value. Supplies the value M in PLL0 frequency calculations. The value stored here is M - 1. Note: Not all values of M are needed, and therefore some are not supported by hardware."]
    #[inline(always)]
    #[must_use]
    pub fn msel0(&mut self) -> Msel0W<Pll0cfgSpec> {
        Msel0W::new(self, 0)
    }
    #[doc = "Bits 16:23 - PLL0 Pre-Divider value. Supplies the value N in PLL0 frequency calculations. The value stored here is N - 1. Supported values for N are 1 through 32."]
    #[inline(always)]
    #[must_use]
    pub fn nsel0(&mut self) -> Nsel0W<Pll0cfgSpec> {
        Nsel0W::new(self, 16)
    }
}
#[doc = "PLL0 Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll0cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll0cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pll0cfgSpec;
impl crate::RegisterSpec for Pll0cfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll0cfg::R`](R) reader structure"]
impl crate::Readable for Pll0cfgSpec {}
#[doc = "`write(|w| ..)` method takes [`pll0cfg::W`](W) writer structure"]
impl crate::Writable for Pll0cfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLL0CFG to value 0"]
impl crate::Resettable for Pll0cfgSpec {
    const RESET_VALUE: u32 = 0;
}
