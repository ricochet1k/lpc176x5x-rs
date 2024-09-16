#[doc = "Register `DEVINTCLR` writer"]
pub type W = crate::W<DevintclrSpec>;
#[doc = "Field `FRAMECLR` writer - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is cleared."]
pub type FrameclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP_FASTCLR` writer - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is cleared."]
pub type EpFastclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP_SLOWCLR` writer - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is cleared."]
pub type EpSlowclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEV_STATCLR` writer - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is cleared."]
pub type DevStatclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCEMPTYCLR` writer - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is cleared."]
pub type CcemptyclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDFULLCLR` writer - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is cleared."]
pub type CdfullclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RxENDPKTCLR` writer - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is cleared."]
pub type RxEndpktclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TxENDPKTCLR` writer - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is cleared."]
pub type TxEndpktclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP_RLZEDCLR` writer - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is cleared."]
pub type EpRlzedclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR_INTCLR` writer - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is cleared."]
pub type ErrIntclrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn frameclr(&mut self) -> FrameclrW<DevintclrSpec> {
        FrameclrW::new(self, 0)
    }
    #[doc = "Bit 1 - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn ep_fastclr(&mut self) -> EpFastclrW<DevintclrSpec> {
        EpFastclrW::new(self, 1)
    }
    #[doc = "Bit 2 - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn ep_slowclr(&mut self) -> EpSlowclrW<DevintclrSpec> {
        EpSlowclrW::new(self, 2)
    }
    #[doc = "Bit 3 - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn dev_statclr(&mut self) -> DevStatclrW<DevintclrSpec> {
        DevStatclrW::new(self, 3)
    }
    #[doc = "Bit 4 - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn ccemptyclr(&mut self) -> CcemptyclrW<DevintclrSpec> {
        CcemptyclrW::new(self, 4)
    }
    #[doc = "Bit 5 - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn cdfullclr(&mut self) -> CdfullclrW<DevintclrSpec> {
        CdfullclrW::new(self, 5)
    }
    #[doc = "Bit 6 - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn rx_endpktclr(&mut self) -> RxEndpktclrW<DevintclrSpec> {
        RxEndpktclrW::new(self, 6)
    }
    #[doc = "Bit 7 - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn tx_endpktclr(&mut self) -> TxEndpktclrW<DevintclrSpec> {
        TxEndpktclrW::new(self, 7)
    }
    #[doc = "Bit 8 - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn ep_rlzedclr(&mut self) -> EpRlzedclrW<DevintclrSpec> {
        EpRlzedclrW::new(self, 8)
    }
    #[doc = "Bit 9 - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn err_intclr(&mut self) -> ErrIntclrW<DevintclrSpec> {
        ErrIntclrW::new(self, 9)
    }
}
#[doc = "USB Device Interrupt Clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devintclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevintclrSpec;
impl crate::RegisterSpec for DevintclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`devintclr::W`](W) writer structure"]
impl crate::Writable for DevintclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEVINTCLR to value 0"]
impl crate::Resettable for DevintclrSpec {
    const RESET_VALUE: u32 = 0;
}
