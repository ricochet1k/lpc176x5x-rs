#[doc = "Register `PINMODE3` reader"]
pub type R = crate::R<Pinmode3Spec>;
#[doc = "Register `PINMODE3` writer"]
pub type W = crate::W<Pinmode3Spec>;
#[doc = "Port 1 pin 16 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_16mode {
    #[doc = "0: Pull-up. P1.16 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P1.16 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P1.16 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P1.16 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<P1_16mode> for u8 {
    #[inline(always)]
    fn from(variant: P1_16mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_16mode {
    type Ux = u8;
}
impl crate::IsEnum for P1_16mode {}
#[doc = "Field `P1_16MODE` reader - Port 1 pin 16 control."]
pub type P1_16modeR = crate::FieldReader<P1_16mode>;
impl P1_16modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_16mode {
        match self.bits {
            0 => P1_16mode::PullUp,
            1 => P1_16mode::Repeater,
            2 => P1_16mode::Disabled,
            3 => P1_16mode::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P1.16 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_16mode::PullUp
    }
    #[doc = "Repeater. P1.16 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_16mode::Repeater
    }
    #[doc = "Disabled. P1.16 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_16mode::Disabled
    }
    #[doc = "Pull-down. P1.16 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_16mode::PullDown
    }
}
#[doc = "Field `P1_16MODE` writer - Port 1 pin 16 control."]
pub type P1_16modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, P1_16mode, crate::Safe>;
impl<'a, REG> P1_16modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P1.16 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(P1_16mode::PullUp)
    }
    #[doc = "Repeater. P1.16 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(P1_16mode::Repeater)
    }
    #[doc = "Disabled. P1.16 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P1_16mode::Disabled)
    }
    #[doc = "Pull-down. P1.16 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(P1_16mode::PullDown)
    }
}
#[doc = "Port 1 pin 17 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_17mode {
    #[doc = "0: Pull-up. P1.17 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P1.17 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P1.17 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P1.17 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<P1_17mode> for u8 {
    #[inline(always)]
    fn from(variant: P1_17mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_17mode {
    type Ux = u8;
}
impl crate::IsEnum for P1_17mode {}
#[doc = "Field `P1_17MODE` reader - Port 1 pin 17 control."]
pub type P1_17modeR = crate::FieldReader<P1_17mode>;
impl P1_17modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_17mode {
        match self.bits {
            0 => P1_17mode::PullUp,
            1 => P1_17mode::Repeater,
            2 => P1_17mode::Disabled,
            3 => P1_17mode::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P1.17 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_17mode::PullUp
    }
    #[doc = "Repeater. P1.17 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_17mode::Repeater
    }
    #[doc = "Disabled. P1.17 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_17mode::Disabled
    }
    #[doc = "Pull-down. P1.17 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_17mode::PullDown
    }
}
#[doc = "Field `P1_17MODE` writer - Port 1 pin 17 control."]
pub type P1_17modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, P1_17mode, crate::Safe>;
impl<'a, REG> P1_17modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P1.17 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(P1_17mode::PullUp)
    }
    #[doc = "Repeater. P1.17 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(P1_17mode::Repeater)
    }
    #[doc = "Disabled. P1.17 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P1_17mode::Disabled)
    }
    #[doc = "Pull-down. P1.17 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(P1_17mode::PullDown)
    }
}
#[doc = "Port 1 pin 18 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_18mode {
    #[doc = "0: Pull-up. P1.18 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P1.18 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P1.18 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P1.18 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<P1_18mode> for u8 {
    #[inline(always)]
    fn from(variant: P1_18mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_18mode {
    type Ux = u8;
}
impl crate::IsEnum for P1_18mode {}
#[doc = "Field `P1_18MODE` reader - Port 1 pin 18 control."]
pub type P1_18modeR = crate::FieldReader<P1_18mode>;
impl P1_18modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_18mode {
        match self.bits {
            0 => P1_18mode::PullUp,
            1 => P1_18mode::Repeater,
            2 => P1_18mode::Disabled,
            3 => P1_18mode::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P1.18 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_18mode::PullUp
    }
    #[doc = "Repeater. P1.18 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_18mode::Repeater
    }
    #[doc = "Disabled. P1.18 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_18mode::Disabled
    }
    #[doc = "Pull-down. P1.18 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_18mode::PullDown
    }
}
#[doc = "Field `P1_18MODE` writer - Port 1 pin 18 control."]
pub type P1_18modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, P1_18mode, crate::Safe>;
impl<'a, REG> P1_18modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P1.18 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(P1_18mode::PullUp)
    }
    #[doc = "Repeater. P1.18 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(P1_18mode::Repeater)
    }
    #[doc = "Disabled. P1.18 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P1_18mode::Disabled)
    }
    #[doc = "Pull-down. P1.18 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(P1_18mode::PullDown)
    }
}
#[doc = "Port 1 pin 19 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_19mode {
    #[doc = "0: Pull-up. P1.19 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P1.19 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P1.19 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P1.19 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<P1_19mode> for u8 {
    #[inline(always)]
    fn from(variant: P1_19mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_19mode {
    type Ux = u8;
}
impl crate::IsEnum for P1_19mode {}
#[doc = "Field `P1_19MODE` reader - Port 1 pin 19 control."]
pub type P1_19modeR = crate::FieldReader<P1_19mode>;
impl P1_19modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_19mode {
        match self.bits {
            0 => P1_19mode::PullUp,
            1 => P1_19mode::Repeater,
            2 => P1_19mode::Disabled,
            3 => P1_19mode::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P1.19 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_19mode::PullUp
    }
    #[doc = "Repeater. P1.19 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_19mode::Repeater
    }
    #[doc = "Disabled. P1.19 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_19mode::Disabled
    }
    #[doc = "Pull-down. P1.19 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_19mode::PullDown
    }
}
#[doc = "Field `P1_19MODE` writer - Port 1 pin 19 control."]
pub type P1_19modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, P1_19mode, crate::Safe>;
impl<'a, REG> P1_19modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P1.19 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(P1_19mode::PullUp)
    }
    #[doc = "Repeater. P1.19 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(P1_19mode::Repeater)
    }
    #[doc = "Disabled. P1.19 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P1_19mode::Disabled)
    }
    #[doc = "Pull-down. P1.19 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(P1_19mode::PullDown)
    }
}
#[doc = "Port 1 pin 20 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_20mode {
    #[doc = "0: Pull-up. P1.20 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P1.20 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P1.20 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P1.20 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<P1_20mode> for u8 {
    #[inline(always)]
    fn from(variant: P1_20mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_20mode {
    type Ux = u8;
}
impl crate::IsEnum for P1_20mode {}
#[doc = "Field `P1_20MODE` reader - Port 1 pin 20 control."]
pub type P1_20modeR = crate::FieldReader<P1_20mode>;
impl P1_20modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_20mode {
        match self.bits {
            0 => P1_20mode::PullUp,
            1 => P1_20mode::Repeater,
            2 => P1_20mode::Disabled,
            3 => P1_20mode::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P1.20 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_20mode::PullUp
    }
    #[doc = "Repeater. P1.20 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_20mode::Repeater
    }
    #[doc = "Disabled. P1.20 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_20mode::Disabled
    }
    #[doc = "Pull-down. P1.20 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_20mode::PullDown
    }
}
#[doc = "Field `P1_20MODE` writer - Port 1 pin 20 control."]
pub type P1_20modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, P1_20mode, crate::Safe>;
impl<'a, REG> P1_20modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P1.20 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(P1_20mode::PullUp)
    }
    #[doc = "Repeater. P1.20 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(P1_20mode::Repeater)
    }
    #[doc = "Disabled. P1.20 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P1_20mode::Disabled)
    }
    #[doc = "Pull-down. P1.20 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(P1_20mode::PullDown)
    }
}
#[doc = "Port 1 pin 21 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_21mode {
    #[doc = "0: Pull-up. P1.21 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P1.21 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P1.21 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P1.21 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<P1_21mode> for u8 {
    #[inline(always)]
    fn from(variant: P1_21mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_21mode {
    type Ux = u8;
}
impl crate::IsEnum for P1_21mode {}
#[doc = "Field `P1_21MODE` reader - Port 1 pin 21 control."]
pub type P1_21modeR = crate::FieldReader<P1_21mode>;
impl P1_21modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_21mode {
        match self.bits {
            0 => P1_21mode::PullUp,
            1 => P1_21mode::Repeater,
            2 => P1_21mode::Disabled,
            3 => P1_21mode::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P1.21 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_21mode::PullUp
    }
    #[doc = "Repeater. P1.21 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_21mode::Repeater
    }
    #[doc = "Disabled. P1.21 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_21mode::Disabled
    }
    #[doc = "Pull-down. P1.21 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_21mode::PullDown
    }
}
#[doc = "Field `P1_21MODE` writer - Port 1 pin 21 control."]
pub type P1_21modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, P1_21mode, crate::Safe>;
impl<'a, REG> P1_21modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P1.21 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(P1_21mode::PullUp)
    }
    #[doc = "Repeater. P1.21 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(P1_21mode::Repeater)
    }
    #[doc = "Disabled. P1.21 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P1_21mode::Disabled)
    }
    #[doc = "Pull-down. P1.21 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(P1_21mode::PullDown)
    }
}
#[doc = "Port 1 pin 22 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_22mode {
    #[doc = "0: Pull-up. P1.22 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P1.22 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P1.22 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P1.22 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<P1_22mode> for u8 {
    #[inline(always)]
    fn from(variant: P1_22mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_22mode {
    type Ux = u8;
}
impl crate::IsEnum for P1_22mode {}
#[doc = "Field `P1_22MODE` reader - Port 1 pin 22 control."]
pub type P1_22modeR = crate::FieldReader<P1_22mode>;
impl P1_22modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_22mode {
        match self.bits {
            0 => P1_22mode::PullUp,
            1 => P1_22mode::Repeater,
            2 => P1_22mode::Disabled,
            3 => P1_22mode::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P1.22 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_22mode::PullUp
    }
    #[doc = "Repeater. P1.22 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_22mode::Repeater
    }
    #[doc = "Disabled. P1.22 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_22mode::Disabled
    }
    #[doc = "Pull-down. P1.22 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_22mode::PullDown
    }
}
#[doc = "Field `P1_22MODE` writer - Port 1 pin 22 control."]
pub type P1_22modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, P1_22mode, crate::Safe>;
impl<'a, REG> P1_22modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P1.22 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(P1_22mode::PullUp)
    }
    #[doc = "Repeater. P1.22 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(P1_22mode::Repeater)
    }
    #[doc = "Disabled. P1.22 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P1_22mode::Disabled)
    }
    #[doc = "Pull-down. P1.22 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(P1_22mode::PullDown)
    }
}
#[doc = "Port 1 pin 23 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_23mode {
    #[doc = "0: Pull-up. P1.23 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P1.23 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P1.23 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P1.23 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<P1_23mode> for u8 {
    #[inline(always)]
    fn from(variant: P1_23mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_23mode {
    type Ux = u8;
}
impl crate::IsEnum for P1_23mode {}
#[doc = "Field `P1_23MODE` reader - Port 1 pin 23 control."]
pub type P1_23modeR = crate::FieldReader<P1_23mode>;
impl P1_23modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_23mode {
        match self.bits {
            0 => P1_23mode::PullUp,
            1 => P1_23mode::Repeater,
            2 => P1_23mode::Disabled,
            3 => P1_23mode::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P1.23 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_23mode::PullUp
    }
    #[doc = "Repeater. P1.23 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_23mode::Repeater
    }
    #[doc = "Disabled. P1.23 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_23mode::Disabled
    }
    #[doc = "Pull-down. P1.23 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_23mode::PullDown
    }
}
#[doc = "Field `P1_23MODE` writer - Port 1 pin 23 control."]
pub type P1_23modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, P1_23mode, crate::Safe>;
impl<'a, REG> P1_23modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P1.23 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(P1_23mode::PullUp)
    }
    #[doc = "Repeater. P1.23 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(P1_23mode::Repeater)
    }
    #[doc = "Disabled. P1.23 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P1_23mode::Disabled)
    }
    #[doc = "Pull-down. P1.23 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(P1_23mode::PullDown)
    }
}
#[doc = "Port 1 pin 24 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_24mode {
    #[doc = "0: Pull-up. P1.24 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P1.24 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P1.24 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P1.24 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<P1_24mode> for u8 {
    #[inline(always)]
    fn from(variant: P1_24mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_24mode {
    type Ux = u8;
}
impl crate::IsEnum for P1_24mode {}
#[doc = "Field `P1_24MODE` reader - Port 1 pin 24 control."]
pub type P1_24modeR = crate::FieldReader<P1_24mode>;
impl P1_24modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_24mode {
        match self.bits {
            0 => P1_24mode::PullUp,
            1 => P1_24mode::Repeater,
            2 => P1_24mode::Disabled,
            3 => P1_24mode::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P1.24 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_24mode::PullUp
    }
    #[doc = "Repeater. P1.24 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_24mode::Repeater
    }
    #[doc = "Disabled. P1.24 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_24mode::Disabled
    }
    #[doc = "Pull-down. P1.24 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_24mode::PullDown
    }
}
#[doc = "Field `P1_24MODE` writer - Port 1 pin 24 control."]
pub type P1_24modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, P1_24mode, crate::Safe>;
impl<'a, REG> P1_24modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P1.24 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(P1_24mode::PullUp)
    }
    #[doc = "Repeater. P1.24 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(P1_24mode::Repeater)
    }
    #[doc = "Disabled. P1.24 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P1_24mode::Disabled)
    }
    #[doc = "Pull-down. P1.24 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(P1_24mode::PullDown)
    }
}
#[doc = "Port 1 pin 25 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_25mode {
    #[doc = "0: Pull-up. P1.25 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P1.25 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P1.25 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P1.25 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<P1_25mode> for u8 {
    #[inline(always)]
    fn from(variant: P1_25mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_25mode {
    type Ux = u8;
}
impl crate::IsEnum for P1_25mode {}
#[doc = "Field `P1_25MODE` reader - Port 1 pin 25 control."]
pub type P1_25modeR = crate::FieldReader<P1_25mode>;
impl P1_25modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_25mode {
        match self.bits {
            0 => P1_25mode::PullUp,
            1 => P1_25mode::Repeater,
            2 => P1_25mode::Disabled,
            3 => P1_25mode::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P1.25 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_25mode::PullUp
    }
    #[doc = "Repeater. P1.25 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_25mode::Repeater
    }
    #[doc = "Disabled. P1.25 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_25mode::Disabled
    }
    #[doc = "Pull-down. P1.25 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_25mode::PullDown
    }
}
#[doc = "Field `P1_25MODE` writer - Port 1 pin 25 control."]
pub type P1_25modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, P1_25mode, crate::Safe>;
impl<'a, REG> P1_25modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P1.25 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(P1_25mode::PullUp)
    }
    #[doc = "Repeater. P1.25 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(P1_25mode::Repeater)
    }
    #[doc = "Disabled. P1.25 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P1_25mode::Disabled)
    }
    #[doc = "Pull-down. P1.25 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(P1_25mode::PullDown)
    }
}
#[doc = "Port 1 pin 26 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_26mode {
    #[doc = "0: Pull-up. P1.26 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P1.26 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P1.26 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P1.26 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<P1_26mode> for u8 {
    #[inline(always)]
    fn from(variant: P1_26mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_26mode {
    type Ux = u8;
}
impl crate::IsEnum for P1_26mode {}
#[doc = "Field `P1_26MODE` reader - Port 1 pin 26 control."]
pub type P1_26modeR = crate::FieldReader<P1_26mode>;
impl P1_26modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_26mode {
        match self.bits {
            0 => P1_26mode::PullUp,
            1 => P1_26mode::Repeater,
            2 => P1_26mode::Disabled,
            3 => P1_26mode::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P1.26 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_26mode::PullUp
    }
    #[doc = "Repeater. P1.26 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_26mode::Repeater
    }
    #[doc = "Disabled. P1.26 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_26mode::Disabled
    }
    #[doc = "Pull-down. P1.26 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_26mode::PullDown
    }
}
#[doc = "Field `P1_26MODE` writer - Port 1 pin 26 control."]
pub type P1_26modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, P1_26mode, crate::Safe>;
impl<'a, REG> P1_26modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P1.26 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(P1_26mode::PullUp)
    }
    #[doc = "Repeater. P1.26 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(P1_26mode::Repeater)
    }
    #[doc = "Disabled. P1.26 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P1_26mode::Disabled)
    }
    #[doc = "Pull-down. P1.26 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(P1_26mode::PullDown)
    }
}
#[doc = "Port 1 pin 27 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_27mode {
    #[doc = "0: Pull-up. P1.27 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P1.27 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P1.27 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P1.27 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<P1_27mode> for u8 {
    #[inline(always)]
    fn from(variant: P1_27mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_27mode {
    type Ux = u8;
}
impl crate::IsEnum for P1_27mode {}
#[doc = "Field `P1_27MODE` reader - Port 1 pin 27 control."]
pub type P1_27modeR = crate::FieldReader<P1_27mode>;
impl P1_27modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_27mode {
        match self.bits {
            0 => P1_27mode::PullUp,
            1 => P1_27mode::Repeater,
            2 => P1_27mode::Disabled,
            3 => P1_27mode::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P1.27 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_27mode::PullUp
    }
    #[doc = "Repeater. P1.27 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_27mode::Repeater
    }
    #[doc = "Disabled. P1.27 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_27mode::Disabled
    }
    #[doc = "Pull-down. P1.27 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_27mode::PullDown
    }
}
#[doc = "Field `P1_27MODE` writer - Port 1 pin 27 control."]
pub type P1_27modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, P1_27mode, crate::Safe>;
impl<'a, REG> P1_27modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P1.27 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(P1_27mode::PullUp)
    }
    #[doc = "Repeater. P1.27 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(P1_27mode::Repeater)
    }
    #[doc = "Disabled. P1.27 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P1_27mode::Disabled)
    }
    #[doc = "Pull-down. P1.27 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(P1_27mode::PullDown)
    }
}
#[doc = "Port 1 pin 28 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_28mode {
    #[doc = "0: Pull-up. P1.28 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P1.28 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P1.28 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P1.28 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<P1_28mode> for u8 {
    #[inline(always)]
    fn from(variant: P1_28mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_28mode {
    type Ux = u8;
}
impl crate::IsEnum for P1_28mode {}
#[doc = "Field `P1_28MODE` reader - Port 1 pin 28 control."]
pub type P1_28modeR = crate::FieldReader<P1_28mode>;
impl P1_28modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_28mode {
        match self.bits {
            0 => P1_28mode::PullUp,
            1 => P1_28mode::Repeater,
            2 => P1_28mode::Disabled,
            3 => P1_28mode::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P1.28 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_28mode::PullUp
    }
    #[doc = "Repeater. P1.28 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_28mode::Repeater
    }
    #[doc = "Disabled. P1.28 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_28mode::Disabled
    }
    #[doc = "Pull-down. P1.28 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_28mode::PullDown
    }
}
#[doc = "Field `P1_28MODE` writer - Port 1 pin 28 control."]
pub type P1_28modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, P1_28mode, crate::Safe>;
impl<'a, REG> P1_28modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P1.28 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(P1_28mode::PullUp)
    }
    #[doc = "Repeater. P1.28 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(P1_28mode::Repeater)
    }
    #[doc = "Disabled. P1.28 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P1_28mode::Disabled)
    }
    #[doc = "Pull-down. P1.28 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(P1_28mode::PullDown)
    }
}
#[doc = "Port 1 pin 29 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_29mode {
    #[doc = "0: Pull-up. P1.29 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P1.29 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P1.29 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P1.29 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<P1_29mode> for u8 {
    #[inline(always)]
    fn from(variant: P1_29mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_29mode {
    type Ux = u8;
}
impl crate::IsEnum for P1_29mode {}
#[doc = "Field `P1_29MODE` reader - Port 1 pin 29 control."]
pub type P1_29modeR = crate::FieldReader<P1_29mode>;
impl P1_29modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_29mode {
        match self.bits {
            0 => P1_29mode::PullUp,
            1 => P1_29mode::Repeater,
            2 => P1_29mode::Disabled,
            3 => P1_29mode::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P1.29 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_29mode::PullUp
    }
    #[doc = "Repeater. P1.29 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_29mode::Repeater
    }
    #[doc = "Disabled. P1.29 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_29mode::Disabled
    }
    #[doc = "Pull-down. P1.29 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_29mode::PullDown
    }
}
#[doc = "Field `P1_29MODE` writer - Port 1 pin 29 control."]
pub type P1_29modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, P1_29mode, crate::Safe>;
impl<'a, REG> P1_29modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P1.29 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(P1_29mode::PullUp)
    }
    #[doc = "Repeater. P1.29 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(P1_29mode::Repeater)
    }
    #[doc = "Disabled. P1.29 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P1_29mode::Disabled)
    }
    #[doc = "Pull-down. P1.29 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(P1_29mode::PullDown)
    }
}
#[doc = "Port 1 pin 30 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_30mode {
    #[doc = "0: Pull-up. P1.30 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P1.30 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P1.30 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P1.30 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<P1_30mode> for u8 {
    #[inline(always)]
    fn from(variant: P1_30mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_30mode {
    type Ux = u8;
}
impl crate::IsEnum for P1_30mode {}
#[doc = "Field `P1_30MODE` reader - Port 1 pin 30 control."]
pub type P1_30modeR = crate::FieldReader<P1_30mode>;
impl P1_30modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_30mode {
        match self.bits {
            0 => P1_30mode::PullUp,
            1 => P1_30mode::Repeater,
            2 => P1_30mode::Disabled,
            3 => P1_30mode::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P1.30 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_30mode::PullUp
    }
    #[doc = "Repeater. P1.30 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_30mode::Repeater
    }
    #[doc = "Disabled. P1.30 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_30mode::Disabled
    }
    #[doc = "Pull-down. P1.30 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_30mode::PullDown
    }
}
#[doc = "Field `P1_30MODE` writer - Port 1 pin 30 control."]
pub type P1_30modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, P1_30mode, crate::Safe>;
impl<'a, REG> P1_30modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P1.30 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(P1_30mode::PullUp)
    }
    #[doc = "Repeater. P1.30 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(P1_30mode::Repeater)
    }
    #[doc = "Disabled. P1.30 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P1_30mode::Disabled)
    }
    #[doc = "Pull-down. P1.30 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(P1_30mode::PullDown)
    }
}
#[doc = "Port 1 pin 31 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_31mode {
    #[doc = "0: Pull-up. P1.31 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P1.31 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P1.31 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P1.31 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<P1_31mode> for u8 {
    #[inline(always)]
    fn from(variant: P1_31mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_31mode {
    type Ux = u8;
}
impl crate::IsEnum for P1_31mode {}
#[doc = "Field `P1_31MODE` reader - Port 1 pin 31 control."]
pub type P1_31modeR = crate::FieldReader<P1_31mode>;
impl P1_31modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_31mode {
        match self.bits {
            0 => P1_31mode::PullUp,
            1 => P1_31mode::Repeater,
            2 => P1_31mode::Disabled,
            3 => P1_31mode::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P1.31 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_31mode::PullUp
    }
    #[doc = "Repeater. P1.31 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_31mode::Repeater
    }
    #[doc = "Disabled. P1.31 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_31mode::Disabled
    }
    #[doc = "Pull-down. P1.31 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_31mode::PullDown
    }
}
#[doc = "Field `P1_31MODE` writer - Port 1 pin 31 control."]
pub type P1_31modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, P1_31mode, crate::Safe>;
impl<'a, REG> P1_31modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P1.31 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(P1_31mode::PullUp)
    }
    #[doc = "Repeater. P1.31 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(P1_31mode::Repeater)
    }
    #[doc = "Disabled. P1.31 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P1_31mode::Disabled)
    }
    #[doc = "Pull-down. P1.31 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(P1_31mode::PullDown)
    }
}
impl R {
    #[doc = "Bits 0:1 - Port 1 pin 16 control."]
    #[inline(always)]
    pub fn p1_16mode(&self) -> P1_16modeR {
        P1_16modeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port 1 pin 17 control."]
    #[inline(always)]
    pub fn p1_17mode(&self) -> P1_17modeR {
        P1_17modeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port 1 pin 18 control."]
    #[inline(always)]
    pub fn p1_18mode(&self) -> P1_18modeR {
        P1_18modeR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port 1 pin 19 control."]
    #[inline(always)]
    pub fn p1_19mode(&self) -> P1_19modeR {
        P1_19modeR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port 1 pin 20 control."]
    #[inline(always)]
    pub fn p1_20mode(&self) -> P1_20modeR {
        P1_20modeR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port 1 pin 21 control."]
    #[inline(always)]
    pub fn p1_21mode(&self) -> P1_21modeR {
        P1_21modeR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port 1 pin 22 control."]
    #[inline(always)]
    pub fn p1_22mode(&self) -> P1_22modeR {
        P1_22modeR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port 1 pin 23 control."]
    #[inline(always)]
    pub fn p1_23mode(&self) -> P1_23modeR {
        P1_23modeR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port 1 pin 24 control."]
    #[inline(always)]
    pub fn p1_24mode(&self) -> P1_24modeR {
        P1_24modeR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Port 1 pin 25 control."]
    #[inline(always)]
    pub fn p1_25mode(&self) -> P1_25modeR {
        P1_25modeR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Port 1 pin 26 control."]
    #[inline(always)]
    pub fn p1_26mode(&self) -> P1_26modeR {
        P1_26modeR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Port 1 pin 27 control."]
    #[inline(always)]
    pub fn p1_27mode(&self) -> P1_27modeR {
        P1_27modeR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Port 1 pin 28 control."]
    #[inline(always)]
    pub fn p1_28mode(&self) -> P1_28modeR {
        P1_28modeR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Port 1 pin 29 control."]
    #[inline(always)]
    pub fn p1_29mode(&self) -> P1_29modeR {
        P1_29modeR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Port 1 pin 30 control."]
    #[inline(always)]
    pub fn p1_30mode(&self) -> P1_30modeR {
        P1_30modeR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Port 1 pin 31 control."]
    #[inline(always)]
    pub fn p1_31mode(&self) -> P1_31modeR {
        P1_31modeR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port 1 pin 16 control."]
    #[inline(always)]
    #[must_use]
    pub fn p1_16mode(&mut self) -> P1_16modeW<Pinmode3Spec> {
        P1_16modeW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Port 1 pin 17 control."]
    #[inline(always)]
    #[must_use]
    pub fn p1_17mode(&mut self) -> P1_17modeW<Pinmode3Spec> {
        P1_17modeW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Port 1 pin 18 control."]
    #[inline(always)]
    #[must_use]
    pub fn p1_18mode(&mut self) -> P1_18modeW<Pinmode3Spec> {
        P1_18modeW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Port 1 pin 19 control."]
    #[inline(always)]
    #[must_use]
    pub fn p1_19mode(&mut self) -> P1_19modeW<Pinmode3Spec> {
        P1_19modeW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Port 1 pin 20 control."]
    #[inline(always)]
    #[must_use]
    pub fn p1_20mode(&mut self) -> P1_20modeW<Pinmode3Spec> {
        P1_20modeW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Port 1 pin 21 control."]
    #[inline(always)]
    #[must_use]
    pub fn p1_21mode(&mut self) -> P1_21modeW<Pinmode3Spec> {
        P1_21modeW::new(self, 10)
    }
    #[doc = "Bits 12:13 - Port 1 pin 22 control."]
    #[inline(always)]
    #[must_use]
    pub fn p1_22mode(&mut self) -> P1_22modeW<Pinmode3Spec> {
        P1_22modeW::new(self, 12)
    }
    #[doc = "Bits 14:15 - Port 1 pin 23 control."]
    #[inline(always)]
    #[must_use]
    pub fn p1_23mode(&mut self) -> P1_23modeW<Pinmode3Spec> {
        P1_23modeW::new(self, 14)
    }
    #[doc = "Bits 16:17 - Port 1 pin 24 control."]
    #[inline(always)]
    #[must_use]
    pub fn p1_24mode(&mut self) -> P1_24modeW<Pinmode3Spec> {
        P1_24modeW::new(self, 16)
    }
    #[doc = "Bits 18:19 - Port 1 pin 25 control."]
    #[inline(always)]
    #[must_use]
    pub fn p1_25mode(&mut self) -> P1_25modeW<Pinmode3Spec> {
        P1_25modeW::new(self, 18)
    }
    #[doc = "Bits 20:21 - Port 1 pin 26 control."]
    #[inline(always)]
    #[must_use]
    pub fn p1_26mode(&mut self) -> P1_26modeW<Pinmode3Spec> {
        P1_26modeW::new(self, 20)
    }
    #[doc = "Bits 22:23 - Port 1 pin 27 control."]
    #[inline(always)]
    #[must_use]
    pub fn p1_27mode(&mut self) -> P1_27modeW<Pinmode3Spec> {
        P1_27modeW::new(self, 22)
    }
    #[doc = "Bits 24:25 - Port 1 pin 28 control."]
    #[inline(always)]
    #[must_use]
    pub fn p1_28mode(&mut self) -> P1_28modeW<Pinmode3Spec> {
        P1_28modeW::new(self, 24)
    }
    #[doc = "Bits 26:27 - Port 1 pin 29 control."]
    #[inline(always)]
    #[must_use]
    pub fn p1_29mode(&mut self) -> P1_29modeW<Pinmode3Spec> {
        P1_29modeW::new(self, 26)
    }
    #[doc = "Bits 28:29 - Port 1 pin 30 control."]
    #[inline(always)]
    #[must_use]
    pub fn p1_30mode(&mut self) -> P1_30modeW<Pinmode3Spec> {
        P1_30modeW::new(self, 28)
    }
    #[doc = "Bits 30:31 - Port 1 pin 31 control."]
    #[inline(always)]
    #[must_use]
    pub fn p1_31mode(&mut self) -> P1_31modeW<Pinmode3Spec> {
        P1_31modeW::new(self, 30)
    }
}
#[doc = "Pin mode select register 3.\n\nYou can [`read`](crate::Reg::read) this register and get [`pinmode3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pinmode3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pinmode3Spec;
impl crate::RegisterSpec for Pinmode3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pinmode3::R`](R) reader structure"]
impl crate::Readable for Pinmode3Spec {}
#[doc = "`write(|w| ..)` method takes [`pinmode3::W`](W) writer structure"]
impl crate::Writable for Pinmode3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PINMODE3 to value 0"]
impl crate::Resettable for Pinmode3Spec {
    const RESET_VALUE: u32 = 0;
}
