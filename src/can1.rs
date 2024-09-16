#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mod_: Mod,
    cmr: Cmr,
    gsr: Gsr,
    icr: Icr,
    ier: Ier,
    btr: Btr,
    ewl: Ewl,
    sr: Sr,
    rfs: Rfs,
    rid: Rid,
    rda: Rda,
    rdb: Rdb,
    tfi: (),
    _reserved13: [u8; 0x04],
    tid: (),
    _reserved14: [u8; 0x04],
    tda: (),
    _reserved15: [u8; 0x04],
    tdb: (),
}
impl RegisterBlock {
    #[doc = "0x00 - Controls the operating mode of the CAN Controller."]
    #[inline(always)]
    pub const fn mod_(&self) -> &Mod {
        &self.mod_
    }
    #[doc = "0x04 - Command bits that affect the state of the CAN Controller"]
    #[inline(always)]
    pub const fn cmr(&self) -> &Cmr {
        &self.cmr
    }
    #[doc = "0x08 - Global Controller Status and Error Counters. The error counters can only be written when RM in CANMOD is 1."]
    #[inline(always)]
    pub const fn gsr(&self) -> &Gsr {
        &self.gsr
    }
    #[doc = "0x0c - Interrupt status, Arbitration Lost Capture, Error Code Capture"]
    #[inline(always)]
    pub const fn icr(&self) -> &Icr {
        &self.icr
    }
    #[doc = "0x10 - Interrupt Enable"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    #[doc = "0x14 - Bus Timing. Can only be written when RM in CANMOD is 1."]
    #[inline(always)]
    pub const fn btr(&self) -> &Btr {
        &self.btr
    }
    #[doc = "0x18 - Error Warning Limit. Can only be written when RM in CANMOD is 1."]
    #[inline(always)]
    pub const fn ewl(&self) -> &Ewl {
        &self.ewl
    }
    #[doc = "0x1c - Status Register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x20 - Receive frame status. Can only be written when RM in CANMOD is 1."]
    #[inline(always)]
    pub const fn rfs(&self) -> &Rfs {
        &self.rfs
    }
    #[doc = "0x24 - Received Identifier. Can only be written when RM in CANMOD is 1."]
    #[inline(always)]
    pub const fn rid(&self) -> &Rid {
        &self.rid
    }
    #[doc = "0x28 - Received data bytes 1-4. Can only be written when RM in CANMOD is 1."]
    #[inline(always)]
    pub const fn rda(&self) -> &Rda {
        &self.rda
    }
    #[doc = "0x2c - Received data bytes 5-8. Can only be written when RM in CANMOD is 1."]
    #[inline(always)]
    pub const fn rdb(&self) -> &Rdb {
        &self.rdb
    }
    #[doc = "0x30..0x3c - Transmit frame info (Tx Buffer )"]
    #[inline(always)]
    pub const fn tfi(&self, n: usize) -> &Tfi {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(48)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x30..0x3c - Transmit frame info (Tx Buffer )"]
    #[inline(always)]
    pub fn tfi_iter(&self) -> impl Iterator<Item = &Tfi> {
        (0..3).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(48)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0x30 - Transmit frame info (Tx Buffer )"]
    #[inline(always)]
    pub const fn tfi1(&self) -> &Tfi {
        self.tfi(0)
    }
    #[doc = "0x40 - Transmit frame info (Tx Buffer )"]
    #[inline(always)]
    pub const fn tfi2(&self) -> &Tfi {
        self.tfi(1)
    }
    #[doc = "0x50 - Transmit frame info (Tx Buffer )"]
    #[inline(always)]
    pub const fn tfi3(&self) -> &Tfi {
        self.tfi(2)
    }
    #[doc = "0x34..0x40 - Transmit Identifier (Tx Buffer)"]
    #[inline(always)]
    pub const fn tid(&self, n: usize) -> &Tid {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(52)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x34..0x40 - Transmit Identifier (Tx Buffer)"]
    #[inline(always)]
    pub fn tid_iter(&self) -> impl Iterator<Item = &Tid> {
        (0..3).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(52)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0x34 - Transmit Identifier (Tx Buffer)"]
    #[inline(always)]
    pub const fn tid1(&self) -> &Tid {
        self.tid(0)
    }
    #[doc = "0x44 - Transmit Identifier (Tx Buffer)"]
    #[inline(always)]
    pub const fn tid2(&self) -> &Tid {
        self.tid(1)
    }
    #[doc = "0x54 - Transmit Identifier (Tx Buffer)"]
    #[inline(always)]
    pub const fn tid3(&self) -> &Tid {
        self.tid(2)
    }
    #[doc = "0x38..0x44 - Transmit data bytes 1-4 (Tx Buffer)"]
    #[inline(always)]
    pub const fn tda(&self, n: usize) -> &Tda {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(56)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x38..0x44 - Transmit data bytes 1-4 (Tx Buffer)"]
    #[inline(always)]
    pub fn tda_iter(&self) -> impl Iterator<Item = &Tda> {
        (0..3).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(56)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0x38 - Transmit data bytes 1-4 (Tx Buffer)"]
    #[inline(always)]
    pub const fn tda1(&self) -> &Tda {
        self.tda(0)
    }
    #[doc = "0x48 - Transmit data bytes 1-4 (Tx Buffer)"]
    #[inline(always)]
    pub const fn tda2(&self) -> &Tda {
        self.tda(1)
    }
    #[doc = "0x58 - Transmit data bytes 1-4 (Tx Buffer)"]
    #[inline(always)]
    pub const fn tda3(&self) -> &Tda {
        self.tda(2)
    }
    #[doc = "0x3c..0x48 - Transmit data bytes 5-8 (Tx Buffer )"]
    #[inline(always)]
    pub const fn tdb(&self, n: usize) -> &Tdb {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(60)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x3c..0x48 - Transmit data bytes 5-8 (Tx Buffer )"]
    #[inline(always)]
    pub fn tdb_iter(&self) -> impl Iterator<Item = &Tdb> {
        (0..3).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(60)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0x3c - Transmit data bytes 5-8 (Tx Buffer )"]
    #[inline(always)]
    pub const fn tdb1(&self) -> &Tdb {
        self.tdb(0)
    }
    #[doc = "0x4c - Transmit data bytes 5-8 (Tx Buffer )"]
    #[inline(always)]
    pub const fn tdb2(&self) -> &Tdb {
        self.tdb(1)
    }
    #[doc = "0x5c - Transmit data bytes 5-8 (Tx Buffer )"]
    #[inline(always)]
    pub const fn tdb3(&self) -> &Tdb {
        self.tdb(2)
    }
}
#[doc = "MOD (rw) register accessor: Controls the operating mode of the CAN Controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`mod_::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mod_::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mod_`]
module"]
#[doc(alias = "MOD")]
pub type Mod = crate::Reg<mod_::ModSpec>;
#[doc = "Controls the operating mode of the CAN Controller."]
pub mod mod_;
#[doc = "CMR (w) register accessor: Command bits that affect the state of the CAN Controller\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmr`]
module"]
#[doc(alias = "CMR")]
pub type Cmr = crate::Reg<cmr::CmrSpec>;
#[doc = "Command bits that affect the state of the CAN Controller"]
pub mod cmr;
#[doc = "GSR (r) register accessor: Global Controller Status and Error Counters. The error counters can only be written when RM in CANMOD is 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`gsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsr`]
module"]
#[doc(alias = "GSR")]
pub type Gsr = crate::Reg<gsr::GsrSpec>;
#[doc = "Global Controller Status and Error Counters. The error counters can only be written when RM in CANMOD is 1."]
pub mod gsr;
#[doc = "ICR (r) register accessor: Interrupt status, Arbitration Lost Capture, Error Code Capture\n\nYou can [`read`](crate::Reg::read) this register and get [`icr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`]
module"]
#[doc(alias = "ICR")]
pub type Icr = crate::Reg<icr::IcrSpec>;
#[doc = "Interrupt status, Arbitration Lost Capture, Error Code Capture"]
pub mod icr;
#[doc = "IER (rw) register accessor: Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`]
module"]
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IerSpec>;
#[doc = "Interrupt Enable"]
pub mod ier;
#[doc = "BTR (rw) register accessor: Bus Timing. Can only be written when RM in CANMOD is 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`btr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`btr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@btr`]
module"]
#[doc(alias = "BTR")]
pub type Btr = crate::Reg<btr::BtrSpec>;
#[doc = "Bus Timing. Can only be written when RM in CANMOD is 1."]
pub mod btr;
#[doc = "EWL (rw) register accessor: Error Warning Limit. Can only be written when RM in CANMOD is 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`ewl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ewl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ewl`]
module"]
#[doc(alias = "EWL")]
pub type Ewl = crate::Reg<ewl::EwlSpec>;
#[doc = "Error Warning Limit. Can only be written when RM in CANMOD is 1."]
pub mod ewl;
#[doc = "SR (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "Status Register"]
pub mod sr;
#[doc = "RFS (rw) register accessor: Receive frame status. Can only be written when RM in CANMOD is 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`rfs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfs`]
module"]
#[doc(alias = "RFS")]
pub type Rfs = crate::Reg<rfs::RfsSpec>;
#[doc = "Receive frame status. Can only be written when RM in CANMOD is 1."]
pub mod rfs;
#[doc = "RID (rw) register accessor: Received Identifier. Can only be written when RM in CANMOD is 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`rid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rid`]
module"]
#[doc(alias = "RID")]
pub type Rid = crate::Reg<rid::RidSpec>;
#[doc = "Received Identifier. Can only be written when RM in CANMOD is 1."]
pub mod rid;
#[doc = "RDA (rw) register accessor: Received data bytes 1-4. Can only be written when RM in CANMOD is 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`rda::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rda::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rda`]
module"]
#[doc(alias = "RDA")]
pub type Rda = crate::Reg<rda::RdaSpec>;
#[doc = "Received data bytes 1-4. Can only be written when RM in CANMOD is 1."]
pub mod rda;
#[doc = "RDB (rw) register accessor: Received data bytes 5-8. Can only be written when RM in CANMOD is 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`rdb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rdb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdb`]
module"]
#[doc(alias = "RDB")]
pub type Rdb = crate::Reg<rdb::RdbSpec>;
#[doc = "Received data bytes 5-8. Can only be written when RM in CANMOD is 1."]
pub mod rdb;
#[doc = "TFI (rw) register accessor: Transmit frame info (Tx Buffer )\n\nYou can [`read`](crate::Reg::read) this register and get [`tfi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tfi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tfi`]
module"]
#[doc(alias = "TFI")]
pub type Tfi = crate::Reg<tfi::TfiSpec>;
#[doc = "Transmit frame info (Tx Buffer )"]
pub mod tfi;
#[doc = "TID (rw) register accessor: Transmit Identifier (Tx Buffer)\n\nYou can [`read`](crate::Reg::read) this register and get [`tid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tid`]
module"]
#[doc(alias = "TID")]
pub type Tid = crate::Reg<tid::TidSpec>;
#[doc = "Transmit Identifier (Tx Buffer)"]
pub mod tid;
#[doc = "TDA (rw) register accessor: Transmit data bytes 1-4 (Tx Buffer)\n\nYou can [`read`](crate::Reg::read) this register and get [`tda::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tda::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tda`]
module"]
#[doc(alias = "TDA")]
pub type Tda = crate::Reg<tda::TdaSpec>;
#[doc = "Transmit data bytes 1-4 (Tx Buffer)"]
pub mod tda;
#[doc = "TDB (rw) register accessor: Transmit data bytes 5-8 (Tx Buffer )\n\nYou can [`read`](crate::Reg::read) this register and get [`tdb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdb`]
module"]
#[doc(alias = "TDB")]
pub type Tdb = crate::Reg<tdb::TdbSpec>;
#[doc = "Transmit data bytes 5-8 (Tx Buffer )"]
pub mod tdb;
