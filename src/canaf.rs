#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    afmr: Afmr,
    sff_sa: SffSa,
    sff_grp_sa: SffGrpSa,
    eff_sa: EffSa,
    eff_grp_sa: EffGrpSa,
    endoftable: Endoftable,
    luterrad: Luterrad,
    luterr: Luterr,
    fcanie: Fcanie,
    fcanic0: Fcanic0,
    fcanic1: Fcanic1,
}
impl RegisterBlock {
    #[doc = "0x00 - Acceptance Filter Register"]
    #[inline(always)]
    pub const fn afmr(&self) -> &Afmr {
        &self.afmr
    }
    #[doc = "0x04 - Standard Frame Individual Start Address Register"]
    #[inline(always)]
    pub const fn sff_sa(&self) -> &SffSa {
        &self.sff_sa
    }
    #[doc = "0x08 - Standard Frame Group Start Address Register"]
    #[inline(always)]
    pub const fn sff_grp_sa(&self) -> &SffGrpSa {
        &self.sff_grp_sa
    }
    #[doc = "0x0c - Extended Frame Start Address Register"]
    #[inline(always)]
    pub const fn eff_sa(&self) -> &EffSa {
        &self.eff_sa
    }
    #[doc = "0x10 - Extended Frame Group Start Address Register"]
    #[inline(always)]
    pub const fn eff_grp_sa(&self) -> &EffGrpSa {
        &self.eff_grp_sa
    }
    #[doc = "0x14 - End of AF Tables register"]
    #[inline(always)]
    pub const fn endoftable(&self) -> &Endoftable {
        &self.endoftable
    }
    #[doc = "0x18 - LUT Error Address register"]
    #[inline(always)]
    pub const fn luterrad(&self) -> &Luterrad {
        &self.luterrad
    }
    #[doc = "0x1c - LUT Error Register"]
    #[inline(always)]
    pub const fn luterr(&self) -> &Luterr {
        &self.luterr
    }
    #[doc = "0x20 - FullCAN interrupt enable register"]
    #[inline(always)]
    pub const fn fcanie(&self) -> &Fcanie {
        &self.fcanie
    }
    #[doc = "0x24 - FullCAN interrupt and capture register0"]
    #[inline(always)]
    pub const fn fcanic0(&self) -> &Fcanic0 {
        &self.fcanic0
    }
    #[doc = "0x28 - FullCAN interrupt and capture register1"]
    #[inline(always)]
    pub const fn fcanic1(&self) -> &Fcanic1 {
        &self.fcanic1
    }
}
#[doc = "AFMR (rw) register accessor: Acceptance Filter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`afmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@afmr`]
module"]
#[doc(alias = "AFMR")]
pub type Afmr = crate::Reg<afmr::AfmrSpec>;
#[doc = "Acceptance Filter Register"]
pub mod afmr;
#[doc = "SFF_SA (rw) register accessor: Standard Frame Individual Start Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sff_sa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sff_sa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sff_sa`]
module"]
#[doc(alias = "SFF_SA")]
pub type SffSa = crate::Reg<sff_sa::SffSaSpec>;
#[doc = "Standard Frame Individual Start Address Register"]
pub mod sff_sa;
#[doc = "SFF_GRP_SA (rw) register accessor: Standard Frame Group Start Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sff_grp_sa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sff_grp_sa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sff_grp_sa`]
module"]
#[doc(alias = "SFF_GRP_SA")]
pub type SffGrpSa = crate::Reg<sff_grp_sa::SffGrpSaSpec>;
#[doc = "Standard Frame Group Start Address Register"]
pub mod sff_grp_sa;
#[doc = "EFF_SA (rw) register accessor: Extended Frame Start Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`eff_sa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eff_sa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eff_sa`]
module"]
#[doc(alias = "EFF_SA")]
pub type EffSa = crate::Reg<eff_sa::EffSaSpec>;
#[doc = "Extended Frame Start Address Register"]
pub mod eff_sa;
#[doc = "EFF_GRP_SA (rw) register accessor: Extended Frame Group Start Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`eff_grp_sa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eff_grp_sa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eff_grp_sa`]
module"]
#[doc(alias = "EFF_GRP_SA")]
pub type EffGrpSa = crate::Reg<eff_grp_sa::EffGrpSaSpec>;
#[doc = "Extended Frame Group Start Address Register"]
pub mod eff_grp_sa;
#[doc = "ENDOFTABLE (rw) register accessor: End of AF Tables register\n\nYou can [`read`](crate::Reg::read) this register and get [`endoftable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`endoftable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@endoftable`]
module"]
#[doc(alias = "ENDOFTABLE")]
pub type Endoftable = crate::Reg<endoftable::EndoftableSpec>;
#[doc = "End of AF Tables register"]
pub mod endoftable;
#[doc = "LUTERRAD (r) register accessor: LUT Error Address register\n\nYou can [`read`](crate::Reg::read) this register and get [`luterrad::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@luterrad`]
module"]
#[doc(alias = "LUTERRAD")]
pub type Luterrad = crate::Reg<luterrad::LuterradSpec>;
#[doc = "LUT Error Address register"]
pub mod luterrad;
#[doc = "LUTERR (r) register accessor: LUT Error Register\n\nYou can [`read`](crate::Reg::read) this register and get [`luterr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@luterr`]
module"]
#[doc(alias = "LUTERR")]
pub type Luterr = crate::Reg<luterr::LuterrSpec>;
#[doc = "LUT Error Register"]
pub mod luterr;
#[doc = "FCANIE (rw) register accessor: FullCAN interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`fcanie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcanie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcanie`]
module"]
#[doc(alias = "FCANIE")]
pub type Fcanie = crate::Reg<fcanie::FcanieSpec>;
#[doc = "FullCAN interrupt enable register"]
pub mod fcanie;
#[doc = "FCANIC0 (rw) register accessor: FullCAN interrupt and capture register0\n\nYou can [`read`](crate::Reg::read) this register and get [`fcanic0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcanic0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcanic0`]
module"]
#[doc(alias = "FCANIC0")]
pub type Fcanic0 = crate::Reg<fcanic0::Fcanic0Spec>;
#[doc = "FullCAN interrupt and capture register0"]
pub mod fcanic0;
#[doc = "FCANIC1 (rw) register accessor: FullCAN interrupt and capture register1\n\nYou can [`read`](crate::Reg::read) this register and get [`fcanic1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcanic1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcanic1`]
module"]
#[doc(alias = "FCANIC1")]
pub type Fcanic1 = crate::Reg<fcanic1::Fcanic1Spec>;
#[doc = "FullCAN interrupt and capture register1"]
pub mod fcanic1;
