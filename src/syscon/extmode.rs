#[doc = "Register `EXTMODE` reader"]
pub type R = crate::R<ExtmodeSpec>;
#[doc = "Register `EXTMODE` writer"]
pub type W = crate::W<ExtmodeSpec>;
#[doc = "External interrupt 0 EINT0 mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Extmode0 {
    #[doc = "0: Level-sensitive. Level-sensitivity is selected for EINT0."]
    LevelSensitive = 0,
    #[doc = "1: Edge-sensitive. EINT0 is edge sensitive."]
    EdgeSensitive = 1,
}
impl From<Extmode0> for bool {
    #[inline(always)]
    fn from(variant: Extmode0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTMODE0` reader - External interrupt 0 EINT0 mode."]
pub type Extmode0R = crate::BitReader<Extmode0>;
impl Extmode0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extmode0 {
        match self.bits {
            false => Extmode0::LevelSensitive,
            true => Extmode0::EdgeSensitive,
        }
    }
    #[doc = "Level-sensitive. Level-sensitivity is selected for EINT0."]
    #[inline(always)]
    pub fn is_level_sensitive(&self) -> bool {
        *self == Extmode0::LevelSensitive
    }
    #[doc = "Edge-sensitive. EINT0 is edge sensitive."]
    #[inline(always)]
    pub fn is_edge_sensitive(&self) -> bool {
        *self == Extmode0::EdgeSensitive
    }
}
#[doc = "Field `EXTMODE0` writer - External interrupt 0 EINT0 mode."]
pub type Extmode0W<'a, REG> = crate::BitWriter<'a, REG, Extmode0>;
impl<'a, REG> Extmode0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level-sensitive. Level-sensitivity is selected for EINT0."]
    #[inline(always)]
    pub fn level_sensitive(self) -> &'a mut crate::W<REG> {
        self.variant(Extmode0::LevelSensitive)
    }
    #[doc = "Edge-sensitive. EINT0 is edge sensitive."]
    #[inline(always)]
    pub fn edge_sensitive(self) -> &'a mut crate::W<REG> {
        self.variant(Extmode0::EdgeSensitive)
    }
}
#[doc = "External interrupt 1 EINT1 mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Extmode1 {
    #[doc = "0: Level-sensitive. Level-sensitivity is selected for EINT1."]
    LevelSensitive = 0,
    #[doc = "1: Edge-sensitive. EINT1 is edge sensitive."]
    EdgeSensitive = 1,
}
impl From<Extmode1> for bool {
    #[inline(always)]
    fn from(variant: Extmode1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTMODE1` reader - External interrupt 1 EINT1 mode."]
pub type Extmode1R = crate::BitReader<Extmode1>;
impl Extmode1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extmode1 {
        match self.bits {
            false => Extmode1::LevelSensitive,
            true => Extmode1::EdgeSensitive,
        }
    }
    #[doc = "Level-sensitive. Level-sensitivity is selected for EINT1."]
    #[inline(always)]
    pub fn is_level_sensitive(&self) -> bool {
        *self == Extmode1::LevelSensitive
    }
    #[doc = "Edge-sensitive. EINT1 is edge sensitive."]
    #[inline(always)]
    pub fn is_edge_sensitive(&self) -> bool {
        *self == Extmode1::EdgeSensitive
    }
}
#[doc = "Field `EXTMODE1` writer - External interrupt 1 EINT1 mode."]
pub type Extmode1W<'a, REG> = crate::BitWriter<'a, REG, Extmode1>;
impl<'a, REG> Extmode1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level-sensitive. Level-sensitivity is selected for EINT1."]
    #[inline(always)]
    pub fn level_sensitive(self) -> &'a mut crate::W<REG> {
        self.variant(Extmode1::LevelSensitive)
    }
    #[doc = "Edge-sensitive. EINT1 is edge sensitive."]
    #[inline(always)]
    pub fn edge_sensitive(self) -> &'a mut crate::W<REG> {
        self.variant(Extmode1::EdgeSensitive)
    }
}
#[doc = "External interrupt 2 EINT2 mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Extmode2 {
    #[doc = "0: Level-sensitive. Level-sensitivity is selected for EINT2."]
    LevelSensitive = 0,
    #[doc = "1: Edge-sensitive. EINT2 is edge sensitive."]
    EdgeSensitive = 1,
}
impl From<Extmode2> for bool {
    #[inline(always)]
    fn from(variant: Extmode2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTMODE2` reader - External interrupt 2 EINT2 mode."]
pub type Extmode2R = crate::BitReader<Extmode2>;
impl Extmode2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extmode2 {
        match self.bits {
            false => Extmode2::LevelSensitive,
            true => Extmode2::EdgeSensitive,
        }
    }
    #[doc = "Level-sensitive. Level-sensitivity is selected for EINT2."]
    #[inline(always)]
    pub fn is_level_sensitive(&self) -> bool {
        *self == Extmode2::LevelSensitive
    }
    #[doc = "Edge-sensitive. EINT2 is edge sensitive."]
    #[inline(always)]
    pub fn is_edge_sensitive(&self) -> bool {
        *self == Extmode2::EdgeSensitive
    }
}
#[doc = "Field `EXTMODE2` writer - External interrupt 2 EINT2 mode."]
pub type Extmode2W<'a, REG> = crate::BitWriter<'a, REG, Extmode2>;
impl<'a, REG> Extmode2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level-sensitive. Level-sensitivity is selected for EINT2."]
    #[inline(always)]
    pub fn level_sensitive(self) -> &'a mut crate::W<REG> {
        self.variant(Extmode2::LevelSensitive)
    }
    #[doc = "Edge-sensitive. EINT2 is edge sensitive."]
    #[inline(always)]
    pub fn edge_sensitive(self) -> &'a mut crate::W<REG> {
        self.variant(Extmode2::EdgeSensitive)
    }
}
#[doc = "External interrupt 3 EINT3 mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Extmode3 {
    #[doc = "0: Level-sensitive. Level-sensitivity is selected for EINT3."]
    LevelSensitive = 0,
    #[doc = "1: Edge-sensitive. EINT3 is edge sensitive."]
    EdgeSensitive = 1,
}
impl From<Extmode3> for bool {
    #[inline(always)]
    fn from(variant: Extmode3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTMODE3` reader - External interrupt 3 EINT3 mode."]
pub type Extmode3R = crate::BitReader<Extmode3>;
impl Extmode3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extmode3 {
        match self.bits {
            false => Extmode3::LevelSensitive,
            true => Extmode3::EdgeSensitive,
        }
    }
    #[doc = "Level-sensitive. Level-sensitivity is selected for EINT3."]
    #[inline(always)]
    pub fn is_level_sensitive(&self) -> bool {
        *self == Extmode3::LevelSensitive
    }
    #[doc = "Edge-sensitive. EINT3 is edge sensitive."]
    #[inline(always)]
    pub fn is_edge_sensitive(&self) -> bool {
        *self == Extmode3::EdgeSensitive
    }
}
#[doc = "Field `EXTMODE3` writer - External interrupt 3 EINT3 mode."]
pub type Extmode3W<'a, REG> = crate::BitWriter<'a, REG, Extmode3>;
impl<'a, REG> Extmode3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level-sensitive. Level-sensitivity is selected for EINT3."]
    #[inline(always)]
    pub fn level_sensitive(self) -> &'a mut crate::W<REG> {
        self.variant(Extmode3::LevelSensitive)
    }
    #[doc = "Edge-sensitive. EINT3 is edge sensitive."]
    #[inline(always)]
    pub fn edge_sensitive(self) -> &'a mut crate::W<REG> {
        self.variant(Extmode3::EdgeSensitive)
    }
}
impl R {
    #[doc = "Bit 0 - External interrupt 0 EINT0 mode."]
    #[inline(always)]
    pub fn extmode0(&self) -> Extmode0R {
        Extmode0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External interrupt 1 EINT1 mode."]
    #[inline(always)]
    pub fn extmode1(&self) -> Extmode1R {
        Extmode1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External interrupt 2 EINT2 mode."]
    #[inline(always)]
    pub fn extmode2(&self) -> Extmode2R {
        Extmode2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - External interrupt 3 EINT3 mode."]
    #[inline(always)]
    pub fn extmode3(&self) -> Extmode3R {
        Extmode3R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - External interrupt 0 EINT0 mode."]
    #[inline(always)]
    #[must_use]
    pub fn extmode0(&mut self) -> Extmode0W<ExtmodeSpec> {
        Extmode0W::new(self, 0)
    }
    #[doc = "Bit 1 - External interrupt 1 EINT1 mode."]
    #[inline(always)]
    #[must_use]
    pub fn extmode1(&mut self) -> Extmode1W<ExtmodeSpec> {
        Extmode1W::new(self, 1)
    }
    #[doc = "Bit 2 - External interrupt 2 EINT2 mode."]
    #[inline(always)]
    #[must_use]
    pub fn extmode2(&mut self) -> Extmode2W<ExtmodeSpec> {
        Extmode2W::new(self, 2)
    }
    #[doc = "Bit 3 - External interrupt 3 EINT3 mode."]
    #[inline(always)]
    #[must_use]
    pub fn extmode3(&mut self) -> Extmode3W<ExtmodeSpec> {
        Extmode3W::new(self, 3)
    }
}
#[doc = "External Interrupt Mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`extmode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extmode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtmodeSpec;
impl crate::RegisterSpec for ExtmodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extmode::R`](R) reader structure"]
impl crate::Readable for ExtmodeSpec {}
#[doc = "`write(|w| ..)` method takes [`extmode::W`](W) writer structure"]
impl crate::Writable for ExtmodeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTMODE to value 0"]
impl crate::Resettable for ExtmodeSpec {
    const RESET_VALUE: u32 = 0;
}
