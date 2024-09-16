#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved_0_dll: [u8; 0x04],
    _reserved_1_dlm: [u8; 0x04],
    _reserved_2_fcr: [u8; 0x04],
    lcr: Lcr,
    mcr: Mcr,
    lsr: Lsr,
    msr: Msr,
    scr: Scr,
    acr: Acr,
    _reserved9: [u8; 0x04],
    fdr: Fdr,
    _reserved10: [u8; 0x04],
    ter: Ter,
    _reserved11: [u8; 0x18],
    rs485ctrl: Rs485ctrl,
    rs485adrmatch: Rs485adrmatch,
    rs485dly: Rs485dly,
}
impl RegisterBlock {
    #[doc = "0x00 - DLAB =1. Divisor Latch LSB. Least significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider."]
    #[inline(always)]
    pub const fn dll(&self) -> &Dll {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    #[doc = "0x00 - DLAB =0. Transmit Holding Register. The next character to be transmitted is written here."]
    #[inline(always)]
    pub const fn thr(&self) -> &Thr {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    #[doc = "0x00 - DLAB =0 Receiver Buffer Register. Contains the next received character to be read."]
    #[inline(always)]
    pub const fn rbr(&self) -> &Rbr {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    #[doc = "0x04 - DLAB =0. Interrupt Enable Register. Contains individual interrupt enable bits for the 7 potential UART1 interrupts."]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        unsafe { &*(self as *const Self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x04 - DLAB =1. Divisor Latch MSB. Most significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider."]
    #[inline(always)]
    pub const fn dlm(&self) -> &Dlm {
        unsafe { &*(self as *const Self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x08 - FIFO Control Register. Controls UART1 FIFO usage and modes."]
    #[inline(always)]
    pub const fn fcr(&self) -> &Fcr {
        unsafe { &*(self as *const Self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x08 - Interrupt ID Register. Identifies which interrupt(s) are pending."]
    #[inline(always)]
    pub const fn iir(&self) -> &Iir {
        unsafe { &*(self as *const Self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x0c - Line Control Register. Contains controls for frame formatting and break generation."]
    #[inline(always)]
    pub const fn lcr(&self) -> &Lcr {
        &self.lcr
    }
    #[doc = "0x10 - Modem Control Register. Contains controls for flow control handshaking and loopback mode."]
    #[inline(always)]
    pub const fn mcr(&self) -> &Mcr {
        &self.mcr
    }
    #[doc = "0x14 - Line Status Register. Contains flags for transmit and receive status, including line errors."]
    #[inline(always)]
    pub const fn lsr(&self) -> &Lsr {
        &self.lsr
    }
    #[doc = "0x18 - Modem Status Register. Contains handshake signal status flags."]
    #[inline(always)]
    pub const fn msr(&self) -> &Msr {
        &self.msr
    }
    #[doc = "0x1c - Scratch Pad Register. 8-bit temporary storage for software."]
    #[inline(always)]
    pub const fn scr(&self) -> &Scr {
        &self.scr
    }
    #[doc = "0x20 - Auto-baud Control Register. Contains controls for the auto-baud feature."]
    #[inline(always)]
    pub const fn acr(&self) -> &Acr {
        &self.acr
    }
    #[doc = "0x28 - Fractional Divider Register. Generates a clock input for the baud rate divider."]
    #[inline(always)]
    pub const fn fdr(&self) -> &Fdr {
        &self.fdr
    }
    #[doc = "0x30 - Transmit Enable Register. Turns off UART transmitter for use with software flow control."]
    #[inline(always)]
    pub const fn ter(&self) -> &Ter {
        &self.ter
    }
    #[doc = "0x4c - RS-485/EIA-485 Control. Contains controls to configure various aspects of RS-485/EIA-485 modes."]
    #[inline(always)]
    pub const fn rs485ctrl(&self) -> &Rs485ctrl {
        &self.rs485ctrl
    }
    #[doc = "0x50 - RS-485/EIA-485 address match. Contains the address match value for RS-485/EIA-485 mode."]
    #[inline(always)]
    pub const fn rs485adrmatch(&self) -> &Rs485adrmatch {
        &self.rs485adrmatch
    }
    #[doc = "0x54 - RS-485/EIA-485 direction control delay."]
    #[inline(always)]
    pub const fn rs485dly(&self) -> &Rs485dly {
        &self.rs485dly
    }
}
#[doc = "RBR (r) register accessor: DLAB =0 Receiver Buffer Register. Contains the next received character to be read.\n\nYou can [`read`](crate::Reg::read) this register and get [`rbr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">The register is <b>modified</b> in some way after a read operation.</div>\n\nFor information about available fields see [`mod@rbr`]
module"]
#[doc(alias = "RBR")]
pub type Rbr = crate::Reg<rbr::RbrSpec>;
#[doc = "DLAB =0 Receiver Buffer Register. Contains the next received character to be read."]
pub mod rbr;
#[doc = "THR (w) register accessor: DLAB =0. Transmit Holding Register. The next character to be transmitted is written here.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`thr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@thr`]
module"]
#[doc(alias = "THR")]
pub type Thr = crate::Reg<thr::ThrSpec>;
#[doc = "DLAB =0. Transmit Holding Register. The next character to be transmitted is written here."]
pub mod thr;
#[doc = "DLL (rw) register accessor: DLAB =1. Divisor Latch LSB. Least significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider.\n\nYou can [`read`](crate::Reg::read) this register and get [`dll::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dll::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dll`]
module"]
#[doc(alias = "DLL")]
pub type Dll = crate::Reg<dll::DllSpec>;
#[doc = "DLAB =1. Divisor Latch LSB. Least significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider."]
pub mod dll;
#[doc = "DLM (rw) register accessor: DLAB =1. Divisor Latch MSB. Most significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider.\n\nYou can [`read`](crate::Reg::read) this register and get [`dlm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dlm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dlm`]
module"]
#[doc(alias = "DLM")]
pub type Dlm = crate::Reg<dlm::DlmSpec>;
#[doc = "DLAB =1. Divisor Latch MSB. Most significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider."]
pub mod dlm;
#[doc = "IER (rw) register accessor: DLAB =0. Interrupt Enable Register. Contains individual interrupt enable bits for the 7 potential UART1 interrupts.\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`]
module"]
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IerSpec>;
#[doc = "DLAB =0. Interrupt Enable Register. Contains individual interrupt enable bits for the 7 potential UART1 interrupts."]
pub mod ier;
#[doc = "IIR (r) register accessor: Interrupt ID Register. Identifies which interrupt(s) are pending.\n\nYou can [`read`](crate::Reg::read) this register and get [`iir::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iir`]
module"]
#[doc(alias = "IIR")]
pub type Iir = crate::Reg<iir::IirSpec>;
#[doc = "Interrupt ID Register. Identifies which interrupt(s) are pending."]
pub mod iir;
#[doc = "FCR (w) register accessor: FIFO Control Register. Controls UART1 FIFO usage and modes.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcr`]
module"]
#[doc(alias = "FCR")]
pub type Fcr = crate::Reg<fcr::FcrSpec>;
#[doc = "FIFO Control Register. Controls UART1 FIFO usage and modes."]
pub mod fcr;
#[doc = "LCR (rw) register accessor: Line Control Register. Contains controls for frame formatting and break generation.\n\nYou can [`read`](crate::Reg::read) this register and get [`lcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcr`]
module"]
#[doc(alias = "LCR")]
pub type Lcr = crate::Reg<lcr::LcrSpec>;
#[doc = "Line Control Register. Contains controls for frame formatting and break generation."]
pub mod lcr;
#[doc = "MCR (rw) register accessor: Modem Control Register. Contains controls for flow control handshaking and loopback mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`mcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcr`]
module"]
#[doc(alias = "MCR")]
pub type Mcr = crate::Reg<mcr::McrSpec>;
#[doc = "Modem Control Register. Contains controls for flow control handshaking and loopback mode."]
pub mod mcr;
#[doc = "LSR (r) register accessor: Line Status Register. Contains flags for transmit and receive status, including line errors.\n\nYou can [`read`](crate::Reg::read) this register and get [`lsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">The register is <b>modified</b> in some way after a read operation.</div>\n\nFor information about available fields see [`mod@lsr`]
module"]
#[doc(alias = "LSR")]
pub type Lsr = crate::Reg<lsr::LsrSpec>;
#[doc = "Line Status Register. Contains flags for transmit and receive status, including line errors."]
pub mod lsr;
#[doc = "MSR (r) register accessor: Modem Status Register. Contains handshake signal status flags.\n\nYou can [`read`](crate::Reg::read) this register and get [`msr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">The register is <b>modified</b> in some way after a read operation.</div>\n\nFor information about available fields see [`mod@msr`]
module"]
#[doc(alias = "MSR")]
pub type Msr = crate::Reg<msr::MsrSpec>;
#[doc = "Modem Status Register. Contains handshake signal status flags."]
pub mod msr;
#[doc = "SCR (rw) register accessor: Scratch Pad Register. 8-bit temporary storage for software.\n\nYou can [`read`](crate::Reg::read) this register and get [`scr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scr`]
module"]
#[doc(alias = "SCR")]
pub type Scr = crate::Reg<scr::ScrSpec>;
#[doc = "Scratch Pad Register. 8-bit temporary storage for software."]
pub mod scr;
#[doc = "ACR (rw) register accessor: Auto-baud Control Register. Contains controls for the auto-baud feature.\n\nYou can [`read`](crate::Reg::read) this register and get [`acr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acr`]
module"]
#[doc(alias = "ACR")]
pub type Acr = crate::Reg<acr::AcrSpec>;
#[doc = "Auto-baud Control Register. Contains controls for the auto-baud feature."]
pub mod acr;
#[doc = "FDR (rw) register accessor: Fractional Divider Register. Generates a clock input for the baud rate divider.\n\nYou can [`read`](crate::Reg::read) this register and get [`fdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdr`]
module"]
#[doc(alias = "FDR")]
pub type Fdr = crate::Reg<fdr::FdrSpec>;
#[doc = "Fractional Divider Register. Generates a clock input for the baud rate divider."]
pub mod fdr;
#[doc = "TER (rw) register accessor: Transmit Enable Register. Turns off UART transmitter for use with software flow control.\n\nYou can [`read`](crate::Reg::read) this register and get [`ter::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ter::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ter`]
module"]
#[doc(alias = "TER")]
pub type Ter = crate::Reg<ter::TerSpec>;
#[doc = "Transmit Enable Register. Turns off UART transmitter for use with software flow control."]
pub mod ter;
#[doc = "RS485CTRL (rw) register accessor: RS-485/EIA-485 Control. Contains controls to configure various aspects of RS-485/EIA-485 modes.\n\nYou can [`read`](crate::Reg::read) this register and get [`rs485ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rs485ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rs485ctrl`]
module"]
#[doc(alias = "RS485CTRL")]
pub type Rs485ctrl = crate::Reg<rs485ctrl::Rs485ctrlSpec>;
#[doc = "RS-485/EIA-485 Control. Contains controls to configure various aspects of RS-485/EIA-485 modes."]
pub mod rs485ctrl;
#[doc = "RS485ADRMATCH (rw) register accessor: RS-485/EIA-485 address match. Contains the address match value for RS-485/EIA-485 mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`rs485adrmatch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rs485adrmatch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rs485adrmatch`]
module"]
#[doc(alias = "RS485ADRMATCH")]
pub type Rs485adrmatch = crate::Reg<rs485adrmatch::Rs485adrmatchSpec>;
#[doc = "RS-485/EIA-485 address match. Contains the address match value for RS-485/EIA-485 mode."]
pub mod rs485adrmatch;
#[doc = "RS485DLY (rw) register accessor: RS-485/EIA-485 direction control delay.\n\nYou can [`read`](crate::Reg::read) this register and get [`rs485dly::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rs485dly::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rs485dly`]
module"]
#[doc(alias = "RS485DLY")]
pub type Rs485dly = crate::Reg<rs485dly::Rs485dlySpec>;
#[doc = "RS-485/EIA-485 direction control delay."]
pub mod rs485dly;
