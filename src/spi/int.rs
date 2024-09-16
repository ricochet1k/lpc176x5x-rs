#[doc = "Register `INT` reader"]
pub type R = crate::R<IntSpec>;
#[doc = "Register `INT` writer"]
pub type W = crate::W<IntSpec>;
#[doc = "Field `SPIF` reader - SPI interrupt flag. Set by the SPI interface to generate an interrupt. Cleared by writing a 1 to this bit. Note: this bit will be set once when SPIE = 1 and at least one of SPIF and WCOL bits is 1. However, only when the SPI Interrupt bit is set and SPI0 Interrupt is enabled in the NVIC, SPI based interrupt can be processed by interrupt handling software."]
pub type SpifR = crate::BitReader;
#[doc = "Field `SPIF` writer - SPI interrupt flag. Set by the SPI interface to generate an interrupt. Cleared by writing a 1 to this bit. Note: this bit will be set once when SPIE = 1 and at least one of SPIF and WCOL bits is 1. However, only when the SPI Interrupt bit is set and SPI0 Interrupt is enabled in the NVIC, SPI based interrupt can be processed by interrupt handling software."]
pub type SpifW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SPI interrupt flag. Set by the SPI interface to generate an interrupt. Cleared by writing a 1 to this bit. Note: this bit will be set once when SPIE = 1 and at least one of SPIF and WCOL bits is 1. However, only when the SPI Interrupt bit is set and SPI0 Interrupt is enabled in the NVIC, SPI based interrupt can be processed by interrupt handling software."]
    #[inline(always)]
    pub fn spif(&self) -> SpifR {
        SpifR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI interrupt flag. Set by the SPI interface to generate an interrupt. Cleared by writing a 1 to this bit. Note: this bit will be set once when SPIE = 1 and at least one of SPIF and WCOL bits is 1. However, only when the SPI Interrupt bit is set and SPI0 Interrupt is enabled in the NVIC, SPI based interrupt can be processed by interrupt handling software."]
    #[inline(always)]
    #[must_use]
    pub fn spif(&mut self) -> SpifW<IntSpec> {
        SpifW::new(self, 0)
    }
}
#[doc = "SPI Interrupt Flag. This register contains the interrupt flag for the SPI interface.\n\nYou can [`read`](crate::Reg::read) this register and get [`int::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntSpec;
impl crate::RegisterSpec for IntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int::R`](R) reader structure"]
impl crate::Readable for IntSpec {}
#[doc = "`write(|w| ..)` method takes [`int::W`](W) writer structure"]
impl crate::Writable for IntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT to value 0"]
impl crate::Resettable for IntSpec {
    const RESET_VALUE: u32 = 0;
}
