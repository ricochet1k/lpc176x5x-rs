#[doc = "Register `VEL` reader"]
pub type R = crate::R<VelSpec>;
#[doc = "Field `VELPC` reader - Current velocity pulse count."]
pub type VelpcR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Current velocity pulse count."]
    #[inline(always)]
    pub fn velpc(&self) -> VelpcR {
        VelpcR::new(self.bits)
    }
}
#[doc = "Velocity counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`vel::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VelSpec;
impl crate::RegisterSpec for VelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vel::R`](R) reader structure"]
impl crate::Readable for VelSpec {}
#[doc = "`reset()` method sets VEL to value 0"]
impl crate::Resettable for VelSpec {
    const RESET_VALUE: u32 = 0;
}
