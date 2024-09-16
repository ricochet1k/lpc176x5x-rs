#[doc = "Register `MR[%s]` reader"]
pub type R = crate::R<MrSpec>;
#[doc = "Register `MR[%s]` writer"]
pub type W = crate::W<MrSpec>;
#[doc = "Field `MATCH` reader - Timer counter match value."]
pub type MatchR = crate::FieldReader<u32>;
#[doc = "Field `MATCH` writer - Timer counter match value."]
pub type MatchW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Timer counter match value."]
    #[inline(always)]
    pub fn match_(&self) -> MatchR {
        MatchR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Timer counter match value."]
    #[inline(always)]
    #[must_use]
    pub fn match_(&mut self) -> MatchW<MrSpec> {
        MatchW::new(self, 0)
    }
}
#[doc = "Match Register 0. MR0 can be enabled through the MCR to reset the TC, stop both the TC and PC, and/or generate an interrupt every time MR0 matches the TC.\n\nYou can [`read`](crate::Reg::read) this register and get [`mr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MrSpec;
impl crate::RegisterSpec for MrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mr::R`](R) reader structure"]
impl crate::Readable for MrSpec {}
#[doc = "`write(|w| ..)` method takes [`mr::W`](W) writer structure"]
impl crate::Writable for MrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MR[%s]
to value 0"]
impl crate::Resettable for MrSpec {
    const RESET_VALUE: u32 = 0;
}
