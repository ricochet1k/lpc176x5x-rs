#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mask: [Mask; 512],
}
impl RegisterBlock {
    #[doc = "0x00..0x800 - CAN AF ram access register"]
    #[inline(always)]
    pub const fn mask(&self, n: usize) -> &Mask {
        &self.mask[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x800 - CAN AF ram access register"]
    #[inline(always)]
    pub fn mask_iter(&self) -> impl Iterator<Item = &Mask> {
        self.mask.iter()
    }
}
#[doc = "MASK (rw) register accessor: CAN AF ram access register\n\nYou can [`read`](crate::Reg::read) this register and get [`mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mask`]
module"]
#[doc(alias = "MASK")]
pub type Mask = crate::Reg<mask::MaskSpec>;
#[doc = "CAN AF ram access register"]
pub mod mask;
