#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    ctrl: Ctrl,
    cntval: Cntval,
}
impl RegisterBlock {
    #[doc = "0x00 - D/A Converter Register. This register contains the digital value to be converted to analog and a power control bit."]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x04 - DAC Control register. This register controls DMA and timer operation."]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x08 - DAC Counter Value register. This register contains the reload value for the DAC DMA/Interrupt timer."]
    #[inline(always)]
    pub const fn cntval(&self) -> &Cntval {
        &self.cntval
    }
}
#[doc = "CR (rw) register accessor: D/A Converter Register. This register contains the digital value to be converted to analog and a power control bit.\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "D/A Converter Register. This register contains the digital value to be converted to analog and a power control bit."]
pub mod cr;
#[doc = "CTRL (rw) register accessor: DAC Control register. This register controls DMA and timer operation.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "DAC Control register. This register controls DMA and timer operation."]
pub mod ctrl;
#[doc = "CNTVAL (rw) register accessor: DAC Counter Value register. This register contains the reload value for the DAC DMA/Interrupt timer.\n\nYou can [`read`](crate::Reg::read) this register and get [`cntval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntval`]
module"]
#[doc(alias = "CNTVAL")]
pub type Cntval = crate::Reg<cntval::CntvalSpec>;
#[doc = "DAC Counter Value register. This register contains the reload value for the DAC DMA/Interrupt timer."]
pub mod cntval;
