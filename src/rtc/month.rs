#[doc = "Register `MONTH` reader"]
pub type R = crate::R<MonthSpec>;
#[doc = "Register `MONTH` writer"]
pub type W = crate::W<MonthSpec>;
#[doc = "Field `MONTH` reader - Month value in the range of 1 to 12."]
pub type MonthR = crate::FieldReader;
#[doc = "Field `MONTH` writer - Month value in the range of 1 to 12."]
pub type MonthW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Month value in the range of 1 to 12."]
    #[inline(always)]
    pub fn month(&self) -> MonthR {
        MonthR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Month value in the range of 1 to 12."]
    #[inline(always)]
    #[must_use]
    pub fn month(&mut self) -> MonthW<MonthSpec> {
        MonthW::new(self, 0)
    }
}
#[doc = "Months Register\n\nYou can [`read`](crate::Reg::read) this register and get [`month::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`month::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MonthSpec;
impl crate::RegisterSpec for MonthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`month::R`](R) reader structure"]
impl crate::Readable for MonthSpec {}
#[doc = "`write(|w| ..)` method takes [`month::W`](W) writer structure"]
impl crate::Writable for MonthSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MONTH to value 0"]
impl crate::Resettable for MonthSpec {
    const RESET_VALUE: u32 = 0;
}
