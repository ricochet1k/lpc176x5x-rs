#[doc = "Register `DOW` reader"]
pub type R = crate::R<DowSpec>;
#[doc = "Register `DOW` writer"]
pub type W = crate::W<DowSpec>;
#[doc = "Field `DOW` reader - Day of week value in the range of 0 to 6."]
pub type DowR = crate::FieldReader;
#[doc = "Field `DOW` writer - Day of week value in the range of 0 to 6."]
pub type DowW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Day of week value in the range of 0 to 6."]
    #[inline(always)]
    pub fn dow(&self) -> DowR {
        DowR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Day of week value in the range of 0 to 6."]
    #[inline(always)]
    #[must_use]
    pub fn dow(&mut self) -> DowW<DowSpec> {
        DowW::new(self, 0)
    }
}
#[doc = "Day of Week Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dow::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dow::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DowSpec;
impl crate::RegisterSpec for DowSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dow::R`](R) reader structure"]
impl crate::Readable for DowSpec {}
#[doc = "`write(|w| ..)` method takes [`dow::W`](W) writer structure"]
impl crate::Writable for DowSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOW to value 0"]
impl crate::Resettable for DowSpec {
    const RESET_VALUE: u32 = 0;
}
