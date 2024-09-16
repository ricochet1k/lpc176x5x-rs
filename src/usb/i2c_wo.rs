#[doc = "Register `I2C_WO` writer"]
pub type W = crate::W<I2cWoSpec>;
#[doc = "Field `TXDATA` writer - Transmit data."]
pub type TxdataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `START` writer - When 1, issue a START condition before transmitting this byte."]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP` writer - When 1, issue a STOP condition after transmitting this byte."]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bits 0:7 - Transmit data."]
    #[inline(always)]
    #[must_use]
    pub fn txdata(&mut self) -> TxdataW<I2cWoSpec> {
        TxdataW::new(self, 0)
    }
    #[doc = "Bit 8 - When 1, issue a START condition before transmitting this byte."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> StartW<I2cWoSpec> {
        StartW::new(self, 8)
    }
    #[doc = "Bit 9 - When 1, issue a STOP condition after transmitting this byte."]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> StopW<I2cWoSpec> {
        StopW::new(self, 9)
    }
}
#[doc = "I2C Transmit\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_wo::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cWoSpec;
impl crate::RegisterSpec for I2cWoSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`i2c_wo::W`](W) writer structure"]
impl crate::Writable for I2cWoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2C_WO to value 0"]
impl crate::Resettable for I2cWoSpec {
    const RESET_VALUE: u32 = 0;
}
