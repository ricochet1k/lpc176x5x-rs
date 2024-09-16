#[doc = "Register `CON_CLR` writer"]
pub type W = crate::W<ConClrSpec>;
#[doc = "Field `RUN0_CLR` writer - Writing a one clears the corresponding bit in the CON register."]
pub type Run0ClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CENTER0_CLR` writer - Writing a one clears the corresponding bit in the CON register."]
pub type Center0ClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POLA0_CLR` writer - Writing a one clears the corresponding bit in the CON register."]
pub type Pola0ClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTE0_CLR` writer - Writing a one clears the corresponding bit in the CON register."]
pub type Dte0ClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISUP0_CLR` writer - Writing a one clears the corresponding bit in the CON register."]
pub type Disup0ClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RUN1_CLR` writer - Writing a one clears the corresponding bit in the CON register."]
pub type Run1ClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CENTER1_CLR` writer - Writing a one clears the corresponding bit in the CON register."]
pub type Center1ClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POLA1_CLR` writer - Writing a one clears the corresponding bit in the CON register."]
pub type Pola1ClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTE1_CLR` writer - Writing a one clears the corresponding bit in the CON register."]
pub type Dte1ClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISUP1_CLR` writer - Writing a one clears the corresponding bit in the CON register."]
pub type Disup1ClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RUN2_CLR` writer - Writing a one clears the corresponding bit in the CON register."]
pub type Run2ClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CENTER2_CLR` writer - Writing a one clears the corresponding bit in the CON register."]
pub type Center2ClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POLA2_CLR` writer - Writing a one clears the corresponding bit in the CON register."]
pub type Pola2ClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTE2_CLR` writer - Writing a one clears the corresponding bit in the CON register."]
pub type Dte2ClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISUP2_CLR` writer - Writing a one clears the corresponding bit in the CON register."]
pub type Disup2ClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INVBDC_CLR` writer - Writing a one clears the corresponding bit in the CON register."]
pub type InvbdcClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACMOD_CLR` writer - Writing a one clears the corresponding bit in the CON register."]
pub type AcmodClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCMODE_CLR` writer - Writing a one clears the corresponding bit in the CON register."]
pub type DcmodeClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    #[must_use]
    pub fn run0_clr(&mut self) -> Run0ClrW<ConClrSpec> {
        Run0ClrW::new(self, 0)
    }
    #[doc = "Bit 1 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    #[must_use]
    pub fn center0_clr(&mut self) -> Center0ClrW<ConClrSpec> {
        Center0ClrW::new(self, 1)
    }
    #[doc = "Bit 2 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    #[must_use]
    pub fn pola0_clr(&mut self) -> Pola0ClrW<ConClrSpec> {
        Pola0ClrW::new(self, 2)
    }
    #[doc = "Bit 3 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    #[must_use]
    pub fn dte0_clr(&mut self) -> Dte0ClrW<ConClrSpec> {
        Dte0ClrW::new(self, 3)
    }
    #[doc = "Bit 4 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    #[must_use]
    pub fn disup0_clr(&mut self) -> Disup0ClrW<ConClrSpec> {
        Disup0ClrW::new(self, 4)
    }
    #[doc = "Bit 8 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    #[must_use]
    pub fn run1_clr(&mut self) -> Run1ClrW<ConClrSpec> {
        Run1ClrW::new(self, 8)
    }
    #[doc = "Bit 9 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    #[must_use]
    pub fn center1_clr(&mut self) -> Center1ClrW<ConClrSpec> {
        Center1ClrW::new(self, 9)
    }
    #[doc = "Bit 10 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    #[must_use]
    pub fn pola1_clr(&mut self) -> Pola1ClrW<ConClrSpec> {
        Pola1ClrW::new(self, 10)
    }
    #[doc = "Bit 11 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    #[must_use]
    pub fn dte1_clr(&mut self) -> Dte1ClrW<ConClrSpec> {
        Dte1ClrW::new(self, 11)
    }
    #[doc = "Bit 12 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    #[must_use]
    pub fn disup1_clr(&mut self) -> Disup1ClrW<ConClrSpec> {
        Disup1ClrW::new(self, 12)
    }
    #[doc = "Bit 16 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    #[must_use]
    pub fn run2_clr(&mut self) -> Run2ClrW<ConClrSpec> {
        Run2ClrW::new(self, 16)
    }
    #[doc = "Bit 17 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    #[must_use]
    pub fn center2_clr(&mut self) -> Center2ClrW<ConClrSpec> {
        Center2ClrW::new(self, 17)
    }
    #[doc = "Bit 18 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    #[must_use]
    pub fn pola2_clr(&mut self) -> Pola2ClrW<ConClrSpec> {
        Pola2ClrW::new(self, 18)
    }
    #[doc = "Bit 19 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    #[must_use]
    pub fn dte2_clr(&mut self) -> Dte2ClrW<ConClrSpec> {
        Dte2ClrW::new(self, 19)
    }
    #[doc = "Bit 20 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    #[must_use]
    pub fn disup2_clr(&mut self) -> Disup2ClrW<ConClrSpec> {
        Disup2ClrW::new(self, 20)
    }
    #[doc = "Bit 29 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    #[must_use]
    pub fn invbdc_clr(&mut self) -> InvbdcClrW<ConClrSpec> {
        InvbdcClrW::new(self, 29)
    }
    #[doc = "Bit 30 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    #[must_use]
    pub fn acmod_clr(&mut self) -> AcmodClrW<ConClrSpec> {
        AcmodClrW::new(self, 30)
    }
    #[doc = "Bit 31 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    #[must_use]
    pub fn dcmode_clr(&mut self) -> DcmodeClrW<ConClrSpec> {
        DcmodeClrW::new(self, 31)
    }
}
#[doc = "PWM Control clear address\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`con_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConClrSpec;
impl crate::RegisterSpec for ConClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`con_clr::W`](W) writer structure"]
impl crate::Writable for ConClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CON_CLR to value 0"]
impl crate::Resettable for ConClrSpec {
    const RESET_VALUE: u32 = 0;
}
