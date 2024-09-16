#[doc = "Register `TXPRODUCEINDEX` reader"]
pub type R = crate::R<TxproduceindexSpec>;
#[doc = "Register `TXPRODUCEINDEX` writer"]
pub type W = crate::W<TxproduceindexSpec>;
#[doc = "Field `TXPI` reader - TxProduceIndex. Index of the descriptor that is going to be filled next by the transmit software driver."]
pub type TxpiR = crate::FieldReader<u16>;
#[doc = "Field `TXPI` writer - TxProduceIndex. Index of the descriptor that is going to be filled next by the transmit software driver."]
pub type TxpiW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - TxProduceIndex. Index of the descriptor that is going to be filled next by the transmit software driver."]
    #[inline(always)]
    pub fn txpi(&self) -> TxpiR {
        TxpiR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - TxProduceIndex. Index of the descriptor that is going to be filled next by the transmit software driver."]
    #[inline(always)]
    #[must_use]
    pub fn txpi(&mut self) -> TxpiW<TxproduceindexSpec> {
        TxpiW::new(self, 0)
    }
}
#[doc = "Transmit produce index register.\n\nYou can [`read`](crate::Reg::read) this register and get [`txproduceindex::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txproduceindex::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxproduceindexSpec;
impl crate::RegisterSpec for TxproduceindexSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txproduceindex::R`](R) reader structure"]
impl crate::Readable for TxproduceindexSpec {}
#[doc = "`write(|w| ..)` method takes [`txproduceindex::W`](W) writer structure"]
impl crate::Writable for TxproduceindexSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXPRODUCEINDEX to value 0"]
impl crate::Resettable for TxproduceindexSpec {
    const RESET_VALUE: u32 = 0;
}
