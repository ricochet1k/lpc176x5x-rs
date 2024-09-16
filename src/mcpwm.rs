#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    con: Con,
    con_set: ConSet,
    con_clr: ConClr,
    capcon: Capcon,
    capcon_set: CapconSet,
    capcon_clr: CapconClr,
    tc: [Tc; 3],
    lim: [Lim; 3],
    mat: [Mat; 3],
    dt: Dt,
    cp: Cp,
    cap: [Cap; 3],
    inten: Inten,
    inten_set: IntenSet,
    inten_clr: IntenClr,
    cntcon: Cntcon,
    cntcon_set: CntconSet,
    cntcon_clr: CntconClr,
    intf: Intf,
    intf_set: IntfSet,
    intf_clr: IntfClr,
    cap_clr: CapClr,
}
impl RegisterBlock {
    #[doc = "0x00 - PWM Control read address"]
    #[inline(always)]
    pub const fn con(&self) -> &Con {
        &self.con
    }
    #[doc = "0x04 - PWM Control set address"]
    #[inline(always)]
    pub const fn con_set(&self) -> &ConSet {
        &self.con_set
    }
    #[doc = "0x08 - PWM Control clear address"]
    #[inline(always)]
    pub const fn con_clr(&self) -> &ConClr {
        &self.con_clr
    }
    #[doc = "0x0c - Capture Control read address"]
    #[inline(always)]
    pub const fn capcon(&self) -> &Capcon {
        &self.capcon
    }
    #[doc = "0x10 - Capture Control set address"]
    #[inline(always)]
    pub const fn capcon_set(&self) -> &CapconSet {
        &self.capcon_set
    }
    #[doc = "0x14 - Event Control clear address"]
    #[inline(always)]
    pub const fn capcon_clr(&self) -> &CapconClr {
        &self.capcon_clr
    }
    #[doc = "0x18..0x24 - Timer Counter register"]
    #[inline(always)]
    pub const fn tc(&self, n: usize) -> &Tc {
        &self.tc[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x18..0x24 - Timer Counter register"]
    #[inline(always)]
    pub fn tc_iter(&self) -> impl Iterator<Item = &Tc> {
        self.tc.iter()
    }
    #[doc = "0x24..0x30 - Limit register"]
    #[inline(always)]
    pub const fn lim(&self, n: usize) -> &Lim {
        &self.lim[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x24..0x30 - Limit register"]
    #[inline(always)]
    pub fn lim_iter(&self) -> impl Iterator<Item = &Lim> {
        self.lim.iter()
    }
    #[doc = "0x30..0x3c - Match register"]
    #[inline(always)]
    pub const fn mat(&self, n: usize) -> &Mat {
        &self.mat[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x30..0x3c - Match register"]
    #[inline(always)]
    pub fn mat_iter(&self) -> impl Iterator<Item = &Mat> {
        self.mat.iter()
    }
    #[doc = "0x3c - Dead time register"]
    #[inline(always)]
    pub const fn dt(&self) -> &Dt {
        &self.dt
    }
    #[doc = "0x40 - Communication Pattern register"]
    #[inline(always)]
    pub const fn cp(&self) -> &Cp {
        &self.cp
    }
    #[doc = "0x44..0x50 - Capture register"]
    #[inline(always)]
    pub const fn cap(&self, n: usize) -> &Cap {
        &self.cap[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x44..0x50 - Capture register"]
    #[inline(always)]
    pub fn cap_iter(&self) -> impl Iterator<Item = &Cap> {
        self.cap.iter()
    }
    #[doc = "0x50 - Interrupt Enable read address"]
    #[inline(always)]
    pub const fn inten(&self) -> &Inten {
        &self.inten
    }
    #[doc = "0x54 - Interrupt Enable set address"]
    #[inline(always)]
    pub const fn inten_set(&self) -> &IntenSet {
        &self.inten_set
    }
    #[doc = "0x58 - Interrupt Enable clear address"]
    #[inline(always)]
    pub const fn inten_clr(&self) -> &IntenClr {
        &self.inten_clr
    }
    #[doc = "0x5c - Count Control read address"]
    #[inline(always)]
    pub const fn cntcon(&self) -> &Cntcon {
        &self.cntcon
    }
    #[doc = "0x60 - Count Control set address"]
    #[inline(always)]
    pub const fn cntcon_set(&self) -> &CntconSet {
        &self.cntcon_set
    }
    #[doc = "0x64 - Count Control clear address"]
    #[inline(always)]
    pub const fn cntcon_clr(&self) -> &CntconClr {
        &self.cntcon_clr
    }
    #[doc = "0x68 - Interrupt flags read address"]
    #[inline(always)]
    pub const fn intf(&self) -> &Intf {
        &self.intf
    }
    #[doc = "0x6c - Interrupt flags set address"]
    #[inline(always)]
    pub const fn intf_set(&self) -> &IntfSet {
        &self.intf_set
    }
    #[doc = "0x70 - Interrupt flags clear address"]
    #[inline(always)]
    pub const fn intf_clr(&self) -> &IntfClr {
        &self.intf_clr
    }
    #[doc = "0x74 - Capture clear address"]
    #[inline(always)]
    pub const fn cap_clr(&self) -> &CapClr {
        &self.cap_clr
    }
}
#[doc = "CON (r) register accessor: PWM Control read address\n\nYou can [`read`](crate::Reg::read) this register and get [`con::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@con`]
module"]
#[doc(alias = "CON")]
pub type Con = crate::Reg<con::ConSpec>;
#[doc = "PWM Control read address"]
#[path = "mcpwm/con_.rs"]
pub mod con;
#[doc = "CON_SET (w) register accessor: PWM Control set address\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`con_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@con_set`]
module"]
#[doc(alias = "CON_SET")]
pub type ConSet = crate::Reg<con_set::ConSetSpec>;
#[doc = "PWM Control set address"]
pub mod con_set;
#[doc = "CON_CLR (w) register accessor: PWM Control clear address\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`con_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@con_clr`]
module"]
#[doc(alias = "CON_CLR")]
pub type ConClr = crate::Reg<con_clr::ConClrSpec>;
#[doc = "PWM Control clear address"]
pub mod con_clr;
#[doc = "CAPCON (r) register accessor: Capture Control read address\n\nYou can [`read`](crate::Reg::read) this register and get [`capcon::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capcon`]
module"]
#[doc(alias = "CAPCON")]
pub type Capcon = crate::Reg<capcon::CapconSpec>;
#[doc = "Capture Control read address"]
pub mod capcon;
#[doc = "CAPCON_SET (w) register accessor: Capture Control set address\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capcon_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capcon_set`]
module"]
#[doc(alias = "CAPCON_SET")]
pub type CapconSet = crate::Reg<capcon_set::CapconSetSpec>;
#[doc = "Capture Control set address"]
pub mod capcon_set;
#[doc = "CAPCON_CLR (w) register accessor: Event Control clear address\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capcon_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capcon_clr`]
module"]
#[doc(alias = "CAPCON_CLR")]
pub type CapconClr = crate::Reg<capcon_clr::CapconClrSpec>;
#[doc = "Event Control clear address"]
pub mod capcon_clr;
#[doc = "TC (rw) register accessor: Timer Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`tc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tc`]
module"]
#[doc(alias = "TC")]
pub type Tc = crate::Reg<tc::TcSpec>;
#[doc = "Timer Counter register"]
pub mod tc;
#[doc = "LIM (rw) register accessor: Limit register\n\nYou can [`read`](crate::Reg::read) this register and get [`lim::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lim::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lim`]
module"]
#[doc(alias = "LIM")]
pub type Lim = crate::Reg<lim::LimSpec>;
#[doc = "Limit register"]
pub mod lim;
#[doc = "MAT (rw) register accessor: Match register\n\nYou can [`read`](crate::Reg::read) this register and get [`mat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mat`]
module"]
#[doc(alias = "MAT")]
pub type Mat = crate::Reg<mat::MatSpec>;
#[doc = "Match register"]
pub mod mat;
#[doc = "DT (rw) register accessor: Dead time register\n\nYou can [`read`](crate::Reg::read) this register and get [`dt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt`]
module"]
#[doc(alias = "DT")]
pub type Dt = crate::Reg<dt::DtSpec>;
#[doc = "Dead time register"]
pub mod dt;
#[doc = "CP (rw) register accessor: Communication Pattern register\n\nYou can [`read`](crate::Reg::read) this register and get [`cp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cp`]
module"]
#[doc(alias = "CP")]
pub type Cp = crate::Reg<cp::CpSpec>;
#[doc = "Communication Pattern register"]
pub mod cp;
#[doc = "CAP (r) register accessor: Capture register\n\nYou can [`read`](crate::Reg::read) this register and get [`cap::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap`]
module"]
#[doc(alias = "CAP")]
pub type Cap = crate::Reg<cap::CapSpec>;
#[doc = "Capture register"]
pub mod cap;
#[doc = "INTEN (r) register accessor: Interrupt Enable read address\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`]
module"]
#[doc(alias = "INTEN")]
pub type Inten = crate::Reg<inten::IntenSpec>;
#[doc = "Interrupt Enable read address"]
pub mod inten;
#[doc = "INTEN_SET (w) register accessor: Interrupt Enable set address\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten_set`]
module"]
#[doc(alias = "INTEN_SET")]
pub type IntenSet = crate::Reg<inten_set::IntenSetSpec>;
#[doc = "Interrupt Enable set address"]
pub mod inten_set;
#[doc = "INTEN_CLR (w) register accessor: Interrupt Enable clear address\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten_clr`]
module"]
#[doc(alias = "INTEN_CLR")]
pub type IntenClr = crate::Reg<inten_clr::IntenClrSpec>;
#[doc = "Interrupt Enable clear address"]
pub mod inten_clr;
#[doc = "INTF (r) register accessor: Interrupt flags read address\n\nYou can [`read`](crate::Reg::read) this register and get [`intf::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf`]
module"]
#[doc(alias = "INTF")]
pub type Intf = crate::Reg<intf::IntfSpec>;
#[doc = "Interrupt flags read address"]
pub mod intf;
#[doc = "INTF_SET (w) register accessor: Interrupt flags set address\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_set`]
module"]
#[doc(alias = "INTF_SET")]
pub type IntfSet = crate::Reg<intf_set::IntfSetSpec>;
#[doc = "Interrupt flags set address"]
pub mod intf_set;
#[doc = "INTF_CLR (w) register accessor: Interrupt flags clear address\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_clr`]
module"]
#[doc(alias = "INTF_CLR")]
pub type IntfClr = crate::Reg<intf_clr::IntfClrSpec>;
#[doc = "Interrupt flags clear address"]
pub mod intf_clr;
#[doc = "CNTCON (r) register accessor: Count Control read address\n\nYou can [`read`](crate::Reg::read) this register and get [`cntcon::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntcon`]
module"]
#[doc(alias = "CNTCON")]
pub type Cntcon = crate::Reg<cntcon::CntconSpec>;
#[doc = "Count Control read address"]
pub mod cntcon;
#[doc = "CNTCON_SET (w) register accessor: Count Control set address\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntcon_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntcon_set`]
module"]
#[doc(alias = "CNTCON_SET")]
pub type CntconSet = crate::Reg<cntcon_set::CntconSetSpec>;
#[doc = "Count Control set address"]
pub mod cntcon_set;
#[doc = "CNTCON_CLR (w) register accessor: Count Control clear address\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntcon_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntcon_clr`]
module"]
#[doc(alias = "CNTCON_CLR")]
pub type CntconClr = crate::Reg<cntcon_clr::CntconClrSpec>;
#[doc = "Count Control clear address"]
pub mod cntcon_clr;
#[doc = "CAP_CLR (w) register accessor: Capture clear address\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap_clr`]
module"]
#[doc(alias = "CAP_CLR")]
pub type CapClr = crate::Reg<cap_clr::CapClrSpec>;
#[doc = "Capture clear address"]
pub mod cap_clr;
