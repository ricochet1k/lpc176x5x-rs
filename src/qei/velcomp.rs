#[doc = "Register `VELCOMP` reader"]
pub type R = crate::R<VelcompSpec>;
#[doc = "Register `VELCOMP` writer"]
pub type W = crate::W<VelcompSpec>;
#[doc = "Field `VELPC` reader - Compare velocity pulse count."]
pub type VelpcR = crate::FieldReader<u32>;
#[doc = "Field `VELPC` writer - Compare velocity pulse count."]
pub type VelpcW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Compare velocity pulse count."]
    #[inline(always)]
    pub fn velpc(&self) -> VelpcR {
        VelpcR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Compare velocity pulse count."]
    #[inline(always)]
    #[must_use]
    pub fn velpc(&mut self) -> VelpcW<VelcompSpec> {
        VelpcW::new(self, 0)
    }
}
#[doc = "Velocity compare register\n\nYou can [`read`](crate::Reg::read) this register and get [`velcomp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`velcomp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VelcompSpec;
impl crate::RegisterSpec for VelcompSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`velcomp::R`](R) reader structure"]
impl crate::Readable for VelcompSpec {}
#[doc = "`write(|w| ..)` method takes [`velcomp::W`](W) writer structure"]
impl crate::Writable for VelcompSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VELCOMP to value 0"]
impl crate::Resettable for VelcompSpec {
    const RESET_VALUE: u32 = 0;
}
