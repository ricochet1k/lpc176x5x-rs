#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `RXSTATUS` reader - If 1, the receive channel is active. If 0, the receive channel is inactive."]
pub type RxstatusR = crate::BitReader;
#[doc = "Field `TXSTATUS` reader - If 1, the transmit channel is active. If 0, the transmit channel is inactive."]
pub type TxstatusR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - If 1, the receive channel is active. If 0, the receive channel is inactive."]
    #[inline(always)]
    pub fn rxstatus(&self) -> RxstatusR {
        RxstatusR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - If 1, the transmit channel is active. If 0, the transmit channel is inactive."]
    #[inline(always)]
    pub fn txstatus(&self) -> TxstatusR {
        TxstatusR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0;
}
