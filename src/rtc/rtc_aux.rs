#[doc = "Register `RTC_AUX` reader"]
pub type R = crate::R<RtcAuxSpec>;
#[doc = "Register `RTC_AUX` writer"]
pub type W = crate::W<RtcAuxSpec>;
#[doc = "Field `RTC_OSCF` reader - RTC Oscillator Fail detect flag. Read: this bit is set if the RTC oscillator stops, and when RTC power is first turned on. An interrupt will occur when this bit is set, the RTC_OSCFEN bit in RTC_AUXEN is a 1, and the RTC interrupt is enabled in the NVIC. Write: writing a 1 to this bit clears the flag."]
pub type RtcOscfR = crate::BitReader;
#[doc = "Field `RTC_OSCF` writer - RTC Oscillator Fail detect flag. Read: this bit is set if the RTC oscillator stops, and when RTC power is first turned on. An interrupt will occur when this bit is set, the RTC_OSCFEN bit in RTC_AUXEN is a 1, and the RTC interrupt is enabled in the NVIC. Write: writing a 1 to this bit clears the flag."]
pub type RtcOscfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC_PDOUT` reader - When 0: the RTC_ALARM pin reflects the RTC alarm status. When 1: the RTC_ALARM pin indicates Deep Power-down mode."]
pub type RtcPdoutR = crate::BitReader;
#[doc = "Field `RTC_PDOUT` writer - When 0: the RTC_ALARM pin reflects the RTC alarm status. When 1: the RTC_ALARM pin indicates Deep Power-down mode."]
pub type RtcPdoutW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - RTC Oscillator Fail detect flag. Read: this bit is set if the RTC oscillator stops, and when RTC power is first turned on. An interrupt will occur when this bit is set, the RTC_OSCFEN bit in RTC_AUXEN is a 1, and the RTC interrupt is enabled in the NVIC. Write: writing a 1 to this bit clears the flag."]
    #[inline(always)]
    pub fn rtc_oscf(&self) -> RtcOscfR {
        RtcOscfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - When 0: the RTC_ALARM pin reflects the RTC alarm status. When 1: the RTC_ALARM pin indicates Deep Power-down mode."]
    #[inline(always)]
    pub fn rtc_pdout(&self) -> RtcPdoutR {
        RtcPdoutR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - RTC Oscillator Fail detect flag. Read: this bit is set if the RTC oscillator stops, and when RTC power is first turned on. An interrupt will occur when this bit is set, the RTC_OSCFEN bit in RTC_AUXEN is a 1, and the RTC interrupt is enabled in the NVIC. Write: writing a 1 to this bit clears the flag."]
    #[inline(always)]
    #[must_use]
    pub fn rtc_oscf(&mut self) -> RtcOscfW<RtcAuxSpec> {
        RtcOscfW::new(self, 4)
    }
    #[doc = "Bit 6 - When 0: the RTC_ALARM pin reflects the RTC alarm status. When 1: the RTC_ALARM pin indicates Deep Power-down mode."]
    #[inline(always)]
    #[must_use]
    pub fn rtc_pdout(&mut self) -> RtcPdoutW<RtcAuxSpec> {
        RtcPdoutW::new(self, 6)
    }
}
#[doc = "RTC Auxiliary control register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_aux::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_aux::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcAuxSpec;
impl crate::RegisterSpec for RtcAuxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_aux::R`](R) reader structure"]
impl crate::Readable for RtcAuxSpec {}
#[doc = "`write(|w| ..)` method takes [`rtc_aux::W`](W) writer structure"]
impl crate::Writable for RtcAuxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTC_AUX to value 0x10"]
impl crate::Resettable for RtcAuxSpec {
    const RESET_VALUE: u32 = 0x10;
}
