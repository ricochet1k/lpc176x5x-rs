#[doc = "Register `SCS` reader"]
pub type R = crate::R<ScsSpec>;
#[doc = "Register `SCS` writer"]
pub type W = crate::W<ScsSpec>;
#[doc = "Main oscillator range select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oscrange {
    #[doc = "0: Low. The frequency range of the main oscillator is 1 MHz to 20 MHz."]
    Low = 0,
    #[doc = "1: High. The frequency range of the main oscillator is 15 MHz to 25 MHz."]
    High = 1,
}
impl From<Oscrange> for bool {
    #[inline(always)]
    fn from(variant: Oscrange) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSCRANGE` reader - Main oscillator range select."]
pub type OscrangeR = crate::BitReader<Oscrange>;
impl OscrangeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oscrange {
        match self.bits {
            false => Oscrange::Low,
            true => Oscrange::High,
        }
    }
    #[doc = "Low. The frequency range of the main oscillator is 1 MHz to 20 MHz."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Oscrange::Low
    }
    #[doc = "High. The frequency range of the main oscillator is 15 MHz to 25 MHz."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Oscrange::High
    }
}
#[doc = "Field `OSCRANGE` writer - Main oscillator range select."]
pub type OscrangeW<'a, REG> = crate::BitWriter<'a, REG, Oscrange>;
impl<'a, REG> OscrangeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low. The frequency range of the main oscillator is 1 MHz to 20 MHz."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Oscrange::Low)
    }
    #[doc = "High. The frequency range of the main oscillator is 15 MHz to 25 MHz."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Oscrange::High)
    }
}
#[doc = "Main oscillator enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oscen {
    #[doc = "0: Disabled. The main oscillator is disabled."]
    Disabled = 0,
    #[doc = "1: Enabled.The main oscillator is enabled, and will start up if the correct external circuitry is connected to the XTAL1 and XTAL2 pins."]
    Enabled = 1,
}
impl From<Oscen> for bool {
    #[inline(always)]
    fn from(variant: Oscen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSCEN` reader - Main oscillator enable."]
pub type OscenR = crate::BitReader<Oscen>;
impl OscenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oscen {
        match self.bits {
            false => Oscen::Disabled,
            true => Oscen::Enabled,
        }
    }
    #[doc = "Disabled. The main oscillator is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Oscen::Disabled
    }
    #[doc = "Enabled.The main oscillator is enabled, and will start up if the correct external circuitry is connected to the XTAL1 and XTAL2 pins."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Oscen::Enabled
    }
}
#[doc = "Field `OSCEN` writer - Main oscillator enable."]
pub type OscenW<'a, REG> = crate::BitWriter<'a, REG, Oscen>;
impl<'a, REG> OscenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. The main oscillator is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Oscen::Disabled)
    }
    #[doc = "Enabled.The main oscillator is enabled, and will start up if the correct external circuitry is connected to the XTAL1 and XTAL2 pins."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Oscen::Enabled)
    }
}
#[doc = "Main oscillator status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oscstat {
    #[doc = "0: Not ready. The main oscillator is not ready to be used as a clock source."]
    NotReady = 0,
    #[doc = "1: Ready. The main oscillator is ready to be used as a clock source. The main oscillator must be enabled via the OSCEN bit."]
    Ready = 1,
}
impl From<Oscstat> for bool {
    #[inline(always)]
    fn from(variant: Oscstat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSCSTAT` reader - Main oscillator status."]
pub type OscstatR = crate::BitReader<Oscstat>;
impl OscstatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oscstat {
        match self.bits {
            false => Oscstat::NotReady,
            true => Oscstat::Ready,
        }
    }
    #[doc = "Not ready. The main oscillator is not ready to be used as a clock source."]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == Oscstat::NotReady
    }
    #[doc = "Ready. The main oscillator is ready to be used as a clock source. The main oscillator must be enabled via the OSCEN bit."]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == Oscstat::Ready
    }
}
#[doc = "Field `OSCSTAT` writer - Main oscillator status."]
pub type OscstatW<'a, REG> = crate::BitWriter<'a, REG, Oscstat>;
impl<'a, REG> OscstatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not ready. The main oscillator is not ready to be used as a clock source."]
    #[inline(always)]
    pub fn not_ready(self) -> &'a mut crate::W<REG> {
        self.variant(Oscstat::NotReady)
    }
    #[doc = "Ready. The main oscillator is ready to be used as a clock source. The main oscillator must be enabled via the OSCEN bit."]
    #[inline(always)]
    pub fn ready(self) -> &'a mut crate::W<REG> {
        self.variant(Oscstat::Ready)
    }
}
impl R {
    #[doc = "Bit 4 - Main oscillator range select."]
    #[inline(always)]
    pub fn oscrange(&self) -> OscrangeR {
        OscrangeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Main oscillator enable."]
    #[inline(always)]
    pub fn oscen(&self) -> OscenR {
        OscenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Main oscillator status."]
    #[inline(always)]
    pub fn oscstat(&self) -> OscstatR {
        OscstatR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Main oscillator range select."]
    #[inline(always)]
    #[must_use]
    pub fn oscrange(&mut self) -> OscrangeW<ScsSpec> {
        OscrangeW::new(self, 4)
    }
    #[doc = "Bit 5 - Main oscillator enable."]
    #[inline(always)]
    #[must_use]
    pub fn oscen(&mut self) -> OscenW<ScsSpec> {
        OscenW::new(self, 5)
    }
    #[doc = "Bit 6 - Main oscillator status."]
    #[inline(always)]
    #[must_use]
    pub fn oscstat(&mut self) -> OscstatW<ScsSpec> {
        OscstatW::new(self, 6)
    }
}
#[doc = "System control and status\n\nYou can [`read`](crate::Reg::read) this register and get [`scs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScsSpec;
impl crate::RegisterSpec for ScsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scs::R`](R) reader structure"]
impl crate::Readable for ScsSpec {}
#[doc = "`write(|w| ..)` method takes [`scs::W`](W) writer structure"]
impl crate::Writable for ScsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCS to value 0"]
impl crate::Resettable for ScsSpec {
    const RESET_VALUE: u32 = 0;
}
