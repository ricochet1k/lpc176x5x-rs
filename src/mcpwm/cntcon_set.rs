#[doc = "Register `CNTCON_SET` writer"]
pub type W = crate::W<CntconSetSpec>;
#[doc = "Field `TC0MCI0_RE_SET` writer - Writing a one sets the corresponding bit in the CNTCON register."]
pub type Tc0mci0ReSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC0MCI0_FE_SET` writer - Writing a one sets the corresponding bit in the CNTCON register."]
pub type Tc0mci0FeSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC0MCI1_RE_SET` writer - Writing a one sets the corresponding bit in the CNTCON register."]
pub type Tc0mci1ReSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC0MCI1_FE_SET` writer - Writing a one sets the corresponding bit in the CNTCON register."]
pub type Tc0mci1FeSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC0MCI2_RE_SET` writer - Writing a one sets the corresponding bit in the CNTCON register."]
pub type Tc0mci2ReSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC0MCI2_FE_SET` writer - Writing a one sets the corresponding bit in the CNTCON register."]
pub type Tc0mci2FeSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC1MCI0_RE_SET` writer - Writing a one sets the corresponding bit in the CNTCON register."]
pub type Tc1mci0ReSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC1MCI0_FE_SET` writer - Writing a one sets the corresponding bit in the CNTCON register."]
pub type Tc1mci0FeSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC1MCI1_RE_SET` writer - Writing a one sets the corresponding bit in the CNTCON register."]
pub type Tc1mci1ReSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC1MCI1_FE_SET` writer - Writing a one sets the corresponding bit in the CNTCON register."]
pub type Tc1mci1FeSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC1MCI2_RE_SET` writer - Writing a one sets the corresponding bit in the CNTCON register."]
pub type Tc1mci2ReSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC1MCI2_FE_SET` writer - Writing a one sets the corresponding bit in the CNTCON register."]
pub type Tc1mci2FeSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC2MCI0_RE_SET` writer - Writing a one sets the corresponding bit in the CNTCON register."]
pub type Tc2mci0ReSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC2MCI0_FE_SET` writer - Writing a one sets the corresponding bit in the CNTCON register."]
pub type Tc2mci0FeSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC2MCI1_RE_SET` writer - Writing a one sets the corresponding bit in the CNTCON register."]
pub type Tc2mci1ReSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC2MCI1_FE_SET` writer - Writing a one sets the corresponding bit in the CNTCON register."]
pub type Tc2mci1FeSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC2MCI2_RE_SET` writer - Writing a one sets the corresponding bit in the CNTCON register."]
pub type Tc2mci2ReSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC2MCI2_FE_SET` writer - Writing a one sets the corresponding bit in the CNTCON register."]
pub type Tc2mci2FeSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTR0_SET` writer - Writing a one sets the corresponding bit in the CNTCON register."]
pub type Cntr0SetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTR1_SET` writer - Writing a one sets the corresponding bit in the CNTCON register."]
pub type Cntr1SetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTR2_SET` writer - Writing a one sets the corresponding bit in the CNTCON register."]
pub type Cntr2SetW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    #[must_use]
    pub fn tc0mci0_re_set(&mut self) -> Tc0mci0ReSetW<CntconSetSpec> {
        Tc0mci0ReSetW::new(self, 0)
    }
    #[doc = "Bit 1 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    #[must_use]
    pub fn tc0mci0_fe_set(&mut self) -> Tc0mci0FeSetW<CntconSetSpec> {
        Tc0mci0FeSetW::new(self, 1)
    }
    #[doc = "Bit 2 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    #[must_use]
    pub fn tc0mci1_re_set(&mut self) -> Tc0mci1ReSetW<CntconSetSpec> {
        Tc0mci1ReSetW::new(self, 2)
    }
    #[doc = "Bit 3 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    #[must_use]
    pub fn tc0mci1_fe_set(&mut self) -> Tc0mci1FeSetW<CntconSetSpec> {
        Tc0mci1FeSetW::new(self, 3)
    }
    #[doc = "Bit 4 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    #[must_use]
    pub fn tc0mci2_re_set(&mut self) -> Tc0mci2ReSetW<CntconSetSpec> {
        Tc0mci2ReSetW::new(self, 4)
    }
    #[doc = "Bit 5 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    #[must_use]
    pub fn tc0mci2_fe_set(&mut self) -> Tc0mci2FeSetW<CntconSetSpec> {
        Tc0mci2FeSetW::new(self, 5)
    }
    #[doc = "Bit 6 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    #[must_use]
    pub fn tc1mci0_re_set(&mut self) -> Tc1mci0ReSetW<CntconSetSpec> {
        Tc1mci0ReSetW::new(self, 6)
    }
    #[doc = "Bit 7 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    #[must_use]
    pub fn tc1mci0_fe_set(&mut self) -> Tc1mci0FeSetW<CntconSetSpec> {
        Tc1mci0FeSetW::new(self, 7)
    }
    #[doc = "Bit 8 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    #[must_use]
    pub fn tc1mci1_re_set(&mut self) -> Tc1mci1ReSetW<CntconSetSpec> {
        Tc1mci1ReSetW::new(self, 8)
    }
    #[doc = "Bit 9 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    #[must_use]
    pub fn tc1mci1_fe_set(&mut self) -> Tc1mci1FeSetW<CntconSetSpec> {
        Tc1mci1FeSetW::new(self, 9)
    }
    #[doc = "Bit 10 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    #[must_use]
    pub fn tc1mci2_re_set(&mut self) -> Tc1mci2ReSetW<CntconSetSpec> {
        Tc1mci2ReSetW::new(self, 10)
    }
    #[doc = "Bit 11 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    #[must_use]
    pub fn tc1mci2_fe_set(&mut self) -> Tc1mci2FeSetW<CntconSetSpec> {
        Tc1mci2FeSetW::new(self, 11)
    }
    #[doc = "Bit 12 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    #[must_use]
    pub fn tc2mci0_re_set(&mut self) -> Tc2mci0ReSetW<CntconSetSpec> {
        Tc2mci0ReSetW::new(self, 12)
    }
    #[doc = "Bit 13 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    #[must_use]
    pub fn tc2mci0_fe_set(&mut self) -> Tc2mci0FeSetW<CntconSetSpec> {
        Tc2mci0FeSetW::new(self, 13)
    }
    #[doc = "Bit 14 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    #[must_use]
    pub fn tc2mci1_re_set(&mut self) -> Tc2mci1ReSetW<CntconSetSpec> {
        Tc2mci1ReSetW::new(self, 14)
    }
    #[doc = "Bit 15 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    #[must_use]
    pub fn tc2mci1_fe_set(&mut self) -> Tc2mci1FeSetW<CntconSetSpec> {
        Tc2mci1FeSetW::new(self, 15)
    }
    #[doc = "Bit 16 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    #[must_use]
    pub fn tc2mci2_re_set(&mut self) -> Tc2mci2ReSetW<CntconSetSpec> {
        Tc2mci2ReSetW::new(self, 16)
    }
    #[doc = "Bit 17 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    #[must_use]
    pub fn tc2mci2_fe_set(&mut self) -> Tc2mci2FeSetW<CntconSetSpec> {
        Tc2mci2FeSetW::new(self, 17)
    }
    #[doc = "Bit 29 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    #[must_use]
    pub fn cntr0_set(&mut self) -> Cntr0SetW<CntconSetSpec> {
        Cntr0SetW::new(self, 29)
    }
    #[doc = "Bit 30 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    #[must_use]
    pub fn cntr1_set(&mut self) -> Cntr1SetW<CntconSetSpec> {
        Cntr1SetW::new(self, 30)
    }
    #[doc = "Bit 31 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    #[must_use]
    pub fn cntr2_set(&mut self) -> Cntr2SetW<CntconSetSpec> {
        Cntr2SetW::new(self, 31)
    }
}
#[doc = "Count Control set address\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntcon_set::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CntconSetSpec;
impl crate::RegisterSpec for CntconSetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cntcon_set::W`](W) writer structure"]
impl crate::Writable for CntconSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CNTCON_SET to value 0"]
impl crate::Resettable for CntconSetSpec {
    const RESET_VALUE: u32 = 0;
}
