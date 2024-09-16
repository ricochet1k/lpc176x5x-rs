#[doc = "Register `FCR` writer"]
pub type W = crate::W<FcrSpec>;
#[doc = "FIFO Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fifoen {
    #[doc = "0: UARTn FIFOs are disabled. Must not be used in the application."]
    UartnFifosAreDisa = 0,
    #[doc = "1: Active high enable for both UARTn Rx and TX FIFOs and UnFCR\\[7:1\\]
access. This bit must be set for proper UART operation. Any transition on this bit will automatically clear the related UART FIFOs."]
    ActiveHighEnableF = 1,
}
impl From<Fifoen> for bool {
    #[inline(always)]
    fn from(variant: Fifoen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFOEN` writer - FIFO Enable."]
pub type FifoenW<'a, REG> = crate::BitWriter<'a, REG, Fifoen>;
impl<'a, REG> FifoenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "UARTn FIFOs are disabled. Must not be used in the application."]
    #[inline(always)]
    pub fn uartn_fifos_are_disa(self) -> &'a mut crate::W<REG> {
        self.variant(Fifoen::UartnFifosAreDisa)
    }
    #[doc = "Active high enable for both UARTn Rx and TX FIFOs and UnFCR\\[7:1\\]
access. This bit must be set for proper UART operation. Any transition on this bit will automatically clear the related UART FIFOs."]
    #[inline(always)]
    pub fn active_high_enable_f(self) -> &'a mut crate::W<REG> {
        self.variant(Fifoen::ActiveHighEnableF)
    }
}
#[doc = "RX FIFO Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxfifores {
    #[doc = "0: No impact on either of UARTn FIFOs."]
    NoImpactOnEither_ = 0,
    #[doc = "1: Writing a logic 1 to UnFCR\\[1\\]
will clear all bytes in UARTn Rx FIFO, reset the pointer logic. This bit is self-clearing."]
    WritingALogic1To = 1,
}
impl From<Rxfifores> for bool {
    #[inline(always)]
    fn from(variant: Rxfifores) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFIFORES` writer - RX FIFO Reset."]
pub type RxfiforesW<'a, REG> = crate::BitWriter<'a, REG, Rxfifores>;
impl<'a, REG> RxfiforesW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No impact on either of UARTn FIFOs."]
    #[inline(always)]
    pub fn no_impact_on_either_(self) -> &'a mut crate::W<REG> {
        self.variant(Rxfifores::NoImpactOnEither_)
    }
    #[doc = "Writing a logic 1 to UnFCR\\[1\\]
will clear all bytes in UARTn Rx FIFO, reset the pointer logic. This bit is self-clearing."]
    #[inline(always)]
    pub fn writing_a_logic_1_to(self) -> &'a mut crate::W<REG> {
        self.variant(Rxfifores::WritingALogic1To)
    }
}
#[doc = "TX FIFO Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txfifores {
    #[doc = "0: No impact on either of UARTn FIFOs."]
    NoImpactOnEither_ = 0,
    #[doc = "1: Writing a logic 1 to UnFCR\\[2\\]
will clear all bytes in UARTn TX FIFO, reset the pointer logic. This bit is self-clearing."]
    WritingALogic1To = 1,
}
impl From<Txfifores> for bool {
    #[inline(always)]
    fn from(variant: Txfifores) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFIFORES` writer - TX FIFO Reset."]
pub type TxfiforesW<'a, REG> = crate::BitWriter<'a, REG, Txfifores>;
impl<'a, REG> TxfiforesW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No impact on either of UARTn FIFOs."]
    #[inline(always)]
    pub fn no_impact_on_either_(self) -> &'a mut crate::W<REG> {
        self.variant(Txfifores::NoImpactOnEither_)
    }
    #[doc = "Writing a logic 1 to UnFCR\\[2\\]
will clear all bytes in UARTn TX FIFO, reset the pointer logic. This bit is self-clearing."]
    #[inline(always)]
    pub fn writing_a_logic_1_to(self) -> &'a mut crate::W<REG> {
        self.variant(Txfifores::WritingALogic1To)
    }
}
#[doc = "Field `DMAMODE` writer - DMA Mode Select. When the FIFO enable (bit 0 of this register) is set, this bit selects the DMA mode. See Section 18.6.6.1."]
pub type DmamodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "RX Trigger Level. These two bits determine how many receiver UARTn FIFO characters must be written before an interrupt or DMA request is activated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rxtriglvl {
    #[doc = "0: Trigger level 0 (1 character or 0x01)."]
    TriggerLevel0_1C = 0,
    #[doc = "1: Trigger level 1 (4 characters or 0x04)."]
    TriggerLevel1_4C = 1,
    #[doc = "2: Trigger level 2 (8 characters or 0x08)."]
    TriggerLevel2_8C = 2,
    #[doc = "3: Trigger level 3 (14 characters or 0x0E)."]
    TriggerLevel3_14_ = 3,
}
impl From<Rxtriglvl> for u8 {
    #[inline(always)]
    fn from(variant: Rxtriglvl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rxtriglvl {
    type Ux = u8;
}
impl crate::IsEnum for Rxtriglvl {}
#[doc = "Field `RXTRIGLVL` writer - RX Trigger Level. These two bits determine how many receiver UARTn FIFO characters must be written before an interrupt or DMA request is activated."]
pub type RxtriglvlW<'a, REG> = crate::FieldWriter<'a, REG, 2, Rxtriglvl, crate::Safe>;
impl<'a, REG> RxtriglvlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Trigger level 0 (1 character or 0x01)."]
    #[inline(always)]
    pub fn trigger_level_0_1_c(self) -> &'a mut crate::W<REG> {
        self.variant(Rxtriglvl::TriggerLevel0_1C)
    }
    #[doc = "Trigger level 1 (4 characters or 0x04)."]
    #[inline(always)]
    pub fn trigger_level_1_4_c(self) -> &'a mut crate::W<REG> {
        self.variant(Rxtriglvl::TriggerLevel1_4C)
    }
    #[doc = "Trigger level 2 (8 characters or 0x08)."]
    #[inline(always)]
    pub fn trigger_level_2_8_c(self) -> &'a mut crate::W<REG> {
        self.variant(Rxtriglvl::TriggerLevel2_8C)
    }
    #[doc = "Trigger level 3 (14 characters or 0x0E)."]
    #[inline(always)]
    pub fn trigger_level_3_14_(self) -> &'a mut crate::W<REG> {
        self.variant(Rxtriglvl::TriggerLevel3_14_)
    }
}
impl W {
    #[doc = "Bit 0 - FIFO Enable."]
    #[inline(always)]
    #[must_use]
    pub fn fifoen(&mut self) -> FifoenW<FcrSpec> {
        FifoenW::new(self, 0)
    }
    #[doc = "Bit 1 - RX FIFO Reset."]
    #[inline(always)]
    #[must_use]
    pub fn rxfifores(&mut self) -> RxfiforesW<FcrSpec> {
        RxfiforesW::new(self, 1)
    }
    #[doc = "Bit 2 - TX FIFO Reset."]
    #[inline(always)]
    #[must_use]
    pub fn txfifores(&mut self) -> TxfiforesW<FcrSpec> {
        TxfiforesW::new(self, 2)
    }
    #[doc = "Bit 3 - DMA Mode Select. When the FIFO enable (bit 0 of this register) is set, this bit selects the DMA mode. See Section 18.6.6.1."]
    #[inline(always)]
    #[must_use]
    pub fn dmamode(&mut self) -> DmamodeW<FcrSpec> {
        DmamodeW::new(self, 3)
    }
    #[doc = "Bits 6:7 - RX Trigger Level. These two bits determine how many receiver UARTn FIFO characters must be written before an interrupt or DMA request is activated."]
    #[inline(always)]
    #[must_use]
    pub fn rxtriglvl(&mut self) -> RxtriglvlW<FcrSpec> {
        RxtriglvlW::new(self, 6)
    }
}
#[doc = "FIFO Control Register. Controls UART FIFO usage and modes.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcrSpec;
impl crate::RegisterSpec for FcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`fcr::W`](W) writer structure"]
impl crate::Writable for FcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FCR to value 0"]
impl crate::Resettable for FcrSpec {
    const RESET_VALUE: u32 = 0;
}
