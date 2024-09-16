#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mod_: Mod,
    tc: Tc,
    feed: Feed,
    tv: Tv,
    clksel: Clksel,
}
impl RegisterBlock {
    #[doc = "0x00 - Watchdog mode register. This register determines the basic mode and status of the Watchdog Timer."]
    #[inline(always)]
    pub const fn mod_(&self) -> &Mod {
        &self.mod_
    }
    #[doc = "0x04 - Watchdog timer constant register. The value in this register determines the time-out value."]
    #[inline(always)]
    pub const fn tc(&self) -> &Tc {
        &self.tc
    }
    #[doc = "0x08 - Watchdog feed sequence register. Writing 0xAA followed by 0x55 to this register reloads the Watchdog timer with the value contained in WDTC."]
    #[inline(always)]
    pub const fn feed(&self) -> &Feed {
        &self.feed
    }
    #[doc = "0x0c - Watchdog timer value register. This register reads out the current value of the Watchdog timer."]
    #[inline(always)]
    pub const fn tv(&self) -> &Tv {
        &self.tv
    }
    #[doc = "0x10 - Watchdog clock select register."]
    #[inline(always)]
    pub const fn clksel(&self) -> &Clksel {
        &self.clksel
    }
}
#[doc = "MOD (rw) register accessor: Watchdog mode register. This register determines the basic mode and status of the Watchdog Timer.\n\nYou can [`read`](crate::Reg::read) this register and get [`mod_::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mod_::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mod_`]
module"]
#[doc(alias = "MOD")]
pub type Mod = crate::Reg<mod_::ModSpec>;
#[doc = "Watchdog mode register. This register determines the basic mode and status of the Watchdog Timer."]
pub mod mod_;
#[doc = "TC (rw) register accessor: Watchdog timer constant register. The value in this register determines the time-out value.\n\nYou can [`read`](crate::Reg::read) this register and get [`tc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tc`]
module"]
#[doc(alias = "TC")]
pub type Tc = crate::Reg<tc::TcSpec>;
#[doc = "Watchdog timer constant register. The value in this register determines the time-out value."]
pub mod tc;
#[doc = "FEED (w) register accessor: Watchdog feed sequence register. Writing 0xAA followed by 0x55 to this register reloads the Watchdog timer with the value contained in WDTC.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`feed::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@feed`]
module"]
#[doc(alias = "FEED")]
pub type Feed = crate::Reg<feed::FeedSpec>;
#[doc = "Watchdog feed sequence register. Writing 0xAA followed by 0x55 to this register reloads the Watchdog timer with the value contained in WDTC."]
pub mod feed;
#[doc = "TV (r) register accessor: Watchdog timer value register. This register reads out the current value of the Watchdog timer.\n\nYou can [`read`](crate::Reg::read) this register and get [`tv::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tv`]
module"]
#[doc(alias = "TV")]
pub type Tv = crate::Reg<tv::TvSpec>;
#[doc = "Watchdog timer value register. This register reads out the current value of the Watchdog timer."]
pub mod tv;
#[doc = "CLKSEL (rw) register accessor: Watchdog clock select register.\n\nYou can [`read`](crate::Reg::read) this register and get [`clksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel`]
module"]
#[doc(alias = "CLKSEL")]
pub type Clksel = crate::Reg<clksel::ClkselSpec>;
#[doc = "Watchdog clock select register."]
pub mod clksel;
