#[doc = "Register `DEVINTEN` reader"]
pub type R = crate::R<DevintenSpec>;
#[doc = "Register `DEVINTEN` writer"]
pub type W = crate::W<DevintenSpec>;
#[doc = "Field `FRAMEEN` reader - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
pub type FrameenR = crate::BitReader;
#[doc = "Field `FRAMEEN` writer - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
pub type FrameenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP_FASTEN` reader - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
pub type EpFastenR = crate::BitReader;
#[doc = "Field `EP_FASTEN` writer - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
pub type EpFastenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP_SLOWEN` reader - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
pub type EpSlowenR = crate::BitReader;
#[doc = "Field `EP_SLOWEN` writer - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
pub type EpSlowenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEV_STATEN` reader - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
pub type DevStatenR = crate::BitReader;
#[doc = "Field `DEV_STATEN` writer - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
pub type DevStatenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCEMPTYEN` reader - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
pub type CcemptyenR = crate::BitReader;
#[doc = "Field `CCEMPTYEN` writer - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
pub type CcemptyenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDFULLEN` reader - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
pub type CdfullenR = crate::BitReader;
#[doc = "Field `CDFULLEN` writer - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
pub type CdfullenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RxENDPKTEN` reader - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
pub type RxEndpktenR = crate::BitReader;
#[doc = "Field `RxENDPKTEN` writer - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
pub type RxEndpktenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TxENDPKTEN` reader - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
pub type TxEndpktenR = crate::BitReader;
#[doc = "Field `TxENDPKTEN` writer - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
pub type TxEndpktenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP_RLZEDEN` reader - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
pub type EpRlzedenR = crate::BitReader;
#[doc = "Field `EP_RLZEDEN` writer - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
pub type EpRlzedenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR_INTEN` reader - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
pub type ErrIntenR = crate::BitReader;
#[doc = "Field `ERR_INTEN` writer - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
pub type ErrIntenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
    #[inline(always)]
    pub fn frameen(&self) -> FrameenR {
        FrameenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
    #[inline(always)]
    pub fn ep_fasten(&self) -> EpFastenR {
        EpFastenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
    #[inline(always)]
    pub fn ep_slowen(&self) -> EpSlowenR {
        EpSlowenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
    #[inline(always)]
    pub fn dev_staten(&self) -> DevStatenR {
        DevStatenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
    #[inline(always)]
    pub fn ccemptyen(&self) -> CcemptyenR {
        CcemptyenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
    #[inline(always)]
    pub fn cdfullen(&self) -> CdfullenR {
        CdfullenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
    #[inline(always)]
    pub fn rx_endpkten(&self) -> RxEndpktenR {
        RxEndpktenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
    #[inline(always)]
    pub fn tx_endpkten(&self) -> TxEndpktenR {
        TxEndpktenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
    #[inline(always)]
    pub fn ep_rlzeden(&self) -> EpRlzedenR {
        EpRlzedenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
    #[inline(always)]
    pub fn err_inten(&self) -> ErrIntenR {
        ErrIntenR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
    #[inline(always)]
    #[must_use]
    pub fn frameen(&mut self) -> FrameenW<DevintenSpec> {
        FrameenW::new(self, 0)
    }
    #[doc = "Bit 1 - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
    #[inline(always)]
    #[must_use]
    pub fn ep_fasten(&mut self) -> EpFastenW<DevintenSpec> {
        EpFastenW::new(self, 1)
    }
    #[doc = "Bit 2 - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
    #[inline(always)]
    #[must_use]
    pub fn ep_slowen(&mut self) -> EpSlowenW<DevintenSpec> {
        EpSlowenW::new(self, 2)
    }
    #[doc = "Bit 3 - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
    #[inline(always)]
    #[must_use]
    pub fn dev_staten(&mut self) -> DevStatenW<DevintenSpec> {
        DevStatenW::new(self, 3)
    }
    #[doc = "Bit 4 - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
    #[inline(always)]
    #[must_use]
    pub fn ccemptyen(&mut self) -> CcemptyenW<DevintenSpec> {
        CcemptyenW::new(self, 4)
    }
    #[doc = "Bit 5 - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
    #[inline(always)]
    #[must_use]
    pub fn cdfullen(&mut self) -> CdfullenW<DevintenSpec> {
        CdfullenW::new(self, 5)
    }
    #[doc = "Bit 6 - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
    #[inline(always)]
    #[must_use]
    pub fn rx_endpkten(&mut self) -> RxEndpktenW<DevintenSpec> {
        RxEndpktenW::new(self, 6)
    }
    #[doc = "Bit 7 - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
    #[inline(always)]
    #[must_use]
    pub fn tx_endpkten(&mut self) -> TxEndpktenW<DevintenSpec> {
        TxEndpktenW::new(self, 7)
    }
    #[doc = "Bit 8 - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
    #[inline(always)]
    #[must_use]
    pub fn ep_rlzeden(&mut self) -> EpRlzedenW<DevintenSpec> {
        EpRlzedenW::new(self, 8)
    }
    #[doc = "Bit 9 - 0 = No interrupt is generated. 1 = An interrupt will be generated when the corresponding bit in the Device Interrupt Status (USBDevIntSt) register (Table 261) is set. By default, the interrupt is routed to the USB_INT_REQ_LP interrupt line. Optionally, either the EP_FAST or FRAME interrupt may be routed to the USB_INT_REQ_HP interrupt line by changing the value of USBDevIntPri."]
    #[inline(always)]
    #[must_use]
    pub fn err_inten(&mut self) -> ErrIntenW<DevintenSpec> {
        ErrIntenW::new(self, 9)
    }
}
#[doc = "USB Device Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`devinten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devinten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevintenSpec;
impl crate::RegisterSpec for DevintenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devinten::R`](R) reader structure"]
impl crate::Readable for DevintenSpec {}
#[doc = "`write(|w| ..)` method takes [`devinten::W`](W) writer structure"]
impl crate::Writable for DevintenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEVINTEN to value 0"]
impl crate::Resettable for DevintenSpec {
    const RESET_VALUE: u32 = 0;
}
