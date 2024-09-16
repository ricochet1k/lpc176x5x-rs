#[doc = "Register `PINMODE_OD0` reader"]
pub type R = crate::R<PinmodeOd0Spec>;
#[doc = "Register `PINMODE_OD0` writer"]
pub type W = crate::W<PinmodeOd0Spec>;
#[doc = "Port 0 pin 0 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P0_00od {
    #[doc = "0: Normal. P0.0 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P0.0 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<P0_00od> for bool {
    #[inline(always)]
    fn from(variant: P0_00od) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P0_00OD` reader - Port 0 pin 0 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
pub type P0_00odR = crate::BitReader<P0_00od>;
impl P0_00odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_00od {
        match self.bits {
            false => P0_00od::Normal,
            true => P0_00od::OpenDrain,
        }
    }
    #[doc = "Normal. P0.0 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_00od::Normal
    }
    #[doc = "Open-drain. P0.0 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_00od::OpenDrain
    }
}
#[doc = "Field `P0_00OD` writer - Port 0 pin 0 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
pub type P0_00odW<'a, REG> = crate::BitWriter<'a, REG, P0_00od>;
impl<'a, REG> P0_00odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P0.0 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(P0_00od::Normal)
    }
    #[doc = "Open-drain. P0.0 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(P0_00od::OpenDrain)
    }
}
#[doc = "Port 0 pin 1 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P0_01od {
    #[doc = "0: Normal. P0.1 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P0.1 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<P0_01od> for bool {
    #[inline(always)]
    fn from(variant: P0_01od) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P0_01OD` reader - Port 0 pin 1 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
pub type P0_01odR = crate::BitReader<P0_01od>;
impl P0_01odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_01od {
        match self.bits {
            false => P0_01od::Normal,
            true => P0_01od::OpenDrain,
        }
    }
    #[doc = "Normal. P0.1 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_01od::Normal
    }
    #[doc = "Open-drain. P0.1 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_01od::OpenDrain
    }
}
#[doc = "Field `P0_01OD` writer - Port 0 pin 1 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
pub type P0_01odW<'a, REG> = crate::BitWriter<'a, REG, P0_01od>;
impl<'a, REG> P0_01odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P0.1 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(P0_01od::Normal)
    }
    #[doc = "Open-drain. P0.1 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(P0_01od::OpenDrain)
    }
}
#[doc = "Port 0 pin 2 open drain mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P0_02od {
    #[doc = "0: Normal. P0.2 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P0.2 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<P0_02od> for bool {
    #[inline(always)]
    fn from(variant: P0_02od) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P0_02OD` reader - Port 0 pin 2 open drain mode control"]
pub type P0_02odR = crate::BitReader<P0_02od>;
impl P0_02odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_02od {
        match self.bits {
            false => P0_02od::Normal,
            true => P0_02od::OpenDrain,
        }
    }
    #[doc = "Normal. P0.2 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_02od::Normal
    }
    #[doc = "Open-drain. P0.2 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_02od::OpenDrain
    }
}
#[doc = "Field `P0_02OD` writer - Port 0 pin 2 open drain mode control"]
pub type P0_02odW<'a, REG> = crate::BitWriter<'a, REG, P0_02od>;
impl<'a, REG> P0_02odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P0.2 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(P0_02od::Normal)
    }
    #[doc = "Open-drain. P0.2 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(P0_02od::OpenDrain)
    }
}
#[doc = "Port 0 pin 3 open drain mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P0_03od {
    #[doc = "0: Normal. P0.3 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P0.3 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<P0_03od> for bool {
    #[inline(always)]
    fn from(variant: P0_03od) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P0_03OD` reader - Port 0 pin 3 open drain mode control"]
pub type P0_03odR = crate::BitReader<P0_03od>;
impl P0_03odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_03od {
        match self.bits {
            false => P0_03od::Normal,
            true => P0_03od::OpenDrain,
        }
    }
    #[doc = "Normal. P0.3 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_03od::Normal
    }
    #[doc = "Open-drain. P0.3 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_03od::OpenDrain
    }
}
#[doc = "Field `P0_03OD` writer - Port 0 pin 3 open drain mode control"]
pub type P0_03odW<'a, REG> = crate::BitWriter<'a, REG, P0_03od>;
impl<'a, REG> P0_03odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P0.3 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(P0_03od::Normal)
    }
    #[doc = "Open-drain. P0.3 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(P0_03od::OpenDrain)
    }
}
#[doc = "Port 0 pin 4 open drain mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P0_04od {
    #[doc = "0: Normal. P0.4 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P0.4 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<P0_04od> for bool {
    #[inline(always)]
    fn from(variant: P0_04od) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P0_04OD` reader - Port 0 pin 4 open drain mode control"]
pub type P0_04odR = crate::BitReader<P0_04od>;
impl P0_04odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_04od {
        match self.bits {
            false => P0_04od::Normal,
            true => P0_04od::OpenDrain,
        }
    }
    #[doc = "Normal. P0.4 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_04od::Normal
    }
    #[doc = "Open-drain. P0.4 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_04od::OpenDrain
    }
}
#[doc = "Field `P0_04OD` writer - Port 0 pin 4 open drain mode control"]
pub type P0_04odW<'a, REG> = crate::BitWriter<'a, REG, P0_04od>;
impl<'a, REG> P0_04odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P0.4 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(P0_04od::Normal)
    }
    #[doc = "Open-drain. P0.4 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(P0_04od::OpenDrain)
    }
}
#[doc = "Port 0 pin 5 open drain mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P0_05od {
    #[doc = "0: Normal. P0.5 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P0.5 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<P0_05od> for bool {
    #[inline(always)]
    fn from(variant: P0_05od) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P0_05OD` reader - Port 0 pin 5 open drain mode control"]
pub type P0_05odR = crate::BitReader<P0_05od>;
impl P0_05odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_05od {
        match self.bits {
            false => P0_05od::Normal,
            true => P0_05od::OpenDrain,
        }
    }
    #[doc = "Normal. P0.5 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_05od::Normal
    }
    #[doc = "Open-drain. P0.5 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_05od::OpenDrain
    }
}
#[doc = "Field `P0_05OD` writer - Port 0 pin 5 open drain mode control"]
pub type P0_05odW<'a, REG> = crate::BitWriter<'a, REG, P0_05od>;
impl<'a, REG> P0_05odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P0.5 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(P0_05od::Normal)
    }
    #[doc = "Open-drain. P0.5 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(P0_05od::OpenDrain)
    }
}
#[doc = "Port 0 pin 6 open drain mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P0_06od {
    #[doc = "0: Normal. P0.6 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P0.6 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<P0_06od> for bool {
    #[inline(always)]
    fn from(variant: P0_06od) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P0_06OD` reader - Port 0 pin 6 open drain mode control"]
pub type P0_06odR = crate::BitReader<P0_06od>;
impl P0_06odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_06od {
        match self.bits {
            false => P0_06od::Normal,
            true => P0_06od::OpenDrain,
        }
    }
    #[doc = "Normal. P0.6 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_06od::Normal
    }
    #[doc = "Open-drain. P0.6 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_06od::OpenDrain
    }
}
#[doc = "Field `P0_06OD` writer - Port 0 pin 6 open drain mode control"]
pub type P0_06odW<'a, REG> = crate::BitWriter<'a, REG, P0_06od>;
impl<'a, REG> P0_06odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P0.6 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(P0_06od::Normal)
    }
    #[doc = "Open-drain. P0.6 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(P0_06od::OpenDrain)
    }
}
#[doc = "Port 0 pin 7 open drain mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P0_07od {
    #[doc = "0: Normal. P0.7 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P0.7 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<P0_07od> for bool {
    #[inline(always)]
    fn from(variant: P0_07od) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P0_07OD` reader - Port 0 pin 7 open drain mode control"]
pub type P0_07odR = crate::BitReader<P0_07od>;
impl P0_07odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_07od {
        match self.bits {
            false => P0_07od::Normal,
            true => P0_07od::OpenDrain,
        }
    }
    #[doc = "Normal. P0.7 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_07od::Normal
    }
    #[doc = "Open-drain. P0.7 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_07od::OpenDrain
    }
}
#[doc = "Field `P0_07OD` writer - Port 0 pin 7 open drain mode control"]
pub type P0_07odW<'a, REG> = crate::BitWriter<'a, REG, P0_07od>;
impl<'a, REG> P0_07odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P0.7 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(P0_07od::Normal)
    }
    #[doc = "Open-drain. P0.7 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(P0_07od::OpenDrain)
    }
}
#[doc = "Port 0 pin 8 open drain mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P0_08od {
    #[doc = "0: Normal. P0.8 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P0.8 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<P0_08od> for bool {
    #[inline(always)]
    fn from(variant: P0_08od) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P0_08OD` reader - Port 0 pin 8 open drain mode control"]
pub type P0_08odR = crate::BitReader<P0_08od>;
impl P0_08odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_08od {
        match self.bits {
            false => P0_08od::Normal,
            true => P0_08od::OpenDrain,
        }
    }
    #[doc = "Normal. P0.8 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_08od::Normal
    }
    #[doc = "Open-drain. P0.8 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_08od::OpenDrain
    }
}
#[doc = "Field `P0_08OD` writer - Port 0 pin 8 open drain mode control"]
pub type P0_08odW<'a, REG> = crate::BitWriter<'a, REG, P0_08od>;
impl<'a, REG> P0_08odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P0.8 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(P0_08od::Normal)
    }
    #[doc = "Open-drain. P0.8 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(P0_08od::OpenDrain)
    }
}
#[doc = "Port 0 pin 9 open drain mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P0_09od {
    #[doc = "0: Normal. P0.9 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P0.9 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<P0_09od> for bool {
    #[inline(always)]
    fn from(variant: P0_09od) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P0_09OD` reader - Port 0 pin 9 open drain mode control"]
pub type P0_09odR = crate::BitReader<P0_09od>;
impl P0_09odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_09od {
        match self.bits {
            false => P0_09od::Normal,
            true => P0_09od::OpenDrain,
        }
    }
    #[doc = "Normal. P0.9 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_09od::Normal
    }
    #[doc = "Open-drain. P0.9 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_09od::OpenDrain
    }
}
#[doc = "Field `P0_09OD` writer - Port 0 pin 9 open drain mode control"]
pub type P0_09odW<'a, REG> = crate::BitWriter<'a, REG, P0_09od>;
impl<'a, REG> P0_09odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P0.9 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(P0_09od::Normal)
    }
    #[doc = "Open-drain. P0.9 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(P0_09od::OpenDrain)
    }
}
#[doc = "Port 0 pin 10 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P0_10od {
    #[doc = "0: Normal. P0.10 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P0.10 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<P0_10od> for bool {
    #[inline(always)]
    fn from(variant: P0_10od) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P0_10OD` reader - Port 0 pin 10 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
pub type P0_10odR = crate::BitReader<P0_10od>;
impl P0_10odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_10od {
        match self.bits {
            false => P0_10od::Normal,
            true => P0_10od::OpenDrain,
        }
    }
    #[doc = "Normal. P0.10 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_10od::Normal
    }
    #[doc = "Open-drain. P0.10 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_10od::OpenDrain
    }
}
#[doc = "Field `P0_10OD` writer - Port 0 pin 10 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
pub type P0_10odW<'a, REG> = crate::BitWriter<'a, REG, P0_10od>;
impl<'a, REG> P0_10odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P0.10 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(P0_10od::Normal)
    }
    #[doc = "Open-drain. P0.10 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(P0_10od::OpenDrain)
    }
}
#[doc = "Port 0 pin 11 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P0_11od {
    #[doc = "0: Normal. P0.11 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P0.11 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<P0_11od> for bool {
    #[inline(always)]
    fn from(variant: P0_11od) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P0_11OD` reader - Port 0 pin 11 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
pub type P0_11odR = crate::BitReader<P0_11od>;
impl P0_11odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_11od {
        match self.bits {
            false => P0_11od::Normal,
            true => P0_11od::OpenDrain,
        }
    }
    #[doc = "Normal. P0.11 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_11od::Normal
    }
    #[doc = "Open-drain. P0.11 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_11od::OpenDrain
    }
}
#[doc = "Field `P0_11OD` writer - Port 0 pin 11 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
pub type P0_11odW<'a, REG> = crate::BitWriter<'a, REG, P0_11od>;
impl<'a, REG> P0_11odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P0.11 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(P0_11od::Normal)
    }
    #[doc = "Open-drain. P0.11 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(P0_11od::OpenDrain)
    }
}
#[doc = "Port 0 pin 15 open drain mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P0_15od {
    #[doc = "0: Normal. P0.15 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P0.15 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<P0_15od> for bool {
    #[inline(always)]
    fn from(variant: P0_15od) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P0_15OD` reader - Port 0 pin 15 open drain mode control"]
pub type P0_15odR = crate::BitReader<P0_15od>;
impl P0_15odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_15od {
        match self.bits {
            false => P0_15od::Normal,
            true => P0_15od::OpenDrain,
        }
    }
    #[doc = "Normal. P0.15 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_15od::Normal
    }
    #[doc = "Open-drain. P0.15 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_15od::OpenDrain
    }
}
#[doc = "Field `P0_15OD` writer - Port 0 pin 15 open drain mode control"]
pub type P0_15odW<'a, REG> = crate::BitWriter<'a, REG, P0_15od>;
impl<'a, REG> P0_15odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P0.15 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(P0_15od::Normal)
    }
    #[doc = "Open-drain. P0.15 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(P0_15od::OpenDrain)
    }
}
#[doc = "Port 0 pin 16 open drain mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P0_16od {
    #[doc = "0: Normal. P0.16 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P0.16 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<P0_16od> for bool {
    #[inline(always)]
    fn from(variant: P0_16od) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P0_16OD` reader - Port 0 pin 16 open drain mode control"]
pub type P0_16odR = crate::BitReader<P0_16od>;
impl P0_16odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_16od {
        match self.bits {
            false => P0_16od::Normal,
            true => P0_16od::OpenDrain,
        }
    }
    #[doc = "Normal. P0.16 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_16od::Normal
    }
    #[doc = "Open-drain. P0.16 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_16od::OpenDrain
    }
}
#[doc = "Field `P0_16OD` writer - Port 0 pin 16 open drain mode control"]
pub type P0_16odW<'a, REG> = crate::BitWriter<'a, REG, P0_16od>;
impl<'a, REG> P0_16odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P0.16 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(P0_16od::Normal)
    }
    #[doc = "Open-drain. P0.16 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(P0_16od::OpenDrain)
    }
}
#[doc = "Port 0 pin 17 open drain mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P0_17od {
    #[doc = "0: Normal. P0.17 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P0.17 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<P0_17od> for bool {
    #[inline(always)]
    fn from(variant: P0_17od) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P0_17OD` reader - Port 0 pin 17 open drain mode control"]
pub type P0_17odR = crate::BitReader<P0_17od>;
impl P0_17odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_17od {
        match self.bits {
            false => P0_17od::Normal,
            true => P0_17od::OpenDrain,
        }
    }
    #[doc = "Normal. P0.17 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_17od::Normal
    }
    #[doc = "Open-drain. P0.17 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_17od::OpenDrain
    }
}
#[doc = "Field `P0_17OD` writer - Port 0 pin 17 open drain mode control"]
pub type P0_17odW<'a, REG> = crate::BitWriter<'a, REG, P0_17od>;
impl<'a, REG> P0_17odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P0.17 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(P0_17od::Normal)
    }
    #[doc = "Open-drain. P0.17 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(P0_17od::OpenDrain)
    }
}
#[doc = "Port 0 pin 18 open drain mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P0_18od {
    #[doc = "0: Normal. P0.18 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P0.18 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<P0_18od> for bool {
    #[inline(always)]
    fn from(variant: P0_18od) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P0_18OD` reader - Port 0 pin 18 open drain mode control"]
pub type P0_18odR = crate::BitReader<P0_18od>;
impl P0_18odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_18od {
        match self.bits {
            false => P0_18od::Normal,
            true => P0_18od::OpenDrain,
        }
    }
    #[doc = "Normal. P0.18 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_18od::Normal
    }
    #[doc = "Open-drain. P0.18 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_18od::OpenDrain
    }
}
#[doc = "Field `P0_18OD` writer - Port 0 pin 18 open drain mode control"]
pub type P0_18odW<'a, REG> = crate::BitWriter<'a, REG, P0_18od>;
impl<'a, REG> P0_18odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P0.18 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(P0_18od::Normal)
    }
    #[doc = "Open-drain. P0.18 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(P0_18od::OpenDrain)
    }
}
#[doc = "Port 0 pin 19 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P0_19od {
    #[doc = "0: Normal. P0.19 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P0.19 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<P0_19od> for bool {
    #[inline(always)]
    fn from(variant: P0_19od) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P0_19OD` reader - Port 0 pin 19 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
pub type P0_19odR = crate::BitReader<P0_19od>;
impl P0_19odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_19od {
        match self.bits {
            false => P0_19od::Normal,
            true => P0_19od::OpenDrain,
        }
    }
    #[doc = "Normal. P0.19 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_19od::Normal
    }
    #[doc = "Open-drain. P0.19 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_19od::OpenDrain
    }
}
#[doc = "Field `P0_19OD` writer - Port 0 pin 19 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
pub type P0_19odW<'a, REG> = crate::BitWriter<'a, REG, P0_19od>;
impl<'a, REG> P0_19odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P0.19 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(P0_19od::Normal)
    }
    #[doc = "Open-drain. P0.19 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(P0_19od::OpenDrain)
    }
}
#[doc = "Port 0 pin 20open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P0_20od {
    #[doc = "0: Normal. P0.20 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P0.20 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<P0_20od> for bool {
    #[inline(always)]
    fn from(variant: P0_20od) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P0_20OD` reader - Port 0 pin 20open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
pub type P0_20odR = crate::BitReader<P0_20od>;
impl P0_20odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_20od {
        match self.bits {
            false => P0_20od::Normal,
            true => P0_20od::OpenDrain,
        }
    }
    #[doc = "Normal. P0.20 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_20od::Normal
    }
    #[doc = "Open-drain. P0.20 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_20od::OpenDrain
    }
}
#[doc = "Field `P0_20OD` writer - Port 0 pin 20open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
pub type P0_20odW<'a, REG> = crate::BitWriter<'a, REG, P0_20od>;
impl<'a, REG> P0_20odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P0.20 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(P0_20od::Normal)
    }
    #[doc = "Open-drain. P0.20 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(P0_20od::OpenDrain)
    }
}
#[doc = "Port 0 pin 21 open drain mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P0_21od {
    #[doc = "0: Normal. P0.21 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P0.21 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<P0_21od> for bool {
    #[inline(always)]
    fn from(variant: P0_21od) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P0_21OD` reader - Port 0 pin 21 open drain mode control"]
pub type P0_21odR = crate::BitReader<P0_21od>;
impl P0_21odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_21od {
        match self.bits {
            false => P0_21od::Normal,
            true => P0_21od::OpenDrain,
        }
    }
    #[doc = "Normal. P0.21 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_21od::Normal
    }
    #[doc = "Open-drain. P0.21 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_21od::OpenDrain
    }
}
#[doc = "Field `P0_21OD` writer - Port 0 pin 21 open drain mode control"]
pub type P0_21odW<'a, REG> = crate::BitWriter<'a, REG, P0_21od>;
impl<'a, REG> P0_21odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P0.21 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(P0_21od::Normal)
    }
    #[doc = "Open-drain. P0.21 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(P0_21od::OpenDrain)
    }
}
#[doc = "Port 0 pin 22 open drain mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P0_22od {
    #[doc = "0: Normal. P0.22 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P0.22 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<P0_22od> for bool {
    #[inline(always)]
    fn from(variant: P0_22od) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P0_22OD` reader - Port 0 pin 22 open drain mode control"]
pub type P0_22odR = crate::BitReader<P0_22od>;
impl P0_22odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_22od {
        match self.bits {
            false => P0_22od::Normal,
            true => P0_22od::OpenDrain,
        }
    }
    #[doc = "Normal. P0.22 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_22od::Normal
    }
    #[doc = "Open-drain. P0.22 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_22od::OpenDrain
    }
}
#[doc = "Field `P0_22OD` writer - Port 0 pin 22 open drain mode control"]
pub type P0_22odW<'a, REG> = crate::BitWriter<'a, REG, P0_22od>;
impl<'a, REG> P0_22odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P0.22 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(P0_22od::Normal)
    }
    #[doc = "Open-drain. P0.22 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(P0_22od::OpenDrain)
    }
}
#[doc = "Port 0 pin 23 open drain mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P0_23od {
    #[doc = "0: Normal. P0.23 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P0.23 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<P0_23od> for bool {
    #[inline(always)]
    fn from(variant: P0_23od) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P0_23OD` reader - Port 0 pin 23 open drain mode control"]
pub type P0_23odR = crate::BitReader<P0_23od>;
impl P0_23odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_23od {
        match self.bits {
            false => P0_23od::Normal,
            true => P0_23od::OpenDrain,
        }
    }
    #[doc = "Normal. P0.23 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_23od::Normal
    }
    #[doc = "Open-drain. P0.23 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_23od::OpenDrain
    }
}
#[doc = "Field `P0_23OD` writer - Port 0 pin 23 open drain mode control"]
pub type P0_23odW<'a, REG> = crate::BitWriter<'a, REG, P0_23od>;
impl<'a, REG> P0_23odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P0.23 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(P0_23od::Normal)
    }
    #[doc = "Open-drain. P0.23 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(P0_23od::OpenDrain)
    }
}
#[doc = "Port 0 pin 24open drain mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P0_24od {
    #[doc = "0: Normal. P0.23 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P0.23 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<P0_24od> for bool {
    #[inline(always)]
    fn from(variant: P0_24od) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P0_24OD` reader - Port 0 pin 24open drain mode control"]
pub type P0_24odR = crate::BitReader<P0_24od>;
impl P0_24odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_24od {
        match self.bits {
            false => P0_24od::Normal,
            true => P0_24od::OpenDrain,
        }
    }
    #[doc = "Normal. P0.23 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_24od::Normal
    }
    #[doc = "Open-drain. P0.23 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_24od::OpenDrain
    }
}
#[doc = "Field `P0_24OD` writer - Port 0 pin 24open drain mode control"]
pub type P0_24odW<'a, REG> = crate::BitWriter<'a, REG, P0_24od>;
impl<'a, REG> P0_24odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P0.23 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(P0_24od::Normal)
    }
    #[doc = "Open-drain. P0.23 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(P0_24od::OpenDrain)
    }
}
#[doc = "Port 0 pin 25 open drain mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P0_25od {
    #[doc = "0: Normal. P0.25 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P0.25 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<P0_25od> for bool {
    #[inline(always)]
    fn from(variant: P0_25od) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P0_25OD` reader - Port 0 pin 25 open drain mode control"]
pub type P0_25odR = crate::BitReader<P0_25od>;
impl P0_25odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_25od {
        match self.bits {
            false => P0_25od::Normal,
            true => P0_25od::OpenDrain,
        }
    }
    #[doc = "Normal. P0.25 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_25od::Normal
    }
    #[doc = "Open-drain. P0.25 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_25od::OpenDrain
    }
}
#[doc = "Field `P0_25OD` writer - Port 0 pin 25 open drain mode control"]
pub type P0_25odW<'a, REG> = crate::BitWriter<'a, REG, P0_25od>;
impl<'a, REG> P0_25odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P0.25 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(P0_25od::Normal)
    }
    #[doc = "Open-drain. P0.25 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(P0_25od::OpenDrain)
    }
}
#[doc = "Port 0 pin 26 open drain mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P0_26od {
    #[doc = "0: Normal. P0.26 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P0.26 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<P0_26od> for bool {
    #[inline(always)]
    fn from(variant: P0_26od) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P0_26OD` reader - Port 0 pin 26 open drain mode control"]
pub type P0_26odR = crate::BitReader<P0_26od>;
impl P0_26odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_26od {
        match self.bits {
            false => P0_26od::Normal,
            true => P0_26od::OpenDrain,
        }
    }
    #[doc = "Normal. P0.26 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_26od::Normal
    }
    #[doc = "Open-drain. P0.26 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_26od::OpenDrain
    }
}
#[doc = "Field `P0_26OD` writer - Port 0 pin 26 open drain mode control"]
pub type P0_26odW<'a, REG> = crate::BitWriter<'a, REG, P0_26od>;
impl<'a, REG> P0_26odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P0.26 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(P0_26od::Normal)
    }
    #[doc = "Open-drain. P0.26 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(P0_26od::OpenDrain)
    }
}
#[doc = "Port 0 pin 29 open drain mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P0_29od {
    #[doc = "0: Normal. P0.29 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P0.29 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<P0_29od> for bool {
    #[inline(always)]
    fn from(variant: P0_29od) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P0_29OD` reader - Port 0 pin 29 open drain mode control"]
pub type P0_29odR = crate::BitReader<P0_29od>;
impl P0_29odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_29od {
        match self.bits {
            false => P0_29od::Normal,
            true => P0_29od::OpenDrain,
        }
    }
    #[doc = "Normal. P0.29 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_29od::Normal
    }
    #[doc = "Open-drain. P0.29 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_29od::OpenDrain
    }
}
#[doc = "Field `P0_29OD` writer - Port 0 pin 29 open drain mode control"]
pub type P0_29odW<'a, REG> = crate::BitWriter<'a, REG, P0_29od>;
impl<'a, REG> P0_29odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P0.29 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(P0_29od::Normal)
    }
    #[doc = "Open-drain. P0.29 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(P0_29od::OpenDrain)
    }
}
#[doc = "Port 0 pin 30 open drain mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P0_30od {
    #[doc = "0: Normal. P0.30 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P0.30 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<P0_30od> for bool {
    #[inline(always)]
    fn from(variant: P0_30od) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P0_30OD` reader - Port 0 pin 30 open drain mode control"]
pub type P0_30odR = crate::BitReader<P0_30od>;
impl P0_30odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_30od {
        match self.bits {
            false => P0_30od::Normal,
            true => P0_30od::OpenDrain,
        }
    }
    #[doc = "Normal. P0.30 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_30od::Normal
    }
    #[doc = "Open-drain. P0.30 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_30od::OpenDrain
    }
}
#[doc = "Field `P0_30OD` writer - Port 0 pin 30 open drain mode control"]
pub type P0_30odW<'a, REG> = crate::BitWriter<'a, REG, P0_30od>;
impl<'a, REG> P0_30odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P0.30 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(P0_30od::Normal)
    }
    #[doc = "Open-drain. P0.30 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(P0_30od::OpenDrain)
    }
}
impl R {
    #[doc = "Bit 0 - Port 0 pin 0 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
    #[inline(always)]
    pub fn p0_00od(&self) -> P0_00odR {
        P0_00odR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port 0 pin 1 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
    #[inline(always)]
    pub fn p0_01od(&self) -> P0_01odR {
        P0_01odR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port 0 pin 2 open drain mode control"]
    #[inline(always)]
    pub fn p0_02od(&self) -> P0_02odR {
        P0_02odR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port 0 pin 3 open drain mode control"]
    #[inline(always)]
    pub fn p0_03od(&self) -> P0_03odR {
        P0_03odR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port 0 pin 4 open drain mode control"]
    #[inline(always)]
    pub fn p0_04od(&self) -> P0_04odR {
        P0_04odR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port 0 pin 5 open drain mode control"]
    #[inline(always)]
    pub fn p0_05od(&self) -> P0_05odR {
        P0_05odR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port 0 pin 6 open drain mode control"]
    #[inline(always)]
    pub fn p0_06od(&self) -> P0_06odR {
        P0_06odR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port 0 pin 7 open drain mode control"]
    #[inline(always)]
    pub fn p0_07od(&self) -> P0_07odR {
        P0_07odR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port 0 pin 8 open drain mode control"]
    #[inline(always)]
    pub fn p0_08od(&self) -> P0_08odR {
        P0_08odR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port 0 pin 9 open drain mode control"]
    #[inline(always)]
    pub fn p0_09od(&self) -> P0_09odR {
        P0_09odR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port 0 pin 10 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
    #[inline(always)]
    pub fn p0_10od(&self) -> P0_10odR {
        P0_10odR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port 0 pin 11 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
    #[inline(always)]
    pub fn p0_11od(&self) -> P0_11odR {
        P0_11odR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - Port 0 pin 15 open drain mode control"]
    #[inline(always)]
    pub fn p0_15od(&self) -> P0_15odR {
        P0_15odR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Port 0 pin 16 open drain mode control"]
    #[inline(always)]
    pub fn p0_16od(&self) -> P0_16odR {
        P0_16odR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Port 0 pin 17 open drain mode control"]
    #[inline(always)]
    pub fn p0_17od(&self) -> P0_17odR {
        P0_17odR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Port 0 pin 18 open drain mode control"]
    #[inline(always)]
    pub fn p0_18od(&self) -> P0_18odR {
        P0_18odR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Port 0 pin 19 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
    #[inline(always)]
    pub fn p0_19od(&self) -> P0_19odR {
        P0_19odR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Port 0 pin 20open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
    #[inline(always)]
    pub fn p0_20od(&self) -> P0_20odR {
        P0_20odR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Port 0 pin 21 open drain mode control"]
    #[inline(always)]
    pub fn p0_21od(&self) -> P0_21odR {
        P0_21odR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Port 0 pin 22 open drain mode control"]
    #[inline(always)]
    pub fn p0_22od(&self) -> P0_22odR {
        P0_22odR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Port 0 pin 23 open drain mode control"]
    #[inline(always)]
    pub fn p0_23od(&self) -> P0_23odR {
        P0_23odR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Port 0 pin 24open drain mode control"]
    #[inline(always)]
    pub fn p0_24od(&self) -> P0_24odR {
        P0_24odR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Port 0 pin 25 open drain mode control"]
    #[inline(always)]
    pub fn p0_25od(&self) -> P0_25odR {
        P0_25odR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Port 0 pin 26 open drain mode control"]
    #[inline(always)]
    pub fn p0_26od(&self) -> P0_26odR {
        P0_26odR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 29 - Port 0 pin 29 open drain mode control"]
    #[inline(always)]
    pub fn p0_29od(&self) -> P0_29odR {
        P0_29odR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Port 0 pin 30 open drain mode control"]
    #[inline(always)]
    pub fn p0_30od(&self) -> P0_30odR {
        P0_30odR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port 0 pin 0 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
    #[inline(always)]
    #[must_use]
    pub fn p0_00od(&mut self) -> P0_00odW<PinmodeOd0Spec> {
        P0_00odW::new(self, 0)
    }
    #[doc = "Bit 1 - Port 0 pin 1 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
    #[inline(always)]
    #[must_use]
    pub fn p0_01od(&mut self) -> P0_01odW<PinmodeOd0Spec> {
        P0_01odW::new(self, 1)
    }
    #[doc = "Bit 2 - Port 0 pin 2 open drain mode control"]
    #[inline(always)]
    #[must_use]
    pub fn p0_02od(&mut self) -> P0_02odW<PinmodeOd0Spec> {
        P0_02odW::new(self, 2)
    }
    #[doc = "Bit 3 - Port 0 pin 3 open drain mode control"]
    #[inline(always)]
    #[must_use]
    pub fn p0_03od(&mut self) -> P0_03odW<PinmodeOd0Spec> {
        P0_03odW::new(self, 3)
    }
    #[doc = "Bit 4 - Port 0 pin 4 open drain mode control"]
    #[inline(always)]
    #[must_use]
    pub fn p0_04od(&mut self) -> P0_04odW<PinmodeOd0Spec> {
        P0_04odW::new(self, 4)
    }
    #[doc = "Bit 5 - Port 0 pin 5 open drain mode control"]
    #[inline(always)]
    #[must_use]
    pub fn p0_05od(&mut self) -> P0_05odW<PinmodeOd0Spec> {
        P0_05odW::new(self, 5)
    }
    #[doc = "Bit 6 - Port 0 pin 6 open drain mode control"]
    #[inline(always)]
    #[must_use]
    pub fn p0_06od(&mut self) -> P0_06odW<PinmodeOd0Spec> {
        P0_06odW::new(self, 6)
    }
    #[doc = "Bit 7 - Port 0 pin 7 open drain mode control"]
    #[inline(always)]
    #[must_use]
    pub fn p0_07od(&mut self) -> P0_07odW<PinmodeOd0Spec> {
        P0_07odW::new(self, 7)
    }
    #[doc = "Bit 8 - Port 0 pin 8 open drain mode control"]
    #[inline(always)]
    #[must_use]
    pub fn p0_08od(&mut self) -> P0_08odW<PinmodeOd0Spec> {
        P0_08odW::new(self, 8)
    }
    #[doc = "Bit 9 - Port 0 pin 9 open drain mode control"]
    #[inline(always)]
    #[must_use]
    pub fn p0_09od(&mut self) -> P0_09odW<PinmodeOd0Spec> {
        P0_09odW::new(self, 9)
    }
    #[doc = "Bit 10 - Port 0 pin 10 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
    #[inline(always)]
    #[must_use]
    pub fn p0_10od(&mut self) -> P0_10odW<PinmodeOd0Spec> {
        P0_10odW::new(self, 10)
    }
    #[doc = "Bit 11 - Port 0 pin 11 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
    #[inline(always)]
    #[must_use]
    pub fn p0_11od(&mut self) -> P0_11odW<PinmodeOd0Spec> {
        P0_11odW::new(self, 11)
    }
    #[doc = "Bit 15 - Port 0 pin 15 open drain mode control"]
    #[inline(always)]
    #[must_use]
    pub fn p0_15od(&mut self) -> P0_15odW<PinmodeOd0Spec> {
        P0_15odW::new(self, 15)
    }
    #[doc = "Bit 16 - Port 0 pin 16 open drain mode control"]
    #[inline(always)]
    #[must_use]
    pub fn p0_16od(&mut self) -> P0_16odW<PinmodeOd0Spec> {
        P0_16odW::new(self, 16)
    }
    #[doc = "Bit 17 - Port 0 pin 17 open drain mode control"]
    #[inline(always)]
    #[must_use]
    pub fn p0_17od(&mut self) -> P0_17odW<PinmodeOd0Spec> {
        P0_17odW::new(self, 17)
    }
    #[doc = "Bit 18 - Port 0 pin 18 open drain mode control"]
    #[inline(always)]
    #[must_use]
    pub fn p0_18od(&mut self) -> P0_18odW<PinmodeOd0Spec> {
        P0_18odW::new(self, 18)
    }
    #[doc = "Bit 19 - Port 0 pin 19 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
    #[inline(always)]
    #[must_use]
    pub fn p0_19od(&mut self) -> P0_19odW<PinmodeOd0Spec> {
        P0_19odW::new(self, 19)
    }
    #[doc = "Bit 20 - Port 0 pin 20open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
    #[inline(always)]
    #[must_use]
    pub fn p0_20od(&mut self) -> P0_20odW<PinmodeOd0Spec> {
        P0_20odW::new(self, 20)
    }
    #[doc = "Bit 21 - Port 0 pin 21 open drain mode control"]
    #[inline(always)]
    #[must_use]
    pub fn p0_21od(&mut self) -> P0_21odW<PinmodeOd0Spec> {
        P0_21odW::new(self, 21)
    }
    #[doc = "Bit 22 - Port 0 pin 22 open drain mode control"]
    #[inline(always)]
    #[must_use]
    pub fn p0_22od(&mut self) -> P0_22odW<PinmodeOd0Spec> {
        P0_22odW::new(self, 22)
    }
    #[doc = "Bit 23 - Port 0 pin 23 open drain mode control"]
    #[inline(always)]
    #[must_use]
    pub fn p0_23od(&mut self) -> P0_23odW<PinmodeOd0Spec> {
        P0_23odW::new(self, 23)
    }
    #[doc = "Bit 24 - Port 0 pin 24open drain mode control"]
    #[inline(always)]
    #[must_use]
    pub fn p0_24od(&mut self) -> P0_24odW<PinmodeOd0Spec> {
        P0_24odW::new(self, 24)
    }
    #[doc = "Bit 25 - Port 0 pin 25 open drain mode control"]
    #[inline(always)]
    #[must_use]
    pub fn p0_25od(&mut self) -> P0_25odW<PinmodeOd0Spec> {
        P0_25odW::new(self, 25)
    }
    #[doc = "Bit 26 - Port 0 pin 26 open drain mode control"]
    #[inline(always)]
    #[must_use]
    pub fn p0_26od(&mut self) -> P0_26odW<PinmodeOd0Spec> {
        P0_26odW::new(self, 26)
    }
    #[doc = "Bit 29 - Port 0 pin 29 open drain mode control"]
    #[inline(always)]
    #[must_use]
    pub fn p0_29od(&mut self) -> P0_29odW<PinmodeOd0Spec> {
        P0_29odW::new(self, 29)
    }
    #[doc = "Bit 30 - Port 0 pin 30 open drain mode control"]
    #[inline(always)]
    #[must_use]
    pub fn p0_30od(&mut self) -> P0_30odW<PinmodeOd0Spec> {
        P0_30odW::new(self, 30)
    }
}
#[doc = "Open drain mode control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pinmode_od0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pinmode_od0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PinmodeOd0Spec;
impl crate::RegisterSpec for PinmodeOd0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pinmode_od0::R`](R) reader structure"]
impl crate::Readable for PinmodeOd0Spec {}
#[doc = "`write(|w| ..)` method takes [`pinmode_od0::W`](W) writer structure"]
impl crate::Writable for PinmodeOd0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PINMODE_OD0 to value 0"]
impl crate::Resettable for PinmodeOd0Spec {
    const RESET_VALUE: u32 = 0;
}
