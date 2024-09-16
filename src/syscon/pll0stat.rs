#[doc = "Register `PLL0STAT` reader"]
pub type R = crate::R<Pll0statSpec>;
#[doc = "Field `MSEL0` reader - Read-back for the PLL0 Multiplier value. This is the value currently used by PLL0, and is one less than the actual multiplier."]
pub type Msel0R = crate::FieldReader<u16>;
#[doc = "Field `NSEL0` reader - Read-back for the PLL0 Pre-Divider value. This is the value currently used by PLL0, and is one less than the actual divider."]
pub type Nsel0R = crate::FieldReader;
#[doc = "Field `PLLE0_STAT` reader - Read-back for the PLL0 Enable bit. This bit reflects the state of the PLEC0 bit in PLL0CON after a valid PLL0 feed. When one, PLL0 is currently enabled. When zero, PLL0 is turned off. This bit is automatically cleared when Power-down mode is entered."]
pub type Plle0StatR = crate::BitReader;
#[doc = "Field `PLLC0_STAT` reader - Read-back for the PLL0 Connect bit. This bit reflects the state of the PLLC0 bit in PLL0CON after a valid PLL0 feed. When PLLC0 and PLLE0 are both one, PLL0 is connected as the clock source for the CPU. When either PLLC0 or PLLE0 is zero, PLL0 is bypassed. This bit is automatically cleared when Power-down mode is entered."]
pub type Pllc0StatR = crate::BitReader;
#[doc = "Field `PLOCK0` reader - Reflects the PLL0 Lock status. When zero, PLL0 is not locked. When one, PLL0 is locked onto the requested frequency. See text for details."]
pub type Plock0R = crate::BitReader;
impl R {
    #[doc = "Bits 0:14 - Read-back for the PLL0 Multiplier value. This is the value currently used by PLL0, and is one less than the actual multiplier."]
    #[inline(always)]
    pub fn msel0(&self) -> Msel0R {
        Msel0R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:23 - Read-back for the PLL0 Pre-Divider value. This is the value currently used by PLL0, and is one less than the actual divider."]
    #[inline(always)]
    pub fn nsel0(&self) -> Nsel0R {
        Nsel0R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Read-back for the PLL0 Enable bit. This bit reflects the state of the PLEC0 bit in PLL0CON after a valid PLL0 feed. When one, PLL0 is currently enabled. When zero, PLL0 is turned off. This bit is automatically cleared when Power-down mode is entered."]
    #[inline(always)]
    pub fn plle0_stat(&self) -> Plle0StatR {
        Plle0StatR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Read-back for the PLL0 Connect bit. This bit reflects the state of the PLLC0 bit in PLL0CON after a valid PLL0 feed. When PLLC0 and PLLE0 are both one, PLL0 is connected as the clock source for the CPU. When either PLLC0 or PLLE0 is zero, PLL0 is bypassed. This bit is automatically cleared when Power-down mode is entered."]
    #[inline(always)]
    pub fn pllc0_stat(&self) -> Pllc0StatR {
        Pllc0StatR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Reflects the PLL0 Lock status. When zero, PLL0 is not locked. When one, PLL0 is locked onto the requested frequency. See text for details."]
    #[inline(always)]
    pub fn plock0(&self) -> Plock0R {
        Plock0R::new(((self.bits >> 26) & 1) != 0)
    }
}
#[doc = "PLL0 Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll0stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pll0statSpec;
impl crate::RegisterSpec for Pll0statSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll0stat::R`](R) reader structure"]
impl crate::Readable for Pll0statSpec {}
#[doc = "`reset()` method sets PLL0STAT to value 0"]
impl crate::Resettable for Pll0statSpec {
    const RESET_VALUE: u32 = 0;
}
