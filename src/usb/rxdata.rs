#[doc = "Register `RXDATA` reader"]
pub type R = crate::R<RxdataSpec>;
#[doc = "Field `RX_DATA` reader - Data received."]
pub type RxDataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Data received."]
    #[inline(always)]
    pub fn rx_data(&self) -> RxDataR {
        RxDataR::new(self.bits)
    }
}
#[doc = "USB Receive Data\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdata::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxdataSpec;
impl crate::RegisterSpec for RxdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxdata::R`](R) reader structure"]
impl crate::Readable for RxdataSpec {}
#[doc = "`reset()` method sets RXDATA to value 0"]
impl crate::Resettable for RxdataSpec {
    const RESET_VALUE: u32 = 0;
}
