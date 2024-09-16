#[doc = "Register `MCFG` reader"]
pub type R = crate::R<McfgSpec>;
#[doc = "Register `MCFG` writer"]
pub type W = crate::W<McfgSpec>;
#[doc = "Field `SCANINC` reader - SCAN INCREMENT. Set this bit to cause the MII Management hardware to perform read cycles across a range of PHYs. When set, the MII Management hardware will perform read cycles from address 1 through the value set in PHY ADDRESS\\[4:0\\]. Clear this bit to allow continuous reads of the same PHY."]
pub type ScanincR = crate::BitReader;
#[doc = "Field `SCANINC` writer - SCAN INCREMENT. Set this bit to cause the MII Management hardware to perform read cycles across a range of PHYs. When set, the MII Management hardware will perform read cycles from address 1 through the value set in PHY ADDRESS\\[4:0\\]. Clear this bit to allow continuous reads of the same PHY."]
pub type ScanincW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUPPPREAMBLE` reader - SUPPRESS PREAMBLE. Set this bit to cause the MII Management hardware to perform read/write cycles without the 32-bit preamble field. Clear this bit to cause normal cycles to be performed. Some PHYs support suppressed preamble."]
pub type SupppreambleR = crate::BitReader;
#[doc = "Field `SUPPPREAMBLE` writer - SUPPRESS PREAMBLE. Set this bit to cause the MII Management hardware to perform read/write cycles without the 32-bit preamble field. Clear this bit to cause normal cycles to be performed. Some PHYs support suppressed preamble."]
pub type SupppreambleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLOCKSEL` reader - CLOCK SELECT. This field is used by the clock divide logic in creating the MII Management Clock (MDC) which IEEE 802.3u defines to be no faster than 2.5 MHz. Some PHYs support clock rates up to 12.5 MHz, however. The AHB bus clock (HCLK) is divided by the specified amount. Refer to Table 160 below for the definition of values for this field."]
pub type ClockselR = crate::FieldReader;
#[doc = "Field `CLOCKSEL` writer - CLOCK SELECT. This field is used by the clock divide logic in creating the MII Management Clock (MDC) which IEEE 802.3u defines to be no faster than 2.5 MHz. Some PHYs support clock rates up to 12.5 MHz, however. The AHB bus clock (HCLK) is divided by the specified amount. Refer to Table 160 below for the definition of values for this field."]
pub type ClockselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RESETMIIMGMT` reader - RESET MII MGMT. This bit resets the MII Management hardware."]
pub type ResetmiimgmtR = crate::BitReader;
#[doc = "Field `RESETMIIMGMT` writer - RESET MII MGMT. This bit resets the MII Management hardware."]
pub type ResetmiimgmtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCAN INCREMENT. Set this bit to cause the MII Management hardware to perform read cycles across a range of PHYs. When set, the MII Management hardware will perform read cycles from address 1 through the value set in PHY ADDRESS\\[4:0\\]. Clear this bit to allow continuous reads of the same PHY."]
    #[inline(always)]
    pub fn scaninc(&self) -> ScanincR {
        ScanincR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SUPPRESS PREAMBLE. Set this bit to cause the MII Management hardware to perform read/write cycles without the 32-bit preamble field. Clear this bit to cause normal cycles to be performed. Some PHYs support suppressed preamble."]
    #[inline(always)]
    pub fn supppreamble(&self) -> SupppreambleR {
        SupppreambleR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - CLOCK SELECT. This field is used by the clock divide logic in creating the MII Management Clock (MDC) which IEEE 802.3u defines to be no faster than 2.5 MHz. Some PHYs support clock rates up to 12.5 MHz, however. The AHB bus clock (HCLK) is divided by the specified amount. Refer to Table 160 below for the definition of values for this field."]
    #[inline(always)]
    pub fn clocksel(&self) -> ClockselR {
        ClockselR::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - RESET MII MGMT. This bit resets the MII Management hardware."]
    #[inline(always)]
    pub fn resetmiimgmt(&self) -> ResetmiimgmtR {
        ResetmiimgmtR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCAN INCREMENT. Set this bit to cause the MII Management hardware to perform read cycles across a range of PHYs. When set, the MII Management hardware will perform read cycles from address 1 through the value set in PHY ADDRESS\\[4:0\\]. Clear this bit to allow continuous reads of the same PHY."]
    #[inline(always)]
    #[must_use]
    pub fn scaninc(&mut self) -> ScanincW<McfgSpec> {
        ScanincW::new(self, 0)
    }
    #[doc = "Bit 1 - SUPPRESS PREAMBLE. Set this bit to cause the MII Management hardware to perform read/write cycles without the 32-bit preamble field. Clear this bit to cause normal cycles to be performed. Some PHYs support suppressed preamble."]
    #[inline(always)]
    #[must_use]
    pub fn supppreamble(&mut self) -> SupppreambleW<McfgSpec> {
        SupppreambleW::new(self, 1)
    }
    #[doc = "Bits 2:5 - CLOCK SELECT. This field is used by the clock divide logic in creating the MII Management Clock (MDC) which IEEE 802.3u defines to be no faster than 2.5 MHz. Some PHYs support clock rates up to 12.5 MHz, however. The AHB bus clock (HCLK) is divided by the specified amount. Refer to Table 160 below for the definition of values for this field."]
    #[inline(always)]
    #[must_use]
    pub fn clocksel(&mut self) -> ClockselW<McfgSpec> {
        ClockselW::new(self, 2)
    }
    #[doc = "Bit 15 - RESET MII MGMT. This bit resets the MII Management hardware."]
    #[inline(always)]
    #[must_use]
    pub fn resetmiimgmt(&mut self) -> ResetmiimgmtW<McfgSpec> {
        ResetmiimgmtW::new(self, 15)
    }
}
#[doc = "MII Mgmt Configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`mcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McfgSpec;
impl crate::RegisterSpec for McfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcfg::R`](R) reader structure"]
impl crate::Readable for McfgSpec {}
#[doc = "`write(|w| ..)` method takes [`mcfg::W`](W) writer structure"]
impl crate::Writable for McfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCFG to value 0"]
impl crate::Resettable for McfgSpec {
    const RESET_VALUE: u32 = 0;
}
