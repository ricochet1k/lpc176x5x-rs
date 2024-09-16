#[doc = "Register `I2CPADCFG` reader"]
pub type R = crate::R<I2cpadcfgSpec>;
#[doc = "Register `I2CPADCFG` writer"]
pub type W = crate::W<I2cpadcfgSpec>;
#[doc = "Drive mode control for the SDA0 pin, P0.27.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdadrv0 {
    #[doc = "0: Standard. The SDA0 pin is in the standard drive mode."]
    Standard = 0,
    #[doc = "1: Fast-mode plus. The SDA0 pin is in Fast Mode Plus drive mode."]
    FastModePlus = 1,
}
impl From<Sdadrv0> for bool {
    #[inline(always)]
    fn from(variant: Sdadrv0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDADRV0` reader - Drive mode control for the SDA0 pin, P0.27."]
pub type Sdadrv0R = crate::BitReader<Sdadrv0>;
impl Sdadrv0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdadrv0 {
        match self.bits {
            false => Sdadrv0::Standard,
            true => Sdadrv0::FastModePlus,
        }
    }
    #[doc = "Standard. The SDA0 pin is in the standard drive mode."]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == Sdadrv0::Standard
    }
    #[doc = "Fast-mode plus. The SDA0 pin is in Fast Mode Plus drive mode."]
    #[inline(always)]
    pub fn is_fast_mode_plus(&self) -> bool {
        *self == Sdadrv0::FastModePlus
    }
}
#[doc = "Field `SDADRV0` writer - Drive mode control for the SDA0 pin, P0.27."]
pub type Sdadrv0W<'a, REG> = crate::BitWriter<'a, REG, Sdadrv0>;
impl<'a, REG> Sdadrv0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Standard. The SDA0 pin is in the standard drive mode."]
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(Sdadrv0::Standard)
    }
    #[doc = "Fast-mode plus. The SDA0 pin is in Fast Mode Plus drive mode."]
    #[inline(always)]
    pub fn fast_mode_plus(self) -> &'a mut crate::W<REG> {
        self.variant(Sdadrv0::FastModePlus)
    }
}
#[doc = "I 2C filter mode control for the SDA0 pin, P0.27.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdai2c0 {
    #[doc = "0: Enabled. The SDA0 pin has I2C glitch filtering and slew rate control enabled."]
    Enabled = 0,
    #[doc = "1: Disabled. The SDA0 pin has I2C glitch filtering and slew rate control disabled."]
    Disabled = 1,
}
impl From<Sdai2c0> for bool {
    #[inline(always)]
    fn from(variant: Sdai2c0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDAI2C0` reader - I 2C filter mode control for the SDA0 pin, P0.27."]
pub type Sdai2c0R = crate::BitReader<Sdai2c0>;
impl Sdai2c0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdai2c0 {
        match self.bits {
            false => Sdai2c0::Enabled,
            true => Sdai2c0::Disabled,
        }
    }
    #[doc = "Enabled. The SDA0 pin has I2C glitch filtering and slew rate control enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Sdai2c0::Enabled
    }
    #[doc = "Disabled. The SDA0 pin has I2C glitch filtering and slew rate control disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Sdai2c0::Disabled
    }
}
#[doc = "Field `SDAI2C0` writer - I 2C filter mode control for the SDA0 pin, P0.27."]
pub type Sdai2c0W<'a, REG> = crate::BitWriter<'a, REG, Sdai2c0>;
impl<'a, REG> Sdai2c0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enabled. The SDA0 pin has I2C glitch filtering and slew rate control enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sdai2c0::Enabled)
    }
    #[doc = "Disabled. The SDA0 pin has I2C glitch filtering and slew rate control disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sdai2c0::Disabled)
    }
}
#[doc = "Drive mode control for the SCL0 pin, P0.28.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scldrv0 {
    #[doc = "0: Standard. The SCL0 pin is in the standard drive mode."]
    Standard = 0,
    #[doc = "1: Fast-mode plus. The SCL0 pin is in Fast Mode Plus drive mode."]
    FastModePlus = 1,
}
impl From<Scldrv0> for bool {
    #[inline(always)]
    fn from(variant: Scldrv0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCLDRV0` reader - Drive mode control for the SCL0 pin, P0.28."]
pub type Scldrv0R = crate::BitReader<Scldrv0>;
impl Scldrv0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Scldrv0 {
        match self.bits {
            false => Scldrv0::Standard,
            true => Scldrv0::FastModePlus,
        }
    }
    #[doc = "Standard. The SCL0 pin is in the standard drive mode."]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == Scldrv0::Standard
    }
    #[doc = "Fast-mode plus. The SCL0 pin is in Fast Mode Plus drive mode."]
    #[inline(always)]
    pub fn is_fast_mode_plus(&self) -> bool {
        *self == Scldrv0::FastModePlus
    }
}
#[doc = "Field `SCLDRV0` writer - Drive mode control for the SCL0 pin, P0.28."]
pub type Scldrv0W<'a, REG> = crate::BitWriter<'a, REG, Scldrv0>;
impl<'a, REG> Scldrv0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Standard. The SCL0 pin is in the standard drive mode."]
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(Scldrv0::Standard)
    }
    #[doc = "Fast-mode plus. The SCL0 pin is in Fast Mode Plus drive mode."]
    #[inline(always)]
    pub fn fast_mode_plus(self) -> &'a mut crate::W<REG> {
        self.variant(Scldrv0::FastModePlus)
    }
}
#[doc = "I 2C filter mode control for the SCL0 pin, P0.28.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scli2c0 {
    #[doc = "0: Enabled. The SCL0 pin has I2C glitch filtering and slew rate control enabled."]
    Enabled = 0,
    #[doc = "1: Disabled. The SCL0 pin has I2C glitch filtering and slew rate control disabled."]
    Disabled = 1,
}
impl From<Scli2c0> for bool {
    #[inline(always)]
    fn from(variant: Scli2c0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCLI2C0` reader - I 2C filter mode control for the SCL0 pin, P0.28."]
pub type Scli2c0R = crate::BitReader<Scli2c0>;
impl Scli2c0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Scli2c0 {
        match self.bits {
            false => Scli2c0::Enabled,
            true => Scli2c0::Disabled,
        }
    }
    #[doc = "Enabled. The SCL0 pin has I2C glitch filtering and slew rate control enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Scli2c0::Enabled
    }
    #[doc = "Disabled. The SCL0 pin has I2C glitch filtering and slew rate control disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Scli2c0::Disabled
    }
}
#[doc = "Field `SCLI2C0` writer - I 2C filter mode control for the SCL0 pin, P0.28."]
pub type Scli2c0W<'a, REG> = crate::BitWriter<'a, REG, Scli2c0>;
impl<'a, REG> Scli2c0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enabled. The SCL0 pin has I2C glitch filtering and slew rate control enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Scli2c0::Enabled)
    }
    #[doc = "Disabled. The SCL0 pin has I2C glitch filtering and slew rate control disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Scli2c0::Disabled)
    }
}
impl R {
    #[doc = "Bit 0 - Drive mode control for the SDA0 pin, P0.27."]
    #[inline(always)]
    pub fn sdadrv0(&self) -> Sdadrv0R {
        Sdadrv0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I 2C filter mode control for the SDA0 pin, P0.27."]
    #[inline(always)]
    pub fn sdai2c0(&self) -> Sdai2c0R {
        Sdai2c0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Drive mode control for the SCL0 pin, P0.28."]
    #[inline(always)]
    pub fn scldrv0(&self) -> Scldrv0R {
        Scldrv0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I 2C filter mode control for the SCL0 pin, P0.28."]
    #[inline(always)]
    pub fn scli2c0(&self) -> Scli2c0R {
        Scli2c0R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Drive mode control for the SDA0 pin, P0.27."]
    #[inline(always)]
    #[must_use]
    pub fn sdadrv0(&mut self) -> Sdadrv0W<I2cpadcfgSpec> {
        Sdadrv0W::new(self, 0)
    }
    #[doc = "Bit 1 - I 2C filter mode control for the SDA0 pin, P0.27."]
    #[inline(always)]
    #[must_use]
    pub fn sdai2c0(&mut self) -> Sdai2c0W<I2cpadcfgSpec> {
        Sdai2c0W::new(self, 1)
    }
    #[doc = "Bit 2 - Drive mode control for the SCL0 pin, P0.28."]
    #[inline(always)]
    #[must_use]
    pub fn scldrv0(&mut self) -> Scldrv0W<I2cpadcfgSpec> {
        Scldrv0W::new(self, 2)
    }
    #[doc = "Bit 3 - I 2C filter mode control for the SCL0 pin, P0.28."]
    #[inline(always)]
    #[must_use]
    pub fn scli2c0(&mut self) -> Scli2c0W<I2cpadcfgSpec> {
        Scli2c0W::new(self, 3)
    }
}
#[doc = "I2C Pin Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cpadcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cpadcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cpadcfgSpec;
impl crate::RegisterSpec for I2cpadcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2cpadcfg::R`](R) reader structure"]
impl crate::Readable for I2cpadcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`i2cpadcfg::W`](W) writer structure"]
impl crate::Writable for I2cpadcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2CPADCFG to value 0"]
impl crate::Resettable for I2cpadcfgSpec {
    const RESET_VALUE: u32 = 0;
}
