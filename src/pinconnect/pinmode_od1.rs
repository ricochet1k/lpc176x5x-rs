#[doc = "Register `PINMODE_OD1` reader"]
pub type R = crate::R<PinmodeOd1Spec>;
#[doc = "Register `PINMODE_OD1` writer"]
pub type W = crate::W<PinmodeOd1Spec>;
#[doc = "Port 1 pin 0 open drain mode control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P1_00od {
    #[doc = "0: Normal. P1.0 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P1.0 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<P1_00od> for bool {
    #[inline(always)]
    fn from(variant: P1_00od) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P1_00OD` reader - Port 1 pin 0 open drain mode control."]
pub type P1_00odR = crate::BitReader<P1_00od>;
impl P1_00odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_00od {
        match self.bits {
            false => P1_00od::Normal,
            true => P1_00od::OpenDrain,
        }
    }
    #[doc = "Normal. P1.0 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P1_00od::Normal
    }
    #[doc = "Open-drain. P1.0 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_00od::OpenDrain
    }
}
#[doc = "Field `P1_00OD` writer - Port 1 pin 0 open drain mode control."]
pub type P1_00odW<'a, REG> = crate::BitWriter<'a, REG, P1_00od>;
impl<'a, REG> P1_00odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P1.0 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(P1_00od::Normal)
    }
    #[doc = "Open-drain. P1.0 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(P1_00od::OpenDrain)
    }
}
#[doc = "Port 1 pin 1 open drain mode control, see P1.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P1_01od {
    #[doc = "0: Normal. P1.1 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P1.1 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<P1_01od> for bool {
    #[inline(always)]
    fn from(variant: P1_01od) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P1_01OD` reader - Port 1 pin 1 open drain mode control, see P1.00OD"]
pub type P1_01odR = crate::BitReader<P1_01od>;
impl P1_01odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_01od {
        match self.bits {
            false => P1_01od::Normal,
            true => P1_01od::OpenDrain,
        }
    }
    #[doc = "Normal. P1.1 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P1_01od::Normal
    }
    #[doc = "Open-drain. P1.1 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_01od::OpenDrain
    }
}
#[doc = "Field `P1_01OD` writer - Port 1 pin 1 open drain mode control, see P1.00OD"]
pub type P1_01odW<'a, REG> = crate::BitWriter<'a, REG, P1_01od>;
impl<'a, REG> P1_01odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P1.1 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(P1_01od::Normal)
    }
    #[doc = "Open-drain. P1.1 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(P1_01od::OpenDrain)
    }
}
#[doc = "Port 1 pin 4 open drain mode control, see P1.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P1_04od {
    #[doc = "0: Normal. P1.4 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P1.4 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<P1_04od> for bool {
    #[inline(always)]
    fn from(variant: P1_04od) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P1_04OD` reader - Port 1 pin 4 open drain mode control, see P1.00OD"]
pub type P1_04odR = crate::BitReader<P1_04od>;
impl P1_04odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_04od {
        match self.bits {
            false => P1_04od::Normal,
            true => P1_04od::OpenDrain,
        }
    }
    #[doc = "Normal. P1.4 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P1_04od::Normal
    }
    #[doc = "Open-drain. P1.4 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_04od::OpenDrain
    }
}
#[doc = "Field `P1_04OD` writer - Port 1 pin 4 open drain mode control, see P1.00OD"]
pub type P1_04odW<'a, REG> = crate::BitWriter<'a, REG, P1_04od>;
impl<'a, REG> P1_04odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P1.4 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(P1_04od::Normal)
    }
    #[doc = "Open-drain. P1.4 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(P1_04od::OpenDrain)
    }
}
#[doc = "Port 1 pin 8 open drain mode control, see P1.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P1_08od {
    #[doc = "0: Normal. P1.8 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P1.8 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<P1_08od> for bool {
    #[inline(always)]
    fn from(variant: P1_08od) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P1_08OD` reader - Port 1 pin 8 open drain mode control, see P1.00OD"]
pub type P1_08odR = crate::BitReader<P1_08od>;
impl P1_08odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_08od {
        match self.bits {
            false => P1_08od::Normal,
            true => P1_08od::OpenDrain,
        }
    }
    #[doc = "Normal. P1.8 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P1_08od::Normal
    }
    #[doc = "Open-drain. P1.8 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_08od::OpenDrain
    }
}
#[doc = "Field `P1_08OD` writer - Port 1 pin 8 open drain mode control, see P1.00OD"]
pub type P1_08odW<'a, REG> = crate::BitWriter<'a, REG, P1_08od>;
impl<'a, REG> P1_08odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P1.8 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(P1_08od::Normal)
    }
    #[doc = "Open-drain. P1.8 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(P1_08od::OpenDrain)
    }
}
#[doc = "Port 1 pin 9 open drain mode control, see P1.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P1_09od {
    #[doc = "0: Normal. P1.9 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P1.9 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<P1_09od> for bool {
    #[inline(always)]
    fn from(variant: P1_09od) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P1_09OD` reader - Port 1 pin 9 open drain mode control, see P1.00OD"]
pub type P1_09odR = crate::BitReader<P1_09od>;
impl P1_09odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_09od {
        match self.bits {
            false => P1_09od::Normal,
            true => P1_09od::OpenDrain,
        }
    }
    #[doc = "Normal. P1.9 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P1_09od::Normal
    }
    #[doc = "Open-drain. P1.9 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_09od::OpenDrain
    }
}
#[doc = "Field `P1_09OD` writer - Port 1 pin 9 open drain mode control, see P1.00OD"]
pub type P1_09odW<'a, REG> = crate::BitWriter<'a, REG, P1_09od>;
impl<'a, REG> P1_09odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P1.9 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(P1_09od::Normal)
    }
    #[doc = "Open-drain. P1.9 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(P1_09od::OpenDrain)
    }
}
#[doc = "Port 1 pin 10 open drain mode control, see P1.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P1_10od {
    #[doc = "0: Normal. P1.10 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P1.10 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<P1_10od> for bool {
    #[inline(always)]
    fn from(variant: P1_10od) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P1_10OD` reader - Port 1 pin 10 open drain mode control, see P1.00OD"]
pub type P1_10odR = crate::BitReader<P1_10od>;
impl P1_10odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_10od {
        match self.bits {
            false => P1_10od::Normal,
            true => P1_10od::OpenDrain,
        }
    }
    #[doc = "Normal. P1.10 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P1_10od::Normal
    }
    #[doc = "Open-drain. P1.10 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_10od::OpenDrain
    }
}
#[doc = "Field `P1_10OD` writer - Port 1 pin 10 open drain mode control, see P1.00OD"]
pub type P1_10odW<'a, REG> = crate::BitWriter<'a, REG, P1_10od>;
impl<'a, REG> P1_10odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P1.10 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(P1_10od::Normal)
    }
    #[doc = "Open-drain. P1.10 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(P1_10od::OpenDrain)
    }
}
#[doc = "Port 1 pin 14 open drain mode control, see P1.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P1_14od {
    #[doc = "0: Normal. P1.14 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P1.14 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<P1_14od> for bool {
    #[inline(always)]
    fn from(variant: P1_14od) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P1_14OD` reader - Port 1 pin 14 open drain mode control, see P1.00OD"]
pub type P1_14odR = crate::BitReader<P1_14od>;
impl P1_14odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_14od {
        match self.bits {
            false => P1_14od::Normal,
            true => P1_14od::OpenDrain,
        }
    }
    #[doc = "Normal. P1.14 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P1_14od::Normal
    }
    #[doc = "Open-drain. P1.14 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_14od::OpenDrain
    }
}
#[doc = "Field `P1_14OD` writer - Port 1 pin 14 open drain mode control, see P1.00OD"]
pub type P1_14odW<'a, REG> = crate::BitWriter<'a, REG, P1_14od>;
impl<'a, REG> P1_14odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P1.14 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(P1_14od::Normal)
    }
    #[doc = "Open-drain. P1.14 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(P1_14od::OpenDrain)
    }
}
#[doc = "Port 1 pin 15 open drain mode control, see P1.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P1_15od {
    #[doc = "0: Normal. P1.15 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P1.15 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<P1_15od> for bool {
    #[inline(always)]
    fn from(variant: P1_15od) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P1_15OD` reader - Port 1 pin 15 open drain mode control, see P1.00OD"]
pub type P1_15odR = crate::BitReader<P1_15od>;
impl P1_15odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_15od {
        match self.bits {
            false => P1_15od::Normal,
            true => P1_15od::OpenDrain,
        }
    }
    #[doc = "Normal. P1.15 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P1_15od::Normal
    }
    #[doc = "Open-drain. P1.15 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_15od::OpenDrain
    }
}
#[doc = "Field `P1_15OD` writer - Port 1 pin 15 open drain mode control, see P1.00OD"]
pub type P1_15odW<'a, REG> = crate::BitWriter<'a, REG, P1_15od>;
impl<'a, REG> P1_15odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P1.15 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(P1_15od::Normal)
    }
    #[doc = "Open-drain. P1.15 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(P1_15od::OpenDrain)
    }
}
#[doc = "Port 1 pin 16 open drain mode control, see P1.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P1_16od {
    #[doc = "0: Normal. P1.16 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P1.16 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<P1_16od> for bool {
    #[inline(always)]
    fn from(variant: P1_16od) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P1_16OD` reader - Port 1 pin 16 open drain mode control, see P1.00OD"]
pub type P1_16odR = crate::BitReader<P1_16od>;
impl P1_16odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_16od {
        match self.bits {
            false => P1_16od::Normal,
            true => P1_16od::OpenDrain,
        }
    }
    #[doc = "Normal. P1.16 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P1_16od::Normal
    }
    #[doc = "Open-drain. P1.16 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_16od::OpenDrain
    }
}
#[doc = "Field `P1_16OD` writer - Port 1 pin 16 open drain mode control, see P1.00OD"]
pub type P1_16odW<'a, REG> = crate::BitWriter<'a, REG, P1_16od>;
impl<'a, REG> P1_16odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P1.16 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(P1_16od::Normal)
    }
    #[doc = "Open-drain. P1.16 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(P1_16od::OpenDrain)
    }
}
#[doc = "Port 1 pin 17 open drain mode control, see P1.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P1_17od {
    #[doc = "0: Normal. P1.17 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P1.17 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<P1_17od> for bool {
    #[inline(always)]
    fn from(variant: P1_17od) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P1_17OD` reader - Port 1 pin 17 open drain mode control, see P1.00OD"]
pub type P1_17odR = crate::BitReader<P1_17od>;
impl P1_17odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_17od {
        match self.bits {
            false => P1_17od::Normal,
            true => P1_17od::OpenDrain,
        }
    }
    #[doc = "Normal. P1.17 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P1_17od::Normal
    }
    #[doc = "Open-drain. P1.17 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_17od::OpenDrain
    }
}
#[doc = "Field `P1_17OD` writer - Port 1 pin 17 open drain mode control, see P1.00OD"]
pub type P1_17odW<'a, REG> = crate::BitWriter<'a, REG, P1_17od>;
impl<'a, REG> P1_17odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P1.17 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(P1_17od::Normal)
    }
    #[doc = "Open-drain. P1.17 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(P1_17od::OpenDrain)
    }
}
#[doc = "Port 1 pin 18 open drain mode control, see P1.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P1_18od {
    #[doc = "0: Normal. P1.18 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P1.18 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<P1_18od> for bool {
    #[inline(always)]
    fn from(variant: P1_18od) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P1_18OD` reader - Port 1 pin 18 open drain mode control, see P1.00OD"]
pub type P1_18odR = crate::BitReader<P1_18od>;
impl P1_18odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_18od {
        match self.bits {
            false => P1_18od::Normal,
            true => P1_18od::OpenDrain,
        }
    }
    #[doc = "Normal. P1.18 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P1_18od::Normal
    }
    #[doc = "Open-drain. P1.18 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_18od::OpenDrain
    }
}
#[doc = "Field `P1_18OD` writer - Port 1 pin 18 open drain mode control, see P1.00OD"]
pub type P1_18odW<'a, REG> = crate::BitWriter<'a, REG, P1_18od>;
impl<'a, REG> P1_18odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P1.18 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(P1_18od::Normal)
    }
    #[doc = "Open-drain. P1.18 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(P1_18od::OpenDrain)
    }
}
#[doc = "Port 1 pin 19 open drain mode control, see P1.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P1_19od {
    #[doc = "0: Normal. P1.19 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P1.19 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<P1_19od> for bool {
    #[inline(always)]
    fn from(variant: P1_19od) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P1_19OD` reader - Port 1 pin 19 open drain mode control, see P1.00OD"]
pub type P1_19odR = crate::BitReader<P1_19od>;
impl P1_19odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_19od {
        match self.bits {
            false => P1_19od::Normal,
            true => P1_19od::OpenDrain,
        }
    }
    #[doc = "Normal. P1.19 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P1_19od::Normal
    }
    #[doc = "Open-drain. P1.19 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_19od::OpenDrain
    }
}
#[doc = "Field `P1_19OD` writer - Port 1 pin 19 open drain mode control, see P1.00OD"]
pub type P1_19odW<'a, REG> = crate::BitWriter<'a, REG, P1_19od>;
impl<'a, REG> P1_19odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P1.19 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(P1_19od::Normal)
    }
    #[doc = "Open-drain. P1.19 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(P1_19od::OpenDrain)
    }
}
#[doc = "Port 1 pin 20open drain mode control, see P1.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P1_20od {
    #[doc = "0: Normal. P1.20 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P1.20 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<P1_20od> for bool {
    #[inline(always)]
    fn from(variant: P1_20od) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P1_20OD` reader - Port 1 pin 20open drain mode control, see P1.00OD"]
pub type P1_20odR = crate::BitReader<P1_20od>;
impl P1_20odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_20od {
        match self.bits {
            false => P1_20od::Normal,
            true => P1_20od::OpenDrain,
        }
    }
    #[doc = "Normal. P1.20 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P1_20od::Normal
    }
    #[doc = "Open-drain. P1.20 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_20od::OpenDrain
    }
}
#[doc = "Field `P1_20OD` writer - Port 1 pin 20open drain mode control, see P1.00OD"]
pub type P1_20odW<'a, REG> = crate::BitWriter<'a, REG, P1_20od>;
impl<'a, REG> P1_20odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P1.20 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(P1_20od::Normal)
    }
    #[doc = "Open-drain. P1.20 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(P1_20od::OpenDrain)
    }
}
#[doc = "Port 1 pin 21 open drain mode control, see P1.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P1_21od {
    #[doc = "0: Normal. P1.21 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P1.21 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<P1_21od> for bool {
    #[inline(always)]
    fn from(variant: P1_21od) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P1_21OD` reader - Port 1 pin 21 open drain mode control, see P1.00OD"]
pub type P1_21odR = crate::BitReader<P1_21od>;
impl P1_21odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_21od {
        match self.bits {
            false => P1_21od::Normal,
            true => P1_21od::OpenDrain,
        }
    }
    #[doc = "Normal. P1.21 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P1_21od::Normal
    }
    #[doc = "Open-drain. P1.21 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_21od::OpenDrain
    }
}
#[doc = "Field `P1_21OD` writer - Port 1 pin 21 open drain mode control, see P1.00OD"]
pub type P1_21odW<'a, REG> = crate::BitWriter<'a, REG, P1_21od>;
impl<'a, REG> P1_21odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P1.21 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(P1_21od::Normal)
    }
    #[doc = "Open-drain. P1.21 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(P1_21od::OpenDrain)
    }
}
#[doc = "Port 1 pin 22 open drain mode control, see P1.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P1_22od {
    #[doc = "0: Normal. P1.22 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P1.22 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<P1_22od> for bool {
    #[inline(always)]
    fn from(variant: P1_22od) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P1_22OD` reader - Port 1 pin 22 open drain mode control, see P1.00OD"]
pub type P1_22odR = crate::BitReader<P1_22od>;
impl P1_22odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_22od {
        match self.bits {
            false => P1_22od::Normal,
            true => P1_22od::OpenDrain,
        }
    }
    #[doc = "Normal. P1.22 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P1_22od::Normal
    }
    #[doc = "Open-drain. P1.22 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_22od::OpenDrain
    }
}
#[doc = "Field `P1_22OD` writer - Port 1 pin 22 open drain mode control, see P1.00OD"]
pub type P1_22odW<'a, REG> = crate::BitWriter<'a, REG, P1_22od>;
impl<'a, REG> P1_22odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P1.22 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(P1_22od::Normal)
    }
    #[doc = "Open-drain. P1.22 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(P1_22od::OpenDrain)
    }
}
#[doc = "Port 1 pin 23 open drain mode control, see P1.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P1_23od {
    #[doc = "0: Normal. P1.23 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P1.23 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<P1_23od> for bool {
    #[inline(always)]
    fn from(variant: P1_23od) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P1_23OD` reader - Port 1 pin 23 open drain mode control, see P1.00OD"]
pub type P1_23odR = crate::BitReader<P1_23od>;
impl P1_23odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_23od {
        match self.bits {
            false => P1_23od::Normal,
            true => P1_23od::OpenDrain,
        }
    }
    #[doc = "Normal. P1.23 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P1_23od::Normal
    }
    #[doc = "Open-drain. P1.23 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_23od::OpenDrain
    }
}
#[doc = "Field `P1_23OD` writer - Port 1 pin 23 open drain mode control, see P1.00OD"]
pub type P1_23odW<'a, REG> = crate::BitWriter<'a, REG, P1_23od>;
impl<'a, REG> P1_23odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P1.23 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(P1_23od::Normal)
    }
    #[doc = "Open-drain. P1.23 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(P1_23od::OpenDrain)
    }
}
#[doc = "Port 1 pin 24open drain mode control, see P1.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P1_24od {
    #[doc = "0: Normal. P1.24 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P1.24 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<P1_24od> for bool {
    #[inline(always)]
    fn from(variant: P1_24od) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P1_24OD` reader - Port 1 pin 24open drain mode control, see P1.00OD"]
pub type P1_24odR = crate::BitReader<P1_24od>;
impl P1_24odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_24od {
        match self.bits {
            false => P1_24od::Normal,
            true => P1_24od::OpenDrain,
        }
    }
    #[doc = "Normal. P1.24 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P1_24od::Normal
    }
    #[doc = "Open-drain. P1.24 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_24od::OpenDrain
    }
}
#[doc = "Field `P1_24OD` writer - Port 1 pin 24open drain mode control, see P1.00OD"]
pub type P1_24odW<'a, REG> = crate::BitWriter<'a, REG, P1_24od>;
impl<'a, REG> P1_24odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P1.24 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(P1_24od::Normal)
    }
    #[doc = "Open-drain. P1.24 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(P1_24od::OpenDrain)
    }
}
#[doc = "Port 1 pin 25 open drain mode control, see P1.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P1_25od {
    #[doc = "0: Normal. P1.25 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P1.25 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<P1_25od> for bool {
    #[inline(always)]
    fn from(variant: P1_25od) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P1_25OD` reader - Port 1 pin 25 open drain mode control, see P1.00OD"]
pub type P1_25odR = crate::BitReader<P1_25od>;
impl P1_25odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_25od {
        match self.bits {
            false => P1_25od::Normal,
            true => P1_25od::OpenDrain,
        }
    }
    #[doc = "Normal. P1.25 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P1_25od::Normal
    }
    #[doc = "Open-drain. P1.25 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_25od::OpenDrain
    }
}
#[doc = "Field `P1_25OD` writer - Port 1 pin 25 open drain mode control, see P1.00OD"]
pub type P1_25odW<'a, REG> = crate::BitWriter<'a, REG, P1_25od>;
impl<'a, REG> P1_25odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P1.25 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(P1_25od::Normal)
    }
    #[doc = "Open-drain. P1.25 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(P1_25od::OpenDrain)
    }
}
#[doc = "Port 1 pin 26 open drain mode control, see P1.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P1_26od {
    #[doc = "0: Normal. P1.26 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P1.26 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<P1_26od> for bool {
    #[inline(always)]
    fn from(variant: P1_26od) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P1_26OD` reader - Port 1 pin 26 open drain mode control, see P1.00OD"]
pub type P1_26odR = crate::BitReader<P1_26od>;
impl P1_26odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_26od {
        match self.bits {
            false => P1_26od::Normal,
            true => P1_26od::OpenDrain,
        }
    }
    #[doc = "Normal. P1.26 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P1_26od::Normal
    }
    #[doc = "Open-drain. P1.26 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_26od::OpenDrain
    }
}
#[doc = "Field `P1_26OD` writer - Port 1 pin 26 open drain mode control, see P1.00OD"]
pub type P1_26odW<'a, REG> = crate::BitWriter<'a, REG, P1_26od>;
impl<'a, REG> P1_26odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P1.26 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(P1_26od::Normal)
    }
    #[doc = "Open-drain. P1.26 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(P1_26od::OpenDrain)
    }
}
#[doc = "Port 1 pin 27 open drain mode control, see P1.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P1_27od {
    #[doc = "0: Normal. P1.27 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P1.27 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<P1_27od> for bool {
    #[inline(always)]
    fn from(variant: P1_27od) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P1_27OD` reader - Port 1 pin 27 open drain mode control, see P1.00OD"]
pub type P1_27odR = crate::BitReader<P1_27od>;
impl P1_27odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_27od {
        match self.bits {
            false => P1_27od::Normal,
            true => P1_27od::OpenDrain,
        }
    }
    #[doc = "Normal. P1.27 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P1_27od::Normal
    }
    #[doc = "Open-drain. P1.27 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_27od::OpenDrain
    }
}
#[doc = "Field `P1_27OD` writer - Port 1 pin 27 open drain mode control, see P1.00OD"]
pub type P1_27odW<'a, REG> = crate::BitWriter<'a, REG, P1_27od>;
impl<'a, REG> P1_27odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P1.27 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(P1_27od::Normal)
    }
    #[doc = "Open-drain. P1.27 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(P1_27od::OpenDrain)
    }
}
#[doc = "Port 1 pin 28 open drain mode control, see P1.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P1_28od {
    #[doc = "0: Normal. P1.28 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P1.28 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<P1_28od> for bool {
    #[inline(always)]
    fn from(variant: P1_28od) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P1_28OD` reader - Port 1 pin 28 open drain mode control, see P1.00OD"]
pub type P1_28odR = crate::BitReader<P1_28od>;
impl P1_28odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_28od {
        match self.bits {
            false => P1_28od::Normal,
            true => P1_28od::OpenDrain,
        }
    }
    #[doc = "Normal. P1.28 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P1_28od::Normal
    }
    #[doc = "Open-drain. P1.28 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_28od::OpenDrain
    }
}
#[doc = "Field `P1_28OD` writer - Port 1 pin 28 open drain mode control, see P1.00OD"]
pub type P1_28odW<'a, REG> = crate::BitWriter<'a, REG, P1_28od>;
impl<'a, REG> P1_28odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P1.28 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(P1_28od::Normal)
    }
    #[doc = "Open-drain. P1.28 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(P1_28od::OpenDrain)
    }
}
#[doc = "Port 1 pin 29 open drain mode control, see P1.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P1_29od {
    #[doc = "0: Normal. P1.29 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P1.29 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<P1_29od> for bool {
    #[inline(always)]
    fn from(variant: P1_29od) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P1_29OD` reader - Port 1 pin 29 open drain mode control, see P1.00OD"]
pub type P1_29odR = crate::BitReader<P1_29od>;
impl P1_29odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_29od {
        match self.bits {
            false => P1_29od::Normal,
            true => P1_29od::OpenDrain,
        }
    }
    #[doc = "Normal. P1.29 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P1_29od::Normal
    }
    #[doc = "Open-drain. P1.29 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_29od::OpenDrain
    }
}
#[doc = "Field `P1_29OD` writer - Port 1 pin 29 open drain mode control, see P1.00OD"]
pub type P1_29odW<'a, REG> = crate::BitWriter<'a, REG, P1_29od>;
impl<'a, REG> P1_29odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P1.29 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(P1_29od::Normal)
    }
    #[doc = "Open-drain. P1.29 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(P1_29od::OpenDrain)
    }
}
#[doc = "Port 1 pin 30 open drain mode control, see P1.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P1_30od {
    #[doc = "0: Normal. P1.30 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P1.30 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<P1_30od> for bool {
    #[inline(always)]
    fn from(variant: P1_30od) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P1_30OD` reader - Port 1 pin 30 open drain mode control, see P1.00OD"]
pub type P1_30odR = crate::BitReader<P1_30od>;
impl P1_30odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_30od {
        match self.bits {
            false => P1_30od::Normal,
            true => P1_30od::OpenDrain,
        }
    }
    #[doc = "Normal. P1.30 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P1_30od::Normal
    }
    #[doc = "Open-drain. P1.30 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_30od::OpenDrain
    }
}
#[doc = "Field `P1_30OD` writer - Port 1 pin 30 open drain mode control, see P1.00OD"]
pub type P1_30odW<'a, REG> = crate::BitWriter<'a, REG, P1_30od>;
impl<'a, REG> P1_30odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P1.30 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(P1_30od::Normal)
    }
    #[doc = "Open-drain. P1.30 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(P1_30od::OpenDrain)
    }
}
#[doc = "Port 1 pin 31 open drain mode control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P1_31od {
    #[doc = "0: Normal. P1.31 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P1.31 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<P1_31od> for bool {
    #[inline(always)]
    fn from(variant: P1_31od) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P1_31OD` reader - Port 1 pin 31 open drain mode control."]
pub type P1_31odR = crate::BitReader<P1_31od>;
impl P1_31odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_31od {
        match self.bits {
            false => P1_31od::Normal,
            true => P1_31od::OpenDrain,
        }
    }
    #[doc = "Normal. P1.31 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P1_31od::Normal
    }
    #[doc = "Open-drain. P1.31 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_31od::OpenDrain
    }
}
#[doc = "Field `P1_31OD` writer - Port 1 pin 31 open drain mode control."]
pub type P1_31odW<'a, REG> = crate::BitWriter<'a, REG, P1_31od>;
impl<'a, REG> P1_31odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P1.31 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(P1_31od::Normal)
    }
    #[doc = "Open-drain. P1.31 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(P1_31od::OpenDrain)
    }
}
impl R {
    #[doc = "Bit 0 - Port 1 pin 0 open drain mode control."]
    #[inline(always)]
    pub fn p1_00od(&self) -> P1_00odR {
        P1_00odR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port 1 pin 1 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_01od(&self) -> P1_01odR {
        P1_01odR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Port 1 pin 4 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_04od(&self) -> P1_04odR {
        P1_04odR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Port 1 pin 8 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_08od(&self) -> P1_08odR {
        P1_08odR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port 1 pin 9 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_09od(&self) -> P1_09odR {
        P1_09odR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port 1 pin 10 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_10od(&self) -> P1_10odR {
        P1_10odR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 14 - Port 1 pin 14 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_14od(&self) -> P1_14odR {
        P1_14odR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port 1 pin 15 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_15od(&self) -> P1_15odR {
        P1_15odR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Port 1 pin 16 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_16od(&self) -> P1_16odR {
        P1_16odR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Port 1 pin 17 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_17od(&self) -> P1_17odR {
        P1_17odR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Port 1 pin 18 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_18od(&self) -> P1_18odR {
        P1_18odR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Port 1 pin 19 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_19od(&self) -> P1_19odR {
        P1_19odR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Port 1 pin 20open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_20od(&self) -> P1_20odR {
        P1_20odR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Port 1 pin 21 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_21od(&self) -> P1_21odR {
        P1_21odR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Port 1 pin 22 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_22od(&self) -> P1_22odR {
        P1_22odR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Port 1 pin 23 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_23od(&self) -> P1_23odR {
        P1_23odR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Port 1 pin 24open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_24od(&self) -> P1_24odR {
        P1_24odR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Port 1 pin 25 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_25od(&self) -> P1_25odR {
        P1_25odR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Port 1 pin 26 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_26od(&self) -> P1_26odR {
        P1_26odR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Port 1 pin 27 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_27od(&self) -> P1_27odR {
        P1_27odR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Port 1 pin 28 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_28od(&self) -> P1_28odR {
        P1_28odR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Port 1 pin 29 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_29od(&self) -> P1_29odR {
        P1_29odR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Port 1 pin 30 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_30od(&self) -> P1_30odR {
        P1_30odR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Port 1 pin 31 open drain mode control."]
    #[inline(always)]
    pub fn p1_31od(&self) -> P1_31odR {
        P1_31odR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port 1 pin 0 open drain mode control."]
    #[inline(always)]
    #[must_use]
    pub fn p1_00od(&mut self) -> P1_00odW<PinmodeOd1Spec> {
        P1_00odW::new(self, 0)
    }
    #[doc = "Bit 1 - Port 1 pin 1 open drain mode control, see P1.00OD"]
    #[inline(always)]
    #[must_use]
    pub fn p1_01od(&mut self) -> P1_01odW<PinmodeOd1Spec> {
        P1_01odW::new(self, 1)
    }
    #[doc = "Bit 4 - Port 1 pin 4 open drain mode control, see P1.00OD"]
    #[inline(always)]
    #[must_use]
    pub fn p1_04od(&mut self) -> P1_04odW<PinmodeOd1Spec> {
        P1_04odW::new(self, 4)
    }
    #[doc = "Bit 8 - Port 1 pin 8 open drain mode control, see P1.00OD"]
    #[inline(always)]
    #[must_use]
    pub fn p1_08od(&mut self) -> P1_08odW<PinmodeOd1Spec> {
        P1_08odW::new(self, 8)
    }
    #[doc = "Bit 9 - Port 1 pin 9 open drain mode control, see P1.00OD"]
    #[inline(always)]
    #[must_use]
    pub fn p1_09od(&mut self) -> P1_09odW<PinmodeOd1Spec> {
        P1_09odW::new(self, 9)
    }
    #[doc = "Bit 10 - Port 1 pin 10 open drain mode control, see P1.00OD"]
    #[inline(always)]
    #[must_use]
    pub fn p1_10od(&mut self) -> P1_10odW<PinmodeOd1Spec> {
        P1_10odW::new(self, 10)
    }
    #[doc = "Bit 14 - Port 1 pin 14 open drain mode control, see P1.00OD"]
    #[inline(always)]
    #[must_use]
    pub fn p1_14od(&mut self) -> P1_14odW<PinmodeOd1Spec> {
        P1_14odW::new(self, 14)
    }
    #[doc = "Bit 15 - Port 1 pin 15 open drain mode control, see P1.00OD"]
    #[inline(always)]
    #[must_use]
    pub fn p1_15od(&mut self) -> P1_15odW<PinmodeOd1Spec> {
        P1_15odW::new(self, 15)
    }
    #[doc = "Bit 16 - Port 1 pin 16 open drain mode control, see P1.00OD"]
    #[inline(always)]
    #[must_use]
    pub fn p1_16od(&mut self) -> P1_16odW<PinmodeOd1Spec> {
        P1_16odW::new(self, 16)
    }
    #[doc = "Bit 17 - Port 1 pin 17 open drain mode control, see P1.00OD"]
    #[inline(always)]
    #[must_use]
    pub fn p1_17od(&mut self) -> P1_17odW<PinmodeOd1Spec> {
        P1_17odW::new(self, 17)
    }
    #[doc = "Bit 18 - Port 1 pin 18 open drain mode control, see P1.00OD"]
    #[inline(always)]
    #[must_use]
    pub fn p1_18od(&mut self) -> P1_18odW<PinmodeOd1Spec> {
        P1_18odW::new(self, 18)
    }
    #[doc = "Bit 19 - Port 1 pin 19 open drain mode control, see P1.00OD"]
    #[inline(always)]
    #[must_use]
    pub fn p1_19od(&mut self) -> P1_19odW<PinmodeOd1Spec> {
        P1_19odW::new(self, 19)
    }
    #[doc = "Bit 20 - Port 1 pin 20open drain mode control, see P1.00OD"]
    #[inline(always)]
    #[must_use]
    pub fn p1_20od(&mut self) -> P1_20odW<PinmodeOd1Spec> {
        P1_20odW::new(self, 20)
    }
    #[doc = "Bit 21 - Port 1 pin 21 open drain mode control, see P1.00OD"]
    #[inline(always)]
    #[must_use]
    pub fn p1_21od(&mut self) -> P1_21odW<PinmodeOd1Spec> {
        P1_21odW::new(self, 21)
    }
    #[doc = "Bit 22 - Port 1 pin 22 open drain mode control, see P1.00OD"]
    #[inline(always)]
    #[must_use]
    pub fn p1_22od(&mut self) -> P1_22odW<PinmodeOd1Spec> {
        P1_22odW::new(self, 22)
    }
    #[doc = "Bit 23 - Port 1 pin 23 open drain mode control, see P1.00OD"]
    #[inline(always)]
    #[must_use]
    pub fn p1_23od(&mut self) -> P1_23odW<PinmodeOd1Spec> {
        P1_23odW::new(self, 23)
    }
    #[doc = "Bit 24 - Port 1 pin 24open drain mode control, see P1.00OD"]
    #[inline(always)]
    #[must_use]
    pub fn p1_24od(&mut self) -> P1_24odW<PinmodeOd1Spec> {
        P1_24odW::new(self, 24)
    }
    #[doc = "Bit 25 - Port 1 pin 25 open drain mode control, see P1.00OD"]
    #[inline(always)]
    #[must_use]
    pub fn p1_25od(&mut self) -> P1_25odW<PinmodeOd1Spec> {
        P1_25odW::new(self, 25)
    }
    #[doc = "Bit 26 - Port 1 pin 26 open drain mode control, see P1.00OD"]
    #[inline(always)]
    #[must_use]
    pub fn p1_26od(&mut self) -> P1_26odW<PinmodeOd1Spec> {
        P1_26odW::new(self, 26)
    }
    #[doc = "Bit 27 - Port 1 pin 27 open drain mode control, see P1.00OD"]
    #[inline(always)]
    #[must_use]
    pub fn p1_27od(&mut self) -> P1_27odW<PinmodeOd1Spec> {
        P1_27odW::new(self, 27)
    }
    #[doc = "Bit 28 - Port 1 pin 28 open drain mode control, see P1.00OD"]
    #[inline(always)]
    #[must_use]
    pub fn p1_28od(&mut self) -> P1_28odW<PinmodeOd1Spec> {
        P1_28odW::new(self, 28)
    }
    #[doc = "Bit 29 - Port 1 pin 29 open drain mode control, see P1.00OD"]
    #[inline(always)]
    #[must_use]
    pub fn p1_29od(&mut self) -> P1_29odW<PinmodeOd1Spec> {
        P1_29odW::new(self, 29)
    }
    #[doc = "Bit 30 - Port 1 pin 30 open drain mode control, see P1.00OD"]
    #[inline(always)]
    #[must_use]
    pub fn p1_30od(&mut self) -> P1_30odW<PinmodeOd1Spec> {
        P1_30odW::new(self, 30)
    }
    #[doc = "Bit 31 - Port 1 pin 31 open drain mode control."]
    #[inline(always)]
    #[must_use]
    pub fn p1_31od(&mut self) -> P1_31odW<PinmodeOd1Spec> {
        P1_31odW::new(self, 31)
    }
}
#[doc = "Open drain mode control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pinmode_od1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pinmode_od1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PinmodeOd1Spec;
impl crate::RegisterSpec for PinmodeOd1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pinmode_od1::R`](R) reader structure"]
impl crate::Readable for PinmodeOd1Spec {}
#[doc = "`write(|w| ..)` method takes [`pinmode_od1::W`](W) writer structure"]
impl crate::Writable for PinmodeOd1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PINMODE_OD1 to value 0"]
impl crate::Resettable for PinmodeOd1Spec {
    const RESET_VALUE: u32 = 0;
}
