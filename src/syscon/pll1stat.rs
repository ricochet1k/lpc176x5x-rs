#[doc = "Register `PLL1STAT` reader"]
pub type R = crate::R<Pll1statSpec>;
#[doc = "Field `MSEL1` reader - Read-back for the PLL1 Multiplier value. This is the value currently used by PLL1."]
pub type Msel1R = crate::FieldReader;
#[doc = "Field `PSEL1` reader - Read-back for the PLL1 Divider value. This is the value currently used by PLL1."]
pub type Psel1R = crate::FieldReader;
#[doc = "Field `PLLE1_STAT` reader - Read-back for the PLL1 Enable bit. When one, PLL1 is currently activated. When zero, PLL1 is turned off. This bit is automatically cleared when Power-down mode is activated."]
pub type Plle1StatR = crate::BitReader;
#[doc = "Field `PLLC1_STAT` reader - Read-back for the PLL1 Connect bit. When PLLC and PLLE are both one, PLL1 is connected as the clock source for the microcontroller. When either PLLC or PLLE is zero, PLL1 is bypassed and the oscillator clock is used directly by the microcontroller. This bit is automatically cleared when Power-down mode is activated."]
pub type Pllc1StatR = crate::BitReader;
#[doc = "Field `PLOCK1` reader - Reflects the PLL1 Lock status. When zero, PLL1 is not locked. When one, PLL1 is locked onto the requested frequency."]
pub type Plock1R = crate::BitReader;
impl R {
    #[doc = "Bits 0:4 - Read-back for the PLL1 Multiplier value. This is the value currently used by PLL1."]
    #[inline(always)]
    pub fn msel1(&self) -> Msel1R {
        Msel1R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:6 - Read-back for the PLL1 Divider value. This is the value currently used by PLL1."]
    #[inline(always)]
    pub fn psel1(&self) -> Psel1R {
        Psel1R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 8 - Read-back for the PLL1 Enable bit. When one, PLL1 is currently activated. When zero, PLL1 is turned off. This bit is automatically cleared when Power-down mode is activated."]
    #[inline(always)]
    pub fn plle1_stat(&self) -> Plle1StatR {
        Plle1StatR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Read-back for the PLL1 Connect bit. When PLLC and PLLE are both one, PLL1 is connected as the clock source for the microcontroller. When either PLLC or PLLE is zero, PLL1 is bypassed and the oscillator clock is used directly by the microcontroller. This bit is automatically cleared when Power-down mode is activated."]
    #[inline(always)]
    pub fn pllc1_stat(&self) -> Pllc1StatR {
        Pllc1StatR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Reflects the PLL1 Lock status. When zero, PLL1 is not locked. When one, PLL1 is locked onto the requested frequency."]
    #[inline(always)]
    pub fn plock1(&self) -> Plock1R {
        Plock1R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "PLL1 Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll1stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pll1statSpec;
impl crate::RegisterSpec for Pll1statSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll1stat::R`](R) reader structure"]
impl crate::Readable for Pll1statSpec {}
#[doc = "`reset()` method sets PLL1STAT to value 0"]
impl crate::Resettable for Pll1statSpec {
    const RESET_VALUE: u32 = 0;
}
