#[doc = "Register `TIME` reader"]
pub type R = crate::R<TimeSpec>;
#[doc = "Field `VELVAL` reader - Current velocity timer value."]
pub type VelvalR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Current velocity timer value."]
    #[inline(always)]
    pub fn velval(&self) -> VelvalR {
        VelvalR::new(self.bits)
    }
}
#[doc = "Velocity timer register\n\nYou can [`read`](crate::Reg::read) this register and get [`time::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimeSpec;
impl crate::RegisterSpec for TimeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`time::R`](R) reader structure"]
impl crate::Readable for TimeSpec {}
#[doc = "`reset()` method sets TIME to value 0"]
impl crate::Resettable for TimeSpec {
    const RESET_VALUE: u32 = 0;
}
