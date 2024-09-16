#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dir: (),
    _reserved1: [u8; 0x10],
    mask: (),
    _reserved2: [u8; 0x04],
    pin: (),
    _reserved3: [u8; 0x04],
    set: (),
    _reserved4: [u8; 0x04],
    clr: (),
}
impl RegisterBlock {
    #[doc = "0x00..0x14 - GPIO Port Direction control register."]
    #[inline(always)]
    pub const fn dir(&self, n: usize) -> &Dir {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe { &*(self as *const Self).cast::<u8>().add(0).add(32 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x14 - GPIO Port Direction control register."]
    #[inline(always)]
    pub fn dir_iter(&self) -> impl Iterator<Item = &Dir> {
        (0..5)
            .map(move |n| unsafe { &*(self as *const Self).cast::<u8>().add(0).add(32 * n).cast() })
    }
    #[doc = "0x10..0x24 - Mask register for Port."]
    #[inline(always)]
    pub const fn mask(&self, n: usize) -> &Mask {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(16)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x10..0x24 - Mask register for Port."]
    #[inline(always)]
    pub fn mask_iter(&self) -> impl Iterator<Item = &Mask> {
        (0..5).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(16)
                .add(32 * n)
                .cast()
        })
    }
    #[doc = "0x14..0x28 - Port Pin value register using FIOMASK."]
    #[inline(always)]
    pub const fn pin(&self, n: usize) -> &Pin {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(20)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x14..0x28 - Port Pin value register using FIOMASK."]
    #[inline(always)]
    pub fn pin_iter(&self) -> impl Iterator<Item = &Pin> {
        (0..5).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(20)
                .add(32 * n)
                .cast()
        })
    }
    #[doc = "0x18..0x2c - Port Output Set register using FIOMASK."]
    #[inline(always)]
    pub const fn set(&self, n: usize) -> &Set {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(24)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x18..0x2c - Port Output Set register using FIOMASK."]
    #[inline(always)]
    pub fn set_iter(&self) -> impl Iterator<Item = &Set> {
        (0..5).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(24)
                .add(32 * n)
                .cast()
        })
    }
    #[doc = "0x1c..0x30 - Port Output Clear register using FIOMASK."]
    #[inline(always)]
    pub const fn clr(&self, n: usize) -> &Clr {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(28)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1c..0x30 - Port Output Clear register using FIOMASK."]
    #[inline(always)]
    pub fn clr_iter(&self) -> impl Iterator<Item = &Clr> {
        (0..5).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(28)
                .add(32 * n)
                .cast()
        })
    }
}
#[doc = "DIR (rw) register accessor: GPIO Port Direction control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`dir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dir`]
module"]
#[doc(alias = "DIR")]
pub type Dir = crate::Reg<dir::DirSpec>;
#[doc = "GPIO Port Direction control register."]
pub mod dir;
#[doc = "MASK (rw) register accessor: Mask register for Port.\n\nYou can [`read`](crate::Reg::read) this register and get [`mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mask`]
module"]
#[doc(alias = "MASK")]
pub type Mask = crate::Reg<mask::MaskSpec>;
#[doc = "Mask register for Port."]
pub mod mask;
#[doc = "PIN (rw) register accessor: Port Pin value register using FIOMASK.\n\nYou can [`read`](crate::Reg::read) this register and get [`pin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pin`]
module"]
#[doc(alias = "PIN")]
pub type Pin = crate::Reg<pin::PinSpec>;
#[doc = "Port Pin value register using FIOMASK."]
pub mod pin;
#[doc = "SET (rw) register accessor: Port Output Set register using FIOMASK.\n\nYou can [`read`](crate::Reg::read) this register and get [`set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@set`]
module"]
#[doc(alias = "SET")]
pub type Set = crate::Reg<set::SetSpec>;
#[doc = "Port Output Set register using FIOMASK."]
pub mod set;
#[doc = "CLR (w) register accessor: Port Output Clear register using FIOMASK.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clr`]
module"]
#[doc(alias = "CLR")]
pub type Clr = crate::Reg<clr::ClrSpec>;
#[doc = "Port Output Clear register using FIOMASK."]
pub mod clr;
