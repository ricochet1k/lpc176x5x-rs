#[doc = "Register `PINMODE2` reader"]
pub type R = crate::R<Pinmode2Spec>;
#[doc = "Register `PINMODE2` writer"]
pub type W = crate::W<Pinmode2Spec>;
#[doc = "Port 1 pin 0 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_00mode {
    #[doc = "0: Pull-up. P1.0 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P1.0 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P1.0 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P1.0 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<P1_00mode> for u8 {
    #[inline(always)]
    fn from(variant: P1_00mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_00mode {
    type Ux = u8;
}
impl crate::IsEnum for P1_00mode {}
#[doc = "Field `P1_00MODE` reader - Port 1 pin 0 control."]
pub type P1_00modeR = crate::FieldReader<P1_00mode>;
impl P1_00modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_00mode {
        match self.bits {
            0 => P1_00mode::PullUp,
            1 => P1_00mode::Repeater,
            2 => P1_00mode::Disabled,
            3 => P1_00mode::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P1.0 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_00mode::PullUp
    }
    #[doc = "Repeater. P1.0 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_00mode::Repeater
    }
    #[doc = "Disabled. P1.0 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_00mode::Disabled
    }
    #[doc = "Pull-down. P1.0 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_00mode::PullDown
    }
}
#[doc = "Field `P1_00MODE` writer - Port 1 pin 0 control."]
pub type P1_00modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, P1_00mode, crate::Safe>;
impl<'a, REG> P1_00modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P1.0 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(P1_00mode::PullUp)
    }
    #[doc = "Repeater. P1.0 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(P1_00mode::Repeater)
    }
    #[doc = "Disabled. P1.0 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P1_00mode::Disabled)
    }
    #[doc = "Pull-down. P1.0 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(P1_00mode::PullDown)
    }
}
#[doc = "Port 1 pin 1 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_01mode {
    #[doc = "0: Pull-up. P1.1 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P1.1 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P1.1 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P1.1 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<P1_01mode> for u8 {
    #[inline(always)]
    fn from(variant: P1_01mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_01mode {
    type Ux = u8;
}
impl crate::IsEnum for P1_01mode {}
#[doc = "Field `P1_01MODE` reader - Port 1 pin 1 control."]
pub type P1_01modeR = crate::FieldReader<P1_01mode>;
impl P1_01modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_01mode {
        match self.bits {
            0 => P1_01mode::PullUp,
            1 => P1_01mode::Repeater,
            2 => P1_01mode::Disabled,
            3 => P1_01mode::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P1.1 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_01mode::PullUp
    }
    #[doc = "Repeater. P1.1 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_01mode::Repeater
    }
    #[doc = "Disabled. P1.1 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_01mode::Disabled
    }
    #[doc = "Pull-down. P1.1 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_01mode::PullDown
    }
}
#[doc = "Field `P1_01MODE` writer - Port 1 pin 1 control."]
pub type P1_01modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, P1_01mode, crate::Safe>;
impl<'a, REG> P1_01modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P1.1 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(P1_01mode::PullUp)
    }
    #[doc = "Repeater. P1.1 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(P1_01mode::Repeater)
    }
    #[doc = "Disabled. P1.1 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P1_01mode::Disabled)
    }
    #[doc = "Pull-down. P1.1 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(P1_01mode::PullDown)
    }
}
#[doc = "Port 1 pin 4 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_04mode {
    #[doc = "0: Pull-up. P1.4 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P1.4 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P1.4 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P1.4 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<P1_04mode> for u8 {
    #[inline(always)]
    fn from(variant: P1_04mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_04mode {
    type Ux = u8;
}
impl crate::IsEnum for P1_04mode {}
#[doc = "Field `P1_04MODE` reader - Port 1 pin 4 control."]
pub type P1_04modeR = crate::FieldReader<P1_04mode>;
impl P1_04modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_04mode {
        match self.bits {
            0 => P1_04mode::PullUp,
            1 => P1_04mode::Repeater,
            2 => P1_04mode::Disabled,
            3 => P1_04mode::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P1.4 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_04mode::PullUp
    }
    #[doc = "Repeater. P1.4 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_04mode::Repeater
    }
    #[doc = "Disabled. P1.4 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_04mode::Disabled
    }
    #[doc = "Pull-down. P1.4 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_04mode::PullDown
    }
}
#[doc = "Field `P1_04MODE` writer - Port 1 pin 4 control."]
pub type P1_04modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, P1_04mode, crate::Safe>;
impl<'a, REG> P1_04modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P1.4 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(P1_04mode::PullUp)
    }
    #[doc = "Repeater. P1.4 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(P1_04mode::Repeater)
    }
    #[doc = "Disabled. P1.4 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P1_04mode::Disabled)
    }
    #[doc = "Pull-down. P1.4 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(P1_04mode::PullDown)
    }
}
#[doc = "Port 1 pin 8 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_08mode {
    #[doc = "0: Pull-up. P1.8 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P1.8 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P1.8 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P1.8 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<P1_08mode> for u8 {
    #[inline(always)]
    fn from(variant: P1_08mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_08mode {
    type Ux = u8;
}
impl crate::IsEnum for P1_08mode {}
#[doc = "Field `P1_08MODE` reader - Port 1 pin 8 control."]
pub type P1_08modeR = crate::FieldReader<P1_08mode>;
impl P1_08modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_08mode {
        match self.bits {
            0 => P1_08mode::PullUp,
            1 => P1_08mode::Repeater,
            2 => P1_08mode::Disabled,
            3 => P1_08mode::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P1.8 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_08mode::PullUp
    }
    #[doc = "Repeater. P1.8 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_08mode::Repeater
    }
    #[doc = "Disabled. P1.8 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_08mode::Disabled
    }
    #[doc = "Pull-down. P1.8 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_08mode::PullDown
    }
}
#[doc = "Field `P1_08MODE` writer - Port 1 pin 8 control."]
pub type P1_08modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, P1_08mode, crate::Safe>;
impl<'a, REG> P1_08modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P1.8 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(P1_08mode::PullUp)
    }
    #[doc = "Repeater. P1.8 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(P1_08mode::Repeater)
    }
    #[doc = "Disabled. P1.8 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P1_08mode::Disabled)
    }
    #[doc = "Pull-down. P1.8 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(P1_08mode::PullDown)
    }
}
#[doc = "Port 1 pin 9 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_09mode {
    #[doc = "0: Pull-up. P1.9 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P1.9 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P1.9 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P1.9 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<P1_09mode> for u8 {
    #[inline(always)]
    fn from(variant: P1_09mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_09mode {
    type Ux = u8;
}
impl crate::IsEnum for P1_09mode {}
#[doc = "Field `P1_09MODE` reader - Port 1 pin 9 control."]
pub type P1_09modeR = crate::FieldReader<P1_09mode>;
impl P1_09modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_09mode {
        match self.bits {
            0 => P1_09mode::PullUp,
            1 => P1_09mode::Repeater,
            2 => P1_09mode::Disabled,
            3 => P1_09mode::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P1.9 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_09mode::PullUp
    }
    #[doc = "Repeater. P1.9 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_09mode::Repeater
    }
    #[doc = "Disabled. P1.9 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_09mode::Disabled
    }
    #[doc = "Pull-down. P1.9 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_09mode::PullDown
    }
}
#[doc = "Field `P1_09MODE` writer - Port 1 pin 9 control."]
pub type P1_09modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, P1_09mode, crate::Safe>;
impl<'a, REG> P1_09modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P1.9 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(P1_09mode::PullUp)
    }
    #[doc = "Repeater. P1.9 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(P1_09mode::Repeater)
    }
    #[doc = "Disabled. P1.9 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P1_09mode::Disabled)
    }
    #[doc = "Pull-down. P1.9 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(P1_09mode::PullDown)
    }
}
#[doc = "Port 1 pin 10 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_10mode {
    #[doc = "0: Pull-up. P1.10 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P1.10 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P1.10 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P1.10 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<P1_10mode> for u8 {
    #[inline(always)]
    fn from(variant: P1_10mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_10mode {
    type Ux = u8;
}
impl crate::IsEnum for P1_10mode {}
#[doc = "Field `P1_10MODE` reader - Port 1 pin 10 control."]
pub type P1_10modeR = crate::FieldReader<P1_10mode>;
impl P1_10modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_10mode {
        match self.bits {
            0 => P1_10mode::PullUp,
            1 => P1_10mode::Repeater,
            2 => P1_10mode::Disabled,
            3 => P1_10mode::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P1.10 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_10mode::PullUp
    }
    #[doc = "Repeater. P1.10 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_10mode::Repeater
    }
    #[doc = "Disabled. P1.10 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_10mode::Disabled
    }
    #[doc = "Pull-down. P1.10 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_10mode::PullDown
    }
}
#[doc = "Field `P1_10MODE` writer - Port 1 pin 10 control."]
pub type P1_10modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, P1_10mode, crate::Safe>;
impl<'a, REG> P1_10modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P1.10 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(P1_10mode::PullUp)
    }
    #[doc = "Repeater. P1.10 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(P1_10mode::Repeater)
    }
    #[doc = "Disabled. P1.10 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P1_10mode::Disabled)
    }
    #[doc = "Pull-down. P1.10 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(P1_10mode::PullDown)
    }
}
#[doc = "Port 1 pin 14 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_14mode {
    #[doc = "0: Pull-up. P1.14 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P1.14 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P1.14 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P1.14 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<P1_14mode> for u8 {
    #[inline(always)]
    fn from(variant: P1_14mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_14mode {
    type Ux = u8;
}
impl crate::IsEnum for P1_14mode {}
#[doc = "Field `P1_14MODE` reader - Port 1 pin 14 control."]
pub type P1_14modeR = crate::FieldReader<P1_14mode>;
impl P1_14modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_14mode {
        match self.bits {
            0 => P1_14mode::PullUp,
            1 => P1_14mode::Repeater,
            2 => P1_14mode::Disabled,
            3 => P1_14mode::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P1.14 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_14mode::PullUp
    }
    #[doc = "Repeater. P1.14 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_14mode::Repeater
    }
    #[doc = "Disabled. P1.14 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_14mode::Disabled
    }
    #[doc = "Pull-down. P1.14 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_14mode::PullDown
    }
}
#[doc = "Field `P1_14MODE` writer - Port 1 pin 14 control."]
pub type P1_14modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, P1_14mode, crate::Safe>;
impl<'a, REG> P1_14modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P1.14 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(P1_14mode::PullUp)
    }
    #[doc = "Repeater. P1.14 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(P1_14mode::Repeater)
    }
    #[doc = "Disabled. P1.14 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P1_14mode::Disabled)
    }
    #[doc = "Pull-down. P1.14 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(P1_14mode::PullDown)
    }
}
#[doc = "Port 1 pin 15 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_15mode {
    #[doc = "0: Pull-up. P1.15 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P1.15 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P1.15 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P1.15 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<P1_15mode> for u8 {
    #[inline(always)]
    fn from(variant: P1_15mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_15mode {
    type Ux = u8;
}
impl crate::IsEnum for P1_15mode {}
#[doc = "Field `P1_15MODE` reader - Port 1 pin 15 control."]
pub type P1_15modeR = crate::FieldReader<P1_15mode>;
impl P1_15modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_15mode {
        match self.bits {
            0 => P1_15mode::PullUp,
            1 => P1_15mode::Repeater,
            2 => P1_15mode::Disabled,
            3 => P1_15mode::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P1.15 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_15mode::PullUp
    }
    #[doc = "Repeater. P1.15 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_15mode::Repeater
    }
    #[doc = "Disabled. P1.15 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_15mode::Disabled
    }
    #[doc = "Pull-down. P1.15 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_15mode::PullDown
    }
}
#[doc = "Field `P1_15MODE` writer - Port 1 pin 15 control."]
pub type P1_15modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, P1_15mode, crate::Safe>;
impl<'a, REG> P1_15modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P1.15 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(P1_15mode::PullUp)
    }
    #[doc = "Repeater. P1.15 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(P1_15mode::Repeater)
    }
    #[doc = "Disabled. P1.15 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(P1_15mode::Disabled)
    }
    #[doc = "Pull-down. P1.15 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(P1_15mode::PullDown)
    }
}
impl R {
    #[doc = "Bits 0:1 - Port 1 pin 0 control."]
    #[inline(always)]
    pub fn p1_00mode(&self) -> P1_00modeR {
        P1_00modeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port 1 pin 1 control."]
    #[inline(always)]
    pub fn p1_01mode(&self) -> P1_01modeR {
        P1_01modeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port 1 pin 4 control."]
    #[inline(always)]
    pub fn p1_04mode(&self) -> P1_04modeR {
        P1_04modeR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port 1 pin 8 control."]
    #[inline(always)]
    pub fn p1_08mode(&self) -> P1_08modeR {
        P1_08modeR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Port 1 pin 9 control."]
    #[inline(always)]
    pub fn p1_09mode(&self) -> P1_09modeR {
        P1_09modeR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Port 1 pin 10 control."]
    #[inline(always)]
    pub fn p1_10mode(&self) -> P1_10modeR {
        P1_10modeR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Port 1 pin 14 control."]
    #[inline(always)]
    pub fn p1_14mode(&self) -> P1_14modeR {
        P1_14modeR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Port 1 pin 15 control."]
    #[inline(always)]
    pub fn p1_15mode(&self) -> P1_15modeR {
        P1_15modeR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port 1 pin 0 control."]
    #[inline(always)]
    #[must_use]
    pub fn p1_00mode(&mut self) -> P1_00modeW<Pinmode2Spec> {
        P1_00modeW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Port 1 pin 1 control."]
    #[inline(always)]
    #[must_use]
    pub fn p1_01mode(&mut self) -> P1_01modeW<Pinmode2Spec> {
        P1_01modeW::new(self, 2)
    }
    #[doc = "Bits 8:9 - Port 1 pin 4 control."]
    #[inline(always)]
    #[must_use]
    pub fn p1_04mode(&mut self) -> P1_04modeW<Pinmode2Spec> {
        P1_04modeW::new(self, 8)
    }
    #[doc = "Bits 16:17 - Port 1 pin 8 control."]
    #[inline(always)]
    #[must_use]
    pub fn p1_08mode(&mut self) -> P1_08modeW<Pinmode2Spec> {
        P1_08modeW::new(self, 16)
    }
    #[doc = "Bits 18:19 - Port 1 pin 9 control."]
    #[inline(always)]
    #[must_use]
    pub fn p1_09mode(&mut self) -> P1_09modeW<Pinmode2Spec> {
        P1_09modeW::new(self, 18)
    }
    #[doc = "Bits 20:21 - Port 1 pin 10 control."]
    #[inline(always)]
    #[must_use]
    pub fn p1_10mode(&mut self) -> P1_10modeW<Pinmode2Spec> {
        P1_10modeW::new(self, 20)
    }
    #[doc = "Bits 28:29 - Port 1 pin 14 control."]
    #[inline(always)]
    #[must_use]
    pub fn p1_14mode(&mut self) -> P1_14modeW<Pinmode2Spec> {
        P1_14modeW::new(self, 28)
    }
    #[doc = "Bits 30:31 - Port 1 pin 15 control."]
    #[inline(always)]
    #[must_use]
    pub fn p1_15mode(&mut self) -> P1_15modeW<Pinmode2Spec> {
        P1_15modeW::new(self, 30)
    }
}
#[doc = "Pin mode select register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`pinmode2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pinmode2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pinmode2Spec;
impl crate::RegisterSpec for Pinmode2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pinmode2::R`](R) reader structure"]
impl crate::Readable for Pinmode2Spec {}
#[doc = "`write(|w| ..)` method takes [`pinmode2::W`](W) writer structure"]
impl crate::Writable for Pinmode2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PINMODE2 to value 0"]
impl crate::Resettable for Pinmode2Spec {
    const RESET_VALUE: u32 = 0;
}
