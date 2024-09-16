#[doc = "Register `PINMODE0` reader"]
pub type R = crate::R<Pinmode0Spec>;
#[doc = "Register `PINMODE0` writer"]
pub type W = crate::W<Pinmode0Spec>;
#[doc = "Port 0 pin 0 on-chip pull-up/down resistor control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P0_00mode {
    #[doc = "0: Pull-up. P0.0 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P0.0 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P0.0 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P0.0 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<P0_00mode> for u8 {
    #[inline(always)]
    fn from(variant: P0_00mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P0_00mode {
    type Ux = u8;
}
impl crate::IsEnum for P0_00mode {}
#[doc = "Field `P0_00MODE` reader - Port 0 pin 0 on-chip pull-up/down resistor control."]
pub type P0_00modeR = crate::FieldReader<P0_00mode>;
impl P0_00modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_00mode {
        match self.bits {
            0 => P0_00mode::PullUp,
            1 => P0_00mode::Repeater,
            2 => P0_00mode::Disabled,
            3 => P0_00mode::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P0.0 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_00mode::PullUp
    }
    #[doc = "Repeater. P0.0 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P0_00mode::Repeater
    }
    #[doc = "Disabled. P0.0 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P0_00mode::Disabled
    }
    #[doc = "Pull-down. P0.0 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_00mode::PullDown
    }
}
#[doc = "Field `P0_00MODE` writer - Port 0 pin 0 on-chip pull-up/down resistor control."]
pub type P0_00modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, P0_00mode, crate::Safe>;
impl<'a, REG> P0_00modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P0.0 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(P0_00mode::PullUp)
    }
    #[doc = "Repeater. P0.0 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(P0_00mode::Repeater)
    }
    #[doc = "Disabled. P0.0 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P0_00mode::Disabled)
    }
    #[doc = "Pull-down. P0.0 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(P0_00mode::PullDown)
    }
}
#[doc = "Port 0 pin 1 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P0_01mode {
    #[doc = "0: Pull-up. P0.1 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P0.1 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P0.1 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P0.1 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<P0_01mode> for u8 {
    #[inline(always)]
    fn from(variant: P0_01mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P0_01mode {
    type Ux = u8;
}
impl crate::IsEnum for P0_01mode {}
#[doc = "Field `P0_01MODE` reader - Port 0 pin 1 control."]
pub type P0_01modeR = crate::FieldReader<P0_01mode>;
impl P0_01modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_01mode {
        match self.bits {
            0 => P0_01mode::PullUp,
            1 => P0_01mode::Repeater,
            2 => P0_01mode::Disabled,
            3 => P0_01mode::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P0.1 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_01mode::PullUp
    }
    #[doc = "Repeater. P0.1 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P0_01mode::Repeater
    }
    #[doc = "Disabled. P0.1 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P0_01mode::Disabled
    }
    #[doc = "Pull-down. P0.1 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_01mode::PullDown
    }
}
#[doc = "Field `P0_01MODE` writer - Port 0 pin 1 control."]
pub type P0_01modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, P0_01mode, crate::Safe>;
impl<'a, REG> P0_01modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P0.1 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(P0_01mode::PullUp)
    }
    #[doc = "Repeater. P0.1 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(P0_01mode::Repeater)
    }
    #[doc = "Disabled. P0.1 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P0_01mode::Disabled)
    }
    #[doc = "Pull-down. P0.1 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(P0_01mode::PullDown)
    }
}
#[doc = "Port 0 pin 2 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P0_02mode {
    #[doc = "0: Pull-up. P0.2 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P0.2 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P0.2 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P0.2 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<P0_02mode> for u8 {
    #[inline(always)]
    fn from(variant: P0_02mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P0_02mode {
    type Ux = u8;
}
impl crate::IsEnum for P0_02mode {}
#[doc = "Field `P0_02MODE` reader - Port 0 pin 2 control."]
pub type P0_02modeR = crate::FieldReader<P0_02mode>;
impl P0_02modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_02mode {
        match self.bits {
            0 => P0_02mode::PullUp,
            1 => P0_02mode::Repeater,
            2 => P0_02mode::Disabled,
            3 => P0_02mode::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P0.2 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_02mode::PullUp
    }
    #[doc = "Repeater. P0.2 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P0_02mode::Repeater
    }
    #[doc = "Disabled. P0.2 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P0_02mode::Disabled
    }
    #[doc = "Pull-down. P0.2 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_02mode::PullDown
    }
}
#[doc = "Field `P0_02MODE` writer - Port 0 pin 2 control."]
pub type P0_02modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, P0_02mode, crate::Safe>;
impl<'a, REG> P0_02modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P0.2 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(P0_02mode::PullUp)
    }
    #[doc = "Repeater. P0.2 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(P0_02mode::Repeater)
    }
    #[doc = "Disabled. P0.2 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P0_02mode::Disabled)
    }
    #[doc = "Pull-down. P0.2 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(P0_02mode::PullDown)
    }
}
#[doc = "Port 0 pin 3 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P0_03mode {
    #[doc = "0: Pull-up. P0.3 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P0.3 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P0.3 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P0.3 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<P0_03mode> for u8 {
    #[inline(always)]
    fn from(variant: P0_03mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P0_03mode {
    type Ux = u8;
}
impl crate::IsEnum for P0_03mode {}
#[doc = "Field `P0_03MODE` reader - Port 0 pin 3 control."]
pub type P0_03modeR = crate::FieldReader<P0_03mode>;
impl P0_03modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_03mode {
        match self.bits {
            0 => P0_03mode::PullUp,
            1 => P0_03mode::Repeater,
            2 => P0_03mode::Disabled,
            3 => P0_03mode::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P0.3 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_03mode::PullUp
    }
    #[doc = "Repeater. P0.3 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P0_03mode::Repeater
    }
    #[doc = "Disabled. P0.3 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P0_03mode::Disabled
    }
    #[doc = "Pull-down. P0.3 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_03mode::PullDown
    }
}
#[doc = "Field `P0_03MODE` writer - Port 0 pin 3 control."]
pub type P0_03modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, P0_03mode, crate::Safe>;
impl<'a, REG> P0_03modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P0.3 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(P0_03mode::PullUp)
    }
    #[doc = "Repeater. P0.3 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(P0_03mode::Repeater)
    }
    #[doc = "Disabled. P0.3 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P0_03mode::Disabled)
    }
    #[doc = "Pull-down. P0.3 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(P0_03mode::PullDown)
    }
}
#[doc = "Port 0 pin 4 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P0_04mode {
    #[doc = "0: Pull-up. P0.4 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P0.4 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P0.4 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P0.4 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<P0_04mode> for u8 {
    #[inline(always)]
    fn from(variant: P0_04mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P0_04mode {
    type Ux = u8;
}
impl crate::IsEnum for P0_04mode {}
#[doc = "Field `P0_04MODE` reader - Port 0 pin 4 control."]
pub type P0_04modeR = crate::FieldReader<P0_04mode>;
impl P0_04modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_04mode {
        match self.bits {
            0 => P0_04mode::PullUp,
            1 => P0_04mode::Repeater,
            2 => P0_04mode::Disabled,
            3 => P0_04mode::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P0.4 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_04mode::PullUp
    }
    #[doc = "Repeater. P0.4 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P0_04mode::Repeater
    }
    #[doc = "Disabled. P0.4 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P0_04mode::Disabled
    }
    #[doc = "Pull-down. P0.4 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_04mode::PullDown
    }
}
#[doc = "Field `P0_04MODE` writer - Port 0 pin 4 control."]
pub type P0_04modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, P0_04mode, crate::Safe>;
impl<'a, REG> P0_04modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P0.4 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(P0_04mode::PullUp)
    }
    #[doc = "Repeater. P0.4 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(P0_04mode::Repeater)
    }
    #[doc = "Disabled. P0.4 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P0_04mode::Disabled)
    }
    #[doc = "Pull-down. P0.4 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(P0_04mode::PullDown)
    }
}
#[doc = "Port 0 pin 5 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P0_05mode {
    #[doc = "0: Pull-up. P0.5 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P0.5 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P0.5 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P0.5 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<P0_05mode> for u8 {
    #[inline(always)]
    fn from(variant: P0_05mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P0_05mode {
    type Ux = u8;
}
impl crate::IsEnum for P0_05mode {}
#[doc = "Field `P0_05MODE` reader - Port 0 pin 5 control."]
pub type P0_05modeR = crate::FieldReader<P0_05mode>;
impl P0_05modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_05mode {
        match self.bits {
            0 => P0_05mode::PullUp,
            1 => P0_05mode::Repeater,
            2 => P0_05mode::Disabled,
            3 => P0_05mode::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P0.5 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_05mode::PullUp
    }
    #[doc = "Repeater. P0.5 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P0_05mode::Repeater
    }
    #[doc = "Disabled. P0.5 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P0_05mode::Disabled
    }
    #[doc = "Pull-down. P0.5 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_05mode::PullDown
    }
}
#[doc = "Field `P0_05MODE` writer - Port 0 pin 5 control."]
pub type P0_05modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, P0_05mode, crate::Safe>;
impl<'a, REG> P0_05modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P0.5 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(P0_05mode::PullUp)
    }
    #[doc = "Repeater. P0.5 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(P0_05mode::Repeater)
    }
    #[doc = "Disabled. P0.5 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P0_05mode::Disabled)
    }
    #[doc = "Pull-down. P0.5 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(P0_05mode::PullDown)
    }
}
#[doc = "Port 0 pin 6 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P0_06mode {
    #[doc = "0: Pull-up. P0.6 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Disabled. Repeater. P0.6 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P0.6 pin has neither pull-up nor pull-down."]
    Neither = 2,
    #[doc = "3: Pull-down. P0.6 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<P0_06mode> for u8 {
    #[inline(always)]
    fn from(variant: P0_06mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P0_06mode {
    type Ux = u8;
}
impl crate::IsEnum for P0_06mode {}
#[doc = "Field `P0_06MODE` reader - Port 0 pin 6 control."]
pub type P0_06modeR = crate::FieldReader<P0_06mode>;
impl P0_06modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_06mode {
        match self.bits {
            0 => P0_06mode::PullUp,
            1 => P0_06mode::Repeater,
            2 => P0_06mode::Neither,
            3 => P0_06mode::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P0.6 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_06mode::PullUp
    }
    #[doc = "Disabled. Repeater. P0.6 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P0_06mode::Repeater
    }
    #[doc = "Disabled. P0.6 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_neither(&self) -> bool {
        *self == P0_06mode::Neither
    }
    #[doc = "Pull-down. P0.6 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_06mode::PullDown
    }
}
#[doc = "Field `P0_06MODE` writer - Port 0 pin 6 control."]
pub type P0_06modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, P0_06mode, crate::Safe>;
impl<'a, REG> P0_06modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P0.6 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(P0_06mode::PullUp)
    }
    #[doc = "Disabled. Repeater. P0.6 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(P0_06mode::Repeater)
    }
    #[doc = "Disabled. P0.6 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn neither(self) -> &'a mut crate::W<REG> {
        self.variant(P0_06mode::Neither)
    }
    #[doc = "Pull-down. P0.6 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(P0_06mode::PullDown)
    }
}
#[doc = "Port 0 pin 7 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P0_07mode {
    #[doc = "0: Pull-up. P0.7 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P0.7 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P0.7 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P0.7 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<P0_07mode> for u8 {
    #[inline(always)]
    fn from(variant: P0_07mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P0_07mode {
    type Ux = u8;
}
impl crate::IsEnum for P0_07mode {}
#[doc = "Field `P0_07MODE` reader - Port 0 pin 7 control."]
pub type P0_07modeR = crate::FieldReader<P0_07mode>;
impl P0_07modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_07mode {
        match self.bits {
            0 => P0_07mode::PullUp,
            1 => P0_07mode::Repeater,
            2 => P0_07mode::Disabled,
            3 => P0_07mode::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P0.7 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_07mode::PullUp
    }
    #[doc = "Repeater. P0.7 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P0_07mode::Repeater
    }
    #[doc = "Disabled. P0.7 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P0_07mode::Disabled
    }
    #[doc = "Pull-down. P0.7 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_07mode::PullDown
    }
}
#[doc = "Field `P0_07MODE` writer - Port 0 pin 7 control."]
pub type P0_07modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, P0_07mode, crate::Safe>;
impl<'a, REG> P0_07modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P0.7 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(P0_07mode::PullUp)
    }
    #[doc = "Repeater. P0.7 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(P0_07mode::Repeater)
    }
    #[doc = "Disabled. P0.7 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P0_07mode::Disabled)
    }
    #[doc = "Pull-down. P0.7 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(P0_07mode::PullDown)
    }
}
#[doc = "Port 0 pin 8 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P0_08mode {
    #[doc = "0: Pull-up. P0.8 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P0.8 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P0.8 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P0.8 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<P0_08mode> for u8 {
    #[inline(always)]
    fn from(variant: P0_08mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P0_08mode {
    type Ux = u8;
}
impl crate::IsEnum for P0_08mode {}
#[doc = "Field `P0_08MODE` reader - Port 0 pin 8 control."]
pub type P0_08modeR = crate::FieldReader<P0_08mode>;
impl P0_08modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_08mode {
        match self.bits {
            0 => P0_08mode::PullUp,
            1 => P0_08mode::Repeater,
            2 => P0_08mode::Disabled,
            3 => P0_08mode::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P0.8 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_08mode::PullUp
    }
    #[doc = "Repeater. P0.8 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P0_08mode::Repeater
    }
    #[doc = "Disabled. P0.8 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P0_08mode::Disabled
    }
    #[doc = "Pull-down. P0.8 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_08mode::PullDown
    }
}
#[doc = "Field `P0_08MODE` writer - Port 0 pin 8 control."]
pub type P0_08modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, P0_08mode, crate::Safe>;
impl<'a, REG> P0_08modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P0.8 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(P0_08mode::PullUp)
    }
    #[doc = "Repeater. P0.8 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(P0_08mode::Repeater)
    }
    #[doc = "Disabled. P0.8 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P0_08mode::Disabled)
    }
    #[doc = "Pull-down. P0.8 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(P0_08mode::PullDown)
    }
}
#[doc = "Port 0 pin 9 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P0_09mode {
    #[doc = "0: Pull-up. P0.9 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P0.9 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P0.9 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P0.9 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<P0_09mode> for u8 {
    #[inline(always)]
    fn from(variant: P0_09mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P0_09mode {
    type Ux = u8;
}
impl crate::IsEnum for P0_09mode {}
#[doc = "Field `P0_09MODE` reader - Port 0 pin 9 control."]
pub type P0_09modeR = crate::FieldReader<P0_09mode>;
impl P0_09modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_09mode {
        match self.bits {
            0 => P0_09mode::PullUp,
            1 => P0_09mode::Repeater,
            2 => P0_09mode::Disabled,
            3 => P0_09mode::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P0.9 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_09mode::PullUp
    }
    #[doc = "Repeater. P0.9 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P0_09mode::Repeater
    }
    #[doc = "Disabled. P0.9 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P0_09mode::Disabled
    }
    #[doc = "Pull-down. P0.9 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_09mode::PullDown
    }
}
#[doc = "Field `P0_09MODE` writer - Port 0 pin 9 control."]
pub type P0_09modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, P0_09mode, crate::Safe>;
impl<'a, REG> P0_09modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P0.9 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(P0_09mode::PullUp)
    }
    #[doc = "Repeater. P0.9 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(P0_09mode::Repeater)
    }
    #[doc = "Disabled. P0.9 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P0_09mode::Disabled)
    }
    #[doc = "Pull-down. P0.9 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(P0_09mode::PullDown)
    }
}
#[doc = "Port 0 pin 10 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P0_10mode {
    #[doc = "0: Pull-up. P0.10 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P0.10 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P0.10 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P0.10 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<P0_10mode> for u8 {
    #[inline(always)]
    fn from(variant: P0_10mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P0_10mode {
    type Ux = u8;
}
impl crate::IsEnum for P0_10mode {}
#[doc = "Field `P0_10MODE` reader - Port 0 pin 10 control."]
pub type P0_10modeR = crate::FieldReader<P0_10mode>;
impl P0_10modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_10mode {
        match self.bits {
            0 => P0_10mode::PullUp,
            1 => P0_10mode::Repeater,
            2 => P0_10mode::Disabled,
            3 => P0_10mode::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P0.10 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_10mode::PullUp
    }
    #[doc = "Repeater. P0.10 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P0_10mode::Repeater
    }
    #[doc = "Disabled. P0.10 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P0_10mode::Disabled
    }
    #[doc = "Pull-down. P0.10 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_10mode::PullDown
    }
}
#[doc = "Field `P0_10MODE` writer - Port 0 pin 10 control."]
pub type P0_10modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, P0_10mode, crate::Safe>;
impl<'a, REG> P0_10modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P0.10 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(P0_10mode::PullUp)
    }
    #[doc = "Repeater. P0.10 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(P0_10mode::Repeater)
    }
    #[doc = "Disabled. P0.10 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P0_10mode::Disabled)
    }
    #[doc = "Pull-down. P0.10 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(P0_10mode::PullDown)
    }
}
#[doc = "Port 0 pin 11 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P0_11mode {
    #[doc = "0: Pull-up. P0.11 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P0.11 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P0.11 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P0.11 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<P0_11mode> for u8 {
    #[inline(always)]
    fn from(variant: P0_11mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P0_11mode {
    type Ux = u8;
}
impl crate::IsEnum for P0_11mode {}
#[doc = "Field `P0_11MODE` reader - Port 0 pin 11 control."]
pub type P0_11modeR = crate::FieldReader<P0_11mode>;
impl P0_11modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_11mode {
        match self.bits {
            0 => P0_11mode::PullUp,
            1 => P0_11mode::Repeater,
            2 => P0_11mode::Disabled,
            3 => P0_11mode::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P0.11 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_11mode::PullUp
    }
    #[doc = "Repeater. P0.11 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P0_11mode::Repeater
    }
    #[doc = "Disabled. P0.11 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P0_11mode::Disabled
    }
    #[doc = "Pull-down. P0.11 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_11mode::PullDown
    }
}
#[doc = "Field `P0_11MODE` writer - Port 0 pin 11 control."]
pub type P0_11modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, P0_11mode, crate::Safe>;
impl<'a, REG> P0_11modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P0.11 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(P0_11mode::PullUp)
    }
    #[doc = "Repeater. P0.11 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(P0_11mode::Repeater)
    }
    #[doc = "Disabled. P0.11 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P0_11mode::Disabled)
    }
    #[doc = "Pull-down. P0.11 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(P0_11mode::PullDown)
    }
}
#[doc = "Port 0 pin 15 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P0_15mode {
    #[doc = "0: Pull-up. P0.15 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P0.15 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P0.15 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P0.15 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<P0_15mode> for u8 {
    #[inline(always)]
    fn from(variant: P0_15mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P0_15mode {
    type Ux = u8;
}
impl crate::IsEnum for P0_15mode {}
#[doc = "Field `P0_15MODE` reader - Port 0 pin 15 control."]
pub type P0_15modeR = crate::FieldReader<P0_15mode>;
impl P0_15modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_15mode {
        match self.bits {
            0 => P0_15mode::PullUp,
            1 => P0_15mode::Repeater,
            2 => P0_15mode::Disabled,
            3 => P0_15mode::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P0.15 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_15mode::PullUp
    }
    #[doc = "Repeater. P0.15 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P0_15mode::Repeater
    }
    #[doc = "Disabled. P0.15 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P0_15mode::Disabled
    }
    #[doc = "Pull-down. P0.15 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_15mode::PullDown
    }
}
#[doc = "Field `P0_15MODE` writer - Port 0 pin 15 control."]
pub type P0_15modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, P0_15mode, crate::Safe>;
impl<'a, REG> P0_15modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P0.15 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(P0_15mode::PullUp)
    }
    #[doc = "Repeater. P0.15 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(P0_15mode::Repeater)
    }
    #[doc = "Disabled. P0.15 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P0_15mode::Disabled)
    }
    #[doc = "Pull-down. P0.15 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(P0_15mode::PullDown)
    }
}
impl R {
    #[doc = "Bits 0:1 - Port 0 pin 0 on-chip pull-up/down resistor control."]
    #[inline(always)]
    pub fn p0_00mode(&self) -> P0_00modeR {
        P0_00modeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port 0 pin 1 control."]
    #[inline(always)]
    pub fn p0_01mode(&self) -> P0_01modeR {
        P0_01modeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port 0 pin 2 control."]
    #[inline(always)]
    pub fn p0_02mode(&self) -> P0_02modeR {
        P0_02modeR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port 0 pin 3 control."]
    #[inline(always)]
    pub fn p0_03mode(&self) -> P0_03modeR {
        P0_03modeR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port 0 pin 4 control."]
    #[inline(always)]
    pub fn p0_04mode(&self) -> P0_04modeR {
        P0_04modeR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port 0 pin 5 control."]
    #[inline(always)]
    pub fn p0_05mode(&self) -> P0_05modeR {
        P0_05modeR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port 0 pin 6 control."]
    #[inline(always)]
    pub fn p0_06mode(&self) -> P0_06modeR {
        P0_06modeR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port 0 pin 7 control."]
    #[inline(always)]
    pub fn p0_07mode(&self) -> P0_07modeR {
        P0_07modeR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port 0 pin 8 control."]
    #[inline(always)]
    pub fn p0_08mode(&self) -> P0_08modeR {
        P0_08modeR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Port 0 pin 9 control."]
    #[inline(always)]
    pub fn p0_09mode(&self) -> P0_09modeR {
        P0_09modeR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Port 0 pin 10 control."]
    #[inline(always)]
    pub fn p0_10mode(&self) -> P0_10modeR {
        P0_10modeR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Port 0 pin 11 control."]
    #[inline(always)]
    pub fn p0_11mode(&self) -> P0_11modeR {
        P0_11modeR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Port 0 pin 15 control."]
    #[inline(always)]
    pub fn p0_15mode(&self) -> P0_15modeR {
        P0_15modeR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port 0 pin 0 on-chip pull-up/down resistor control."]
    #[inline(always)]
    #[must_use]
    pub fn p0_00mode(&mut self) -> P0_00modeW<Pinmode0Spec> {
        P0_00modeW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Port 0 pin 1 control."]
    #[inline(always)]
    #[must_use]
    pub fn p0_01mode(&mut self) -> P0_01modeW<Pinmode0Spec> {
        P0_01modeW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Port 0 pin 2 control."]
    #[inline(always)]
    #[must_use]
    pub fn p0_02mode(&mut self) -> P0_02modeW<Pinmode0Spec> {
        P0_02modeW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Port 0 pin 3 control."]
    #[inline(always)]
    #[must_use]
    pub fn p0_03mode(&mut self) -> P0_03modeW<Pinmode0Spec> {
        P0_03modeW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Port 0 pin 4 control."]
    #[inline(always)]
    #[must_use]
    pub fn p0_04mode(&mut self) -> P0_04modeW<Pinmode0Spec> {
        P0_04modeW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Port 0 pin 5 control."]
    #[inline(always)]
    #[must_use]
    pub fn p0_05mode(&mut self) -> P0_05modeW<Pinmode0Spec> {
        P0_05modeW::new(self, 10)
    }
    #[doc = "Bits 12:13 - Port 0 pin 6 control."]
    #[inline(always)]
    #[must_use]
    pub fn p0_06mode(&mut self) -> P0_06modeW<Pinmode0Spec> {
        P0_06modeW::new(self, 12)
    }
    #[doc = "Bits 14:15 - Port 0 pin 7 control."]
    #[inline(always)]
    #[must_use]
    pub fn p0_07mode(&mut self) -> P0_07modeW<Pinmode0Spec> {
        P0_07modeW::new(self, 14)
    }
    #[doc = "Bits 16:17 - Port 0 pin 8 control."]
    #[inline(always)]
    #[must_use]
    pub fn p0_08mode(&mut self) -> P0_08modeW<Pinmode0Spec> {
        P0_08modeW::new(self, 16)
    }
    #[doc = "Bits 18:19 - Port 0 pin 9 control."]
    #[inline(always)]
    #[must_use]
    pub fn p0_09mode(&mut self) -> P0_09modeW<Pinmode0Spec> {
        P0_09modeW::new(self, 18)
    }
    #[doc = "Bits 20:21 - Port 0 pin 10 control."]
    #[inline(always)]
    #[must_use]
    pub fn p0_10mode(&mut self) -> P0_10modeW<Pinmode0Spec> {
        P0_10modeW::new(self, 20)
    }
    #[doc = "Bits 22:23 - Port 0 pin 11 control."]
    #[inline(always)]
    #[must_use]
    pub fn p0_11mode(&mut self) -> P0_11modeW<Pinmode0Spec> {
        P0_11modeW::new(self, 22)
    }
    #[doc = "Bits 30:31 - Port 0 pin 15 control."]
    #[inline(always)]
    #[must_use]
    pub fn p0_15mode(&mut self) -> P0_15modeW<Pinmode0Spec> {
        P0_15modeW::new(self, 30)
    }
}
#[doc = "Pin mode select register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pinmode0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pinmode0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pinmode0Spec;
impl crate::RegisterSpec for Pinmode0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pinmode0::R`](R) reader structure"]
impl crate::Readable for Pinmode0Spec {}
#[doc = "`write(|w| ..)` method takes [`pinmode0::W`](W) writer structure"]
impl crate::Writable for Pinmode0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PINMODE0 to value 0"]
impl crate::Resettable for Pinmode0Spec {
    const RESET_VALUE: u32 = 0;
}
