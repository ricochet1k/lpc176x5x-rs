#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    status: Status,
    statr0: Statr0,
    statf0: Statf0,
    clr0: Clr0,
    enr0: Enr0,
    enf0: Enf0,
    _reserved6: [u8; 0x0c],
    statr2: Statr2,
    statf2: Statf2,
    clr2: Clr2,
    enr2: Enr2,
    enf2: Enf2,
}
impl RegisterBlock {
    #[doc = "0x00 - GPIO overall Interrupt Status."]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x04 - GPIO Interrupt Status for Rising edge for Port 0."]
    #[inline(always)]
    pub const fn statr0(&self) -> &Statr0 {
        &self.statr0
    }
    #[doc = "0x08 - GPIO Interrupt Status for Falling edge for Port 0."]
    #[inline(always)]
    pub const fn statf0(&self) -> &Statf0 {
        &self.statf0
    }
    #[doc = "0x0c - GPIO Interrupt Clear."]
    #[inline(always)]
    pub const fn clr0(&self) -> &Clr0 {
        &self.clr0
    }
    #[doc = "0x10 - GPIO Interrupt Enable for Rising edge for Port 0."]
    #[inline(always)]
    pub const fn enr0(&self) -> &Enr0 {
        &self.enr0
    }
    #[doc = "0x14 - GPIO Interrupt Enable for Falling edge for Port 0."]
    #[inline(always)]
    pub const fn enf0(&self) -> &Enf0 {
        &self.enf0
    }
    #[doc = "0x24 - GPIO Interrupt Status for Rising edge for Port 0."]
    #[inline(always)]
    pub const fn statr2(&self) -> &Statr2 {
        &self.statr2
    }
    #[doc = "0x28 - GPIO Interrupt Status for Falling edge for Port 0."]
    #[inline(always)]
    pub const fn statf2(&self) -> &Statf2 {
        &self.statf2
    }
    #[doc = "0x2c - GPIO Interrupt Clear."]
    #[inline(always)]
    pub const fn clr2(&self) -> &Clr2 {
        &self.clr2
    }
    #[doc = "0x30 - GPIO Interrupt Enable for Rising edge for Port 0."]
    #[inline(always)]
    pub const fn enr2(&self) -> &Enr2 {
        &self.enr2
    }
    #[doc = "0x34 - GPIO Interrupt Enable for Falling edge for Port 0."]
    #[inline(always)]
    pub const fn enf2(&self) -> &Enf2 {
        &self.enf2
    }
}
#[doc = "STATUS (r) register accessor: GPIO overall Interrupt Status.\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "GPIO overall Interrupt Status."]
pub mod status;
#[doc = "STATR0 (r) register accessor: GPIO Interrupt Status for Rising edge for Port 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`statr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@statr0`]
module"]
#[doc(alias = "STATR0")]
pub type Statr0 = crate::Reg<statr0::Statr0Spec>;
#[doc = "GPIO Interrupt Status for Rising edge for Port 0."]
pub mod statr0;
#[doc = "STATF0 (r) register accessor: GPIO Interrupt Status for Falling edge for Port 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`statf0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@statf0`]
module"]
#[doc(alias = "STATF0")]
pub type Statf0 = crate::Reg<statf0::Statf0Spec>;
#[doc = "GPIO Interrupt Status for Falling edge for Port 0."]
pub mod statf0;
#[doc = "CLR0 (w) register accessor: GPIO Interrupt Clear.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clr0`]
module"]
#[doc(alias = "CLR0")]
pub type Clr0 = crate::Reg<clr0::Clr0Spec>;
#[doc = "GPIO Interrupt Clear."]
pub mod clr0;
#[doc = "ENR0 (rw) register accessor: GPIO Interrupt Enable for Rising edge for Port 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`enr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enr0`]
module"]
#[doc(alias = "ENR0")]
pub type Enr0 = crate::Reg<enr0::Enr0Spec>;
#[doc = "GPIO Interrupt Enable for Rising edge for Port 0."]
pub mod enr0;
#[doc = "ENF0 (rw) register accessor: GPIO Interrupt Enable for Falling edge for Port 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`enf0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enf0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enf0`]
module"]
#[doc(alias = "ENF0")]
pub type Enf0 = crate::Reg<enf0::Enf0Spec>;
#[doc = "GPIO Interrupt Enable for Falling edge for Port 0."]
pub mod enf0;
#[doc = "STATR2 (r) register accessor: GPIO Interrupt Status for Rising edge for Port 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`statr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@statr2`]
module"]
#[doc(alias = "STATR2")]
pub type Statr2 = crate::Reg<statr2::Statr2Spec>;
#[doc = "GPIO Interrupt Status for Rising edge for Port 0."]
pub mod statr2;
#[doc = "STATF2 (r) register accessor: GPIO Interrupt Status for Falling edge for Port 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`statf2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@statf2`]
module"]
#[doc(alias = "STATF2")]
pub type Statf2 = crate::Reg<statf2::Statf2Spec>;
#[doc = "GPIO Interrupt Status for Falling edge for Port 0."]
pub mod statf2;
#[doc = "CLR2 (w) register accessor: GPIO Interrupt Clear.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clr2`]
module"]
#[doc(alias = "CLR2")]
pub type Clr2 = crate::Reg<clr2::Clr2Spec>;
#[doc = "GPIO Interrupt Clear."]
pub mod clr2;
#[doc = "ENR2 (rw) register accessor: GPIO Interrupt Enable for Rising edge for Port 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`enr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enr2`]
module"]
#[doc(alias = "ENR2")]
pub type Enr2 = crate::Reg<enr2::Enr2Spec>;
#[doc = "GPIO Interrupt Enable for Rising edge for Port 0."]
pub mod enr2;
#[doc = "ENF2 (rw) register accessor: GPIO Interrupt Enable for Falling edge for Port 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`enf2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enf2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enf2`]
module"]
#[doc(alias = "ENF2")]
pub type Enf2 = crate::Reg<enf2::Enf2Spec>;
#[doc = "GPIO Interrupt Enable for Falling edge for Port 0."]
pub mod enf2;
