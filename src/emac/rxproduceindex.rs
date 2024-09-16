#[doc = "Register `RXPRODUCEINDEX` reader"]
pub type R = crate::R<RxproduceindexSpec>;
#[doc = "Field `RXPRODUCEIX` reader - Index of the descriptor that is going to be filled next by the receive datapath."]
pub type RxproduceixR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Index of the descriptor that is going to be filled next by the receive datapath."]
    #[inline(always)]
    pub fn rxproduceix(&self) -> RxproduceixR {
        RxproduceixR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Receive produce index register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rxproduceindex::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxproduceindexSpec;
impl crate::RegisterSpec for RxproduceindexSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxproduceindex::R`](R) reader structure"]
impl crate::Readable for RxproduceindexSpec {}
#[doc = "`reset()` method sets RXPRODUCEINDEX to value 0"]
impl crate::Resettable for RxproduceindexSpec {
    const RESET_VALUE: u32 = 0;
}
