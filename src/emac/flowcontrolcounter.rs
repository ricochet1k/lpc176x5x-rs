#[doc = "Register `FLOWCONTROLCOUNTER` reader"]
pub type R = crate::R<FlowcontrolcounterSpec>;
#[doc = "Register `FLOWCONTROLCOUNTER` writer"]
pub type W = crate::W<FlowcontrolcounterSpec>;
#[doc = "Field `MC` reader - MirrorCounter. In full duplex mode the MirrorCounter specifies the number of cycles before re-issuing the Pause control frame."]
pub type McR = crate::FieldReader<u16>;
#[doc = "Field `MC` writer - MirrorCounter. In full duplex mode the MirrorCounter specifies the number of cycles before re-issuing the Pause control frame."]
pub type McW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PT` reader - PauseTimer. In full-duplex mode the PauseTimer specifies the value that is inserted into the pause timer field of a pause flow control frame. In half duplex mode the PauseTimer specifies the number of backpressure cycles."]
pub type PtR = crate::FieldReader<u16>;
#[doc = "Field `PT` writer - PauseTimer. In full-duplex mode the PauseTimer specifies the value that is inserted into the pause timer field of a pause flow control frame. In half duplex mode the PauseTimer specifies the number of backpressure cycles."]
pub type PtW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - MirrorCounter. In full duplex mode the MirrorCounter specifies the number of cycles before re-issuing the Pause control frame."]
    #[inline(always)]
    pub fn mc(&self) -> McR {
        McR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - PauseTimer. In full-duplex mode the PauseTimer specifies the value that is inserted into the pause timer field of a pause flow control frame. In half duplex mode the PauseTimer specifies the number of backpressure cycles."]
    #[inline(always)]
    pub fn pt(&self) -> PtR {
        PtR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - MirrorCounter. In full duplex mode the MirrorCounter specifies the number of cycles before re-issuing the Pause control frame."]
    #[inline(always)]
    #[must_use]
    pub fn mc(&mut self) -> McW<FlowcontrolcounterSpec> {
        McW::new(self, 0)
    }
    #[doc = "Bits 16:31 - PauseTimer. In full-duplex mode the PauseTimer specifies the value that is inserted into the pause timer field of a pause flow control frame. In half duplex mode the PauseTimer specifies the number of backpressure cycles."]
    #[inline(always)]
    #[must_use]
    pub fn pt(&mut self) -> PtW<FlowcontrolcounterSpec> {
        PtW::new(self, 16)
    }
}
#[doc = "Flow control counter register.\n\nYou can [`read`](crate::Reg::read) this register and get [`flowcontrolcounter::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flowcontrolcounter::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlowcontrolcounterSpec;
impl crate::RegisterSpec for FlowcontrolcounterSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flowcontrolcounter::R`](R) reader structure"]
impl crate::Readable for FlowcontrolcounterSpec {}
#[doc = "`write(|w| ..)` method takes [`flowcontrolcounter::W`](W) writer structure"]
impl crate::Writable for FlowcontrolcounterSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLOWCONTROLCOUNTER to value 0"]
impl crate::Resettable for FlowcontrolcounterSpec {
    const RESET_VALUE: u32 = 0;
}
