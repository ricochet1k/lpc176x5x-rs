#[doc = "Register `TEST` reader"]
pub type R = crate::R<TestSpec>;
#[doc = "Register `TEST` writer"]
pub type W = crate::W<TestSpec>;
#[doc = "Field `SCPQ` reader - SHORTCUT PAUSE QUANTA. This bit reduces the effective PAUSE quanta from 64 byte-times to 1 byte-time."]
pub type ScpqR = crate::BitReader;
#[doc = "Field `SCPQ` writer - SHORTCUT PAUSE QUANTA. This bit reduces the effective PAUSE quanta from 64 byte-times to 1 byte-time."]
pub type ScpqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TESTPAUSE` reader - This bit causes the MAC Control sublayer to inhibit transmissions, just as if a PAUSE Receive Control frame with a nonzero pause time parameter was received."]
pub type TestpauseR = crate::BitReader;
#[doc = "Field `TESTPAUSE` writer - This bit causes the MAC Control sublayer to inhibit transmissions, just as if a PAUSE Receive Control frame with a nonzero pause time parameter was received."]
pub type TestpauseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TESTBP` reader - TEST BACKPRESSURE. Setting this bit will cause the MAC to assert backpressure on the link. Backpressure causes preamble to be transmitted, raising carrier sense. A transmit packet from the system will be sent during backpressure."]
pub type TestbpR = crate::BitReader;
#[doc = "Field `TESTBP` writer - TEST BACKPRESSURE. Setting this bit will cause the MAC to assert backpressure on the link. Backpressure causes preamble to be transmitted, raising carrier sense. A transmit packet from the system will be sent during backpressure."]
pub type TestbpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SHORTCUT PAUSE QUANTA. This bit reduces the effective PAUSE quanta from 64 byte-times to 1 byte-time."]
    #[inline(always)]
    pub fn scpq(&self) -> ScpqR {
        ScpqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit causes the MAC Control sublayer to inhibit transmissions, just as if a PAUSE Receive Control frame with a nonzero pause time parameter was received."]
    #[inline(always)]
    pub fn testpause(&self) -> TestpauseR {
        TestpauseR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TEST BACKPRESSURE. Setting this bit will cause the MAC to assert backpressure on the link. Backpressure causes preamble to be transmitted, raising carrier sense. A transmit packet from the system will be sent during backpressure."]
    #[inline(always)]
    pub fn testbp(&self) -> TestbpR {
        TestbpR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SHORTCUT PAUSE QUANTA. This bit reduces the effective PAUSE quanta from 64 byte-times to 1 byte-time."]
    #[inline(always)]
    #[must_use]
    pub fn scpq(&mut self) -> ScpqW<TestSpec> {
        ScpqW::new(self, 0)
    }
    #[doc = "Bit 1 - This bit causes the MAC Control sublayer to inhibit transmissions, just as if a PAUSE Receive Control frame with a nonzero pause time parameter was received."]
    #[inline(always)]
    #[must_use]
    pub fn testpause(&mut self) -> TestpauseW<TestSpec> {
        TestpauseW::new(self, 1)
    }
    #[doc = "Bit 2 - TEST BACKPRESSURE. Setting this bit will cause the MAC to assert backpressure on the link. Backpressure causes preamble to be transmitted, raising carrier sense. A transmit packet from the system will be sent during backpressure."]
    #[inline(always)]
    #[must_use]
    pub fn testbp(&mut self) -> TestbpW<TestSpec> {
        TestbpW::new(self, 2)
    }
}
#[doc = "Test register.\n\nYou can [`read`](crate::Reg::read) this register and get [`test::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`test::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TestSpec;
impl crate::RegisterSpec for TestSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`test::R`](R) reader structure"]
impl crate::Readable for TestSpec {}
#[doc = "`write(|w| ..)` method takes [`test::W`](W) writer structure"]
impl crate::Writable for TestSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TEST to value 0"]
impl crate::Resettable for TestSpec {
    const RESET_VALUE: u32 = 0;
}
