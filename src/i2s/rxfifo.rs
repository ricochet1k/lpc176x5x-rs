#[doc = "Register `RXFIFO` reader"]
pub type R = crate::R<RxfifoSpec>;
#[doc = "Field `I2SRXFIFO` reader - 8 x 32-bit transmit FIFO."]
pub type I2srxfifoR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 8 x 32-bit transmit FIFO."]
    #[inline(always)]
    pub fn i2srxfifo(&self) -> I2srxfifoR {
        I2srxfifoR::new(self.bits)
    }
}
#[doc = "I2S Receive FIFO. Access register for the 8 x 32-bit receiver FIFO.\n\nYou can [`read`](crate::Reg::read) this register and get [`rxfifo::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">The register is <b>modified</b> in some way after a read operation.</div>"]
pub struct RxfifoSpec;
impl crate::RegisterSpec for RxfifoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxfifo::R`](R) reader structure"]
impl crate::Readable for RxfifoSpec {}
#[doc = "`reset()` method sets RXFIFO to value 0"]
impl crate::Resettable for RxfifoSpec {
    const RESET_VALUE: u32 = 0;
}
