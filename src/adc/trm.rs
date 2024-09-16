#[doc = "Register `TRM` reader"]
pub type R = crate::R<TrmSpec>;
#[doc = "Register `TRM` writer"]
pub type W = crate::W<TrmSpec>;
#[doc = "Field `ADCOFFS` reader - Offset trim bits for ADC operation. Initialized by the boot code. Can be overwritten by the user."]
pub type AdcoffsR = crate::FieldReader;
#[doc = "Field `ADCOFFS` writer - Offset trim bits for ADC operation. Initialized by the boot code. Can be overwritten by the user."]
pub type AdcoffsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TRIM` reader - written-to by boot code. Can not be overwritten by the user. These bits are locked after boot code write."]
pub type TrimR = crate::FieldReader;
#[doc = "Field `TRIM` writer - written-to by boot code. Can not be overwritten by the user. These bits are locked after boot code write."]
pub type TrimW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 4:7 - Offset trim bits for ADC operation. Initialized by the boot code. Can be overwritten by the user."]
    #[inline(always)]
    pub fn adcoffs(&self) -> AdcoffsR {
        AdcoffsR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - written-to by boot code. Can not be overwritten by the user. These bits are locked after boot code write."]
    #[inline(always)]
    pub fn trim(&self) -> TrimR {
        TrimR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 4:7 - Offset trim bits for ADC operation. Initialized by the boot code. Can be overwritten by the user."]
    #[inline(always)]
    #[must_use]
    pub fn adcoffs(&mut self) -> AdcoffsW<TrmSpec> {
        AdcoffsW::new(self, 4)
    }
    #[doc = "Bits 8:11 - written-to by boot code. Can not be overwritten by the user. These bits are locked after boot code write."]
    #[inline(always)]
    #[must_use]
    pub fn trim(&mut self) -> TrimW<TrmSpec> {
        TrimW::new(self, 8)
    }
}
#[doc = "ADC trim register.\n\nYou can [`read`](crate::Reg::read) this register and get [`trm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrmSpec;
impl crate::RegisterSpec for TrmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trm::R`](R) reader structure"]
impl crate::Readable for TrmSpec {}
#[doc = "`write(|w| ..)` method takes [`trm::W`](W) writer structure"]
impl crate::Writable for TrmSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRM to value 0"]
impl crate::Resettable for TrmSpec {
    const RESET_VALUE: u32 = 0;
}
