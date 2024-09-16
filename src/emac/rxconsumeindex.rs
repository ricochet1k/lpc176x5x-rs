#[doc = "Register `RXCONSUMEINDEX` reader"]
pub type R = crate::R<RxconsumeindexSpec>;
#[doc = "Register `RXCONSUMEINDEX` writer"]
pub type W = crate::W<RxconsumeindexSpec>;
#[doc = "Field `RXCONSUMEIX` reader - Index of the descriptor that is going to be processed next by the receive"]
pub type RxconsumeixR = crate::FieldReader<u16>;
#[doc = "Field `RXCONSUMEIX` writer - Index of the descriptor that is going to be processed next by the receive"]
pub type RxconsumeixW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Index of the descriptor that is going to be processed next by the receive"]
    #[inline(always)]
    pub fn rxconsumeix(&self) -> RxconsumeixR {
        RxconsumeixR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Index of the descriptor that is going to be processed next by the receive"]
    #[inline(always)]
    #[must_use]
    pub fn rxconsumeix(&mut self) -> RxconsumeixW<RxconsumeindexSpec> {
        RxconsumeixW::new(self, 0)
    }
}
#[doc = "Receive consume index register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rxconsumeindex::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxconsumeindex::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxconsumeindexSpec;
impl crate::RegisterSpec for RxconsumeindexSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxconsumeindex::R`](R) reader structure"]
impl crate::Readable for RxconsumeindexSpec {}
#[doc = "`write(|w| ..)` method takes [`rxconsumeindex::W`](W) writer structure"]
impl crate::Writable for RxconsumeindexSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXCONSUMEINDEX to value 0"]
impl crate::Resettable for RxconsumeindexSpec {
    const RESET_VALUE: u32 = 0;
}
