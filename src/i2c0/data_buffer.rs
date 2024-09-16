#[doc = "Register `DATA_BUFFER` reader"]
pub type R = crate::R<DataBufferSpec>;
#[doc = "Field `Data` reader - This register holds contents of the 8 MSBs of the DAT shift register."]
pub type DataR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - This register holds contents of the 8 MSBs of the DAT shift register."]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Data buffer register. The contents of the 8 MSBs of the DAT shift register will be transferred to the DATA_BUFFER automatically after every nine bits (8 bits of data plus ACK or NACK) has been received on the bus.\n\nYou can [`read`](crate::Reg::read) this register and get [`data_buffer::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataBufferSpec;
impl crate::RegisterSpec for DataBufferSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data_buffer::R`](R) reader structure"]
impl crate::Readable for DataBufferSpec {}
#[doc = "`reset()` method sets DATA_BUFFER to value 0"]
impl crate::Resettable for DataBufferSpec {
    const RESET_VALUE: u32 = 0;
}
