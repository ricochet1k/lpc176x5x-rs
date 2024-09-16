#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pinsel0: Pinsel0,
    pinsel1: Pinsel1,
    pinsel2: Pinsel2,
    pinsel3: Pinsel3,
    pinsel4: Pinsel4,
    _reserved5: [u8; 0x08],
    pinsel7: Pinsel7,
    _reserved6: [u8; 0x04],
    pinsel9: Pinsel9,
    pinsel10: Pinsel10,
    _reserved8: [u8; 0x14],
    pinmode0: Pinmode0,
    pinmode1: Pinmode1,
    pinmode2: Pinmode2,
    pinmode3: Pinmode3,
    pinmode4: Pinmode4,
    _reserved13: [u8; 0x08],
    pinmode7: Pinmode7,
    _reserved14: [u8; 0x04],
    pinmode9: Pinmode9,
    pinmode_od0: PinmodeOd0,
    pinmode_od1: PinmodeOd1,
    pinmode_od2: PinmodeOd2,
    pinmode_od3: PinmodeOd3,
    pinmode_od4: PinmodeOd4,
    i2cpadcfg: I2cpadcfg,
}
impl RegisterBlock {
    #[doc = "0x00 - Pin function select register 0."]
    #[inline(always)]
    pub const fn pinsel0(&self) -> &Pinsel0 {
        &self.pinsel0
    }
    #[doc = "0x04 - Pin function select register 1."]
    #[inline(always)]
    pub const fn pinsel1(&self) -> &Pinsel1 {
        &self.pinsel1
    }
    #[doc = "0x08 - Pin function select register 2."]
    #[inline(always)]
    pub const fn pinsel2(&self) -> &Pinsel2 {
        &self.pinsel2
    }
    #[doc = "0x0c - Pin function select register 3."]
    #[inline(always)]
    pub const fn pinsel3(&self) -> &Pinsel3 {
        &self.pinsel3
    }
    #[doc = "0x10 - Pin function select register 4"]
    #[inline(always)]
    pub const fn pinsel4(&self) -> &Pinsel4 {
        &self.pinsel4
    }
    #[doc = "0x1c - Pin function select register 7"]
    #[inline(always)]
    pub const fn pinsel7(&self) -> &Pinsel7 {
        &self.pinsel7
    }
    #[doc = "0x24 - Pin function select register 9"]
    #[inline(always)]
    pub const fn pinsel9(&self) -> &Pinsel9 {
        &self.pinsel9
    }
    #[doc = "0x28 - Pin function select register 10"]
    #[inline(always)]
    pub const fn pinsel10(&self) -> &Pinsel10 {
        &self.pinsel10
    }
    #[doc = "0x40 - Pin mode select register 0"]
    #[inline(always)]
    pub const fn pinmode0(&self) -> &Pinmode0 {
        &self.pinmode0
    }
    #[doc = "0x44 - Pin mode select register 1"]
    #[inline(always)]
    pub const fn pinmode1(&self) -> &Pinmode1 {
        &self.pinmode1
    }
    #[doc = "0x48 - Pin mode select register 2"]
    #[inline(always)]
    pub const fn pinmode2(&self) -> &Pinmode2 {
        &self.pinmode2
    }
    #[doc = "0x4c - Pin mode select register 3."]
    #[inline(always)]
    pub const fn pinmode3(&self) -> &Pinmode3 {
        &self.pinmode3
    }
    #[doc = "0x50 - Pin mode select register 4"]
    #[inline(always)]
    pub const fn pinmode4(&self) -> &Pinmode4 {
        &self.pinmode4
    }
    #[doc = "0x5c - Pin mode select register 7"]
    #[inline(always)]
    pub const fn pinmode7(&self) -> &Pinmode7 {
        &self.pinmode7
    }
    #[doc = "0x64 - Pin mode select register 9"]
    #[inline(always)]
    pub const fn pinmode9(&self) -> &Pinmode9 {
        &self.pinmode9
    }
    #[doc = "0x68 - Open drain mode control register 0"]
    #[inline(always)]
    pub const fn pinmode_od0(&self) -> &PinmodeOd0 {
        &self.pinmode_od0
    }
    #[doc = "0x6c - Open drain mode control register 1"]
    #[inline(always)]
    pub const fn pinmode_od1(&self) -> &PinmodeOd1 {
        &self.pinmode_od1
    }
    #[doc = "0x70 - Open drain mode control register 2"]
    #[inline(always)]
    pub const fn pinmode_od2(&self) -> &PinmodeOd2 {
        &self.pinmode_od2
    }
    #[doc = "0x74 - Open drain mode control register 3"]
    #[inline(always)]
    pub const fn pinmode_od3(&self) -> &PinmodeOd3 {
        &self.pinmode_od3
    }
    #[doc = "0x78 - Open drain mode control register 4"]
    #[inline(always)]
    pub const fn pinmode_od4(&self) -> &PinmodeOd4 {
        &self.pinmode_od4
    }
    #[doc = "0x7c - I2C Pin Configuration register"]
    #[inline(always)]
    pub const fn i2cpadcfg(&self) -> &I2cpadcfg {
        &self.i2cpadcfg
    }
}
#[doc = "PINSEL0 (rw) register accessor: Pin function select register 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`pinsel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pinsel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pinsel0`]
module"]
#[doc(alias = "PINSEL0")]
pub type Pinsel0 = crate::Reg<pinsel0::Pinsel0Spec>;
#[doc = "Pin function select register 0."]
pub mod pinsel0;
#[doc = "PINSEL1 (rw) register accessor: Pin function select register 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`pinsel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pinsel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pinsel1`]
module"]
#[doc(alias = "PINSEL1")]
pub type Pinsel1 = crate::Reg<pinsel1::Pinsel1Spec>;
#[doc = "Pin function select register 1."]
pub mod pinsel1;
#[doc = "PINSEL2 (rw) register accessor: Pin function select register 2.\n\nYou can [`read`](crate::Reg::read) this register and get [`pinsel2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pinsel2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pinsel2`]
module"]
#[doc(alias = "PINSEL2")]
pub type Pinsel2 = crate::Reg<pinsel2::Pinsel2Spec>;
#[doc = "Pin function select register 2."]
pub mod pinsel2;
#[doc = "PINSEL3 (rw) register accessor: Pin function select register 3.\n\nYou can [`read`](crate::Reg::read) this register and get [`pinsel3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pinsel3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pinsel3`]
module"]
#[doc(alias = "PINSEL3")]
pub type Pinsel3 = crate::Reg<pinsel3::Pinsel3Spec>;
#[doc = "Pin function select register 3."]
pub mod pinsel3;
#[doc = "PINSEL4 (rw) register accessor: Pin function select register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`pinsel4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pinsel4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pinsel4`]
module"]
#[doc(alias = "PINSEL4")]
pub type Pinsel4 = crate::Reg<pinsel4::Pinsel4Spec>;
#[doc = "Pin function select register 4"]
pub mod pinsel4;
#[doc = "PINSEL7 (rw) register accessor: Pin function select register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`pinsel7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pinsel7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pinsel7`]
module"]
#[doc(alias = "PINSEL7")]
pub type Pinsel7 = crate::Reg<pinsel7::Pinsel7Spec>;
#[doc = "Pin function select register 7"]
pub mod pinsel7;
#[doc = "PINSEL9 (rw) register accessor: Pin function select register 9\n\nYou can [`read`](crate::Reg::read) this register and get [`pinsel9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pinsel9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pinsel9`]
module"]
#[doc(alias = "PINSEL9")]
pub type Pinsel9 = crate::Reg<pinsel9::Pinsel9Spec>;
#[doc = "Pin function select register 9"]
pub mod pinsel9;
#[doc = "PINSEL10 (rw) register accessor: Pin function select register 10\n\nYou can [`read`](crate::Reg::read) this register and get [`pinsel10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pinsel10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pinsel10`]
module"]
#[doc(alias = "PINSEL10")]
pub type Pinsel10 = crate::Reg<pinsel10::Pinsel10Spec>;
#[doc = "Pin function select register 10"]
pub mod pinsel10;
#[doc = "PINMODE0 (rw) register accessor: Pin mode select register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pinmode0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pinmode0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pinmode0`]
module"]
#[doc(alias = "PINMODE0")]
pub type Pinmode0 = crate::Reg<pinmode0::Pinmode0Spec>;
#[doc = "Pin mode select register 0"]
pub mod pinmode0;
#[doc = "PINMODE1 (rw) register accessor: Pin mode select register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pinmode1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pinmode1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pinmode1`]
module"]
#[doc(alias = "PINMODE1")]
pub type Pinmode1 = crate::Reg<pinmode1::Pinmode1Spec>;
#[doc = "Pin mode select register 1"]
pub mod pinmode1;
#[doc = "PINMODE2 (rw) register accessor: Pin mode select register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`pinmode2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pinmode2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pinmode2`]
module"]
#[doc(alias = "PINMODE2")]
pub type Pinmode2 = crate::Reg<pinmode2::Pinmode2Spec>;
#[doc = "Pin mode select register 2"]
pub mod pinmode2;
#[doc = "PINMODE3 (rw) register accessor: Pin mode select register 3.\n\nYou can [`read`](crate::Reg::read) this register and get [`pinmode3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pinmode3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pinmode3`]
module"]
#[doc(alias = "PINMODE3")]
pub type Pinmode3 = crate::Reg<pinmode3::Pinmode3Spec>;
#[doc = "Pin mode select register 3."]
pub mod pinmode3;
#[doc = "PINMODE4 (rw) register accessor: Pin mode select register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`pinmode4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pinmode4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pinmode4`]
module"]
#[doc(alias = "PINMODE4")]
pub type Pinmode4 = crate::Reg<pinmode4::Pinmode4Spec>;
#[doc = "Pin mode select register 4"]
pub mod pinmode4;
#[doc = "PINMODE7 (rw) register accessor: Pin mode select register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`pinmode7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pinmode7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pinmode7`]
module"]
#[doc(alias = "PINMODE7")]
pub type Pinmode7 = crate::Reg<pinmode7::Pinmode7Spec>;
#[doc = "Pin mode select register 7"]
pub mod pinmode7;
#[doc = "PINMODE9 (rw) register accessor: Pin mode select register 9\n\nYou can [`read`](crate::Reg::read) this register and get [`pinmode9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pinmode9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pinmode9`]
module"]
#[doc(alias = "PINMODE9")]
pub type Pinmode9 = crate::Reg<pinmode9::Pinmode9Spec>;
#[doc = "Pin mode select register 9"]
pub mod pinmode9;
#[doc = "PINMODE_OD0 (rw) register accessor: Open drain mode control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pinmode_od0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pinmode_od0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pinmode_od0`]
module"]
#[doc(alias = "PINMODE_OD0")]
pub type PinmodeOd0 = crate::Reg<pinmode_od0::PinmodeOd0Spec>;
#[doc = "Open drain mode control register 0"]
pub mod pinmode_od0;
#[doc = "PINMODE_OD1 (rw) register accessor: Open drain mode control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pinmode_od1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pinmode_od1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pinmode_od1`]
module"]
#[doc(alias = "PINMODE_OD1")]
pub type PinmodeOd1 = crate::Reg<pinmode_od1::PinmodeOd1Spec>;
#[doc = "Open drain mode control register 1"]
pub mod pinmode_od1;
#[doc = "PINMODE_OD2 (rw) register accessor: Open drain mode control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`pinmode_od2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pinmode_od2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pinmode_od2`]
module"]
#[doc(alias = "PINMODE_OD2")]
pub type PinmodeOd2 = crate::Reg<pinmode_od2::PinmodeOd2Spec>;
#[doc = "Open drain mode control register 2"]
pub mod pinmode_od2;
#[doc = "PINMODE_OD3 (rw) register accessor: Open drain mode control register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`pinmode_od3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pinmode_od3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pinmode_od3`]
module"]
#[doc(alias = "PINMODE_OD3")]
pub type PinmodeOd3 = crate::Reg<pinmode_od3::PinmodeOd3Spec>;
#[doc = "Open drain mode control register 3"]
pub mod pinmode_od3;
#[doc = "PINMODE_OD4 (rw) register accessor: Open drain mode control register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`pinmode_od4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pinmode_od4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pinmode_od4`]
module"]
#[doc(alias = "PINMODE_OD4")]
pub type PinmodeOd4 = crate::Reg<pinmode_od4::PinmodeOd4Spec>;
#[doc = "Open drain mode control register 4"]
pub mod pinmode_od4;
#[doc = "I2CPADCFG (rw) register accessor: I2C Pin Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cpadcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cpadcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cpadcfg`]
module"]
#[doc(alias = "I2CPADCFG")]
pub type I2cpadcfg = crate::Reg<i2cpadcfg::I2cpadcfgSpec>;
#[doc = "I2C Pin Configuration register"]
pub mod i2cpadcfg;
