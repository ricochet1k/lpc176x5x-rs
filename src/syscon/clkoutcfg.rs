#[doc = "Register `CLKOUTCFG` reader"]
pub type R = crate::R<ClkoutcfgSpec>;
#[doc = "Register `CLKOUTCFG` writer"]
pub type W = crate::W<ClkoutcfgSpec>;
#[doc = "Selects the clock source for the CLKOUT function. Other values are reserved. Do not use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clkoutsel {
    #[doc = "0: Selects the CPU clock as the CLKOUT source."]
    SelectsTheCpuCloc = 0,
    #[doc = "1: Selects the main oscillator as the CLKOUT source."]
    SelectsTheMainOsc = 1,
    #[doc = "2: Selects the Internal RC oscillator as the CLKOUT source."]
    SelectsTheInternal = 2,
    #[doc = "3: Selects the USB clock as the CLKOUT source."]
    SelectsTheUsbCloc = 3,
    #[doc = "4: Selects the RTC oscillator as the CLKOUT source."]
    SelectsTheRtcOsci = 4,
}
impl From<Clkoutsel> for u8 {
    #[inline(always)]
    fn from(variant: Clkoutsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clkoutsel {
    type Ux = u8;
}
impl crate::IsEnum for Clkoutsel {}
#[doc = "Field `CLKOUTSEL` reader - Selects the clock source for the CLKOUT function. Other values are reserved. Do not use."]
pub type ClkoutselR = crate::FieldReader<Clkoutsel>;
impl ClkoutselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Clkoutsel> {
        match self.bits {
            0 => Some(Clkoutsel::SelectsTheCpuCloc),
            1 => Some(Clkoutsel::SelectsTheMainOsc),
            2 => Some(Clkoutsel::SelectsTheInternal),
            3 => Some(Clkoutsel::SelectsTheUsbCloc),
            4 => Some(Clkoutsel::SelectsTheRtcOsci),
            _ => None,
        }
    }
    #[doc = "Selects the CPU clock as the CLKOUT source."]
    #[inline(always)]
    pub fn is_selects_the_cpu_cloc(&self) -> bool {
        *self == Clkoutsel::SelectsTheCpuCloc
    }
    #[doc = "Selects the main oscillator as the CLKOUT source."]
    #[inline(always)]
    pub fn is_selects_the_main_osc(&self) -> bool {
        *self == Clkoutsel::SelectsTheMainOsc
    }
    #[doc = "Selects the Internal RC oscillator as the CLKOUT source."]
    #[inline(always)]
    pub fn is_selects_the_internal(&self) -> bool {
        *self == Clkoutsel::SelectsTheInternal
    }
    #[doc = "Selects the USB clock as the CLKOUT source."]
    #[inline(always)]
    pub fn is_selects_the_usb_cloc(&self) -> bool {
        *self == Clkoutsel::SelectsTheUsbCloc
    }
    #[doc = "Selects the RTC oscillator as the CLKOUT source."]
    #[inline(always)]
    pub fn is_selects_the_rtc_osci(&self) -> bool {
        *self == Clkoutsel::SelectsTheRtcOsci
    }
}
#[doc = "Field `CLKOUTSEL` writer - Selects the clock source for the CLKOUT function. Other values are reserved. Do not use."]
pub type ClkoutselW<'a, REG> = crate::FieldWriter<'a, REG, 4, Clkoutsel>;
impl<'a, REG> ClkoutselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Selects the CPU clock as the CLKOUT source."]
    #[inline(always)]
    pub fn selects_the_cpu_cloc(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel::SelectsTheCpuCloc)
    }
    #[doc = "Selects the main oscillator as the CLKOUT source."]
    #[inline(always)]
    pub fn selects_the_main_osc(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel::SelectsTheMainOsc)
    }
    #[doc = "Selects the Internal RC oscillator as the CLKOUT source."]
    #[inline(always)]
    pub fn selects_the_internal(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel::SelectsTheInternal)
    }
    #[doc = "Selects the USB clock as the CLKOUT source."]
    #[inline(always)]
    pub fn selects_the_usb_cloc(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel::SelectsTheUsbCloc)
    }
    #[doc = "Selects the RTC oscillator as the CLKOUT source."]
    #[inline(always)]
    pub fn selects_the_rtc_osci(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel::SelectsTheRtcOsci)
    }
}
#[doc = "Field `CLKOUTDIV` reader - Integer value to divide the output clock by, minus one. 0 = Clock is divided by 1 1 = Clock is divided by 2. 2 = Clock is divided by 3. ... 15 = Clock is divided by 16."]
pub type ClkoutdivR = crate::FieldReader;
#[doc = "Field `CLKOUTDIV` writer - Integer value to divide the output clock by, minus one. 0 = Clock is divided by 1 1 = Clock is divided by 2. 2 = Clock is divided by 3. ... 15 = Clock is divided by 16."]
pub type ClkoutdivW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CLKOUT_EN` reader - CLKOUT enable control, allows switching the CLKOUT source without glitches. Clear to stop CLKOUT on the next falling edge. Set to enable CLKOUT."]
pub type ClkoutEnR = crate::BitReader;
#[doc = "Field `CLKOUT_EN` writer - CLKOUT enable control, allows switching the CLKOUT source without glitches. Clear to stop CLKOUT on the next falling edge. Set to enable CLKOUT."]
pub type ClkoutEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKOUT_ACT` reader - CLKOUT activity indication. Reads as 1 when CLKOUT is enabled. Read as 0 when CLKOUT has been disabled via the CLKOUT_EN bit and the clock has completed being stopped."]
pub type ClkoutActR = crate::BitReader;
#[doc = "Field `CLKOUT_ACT` writer - CLKOUT activity indication. Reads as 1 when CLKOUT is enabled. Read as 0 when CLKOUT has been disabled via the CLKOUT_EN bit and the clock has completed being stopped."]
pub type ClkoutActW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Selects the clock source for the CLKOUT function. Other values are reserved. Do not use."]
    #[inline(always)]
    pub fn clkoutsel(&self) -> ClkoutselR {
        ClkoutselR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Integer value to divide the output clock by, minus one. 0 = Clock is divided by 1 1 = Clock is divided by 2. 2 = Clock is divided by 3. ... 15 = Clock is divided by 16."]
    #[inline(always)]
    pub fn clkoutdiv(&self) -> ClkoutdivR {
        ClkoutdivR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - CLKOUT enable control, allows switching the CLKOUT source without glitches. Clear to stop CLKOUT on the next falling edge. Set to enable CLKOUT."]
    #[inline(always)]
    pub fn clkout_en(&self) -> ClkoutEnR {
        ClkoutEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CLKOUT activity indication. Reads as 1 when CLKOUT is enabled. Read as 0 when CLKOUT has been disabled via the CLKOUT_EN bit and the clock has completed being stopped."]
    #[inline(always)]
    pub fn clkout_act(&self) -> ClkoutActR {
        ClkoutActR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Selects the clock source for the CLKOUT function. Other values are reserved. Do not use."]
    #[inline(always)]
    #[must_use]
    pub fn clkoutsel(&mut self) -> ClkoutselW<ClkoutcfgSpec> {
        ClkoutselW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Integer value to divide the output clock by, minus one. 0 = Clock is divided by 1 1 = Clock is divided by 2. 2 = Clock is divided by 3. ... 15 = Clock is divided by 16."]
    #[inline(always)]
    #[must_use]
    pub fn clkoutdiv(&mut self) -> ClkoutdivW<ClkoutcfgSpec> {
        ClkoutdivW::new(self, 4)
    }
    #[doc = "Bit 8 - CLKOUT enable control, allows switching the CLKOUT source without glitches. Clear to stop CLKOUT on the next falling edge. Set to enable CLKOUT."]
    #[inline(always)]
    #[must_use]
    pub fn clkout_en(&mut self) -> ClkoutEnW<ClkoutcfgSpec> {
        ClkoutEnW::new(self, 8)
    }
    #[doc = "Bit 9 - CLKOUT activity indication. Reads as 1 when CLKOUT is enabled. Read as 0 when CLKOUT has been disabled via the CLKOUT_EN bit and the clock has completed being stopped."]
    #[inline(always)]
    #[must_use]
    pub fn clkout_act(&mut self) -> ClkoutActW<ClkoutcfgSpec> {
        ClkoutActW::new(self, 9)
    }
}
#[doc = "Clock Output Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clkoutcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkoutcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkoutcfgSpec;
impl crate::RegisterSpec for ClkoutcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkoutcfg::R`](R) reader structure"]
impl crate::Readable for ClkoutcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`clkoutcfg::W`](W) writer structure"]
impl crate::Writable for ClkoutcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKOUTCFG to value 0"]
impl crate::Resettable for ClkoutcfgSpec {
    const RESET_VALUE: u32 = 0;
}
