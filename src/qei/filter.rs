#[doc = "Register `FILTER` reader"]
pub type R = crate::R<FilterSpec>;
#[doc = "Register `FILTER` writer"]
pub type W = crate::W<FilterSpec>;
#[doc = "Field `FILTA` reader - Digital filter sampling delay."]
pub type FiltaR = crate::FieldReader<u32>;
#[doc = "Field `FILTA` writer - Digital filter sampling delay."]
pub type FiltaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Digital filter sampling delay."]
    #[inline(always)]
    pub fn filta(&self) -> FiltaR {
        FiltaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Digital filter sampling delay."]
    #[inline(always)]
    #[must_use]
    pub fn filta(&mut self) -> FiltaW<FilterSpec> {
        FiltaW::new(self, 0)
    }
}
#[doc = "Digital filter register\n\nYou can [`read`](crate::Reg::read) this register and get [`filter::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filter::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FilterSpec;
impl crate::RegisterSpec for FilterSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`filter::R`](R) reader structure"]
impl crate::Readable for FilterSpec {}
#[doc = "`write(|w| ..)` method takes [`filter::W`](W) writer structure"]
impl crate::Writable for FilterSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FILTER to value 0"]
impl crate::Resettable for FilterSpec {
    const RESET_VALUE: u32 = 0;
}
