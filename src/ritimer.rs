#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    compval: Compval,
    mask: Mask,
    ctrl: Ctrl,
    counter: Counter,
}
impl RegisterBlock {
    #[doc = "0x00 - Compare register"]
    #[inline(always)]
    pub const fn compval(&self) -> &Compval {
        &self.compval
    }
    #[doc = "0x04 - Mask register. This register holds the 32-bit mask value. A 1 written to any bit will force a compare on the corresponding bit of the counter and compare register."]
    #[inline(always)]
    pub const fn mask(&self) -> &Mask {
        &self.mask
    }
    #[doc = "0x08 - Control register."]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x0c - 32-bit counter"]
    #[inline(always)]
    pub const fn counter(&self) -> &Counter {
        &self.counter
    }
}
#[doc = "COMPVAL (rw) register accessor: Compare register\n\nYou can [`read`](crate::Reg::read) this register and get [`compval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`compval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@compval`]
module"]
#[doc(alias = "COMPVAL")]
pub type Compval = crate::Reg<compval::CompvalSpec>;
#[doc = "Compare register"]
pub mod compval;
#[doc = "MASK (rw) register accessor: Mask register. This register holds the 32-bit mask value. A 1 written to any bit will force a compare on the corresponding bit of the counter and compare register.\n\nYou can [`read`](crate::Reg::read) this register and get [`mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mask`]
module"]
#[doc(alias = "MASK")]
pub type Mask = crate::Reg<mask::MaskSpec>;
#[doc = "Mask register. This register holds the 32-bit mask value. A 1 written to any bit will force a compare on the corresponding bit of the counter and compare register."]
pub mod mask;
#[doc = "CTRL (rw) register accessor: Control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control register."]
pub mod ctrl;
#[doc = "COUNTER (rw) register accessor: 32-bit counter\n\nYou can [`read`](crate::Reg::read) this register and get [`counter::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`counter::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@counter`]
module"]
#[doc(alias = "COUNTER")]
pub type Counter = crate::Reg<counter::CounterSpec>;
#[doc = "32-bit counter"]
pub mod counter;
