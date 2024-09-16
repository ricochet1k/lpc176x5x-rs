#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    con: Con,
    stat: Stat,
    conf: Conf,
    pos: Pos,
    maxpos: Maxpos,
    cmpos0: Cmpos0,
    cmpos1: Cmpos1,
    cmpos2: Cmpos2,
    inxcnt: Inxcnt,
    inxcmp0: Inxcmp0,
    load: Load,
    time: Time,
    vel: Vel,
    cap: Cap,
    velcomp: Velcomp,
    filter: Filter,
    _reserved16: [u8; 0x0f98],
    iec: Iec,
    ies: Ies,
    intstat: Intstat,
    ie: Ie,
    clr: Clr,
    set: Set,
}
impl RegisterBlock {
    #[doc = "0x00 - Control register"]
    #[inline(always)]
    pub const fn con(&self) -> &Con {
        &self.con
    }
    #[doc = "0x04 - Status register"]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
    }
    #[doc = "0x08 - Configuration register"]
    #[inline(always)]
    pub const fn conf(&self) -> &Conf {
        &self.conf
    }
    #[doc = "0x0c - Position register"]
    #[inline(always)]
    pub const fn pos(&self) -> &Pos {
        &self.pos
    }
    #[doc = "0x10 - Maximum position register"]
    #[inline(always)]
    pub const fn maxpos(&self) -> &Maxpos {
        &self.maxpos
    }
    #[doc = "0x14 - Position compare register 0"]
    #[inline(always)]
    pub const fn cmpos0(&self) -> &Cmpos0 {
        &self.cmpos0
    }
    #[doc = "0x18 - Position compare register 1"]
    #[inline(always)]
    pub const fn cmpos1(&self) -> &Cmpos1 {
        &self.cmpos1
    }
    #[doc = "0x1c - Position compare register 2"]
    #[inline(always)]
    pub const fn cmpos2(&self) -> &Cmpos2 {
        &self.cmpos2
    }
    #[doc = "0x20 - Index count register 0"]
    #[inline(always)]
    pub const fn inxcnt(&self) -> &Inxcnt {
        &self.inxcnt
    }
    #[doc = "0x24 - Index compare register 0"]
    #[inline(always)]
    pub const fn inxcmp0(&self) -> &Inxcmp0 {
        &self.inxcmp0
    }
    #[doc = "0x28 - Velocity timer reload register"]
    #[inline(always)]
    pub const fn load(&self) -> &Load {
        &self.load
    }
    #[doc = "0x2c - Velocity timer register"]
    #[inline(always)]
    pub const fn time(&self) -> &Time {
        &self.time
    }
    #[doc = "0x30 - Velocity counter register"]
    #[inline(always)]
    pub const fn vel(&self) -> &Vel {
        &self.vel
    }
    #[doc = "0x34 - Velocity capture register"]
    #[inline(always)]
    pub const fn cap(&self) -> &Cap {
        &self.cap
    }
    #[doc = "0x38 - Velocity compare register"]
    #[inline(always)]
    pub const fn velcomp(&self) -> &Velcomp {
        &self.velcomp
    }
    #[doc = "0x3c - Digital filter register"]
    #[inline(always)]
    pub const fn filter(&self) -> &Filter {
        &self.filter
    }
    #[doc = "0xfd8 - Interrupt enable clear register"]
    #[inline(always)]
    pub const fn iec(&self) -> &Iec {
        &self.iec
    }
    #[doc = "0xfdc - Interrupt enable set register"]
    #[inline(always)]
    pub const fn ies(&self) -> &Ies {
        &self.ies
    }
    #[doc = "0xfe0 - Interrupt status register"]
    #[inline(always)]
    pub const fn intstat(&self) -> &Intstat {
        &self.intstat
    }
    #[doc = "0xfe4 - Interrupt enable register"]
    #[inline(always)]
    pub const fn ie(&self) -> &Ie {
        &self.ie
    }
    #[doc = "0xfe8 - Interrupt status clear register"]
    #[inline(always)]
    pub const fn clr(&self) -> &Clr {
        &self.clr
    }
    #[doc = "0xfec - Interrupt status set register"]
    #[inline(always)]
    pub const fn set(&self) -> &Set {
        &self.set
    }
}
#[doc = "CON (w) register accessor: Control register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`con::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@con`]
module"]
#[doc(alias = "CON")]
pub type Con = crate::Reg<con::ConSpec>;
#[doc = "Control register"]
#[path = "qei/con_.rs"]
pub mod con;
#[doc = "CONF (rw) register accessor: Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf`]
module"]
#[doc(alias = "CONF")]
pub type Conf = crate::Reg<conf::ConfSpec>;
#[doc = "Configuration register"]
pub mod conf;
#[doc = "STAT (r) register accessor: Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
#[doc(alias = "STAT")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "Status register"]
pub mod stat;
#[doc = "POS (r) register accessor: Position register\n\nYou can [`read`](crate::Reg::read) this register and get [`pos::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pos`]
module"]
#[doc(alias = "POS")]
pub type Pos = crate::Reg<pos::PosSpec>;
#[doc = "Position register"]
pub mod pos;
#[doc = "MAXPOS (rw) register accessor: Maximum position register\n\nYou can [`read`](crate::Reg::read) this register and get [`maxpos::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maxpos::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maxpos`]
module"]
#[doc(alias = "MAXPOS")]
pub type Maxpos = crate::Reg<maxpos::MaxposSpec>;
#[doc = "Maximum position register"]
pub mod maxpos;
#[doc = "CMPOS0 (rw) register accessor: Position compare register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpos0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpos0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpos0`]
module"]
#[doc(alias = "CMPOS0")]
pub type Cmpos0 = crate::Reg<cmpos0::Cmpos0Spec>;
#[doc = "Position compare register 0"]
pub mod cmpos0;
#[doc = "CMPOS1 (rw) register accessor: Position compare register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpos1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpos1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpos1`]
module"]
#[doc(alias = "CMPOS1")]
pub type Cmpos1 = crate::Reg<cmpos1::Cmpos1Spec>;
#[doc = "Position compare register 1"]
pub mod cmpos1;
#[doc = "CMPOS2 (rw) register accessor: Position compare register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpos2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpos2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpos2`]
module"]
#[doc(alias = "CMPOS2")]
pub type Cmpos2 = crate::Reg<cmpos2::Cmpos2Spec>;
#[doc = "Position compare register 2"]
pub mod cmpos2;
#[doc = "INXCNT (r) register accessor: Index count register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`inxcnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inxcnt`]
module"]
#[doc(alias = "INXCNT")]
pub type Inxcnt = crate::Reg<inxcnt::InxcntSpec>;
#[doc = "Index count register 0"]
pub mod inxcnt;
#[doc = "INXCMP0 (rw) register accessor: Index compare register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`inxcmp0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inxcmp0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inxcmp0`]
module"]
#[doc(alias = "INXCMP0")]
pub type Inxcmp0 = crate::Reg<inxcmp0::Inxcmp0Spec>;
#[doc = "Index compare register 0"]
pub mod inxcmp0;
#[doc = "LOAD (rw) register accessor: Velocity timer reload register\n\nYou can [`read`](crate::Reg::read) this register and get [`load::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`load::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@load`]
module"]
#[doc(alias = "LOAD")]
pub type Load = crate::Reg<load::LoadSpec>;
#[doc = "Velocity timer reload register"]
pub mod load;
#[doc = "TIME (r) register accessor: Velocity timer register\n\nYou can [`read`](crate::Reg::read) this register and get [`time::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@time`]
module"]
#[doc(alias = "TIME")]
pub type Time = crate::Reg<time::TimeSpec>;
#[doc = "Velocity timer register"]
pub mod time;
#[doc = "VEL (r) register accessor: Velocity counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`vel::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vel`]
module"]
#[doc(alias = "VEL")]
pub type Vel = crate::Reg<vel::VelSpec>;
#[doc = "Velocity counter register"]
pub mod vel;
#[doc = "CAP (r) register accessor: Velocity capture register\n\nYou can [`read`](crate::Reg::read) this register and get [`cap::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap`]
module"]
#[doc(alias = "CAP")]
pub type Cap = crate::Reg<cap::CapSpec>;
#[doc = "Velocity capture register"]
pub mod cap;
#[doc = "VELCOMP (rw) register accessor: Velocity compare register\n\nYou can [`read`](crate::Reg::read) this register and get [`velcomp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`velcomp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@velcomp`]
module"]
#[doc(alias = "VELCOMP")]
pub type Velcomp = crate::Reg<velcomp::VelcompSpec>;
#[doc = "Velocity compare register"]
pub mod velcomp;
#[doc = "FILTER (rw) register accessor: Digital filter register\n\nYou can [`read`](crate::Reg::read) this register and get [`filter::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filter::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@filter`]
module"]
#[doc(alias = "FILTER")]
pub type Filter = crate::Reg<filter::FilterSpec>;
#[doc = "Digital filter register"]
pub mod filter;
#[doc = "INTSTAT (r) register accessor: Interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`intstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intstat`]
module"]
#[doc(alias = "INTSTAT")]
pub type Intstat = crate::Reg<intstat::IntstatSpec>;
#[doc = "Interrupt status register"]
pub mod intstat;
#[doc = "SET (w) register accessor: Interrupt status set register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@set`]
module"]
#[doc(alias = "SET")]
pub type Set = crate::Reg<set::SetSpec>;
#[doc = "Interrupt status set register"]
pub mod set;
#[doc = "CLR (w) register accessor: Interrupt status clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clr`]
module"]
#[doc(alias = "CLR")]
pub type Clr = crate::Reg<clr::ClrSpec>;
#[doc = "Interrupt status clear register"]
pub mod clr;
#[doc = "IE (r) register accessor: Interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ie::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ie`]
module"]
#[doc(alias = "IE")]
pub type Ie = crate::Reg<ie::IeSpec>;
#[doc = "Interrupt enable register"]
pub mod ie;
#[doc = "IES (w) register accessor: Interrupt enable set register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ies::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ies`]
module"]
#[doc(alias = "IES")]
pub type Ies = crate::Reg<ies::IesSpec>;
#[doc = "Interrupt enable set register"]
pub mod ies;
#[doc = "IEC (w) register accessor: Interrupt enable clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iec::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iec`]
module"]
#[doc(alias = "IEC")]
pub type Iec = crate::Reg<iec::IecSpec>;
#[doc = "Interrupt enable clear register"]
pub mod iec;
