#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    flashcfg: Flashcfg,
    _reserved1: [u8; 0x7c],
    pll0con: Pll0con,
    pll0cfg: Pll0cfg,
    pll0stat: Pll0stat,
    pll0feed: Pll0feed,
    _reserved5: [u8; 0x10],
    pll1con: Pll1con,
    pll1cfg: Pll1cfg,
    pll1stat: Pll1stat,
    pll1feed: Pll1feed,
    _reserved9: [u8; 0x10],
    pcon: Pcon,
    pconp: Pconp,
    _reserved11: [u8; 0x3c],
    cclkcfg: Cclkcfg,
    usbclkcfg: Usbclkcfg,
    clksrcsel: Clksrcsel,
    cansleepclr: Cansleepclr,
    canwakeflags: Canwakeflags,
    _reserved16: [u8; 0x28],
    extint: Extint,
    _reserved17: [u8; 0x04],
    extmode: Extmode,
    extpolar: Extpolar,
    _reserved19: [u8; 0x30],
    rsid: Rsid,
    _reserved20: [u8; 0x1c],
    scs: Scs,
    _reserved21: [u8; 0x04],
    pclksel0: Pclksel0,
    pclksel1: Pclksel1,
    _reserved23: [u8; 0x10],
    usbintst: Usbintst,
    dmacreqsel: Dmacreqsel,
    clkoutcfg: Clkoutcfg,
}
impl RegisterBlock {
    #[doc = "0x00 - Flash Accelerator Configuration Register. Controls flash access timing."]
    #[inline(always)]
    pub const fn flashcfg(&self) -> &Flashcfg {
        &self.flashcfg
    }
    #[doc = "0x80 - PLL0 Control Register"]
    #[inline(always)]
    pub const fn pll0con(&self) -> &Pll0con {
        &self.pll0con
    }
    #[doc = "0x84 - PLL0 Configuration Register"]
    #[inline(always)]
    pub const fn pll0cfg(&self) -> &Pll0cfg {
        &self.pll0cfg
    }
    #[doc = "0x88 - PLL0 Status Register"]
    #[inline(always)]
    pub const fn pll0stat(&self) -> &Pll0stat {
        &self.pll0stat
    }
    #[doc = "0x8c - PLL0 Feed Register"]
    #[inline(always)]
    pub const fn pll0feed(&self) -> &Pll0feed {
        &self.pll0feed
    }
    #[doc = "0xa0 - PLL1 Control Register"]
    #[inline(always)]
    pub const fn pll1con(&self) -> &Pll1con {
        &self.pll1con
    }
    #[doc = "0xa4 - PLL1 Configuration Register"]
    #[inline(always)]
    pub const fn pll1cfg(&self) -> &Pll1cfg {
        &self.pll1cfg
    }
    #[doc = "0xa8 - PLL1 Status Register"]
    #[inline(always)]
    pub const fn pll1stat(&self) -> &Pll1stat {
        &self.pll1stat
    }
    #[doc = "0xac - PLL1 Feed Register"]
    #[inline(always)]
    pub const fn pll1feed(&self) -> &Pll1feed {
        &self.pll1feed
    }
    #[doc = "0xc0 - Power Control Register"]
    #[inline(always)]
    pub const fn pcon(&self) -> &Pcon {
        &self.pcon
    }
    #[doc = "0xc4 - Power Control for Peripherals Register"]
    #[inline(always)]
    pub const fn pconp(&self) -> &Pconp {
        &self.pconp
    }
    #[doc = "0x104 - CPU Clock Configuration Register"]
    #[inline(always)]
    pub const fn cclkcfg(&self) -> &Cclkcfg {
        &self.cclkcfg
    }
    #[doc = "0x108 - USB Clock Configuration Register"]
    #[inline(always)]
    pub const fn usbclkcfg(&self) -> &Usbclkcfg {
        &self.usbclkcfg
    }
    #[doc = "0x10c - Clock Source Select Register"]
    #[inline(always)]
    pub const fn clksrcsel(&self) -> &Clksrcsel {
        &self.clksrcsel
    }
    #[doc = "0x110 - Allows clearing the current CAN channel sleep state as well as reading that state."]
    #[inline(always)]
    pub const fn cansleepclr(&self) -> &Cansleepclr {
        &self.cansleepclr
    }
    #[doc = "0x114 - Allows reading the wake-up state of the CAN channels."]
    #[inline(always)]
    pub const fn canwakeflags(&self) -> &Canwakeflags {
        &self.canwakeflags
    }
    #[doc = "0x140 - External Interrupt Flag Register"]
    #[inline(always)]
    pub const fn extint(&self) -> &Extint {
        &self.extint
    }
    #[doc = "0x148 - External Interrupt Mode register"]
    #[inline(always)]
    pub const fn extmode(&self) -> &Extmode {
        &self.extmode
    }
    #[doc = "0x14c - External Interrupt Polarity Register"]
    #[inline(always)]
    pub const fn extpolar(&self) -> &Extpolar {
        &self.extpolar
    }
    #[doc = "0x180 - Reset Source Identification Register"]
    #[inline(always)]
    pub const fn rsid(&self) -> &Rsid {
        &self.rsid
    }
    #[doc = "0x1a0 - System control and status"]
    #[inline(always)]
    pub const fn scs(&self) -> &Scs {
        &self.scs
    }
    #[doc = "0x1a8 - Peripheral Clock Selection register 0."]
    #[inline(always)]
    pub const fn pclksel0(&self) -> &Pclksel0 {
        &self.pclksel0
    }
    #[doc = "0x1ac - Peripheral Clock Selection register 1."]
    #[inline(always)]
    pub const fn pclksel1(&self) -> &Pclksel1 {
        &self.pclksel1
    }
    #[doc = "0x1c0 - USB Interrupt Status"]
    #[inline(always)]
    pub const fn usbintst(&self) -> &Usbintst {
        &self.usbintst
    }
    #[doc = "0x1c4 - Selects between alternative requests on DMA channels 0 through 7 and 10 through 15"]
    #[inline(always)]
    pub const fn dmacreqsel(&self) -> &Dmacreqsel {
        &self.dmacreqsel
    }
    #[doc = "0x1c8 - Clock Output Configuration Register"]
    #[inline(always)]
    pub const fn clkoutcfg(&self) -> &Clkoutcfg {
        &self.clkoutcfg
    }
}
#[doc = "FLASHCFG (rw) register accessor: Flash Accelerator Configuration Register. Controls flash access timing.\n\nYou can [`read`](crate::Reg::read) this register and get [`flashcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashcfg`]
module"]
#[doc(alias = "FLASHCFG")]
pub type Flashcfg = crate::Reg<flashcfg::FlashcfgSpec>;
#[doc = "Flash Accelerator Configuration Register. Controls flash access timing."]
pub mod flashcfg;
#[doc = "PLL0CON (rw) register accessor: PLL0 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll0con::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll0con::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll0con`]
module"]
#[doc(alias = "PLL0CON")]
pub type Pll0con = crate::Reg<pll0con::Pll0conSpec>;
#[doc = "PLL0 Control Register"]
pub mod pll0con;
#[doc = "PLL0CFG (rw) register accessor: PLL0 Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll0cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll0cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll0cfg`]
module"]
#[doc(alias = "PLL0CFG")]
pub type Pll0cfg = crate::Reg<pll0cfg::Pll0cfgSpec>;
#[doc = "PLL0 Configuration Register"]
pub mod pll0cfg;
#[doc = "PLL0STAT (r) register accessor: PLL0 Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll0stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll0stat`]
module"]
#[doc(alias = "PLL0STAT")]
pub type Pll0stat = crate::Reg<pll0stat::Pll0statSpec>;
#[doc = "PLL0 Status Register"]
pub mod pll0stat;
#[doc = "PLL0FEED (w) register accessor: PLL0 Feed Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll0feed::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll0feed`]
module"]
#[doc(alias = "PLL0FEED")]
pub type Pll0feed = crate::Reg<pll0feed::Pll0feedSpec>;
#[doc = "PLL0 Feed Register"]
pub mod pll0feed;
#[doc = "PLL1CON (rw) register accessor: PLL1 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll1con::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1con::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll1con`]
module"]
#[doc(alias = "PLL1CON")]
pub type Pll1con = crate::Reg<pll1con::Pll1conSpec>;
#[doc = "PLL1 Control Register"]
pub mod pll1con;
#[doc = "PLL1CFG (rw) register accessor: PLL1 Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll1cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll1cfg`]
module"]
#[doc(alias = "PLL1CFG")]
pub type Pll1cfg = crate::Reg<pll1cfg::Pll1cfgSpec>;
#[doc = "PLL1 Configuration Register"]
pub mod pll1cfg;
#[doc = "PLL1STAT (r) register accessor: PLL1 Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll1stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll1stat`]
module"]
#[doc(alias = "PLL1STAT")]
pub type Pll1stat = crate::Reg<pll1stat::Pll1statSpec>;
#[doc = "PLL1 Status Register"]
pub mod pll1stat;
#[doc = "PLL1FEED (w) register accessor: PLL1 Feed Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1feed::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll1feed`]
module"]
#[doc(alias = "PLL1FEED")]
pub type Pll1feed = crate::Reg<pll1feed::Pll1feedSpec>;
#[doc = "PLL1 Feed Register"]
pub mod pll1feed;
#[doc = "PCON (rw) register accessor: Power Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pcon::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcon::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcon`]
module"]
#[doc(alias = "PCON")]
pub type Pcon = crate::Reg<pcon::PconSpec>;
#[doc = "Power Control Register"]
pub mod pcon;
#[doc = "PCONP (rw) register accessor: Power Control for Peripherals Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pconp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pconp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pconp`]
module"]
#[doc(alias = "PCONP")]
pub type Pconp = crate::Reg<pconp::PconpSpec>;
#[doc = "Power Control for Peripherals Register"]
pub mod pconp;
#[doc = "CCLKCFG (rw) register accessor: CPU Clock Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cclkcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cclkcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cclkcfg`]
module"]
#[doc(alias = "CCLKCFG")]
pub type Cclkcfg = crate::Reg<cclkcfg::CclkcfgSpec>;
#[doc = "CPU Clock Configuration Register"]
pub mod cclkcfg;
#[doc = "USBCLKCFG (rw) register accessor: USB Clock Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usbclkcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbclkcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbclkcfg`]
module"]
#[doc(alias = "USBCLKCFG")]
pub type Usbclkcfg = crate::Reg<usbclkcfg::UsbclkcfgSpec>;
#[doc = "USB Clock Configuration Register"]
pub mod usbclkcfg;
#[doc = "CLKSRCSEL (rw) register accessor: Clock Source Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clksrcsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clksrcsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksrcsel`]
module"]
#[doc(alias = "CLKSRCSEL")]
pub type Clksrcsel = crate::Reg<clksrcsel::ClksrcselSpec>;
#[doc = "Clock Source Select Register"]
pub mod clksrcsel;
#[doc = "CANSLEEPCLR (rw) register accessor: Allows clearing the current CAN channel sleep state as well as reading that state.\n\nYou can [`read`](crate::Reg::read) this register and get [`cansleepclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cansleepclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cansleepclr`]
module"]
#[doc(alias = "CANSLEEPCLR")]
pub type Cansleepclr = crate::Reg<cansleepclr::CansleepclrSpec>;
#[doc = "Allows clearing the current CAN channel sleep state as well as reading that state."]
pub mod cansleepclr;
#[doc = "CANWAKEFLAGS (rw) register accessor: Allows reading the wake-up state of the CAN channels.\n\nYou can [`read`](crate::Reg::read) this register and get [`canwakeflags::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`canwakeflags::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@canwakeflags`]
module"]
#[doc(alias = "CANWAKEFLAGS")]
pub type Canwakeflags = crate::Reg<canwakeflags::CanwakeflagsSpec>;
#[doc = "Allows reading the wake-up state of the CAN channels."]
pub mod canwakeflags;
#[doc = "EXTINT (rw) register accessor: External Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`extint::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extint::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extint`]
module"]
#[doc(alias = "EXTINT")]
pub type Extint = crate::Reg<extint::ExtintSpec>;
#[doc = "External Interrupt Flag Register"]
pub mod extint;
#[doc = "EXTMODE (rw) register accessor: External Interrupt Mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`extmode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extmode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extmode`]
module"]
#[doc(alias = "EXTMODE")]
pub type Extmode = crate::Reg<extmode::ExtmodeSpec>;
#[doc = "External Interrupt Mode register"]
pub mod extmode;
#[doc = "EXTPOLAR (rw) register accessor: External Interrupt Polarity Register\n\nYou can [`read`](crate::Reg::read) this register and get [`extpolar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extpolar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extpolar`]
module"]
#[doc(alias = "EXTPOLAR")]
pub type Extpolar = crate::Reg<extpolar::ExtpolarSpec>;
#[doc = "External Interrupt Polarity Register"]
pub mod extpolar;
#[doc = "RSID (rw) register accessor: Reset Source Identification Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rsid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsid`]
module"]
#[doc(alias = "RSID")]
pub type Rsid = crate::Reg<rsid::RsidSpec>;
#[doc = "Reset Source Identification Register"]
pub mod rsid;
#[doc = "SCS (rw) register accessor: System control and status\n\nYou can [`read`](crate::Reg::read) this register and get [`scs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scs`]
module"]
#[doc(alias = "SCS")]
pub type Scs = crate::Reg<scs::ScsSpec>;
#[doc = "System control and status"]
pub mod scs;
#[doc = "PCLKSEL0 (rw) register accessor: Peripheral Clock Selection register 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`pclksel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pclksel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pclksel0`]
module"]
#[doc(alias = "PCLKSEL0")]
pub type Pclksel0 = crate::Reg<pclksel0::Pclksel0Spec>;
#[doc = "Peripheral Clock Selection register 0."]
pub mod pclksel0;
#[doc = "PCLKSEL1 (rw) register accessor: Peripheral Clock Selection register 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`pclksel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pclksel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pclksel1`]
module"]
#[doc(alias = "PCLKSEL1")]
pub type Pclksel1 = crate::Reg<pclksel1::Pclksel1Spec>;
#[doc = "Peripheral Clock Selection register 1."]
pub mod pclksel1;
#[doc = "USBINTST (rw) register accessor: USB Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`usbintst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbintst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbintst`]
module"]
#[doc(alias = "USBINTST")]
pub type Usbintst = crate::Reg<usbintst::UsbintstSpec>;
#[doc = "USB Interrupt Status"]
pub mod usbintst;
#[doc = "DMACREQSEL (rw) register accessor: Selects between alternative requests on DMA channels 0 through 7 and 10 through 15\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacreqsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacreqsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacreqsel`]
module"]
#[doc(alias = "DMACREQSEL")]
pub type Dmacreqsel = crate::Reg<dmacreqsel::DmacreqselSpec>;
#[doc = "Selects between alternative requests on DMA channels 0 through 7 and 10 through 15"]
pub mod dmacreqsel;
#[doc = "CLKOUTCFG (rw) register accessor: Clock Output Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clkoutcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkoutcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkoutcfg`]
module"]
#[doc(alias = "CLKOUTCFG")]
pub type Clkoutcfg = crate::Reg<clkoutcfg::ClkoutcfgSpec>;
#[doc = "Clock Output Configuration Register"]
pub mod clkoutcfg;
