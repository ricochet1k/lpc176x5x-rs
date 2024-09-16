#[doc = "Register `CLKSEL` reader"]
pub type R = crate::R<ClkselSpec>;
#[doc = "Register `CLKSEL` writer"]
pub type W = crate::W<ClkselSpec>;
#[doc = "Selects source of WDT clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clksel {
    #[doc = "0: IRC"]
    Irc = 0,
    #[doc = "1: Peripheral clock"]
    Pclk = 1,
    #[doc = "2: RTC oscillator"]
    Rtcosc = 2,
    #[doc = "3: Reserved."]
    Reserved = 3,
}
impl From<Clksel> for u8 {
    #[inline(always)]
    fn from(variant: Clksel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clksel {
    type Ux = u8;
}
impl crate::IsEnum for Clksel {}
#[doc = "Field `CLKSEL` reader - Selects source of WDT clock"]
pub type ClkselR = crate::FieldReader<Clksel>;
impl ClkselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clksel {
        match self.bits {
            0 => Clksel::Irc,
            1 => Clksel::Pclk,
            2 => Clksel::Rtcosc,
            3 => Clksel::Reserved,
            _ => unreachable!(),
        }
    }
    #[doc = "IRC"]
    #[inline(always)]
    pub fn is_irc(&self) -> bool {
        *self == Clksel::Irc
    }
    #[doc = "Peripheral clock"]
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        *self == Clksel::Pclk
    }
    #[doc = "RTC oscillator"]
    #[inline(always)]
    pub fn is_rtcosc(&self) -> bool {
        *self == Clksel::Rtcosc
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn is_reserved(&self) -> bool {
        *self == Clksel::Reserved
    }
}
#[doc = "Field `CLKSEL` writer - Selects source of WDT clock"]
pub type ClkselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Clksel, crate::Safe>;
impl<'a, REG> ClkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "IRC"]
    #[inline(always)]
    pub fn irc(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Irc)
    }
    #[doc = "Peripheral clock"]
    #[inline(always)]
    pub fn pclk(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Pclk)
    }
    #[doc = "RTC oscillator"]
    #[inline(always)]
    pub fn rtcosc(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Rtcosc)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn reserved(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Reserved)
    }
}
#[doc = "If this bit is set to one writing to this register does not affect bit 0. The clock source can only be changed by first clearing this bit, then writing the new value of bit 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lock {
    #[doc = "0: This bit is set to 0 on any reset. It cannot be cleared by software."]
    Unlocked = 0,
    #[doc = "1: Software can set this bit to 1 at any time. Once WDLOCK is set, the bits of this register cannot be modified."]
    Locked = 1,
}
impl From<Lock> for bool {
    #[inline(always)]
    fn from(variant: Lock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCK` reader - If this bit is set to one writing to this register does not affect bit 0. The clock source can only be changed by first clearing this bit, then writing the new value of bit 0."]
pub type LockR = crate::BitReader<Lock>;
impl LockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lock {
        match self.bits {
            false => Lock::Unlocked,
            true => Lock::Locked,
        }
    }
    #[doc = "This bit is set to 0 on any reset. It cannot be cleared by software."]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == Lock::Unlocked
    }
    #[doc = "Software can set this bit to 1 at any time. Once WDLOCK is set, the bits of this register cannot be modified."]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Lock::Locked
    }
}
#[doc = "Field `LOCK` writer - If this bit is set to one writing to this register does not affect bit 0. The clock source can only be changed by first clearing this bit, then writing the new value of bit 0."]
pub type LockW<'a, REG> = crate::BitWriter<'a, REG, Lock>;
impl<'a, REG> LockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "This bit is set to 0 on any reset. It cannot be cleared by software."]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(Lock::Unlocked)
    }
    #[doc = "Software can set this bit to 1 at any time. Once WDLOCK is set, the bits of this register cannot be modified."]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(Lock::Locked)
    }
}
impl R {
    #[doc = "Bits 0:1 - Selects source of WDT clock"]
    #[inline(always)]
    pub fn clksel(&self) -> ClkselR {
        ClkselR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 31 - If this bit is set to one writing to this register does not affect bit 0. The clock source can only be changed by first clearing this bit, then writing the new value of bit 0."]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selects source of WDT clock"]
    #[inline(always)]
    #[must_use]
    pub fn clksel(&mut self) -> ClkselW<ClkselSpec> {
        ClkselW::new(self, 0)
    }
    #[doc = "Bit 31 - If this bit is set to one writing to this register does not affect bit 0. The clock source can only be changed by first clearing this bit, then writing the new value of bit 0."]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LockW<ClkselSpec> {
        LockW::new(self, 31)
    }
}
#[doc = "Watchdog clock select register.\n\nYou can [`read`](crate::Reg::read) this register and get [`clksel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clksel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkselSpec;
impl crate::RegisterSpec for ClkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clksel::R`](R) reader structure"]
impl crate::Readable for ClkselSpec {}
#[doc = "`write(|w| ..)` method takes [`clksel::W`](W) writer structure"]
impl crate::Writable for ClkselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKSEL to value 0"]
impl crate::Resettable for ClkselSpec {
    const RESET_VALUE: u32 = 0;
}
