#[doc = "Register `COMPVAL` reader"]
pub type R = crate::R<CompvalSpec>;
#[doc = "Register `COMPVAL` writer"]
pub type W = crate::W<CompvalSpec>;
#[doc = "Field `RICOMP` reader - Compare register. Holds the compare value which is compared to the counter."]
pub type RicompR = crate::FieldReader<u32>;
#[doc = "Field `RICOMP` writer - Compare register. Holds the compare value which is compared to the counter."]
pub type RicompW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Compare register. Holds the compare value which is compared to the counter."]
    #[inline(always)]
    pub fn ricomp(&self) -> RicompR {
        RicompR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Compare register. Holds the compare value which is compared to the counter."]
    #[inline(always)]
    #[must_use]
    pub fn ricomp(&mut self) -> RicompW<CompvalSpec> {
        RicompW::new(self, 0)
    }
}
#[doc = "Compare register\n\nYou can [`read`](crate::Reg::read) this register and get [`compval::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`compval::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CompvalSpec;
impl crate::RegisterSpec for CompvalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`compval::R`](R) reader structure"]
impl crate::Readable for CompvalSpec {}
#[doc = "`write(|w| ..)` method takes [`compval::W`](W) writer structure"]
impl crate::Writable for CompvalSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets COMPVAL to value 0xffff_ffff"]
impl crate::Resettable for CompvalSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
