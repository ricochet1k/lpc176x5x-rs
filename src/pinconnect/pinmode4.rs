#[doc = "Register `PINMODE4` reader"]
pub type R = crate::R<Pinmode4Spec>;
#[doc = "Register `PINMODE4` writer"]
pub type W = crate::W<Pinmode4Spec>;
#[doc = "Port 2 pin 0 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P2_00mode {
    #[doc = "0: Pull-up. P2.0 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P2.0 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P2.0 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P2.0 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<P2_00mode> for u8 {
    #[inline(always)]
    fn from(variant: P2_00mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P2_00mode {
    type Ux = u8;
}
impl crate::IsEnum for P2_00mode {}
#[doc = "Field `P2_00MODE` reader - Port 2 pin 0 control."]
pub type P2_00modeR = crate::FieldReader<P2_00mode>;
impl P2_00modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P2_00mode {
        match self.bits {
            0 => P2_00mode::PullUp,
            1 => P2_00mode::Repeater,
            2 => P2_00mode::Disabled,
            3 => P2_00mode::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P2.0 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P2_00mode::PullUp
    }
    #[doc = "Repeater. P2.0 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P2_00mode::Repeater
    }
    #[doc = "Disabled. P2.0 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P2_00mode::Disabled
    }
    #[doc = "Pull-down. P2.0 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P2_00mode::PullDown
    }
}
#[doc = "Field `P2_00MODE` writer - Port 2 pin 0 control."]
pub type P2_00modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, P2_00mode, crate::Safe>;
impl<'a, REG> P2_00modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P2.0 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(P2_00mode::PullUp)
    }
    #[doc = "Repeater. P2.0 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(P2_00mode::Repeater)
    }
    #[doc = "Disabled. P2.0 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P2_00mode::Disabled)
    }
    #[doc = "Pull-down. P2.0 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(P2_00mode::PullDown)
    }
}
#[doc = "Port 2 pin 1 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P2_01mode {
    #[doc = "0: Pull-up. P2.1 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P2.1 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P2.1 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P2.1 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<P2_01mode> for u8 {
    #[inline(always)]
    fn from(variant: P2_01mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P2_01mode {
    type Ux = u8;
}
impl crate::IsEnum for P2_01mode {}
#[doc = "Field `P2_01MODE` reader - Port 2 pin 1 control."]
pub type P2_01modeR = crate::FieldReader<P2_01mode>;
impl P2_01modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P2_01mode {
        match self.bits {
            0 => P2_01mode::PullUp,
            1 => P2_01mode::Repeater,
            2 => P2_01mode::Disabled,
            3 => P2_01mode::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P2.1 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P2_01mode::PullUp
    }
    #[doc = "Repeater. P2.1 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P2_01mode::Repeater
    }
    #[doc = "Disabled. P2.1 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P2_01mode::Disabled
    }
    #[doc = "Pull-down. P2.1 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P2_01mode::PullDown
    }
}
#[doc = "Field `P2_01MODE` writer - Port 2 pin 1 control."]
pub type P2_01modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, P2_01mode, crate::Safe>;
impl<'a, REG> P2_01modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P2.1 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(P2_01mode::PullUp)
    }
    #[doc = "Repeater. P2.1 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(P2_01mode::Repeater)
    }
    #[doc = "Disabled. P2.1 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P2_01mode::Disabled)
    }
    #[doc = "Pull-down. P2.1 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(P2_01mode::PullDown)
    }
}
#[doc = "Port 2 pin 2 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P2_02mode {
    #[doc = "0: Pull-up. P2.2 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P2.2 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P2.2 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P2.2 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<P2_02mode> for u8 {
    #[inline(always)]
    fn from(variant: P2_02mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P2_02mode {
    type Ux = u8;
}
impl crate::IsEnum for P2_02mode {}
#[doc = "Field `P2_02MODE` reader - Port 2 pin 2 control."]
pub type P2_02modeR = crate::FieldReader<P2_02mode>;
impl P2_02modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P2_02mode {
        match self.bits {
            0 => P2_02mode::PullUp,
            1 => P2_02mode::Repeater,
            2 => P2_02mode::Disabled,
            3 => P2_02mode::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P2.2 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P2_02mode::PullUp
    }
    #[doc = "Repeater. P2.2 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P2_02mode::Repeater
    }
    #[doc = "Disabled. P2.2 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P2_02mode::Disabled
    }
    #[doc = "Pull-down. P2.2 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P2_02mode::PullDown
    }
}
#[doc = "Field `P2_02MODE` writer - Port 2 pin 2 control."]
pub type P2_02modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, P2_02mode, crate::Safe>;
impl<'a, REG> P2_02modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P2.2 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(P2_02mode::PullUp)
    }
    #[doc = "Repeater. P2.2 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(P2_02mode::Repeater)
    }
    #[doc = "Disabled. P2.2 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P2_02mode::Disabled)
    }
    #[doc = "Pull-down. P2.2 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(P2_02mode::PullDown)
    }
}
#[doc = "Port 2 pin 3 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P2_03mode {
    #[doc = "0: Pull-up. P2.3 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P2.3 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P2.3 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P2.3 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<P2_03mode> for u8 {
    #[inline(always)]
    fn from(variant: P2_03mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P2_03mode {
    type Ux = u8;
}
impl crate::IsEnum for P2_03mode {}
#[doc = "Field `P2_03MODE` reader - Port 2 pin 3 control."]
pub type P2_03modeR = crate::FieldReader<P2_03mode>;
impl P2_03modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P2_03mode {
        match self.bits {
            0 => P2_03mode::PullUp,
            1 => P2_03mode::Repeater,
            2 => P2_03mode::Disabled,
            3 => P2_03mode::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P2.3 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P2_03mode::PullUp
    }
    #[doc = "Repeater. P2.3 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P2_03mode::Repeater
    }
    #[doc = "Disabled. P2.3 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P2_03mode::Disabled
    }
    #[doc = "Pull-down. P2.3 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P2_03mode::PullDown
    }
}
#[doc = "Field `P2_03MODE` writer - Port 2 pin 3 control."]
pub type P2_03modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, P2_03mode, crate::Safe>;
impl<'a, REG> P2_03modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P2.3 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(P2_03mode::PullUp)
    }
    #[doc = "Repeater. P2.3 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(P2_03mode::Repeater)
    }
    #[doc = "Disabled. P2.3 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P2_03mode::Disabled)
    }
    #[doc = "Pull-down. P2.3 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(P2_03mode::PullDown)
    }
}
#[doc = "Port 2 pin 4 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P2_04mode {
    #[doc = "0: Pull-up. P2.4 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P2.4 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P2.4 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P2.4 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<P2_04mode> for u8 {
    #[inline(always)]
    fn from(variant: P2_04mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P2_04mode {
    type Ux = u8;
}
impl crate::IsEnum for P2_04mode {}
#[doc = "Field `P2_04MODE` reader - Port 2 pin 4 control."]
pub type P2_04modeR = crate::FieldReader<P2_04mode>;
impl P2_04modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P2_04mode {
        match self.bits {
            0 => P2_04mode::PullUp,
            1 => P2_04mode::Repeater,
            2 => P2_04mode::Disabled,
            3 => P2_04mode::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P2.4 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P2_04mode::PullUp
    }
    #[doc = "Repeater. P2.4 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P2_04mode::Repeater
    }
    #[doc = "Disabled. P2.4 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P2_04mode::Disabled
    }
    #[doc = "Pull-down. P2.4 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P2_04mode::PullDown
    }
}
#[doc = "Field `P2_04MODE` writer - Port 2 pin 4 control."]
pub type P2_04modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, P2_04mode, crate::Safe>;
impl<'a, REG> P2_04modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P2.4 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(P2_04mode::PullUp)
    }
    #[doc = "Repeater. P2.4 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(P2_04mode::Repeater)
    }
    #[doc = "Disabled. P2.4 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P2_04mode::Disabled)
    }
    #[doc = "Pull-down. P2.4 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(P2_04mode::PullDown)
    }
}
#[doc = "Port 2 pin 5 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P2_05mode {
    #[doc = "0: Pull-up. P2.5 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P2.5 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P2.5 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P2.5 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<P2_05mode> for u8 {
    #[inline(always)]
    fn from(variant: P2_05mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P2_05mode {
    type Ux = u8;
}
impl crate::IsEnum for P2_05mode {}
#[doc = "Field `P2_05MODE` reader - Port 2 pin 5 control."]
pub type P2_05modeR = crate::FieldReader<P2_05mode>;
impl P2_05modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P2_05mode {
        match self.bits {
            0 => P2_05mode::PullUp,
            1 => P2_05mode::Repeater,
            2 => P2_05mode::Disabled,
            3 => P2_05mode::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P2.5 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P2_05mode::PullUp
    }
    #[doc = "Repeater. P2.5 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P2_05mode::Repeater
    }
    #[doc = "Disabled. P2.5 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P2_05mode::Disabled
    }
    #[doc = "Pull-down. P2.5 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P2_05mode::PullDown
    }
}
#[doc = "Field `P2_05MODE` writer - Port 2 pin 5 control."]
pub type P2_05modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, P2_05mode, crate::Safe>;
impl<'a, REG> P2_05modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P2.5 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(P2_05mode::PullUp)
    }
    #[doc = "Repeater. P2.5 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(P2_05mode::Repeater)
    }
    #[doc = "Disabled. P2.5 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P2_05mode::Disabled)
    }
    #[doc = "Pull-down. P2.5 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(P2_05mode::PullDown)
    }
}
#[doc = "Port 2 pin 6 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P2_06mode {
    #[doc = "0: Pull-up. P2.6 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P2.6 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P2.6 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P2.6 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<P2_06mode> for u8 {
    #[inline(always)]
    fn from(variant: P2_06mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P2_06mode {
    type Ux = u8;
}
impl crate::IsEnum for P2_06mode {}
#[doc = "Field `P2_06MODE` reader - Port 2 pin 6 control."]
pub type P2_06modeR = crate::FieldReader<P2_06mode>;
impl P2_06modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P2_06mode {
        match self.bits {
            0 => P2_06mode::PullUp,
            1 => P2_06mode::Repeater,
            2 => P2_06mode::Disabled,
            3 => P2_06mode::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P2.6 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P2_06mode::PullUp
    }
    #[doc = "Repeater. P2.6 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P2_06mode::Repeater
    }
    #[doc = "Disabled. P2.6 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P2_06mode::Disabled
    }
    #[doc = "Pull-down. P2.6 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P2_06mode::PullDown
    }
}
#[doc = "Field `P2_06MODE` writer - Port 2 pin 6 control."]
pub type P2_06modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, P2_06mode, crate::Safe>;
impl<'a, REG> P2_06modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P2.6 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(P2_06mode::PullUp)
    }
    #[doc = "Repeater. P2.6 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(P2_06mode::Repeater)
    }
    #[doc = "Disabled. P2.6 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P2_06mode::Disabled)
    }
    #[doc = "Pull-down. P2.6 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(P2_06mode::PullDown)
    }
}
#[doc = "Port 2 pin 7 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P2_07mode {
    #[doc = "0: Pull-up. P2.7 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P2.7 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P2.7 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P2.7 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<P2_07mode> for u8 {
    #[inline(always)]
    fn from(variant: P2_07mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P2_07mode {
    type Ux = u8;
}
impl crate::IsEnum for P2_07mode {}
#[doc = "Field `P2_07MODE` reader - Port 2 pin 7 control."]
pub type P2_07modeR = crate::FieldReader<P2_07mode>;
impl P2_07modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P2_07mode {
        match self.bits {
            0 => P2_07mode::PullUp,
            1 => P2_07mode::Repeater,
            2 => P2_07mode::Disabled,
            3 => P2_07mode::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P2.7 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P2_07mode::PullUp
    }
    #[doc = "Repeater. P2.7 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P2_07mode::Repeater
    }
    #[doc = "Disabled. P2.7 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P2_07mode::Disabled
    }
    #[doc = "Pull-down. P2.7 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P2_07mode::PullDown
    }
}
#[doc = "Field `P2_07MODE` writer - Port 2 pin 7 control."]
pub type P2_07modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, P2_07mode, crate::Safe>;
impl<'a, REG> P2_07modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P2.7 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(P2_07mode::PullUp)
    }
    #[doc = "Repeater. P2.7 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(P2_07mode::Repeater)
    }
    #[doc = "Disabled. P2.7 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P2_07mode::Disabled)
    }
    #[doc = "Pull-down. P2.7 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(P2_07mode::PullDown)
    }
}
#[doc = "Port 2 pin 8 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P2_08mode {
    #[doc = "0: Pull-up. P2.8 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P2.8 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P2.8 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P2.8 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<P2_08mode> for u8 {
    #[inline(always)]
    fn from(variant: P2_08mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P2_08mode {
    type Ux = u8;
}
impl crate::IsEnum for P2_08mode {}
#[doc = "Field `P2_08MODE` reader - Port 2 pin 8 control."]
pub type P2_08modeR = crate::FieldReader<P2_08mode>;
impl P2_08modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P2_08mode {
        match self.bits {
            0 => P2_08mode::PullUp,
            1 => P2_08mode::Repeater,
            2 => P2_08mode::Disabled,
            3 => P2_08mode::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P2.8 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P2_08mode::PullUp
    }
    #[doc = "Repeater. P2.8 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P2_08mode::Repeater
    }
    #[doc = "Disabled. P2.8 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P2_08mode::Disabled
    }
    #[doc = "Pull-down. P2.8 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P2_08mode::PullDown
    }
}
#[doc = "Field `P2_08MODE` writer - Port 2 pin 8 control."]
pub type P2_08modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, P2_08mode, crate::Safe>;
impl<'a, REG> P2_08modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P2.8 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(P2_08mode::PullUp)
    }
    #[doc = "Repeater. P2.8 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(P2_08mode::Repeater)
    }
    #[doc = "Disabled. P2.8 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P2_08mode::Disabled)
    }
    #[doc = "Pull-down. P2.8 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(P2_08mode::PullDown)
    }
}
#[doc = "Port 2 pin 9 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P2_09mode {
    #[doc = "0: Pull-up. P2.9 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P2.9 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P2.9 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P2.9 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<P2_09mode> for u8 {
    #[inline(always)]
    fn from(variant: P2_09mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P2_09mode {
    type Ux = u8;
}
impl crate::IsEnum for P2_09mode {}
#[doc = "Field `P2_09MODE` reader - Port 2 pin 9 control."]
pub type P2_09modeR = crate::FieldReader<P2_09mode>;
impl P2_09modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P2_09mode {
        match self.bits {
            0 => P2_09mode::PullUp,
            1 => P2_09mode::Repeater,
            2 => P2_09mode::Disabled,
            3 => P2_09mode::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P2.9 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P2_09mode::PullUp
    }
    #[doc = "Repeater. P2.9 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P2_09mode::Repeater
    }
    #[doc = "Disabled. P2.9 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P2_09mode::Disabled
    }
    #[doc = "Pull-down. P2.9 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P2_09mode::PullDown
    }
}
#[doc = "Field `P2_09MODE` writer - Port 2 pin 9 control."]
pub type P2_09modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, P2_09mode, crate::Safe>;
impl<'a, REG> P2_09modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P2.9 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(P2_09mode::PullUp)
    }
    #[doc = "Repeater. P2.9 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(P2_09mode::Repeater)
    }
    #[doc = "Disabled. P2.9 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P2_09mode::Disabled)
    }
    #[doc = "Pull-down. P2.9 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(P2_09mode::PullDown)
    }
}
#[doc = "Port 2 pin 10 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P2_10mode {
    #[doc = "0: Pull-up. P2.10 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P2.10 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P2.10 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P2.10 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<P2_10mode> for u8 {
    #[inline(always)]
    fn from(variant: P2_10mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P2_10mode {
    type Ux = u8;
}
impl crate::IsEnum for P2_10mode {}
#[doc = "Field `P2_10MODE` reader - Port 2 pin 10 control."]
pub type P2_10modeR = crate::FieldReader<P2_10mode>;
impl P2_10modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P2_10mode {
        match self.bits {
            0 => P2_10mode::PullUp,
            1 => P2_10mode::Repeater,
            2 => P2_10mode::Disabled,
            3 => P2_10mode::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P2.10 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P2_10mode::PullUp
    }
    #[doc = "Repeater. P2.10 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P2_10mode::Repeater
    }
    #[doc = "Disabled. P2.10 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P2_10mode::Disabled
    }
    #[doc = "Pull-down. P2.10 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P2_10mode::PullDown
    }
}
#[doc = "Field `P2_10MODE` writer - Port 2 pin 10 control."]
pub type P2_10modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, P2_10mode, crate::Safe>;
impl<'a, REG> P2_10modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P2.10 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(P2_10mode::PullUp)
    }
    #[doc = "Repeater. P2.10 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(P2_10mode::Repeater)
    }
    #[doc = "Disabled. P2.10 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P2_10mode::Disabled)
    }
    #[doc = "Pull-down. P2.10 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(P2_10mode::PullDown)
    }
}
#[doc = "Port 2 pin 11 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P2_11mode {
    #[doc = "0: Pull-up. P2.11 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P2.11 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P2.11 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P2.11 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<P2_11mode> for u8 {
    #[inline(always)]
    fn from(variant: P2_11mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P2_11mode {
    type Ux = u8;
}
impl crate::IsEnum for P2_11mode {}
#[doc = "Field `P2_11MODE` reader - Port 2 pin 11 control."]
pub type P2_11modeR = crate::FieldReader<P2_11mode>;
impl P2_11modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P2_11mode {
        match self.bits {
            0 => P2_11mode::PullUp,
            1 => P2_11mode::Repeater,
            2 => P2_11mode::Disabled,
            3 => P2_11mode::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P2.11 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P2_11mode::PullUp
    }
    #[doc = "Repeater. P2.11 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P2_11mode::Repeater
    }
    #[doc = "Disabled. P2.11 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P2_11mode::Disabled
    }
    #[doc = "Pull-down. P2.11 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P2_11mode::PullDown
    }
}
#[doc = "Field `P2_11MODE` writer - Port 2 pin 11 control."]
pub type P2_11modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, P2_11mode, crate::Safe>;
impl<'a, REG> P2_11modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P2.11 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(P2_11mode::PullUp)
    }
    #[doc = "Repeater. P2.11 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(P2_11mode::Repeater)
    }
    #[doc = "Disabled. P2.11 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P2_11mode::Disabled)
    }
    #[doc = "Pull-down. P2.11 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(P2_11mode::PullDown)
    }
}
#[doc = "Port 2 pin 12 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P2_12mode {
    #[doc = "0: Pull-up. P2.12 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P2.12 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P2.12 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P2.12 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<P2_12mode> for u8 {
    #[inline(always)]
    fn from(variant: P2_12mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P2_12mode {
    type Ux = u8;
}
impl crate::IsEnum for P2_12mode {}
#[doc = "Field `P2_12MODE` reader - Port 2 pin 12 control."]
pub type P2_12modeR = crate::FieldReader<P2_12mode>;
impl P2_12modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P2_12mode {
        match self.bits {
            0 => P2_12mode::PullUp,
            1 => P2_12mode::Repeater,
            2 => P2_12mode::Disabled,
            3 => P2_12mode::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P2.12 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P2_12mode::PullUp
    }
    #[doc = "Repeater. P2.12 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P2_12mode::Repeater
    }
    #[doc = "Disabled. P2.12 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P2_12mode::Disabled
    }
    #[doc = "Pull-down. P2.12 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P2_12mode::PullDown
    }
}
#[doc = "Field `P2_12MODE` writer - Port 2 pin 12 control."]
pub type P2_12modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, P2_12mode, crate::Safe>;
impl<'a, REG> P2_12modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P2.12 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(P2_12mode::PullUp)
    }
    #[doc = "Repeater. P2.12 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(P2_12mode::Repeater)
    }
    #[doc = "Disabled. P2.12 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P2_12mode::Disabled)
    }
    #[doc = "Pull-down. P2.12 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(P2_12mode::PullDown)
    }
}
#[doc = "Port 2 pin 13 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P2_13mode {
    #[doc = "0: Pull-up. P2.13 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P2.13 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P2.13 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P2.13 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<P2_13mode> for u8 {
    #[inline(always)]
    fn from(variant: P2_13mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P2_13mode {
    type Ux = u8;
}
impl crate::IsEnum for P2_13mode {}
#[doc = "Field `P2_13MODE` reader - Port 2 pin 13 control."]
pub type P2_13modeR = crate::FieldReader<P2_13mode>;
impl P2_13modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P2_13mode {
        match self.bits {
            0 => P2_13mode::PullUp,
            1 => P2_13mode::Repeater,
            2 => P2_13mode::Disabled,
            3 => P2_13mode::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P2.13 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P2_13mode::PullUp
    }
    #[doc = "Repeater. P2.13 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P2_13mode::Repeater
    }
    #[doc = "Disabled. P2.13 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P2_13mode::Disabled
    }
    #[doc = "Pull-down. P2.13 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P2_13mode::PullDown
    }
}
#[doc = "Field `P2_13MODE` writer - Port 2 pin 13 control."]
pub type P2_13modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, P2_13mode, crate::Safe>;
impl<'a, REG> P2_13modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P2.13 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(P2_13mode::PullUp)
    }
    #[doc = "Repeater. P2.13 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(P2_13mode::Repeater)
    }
    #[doc = "Disabled. P2.13 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P2_13mode::Disabled)
    }
    #[doc = "Pull-down. P2.13 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(P2_13mode::PullDown)
    }
}
impl R {
    #[doc = "Bits 0:1 - Port 2 pin 0 control."]
    #[inline(always)]
    pub fn p2_00mode(&self) -> P2_00modeR {
        P2_00modeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port 2 pin 1 control."]
    #[inline(always)]
    pub fn p2_01mode(&self) -> P2_01modeR {
        P2_01modeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port 2 pin 2 control."]
    #[inline(always)]
    pub fn p2_02mode(&self) -> P2_02modeR {
        P2_02modeR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port 2 pin 3 control."]
    #[inline(always)]
    pub fn p2_03mode(&self) -> P2_03modeR {
        P2_03modeR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port 2 pin 4 control."]
    #[inline(always)]
    pub fn p2_04mode(&self) -> P2_04modeR {
        P2_04modeR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port 2 pin 5 control."]
    #[inline(always)]
    pub fn p2_05mode(&self) -> P2_05modeR {
        P2_05modeR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port 2 pin 6 control."]
    #[inline(always)]
    pub fn p2_06mode(&self) -> P2_06modeR {
        P2_06modeR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port 2 pin 7 control."]
    #[inline(always)]
    pub fn p2_07mode(&self) -> P2_07modeR {
        P2_07modeR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port 2 pin 8 control."]
    #[inline(always)]
    pub fn p2_08mode(&self) -> P2_08modeR {
        P2_08modeR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Port 2 pin 9 control."]
    #[inline(always)]
    pub fn p2_09mode(&self) -> P2_09modeR {
        P2_09modeR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Port 2 pin 10 control."]
    #[inline(always)]
    pub fn p2_10mode(&self) -> P2_10modeR {
        P2_10modeR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Port 2 pin 11 control."]
    #[inline(always)]
    pub fn p2_11mode(&self) -> P2_11modeR {
        P2_11modeR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Port 2 pin 12 control."]
    #[inline(always)]
    pub fn p2_12mode(&self) -> P2_12modeR {
        P2_12modeR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Port 2 pin 13 control."]
    #[inline(always)]
    pub fn p2_13mode(&self) -> P2_13modeR {
        P2_13modeR::new(((self.bits >> 26) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port 2 pin 0 control."]
    #[inline(always)]
    #[must_use]
    pub fn p2_00mode(&mut self) -> P2_00modeW<Pinmode4Spec> {
        P2_00modeW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Port 2 pin 1 control."]
    #[inline(always)]
    #[must_use]
    pub fn p2_01mode(&mut self) -> P2_01modeW<Pinmode4Spec> {
        P2_01modeW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Port 2 pin 2 control."]
    #[inline(always)]
    #[must_use]
    pub fn p2_02mode(&mut self) -> P2_02modeW<Pinmode4Spec> {
        P2_02modeW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Port 2 pin 3 control."]
    #[inline(always)]
    #[must_use]
    pub fn p2_03mode(&mut self) -> P2_03modeW<Pinmode4Spec> {
        P2_03modeW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Port 2 pin 4 control."]
    #[inline(always)]
    #[must_use]
    pub fn p2_04mode(&mut self) -> P2_04modeW<Pinmode4Spec> {
        P2_04modeW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Port 2 pin 5 control."]
    #[inline(always)]
    #[must_use]
    pub fn p2_05mode(&mut self) -> P2_05modeW<Pinmode4Spec> {
        P2_05modeW::new(self, 10)
    }
    #[doc = "Bits 12:13 - Port 2 pin 6 control."]
    #[inline(always)]
    #[must_use]
    pub fn p2_06mode(&mut self) -> P2_06modeW<Pinmode4Spec> {
        P2_06modeW::new(self, 12)
    }
    #[doc = "Bits 14:15 - Port 2 pin 7 control."]
    #[inline(always)]
    #[must_use]
    pub fn p2_07mode(&mut self) -> P2_07modeW<Pinmode4Spec> {
        P2_07modeW::new(self, 14)
    }
    #[doc = "Bits 16:17 - Port 2 pin 8 control."]
    #[inline(always)]
    #[must_use]
    pub fn p2_08mode(&mut self) -> P2_08modeW<Pinmode4Spec> {
        P2_08modeW::new(self, 16)
    }
    #[doc = "Bits 18:19 - Port 2 pin 9 control."]
    #[inline(always)]
    #[must_use]
    pub fn p2_09mode(&mut self) -> P2_09modeW<Pinmode4Spec> {
        P2_09modeW::new(self, 18)
    }
    #[doc = "Bits 20:21 - Port 2 pin 10 control."]
    #[inline(always)]
    #[must_use]
    pub fn p2_10mode(&mut self) -> P2_10modeW<Pinmode4Spec> {
        P2_10modeW::new(self, 20)
    }
    #[doc = "Bits 22:23 - Port 2 pin 11 control."]
    #[inline(always)]
    #[must_use]
    pub fn p2_11mode(&mut self) -> P2_11modeW<Pinmode4Spec> {
        P2_11modeW::new(self, 22)
    }
    #[doc = "Bits 24:25 - Port 2 pin 12 control."]
    #[inline(always)]
    #[must_use]
    pub fn p2_12mode(&mut self) -> P2_12modeW<Pinmode4Spec> {
        P2_12modeW::new(self, 24)
    }
    #[doc = "Bits 26:27 - Port 2 pin 13 control."]
    #[inline(always)]
    #[must_use]
    pub fn p2_13mode(&mut self) -> P2_13modeW<Pinmode4Spec> {
        P2_13modeW::new(self, 26)
    }
}
#[doc = "Pin mode select register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`pinmode4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pinmode4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pinmode4Spec;
impl crate::RegisterSpec for Pinmode4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pinmode4::R`](R) reader structure"]
impl crate::Readable for Pinmode4Spec {}
#[doc = "`write(|w| ..)` method takes [`pinmode4::W`](W) writer structure"]
impl crate::Writable for Pinmode4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PINMODE4 to value 0"]
impl crate::Resettable for Pinmode4Spec {
    const RESET_VALUE: u32 = 0;
}
