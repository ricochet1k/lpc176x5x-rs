#[doc = "Register `GPREG%s` reader"]
pub type R = crate::R<GpregSpec>;
#[doc = "Register `GPREG%s` writer"]
pub type W = crate::W<GpregSpec>;
#[doc = "Field `GP` reader - General purpose storage."]
pub type GpR = crate::FieldReader<u32>;
#[doc = "Field `GP` writer - General purpose storage."]
pub type GpW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - General purpose storage."]
    #[inline(always)]
    pub fn gp(&self) -> GpR {
        GpR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - General purpose storage."]
    #[inline(always)]
    #[must_use]
    pub fn gp(&mut self) -> GpW<GpregSpec> {
        GpW::new(self, 0)
    }
}
#[doc = "General Purpose Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`gpreg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpreg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpregSpec;
impl crate::RegisterSpec for GpregSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpreg::R`](R) reader structure"]
impl crate::Readable for GpregSpec {}
#[doc = "`write(|w| ..)` method takes [`gpreg::W`](W) writer structure"]
impl crate::Writable for GpregSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPREG%s to value 0"]
impl crate::Resettable for GpregSpec {
    const RESET_VALUE: u32 = 0;
}
