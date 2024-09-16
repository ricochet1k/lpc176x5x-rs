#[doc = "Register `IR` reader"]
pub type R = crate::R<IrSpec>;
#[doc = "Register `IR` writer"]
pub type W = crate::W<IrSpec>;
#[doc = "Field `PWMMR0INT` reader - Interrupt flag for PWM match channel 0."]
pub type Pwmmr0intR = crate::BitReader;
#[doc = "Field `PWMMR0INT` writer - Interrupt flag for PWM match channel 0."]
pub type Pwmmr0intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWMMR1INT` reader - Interrupt flag for PWM match channel 1."]
pub type Pwmmr1intR = crate::BitReader;
#[doc = "Field `PWMMR1INT` writer - Interrupt flag for PWM match channel 1."]
pub type Pwmmr1intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWMMR2INT` reader - Interrupt flag for PWM match channel 2."]
pub type Pwmmr2intR = crate::BitReader;
#[doc = "Field `PWMMR2INT` writer - Interrupt flag for PWM match channel 2."]
pub type Pwmmr2intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWMMR3INT` reader - Interrupt flag for PWM match channel 3."]
pub type Pwmmr3intR = crate::BitReader;
#[doc = "Field `PWMMR3INT` writer - Interrupt flag for PWM match channel 3."]
pub type Pwmmr3intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWMCAP0INT` reader - Interrupt flag for capture input 0"]
pub type Pwmcap0intR = crate::BitReader;
#[doc = "Field `PWMCAP0INT` writer - Interrupt flag for capture input 0"]
pub type Pwmcap0intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWMCAP1INT` reader - Interrupt flag for capture input 1 (available in PWM1IR only; this bit is reserved in PWM0IR)."]
pub type Pwmcap1intR = crate::BitReader;
#[doc = "Field `PWMCAP1INT` writer - Interrupt flag for capture input 1 (available in PWM1IR only; this bit is reserved in PWM0IR)."]
pub type Pwmcap1intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWMMR4INT` reader - Interrupt flag for PWM match channel 4."]
pub type Pwmmr4intR = crate::BitReader;
#[doc = "Field `PWMMR4INT` writer - Interrupt flag for PWM match channel 4."]
pub type Pwmmr4intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWMMR5INT` reader - Interrupt flag for PWM match channel 5."]
pub type Pwmmr5intR = crate::BitReader;
#[doc = "Field `PWMMR5INT` writer - Interrupt flag for PWM match channel 5."]
pub type Pwmmr5intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWMMR6INT` reader - Interrupt flag for PWM match channel 6."]
pub type Pwmmr6intR = crate::BitReader;
#[doc = "Field `PWMMR6INT` writer - Interrupt flag for PWM match channel 6."]
pub type Pwmmr6intW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Interrupt flag for PWM match channel 0."]
    #[inline(always)]
    pub fn pwmmr0int(&self) -> Pwmmr0intR {
        Pwmmr0intR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt flag for PWM match channel 1."]
    #[inline(always)]
    pub fn pwmmr1int(&self) -> Pwmmr1intR {
        Pwmmr1intR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt flag for PWM match channel 2."]
    #[inline(always)]
    pub fn pwmmr2int(&self) -> Pwmmr2intR {
        Pwmmr2intR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt flag for PWM match channel 3."]
    #[inline(always)]
    pub fn pwmmr3int(&self) -> Pwmmr3intR {
        Pwmmr3intR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt flag for capture input 0"]
    #[inline(always)]
    pub fn pwmcap0int(&self) -> Pwmcap0intR {
        Pwmcap0intR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt flag for capture input 1 (available in PWM1IR only; this bit is reserved in PWM0IR)."]
    #[inline(always)]
    pub fn pwmcap1int(&self) -> Pwmcap1intR {
        Pwmcap1intR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt flag for PWM match channel 4."]
    #[inline(always)]
    pub fn pwmmr4int(&self) -> Pwmmr4intR {
        Pwmmr4intR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt flag for PWM match channel 5."]
    #[inline(always)]
    pub fn pwmmr5int(&self) -> Pwmmr5intR {
        Pwmmr5intR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt flag for PWM match channel 6."]
    #[inline(always)]
    pub fn pwmmr6int(&self) -> Pwmmr6intR {
        Pwmmr6intR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt flag for PWM match channel 0."]
    #[inline(always)]
    #[must_use]
    pub fn pwmmr0int(&mut self) -> Pwmmr0intW<IrSpec> {
        Pwmmr0intW::new(self, 0)
    }
    #[doc = "Bit 1 - Interrupt flag for PWM match channel 1."]
    #[inline(always)]
    #[must_use]
    pub fn pwmmr1int(&mut self) -> Pwmmr1intW<IrSpec> {
        Pwmmr1intW::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt flag for PWM match channel 2."]
    #[inline(always)]
    #[must_use]
    pub fn pwmmr2int(&mut self) -> Pwmmr2intW<IrSpec> {
        Pwmmr2intW::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt flag for PWM match channel 3."]
    #[inline(always)]
    #[must_use]
    pub fn pwmmr3int(&mut self) -> Pwmmr3intW<IrSpec> {
        Pwmmr3intW::new(self, 3)
    }
    #[doc = "Bit 4 - Interrupt flag for capture input 0"]
    #[inline(always)]
    #[must_use]
    pub fn pwmcap0int(&mut self) -> Pwmcap0intW<IrSpec> {
        Pwmcap0intW::new(self, 4)
    }
    #[doc = "Bit 5 - Interrupt flag for capture input 1 (available in PWM1IR only; this bit is reserved in PWM0IR)."]
    #[inline(always)]
    #[must_use]
    pub fn pwmcap1int(&mut self) -> Pwmcap1intW<IrSpec> {
        Pwmcap1intW::new(self, 5)
    }
    #[doc = "Bit 8 - Interrupt flag for PWM match channel 4."]
    #[inline(always)]
    #[must_use]
    pub fn pwmmr4int(&mut self) -> Pwmmr4intW<IrSpec> {
        Pwmmr4intW::new(self, 8)
    }
    #[doc = "Bit 9 - Interrupt flag for PWM match channel 5."]
    #[inline(always)]
    #[must_use]
    pub fn pwmmr5int(&mut self) -> Pwmmr5intW<IrSpec> {
        Pwmmr5intW::new(self, 9)
    }
    #[doc = "Bit 10 - Interrupt flag for PWM match channel 6."]
    #[inline(always)]
    #[must_use]
    pub fn pwmmr6int(&mut self) -> Pwmmr6intW<IrSpec> {
        Pwmmr6intW::new(self, 10)
    }
}
#[doc = "Interrupt Register. The IR can be written to clear interrupts, or read to identify which PWM interrupt sources are pending.\n\nYou can [`read`](crate::Reg::read) this register and get [`ir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrSpec;
impl crate::RegisterSpec for IrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ir::R`](R) reader structure"]
impl crate::Readable for IrSpec {}
#[doc = "`write(|w| ..)` method takes [`ir::W`](W) writer structure"]
impl crate::Writable for IrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IR to value 0"]
impl crate::Resettable for IrSpec {
    const RESET_VALUE: u32 = 0;
}
