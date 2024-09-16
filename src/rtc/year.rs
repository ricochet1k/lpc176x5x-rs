#[doc = "Register `YEAR` reader"]
pub type R = crate::R<YearSpec>;
#[doc = "Register `YEAR` writer"]
pub type W = crate::W<YearSpec>;
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
    pub fn year(&mut self) -> YearW<YearSpec> {
        YearW::new(self, 0)
    }
}
#[doc = "Years Register\n\nYou can [`read`](crate::Reg::read) this register and get [`year::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`year::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct YearSpec;
impl crate::RegisterSpec for YearSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`year::R`](R) reader structure"]
impl crate::Readable for YearSpec {}
#[doc = "`write(|w| ..)` method takes [`year::W`](W) writer structure"]
impl crate::Writable for YearSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets YEAR to value 0"]
impl crate::Resettable for YearSpec {
    const RESET_VALUE: u32 = 0;
}
