#[doc = "Register `CTCR` reader"]
pub type R = crate::R<CtcrSpec>;
#[doc = "Register `CTCR` writer"]
pub type W = crate::W<CtcrSpec>;
#[doc = "Counter/ Timer Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mod {
    #[doc = "0: Timer Mode: the TC is incremented when the Prescale Counter matches the Prescale register."]
    TimerModeTheTcI = 0,
    #[doc = "1: Rising edge counter Mode: the TC is incremented on rising edges of the PWM_CAP input selected by bits 3:2."]
    RisingEdgeCounter_ = 1,
    #[doc = "2: Falling edge counter Mode: the TC is incremented on falling edges of the PWM_CAP input selected by bits 3:2."]
    FallingEdgeCounter = 2,
    #[doc = "3: Dual edge counter Mode: the TC is incremented on both edges of the PWM_CAP input selected by bits 3:2."]
    DualEdgeCounterMo = 3,
}
impl From<Mod> for u8 {
    #[inline(always)]
    fn from(variant: Mod) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mod {
    type Ux = u8;
}
impl crate::IsEnum for Mod {}
#[doc = "Field `MOD` reader - Counter/ Timer Mode"]
pub type ModR = crate::FieldReader<Mod>;
impl ModR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mod {
        match self.bits {
            0 => Mod::TimerModeTheTcI,
            1 => Mod::RisingEdgeCounter_,
            2 => Mod::FallingEdgeCounter,
            3 => Mod::DualEdgeCounterMo,
            _ => unreachable!(),
        }
    }
    #[doc = "Timer Mode: the TC is incremented when the Prescale Counter matches the Prescale register."]
    #[inline(always)]
    pub fn is_timer_mode_the_tc_i(&self) -> bool {
        *self == Mod::TimerModeTheTcI
    }
    #[doc = "Rising edge counter Mode: the TC is incremented on rising edges of the PWM_CAP input selected by bits 3:2."]
    #[inline(always)]
    pub fn is_rising_edge_counter_(&self) -> bool {
        *self == Mod::RisingEdgeCounter_
    }
    #[doc = "Falling edge counter Mode: the TC is incremented on falling edges of the PWM_CAP input selected by bits 3:2."]
    #[inline(always)]
    pub fn is_falling_edge_counter(&self) -> bool {
        *self == Mod::FallingEdgeCounter
    }
    #[doc = "Dual edge counter Mode: the TC is incremented on both edges of the PWM_CAP input selected by bits 3:2."]
    #[inline(always)]
    pub fn is_dual_edge_counter_mo(&self) -> bool {
        *self == Mod::DualEdgeCounterMo
    }
}
#[doc = "Field `MOD` writer - Counter/ Timer Mode"]
pub type ModW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mod, crate::Safe>;
impl<'a, REG> ModW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timer Mode: the TC is incremented when the Prescale Counter matches the Prescale register."]
    #[inline(always)]
    pub fn timer_mode_the_tc_i(self) -> &'a mut crate::W<REG> {
        self.variant(Mod::TimerModeTheTcI)
    }
    #[doc = "Rising edge counter Mode: the TC is incremented on rising edges of the PWM_CAP input selected by bits 3:2."]
    #[inline(always)]
    pub fn rising_edge_counter_(self) -> &'a mut crate::W<REG> {
        self.variant(Mod::RisingEdgeCounter_)
    }
    #[doc = "Falling edge counter Mode: the TC is incremented on falling edges of the PWM_CAP input selected by bits 3:2."]
    #[inline(always)]
    pub fn falling_edge_counter(self) -> &'a mut crate::W<REG> {
        self.variant(Mod::FallingEdgeCounter)
    }
    #[doc = "Dual edge counter Mode: the TC is incremented on both edges of the PWM_CAP input selected by bits 3:2."]
    #[inline(always)]
    pub fn dual_edge_counter_mo(self) -> &'a mut crate::W<REG> {
        self.variant(Mod::DualEdgeCounterMo)
    }
}
#[doc = "Count Input Select. When bits 1:0 are not 00, these bits select which PWM_CAP pin carries the signal used to increment the TC. Other combinations are reserved.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cis {
    #[doc = "0: For PWM0: 00 = PWM0_CAP0 (Other combinations are reserved) For PWM1: 00 = PWM1_CAP0, 01 = PWM1_CAP1 (Other combinations are reserved)"]
    ForPwm0_00EqPwm0_ = 0,
}
impl From<Cis> for u8 {
    #[inline(always)]
    fn from(variant: Cis) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cis {
    type Ux = u8;
}
impl crate::IsEnum for Cis {}
#[doc = "Field `CIS` reader - Count Input Select. When bits 1:0 are not 00, these bits select which PWM_CAP pin carries the signal used to increment the TC. Other combinations are reserved."]
pub type CisR = crate::FieldReader<Cis>;
impl CisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cis> {
        match self.bits {
            0 => Some(Cis::ForPwm0_00EqPwm0_),
            _ => None,
        }
    }
    #[doc = "For PWM0: 00 = PWM0_CAP0 (Other combinations are reserved) For PWM1: 00 = PWM1_CAP0, 01 = PWM1_CAP1 (Other combinations are reserved)"]
    #[inline(always)]
    pub fn is_for_pwm0_00_eq_pwm0_(&self) -> bool {
        *self == Cis::ForPwm0_00EqPwm0_
    }
}
#[doc = "Field `CIS` writer - Count Input Select. When bits 1:0 are not 00, these bits select which PWM_CAP pin carries the signal used to increment the TC. Other combinations are reserved."]
pub type CisW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cis>;
impl<'a, REG> CisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "For PWM0: 00 = PWM0_CAP0 (Other combinations are reserved) For PWM1: 00 = PWM1_CAP0, 01 = PWM1_CAP1 (Other combinations are reserved)"]
    #[inline(always)]
    pub fn for_pwm0_00_eq_pwm0_(self) -> &'a mut crate::W<REG> {
        self.variant(Cis::ForPwm0_00EqPwm0_)
    }
}
impl R {
    #[doc = "Bits 0:1 - Counter/ Timer Mode"]
    #[inline(always)]
    pub fn mod_(&self) -> ModR {
        ModR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Count Input Select. When bits 1:0 are not 00, these bits select which PWM_CAP pin carries the signal used to increment the TC. Other combinations are reserved."]
    #[inline(always)]
    pub fn cis(&self) -> CisR {
        CisR::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Counter/ Timer Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mod_(&mut self) -> ModW<CtcrSpec> {
        ModW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Count Input Select. When bits 1:0 are not 00, these bits select which PWM_CAP pin carries the signal used to increment the TC. Other combinations are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn cis(&mut self) -> CisW<CtcrSpec> {
        CisW::new(self, 2)
    }
}
#[doc = "Count Control Register. The CTCR selects between Timer and Counter mode, and in Counter mode selects the signal and edge(s) for counting.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtcrSpec;
impl crate::RegisterSpec for CtcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctcr::R`](R) reader structure"]
impl crate::Readable for CtcrSpec {}
#[doc = "`write(|w| ..)` method takes [`ctcr::W`](W) writer structure"]
impl crate::Writable for CtcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTCR to value 0"]
impl crate::Resettable for CtcrSpec {
    const RESET_VALUE: u32 = 0;
}
