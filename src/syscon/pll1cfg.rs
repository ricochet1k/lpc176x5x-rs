#[doc = "Register `PLL1CFG` reader"]
pub type R = crate::R<Pll1cfgSpec>;
#[doc = "Register `PLL1CFG` writer"]
pub type W = crate::W<Pll1cfgSpec>;
#[doc = "Field `MSEL1` reader - PLL1 Multiplier value. Supplies the value M in the PLL1 frequency calculations."]
pub type Msel1R = crate::FieldReader;
#[doc = "Field `MSEL1` writer - PLL1 Multiplier value. Supplies the value M in the PLL1 frequency calculations."]
pub type Msel1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PSEL1` reader - PLL1 Divider value. Supplies the value P in the PLL1 frequency calculations."]
pub type Psel1R = crate::FieldReader;
#[doc = "Field `PSEL1` writer - PLL1 Divider value. Supplies the value P in the PLL1 frequency calculations."]
pub type Psel1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:4 - PLL1 Multiplier value. Supplies the value M in the PLL1 frequency calculations."]
    #[inline(always)]
    pub fn msel1(&self) -> Msel1R {
        Msel1R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:6 - PLL1 Divider value. Supplies the value P in the PLL1 frequency calculations."]
    #[inline(always)]
    pub fn psel1(&self) -> Psel1R {
        Psel1R::new(((self.bits >> 5) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - PLL1 Multiplier value. Supplies the value M in the PLL1 frequency calculations."]
    #[inline(always)]
    #[must_use]
    pub fn msel1(&mut self) -> Msel1W<Pll1cfgSpec> {
        Msel1W::new(self, 0)
    }
    #[doc = "Bits 5:6 - PLL1 Divider value. Supplies the value P in the PLL1 frequency calculations."]
    #[inline(always)]
    #[must_use]
    pub fn psel1(&mut self) -> Psel1W<Pll1cfgSpec> {
        Psel1W::new(self, 5)
    }
}
#[doc = "PLL1 Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll1cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pll1cfgSpec;
impl crate::RegisterSpec for Pll1cfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll1cfg::R`](R) reader structure"]
impl crate::Readable for Pll1cfgSpec {}
#[doc = "`write(|w| ..)` method takes [`pll1cfg::W`](W) writer structure"]
impl crate::Writable for Pll1cfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLL1CFG to value 0"]
impl crate::Resettable for Pll1cfgSpec {
    const RESET_VALUE: u32 = 0;
}
