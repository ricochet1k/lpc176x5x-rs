#[doc = "Register `TMR` reader"]
pub type R = crate::R<TmrSpec>;
#[doc = "Register `TMR` writer"]
pub type W = crate::W<TmrSpec>;
#[doc = "Field `TIMEOUT_CNT` reader - The TMR interrupt is set when TMR_CNT reaches this value."]
pub type TimeoutCntR = crate::FieldReader<u16>;
#[doc = "Field `TIMEOUT_CNT` writer - The TMR interrupt is set when TMR_CNT reaches this value."]
pub type TimeoutCntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - The TMR interrupt is set when TMR_CNT reaches this value."]
    #[inline(always)]
    pub fn timeout_cnt(&self) -> TimeoutCntR {
        TimeoutCntR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The TMR interrupt is set when TMR_CNT reaches this value."]
    #[inline(always)]
    #[must_use]
    pub fn timeout_cnt(&mut self) -> TimeoutCntW<TmrSpec> {
        TimeoutCntW::new(self, 0)
    }
}
#[doc = "OTG Timer\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TmrSpec;
impl crate::RegisterSpec for TmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmr::R`](R) reader structure"]
impl crate::Readable for TmrSpec {}
#[doc = "`write(|w| ..)` method takes [`tmr::W`](W) writer structure"]
impl crate::Writable for TmrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TMR to value 0xffff"]
impl crate::Resettable for TmrSpec {
    const RESET_VALUE: u32 = 0xffff;
}
