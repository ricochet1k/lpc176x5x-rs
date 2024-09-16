#[doc = "Register `MCR` reader"]
pub type R = crate::R<McrSpec>;
#[doc = "Register `MCR` writer"]
pub type W = crate::W<McrSpec>;
#[doc = "Field `DTRCTRL` reader - DTR Control. Source for modem output pin, DTR. This bit reads as 0 when modem loopback mode is active."]
pub type DtrctrlR = crate::BitReader;
#[doc = "Field `DTRCTRL` writer - DTR Control. Source for modem output pin, DTR. This bit reads as 0 when modem loopback mode is active."]
pub type DtrctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTSCTRL` reader - RTS Control. Source for modem output pin RTS. This bit reads as 0 when modem loopback mode is active."]
pub type RtsctrlR = crate::BitReader;
#[doc = "Field `RTSCTRL` writer - RTS Control. Source for modem output pin RTS. This bit reads as 0 when modem loopback mode is active."]
pub type RtsctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Loopback Mode Select. The modem loopback mode provides a mechanism to perform diagnostic loopback testing. Serial data from the transmitter is connected internally to serial input of the receiver. Input pin, RXD1, has no effect on loopback and output pin, TXD1 is held in marking state. The 4 modem inputs (CTS, DSR, RI and DCD) are disconnected externally. Externally, the modem outputs (RTS, DTR) are set inactive. Internally, the 4 modem outputs are connected to the 4 modem inputs. As a result of these connections, the upper 4 bits of the MSR will be driven by the lower 4 bits of the MCR rather than the 4 modem inputs in normal mode. This permits modem status interrupts to be generated in loopback mode by writing the lower 4 bits of MCR.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lms {
    #[doc = "0: Disable modem loopback mode."]
    DisableModemLoopba = 0,
    #[doc = "1: Enable modem loopback mode."]
    EnableModemLoopbac = 1,
}
impl From<Lms> for bool {
    #[inline(always)]
    fn from(variant: Lms) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LMS` reader - Loopback Mode Select. The modem loopback mode provides a mechanism to perform diagnostic loopback testing. Serial data from the transmitter is connected internally to serial input of the receiver. Input pin, RXD1, has no effect on loopback and output pin, TXD1 is held in marking state. The 4 modem inputs (CTS, DSR, RI and DCD) are disconnected externally. Externally, the modem outputs (RTS, DTR) are set inactive. Internally, the 4 modem outputs are connected to the 4 modem inputs. As a result of these connections, the upper 4 bits of the MSR will be driven by the lower 4 bits of the MCR rather than the 4 modem inputs in normal mode. This permits modem status interrupts to be generated in loopback mode by writing the lower 4 bits of MCR."]
pub type LmsR = crate::BitReader<Lms>;
impl LmsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lms {
        match self.bits {
            false => Lms::DisableModemLoopba,
            true => Lms::EnableModemLoopbac,
        }
    }
    #[doc = "Disable modem loopback mode."]
    #[inline(always)]
    pub fn is_disable_modem_loopba(&self) -> bool {
        *self == Lms::DisableModemLoopba
    }
    #[doc = "Enable modem loopback mode."]
    #[inline(always)]
    pub fn is_enable_modem_loopbac(&self) -> bool {
        *self == Lms::EnableModemLoopbac
    }
}
#[doc = "Field `LMS` writer - Loopback Mode Select. The modem loopback mode provides a mechanism to perform diagnostic loopback testing. Serial data from the transmitter is connected internally to serial input of the receiver. Input pin, RXD1, has no effect on loopback and output pin, TXD1 is held in marking state. The 4 modem inputs (CTS, DSR, RI and DCD) are disconnected externally. Externally, the modem outputs (RTS, DTR) are set inactive. Internally, the 4 modem outputs are connected to the 4 modem inputs. As a result of these connections, the upper 4 bits of the MSR will be driven by the lower 4 bits of the MCR rather than the 4 modem inputs in normal mode. This permits modem status interrupts to be generated in loopback mode by writing the lower 4 bits of MCR."]
pub type LmsW<'a, REG> = crate::BitWriter<'a, REG, Lms>;
impl<'a, REG> LmsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable modem loopback mode."]
    #[inline(always)]
    pub fn disable_modem_loopba(self) -> &'a mut crate::W<REG> {
        self.variant(Lms::DisableModemLoopba)
    }
    #[doc = "Enable modem loopback mode."]
    #[inline(always)]
    pub fn enable_modem_loopbac(self) -> &'a mut crate::W<REG> {
        self.variant(Lms::EnableModemLoopbac)
    }
}
#[doc = "RTS enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtsen {
    #[doc = "0: Disable auto-rts flow control."]
    DisableAutoRtsFlo = 0,
    #[doc = "1: Enable auto-rts flow control."]
    EnableAutoRtsFlow = 1,
}
impl From<Rtsen> for bool {
    #[inline(always)]
    fn from(variant: Rtsen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTSEN` reader - RTS enable."]
pub type RtsenR = crate::BitReader<Rtsen>;
impl RtsenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtsen {
        match self.bits {
            false => Rtsen::DisableAutoRtsFlo,
            true => Rtsen::EnableAutoRtsFlow,
        }
    }
    #[doc = "Disable auto-rts flow control."]
    #[inline(always)]
    pub fn is_disable_auto_rts_flo(&self) -> bool {
        *self == Rtsen::DisableAutoRtsFlo
    }
    #[doc = "Enable auto-rts flow control."]
    #[inline(always)]
    pub fn is_enable_auto_rts_flow(&self) -> bool {
        *self == Rtsen::EnableAutoRtsFlow
    }
}
#[doc = "Field `RTSEN` writer - RTS enable."]
pub type RtsenW<'a, REG> = crate::BitWriter<'a, REG, Rtsen>;
impl<'a, REG> RtsenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable auto-rts flow control."]
    #[inline(always)]
    pub fn disable_auto_rts_flo(self) -> &'a mut crate::W<REG> {
        self.variant(Rtsen::DisableAutoRtsFlo)
    }
    #[doc = "Enable auto-rts flow control."]
    #[inline(always)]
    pub fn enable_auto_rts_flow(self) -> &'a mut crate::W<REG> {
        self.variant(Rtsen::EnableAutoRtsFlow)
    }
}
#[doc = "CTS enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctsen {
    #[doc = "0: Disable auto-cts flow control."]
    DisableAutoCtsFlo = 0,
    #[doc = "1: Enable auto-cts flow control."]
    EnableAutoCtsFlow = 1,
}
impl From<Ctsen> for bool {
    #[inline(always)]
    fn from(variant: Ctsen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSEN` reader - CTS enable."]
pub type CtsenR = crate::BitReader<Ctsen>;
impl CtsenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctsen {
        match self.bits {
            false => Ctsen::DisableAutoCtsFlo,
            true => Ctsen::EnableAutoCtsFlow,
        }
    }
    #[doc = "Disable auto-cts flow control."]
    #[inline(always)]
    pub fn is_disable_auto_cts_flo(&self) -> bool {
        *self == Ctsen::DisableAutoCtsFlo
    }
    #[doc = "Enable auto-cts flow control."]
    #[inline(always)]
    pub fn is_enable_auto_cts_flow(&self) -> bool {
        *self == Ctsen::EnableAutoCtsFlow
    }
}
#[doc = "Field `CTSEN` writer - CTS enable."]
pub type CtsenW<'a, REG> = crate::BitWriter<'a, REG, Ctsen>;
impl<'a, REG> CtsenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable auto-cts flow control."]
    #[inline(always)]
    pub fn disable_auto_cts_flo(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsen::DisableAutoCtsFlo)
    }
    #[doc = "Enable auto-cts flow control."]
    #[inline(always)]
    pub fn enable_auto_cts_flow(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsen::EnableAutoCtsFlow)
    }
}
impl R {
    #[doc = "Bit 0 - DTR Control. Source for modem output pin, DTR. This bit reads as 0 when modem loopback mode is active."]
    #[inline(always)]
    pub fn dtrctrl(&self) -> DtrctrlR {
        DtrctrlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RTS Control. Source for modem output pin RTS. This bit reads as 0 when modem loopback mode is active."]
    #[inline(always)]
    pub fn rtsctrl(&self) -> RtsctrlR {
        RtsctrlR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Loopback Mode Select. The modem loopback mode provides a mechanism to perform diagnostic loopback testing. Serial data from the transmitter is connected internally to serial input of the receiver. Input pin, RXD1, has no effect on loopback and output pin, TXD1 is held in marking state. The 4 modem inputs (CTS, DSR, RI and DCD) are disconnected externally. Externally, the modem outputs (RTS, DTR) are set inactive. Internally, the 4 modem outputs are connected to the 4 modem inputs. As a result of these connections, the upper 4 bits of the MSR will be driven by the lower 4 bits of the MCR rather than the 4 modem inputs in normal mode. This permits modem status interrupts to be generated in loopback mode by writing the lower 4 bits of MCR."]
    #[inline(always)]
    pub fn lms(&self) -> LmsR {
        LmsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - RTS enable."]
    #[inline(always)]
    pub fn rtsen(&self) -> RtsenR {
        RtsenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CTS enable."]
    #[inline(always)]
    pub fn ctsen(&self) -> CtsenR {
        CtsenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DTR Control. Source for modem output pin, DTR. This bit reads as 0 when modem loopback mode is active."]
    #[inline(always)]
    #[must_use]
    pub fn dtrctrl(&mut self) -> DtrctrlW<McrSpec> {
        DtrctrlW::new(self, 0)
    }
    #[doc = "Bit 1 - RTS Control. Source for modem output pin RTS. This bit reads as 0 when modem loopback mode is active."]
    #[inline(always)]
    #[must_use]
    pub fn rtsctrl(&mut self) -> RtsctrlW<McrSpec> {
        RtsctrlW::new(self, 1)
    }
    #[doc = "Bit 4 - Loopback Mode Select. The modem loopback mode provides a mechanism to perform diagnostic loopback testing. Serial data from the transmitter is connected internally to serial input of the receiver. Input pin, RXD1, has no effect on loopback and output pin, TXD1 is held in marking state. The 4 modem inputs (CTS, DSR, RI and DCD) are disconnected externally. Externally, the modem outputs (RTS, DTR) are set inactive. Internally, the 4 modem outputs are connected to the 4 modem inputs. As a result of these connections, the upper 4 bits of the MSR will be driven by the lower 4 bits of the MCR rather than the 4 modem inputs in normal mode. This permits modem status interrupts to be generated in loopback mode by writing the lower 4 bits of MCR."]
    #[inline(always)]
    #[must_use]
    pub fn lms(&mut self) -> LmsW<McrSpec> {
        LmsW::new(self, 4)
    }
    #[doc = "Bit 6 - RTS enable."]
    #[inline(always)]
    #[must_use]
    pub fn rtsen(&mut self) -> RtsenW<McrSpec> {
        RtsenW::new(self, 6)
    }
    #[doc = "Bit 7 - CTS enable."]
    #[inline(always)]
    #[must_use]
    pub fn ctsen(&mut self) -> CtsenW<McrSpec> {
        CtsenW::new(self, 7)
    }
}
#[doc = "Modem Control Register. Contains controls for flow control handshaking and loopback mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`mcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McrSpec;
impl crate::RegisterSpec for McrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcr::R`](R) reader structure"]
impl crate::Readable for McrSpec {}
#[doc = "`write(|w| ..)` method takes [`mcr::W`](W) writer structure"]
impl crate::Writable for McrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCR to value 0"]
impl crate::Resettable for McrSpec {
    const RESET_VALUE: u32 = 0;
}
