#[doc = "Register `ADOM` reader"]
pub type R = crate::R<AdomSpec>;
#[doc = "Register `ADOM` writer"]
pub type W = crate::W<AdomSpec>;
#[doc = "Field `DOM` reader - Day of month value in the range of 1 to 28, 29, 30, or 31 (depending on the month and whether it is a leap year)."]
pub type DomR = crate::FieldReader;
#[doc = "Field `DOM` writer - Day of month value in the range of 1 to 28, 29, 30, or 31 (depending on the month and whether it is a leap year)."]
pub type DomW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Day of month value in the range of 1 to 28, 29, 30, or 31 (depending on the month and whether it is a leap year)."]
    #[inline(always)]
    pub fn dom(&self) -> DomR {
        DomR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Day of month value in the range of 1 to 28, 29, 30, or 31 (depending on the month and whether it is a leap year)."]
    #[inline(always)]
    #[must_use]
    pub fn dom(&mut self) -> DomW<AdomSpec> {
        DomW::new(self, 0)
    }
}
#[doc = "Alarm value for Day of Month\n\nYou can [`read`](crate::Reg::read) this register and get [`adom::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adom::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdomSpec;
impl crate::RegisterSpec for AdomSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adom::R`](R) reader structure"]
impl crate::Readable for AdomSpec {}
#[doc = "`write(|w| ..)` method takes [`adom::W`](W) writer structure"]
impl crate::Writable for AdomSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADOM to value 0"]
impl crate::Resettable for AdomSpec {
    const RESET_VALUE: u32 = 0;
}
