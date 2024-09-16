#[doc = "Register `DLL` reader"]
pub type R = crate::R<DllSpec>;
#[doc = "Register `DLL` writer"]
pub type W = crate::W<DllSpec>;
#[doc = "Field `DLLSB` reader - The UART1 Divisor Latch LSB Register, along with the U1DLM register, determines the baud rate of the UART1."]
pub type DllsbR = crate::FieldReader;
#[doc = "Field `DLLSB` writer - The UART1 Divisor Latch LSB Register, along with the U1DLM register, determines the baud rate of the UART1."]
pub type DllsbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - The UART1 Divisor Latch LSB Register, along with the U1DLM register, determines the baud rate of the UART1."]
    #[inline(always)]
    pub fn dllsb(&self) -> DllsbR {
        DllsbR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The UART1 Divisor Latch LSB Register, along with the U1DLM register, determines the baud rate of the UART1."]
    #[inline(always)]
    #[must_use]
    pub fn dllsb(&mut self) -> DllsbW<DllSpec> {
        DllsbW::new(self, 0)
    }
}
#[doc = "DLAB =1. Divisor Latch LSB. Least significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider.\n\nYou can [`read`](crate::Reg::read) this register and get [`dll::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dll::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DllSpec;
impl crate::RegisterSpec for DllSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dll::R`](R) reader structure"]
impl crate::Readable for DllSpec {}
#[doc = "`write(|w| ..)` method takes [`dll::W`](W) writer structure"]
impl crate::Writable for DllSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DLL to value 0x01"]
impl crate::Resettable for DllSpec {
    const RESET_VALUE: u32 = 0x01;
}
