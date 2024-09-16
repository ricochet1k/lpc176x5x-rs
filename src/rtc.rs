#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ilr: Ilr,
    _reserved1: [u8; 0x04],
    ccr: Ccr,
    ciir: Ciir,
    amr: Amr,
    ctime0: Ctime0,
    ctime1: Ctime1,
    ctime2: Ctime2,
    sec: Sec,
    min: Min,
    hrs: Hrs,
    dom: Dom,
    dow: Dow,
    doy: Doy,
    month: Month,
    year: Year,
    calibration: Calibration,
    gpreg: [Gpreg; 5],
    rtc_auxen: RtcAuxen,
    rtc_aux: RtcAux,
    asec: Asec,
    amin: Amin,
    ahrs: Ahrs,
    adom: Adom,
    adow: Adow,
    adoy: Adoy,
    amon: Amon,
    ayrs: Ayrs,
}
impl RegisterBlock {
    #[doc = "0x00 - Interrupt Location Register"]
    #[inline(always)]
    pub const fn ilr(&self) -> &Ilr {
        &self.ilr
    }
    #[doc = "0x08 - Clock Control Register"]
    #[inline(always)]
    pub const fn ccr(&self) -> &Ccr {
        &self.ccr
    }
    #[doc = "0x0c - Counter Increment Interrupt Register"]
    #[inline(always)]
    pub const fn ciir(&self) -> &Ciir {
        &self.ciir
    }
    #[doc = "0x10 - Alarm Mask Register"]
    #[inline(always)]
    pub const fn amr(&self) -> &Amr {
        &self.amr
    }
    #[doc = "0x14 - Consolidated Time Register 0"]
    #[inline(always)]
    pub const fn ctime0(&self) -> &Ctime0 {
        &self.ctime0
    }
    #[doc = "0x18 - Consolidated Time Register 1"]
    #[inline(always)]
    pub const fn ctime1(&self) -> &Ctime1 {
        &self.ctime1
    }
    #[doc = "0x1c - Consolidated Time Register 2"]
    #[inline(always)]
    pub const fn ctime2(&self) -> &Ctime2 {
        &self.ctime2
    }
    #[doc = "0x20 - Seconds Counter"]
    #[inline(always)]
    pub const fn sec(&self) -> &Sec {
        &self.sec
    }
    #[doc = "0x24 - Minutes Register"]
    #[inline(always)]
    pub const fn min(&self) -> &Min {
        &self.min
    }
    #[doc = "0x28 - Hours Register"]
    #[inline(always)]
    pub const fn hrs(&self) -> &Hrs {
        &self.hrs
    }
    #[doc = "0x2c - Day of Month Register"]
    #[inline(always)]
    pub const fn dom(&self) -> &Dom {
        &self.dom
    }
    #[doc = "0x30 - Day of Week Register"]
    #[inline(always)]
    pub const fn dow(&self) -> &Dow {
        &self.dow
    }
    #[doc = "0x34 - Day of Year Register"]
    #[inline(always)]
    pub const fn doy(&self) -> &Doy {
        &self.doy
    }
    #[doc = "0x38 - Months Register"]
    #[inline(always)]
    pub const fn month(&self) -> &Month {
        &self.month
    }
    #[doc = "0x3c - Years Register"]
    #[inline(always)]
    pub const fn year(&self) -> &Year {
        &self.year
    }
    #[doc = "0x40 - Calibration Value Register"]
    #[inline(always)]
    pub const fn calibration(&self) -> &Calibration {
        &self.calibration
    }
    #[doc = "0x44..0x58 - General Purpose Register 0"]
    #[inline(always)]
    pub const fn gpreg(&self, n: usize) -> &Gpreg {
        &self.gpreg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x44..0x58 - General Purpose Register 0"]
    #[inline(always)]
    pub fn gpreg_iter(&self) -> impl Iterator<Item = &Gpreg> {
        self.gpreg.iter()
    }
    #[doc = "0x58 - RTC Auxiliary Enable register"]
    #[inline(always)]
    pub const fn rtc_auxen(&self) -> &RtcAuxen {
        &self.rtc_auxen
    }
    #[doc = "0x5c - RTC Auxiliary control register"]
    #[inline(always)]
    pub const fn rtc_aux(&self) -> &RtcAux {
        &self.rtc_aux
    }
    #[doc = "0x60 - Alarm value for Seconds"]
    #[inline(always)]
    pub const fn asec(&self) -> &Asec {
        &self.asec
    }
    #[doc = "0x64 - Alarm value for Minutes"]
    #[inline(always)]
    pub const fn amin(&self) -> &Amin {
        &self.amin
    }
    #[doc = "0x68 - Alarm value for Hours"]
    #[inline(always)]
    pub const fn ahrs(&self) -> &Ahrs {
        &self.ahrs
    }
    #[doc = "0x6c - Alarm value for Day of Month"]
    #[inline(always)]
    pub const fn adom(&self) -> &Adom {
        &self.adom
    }
    #[doc = "0x70 - Alarm value for Day of Week"]
    #[inline(always)]
    pub const fn adow(&self) -> &Adow {
        &self.adow
    }
    #[doc = "0x74 - Alarm value for Day of Year"]
    #[inline(always)]
    pub const fn adoy(&self) -> &Adoy {
        &self.adoy
    }
    #[doc = "0x78 - Alarm value for Months"]
    #[inline(always)]
    pub const fn amon(&self) -> &Amon {
        &self.amon
    }
    #[doc = "0x7c - Alarm value for Year"]
    #[inline(always)]
    pub const fn ayrs(&self) -> &Ayrs {
        &self.ayrs
    }
}
#[doc = "ILR (rw) register accessor: Interrupt Location Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ilr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ilr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ilr`]
module"]
#[doc(alias = "ILR")]
pub type Ilr = crate::Reg<ilr::IlrSpec>;
#[doc = "Interrupt Location Register"]
pub mod ilr;
#[doc = "CCR (rw) register accessor: Clock Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr`]
module"]
#[doc(alias = "CCR")]
pub type Ccr = crate::Reg<ccr::CcrSpec>;
#[doc = "Clock Control Register"]
pub mod ccr;
#[doc = "CIIR (rw) register accessor: Counter Increment Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ciir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ciir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ciir`]
module"]
#[doc(alias = "CIIR")]
pub type Ciir = crate::Reg<ciir::CiirSpec>;
#[doc = "Counter Increment Interrupt Register"]
pub mod ciir;
#[doc = "AMR (rw) register accessor: Alarm Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`amr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`amr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@amr`]
module"]
#[doc(alias = "AMR")]
pub type Amr = crate::Reg<amr::AmrSpec>;
#[doc = "Alarm Mask Register"]
pub mod amr;
#[doc = "CTIME0 (r) register accessor: Consolidated Time Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ctime0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctime0`]
module"]
#[doc(alias = "CTIME0")]
pub type Ctime0 = crate::Reg<ctime0::Ctime0Spec>;
#[doc = "Consolidated Time Register 0"]
pub mod ctime0;
#[doc = "CTIME1 (r) register accessor: Consolidated Time Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctime1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctime1`]
module"]
#[doc(alias = "CTIME1")]
pub type Ctime1 = crate::Reg<ctime1::Ctime1Spec>;
#[doc = "Consolidated Time Register 1"]
pub mod ctime1;
#[doc = "CTIME2 (r) register accessor: Consolidated Time Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ctime2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctime2`]
module"]
#[doc(alias = "CTIME2")]
pub type Ctime2 = crate::Reg<ctime2::Ctime2Spec>;
#[doc = "Consolidated Time Register 2"]
pub mod ctime2;
#[doc = "SEC (rw) register accessor: Seconds Counter\n\nYou can [`read`](crate::Reg::read) this register and get [`sec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec`]
module"]
#[doc(alias = "SEC")]
pub type Sec = crate::Reg<sec::SecSpec>;
#[doc = "Seconds Counter"]
pub mod sec;
#[doc = "MIN (rw) register accessor: Minutes Register\n\nYou can [`read`](crate::Reg::read) this register and get [`min::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`min::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@min`]
module"]
#[doc(alias = "MIN")]
pub type Min = crate::Reg<min::MinSpec>;
#[doc = "Minutes Register"]
pub mod min;
#[doc = "HRS (rw) register accessor: Hours Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hrs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hrs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hrs`]
module"]
#[doc(alias = "HRS")]
pub type Hrs = crate::Reg<hrs::HrsSpec>;
#[doc = "Hours Register"]
pub mod hrs;
#[doc = "DOM (rw) register accessor: Day of Month Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dom::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dom::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dom`]
module"]
#[doc(alias = "DOM")]
pub type Dom = crate::Reg<dom::DomSpec>;
#[doc = "Day of Month Register"]
pub mod dom;
#[doc = "DOW (rw) register accessor: Day of Week Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dow::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dow::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dow`]
module"]
#[doc(alias = "DOW")]
pub type Dow = crate::Reg<dow::DowSpec>;
#[doc = "Day of Week Register"]
pub mod dow;
#[doc = "DOY (rw) register accessor: Day of Year Register\n\nYou can [`read`](crate::Reg::read) this register and get [`doy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doy`]
module"]
#[doc(alias = "DOY")]
pub type Doy = crate::Reg<doy::DoySpec>;
#[doc = "Day of Year Register"]
pub mod doy;
#[doc = "MONTH (rw) register accessor: Months Register\n\nYou can [`read`](crate::Reg::read) this register and get [`month::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`month::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@month`]
module"]
#[doc(alias = "MONTH")]
pub type Month = crate::Reg<month::MonthSpec>;
#[doc = "Months Register"]
pub mod month;
#[doc = "YEAR (rw) register accessor: Years Register\n\nYou can [`read`](crate::Reg::read) this register and get [`year::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`year::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@year`]
module"]
#[doc(alias = "YEAR")]
pub type Year = crate::Reg<year::YearSpec>;
#[doc = "Years Register"]
pub mod year;
#[doc = "CALIBRATION (rw) register accessor: Calibration Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`calibration::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calibration::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calibration`]
module"]
#[doc(alias = "CALIBRATION")]
pub type Calibration = crate::Reg<calibration::CalibrationSpec>;
#[doc = "Calibration Value Register"]
pub mod calibration;
#[doc = "GPREG (rw) register accessor: General Purpose Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`gpreg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpreg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpreg`]
module"]
#[doc(alias = "GPREG")]
pub type Gpreg = crate::Reg<gpreg::GpregSpec>;
#[doc = "General Purpose Register 0"]
pub mod gpreg;
#[doc = "RTC_AUX (rw) register accessor: RTC Auxiliary control register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_aux::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_aux::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_aux`]
module"]
#[doc(alias = "RTC_AUX")]
pub type RtcAux = crate::Reg<rtc_aux::RtcAuxSpec>;
#[doc = "RTC Auxiliary control register"]
pub mod rtc_aux;
#[doc = "RTC_AUXEN (rw) register accessor: RTC Auxiliary Enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_auxen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_auxen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_auxen`]
module"]
#[doc(alias = "RTC_AUXEN")]
pub type RtcAuxen = crate::Reg<rtc_auxen::RtcAuxenSpec>;
#[doc = "RTC Auxiliary Enable register"]
pub mod rtc_auxen;
#[doc = "ASEC (rw) register accessor: Alarm value for Seconds\n\nYou can [`read`](crate::Reg::read) this register and get [`asec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`asec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@asec`]
module"]
#[doc(alias = "ASEC")]
pub type Asec = crate::Reg<asec::AsecSpec>;
#[doc = "Alarm value for Seconds"]
pub mod asec;
#[doc = "AMIN (rw) register accessor: Alarm value for Minutes\n\nYou can [`read`](crate::Reg::read) this register and get [`amin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`amin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@amin`]
module"]
#[doc(alias = "AMIN")]
pub type Amin = crate::Reg<amin::AminSpec>;
#[doc = "Alarm value for Minutes"]
pub mod amin;
#[doc = "AHRS (rw) register accessor: Alarm value for Hours\n\nYou can [`read`](crate::Reg::read) this register and get [`ahrs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahrs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahrs`]
module"]
#[doc(alias = "AHRS")]
pub type Ahrs = crate::Reg<ahrs::AhrsSpec>;
#[doc = "Alarm value for Hours"]
pub mod ahrs;
#[doc = "ADOM (rw) register accessor: Alarm value for Day of Month\n\nYou can [`read`](crate::Reg::read) this register and get [`adom::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adom::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adom`]
module"]
#[doc(alias = "ADOM")]
pub type Adom = crate::Reg<adom::AdomSpec>;
#[doc = "Alarm value for Day of Month"]
pub mod adom;
#[doc = "ADOW (rw) register accessor: Alarm value for Day of Week\n\nYou can [`read`](crate::Reg::read) this register and get [`adow::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adow::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adow`]
module"]
#[doc(alias = "ADOW")]
pub type Adow = crate::Reg<adow::AdowSpec>;
#[doc = "Alarm value for Day of Week"]
pub mod adow;
#[doc = "ADOY (rw) register accessor: Alarm value for Day of Year\n\nYou can [`read`](crate::Reg::read) this register and get [`adoy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adoy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adoy`]
module"]
#[doc(alias = "ADOY")]
pub type Adoy = crate::Reg<adoy::AdoySpec>;
#[doc = "Alarm value for Day of Year"]
pub mod adoy;
#[doc = "AMON (rw) register accessor: Alarm value for Months\n\nYou can [`read`](crate::Reg::read) this register and get [`amon::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`amon::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@amon`]
module"]
#[doc(alias = "AMON")]
pub type Amon = crate::Reg<amon::AmonSpec>;
#[doc = "Alarm value for Months"]
pub mod amon;
#[doc = "AYRS (rw) register accessor: Alarm value for Year\n\nYou can [`read`](crate::Reg::read) this register and get [`ayrs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ayrs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ayrs`]
module"]
#[doc(alias = "AYRS")]
pub type Ayrs = crate::Reg<ayrs::AyrsSpec>;
#[doc = "Alarm value for Year"]
pub mod ayrs;
