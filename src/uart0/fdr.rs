#[doc = "Register `FDR` reader"]
pub type R = crate::R<FdrSpec>;
#[doc = "Register `FDR` writer"]
pub type W = crate::W<FdrSpec>;
#[doc = "Field `DIVADDVAL` reader - Baud-rate generation pre-scaler divisor value. If this field is 0, fractional baud-rate generator will not impact the UARTn baudrate."]
pub type DivaddvalR = crate::FieldReader;
#[doc = "Field `DIVADDVAL` writer - Baud-rate generation pre-scaler divisor value. If this field is 0, fractional baud-rate generator will not impact the UARTn baudrate."]
pub type DivaddvalW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MULVAL` reader - Baud-rate pre-scaler multiplier value. This field must be greater or equal 1 for UARTn to operate properly, regardless of whether the fractional baud-rate generator is used or not."]
pub type MulvalR = crate::FieldReader;
#[doc = "Field `MULVAL` writer - Baud-rate pre-scaler multiplier value. This field must be greater or equal 1 for UARTn to operate properly, regardless of whether the fractional baud-rate generator is used or not."]
pub type MulvalW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Baud-rate generation pre-scaler divisor value. If this field is 0, fractional baud-rate generator will not impact the UARTn baudrate."]
    #[inline(always)]
    pub fn divaddval(&self) -> DivaddvalR {
        DivaddvalR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Baud-rate pre-scaler multiplier value. This field must be greater or equal 1 for UARTn to operate properly, regardless of whether the fractional baud-rate generator is used or not."]
    #[inline(always)]
    pub fn mulval(&self) -> MulvalR {
        MulvalR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Baud-rate generation pre-scaler divisor value. If this field is 0, fractional baud-rate generator will not impact the UARTn baudrate."]
    #[inline(always)]
    #[must_use]
    pub fn divaddval(&mut self) -> DivaddvalW<FdrSpec> {
        DivaddvalW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Baud-rate pre-scaler multiplier value. This field must be greater or equal 1 for UARTn to operate properly, regardless of whether the fractional baud-rate generator is used or not."]
    #[inline(always)]
    #[must_use]
    pub fn mulval(&mut self) -> MulvalW<FdrSpec> {
        MulvalW::new(self, 4)
    }
}
#[doc = "Fractional Divider Register. Generates a clock input for the baud rate divider.\n\nYou can [`read`](crate::Reg::read) this register and get [`fdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdrSpec;
impl crate::RegisterSpec for FdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdr::R`](R) reader structure"]
impl crate::Readable for FdrSpec {}
#[doc = "`write(|w| ..)` method takes [`fdr::W`](W) writer structure"]
impl crate::Writable for FdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FDR to value 0x10"]
impl crate::Resettable for FdrSpec {
    const RESET_VALUE: u32 = 0x10;
}
