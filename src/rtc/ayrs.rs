#[doc = "Register `AYRS` reader"]
pub type R = crate::R<AyrsSpec>;
#[doc = "Register `AYRS` writer"]
pub type W = crate::W<AyrsSpec>;
#[doc = "Field `YEAR` reader - Year value in the range of 0 to 4095."]
pub type YearR = crate::FieldReader<u16>;
#[doc = "Field `YEAR` writer - Year value in the range of 0 to 4095."]
pub type YearW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Year value in the range of 0 to 4095."]
    #[inline(always)]
    pub fn year(&self) -> YearR {
        YearR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Year value in the range of 0 to 4095."]
    #[inline(always)]
    #[must_use]
    pub fn year(&mut self) -> YearW<AyrsSpec> {
        YearW::new(self, 0)
    }
}
#[doc = "Alarm value for Year\n\nYou can [`read`](crate::Reg::read) this register and get [`ayrs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ayrs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AyrsSpec;
impl crate::RegisterSpec for AyrsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ayrs::R`](R) reader structure"]
impl crate::Readable for AyrsSpec {}
#[doc = "`write(|w| ..)` method takes [`ayrs::W`](W) writer structure"]
impl crate::Writable for AyrsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AYRS to value 0"]
impl crate::Resettable for AyrsSpec {
    const RESET_VALUE: u32 = 0;
}
