#[doc = "Register `TXPLEN` writer"]
pub type W = crate::W<TxplenSpec>;
#[doc = "Field `PKT_LNGTH` writer - The remaining number of bytes to be written to the selected endpoint buffer. This field is decremented by 4 by hardware after each write to USBTxData. When this field decrements to 0, the TxENDPKT bit will be set in USBDevIntSt."]
pub type PktLngthW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl W {
    #[doc = "Bits 0:9 - The remaining number of bytes to be written to the selected endpoint buffer. This field is decremented by 4 by hardware after each write to USBTxData. When this field decrements to 0, the TxENDPKT bit will be set in USBDevIntSt."]
    #[inline(always)]
    #[must_use]
    pub fn pkt_lngth(&mut self) -> PktLngthW<TxplenSpec> {
        PktLngthW::new(self, 0)
    }
}
#[doc = "USB Transmit Packet Length\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txplen::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxplenSpec;
impl crate::RegisterSpec for TxplenSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`txplen::W`](W) writer structure"]
impl crate::Writable for TxplenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXPLEN to value 0"]
impl crate::Resettable for TxplenSpec {
    const RESET_VALUE: u32 = 0;
}
