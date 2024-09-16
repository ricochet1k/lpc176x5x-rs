#[doc = "Register `CAP[%s]` reader"]
pub type R = crate::R<CapSpec>;
#[doc = "Field `CAP` reader - Current TC value at a capture event."]
pub type CapR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Current TC value at a capture event."]
    #[inline(always)]
    pub fn cap(&self) -> CapR {
        CapR::new(self.bits)
    }
}
#[doc = "Capture register\n\nYou can [`read`](crate::Reg::read) this register and get [`cap::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CapSpec;
impl crate::RegisterSpec for CapSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cap::R`](R) reader structure"]
impl crate::Readable for CapSpec {}
#[doc = "`reset()` method sets CAP[%s]
to value 0"]
impl crate::Resettable for CapSpec {
    const RESET_VALUE: u32 = 0;
}
