#[doc = "Register `CCR` reader"]
pub type R = crate::R<CcrSpec>;
#[doc = "Register `CCR` writer"]
pub type W = crate::W<CcrSpec>;
#[doc = "Field `COUNTER` reader - SPI0 Clock counter setting."]
pub type CounterR = crate::FieldReader;
#[doc = "Field `COUNTER` writer - SPI0 Clock counter setting."]
pub type CounterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - SPI0 Clock counter setting."]
    #[inline(always)]
    pub fn counter(&self) -> CounterR {
        CounterR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SPI0 Clock counter setting."]
    #[inline(always)]
    #[must_use]
    pub fn counter(&mut self) -> CounterW<CcrSpec> {
        CounterW::new(self, 0)
    }
}
#[doc = "SPI Clock Counter Register. This register controls the frequency of a master's SCK0.\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcrSpec;
impl crate::RegisterSpec for CcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr::R`](R) reader structure"]
impl crate::Readable for CcrSpec {}
#[doc = "`write(|w| ..)` method takes [`ccr::W`](W) writer structure"]
impl crate::Writable for CcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCR to value 0"]
impl crate::Resettable for CcrSpec {
    const RESET_VALUE: u32 = 0;
}
