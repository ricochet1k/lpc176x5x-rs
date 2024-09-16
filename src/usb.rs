#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0xdc],
    rxplen: Rxplen,
    _reserved1: [u8; 0x20],
    intst: Intst,
    inten: Inten,
    intset: Intset,
    intclr: Intclr,
    stctrl: Stctrl,
    tmr: Tmr,
    _reserved7: [u8; 0xe8],
    devintst: Devintst,
    devinten: Devinten,
    devintclr: Devintclr,
    devintset: Devintset,
    cmdcode: Cmdcode,
    cmddata: Cmddata,
    rxdata: Rxdata,
    txdata: Txdata,
    _reserved15: [u8; 0x04],
    txplen: Txplen,
    ctrl: Ctrl,
    devintpri: Devintpri,
    epintst: Epintst,
    epinten: Epinten,
    epintclr: Epintclr,
    epintset: Epintset,
    epintpri: Epintpri,
    reep: Reep,
    epind: Epind,
    maxpsize: Maxpsize,
    dmarst: Dmarst,
    dmarclr: Dmarclr,
    dmarset: Dmarset,
    _reserved29: [u8; 0x24],
    udcah: Udcah,
    epdmast: Epdmast,
    epdmaen: Epdmaen,
    epdmadis: Epdmadis,
    dmaintst: Dmaintst,
    dmainten: Dmainten,
    _reserved35: [u8; 0x08],
    eotintst: Eotintst,
    eotintclr: Eotintclr,
    eotintset: Eotintset,
    nddrintst: Nddrintst,
    nddrintclr: Nddrintclr,
    nddrintset: Nddrintset,
    syserrintst: Syserrintst,
    syserrintclr: Syserrintclr,
    syserrintset: Syserrintset,
    _reserved44: [u8; 0x3c],
    _reserved_44_i2c: [u8; 0x04],
    i2c_sts: I2cSts,
    i2c_ctl: I2cCtl,
    i2c_clkhi: I2cClkhi,
    i2c_clklo: I2cClklo,
    _reserved49: [u8; 0x0ce0],
    _reserved_49_otgclkctrl: [u8; 0x04],
    _reserved_50_otgclkst: [u8; 0x04],
}
impl RegisterBlock {
    #[doc = "0xdc - USB Receive Packet Length"]
    #[inline(always)]
    pub const fn rxplen(&self) -> &Rxplen {
        &self.rxplen
    }
    #[doc = "0x100 - OTG Interrupt Status"]
    #[inline(always)]
    pub const fn intst(&self) -> &Intst {
        &self.intst
    }
    #[doc = "0x104 - OTG Interrupt Enable"]
    #[inline(always)]
    pub const fn inten(&self) -> &Inten {
        &self.inten
    }
    #[doc = "0x108 - OTG Interrupt Set"]
    #[inline(always)]
    pub const fn intset(&self) -> &Intset {
        &self.intset
    }
    #[doc = "0x10c - OTG Interrupt Clear"]
    #[inline(always)]
    pub const fn intclr(&self) -> &Intclr {
        &self.intclr
    }
    #[doc = "0x110 - OTG Status and Control and USB port select"]
    #[inline(always)]
    pub const fn stctrl(&self) -> &Stctrl {
        &self.stctrl
    }
    #[doc = "0x114 - OTG Timer"]
    #[inline(always)]
    pub const fn tmr(&self) -> &Tmr {
        &self.tmr
    }
    #[doc = "0x200 - USB Device Interrupt Status"]
    #[inline(always)]
    pub const fn devintst(&self) -> &Devintst {
        &self.devintst
    }
    #[doc = "0x204 - USB Device Interrupt Enable"]
    #[inline(always)]
    pub const fn devinten(&self) -> &Devinten {
        &self.devinten
    }
    #[doc = "0x208 - USB Device Interrupt Clear"]
    #[inline(always)]
    pub const fn devintclr(&self) -> &Devintclr {
        &self.devintclr
    }
    #[doc = "0x20c - USB Device Interrupt Set"]
    #[inline(always)]
    pub const fn devintset(&self) -> &Devintset {
        &self.devintset
    }
    #[doc = "0x210 - USB Command Code"]
    #[inline(always)]
    pub const fn cmdcode(&self) -> &Cmdcode {
        &self.cmdcode
    }
    #[doc = "0x214 - USB Command Data"]
    #[inline(always)]
    pub const fn cmddata(&self) -> &Cmddata {
        &self.cmddata
    }
    #[doc = "0x218 - USB Receive Data"]
    #[inline(always)]
    pub const fn rxdata(&self) -> &Rxdata {
        &self.rxdata
    }
    #[doc = "0x21c - USB Transmit Data"]
    #[inline(always)]
    pub const fn txdata(&self) -> &Txdata {
        &self.txdata
    }
    #[doc = "0x224 - USB Transmit Packet Length"]
    #[inline(always)]
    pub const fn txplen(&self) -> &Txplen {
        &self.txplen
    }
    #[doc = "0x228 - USB Control"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x22c - USB Device Interrupt Priority"]
    #[inline(always)]
    pub const fn devintpri(&self) -> &Devintpri {
        &self.devintpri
    }
    #[doc = "0x230 - USB Endpoint Interrupt Status"]
    #[inline(always)]
    pub const fn epintst(&self) -> &Epintst {
        &self.epintst
    }
    #[doc = "0x234 - USB Endpoint Interrupt Enable"]
    #[inline(always)]
    pub const fn epinten(&self) -> &Epinten {
        &self.epinten
    }
    #[doc = "0x238 - USB Endpoint Interrupt Clear"]
    #[inline(always)]
    pub const fn epintclr(&self) -> &Epintclr {
        &self.epintclr
    }
    #[doc = "0x23c - USB Endpoint Interrupt Set"]
    #[inline(always)]
    pub const fn epintset(&self) -> &Epintset {
        &self.epintset
    }
    #[doc = "0x240 - USB Endpoint Priority"]
    #[inline(always)]
    pub const fn epintpri(&self) -> &Epintpri {
        &self.epintpri
    }
    #[doc = "0x244 - USB Realize Endpoint"]
    #[inline(always)]
    pub const fn reep(&self) -> &Reep {
        &self.reep
    }
    #[doc = "0x248 - USB Endpoint Index"]
    #[inline(always)]
    pub const fn epind(&self) -> &Epind {
        &self.epind
    }
    #[doc = "0x24c - USB MaxPacketSize"]
    #[inline(always)]
    pub const fn maxpsize(&self) -> &Maxpsize {
        &self.maxpsize
    }
    #[doc = "0x250 - USB DMA Request Status"]
    #[inline(always)]
    pub const fn dmarst(&self) -> &Dmarst {
        &self.dmarst
    }
    #[doc = "0x254 - USB DMA Request Clear"]
    #[inline(always)]
    pub const fn dmarclr(&self) -> &Dmarclr {
        &self.dmarclr
    }
    #[doc = "0x258 - USB DMA Request Set"]
    #[inline(always)]
    pub const fn dmarset(&self) -> &Dmarset {
        &self.dmarset
    }
    #[doc = "0x280 - USB UDCA Head"]
    #[inline(always)]
    pub const fn udcah(&self) -> &Udcah {
        &self.udcah
    }
    #[doc = "0x284 - USB Endpoint DMA Status"]
    #[inline(always)]
    pub const fn epdmast(&self) -> &Epdmast {
        &self.epdmast
    }
    #[doc = "0x288 - USB Endpoint DMA Enable"]
    #[inline(always)]
    pub const fn epdmaen(&self) -> &Epdmaen {
        &self.epdmaen
    }
    #[doc = "0x28c - USB Endpoint DMA Disable"]
    #[inline(always)]
    pub const fn epdmadis(&self) -> &Epdmadis {
        &self.epdmadis
    }
    #[doc = "0x290 - USB DMA Interrupt Status"]
    #[inline(always)]
    pub const fn dmaintst(&self) -> &Dmaintst {
        &self.dmaintst
    }
    #[doc = "0x294 - USB DMA Interrupt Enable"]
    #[inline(always)]
    pub const fn dmainten(&self) -> &Dmainten {
        &self.dmainten
    }
    #[doc = "0x2a0 - USB End of Transfer Interrupt Status"]
    #[inline(always)]
    pub const fn eotintst(&self) -> &Eotintst {
        &self.eotintst
    }
    #[doc = "0x2a4 - USB End of Transfer Interrupt Clear"]
    #[inline(always)]
    pub const fn eotintclr(&self) -> &Eotintclr {
        &self.eotintclr
    }
    #[doc = "0x2a8 - USB End of Transfer Interrupt Set"]
    #[inline(always)]
    pub const fn eotintset(&self) -> &Eotintset {
        &self.eotintset
    }
    #[doc = "0x2ac - USB New DD Request Interrupt Status"]
    #[inline(always)]
    pub const fn nddrintst(&self) -> &Nddrintst {
        &self.nddrintst
    }
    #[doc = "0x2b0 - USB New DD Request Interrupt Clear"]
    #[inline(always)]
    pub const fn nddrintclr(&self) -> &Nddrintclr {
        &self.nddrintclr
    }
    #[doc = "0x2b4 - USB New DD Request Interrupt Set"]
    #[inline(always)]
    pub const fn nddrintset(&self) -> &Nddrintset {
        &self.nddrintset
    }
    #[doc = "0x2b8 - USB System Error Interrupt Status"]
    #[inline(always)]
    pub const fn syserrintst(&self) -> &Syserrintst {
        &self.syserrintst
    }
    #[doc = "0x2bc - USB System Error Interrupt Clear"]
    #[inline(always)]
    pub const fn syserrintclr(&self) -> &Syserrintclr {
        &self.syserrintclr
    }
    #[doc = "0x2c0 - USB System Error Interrupt Set"]
    #[inline(always)]
    pub const fn syserrintset(&self) -> &Syserrintset {
        &self.syserrintset
    }
    #[doc = "0x300 - I2C Transmit"]
    #[inline(always)]
    pub const fn i2c_wo(&self) -> &I2cWo {
        unsafe { &*(self as *const Self).cast::<u8>().add(768).cast() }
    }
    #[doc = "0x300 - I2C Receive"]
    #[inline(always)]
    pub const fn i2c_rx(&self) -> &I2cRx {
        unsafe { &*(self as *const Self).cast::<u8>().add(768).cast() }
    }
    #[doc = "0x304 - I2C Status"]
    #[inline(always)]
    pub const fn i2c_sts(&self) -> &I2cSts {
        &self.i2c_sts
    }
    #[doc = "0x308 - I2C Control"]
    #[inline(always)]
    pub const fn i2c_ctl(&self) -> &I2cCtl {
        &self.i2c_ctl
    }
    #[doc = "0x30c - I2C Clock High"]
    #[inline(always)]
    pub const fn i2c_clkhi(&self) -> &I2cClkhi {
        &self.i2c_clkhi
    }
    #[doc = "0x310 - I2C Clock Low"]
    #[inline(always)]
    pub const fn i2c_clklo(&self) -> &I2cClklo {
        &self.i2c_clklo
    }
    #[doc = "0xff4 - OTG clock controller"]
    #[inline(always)]
    pub const fn otgclkctrl(&self) -> &Otgclkctrl {
        unsafe { &*(self as *const Self).cast::<u8>().add(4084).cast() }
    }
    #[doc = "0xff4 - USB Clock Control"]
    #[inline(always)]
    pub const fn usbclkctrl(&self) -> &Usbclkctrl {
        unsafe { &*(self as *const Self).cast::<u8>().add(4084).cast() }
    }
    #[doc = "0xff8 - OTG clock status"]
    #[inline(always)]
    pub const fn otgclkst(&self) -> &Otgclkst {
        unsafe { &*(self as *const Self).cast::<u8>().add(4088).cast() }
    }
    #[doc = "0xff8 - USB Clock Status"]
    #[inline(always)]
    pub const fn usbclkst(&self) -> &Usbclkst {
        unsafe { &*(self as *const Self).cast::<u8>().add(4088).cast() }
    }
}
#[doc = "INTST (r) register accessor: OTG Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`intst::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intst`]
module"]
#[doc(alias = "INTST")]
pub type Intst = crate::Reg<intst::IntstSpec>;
#[doc = "OTG Interrupt Status"]
pub mod intst;
#[doc = "INTEN (rw) register accessor: OTG Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`]
module"]
#[doc(alias = "INTEN")]
pub type Inten = crate::Reg<inten::IntenSpec>;
#[doc = "OTG Interrupt Enable"]
pub mod inten;
#[doc = "INTSET (w) register accessor: OTG Interrupt Set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intset`]
module"]
#[doc(alias = "INTSET")]
pub type Intset = crate::Reg<intset::IntsetSpec>;
#[doc = "OTG Interrupt Set"]
pub mod intset;
#[doc = "INTCLR (w) register accessor: OTG Interrupt Clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intclr`]
module"]
#[doc(alias = "INTCLR")]
pub type Intclr = crate::Reg<intclr::IntclrSpec>;
#[doc = "OTG Interrupt Clear"]
pub mod intclr;
#[doc = "STCTRL (rw) register accessor: OTG Status and Control and USB port select\n\nYou can [`read`](crate::Reg::read) this register and get [`stctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stctrl`]
module"]
#[doc(alias = "STCTRL")]
pub type Stctrl = crate::Reg<stctrl::StctrlSpec>;
#[doc = "OTG Status and Control and USB port select"]
pub mod stctrl;
#[doc = "TMR (rw) register accessor: OTG Timer\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr`]
module"]
#[doc(alias = "TMR")]
pub type Tmr = crate::Reg<tmr::TmrSpec>;
#[doc = "OTG Timer"]
pub mod tmr;
#[doc = "DEVINTST (r) register accessor: USB Device Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`devintst::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devintst`]
module"]
#[doc(alias = "DEVINTST")]
pub type Devintst = crate::Reg<devintst::DevintstSpec>;
#[doc = "USB Device Interrupt Status"]
pub mod devintst;
#[doc = "DEVINTEN (rw) register accessor: USB Device Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`devinten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devinten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devinten`]
module"]
#[doc(alias = "DEVINTEN")]
pub type Devinten = crate::Reg<devinten::DevintenSpec>;
#[doc = "USB Device Interrupt Enable"]
pub mod devinten;
#[doc = "DEVINTCLR (w) register accessor: USB Device Interrupt Clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devintclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devintclr`]
module"]
#[doc(alias = "DEVINTCLR")]
pub type Devintclr = crate::Reg<devintclr::DevintclrSpec>;
#[doc = "USB Device Interrupt Clear"]
pub mod devintclr;
#[doc = "DEVINTSET (w) register accessor: USB Device Interrupt Set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devintset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devintset`]
module"]
#[doc(alias = "DEVINTSET")]
pub type Devintset = crate::Reg<devintset::DevintsetSpec>;
#[doc = "USB Device Interrupt Set"]
pub mod devintset;
#[doc = "CMDCODE (w) register accessor: USB Command Code\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdcode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdcode`]
module"]
#[doc(alias = "CMDCODE")]
pub type Cmdcode = crate::Reg<cmdcode::CmdcodeSpec>;
#[doc = "USB Command Code"]
pub mod cmdcode;
#[doc = "CMDDATA (r) register accessor: USB Command Data\n\nYou can [`read`](crate::Reg::read) this register and get [`cmddata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmddata`]
module"]
#[doc(alias = "CMDDATA")]
pub type Cmddata = crate::Reg<cmddata::CmddataSpec>;
#[doc = "USB Command Data"]
pub mod cmddata;
#[doc = "RXDATA (r) register accessor: USB Receive Data\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdata`]
module"]
#[doc(alias = "RXDATA")]
pub type Rxdata = crate::Reg<rxdata::RxdataSpec>;
#[doc = "USB Receive Data"]
pub mod rxdata;
#[doc = "TXDATA (w) register accessor: USB Transmit Data\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdata::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdata`]
module"]
#[doc(alias = "TXDATA")]
pub type Txdata = crate::Reg<txdata::TxdataSpec>;
#[doc = "USB Transmit Data"]
pub mod txdata;
#[doc = "RXPLEN (r) register accessor: USB Receive Packet Length\n\nYou can [`read`](crate::Reg::read) this register and get [`rxplen::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxplen`]
module"]
#[doc(alias = "RXPLEN")]
pub type Rxplen = crate::Reg<rxplen::RxplenSpec>;
#[doc = "USB Receive Packet Length"]
pub mod rxplen;
#[doc = "TXPLEN (w) register accessor: USB Transmit Packet Length\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txplen::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txplen`]
module"]
#[doc(alias = "TXPLEN")]
pub type Txplen = crate::Reg<txplen::TxplenSpec>;
#[doc = "USB Transmit Packet Length"]
pub mod txplen;
#[doc = "CTRL (rw) register accessor: USB Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "USB Control"]
pub mod ctrl;
#[doc = "DEVINTPRI (w) register accessor: USB Device Interrupt Priority\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devintpri::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devintpri`]
module"]
#[doc(alias = "DEVINTPRI")]
pub type Devintpri = crate::Reg<devintpri::DevintpriSpec>;
#[doc = "USB Device Interrupt Priority"]
pub mod devintpri;
#[doc = "EPINTST (r) register accessor: USB Endpoint Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`epintst::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epintst`]
module"]
#[doc(alias = "EPINTST")]
pub type Epintst = crate::Reg<epintst::EpintstSpec>;
#[doc = "USB Endpoint Interrupt Status"]
pub mod epintst;
#[doc = "EPINTEN (rw) register accessor: USB Endpoint Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`epinten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epinten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epinten`]
module"]
#[doc(alias = "EPINTEN")]
pub type Epinten = crate::Reg<epinten::EpintenSpec>;
#[doc = "USB Endpoint Interrupt Enable"]
pub mod epinten;
#[doc = "EPINTCLR (w) register accessor: USB Endpoint Interrupt Clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epintclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epintclr`]
module"]
#[doc(alias = "EPINTCLR")]
pub type Epintclr = crate::Reg<epintclr::EpintclrSpec>;
#[doc = "USB Endpoint Interrupt Clear"]
pub mod epintclr;
#[doc = "EPINTSET (w) register accessor: USB Endpoint Interrupt Set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epintset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epintset`]
module"]
#[doc(alias = "EPINTSET")]
pub type Epintset = crate::Reg<epintset::EpintsetSpec>;
#[doc = "USB Endpoint Interrupt Set"]
pub mod epintset;
#[doc = "EPINTPRI (w) register accessor: USB Endpoint Priority\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epintpri::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epintpri`]
module"]
#[doc(alias = "EPINTPRI")]
pub type Epintpri = crate::Reg<epintpri::EpintpriSpec>;
#[doc = "USB Endpoint Priority"]
pub mod epintpri;
#[doc = "REEP (rw) register accessor: USB Realize Endpoint\n\nYou can [`read`](crate::Reg::read) this register and get [`reep::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reep::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reep`]
module"]
#[doc(alias = "REEP")]
pub type Reep = crate::Reg<reep::ReepSpec>;
#[doc = "USB Realize Endpoint"]
pub mod reep;
#[doc = "EPIND (w) register accessor: USB Endpoint Index\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epind::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epind`]
module"]
#[doc(alias = "EPIND")]
pub type Epind = crate::Reg<epind::EpindSpec>;
#[doc = "USB Endpoint Index"]
pub mod epind;
#[doc = "MAXPSIZE (rw) register accessor: USB MaxPacketSize\n\nYou can [`read`](crate::Reg::read) this register and get [`maxpsize::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maxpsize::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maxpsize`]
module"]
#[doc(alias = "MAXPSIZE")]
pub type Maxpsize = crate::Reg<maxpsize::MaxpsizeSpec>;
#[doc = "USB MaxPacketSize"]
pub mod maxpsize;
#[doc = "DMARST (r) register accessor: USB DMA Request Status\n\nYou can [`read`](crate::Reg::read) this register and get [`dmarst::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmarst`]
module"]
#[doc(alias = "DMARST")]
pub type Dmarst = crate::Reg<dmarst::DmarstSpec>;
#[doc = "USB DMA Request Status"]
pub mod dmarst;
#[doc = "DMARCLR (w) register accessor: USB DMA Request Clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmarclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmarclr`]
module"]
#[doc(alias = "DMARCLR")]
pub type Dmarclr = crate::Reg<dmarclr::DmarclrSpec>;
#[doc = "USB DMA Request Clear"]
pub mod dmarclr;
#[doc = "DMARSET (w) register accessor: USB DMA Request Set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmarset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmarset`]
module"]
#[doc(alias = "DMARSET")]
pub type Dmarset = crate::Reg<dmarset::DmarsetSpec>;
#[doc = "USB DMA Request Set"]
pub mod dmarset;
#[doc = "UDCAH (rw) register accessor: USB UDCA Head\n\nYou can [`read`](crate::Reg::read) this register and get [`udcah::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udcah::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udcah`]
module"]
#[doc(alias = "UDCAH")]
pub type Udcah = crate::Reg<udcah::UdcahSpec>;
#[doc = "USB UDCA Head"]
pub mod udcah;
#[doc = "EPDMAST (r) register accessor: USB Endpoint DMA Status\n\nYou can [`read`](crate::Reg::read) this register and get [`epdmast::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epdmast`]
module"]
#[doc(alias = "EPDMAST")]
pub type Epdmast = crate::Reg<epdmast::EpdmastSpec>;
#[doc = "USB Endpoint DMA Status"]
pub mod epdmast;
#[doc = "EPDMAEN (w) register accessor: USB Endpoint DMA Enable\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epdmaen::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epdmaen`]
module"]
#[doc(alias = "EPDMAEN")]
pub type Epdmaen = crate::Reg<epdmaen::EpdmaenSpec>;
#[doc = "USB Endpoint DMA Enable"]
pub mod epdmaen;
#[doc = "EPDMADIS (w) register accessor: USB Endpoint DMA Disable\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epdmadis::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epdmadis`]
module"]
#[doc(alias = "EPDMADIS")]
pub type Epdmadis = crate::Reg<epdmadis::EpdmadisSpec>;
#[doc = "USB Endpoint DMA Disable"]
pub mod epdmadis;
#[doc = "DMAINTST (r) register accessor: USB DMA Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`dmaintst::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmaintst`]
module"]
#[doc(alias = "DMAINTST")]
pub type Dmaintst = crate::Reg<dmaintst::DmaintstSpec>;
#[doc = "USB DMA Interrupt Status"]
pub mod dmaintst;
#[doc = "DMAINTEN (rw) register accessor: USB DMA Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`dmainten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmainten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmainten`]
module"]
#[doc(alias = "DMAINTEN")]
pub type Dmainten = crate::Reg<dmainten::DmaintenSpec>;
#[doc = "USB DMA Interrupt Enable"]
pub mod dmainten;
#[doc = "EOTINTST (r) register accessor: USB End of Transfer Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`eotintst::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eotintst`]
module"]
#[doc(alias = "EOTINTST")]
pub type Eotintst = crate::Reg<eotintst::EotintstSpec>;
#[doc = "USB End of Transfer Interrupt Status"]
pub mod eotintst;
#[doc = "EOTINTCLR (w) register accessor: USB End of Transfer Interrupt Clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eotintclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eotintclr`]
module"]
#[doc(alias = "EOTINTCLR")]
pub type Eotintclr = crate::Reg<eotintclr::EotintclrSpec>;
#[doc = "USB End of Transfer Interrupt Clear"]
pub mod eotintclr;
#[doc = "EOTINTSET (w) register accessor: USB End of Transfer Interrupt Set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eotintset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eotintset`]
module"]
#[doc(alias = "EOTINTSET")]
pub type Eotintset = crate::Reg<eotintset::EotintsetSpec>;
#[doc = "USB End of Transfer Interrupt Set"]
pub mod eotintset;
#[doc = "NDDRINTST (r) register accessor: USB New DD Request Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`nddrintst::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nddrintst`]
module"]
#[doc(alias = "NDDRINTST")]
pub type Nddrintst = crate::Reg<nddrintst::NddrintstSpec>;
#[doc = "USB New DD Request Interrupt Status"]
pub mod nddrintst;
#[doc = "NDDRINTCLR (w) register accessor: USB New DD Request Interrupt Clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nddrintclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nddrintclr`]
module"]
#[doc(alias = "NDDRINTCLR")]
pub type Nddrintclr = crate::Reg<nddrintclr::NddrintclrSpec>;
#[doc = "USB New DD Request Interrupt Clear"]
pub mod nddrintclr;
#[doc = "NDDRINTSET (w) register accessor: USB New DD Request Interrupt Set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nddrintset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nddrintset`]
module"]
#[doc(alias = "NDDRINTSET")]
pub type Nddrintset = crate::Reg<nddrintset::NddrintsetSpec>;
#[doc = "USB New DD Request Interrupt Set"]
pub mod nddrintset;
#[doc = "SYSERRINTST (r) register accessor: USB System Error Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`syserrintst::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syserrintst`]
module"]
#[doc(alias = "SYSERRINTST")]
pub type Syserrintst = crate::Reg<syserrintst::SyserrintstSpec>;
#[doc = "USB System Error Interrupt Status"]
pub mod syserrintst;
#[doc = "SYSERRINTCLR (w) register accessor: USB System Error Interrupt Clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syserrintclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syserrintclr`]
module"]
#[doc(alias = "SYSERRINTCLR")]
pub type Syserrintclr = crate::Reg<syserrintclr::SyserrintclrSpec>;
#[doc = "USB System Error Interrupt Clear"]
pub mod syserrintclr;
#[doc = "SYSERRINTSET (w) register accessor: USB System Error Interrupt Set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syserrintset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syserrintset`]
module"]
#[doc(alias = "SYSERRINTSET")]
pub type Syserrintset = crate::Reg<syserrintset::SyserrintsetSpec>;
#[doc = "USB System Error Interrupt Set"]
pub mod syserrintset;
#[doc = "I2C_RX (r) register accessor: I2C Receive\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_rx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_rx`]
module"]
#[doc(alias = "I2C_RX")]
pub type I2cRx = crate::Reg<i2c_rx::I2cRxSpec>;
#[doc = "I2C Receive"]
pub mod i2c_rx;
#[doc = "I2C_WO (w) register accessor: I2C Transmit\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_wo::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_wo`]
module"]
#[doc(alias = "I2C_WO")]
pub type I2cWo = crate::Reg<i2c_wo::I2cWoSpec>;
#[doc = "I2C Transmit"]
pub mod i2c_wo;
#[doc = "I2C_STS (r) register accessor: I2C Status\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_sts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_sts`]
module"]
#[doc(alias = "I2C_STS")]
pub type I2cSts = crate::Reg<i2c_sts::I2cStsSpec>;
#[doc = "I2C Status"]
pub mod i2c_sts;
#[doc = "I2C_CTL (rw) register accessor: I2C Control\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_ctl`]
module"]
#[doc(alias = "I2C_CTL")]
pub type I2cCtl = crate::Reg<i2c_ctl::I2cCtlSpec>;
#[doc = "I2C Control"]
pub mod i2c_ctl;
#[doc = "I2C_CLKHI (rw) register accessor: I2C Clock High\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_clkhi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_clkhi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_clkhi`]
module"]
#[doc(alias = "I2C_CLKHI")]
pub type I2cClkhi = crate::Reg<i2c_clkhi::I2cClkhiSpec>;
#[doc = "I2C Clock High"]
pub mod i2c_clkhi;
#[doc = "I2C_CLKLO (w) register accessor: I2C Clock Low\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_clklo::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_clklo`]
module"]
#[doc(alias = "I2C_CLKLO")]
pub type I2cClklo = crate::Reg<i2c_clklo::I2cClkloSpec>;
#[doc = "I2C Clock Low"]
pub mod i2c_clklo;
#[doc = "USBCLKCTRL (rw) register accessor: USB Clock Control\n\nYou can [`read`](crate::Reg::read) this register and get [`usbclkctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbclkctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbclkctrl`]
module"]
#[doc(alias = "USBCLKCTRL")]
pub type Usbclkctrl = crate::Reg<usbclkctrl::UsbclkctrlSpec>;
#[doc = "USB Clock Control"]
pub mod usbclkctrl;
#[doc = "OTGCLKCTRL (rw) register accessor: OTG clock controller\n\nYou can [`read`](crate::Reg::read) this register and get [`otgclkctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otgclkctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otgclkctrl`]
module"]
#[doc(alias = "OTGCLKCTRL")]
pub type Otgclkctrl = crate::Reg<otgclkctrl::OtgclkctrlSpec>;
#[doc = "OTG clock controller"]
pub mod otgclkctrl;
#[doc = "USBCLKST (r) register accessor: USB Clock Status\n\nYou can [`read`](crate::Reg::read) this register and get [`usbclkst::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbclkst`]
module"]
#[doc(alias = "USBCLKST")]
pub type Usbclkst = crate::Reg<usbclkst::UsbclkstSpec>;
#[doc = "USB Clock Status"]
pub mod usbclkst;
#[doc = "OTGCLKST (r) register accessor: OTG clock status\n\nYou can [`read`](crate::Reg::read) this register and get [`otgclkst::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otgclkst`]
module"]
#[doc(alias = "OTGCLKST")]
pub type Otgclkst = crate::Reg<otgclkst::OtgclkstSpec>;
#[doc = "OTG clock status"]
pub mod otgclkst;
