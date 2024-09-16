#[doc = "Register `PINMODE_OD2` reader"]
pub type R = crate::R<PinmodeOd2Spec>;
#[doc = "Register `PINMODE_OD2` writer"]
pub type W = crate::W<PinmodeOd2Spec>;
#[doc = "Port 2 pin 0 open drain mode control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P2_00od {
    #[doc = "0: Normal. P2.0 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P2.0 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<P2_00od> for bool {
    #[inline(always)]
    fn from(variant: P2_00od) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P2_00OD` reader - Port 2 pin 0 open drain mode control."]
pub type P2_00odR = crate::BitReader<P2_00od>;
impl P2_00odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P2_00od {
        match self.bits {
            false => P2_00od::Normal,
            true => P2_00od::OpenDrain,
        }
    }
    #[doc = "Normal. P2.0 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P2_00od::Normal
    }
    #[doc = "Open-drain. P2.0 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P2_00od::OpenDrain
    }
}
#[doc = "Field `P2_00OD` writer - Port 2 pin 0 open drain mode control."]
pub type P2_00odW<'a, REG> = crate::BitWriter<'a, REG, P2_00od>;
impl<'a, REG> P2_00odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P2.0 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(P2_00od::Normal)
    }
    #[doc = "Open-drain. P2.0 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(P2_00od::OpenDrain)
    }
}
#[doc = "Port 2 pin 1 open drain mode control, see P2.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P2_01od {
    #[doc = "0: Normal. P2.1 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P2.1p in is in the open drain mode."]
    OpenDrain = 1,
}
impl From<P2_01od> for bool {
    #[inline(always)]
    fn from(variant: P2_01od) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P2_01OD` reader - Port 2 pin 1 open drain mode control, see P2.00OD"]
pub type P2_01odR = crate::BitReader<P2_01od>;
impl P2_01odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P2_01od {
        match self.bits {
            false => P2_01od::Normal,
            true => P2_01od::OpenDrain,
        }
    }
    #[doc = "Normal. P2.1 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P2_01od::Normal
    }
    #[doc = "Open-drain. P2.1p in is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P2_01od::OpenDrain
    }
}
#[doc = "Field `P2_01OD` writer - Port 2 pin 1 open drain mode control, see P2.00OD"]
pub type P2_01odW<'a, REG> = crate::BitWriter<'a, REG, P2_01od>;
impl<'a, REG> P2_01odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P2.1 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(P2_01od::Normal)
    }
    #[doc = "Open-drain. P2.1p in is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(P2_01od::OpenDrain)
    }
}
#[doc = "Port 2 pin 2 open drain mode control, see P2.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P2_02od {
    #[doc = "0: Normal. P2.2 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P2.2 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<P2_02od> for bool {
    #[inline(always)]
    fn from(variant: P2_02od) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P2_02OD` reader - Port 2 pin 2 open drain mode control, see P2.00OD"]
pub type P2_02odR = crate::BitReader<P2_02od>;
impl P2_02odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P2_02od {
        match self.bits {
            false => P2_02od::Normal,
            true => P2_02od::OpenDrain,
        }
    }
    #[doc = "Normal. P2.2 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P2_02od::Normal
    }
    #[doc = "Open-drain. P2.2 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P2_02od::OpenDrain
    }
}
#[doc = "Field `P2_02OD` writer - Port 2 pin 2 open drain mode control, see P2.00OD"]
pub type P2_02odW<'a, REG> = crate::BitWriter<'a, REG, P2_02od>;
impl<'a, REG> P2_02odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P2.2 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(P2_02od::Normal)
    }
    #[doc = "Open-drain. P2.2 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(P2_02od::OpenDrain)
    }
}
#[doc = "Port 2 pin 3 open drain mode control, see P2.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P2_03od {
    #[doc = "0: Normal. P2.3 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P2.3 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<P2_03od> for bool {
    #[inline(always)]
    fn from(variant: P2_03od) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P2_03OD` reader - Port 2 pin 3 open drain mode control, see P2.00OD"]
pub type P2_03odR = crate::BitReader<P2_03od>;
impl P2_03odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P2_03od {
        match self.bits {
            false => P2_03od::Normal,
            true => P2_03od::OpenDrain,
        }
    }
    #[doc = "Normal. P2.3 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P2_03od::Normal
    }
    #[doc = "Open-drain. P2.3 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P2_03od::OpenDrain
    }
}
#[doc = "Field `P2_03OD` writer - Port 2 pin 3 open drain mode control, see P2.00OD"]
pub type P2_03odW<'a, REG> = crate::BitWriter<'a, REG, P2_03od>;
impl<'a, REG> P2_03odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P2.3 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(P2_03od::Normal)
    }
    #[doc = "Open-drain. P2.3 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(P2_03od::OpenDrain)
    }
}
#[doc = "Port 2 pin 4 open drain mode control, see P2.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P2_04od {
    #[doc = "0: Normal. P2.4 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P2.4 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<P2_04od> for bool {
    #[inline(always)]
    fn from(variant: P2_04od) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P2_04OD` reader - Port 2 pin 4 open drain mode control, see P2.00OD"]
pub type P2_04odR = crate::BitReader<P2_04od>;
impl P2_04odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P2_04od {
        match self.bits {
            false => P2_04od::Normal,
            true => P2_04od::OpenDrain,
        }
    }
    #[doc = "Normal. P2.4 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P2_04od::Normal
    }
    #[doc = "Open-drain. P2.4 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P2_04od::OpenDrain
    }
}
#[doc = "Field `P2_04OD` writer - Port 2 pin 4 open drain mode control, see P2.00OD"]
pub type P2_04odW<'a, REG> = crate::BitWriter<'a, REG, P2_04od>;
impl<'a, REG> P2_04odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P2.4 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(P2_04od::Normal)
    }
    #[doc = "Open-drain. P2.4 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(P2_04od::OpenDrain)
    }
}
#[doc = "Port 2 pin 5 open drain mode control, see P2.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P2_05od {
    #[doc = "0: Normal. P2.5 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P2.5 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<P2_05od> for bool {
    #[inline(always)]
    fn from(variant: P2_05od) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P2_05OD` reader - Port 2 pin 5 open drain mode control, see P2.00OD"]
pub type P2_05odR = crate::BitReader<P2_05od>;
impl P2_05odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P2_05od {
        match self.bits {
            false => P2_05od::Normal,
            true => P2_05od::OpenDrain,
        }
    }
    #[doc = "Normal. P2.5 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P2_05od::Normal
    }
    #[doc = "Open-drain. P2.5 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P2_05od::OpenDrain
    }
}
#[doc = "Field `P2_05OD` writer - Port 2 pin 5 open drain mode control, see P2.00OD"]
pub type P2_05odW<'a, REG> = crate::BitWriter<'a, REG, P2_05od>;
impl<'a, REG> P2_05odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P2.5 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(P2_05od::Normal)
    }
    #[doc = "Open-drain. P2.5 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(P2_05od::OpenDrain)
    }
}
#[doc = "Port 2 pin 6 open drain mode control, see P2.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P2_06od {
    #[doc = "0: Normal. P2.6 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P2.6 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<P2_06od> for bool {
    #[inline(always)]
    fn from(variant: P2_06od) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P2_06OD` reader - Port 2 pin 6 open drain mode control, see P2.00OD"]
pub type P2_06odR = crate::BitReader<P2_06od>;
impl P2_06odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P2_06od {
        match self.bits {
            false => P2_06od::Normal,
            true => P2_06od::OpenDrain,
        }
    }
    #[doc = "Normal. P2.6 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P2_06od::Normal
    }
    #[doc = "Open-drain. P2.6 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P2_06od::OpenDrain
    }
}
#[doc = "Field `P2_06OD` writer - Port 2 pin 6 open drain mode control, see P2.00OD"]
pub type P2_06odW<'a, REG> = crate::BitWriter<'a, REG, P2_06od>;
impl<'a, REG> P2_06odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P2.6 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(P2_06od::Normal)
    }
    #[doc = "Open-drain. P2.6 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(P2_06od::OpenDrain)
    }
}
#[doc = "Port 2 pin 7 open drain mode control, see P2.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P2_07od {
    #[doc = "0: Normal. P2.7 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P2.7 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<P2_07od> for bool {
    #[inline(always)]
    fn from(variant: P2_07od) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P2_07OD` reader - Port 2 pin 7 open drain mode control, see P2.00OD"]
pub type P2_07odR = crate::BitReader<P2_07od>;
impl P2_07odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P2_07od {
        match self.bits {
            false => P2_07od::Normal,
            true => P2_07od::OpenDrain,
        }
    }
    #[doc = "Normal. P2.7 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P2_07od::Normal
    }
    #[doc = "Open-drain. P2.7 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P2_07od::OpenDrain
    }
}
#[doc = "Field `P2_07OD` writer - Port 2 pin 7 open drain mode control, see P2.00OD"]
pub type P2_07odW<'a, REG> = crate::BitWriter<'a, REG, P2_07od>;
impl<'a, REG> P2_07odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P2.7 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(P2_07od::Normal)
    }
    #[doc = "Open-drain. P2.7 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(P2_07od::OpenDrain)
    }
}
#[doc = "Port 2 pin 8 open drain mode control, see P2.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P2_08od {
    #[doc = "0: Normal. P2.8 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P2.8 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<P2_08od> for bool {
    #[inline(always)]
    fn from(variant: P2_08od) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P2_08OD` reader - Port 2 pin 8 open drain mode control, see P2.00OD"]
pub type P2_08odR = crate::BitReader<P2_08od>;
impl P2_08odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P2_08od {
        match self.bits {
            false => P2_08od::Normal,
            true => P2_08od::OpenDrain,
        }
    }
    #[doc = "Normal. P2.8 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P2_08od::Normal
    }
    #[doc = "Open-drain. P2.8 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P2_08od::OpenDrain
    }
}
#[doc = "Field `P2_08OD` writer - Port 2 pin 8 open drain mode control, see P2.00OD"]
pub type P2_08odW<'a, REG> = crate::BitWriter<'a, REG, P2_08od>;
impl<'a, REG> P2_08odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P2.8 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(P2_08od::Normal)
    }
    #[doc = "Open-drain. P2.8 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(P2_08od::OpenDrain)
    }
}
#[doc = "Port 2 pin 9 open drain mode control, see P2.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P2_09od {
    #[doc = "0: Normal. P2.9 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P2.9 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<P2_09od> for bool {
    #[inline(always)]
    fn from(variant: P2_09od) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P2_09OD` reader - Port 2 pin 9 open drain mode control, see P2.00OD"]
pub type P2_09odR = crate::BitReader<P2_09od>;
impl P2_09odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P2_09od {
        match self.bits {
            false => P2_09od::Normal,
            true => P2_09od::OpenDrain,
        }
    }
    #[doc = "Normal. P2.9 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P2_09od::Normal
    }
    #[doc = "Open-drain. P2.9 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P2_09od::OpenDrain
    }
}
#[doc = "Field `P2_09OD` writer - Port 2 pin 9 open drain mode control, see P2.00OD"]
pub type P2_09odW<'a, REG> = crate::BitWriter<'a, REG, P2_09od>;
impl<'a, REG> P2_09odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P2.9 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(P2_09od::Normal)
    }
    #[doc = "Open-drain. P2.9 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(P2_09od::OpenDrain)
    }
}
#[doc = "Port 2 pin 10 open drain mode control, see P2.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P2_10od {
    #[doc = "0: Normal. P2.10 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P2.10 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<P2_10od> for bool {
    #[inline(always)]
    fn from(variant: P2_10od) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P2_10OD` reader - Port 2 pin 10 open drain mode control, see P2.00OD"]
pub type P2_10odR = crate::BitReader<P2_10od>;
impl P2_10odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P2_10od {
        match self.bits {
            false => P2_10od::Normal,
            true => P2_10od::OpenDrain,
        }
    }
    #[doc = "Normal. P2.10 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P2_10od::Normal
    }
    #[doc = "Open-drain. P2.10 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P2_10od::OpenDrain
    }
}
#[doc = "Field `P2_10OD` writer - Port 2 pin 10 open drain mode control, see P2.00OD"]
pub type P2_10odW<'a, REG> = crate::BitWriter<'a, REG, P2_10od>;
impl<'a, REG> P2_10odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P2.10 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(P2_10od::Normal)
    }
    #[doc = "Open-drain. P2.10 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(P2_10od::OpenDrain)
    }
}
#[doc = "Port 2 pin 11 open drain mode control, see P2.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P2_11od {
    #[doc = "0: Normal. P2.11 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P2.11 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<P2_11od> for bool {
    #[inline(always)]
    fn from(variant: P2_11od) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P2_11OD` reader - Port 2 pin 11 open drain mode control, see P2.00OD"]
pub type P2_11odR = crate::BitReader<P2_11od>;
impl P2_11odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P2_11od {
        match self.bits {
            false => P2_11od::Normal,
            true => P2_11od::OpenDrain,
        }
    }
    #[doc = "Normal. P2.11 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P2_11od::Normal
    }
    #[doc = "Open-drain. P2.11 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P2_11od::OpenDrain
    }
}
#[doc = "Field `P2_11OD` writer - Port 2 pin 11 open drain mode control, see P2.00OD"]
pub type P2_11odW<'a, REG> = crate::BitWriter<'a, REG, P2_11od>;
impl<'a, REG> P2_11odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P2.11 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(P2_11od::Normal)
    }
    #[doc = "Open-drain. P2.11 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(P2_11od::OpenDrain)
    }
}
#[doc = "Port 2 pin 12 open drain mode control, see P2.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P2_12od {
    #[doc = "0: Normal. P2.12 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P2.12 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<P2_12od> for bool {
    #[inline(always)]
    fn from(variant: P2_12od) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P2_12OD` reader - Port 2 pin 12 open drain mode control, see P2.00OD"]
pub type P2_12odR = crate::BitReader<P2_12od>;
impl P2_12odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P2_12od {
        match self.bits {
            false => P2_12od::Normal,
            true => P2_12od::OpenDrain,
        }
    }
    #[doc = "Normal. P2.12 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P2_12od::Normal
    }
    #[doc = "Open-drain. P2.12 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P2_12od::OpenDrain
    }
}
#[doc = "Field `P2_12OD` writer - Port 2 pin 12 open drain mode control, see P2.00OD"]
pub type P2_12odW<'a, REG> = crate::BitWriter<'a, REG, P2_12od>;
impl<'a, REG> P2_12odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P2.12 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(P2_12od::Normal)
    }
    #[doc = "Open-drain. P2.12 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(P2_12od::OpenDrain)
    }
}
#[doc = "Port 2 pin 13 open drain mode control, see P2.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P2_13od {
    #[doc = "0: Normal. P2.13 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P2.13 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<P2_13od> for bool {
    #[inline(always)]
    fn from(variant: P2_13od) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P2_13OD` reader - Port 2 pin 13 open drain mode control, see P2.00OD"]
pub type P2_13odR = crate::BitReader<P2_13od>;
impl P2_13odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P2_13od {
        match self.bits {
            false => P2_13od::Normal,
            true => P2_13od::OpenDrain,
        }
    }
    #[doc = "Normal. P2.13 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P2_13od::Normal
    }
    #[doc = "Open-drain. P2.13 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P2_13od::OpenDrain
    }
}
#[doc = "Field `P2_13OD` writer - Port 2 pin 13 open drain mode control, see P2.00OD"]
pub type P2_13odW<'a, REG> = crate::BitWriter<'a, REG, P2_13od>;
impl<'a, REG> P2_13odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P2.13 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(P2_13od::Normal)
    }
    #[doc = "Open-drain. P2.13 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(P2_13od::OpenDrain)
    }
}
impl R {
    #[doc = "Bit 0 - Port 2 pin 0 open drain mode control."]
    #[inline(always)]
    pub fn p2_00od(&self) -> P2_00odR {
        P2_00odR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port 2 pin 1 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_01od(&self) -> P2_01odR {
        P2_01odR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port 2 pin 2 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_02od(&self) -> P2_02odR {
        P2_02odR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port 2 pin 3 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_03od(&self) -> P2_03odR {
        P2_03odR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port 2 pin 4 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_04od(&self) -> P2_04odR {
        P2_04odR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port 2 pin 5 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_05od(&self) -> P2_05odR {
        P2_05odR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port 2 pin 6 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_06od(&self) -> P2_06odR {
        P2_06odR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port 2 pin 7 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_07od(&self) -> P2_07odR {
        P2_07odR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port 2 pin 8 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_08od(&self) -> P2_08odR {
        P2_08odR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port 2 pin 9 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_09od(&self) -> P2_09odR {
        P2_09odR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port 2 pin 10 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_10od(&self) -> P2_10odR {
        P2_10odR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port 2 pin 11 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_11od(&self) -> P2_11odR {
        P2_11odR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port 2 pin 12 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_12od(&self) -> P2_12odR {
        P2_12odR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port 2 pin 13 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_13od(&self) -> P2_13odR {
        P2_13odR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port 2 pin 0 open drain mode control."]
    #[inline(always)]
    #[must_use]
    pub fn p2_00od(&mut self) -> P2_00odW<PinmodeOd2Spec> {
        P2_00odW::new(self, 0)
    }
    #[doc = "Bit 1 - Port 2 pin 1 open drain mode control, see P2.00OD"]
    #[inline(always)]
    #[must_use]
    pub fn p2_01od(&mut self) -> P2_01odW<PinmodeOd2Spec> {
        P2_01odW::new(self, 1)
    }
    #[doc = "Bit 2 - Port 2 pin 2 open drain mode control, see P2.00OD"]
    #[inline(always)]
    #[must_use]
    pub fn p2_02od(&mut self) -> P2_02odW<PinmodeOd2Spec> {
        P2_02odW::new(self, 2)
    }
    #[doc = "Bit 3 - Port 2 pin 3 open drain mode control, see P2.00OD"]
    #[inline(always)]
    #[must_use]
    pub fn p2_03od(&mut self) -> P2_03odW<PinmodeOd2Spec> {
        P2_03odW::new(self, 3)
    }
    #[doc = "Bit 4 - Port 2 pin 4 open drain mode control, see P2.00OD"]
    #[inline(always)]
    #[must_use]
    pub fn p2_04od(&mut self) -> P2_04odW<PinmodeOd2Spec> {
        P2_04odW::new(self, 4)
    }
    #[doc = "Bit 5 - Port 2 pin 5 open drain mode control, see P2.00OD"]
    #[inline(always)]
    #[must_use]
    pub fn p2_05od(&mut self) -> P2_05odW<PinmodeOd2Spec> {
        P2_05odW::new(self, 5)
    }
    #[doc = "Bit 6 - Port 2 pin 6 open drain mode control, see P2.00OD"]
    #[inline(always)]
    #[must_use]
    pub fn p2_06od(&mut self) -> P2_06odW<PinmodeOd2Spec> {
        P2_06odW::new(self, 6)
    }
    #[doc = "Bit 7 - Port 2 pin 7 open drain mode control, see P2.00OD"]
    #[inline(always)]
    #[must_use]
    pub fn p2_07od(&mut self) -> P2_07odW<PinmodeOd2Spec> {
        P2_07odW::new(self, 7)
    }
    #[doc = "Bit 8 - Port 2 pin 8 open drain mode control, see P2.00OD"]
    #[inline(always)]
    #[must_use]
    pub fn p2_08od(&mut self) -> P2_08odW<PinmodeOd2Spec> {
        P2_08odW::new(self, 8)
    }
    #[doc = "Bit 9 - Port 2 pin 9 open drain mode control, see P2.00OD"]
    #[inline(always)]
    #[must_use]
    pub fn p2_09od(&mut self) -> P2_09odW<PinmodeOd2Spec> {
        P2_09odW::new(self, 9)
    }
    #[doc = "Bit 10 - Port 2 pin 10 open drain mode control, see P2.00OD"]
    #[inline(always)]
    #[must_use]
    pub fn p2_10od(&mut self) -> P2_10odW<PinmodeOd2Spec> {
        P2_10odW::new(self, 10)
    }
    #[doc = "Bit 11 - Port 2 pin 11 open drain mode control, see P2.00OD"]
    #[inline(always)]
    #[must_use]
    pub fn p2_11od(&mut self) -> P2_11odW<PinmodeOd2Spec> {
        P2_11odW::new(self, 11)
    }
    #[doc = "Bit 12 - Port 2 pin 12 open drain mode control, see P2.00OD"]
    #[inline(always)]
    #[must_use]
    pub fn p2_12od(&mut self) -> P2_12odW<PinmodeOd2Spec> {
        P2_12odW::new(self, 12)
    }
    #[doc = "Bit 13 - Port 2 pin 13 open drain mode control, see P2.00OD"]
    #[inline(always)]
    #[must_use]
    pub fn p2_13od(&mut self) -> P2_13odW<PinmodeOd2Spec> {
        P2_13odW::new(self, 13)
    }
}
#[doc = "Open drain mode control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`pinmode_od2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pinmode_od2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PinmodeOd2Spec;
impl crate::RegisterSpec for PinmodeOd2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pinmode_od2::R`](R) reader structure"]
impl crate::Readable for PinmodeOd2Spec {}
#[doc = "`write(|w| ..)` method takes [`pinmode_od2::W`](W) writer structure"]
impl crate::Writable for PinmodeOd2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PINMODE_OD2 to value 0"]
impl crate::Resettable for PinmodeOd2Spec {
    const RESET_VALUE: u32 = 0;
}
