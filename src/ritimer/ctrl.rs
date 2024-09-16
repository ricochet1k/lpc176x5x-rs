#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ritint {
    #[doc = "1: This bit is set to 1 by hardware whenever the counter value equals the masked compare value specified by the contents of RICOMPVAL and RIMASK registers. Writing a 1 to this bit will clear it to 0. Writing a 0 has no effect."]
    ThisBitIsSetTo1 = 1,
    #[doc = "0: The counter value does not equal the masked compare value."]
    TheCounterValueDo = 0,
}
impl From<Ritint> for bool {
    #[inline(always)]
    fn from(variant: Ritint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RITINT` reader - Interrupt flag"]
pub type RitintR = crate::BitReader<Ritint>;
impl RitintR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ritint {
        match self.bits {
            true => Ritint::ThisBitIsSetTo1,
            false => Ritint::TheCounterValueDo,
        }
    }
    #[doc = "This bit is set to 1 by hardware whenever the counter value equals the masked compare value specified by the contents of RICOMPVAL and RIMASK registers. Writing a 1 to this bit will clear it to 0. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn is_this_bit_is_set_to_1(&self) -> bool {
        *self == Ritint::ThisBitIsSetTo1
    }
    #[doc = "The counter value does not equal the masked compare value."]
    #[inline(always)]
    pub fn is_the_counter_value_do(&self) -> bool {
        *self == Ritint::TheCounterValueDo
    }
}
#[doc = "Field `RITINT` writer - Interrupt flag"]
pub type RitintW<'a, REG> = crate::BitWriter<'a, REG, Ritint>;
impl<'a, REG> RitintW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "This bit is set to 1 by hardware whenever the counter value equals the masked compare value specified by the contents of RICOMPVAL and RIMASK registers. Writing a 1 to this bit will clear it to 0. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn this_bit_is_set_to_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ritint::ThisBitIsSetTo1)
    }
    #[doc = "The counter value does not equal the masked compare value."]
    #[inline(always)]
    pub fn the_counter_value_do(self) -> &'a mut crate::W<REG> {
        self.variant(Ritint::TheCounterValueDo)
    }
}
#[doc = "Timer enable clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ritenclr {
    #[doc = "1: The timer will be cleared to 0 whenever the counter value equals the masked compare value specified by the contents of RICOMPVAL and RIMASK registers. This will occur on the same clock that sets the interrupt flag."]
    TheTimerWillBeCl = 1,
    #[doc = "0: The timer will not be cleared to 0."]
    TheTimerWillNotB = 0,
}
impl From<Ritenclr> for bool {
    #[inline(always)]
    fn from(variant: Ritenclr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RITENCLR` reader - Timer enable clear"]
pub type RitenclrR = crate::BitReader<Ritenclr>;
impl RitenclrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ritenclr {
        match self.bits {
            true => Ritenclr::TheTimerWillBeCl,
            false => Ritenclr::TheTimerWillNotB,
        }
    }
    #[doc = "The timer will be cleared to 0 whenever the counter value equals the masked compare value specified by the contents of RICOMPVAL and RIMASK registers. This will occur on the same clock that sets the interrupt flag."]
    #[inline(always)]
    pub fn is_the_timer_will_be_cl(&self) -> bool {
        *self == Ritenclr::TheTimerWillBeCl
    }
    #[doc = "The timer will not be cleared to 0."]
    #[inline(always)]
    pub fn is_the_timer_will_not_b(&self) -> bool {
        *self == Ritenclr::TheTimerWillNotB
    }
}
#[doc = "Field `RITENCLR` writer - Timer enable clear"]
pub type RitenclrW<'a, REG> = crate::BitWriter<'a, REG, Ritenclr>;
impl<'a, REG> RitenclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The timer will be cleared to 0 whenever the counter value equals the masked compare value specified by the contents of RICOMPVAL and RIMASK registers. This will occur on the same clock that sets the interrupt flag."]
    #[inline(always)]
    pub fn the_timer_will_be_cl(self) -> &'a mut crate::W<REG> {
        self.variant(Ritenclr::TheTimerWillBeCl)
    }
    #[doc = "The timer will not be cleared to 0."]
    #[inline(always)]
    pub fn the_timer_will_not_b(self) -> &'a mut crate::W<REG> {
        self.variant(Ritenclr::TheTimerWillNotB)
    }
}
#[doc = "Timer enable for debug\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ritenbr {
    #[doc = "1: The timer is halted when the processor is halted for debugging."]
    TheTimerIsHalted_ = 1,
    #[doc = "0: Debug has no effect on the timer operation."]
    DebugHasNoEffect_ = 0,
}
impl From<Ritenbr> for bool {
    #[inline(always)]
    fn from(variant: Ritenbr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RITENBR` reader - Timer enable for debug"]
pub type RitenbrR = crate::BitReader<Ritenbr>;
impl RitenbrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ritenbr {
        match self.bits {
            true => Ritenbr::TheTimerIsHalted_,
            false => Ritenbr::DebugHasNoEffect_,
        }
    }
    #[doc = "The timer is halted when the processor is halted for debugging."]
    #[inline(always)]
    pub fn is_the_timer_is_halted_(&self) -> bool {
        *self == Ritenbr::TheTimerIsHalted_
    }
    #[doc = "Debug has no effect on the timer operation."]
    #[inline(always)]
    pub fn is_debug_has_no_effect_(&self) -> bool {
        *self == Ritenbr::DebugHasNoEffect_
    }
}
#[doc = "Field `RITENBR` writer - Timer enable for debug"]
pub type RitenbrW<'a, REG> = crate::BitWriter<'a, REG, Ritenbr>;
impl<'a, REG> RitenbrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The timer is halted when the processor is halted for debugging."]
    #[inline(always)]
    pub fn the_timer_is_halted_(self) -> &'a mut crate::W<REG> {
        self.variant(Ritenbr::TheTimerIsHalted_)
    }
    #[doc = "Debug has no effect on the timer operation."]
    #[inline(always)]
    pub fn debug_has_no_effect_(self) -> &'a mut crate::W<REG> {
        self.variant(Ritenbr::DebugHasNoEffect_)
    }
}
#[doc = "Timer enable.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Riten {
    #[doc = "1: Timer enabled. This can be overruled by a debug halt if enabled in bit 2."]
    TimerEnabledThis_ = 1,
    #[doc = "0: Timer disabled."]
    TimerDisabled_ = 0,
}
impl From<Riten> for bool {
    #[inline(always)]
    fn from(variant: Riten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RITEN` reader - Timer enable."]
pub type RitenR = crate::BitReader<Riten>;
impl RitenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Riten {
        match self.bits {
            true => Riten::TimerEnabledThis_,
            false => Riten::TimerDisabled_,
        }
    }
    #[doc = "Timer enabled. This can be overruled by a debug halt if enabled in bit 2."]
    #[inline(always)]
    pub fn is_timer_enabled_this_(&self) -> bool {
        *self == Riten::TimerEnabledThis_
    }
    #[doc = "Timer disabled."]
    #[inline(always)]
    pub fn is_timer_disabled_(&self) -> bool {
        *self == Riten::TimerDisabled_
    }
}
#[doc = "Field `RITEN` writer - Timer enable."]
pub type RitenW<'a, REG> = crate::BitWriter<'a, REG, Riten>;
impl<'a, REG> RitenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timer enabled. This can be overruled by a debug halt if enabled in bit 2."]
    #[inline(always)]
    pub fn timer_enabled_this_(self) -> &'a mut crate::W<REG> {
        self.variant(Riten::TimerEnabledThis_)
    }
    #[doc = "Timer disabled."]
    #[inline(always)]
    pub fn timer_disabled_(self) -> &'a mut crate::W<REG> {
        self.variant(Riten::TimerDisabled_)
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt flag"]
    #[inline(always)]
    pub fn ritint(&self) -> RitintR {
        RitintR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer enable clear"]
    #[inline(always)]
    pub fn ritenclr(&self) -> RitenclrR {
        RitenclrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer enable for debug"]
    #[inline(always)]
    pub fn ritenbr(&self) -> RitenbrR {
        RitenbrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer enable."]
    #[inline(always)]
    pub fn riten(&self) -> RitenR {
        RitenR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn ritint(&mut self) -> RitintW<CtrlSpec> {
        RitintW::new(self, 0)
    }
    #[doc = "Bit 1 - Timer enable clear"]
    #[inline(always)]
    #[must_use]
    pub fn ritenclr(&mut self) -> RitenclrW<CtrlSpec> {
        RitenclrW::new(self, 1)
    }
    #[doc = "Bit 2 - Timer enable for debug"]
    #[inline(always)]
    #[must_use]
    pub fn ritenbr(&mut self) -> RitenbrW<CtrlSpec> {
        RitenbrW::new(self, 2)
    }
    #[doc = "Bit 3 - Timer enable."]
    #[inline(always)]
    #[must_use]
    pub fn riten(&mut self) -> RitenW<CtrlSpec> {
        RitenW::new(self, 3)
    }
}
#[doc = "Control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0x0c"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0x0c;
}
