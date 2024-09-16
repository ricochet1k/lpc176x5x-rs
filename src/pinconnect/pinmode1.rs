#[doc = "Register `PINMODE1` reader"]
pub type R = crate::R<Pinmode1Spec>;
#[doc = "Register `PINMODE1` writer"]
pub type W = crate::W<Pinmode1Spec>;
#[doc = "Port 1 pin 16 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P0_16mode {
    #[doc = "0: Pull-up. P0.16 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P0.16 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P0.16 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P0.16 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<P0_16mode> for u8 {
    #[inline(always)]
    fn from(variant: P0_16mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P0_16mode {
    type Ux = u8;
}
impl crate::IsEnum for P0_16mode {}
#[doc = "Field `P0_16MODE` reader - Port 1 pin 16 control."]
pub type P0_16modeR = crate::FieldReader<P0_16mode>;
impl P0_16modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_16mode {
        match self.bits {
            0 => P0_16mode::PullUp,
            1 => P0_16mode::Repeater,
            2 => P0_16mode::Disabled,
            3 => P0_16mode::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P0.16 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_16mode::PullUp
    }
    #[doc = "Repeater. P0.16 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P0_16mode::Repeater
    }
    #[doc = "Disabled. P0.16 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P0_16mode::Disabled
    }
    #[doc = "Pull-down. P0.16 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_16mode::PullDown
    }
}
#[doc = "Field `P0_16MODE` writer - Port 1 pin 16 control."]
pub type P0_16modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, P0_16mode, crate::Safe>;
impl<'a, REG> P0_16modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P0.16 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(P0_16mode::PullUp)
    }
    #[doc = "Repeater. P0.16 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(P0_16mode::Repeater)
    }
    #[doc = "Disabled. P0.16 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P0_16mode::Disabled)
    }
    #[doc = "Pull-down. P0.16 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(P0_16mode::PullDown)
    }
}
#[doc = "Port 1 pin 17 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P0_17mode {
    #[doc = "0: Pull-up. P0.17 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P0.17 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P0.17 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P0.17 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<P0_17mode> for u8 {
    #[inline(always)]
    fn from(variant: P0_17mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P0_17mode {
    type Ux = u8;
}
impl crate::IsEnum for P0_17mode {}
#[doc = "Field `P0_17MODE` reader - Port 1 pin 17 control."]
pub type P0_17modeR = crate::FieldReader<P0_17mode>;
impl P0_17modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_17mode {
        match self.bits {
            0 => P0_17mode::PullUp,
            1 => P0_17mode::Repeater,
            2 => P0_17mode::Disabled,
            3 => P0_17mode::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P0.17 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_17mode::PullUp
    }
    #[doc = "Repeater. P0.17 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P0_17mode::Repeater
    }
    #[doc = "Disabled. P0.17 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P0_17mode::Disabled
    }
    #[doc = "Pull-down. P0.17 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_17mode::PullDown
    }
}
#[doc = "Field `P0_17MODE` writer - Port 1 pin 17 control."]
pub type P0_17modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, P0_17mode, crate::Safe>;
impl<'a, REG> P0_17modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P0.17 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(P0_17mode::PullUp)
    }
    #[doc = "Repeater. P0.17 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(P0_17mode::Repeater)
    }
    #[doc = "Disabled. P0.17 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P0_17mode::Disabled)
    }
    #[doc = "Pull-down. P0.17 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(P0_17mode::PullDown)
    }
}
#[doc = "Port 1 pin 18 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P0_18mode {
    #[doc = "0: Pull-up. P0.18 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P0.18 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P0.18 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P0.18 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<P0_18mode> for u8 {
    #[inline(always)]
    fn from(variant: P0_18mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P0_18mode {
    type Ux = u8;
}
impl crate::IsEnum for P0_18mode {}
#[doc = "Field `P0_18MODE` reader - Port 1 pin 18 control."]
pub type P0_18modeR = crate::FieldReader<P0_18mode>;
impl P0_18modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_18mode {
        match self.bits {
            0 => P0_18mode::PullUp,
            1 => P0_18mode::Repeater,
            2 => P0_18mode::Disabled,
            3 => P0_18mode::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P0.18 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_18mode::PullUp
    }
    #[doc = "Repeater. P0.18 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P0_18mode::Repeater
    }
    #[doc = "Disabled. P0.18 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P0_18mode::Disabled
    }
    #[doc = "Pull-down. P0.18 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_18mode::PullDown
    }
}
#[doc = "Field `P0_18MODE` writer - Port 1 pin 18 control."]
pub type P0_18modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, P0_18mode, crate::Safe>;
impl<'a, REG> P0_18modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P0.18 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(P0_18mode::PullUp)
    }
    #[doc = "Repeater. P0.18 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(P0_18mode::Repeater)
    }
    #[doc = "Disabled. P0.18 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P0_18mode::Disabled)
    }
    #[doc = "Pull-down. P0.18 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(P0_18mode::PullDown)
    }
}
#[doc = "Port 1 pin 19 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P0_19mode {
    #[doc = "0: Pull-up. P0.19 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P0.19 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P0.19 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P0.19 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<P0_19mode> for u8 {
    #[inline(always)]
    fn from(variant: P0_19mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P0_19mode {
    type Ux = u8;
}
impl crate::IsEnum for P0_19mode {}
#[doc = "Field `P0_19MODE` reader - Port 1 pin 19 control."]
pub type P0_19modeR = crate::FieldReader<P0_19mode>;
impl P0_19modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_19mode {
        match self.bits {
            0 => P0_19mode::PullUp,
            1 => P0_19mode::Repeater,
            2 => P0_19mode::Disabled,
            3 => P0_19mode::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P0.19 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_19mode::PullUp
    }
    #[doc = "Repeater. P0.19 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P0_19mode::Repeater
    }
    #[doc = "Disabled. P0.19 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P0_19mode::Disabled
    }
    #[doc = "Pull-down. P0.19 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_19mode::PullDown
    }
}
#[doc = "Field `P0_19MODE` writer - Port 1 pin 19 control."]
pub type P0_19modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, P0_19mode, crate::Safe>;
impl<'a, REG> P0_19modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P0.19 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(P0_19mode::PullUp)
    }
    #[doc = "Repeater. P0.19 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(P0_19mode::Repeater)
    }
    #[doc = "Disabled. P0.19 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P0_19mode::Disabled)
    }
    #[doc = "Pull-down. P0.19 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(P0_19mode::PullDown)
    }
}
#[doc = "Port 1 pin 20 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P0_20mode {
    #[doc = "0: Pull-up. P0.20 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P0.20 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P0.20 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P0.20 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<P0_20mode> for u8 {
    #[inline(always)]
    fn from(variant: P0_20mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P0_20mode {
    type Ux = u8;
}
impl crate::IsEnum for P0_20mode {}
#[doc = "Field `P0_20MODE` reader - Port 1 pin 20 control."]
pub type P0_20modeR = crate::FieldReader<P0_20mode>;
impl P0_20modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_20mode {
        match self.bits {
            0 => P0_20mode::PullUp,
            1 => P0_20mode::Repeater,
            2 => P0_20mode::Disabled,
            3 => P0_20mode::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P0.20 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_20mode::PullUp
    }
    #[doc = "Repeater. P0.20 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P0_20mode::Repeater
    }
    #[doc = "Disabled. P0.20 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P0_20mode::Disabled
    }
    #[doc = "Pull-down. P0.20 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_20mode::PullDown
    }
}
#[doc = "Field `P0_20MODE` writer - Port 1 pin 20 control."]
pub type P0_20modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, P0_20mode, crate::Safe>;
impl<'a, REG> P0_20modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P0.20 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(P0_20mode::PullUp)
    }
    #[doc = "Repeater. P0.20 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(P0_20mode::Repeater)
    }
    #[doc = "Disabled. P0.20 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P0_20mode::Disabled)
    }
    #[doc = "Pull-down. P0.20 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(P0_20mode::PullDown)
    }
}
#[doc = "Port 1 pin 21 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P0_21mode {
    #[doc = "0: Pull-up. P0.21 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P0.21 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P0.21 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P0.21 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<P0_21mode> for u8 {
    #[inline(always)]
    fn from(variant: P0_21mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P0_21mode {
    type Ux = u8;
}
impl crate::IsEnum for P0_21mode {}
#[doc = "Field `P0_21MODE` reader - Port 1 pin 21 control."]
pub type P0_21modeR = crate::FieldReader<P0_21mode>;
impl P0_21modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_21mode {
        match self.bits {
            0 => P0_21mode::PullUp,
            1 => P0_21mode::Repeater,
            2 => P0_21mode::Disabled,
            3 => P0_21mode::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P0.21 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_21mode::PullUp
    }
    #[doc = "Repeater. P0.21 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P0_21mode::Repeater
    }
    #[doc = "Disabled. P0.21 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P0_21mode::Disabled
    }
    #[doc = "Pull-down. P0.21 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_21mode::PullDown
    }
}
#[doc = "Field `P0_21MODE` writer - Port 1 pin 21 control."]
pub type P0_21modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, P0_21mode, crate::Safe>;
impl<'a, REG> P0_21modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P0.21 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(P0_21mode::PullUp)
    }
    #[doc = "Repeater. P0.21 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(P0_21mode::Repeater)
    }
    #[doc = "Disabled. P0.21 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P0_21mode::Disabled)
    }
    #[doc = "Pull-down. P0.21 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(P0_21mode::PullDown)
    }
}
#[doc = "Port 1 pin 22 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P0_22mode {
    #[doc = "0: Pull-up. P0.22 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P0.22 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P0.22 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P0.22 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<P0_22mode> for u8 {
    #[inline(always)]
    fn from(variant: P0_22mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P0_22mode {
    type Ux = u8;
}
impl crate::IsEnum for P0_22mode {}
#[doc = "Field `P0_22MODE` reader - Port 1 pin 22 control."]
pub type P0_22modeR = crate::FieldReader<P0_22mode>;
impl P0_22modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_22mode {
        match self.bits {
            0 => P0_22mode::PullUp,
            1 => P0_22mode::Repeater,
            2 => P0_22mode::Disabled,
            3 => P0_22mode::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P0.22 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_22mode::PullUp
    }
    #[doc = "Repeater. P0.22 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P0_22mode::Repeater
    }
    #[doc = "Disabled. P0.22 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P0_22mode::Disabled
    }
    #[doc = "Pull-down. P0.22 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_22mode::PullDown
    }
}
#[doc = "Field `P0_22MODE` writer - Port 1 pin 22 control."]
pub type P0_22modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, P0_22mode, crate::Safe>;
impl<'a, REG> P0_22modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P0.22 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(P0_22mode::PullUp)
    }
    #[doc = "Repeater. P0.22 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(P0_22mode::Repeater)
    }
    #[doc = "Disabled. P0.22 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P0_22mode::Disabled)
    }
    #[doc = "Pull-down. P0.22 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(P0_22mode::PullDown)
    }
}
#[doc = "Port 1 pin 23 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P0_23mode {
    #[doc = "0: Pull-up. P0.23 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P0.23 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P0.23 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P0.23 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<P0_23mode> for u8 {
    #[inline(always)]
    fn from(variant: P0_23mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P0_23mode {
    type Ux = u8;
}
impl crate::IsEnum for P0_23mode {}
#[doc = "Field `P0_23MODE` reader - Port 1 pin 23 control."]
pub type P0_23modeR = crate::FieldReader<P0_23mode>;
impl P0_23modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_23mode {
        match self.bits {
            0 => P0_23mode::PullUp,
            1 => P0_23mode::Repeater,
            2 => P0_23mode::Disabled,
            3 => P0_23mode::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P0.23 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_23mode::PullUp
    }
    #[doc = "Repeater. P0.23 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P0_23mode::Repeater
    }
    #[doc = "Disabled. P0.23 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P0_23mode::Disabled
    }
    #[doc = "Pull-down. P0.23 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_23mode::PullDown
    }
}
#[doc = "Field `P0_23MODE` writer - Port 1 pin 23 control."]
pub type P0_23modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, P0_23mode, crate::Safe>;
impl<'a, REG> P0_23modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P0.23 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(P0_23mode::PullUp)
    }
    #[doc = "Repeater. P0.23 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(P0_23mode::Repeater)
    }
    #[doc = "Disabled. P0.23 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P0_23mode::Disabled)
    }
    #[doc = "Pull-down. P0.23 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(P0_23mode::PullDown)
    }
}
#[doc = "Port 1 pin 24 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P0_24mode {
    #[doc = "0: Pull-up. P0.24 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P0.24 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P0.24 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P0.24 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<P0_24mode> for u8 {
    #[inline(always)]
    fn from(variant: P0_24mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P0_24mode {
    type Ux = u8;
}
impl crate::IsEnum for P0_24mode {}
#[doc = "Field `P0_24MODE` reader - Port 1 pin 24 control."]
pub type P0_24modeR = crate::FieldReader<P0_24mode>;
impl P0_24modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_24mode {
        match self.bits {
            0 => P0_24mode::PullUp,
            1 => P0_24mode::Repeater,
            2 => P0_24mode::Disabled,
            3 => P0_24mode::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P0.24 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_24mode::PullUp
    }
    #[doc = "Repeater. P0.24 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P0_24mode::Repeater
    }
    #[doc = "Disabled. P0.24 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P0_24mode::Disabled
    }
    #[doc = "Pull-down. P0.24 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_24mode::PullDown
    }
}
#[doc = "Field `P0_24MODE` writer - Port 1 pin 24 control."]
pub type P0_24modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, P0_24mode, crate::Safe>;
impl<'a, REG> P0_24modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P0.24 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(P0_24mode::PullUp)
    }
    #[doc = "Repeater. P0.24 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(P0_24mode::Repeater)
    }
    #[doc = "Disabled. P0.24 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P0_24mode::Disabled)
    }
    #[doc = "Pull-down. P0.24 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(P0_24mode::PullDown)
    }
}
#[doc = "Port 1 pin 25 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P0_25mode {
    #[doc = "0: Pull-up. P0.25 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P0.25 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P0.25 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P0.25 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<P0_25mode> for u8 {
    #[inline(always)]
    fn from(variant: P0_25mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P0_25mode {
    type Ux = u8;
}
impl crate::IsEnum for P0_25mode {}
#[doc = "Field `P0_25MODE` reader - Port 1 pin 25 control."]
pub type P0_25modeR = crate::FieldReader<P0_25mode>;
impl P0_25modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_25mode {
        match self.bits {
            0 => P0_25mode::PullUp,
            1 => P0_25mode::Repeater,
            2 => P0_25mode::Disabled,
            3 => P0_25mode::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P0.25 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_25mode::PullUp
    }
    #[doc = "Repeater. P0.25 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P0_25mode::Repeater
    }
    #[doc = "Disabled. P0.25 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P0_25mode::Disabled
    }
    #[doc = "Pull-down. P0.25 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_25mode::PullDown
    }
}
#[doc = "Field `P0_25MODE` writer - Port 1 pin 25 control."]
pub type P0_25modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, P0_25mode, crate::Safe>;
impl<'a, REG> P0_25modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P0.25 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(P0_25mode::PullUp)
    }
    #[doc = "Repeater. P0.25 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(P0_25mode::Repeater)
    }
    #[doc = "Disabled. P0.25 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P0_25mode::Disabled)
    }
    #[doc = "Pull-down. P0.25 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(P0_25mode::PullDown)
    }
}
#[doc = "Port 1 pin 26 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P0_26mode {
    #[doc = "0: Pull-up. P0.26 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P0.26 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P0.26 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P0.26 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<P0_26mode> for u8 {
    #[inline(always)]
    fn from(variant: P0_26mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P0_26mode {
    type Ux = u8;
}
impl crate::IsEnum for P0_26mode {}
#[doc = "Field `P0_26MODE` reader - Port 1 pin 26 control."]
pub type P0_26modeR = crate::FieldReader<P0_26mode>;
impl P0_26modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_26mode {
        match self.bits {
            0 => P0_26mode::PullUp,
            1 => P0_26mode::Repeater,
            2 => P0_26mode::Disabled,
            3 => P0_26mode::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P0.26 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_26mode::PullUp
    }
    #[doc = "Repeater. P0.26 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P0_26mode::Repeater
    }
    #[doc = "Disabled. P0.26 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P0_26mode::Disabled
    }
    #[doc = "Pull-down. P0.26 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_26mode::PullDown
    }
}
#[doc = "Field `P0_26MODE` writer - Port 1 pin 26 control."]
pub type P0_26modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, P0_26mode, crate::Safe>;
impl<'a, REG> P0_26modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P0.26 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(P0_26mode::PullUp)
    }
    #[doc = "Repeater. P0.26 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(P0_26mode::Repeater)
    }
    #[doc = "Disabled. P0.26 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P0_26mode::Disabled)
    }
    #[doc = "Pull-down. P0.26 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(P0_26mode::PullDown)
    }
}
impl R {
    #[doc = "Bits 0:1 - Port 1 pin 16 control."]
    #[inline(always)]
    pub fn p0_16mode(&self) -> P0_16modeR {
        P0_16modeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port 1 pin 17 control."]
    #[inline(always)]
    pub fn p0_17mode(&self) -> P0_17modeR {
        P0_17modeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port 1 pin 18 control."]
    #[inline(always)]
    pub fn p0_18mode(&self) -> P0_18modeR {
        P0_18modeR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port 1 pin 19 control."]
    #[inline(always)]
    pub fn p0_19mode(&self) -> P0_19modeR {
        P0_19modeR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port 1 pin 20 control."]
    #[inline(always)]
    pub fn p0_20mode(&self) -> P0_20modeR {
        P0_20modeR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port 1 pin 21 control."]
    #[inline(always)]
    pub fn p0_21mode(&self) -> P0_21modeR {
        P0_21modeR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port 1 pin 22 control."]
    #[inline(always)]
    pub fn p0_22mode(&self) -> P0_22modeR {
        P0_22modeR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port 1 pin 23 control."]
    #[inline(always)]
    pub fn p0_23mode(&self) -> P0_23modeR {
        P0_23modeR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port 1 pin 24 control."]
    #[inline(always)]
    pub fn p0_24mode(&self) -> P0_24modeR {
        P0_24modeR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Port 1 pin 25 control."]
    #[inline(always)]
    pub fn p0_25mode(&self) -> P0_25modeR {
        P0_25modeR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Port 1 pin 26 control."]
    #[inline(always)]
    pub fn p0_26mode(&self) -> P0_26modeR {
        P0_26modeR::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port 1 pin 16 control."]
    #[inline(always)]
    #[must_use]
    pub fn p0_16mode(&mut self) -> P0_16modeW<Pinmode1Spec> {
        P0_16modeW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Port 1 pin 17 control."]
    #[inline(always)]
    #[must_use]
    pub fn p0_17mode(&mut self) -> P0_17modeW<Pinmode1Spec> {
        P0_17modeW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Port 1 pin 18 control."]
    #[inline(always)]
    #[must_use]
    pub fn p0_18mode(&mut self) -> P0_18modeW<Pinmode1Spec> {
        P0_18modeW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Port 1 pin 19 control."]
    #[inline(always)]
    #[must_use]
    pub fn p0_19mode(&mut self) -> P0_19modeW<Pinmode1Spec> {
        P0_19modeW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Port 1 pin 20 control."]
    #[inline(always)]
    #[must_use]
    pub fn p0_20mode(&mut self) -> P0_20modeW<Pinmode1Spec> {
        P0_20modeW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Port 1 pin 21 control."]
    #[inline(always)]
    #[must_use]
    pub fn p0_21mode(&mut self) -> P0_21modeW<Pinmode1Spec> {
        P0_21modeW::new(self, 10)
    }
    #[doc = "Bits 12:13 - Port 1 pin 22 control."]
    #[inline(always)]
    #[must_use]
    pub fn p0_22mode(&mut self) -> P0_22modeW<Pinmode1Spec> {
        P0_22modeW::new(self, 12)
    }
    #[doc = "Bits 14:15 - Port 1 pin 23 control."]
    #[inline(always)]
    #[must_use]
    pub fn p0_23mode(&mut self) -> P0_23modeW<Pinmode1Spec> {
        P0_23modeW::new(self, 14)
    }
    #[doc = "Bits 16:17 - Port 1 pin 24 control."]
    #[inline(always)]
    #[must_use]
    pub fn p0_24mode(&mut self) -> P0_24modeW<Pinmode1Spec> {
        P0_24modeW::new(self, 16)
    }
    #[doc = "Bits 18:19 - Port 1 pin 25 control."]
    #[inline(always)]
    #[must_use]
    pub fn p0_25mode(&mut self) -> P0_25modeW<Pinmode1Spec> {
        P0_25modeW::new(self, 18)
    }
    #[doc = "Bits 20:21 - Port 1 pin 26 control."]
    #[inline(always)]
    #[must_use]
    pub fn p0_26mode(&mut self) -> P0_26modeW<Pinmode1Spec> {
        P0_26modeW::new(self, 20)
    }
}
#[doc = "Pin mode select register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pinmode1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pinmode1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pinmode1Spec;
impl crate::RegisterSpec for Pinmode1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pinmode1::R`](R) reader structure"]
impl crate::Readable for Pinmode1Spec {}
#[doc = "`write(|w| ..)` method takes [`pinmode1::W`](W) writer structure"]
impl crate::Writable for Pinmode1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PINMODE1 to value 0"]
impl crate::Resettable for Pinmode1Spec {
    const RESET_VALUE: u32 = 0;
}
