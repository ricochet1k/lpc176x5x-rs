#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    sr: Sr,
    dr: Dr,
    ccr: Ccr,
    _reserved4: [u8; 0x0c],
    int: Int,
}
impl RegisterBlock {
    #[doc = "0x00 - SPI Control Register. This register controls the operation of the SPI."]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x04 - SPI Status Register. This register shows the status of the SPI."]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x08 - SPI Data Register. This bi-directional register provides the transmit and receive data for the SPI. Transmit data is provided to the SPI0 by writing to this register. Data received by the SPI0 can be read from this register."]
    #[inline(always)]
    pub const fn dr(&self) -> &Dr {
        &self.dr
    }
    #[doc = "0x0c - SPI Clock Counter Register. This register controls the frequency of a master's SCK0."]
    #[inline(always)]
    pub const fn ccr(&self) -> &Ccr {
        &self.ccr
    }
    #[doc = "0x1c - SPI Interrupt Flag. This register contains the interrupt flag for the SPI interface."]
    #[inline(always)]
    pub const fn int(&self) -> &Int {
        &self.int
    }
}
#[doc = "CR (rw) register accessor: SPI Control Register. This register controls the operation of the SPI.\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "SPI Control Register. This register controls the operation of the SPI."]
pub mod cr;
#[doc = "SR (r) register accessor: SPI Status Register. This register shows the status of the SPI.\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "SPI Status Register. This register shows the status of the SPI."]
pub mod sr;
#[doc = "DR (rw) register accessor: SPI Data Register. This bi-directional register provides the transmit and receive data for the SPI. Transmit data is provided to the SPI0 by writing to this register. Data received by the SPI0 can be read from this register.\n\nYou can [`read`](crate::Reg::read) this register and get [`dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">The register is <b>modified</b> in some way after a read operation.</div>\n\nFor information about available fields see [`mod@dr`]
module"]
#[doc(alias = "DR")]
pub type Dr = crate::Reg<dr::DrSpec>;
#[doc = "SPI Data Register. This bi-directional register provides the transmit and receive data for the SPI. Transmit data is provided to the SPI0 by writing to this register. Data received by the SPI0 can be read from this register."]
pub mod dr;
#[doc = "CCR (rw) register accessor: SPI Clock Counter Register. This register controls the frequency of a master's SCK0.\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr`]
module"]
#[doc(alias = "CCR")]
pub type Ccr = crate::Reg<ccr::CcrSpec>;
#[doc = "SPI Clock Counter Register. This register controls the frequency of a master's SCK0."]
pub mod ccr;
#[doc = "INT (rw) register accessor: SPI Interrupt Flag. This register contains the interrupt flag for the SPI interface.\n\nYou can [`read`](crate::Reg::read) this register and get [`int::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int`]
module"]
#[doc(alias = "INT")]
pub type Int = crate::Reg<int::IntSpec>;
#[doc = "SPI Interrupt Flag. This register contains the interrupt flag for the SPI interface."]
pub mod int;
