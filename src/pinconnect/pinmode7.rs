#[doc = "Register `PINMODE7` reader"]
pub type R = crate::R<Pinmode7Spec>;
#[doc = "Register `PINMODE7` writer"]
pub type W = crate::W<Pinmode7Spec>;
#[doc = "Port 3 pin 25 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P3_25mode {
    #[doc = "0: Pull-up. P3.25 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P3.25 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P3.25 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P3.25 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<P3_25mode> for u8 {
    #[inline(always)]
    fn from(variant: P3_25mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P3_25mode {
    type Ux = u8;
}
impl crate::IsEnum for P3_25mode {}
#[doc = "Field `P3_25MODE` reader - Port 3 pin 25 control."]
pub type P3_25modeR = crate::FieldReader<P3_25mode>;
impl P3_25modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P3_25mode {
        match self.bits {
            0 => P3_25mode::PullUp,
            1 => P3_25mode::Repeater,
            2 => P3_25mode::Disabled,
            3 => P3_25mode::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P3.25 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P3_25mode::PullUp
    }
    #[doc = "Repeater. P3.25 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P3_25mode::Repeater
    }
    #[doc = "Disabled. P3.25 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P3_25mode::Disabled
    }
    #[doc = "Pull-down. P3.25 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P3_25mode::PullDown
    }
}
#[doc = "Field `P3_25MODE` writer - Port 3 pin 25 control."]
pub type P3_25modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, P3_25mode, crate::Safe>;
impl<'a, REG> P3_25modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P3.25 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(P3_25mode::PullUp)
    }
    #[doc = "Repeater. P3.25 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(P3_25mode::Repeater)
    }
    #[doc = "Disabled. P3.25 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P3_25mode::Disabled)
    }
    #[doc = "Pull-down. P3.25 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(P3_25mode::PullDown)
    }
}
#[doc = "Port 3 pin 26 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P3_26mode {
    #[doc = "0: Pull-up. P3.26 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P3.26 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P3.26 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P3.26 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<P3_26mode> for u8 {
    #[inline(always)]
    fn from(variant: P3_26mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P3_26mode {
    type Ux = u8;
}
impl crate::IsEnum for P3_26mode {}
#[doc = "Field `P3_26MODE` reader - Port 3 pin 26 control."]
pub type P3_26modeR = crate::FieldReader<P3_26mode>;
impl P3_26modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P3_26mode {
        match self.bits {
            0 => P3_26mode::PullUp,
            1 => P3_26mode::Repeater,
            2 => P3_26mode::Disabled,
            3 => P3_26mode::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P3.26 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P3_26mode::PullUp
    }
    #[doc = "Repeater. P3.26 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P3_26mode::Repeater
    }
    #[doc = "Disabled. P3.26 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P3_26mode::Disabled
    }
    #[doc = "Pull-down. P3.26 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P3_26mode::PullDown
    }
}
#[doc = "Field `P3_26MODE` writer - Port 3 pin 26 control."]
pub type P3_26modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, P3_26mode, crate::Safe>;
impl<'a, REG> P3_26modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P3.26 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(P3_26mode::PullUp)
    }
    #[doc = "Repeater. P3.26 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(P3_26mode::Repeater)
    }
    #[doc = "Disabled. P3.26 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P3_26mode::Disabled)
    }
    #[doc = "Pull-down. P3.26 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(P3_26mode::PullDown)
    }
}
impl R {
    #[doc = "Bits 18:19 - Port 3 pin 25 control."]
    #[inline(always)]
    pub fn p3_25mode(&self) -> P3_25modeR {
        P3_25modeR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Port 3 pin 26 control."]
    #[inline(always)]
    pub fn p3_26mode(&self) -> P3_26modeR {
        P3_26modeR::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 18:19 - Port 3 pin 25 control."]
    #[inline(always)]
    #[must_use]
    pub fn p3_25mode(&mut self) -> P3_25modeW<Pinmode7Spec> {
        P3_25modeW::new(self, 18)
    }
    #[doc = "Bits 20:21 - Port 3 pin 26 control."]
    #[inline(always)]
    #[must_use]
    pub fn p3_26mode(&mut self) -> P3_26modeW<Pinmode7Spec> {
        P3_26modeW::new(self, 20)
    }
}
#[doc = "Pin mode select register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`pinmode7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pinmode7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pinmode7Spec;
impl crate::RegisterSpec for Pinmode7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pinmode7::R`](R) reader structure"]
impl crate::Readable for Pinmode7Spec {}
#[doc = "`write(|w| ..)` method takes [`pinmode7::W`](W) writer structure"]
impl crate::Writable for Pinmode7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PINMODE7 to value 0"]
impl crate::Resettable for Pinmode7Spec {
    const RESET_VALUE: u32 = 0;
}
