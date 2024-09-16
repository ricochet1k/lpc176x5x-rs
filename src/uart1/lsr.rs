#[doc = "Register `LSR` reader"]
pub type R = crate::R<LsrSpec>;
#[doc = "Receiver Data Ready. LSR\\[0\\]
is set when the RBR holds an unread character and is cleared when the UART1 RBR FIFO is empty.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdr {
    #[doc = "0: The UART1 receiver FIFO is empty."]
    Empty = 0,
    #[doc = "1: The UART1 receiver FIFO is not empty."]
    Notempty = 1,
}
impl From<Rdr> for bool {
    #[inline(always)]
    fn from(variant: Rdr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDR` reader - Receiver Data Ready. LSR\\[0\\]
is set when the RBR holds an unread character and is cleared when the UART1 RBR FIFO is empty."]
pub type RdrR = crate::BitReader<Rdr>;
impl RdrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rdr {
        match self.bits {
            false => Rdr::Empty,
            true => Rdr::Notempty,
        }
    }
    #[doc = "The UART1 receiver FIFO is empty."]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == Rdr::Empty
    }
    #[doc = "The UART1 receiver FIFO is not empty."]
    #[inline(always)]
    pub fn is_notempty(&self) -> bool {
        *self == Rdr::Notempty
    }
}
#[doc = "Overrun Error. The overrun error condition is set as soon as it occurs. An LSR read clears LSR\\[1\\]. LSR\\[1\\]
is set when UART1 RSR has a new character assembled and the UART1 RBR FIFO is full. In this case, the UART1 RBR FIFO will not be overwritten and the character in the UART1 RSR will be lost.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oe {
    #[doc = "0: Overrun error status is inactive."]
    Inactive = 0,
    #[doc = "1: Overrun error status is active."]
    Active = 1,
}
impl From<Oe> for bool {
    #[inline(always)]
    fn from(variant: Oe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OE` reader - Overrun Error. The overrun error condition is set as soon as it occurs. An LSR read clears LSR\\[1\\]. LSR\\[1\\]
is set when UART1 RSR has a new character assembled and the UART1 RBR FIFO is full. In this case, the UART1 RBR FIFO will not be overwritten and the character in the UART1 RSR will be lost."]
pub type OeR = crate::BitReader<Oe>;
impl OeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oe {
        match self.bits {
            false => Oe::Inactive,
            true => Oe::Active,
        }
    }
    #[doc = "Overrun error status is inactive."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Oe::Inactive
    }
    #[doc = "Overrun error status is active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Oe::Active
    }
}
#[doc = "Parity Error. When the parity bit of a received character is in the wrong state, a parity error occurs. An LSR read clears LSR\\[2\\]. Time of parity error detection is dependent on FCR\\[0\\]. Note: A parity error is associated with the character at the top of the UART1 RBR FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pe {
    #[doc = "0: Parity error status is inactive."]
    Inactive = 0,
    #[doc = "1: Parity error status is active."]
    Active = 1,
}
impl From<Pe> for bool {
    #[inline(always)]
    fn from(variant: Pe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PE` reader - Parity Error. When the parity bit of a received character is in the wrong state, a parity error occurs. An LSR read clears LSR\\[2\\]. Time of parity error detection is dependent on FCR\\[0\\]. Note: A parity error is associated with the character at the top of the UART1 RBR FIFO."]
pub type PeR = crate::BitReader<Pe>;
impl PeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pe {
        match self.bits {
            false => Pe::Inactive,
            true => Pe::Active,
        }
    }
    #[doc = "Parity error status is inactive."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Pe::Inactive
    }
    #[doc = "Parity error status is active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Pe::Active
    }
}
#[doc = "Framing Error. When the stop bit of a received character is a logic 0, a framing error occurs. An LSR read clears LSR\\[3\\]. The time of the framing error detection is dependent on FCR0. Upon detection of a framing error, the RX will attempt to resynchronize to the data and assume that the bad stop bit is actually an early start bit. However, it cannot be assumed that the next received byte will be correct even if there is no Framing Error. Note: A framing error is associated with the character at the top of the UART1 RBR FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fe {
    #[doc = "0: Framing error status is inactive."]
    Inactive = 0,
    #[doc = "1: Framing error status is active."]
    Active = 1,
}
impl From<Fe> for bool {
    #[inline(always)]
    fn from(variant: Fe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FE` reader - Framing Error. When the stop bit of a received character is a logic 0, a framing error occurs. An LSR read clears LSR\\[3\\]. The time of the framing error detection is dependent on FCR0. Upon detection of a framing error, the RX will attempt to resynchronize to the data and assume that the bad stop bit is actually an early start bit. However, it cannot be assumed that the next received byte will be correct even if there is no Framing Error. Note: A framing error is associated with the character at the top of the UART1 RBR FIFO."]
pub type FeR = crate::BitReader<Fe>;
impl FeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fe {
        match self.bits {
            false => Fe::Inactive,
            true => Fe::Active,
        }
    }
    #[doc = "Framing error status is inactive."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Fe::Inactive
    }
    #[doc = "Framing error status is active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Fe::Active
    }
}
#[doc = "Break Interrupt. When RXD1 is held in the spacing state (all zeroes) for one full character transmission (start, data, parity, stop), a break interrupt occurs. Once the break condition has been detected, the receiver goes idle until RXD1 goes to marking state (all ones). An LSR read clears this status bit. The time of break detection is dependent on FCR\\[0\\]. Note: The break interrupt is associated with the character at the top of the UART1 RBR FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bi {
    #[doc = "0: Break interrupt status is inactive."]
    Inactive = 0,
    #[doc = "1: Break interrupt status is active."]
    Active = 1,
}
impl From<Bi> for bool {
    #[inline(always)]
    fn from(variant: Bi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BI` reader - Break Interrupt. When RXD1 is held in the spacing state (all zeroes) for one full character transmission (start, data, parity, stop), a break interrupt occurs. Once the break condition has been detected, the receiver goes idle until RXD1 goes to marking state (all ones). An LSR read clears this status bit. The time of break detection is dependent on FCR\\[0\\]. Note: The break interrupt is associated with the character at the top of the UART1 RBR FIFO."]
pub type BiR = crate::BitReader<Bi>;
impl BiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bi {
        match self.bits {
            false => Bi::Inactive,
            true => Bi::Active,
        }
    }
    #[doc = "Break interrupt status is inactive."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Bi::Inactive
    }
    #[doc = "Break interrupt status is active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Bi::Active
    }
}
#[doc = "Transmitter Holding Register Empty. THRE is set immediately upon detection of an empty UART1 THR and is cleared on a THR write.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Thre {
    #[doc = "0: THR contains valid data."]
    Valid = 0,
    #[doc = "1: THR is empty."]
    ThrIsEmpty_ = 1,
}
impl From<Thre> for bool {
    #[inline(always)]
    fn from(variant: Thre) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `THRE` reader - Transmitter Holding Register Empty. THRE is set immediately upon detection of an empty UART1 THR and is cleared on a THR write."]
pub type ThreR = crate::BitReader<Thre>;
impl ThreR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Thre {
        match self.bits {
            false => Thre::Valid,
            true => Thre::ThrIsEmpty_,
        }
    }
    #[doc = "THR contains valid data."]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == Thre::Valid
    }
    #[doc = "THR is empty."]
    #[inline(always)]
    pub fn is_thr_is_empty_(&self) -> bool {
        *self == Thre::ThrIsEmpty_
    }
}
#[doc = "Transmitter Empty. TEMT is set when both THR and TSR are empty; TEMT is cleared when either the TSR or the THR contain valid data.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Temt {
    #[doc = "0: THR and/or the TSR contains valid data."]
    Valid = 0,
    #[doc = "1: THR and the TSR are empty."]
    Empty = 1,
}
impl From<Temt> for bool {
    #[inline(always)]
    fn from(variant: Temt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEMT` reader - Transmitter Empty. TEMT is set when both THR and TSR are empty; TEMT is cleared when either the TSR or the THR contain valid data."]
pub type TemtR = crate::BitReader<Temt>;
impl TemtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Temt {
        match self.bits {
            false => Temt::Valid,
            true => Temt::Empty,
        }
    }
    #[doc = "THR and/or the TSR contains valid data."]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == Temt::Valid
    }
    #[doc = "THR and the TSR are empty."]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == Temt::Empty
    }
}
#[doc = "Error in RX FIFO. LSR\\[7\\]
is set when a character with a RX error such as framing error, parity error or break interrupt, is loaded into the RBR. This bit is cleared when the LSR register is read and there are no subsequent errors in the UART1 FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxfe {
    #[doc = "0: RBR contains no UART1 RX errors or FCR\\[0\\]=0."]
    Noerror = 0,
    #[doc = "1: UART1 RBR contains at least one UART1 RX error."]
    Errors = 1,
}
impl From<Rxfe> for bool {
    #[inline(always)]
    fn from(variant: Rxfe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFE` reader - Error in RX FIFO. LSR\\[7\\]
is set when a character with a RX error such as framing error, parity error or break interrupt, is loaded into the RBR. This bit is cleared when the LSR register is read and there are no subsequent errors in the UART1 FIFO."]
pub type RxfeR = crate::BitReader<Rxfe>;
impl RxfeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxfe {
        match self.bits {
            false => Rxfe::Noerror,
            true => Rxfe::Errors,
        }
    }
    #[doc = "RBR contains no UART1 RX errors or FCR\\[0\\]=0."]
    #[inline(always)]
    pub fn is_noerror(&self) -> bool {
        *self == Rxfe::Noerror
    }
    #[doc = "UART1 RBR contains at least one UART1 RX error."]
    #[inline(always)]
    pub fn is_errors(&self) -> bool {
        *self == Rxfe::Errors
    }
}
impl R {
    #[doc = "Bit 0 - Receiver Data Ready. LSR\\[0\\]
is set when the RBR holds an unread character and is cleared when the UART1 RBR FIFO is empty."]
    #[inline(always)]
    pub fn rdr(&self) -> RdrR {
        RdrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Overrun Error. The overrun error condition is set as soon as it occurs. An LSR read clears LSR\\[1\\]. LSR\\[1\\]
is set when UART1 RSR has a new character assembled and the UART1 RBR FIFO is full. In this case, the UART1 RBR FIFO will not be overwritten and the character in the UART1 RSR will be lost."]
    #[inline(always)]
    pub fn oe(&self) -> OeR {
        OeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Parity Error. When the parity bit of a received character is in the wrong state, a parity error occurs. An LSR read clears LSR\\[2\\]. Time of parity error detection is dependent on FCR\\[0\\]. Note: A parity error is associated with the character at the top of the UART1 RBR FIFO."]
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Framing Error. When the stop bit of a received character is a logic 0, a framing error occurs. An LSR read clears LSR\\[3\\]. The time of the framing error detection is dependent on FCR0. Upon detection of a framing error, the RX will attempt to resynchronize to the data and assume that the bad stop bit is actually an early start bit. However, it cannot be assumed that the next received byte will be correct even if there is no Framing Error. Note: A framing error is associated with the character at the top of the UART1 RBR FIFO."]
    #[inline(always)]
    pub fn fe(&self) -> FeR {
        FeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Break Interrupt. When RXD1 is held in the spacing state (all zeroes) for one full character transmission (start, data, parity, stop), a break interrupt occurs. Once the break condition has been detected, the receiver goes idle until RXD1 goes to marking state (all ones). An LSR read clears this status bit. The time of break detection is dependent on FCR\\[0\\]. Note: The break interrupt is associated with the character at the top of the UART1 RBR FIFO."]
    #[inline(always)]
    pub fn bi(&self) -> BiR {
        BiR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmitter Holding Register Empty. THRE is set immediately upon detection of an empty UART1 THR and is cleared on a THR write."]
    #[inline(always)]
    pub fn thre(&self) -> ThreR {
        ThreR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmitter Empty. TEMT is set when both THR and TSR are empty; TEMT is cleared when either the TSR or the THR contain valid data."]
    #[inline(always)]
    pub fn temt(&self) -> TemtR {
        TemtR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Error in RX FIFO. LSR\\[7\\]
is set when a character with a RX error such as framing error, parity error or break interrupt, is loaded into the RBR. This bit is cleared when the LSR register is read and there are no subsequent errors in the UART1 FIFO."]
    #[inline(always)]
    pub fn rxfe(&self) -> RxfeR {
        RxfeR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Line Status Register. Contains flags for transmit and receive status, including line errors.\n\nYou can [`read`](crate::Reg::read) this register and get [`lsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">The register is <b>modified</b> in some way after a read operation.</div>"]
pub struct LsrSpec;
impl crate::RegisterSpec for LsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lsr::R`](R) reader structure"]
impl crate::Readable for LsrSpec {}
#[doc = "`reset()` method sets LSR to value 0x60"]
impl crate::Resettable for LsrSpec {
    const RESET_VALUE: u32 = 0x60;
}
