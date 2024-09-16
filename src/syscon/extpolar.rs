#[doc = "Register `EXTPOLAR` reader"]
pub type R = crate::R<ExtpolarSpec>;
#[doc = "Register `EXTPOLAR` writer"]
pub type W = crate::W<ExtpolarSpec>;
#[doc = "External interrupt 0 EINT0 polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Extpolar0 {
    #[doc = "0: Falling edge. EINT0 is low-active or falling-edge sensitive (depending on EXTMODE0)."]
    FallingEdge = 0,
    #[doc = "1: Rising edge. EINT0 is high-active or rising-edge sensitive (depending on EXTMODE0)."]
    RisingEdge = 1,
}
impl From<Extpolar0> for bool {
    #[inline(always)]
    fn from(variant: Extpolar0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTPOLAR0` reader - External interrupt 0 EINT0 polarity."]
pub type Extpolar0R = crate::BitReader<Extpolar0>;
impl Extpolar0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extpolar0 {
        match self.bits {
            false => Extpolar0::FallingEdge,
            true => Extpolar0::RisingEdge,
        }
    }
    #[doc = "Falling edge. EINT0 is low-active or falling-edge sensitive (depending on EXTMODE0)."]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == Extpolar0::FallingEdge
    }
    #[doc = "Rising edge. EINT0 is high-active or rising-edge sensitive (depending on EXTMODE0)."]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == Extpolar0::RisingEdge
    }
}
#[doc = "Field `EXTPOLAR0` writer - External interrupt 0 EINT0 polarity."]
pub type Extpolar0W<'a, REG> = crate::BitWriter<'a, REG, Extpolar0>;
impl<'a, REG> Extpolar0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling edge. EINT0 is low-active or falling-edge sensitive (depending on EXTMODE0)."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Extpolar0::FallingEdge)
    }
    #[doc = "Rising edge. EINT0 is high-active or rising-edge sensitive (depending on EXTMODE0)."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Extpolar0::RisingEdge)
    }
}
#[doc = "External interrupt 1 EINT1 polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Extpolar1 {
    #[doc = "0: Falling edge. EINT1 is low-active or falling-edge sensitive (depending on EXTMODE1)."]
    FallingEdge = 0,
    #[doc = "1: Rising edge. EINT1 is high-active or rising-edge sensitive (depending on EXTMODE1)."]
    RisingEdge = 1,
}
impl From<Extpolar1> for bool {
    #[inline(always)]
    fn from(variant: Extpolar1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTPOLAR1` reader - External interrupt 1 EINT1 polarity."]
pub type Extpolar1R = crate::BitReader<Extpolar1>;
impl Extpolar1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extpolar1 {
        match self.bits {
            false => Extpolar1::FallingEdge,
            true => Extpolar1::RisingEdge,
        }
    }
    #[doc = "Falling edge. EINT1 is low-active or falling-edge sensitive (depending on EXTMODE1)."]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == Extpolar1::FallingEdge
    }
    #[doc = "Rising edge. EINT1 is high-active or rising-edge sensitive (depending on EXTMODE1)."]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == Extpolar1::RisingEdge
    }
}
#[doc = "Field `EXTPOLAR1` writer - External interrupt 1 EINT1 polarity."]
pub type Extpolar1W<'a, REG> = crate::BitWriter<'a, REG, Extpolar1>;
impl<'a, REG> Extpolar1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling edge. EINT1 is low-active or falling-edge sensitive (depending on EXTMODE1)."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Extpolar1::FallingEdge)
    }
    #[doc = "Rising edge. EINT1 is high-active or rising-edge sensitive (depending on EXTMODE1)."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Extpolar1::RisingEdge)
    }
}
#[doc = "External interrupt 2 EINT2 polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Extpolar2 {
    #[doc = "0: Falling edge. EINT2 is low-active or falling-edge sensitive (depending on EXTMODE2)."]
    FallingEdge = 0,
    #[doc = "1: Rising edge. EINT2 is high-active or rising-edge sensitive (depending on EXTMODE2)."]
    RisingEdge = 1,
}
impl From<Extpolar2> for bool {
    #[inline(always)]
    fn from(variant: Extpolar2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTPOLAR2` reader - External interrupt 2 EINT2 polarity."]
pub type Extpolar2R = crate::BitReader<Extpolar2>;
impl Extpolar2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extpolar2 {
        match self.bits {
            false => Extpolar2::FallingEdge,
            true => Extpolar2::RisingEdge,
        }
    }
    #[doc = "Falling edge. EINT2 is low-active or falling-edge sensitive (depending on EXTMODE2)."]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == Extpolar2::FallingEdge
    }
    #[doc = "Rising edge. EINT2 is high-active or rising-edge sensitive (depending on EXTMODE2)."]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == Extpolar2::RisingEdge
    }
}
#[doc = "Field `EXTPOLAR2` writer - External interrupt 2 EINT2 polarity."]
pub type Extpolar2W<'a, REG> = crate::BitWriter<'a, REG, Extpolar2>;
impl<'a, REG> Extpolar2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling edge. EINT2 is low-active or falling-edge sensitive (depending on EXTMODE2)."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Extpolar2::FallingEdge)
    }
    #[doc = "Rising edge. EINT2 is high-active or rising-edge sensitive (depending on EXTMODE2)."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Extpolar2::RisingEdge)
    }
}
#[doc = "External interrupt 3 EINT3 polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Extpolar3 {
    #[doc = "0: Falling edge. EINT3 is low-active or falling-edge sensitive (depending on EXTMODE3)."]
    FallingEdge = 0,
    #[doc = "1: Rising edge. EINT3 is high-active or rising-edge sensitive (depending on EXTMODE3)."]
    RisingEdge = 1,
}
impl From<Extpolar3> for bool {
    #[inline(always)]
    fn from(variant: Extpolar3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTPOLAR3` reader - External interrupt 3 EINT3 polarity."]
pub type Extpolar3R = crate::BitReader<Extpolar3>;
impl Extpolar3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extpolar3 {
        match self.bits {
            false => Extpolar3::FallingEdge,
            true => Extpolar3::RisingEdge,
        }
    }
    #[doc = "Falling edge. EINT3 is low-active or falling-edge sensitive (depending on EXTMODE3)."]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == Extpolar3::FallingEdge
    }
    #[doc = "Rising edge. EINT3 is high-active or rising-edge sensitive (depending on EXTMODE3)."]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == Extpolar3::RisingEdge
    }
}
#[doc = "Field `EXTPOLAR3` writer - External interrupt 3 EINT3 polarity."]
pub type Extpolar3W<'a, REG> = crate::BitWriter<'a, REG, Extpolar3>;
impl<'a, REG> Extpolar3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling edge. EINT3 is low-active or falling-edge sensitive (depending on EXTMODE3)."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Extpolar3::FallingEdge)
    }
    #[doc = "Rising edge. EINT3 is high-active or rising-edge sensitive (depending on EXTMODE3)."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Extpolar3::RisingEdge)
    }
}
impl R {
    #[doc = "Bit 0 - External interrupt 0 EINT0 polarity."]
    #[inline(always)]
    pub fn extpolar0(&self) -> Extpolar0R {
        Extpolar0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External interrupt 1 EINT1 polarity."]
    #[inline(always)]
    pub fn extpolar1(&self) -> Extpolar1R {
        Extpolar1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External interrupt 2 EINT2 polarity."]
    #[inline(always)]
    pub fn extpolar2(&self) -> Extpolar2R {
        Extpolar2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - External interrupt 3 EINT3 polarity."]
    #[inline(always)]
    pub fn extpolar3(&self) -> Extpolar3R {
        Extpolar3R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - External interrupt 0 EINT0 polarity."]
    #[inline(always)]
    #[must_use]
    pub fn extpolar0(&mut self) -> Extpolar0W<ExtpolarSpec> {
        Extpolar0W::new(self, 0)
    }
    #[doc = "Bit 1 - External interrupt 1 EINT1 polarity."]
    #[inline(always)]
    #[must_use]
    pub fn extpolar1(&mut self) -> Extpolar1W<ExtpolarSpec> {
        Extpolar1W::new(self, 1)
    }
    #[doc = "Bit 2 - External interrupt 2 EINT2 polarity."]
    #[inline(always)]
    #[must_use]
    pub fn extpolar2(&mut self) -> Extpolar2W<ExtpolarSpec> {
        Extpolar2W::new(self, 2)
    }
    #[doc = "Bit 3 - External interrupt 3 EINT3 polarity."]
    #[inline(always)]
    #[must_use]
    pub fn extpolar3(&mut self) -> Extpolar3W<ExtpolarSpec> {
        Extpolar3W::new(self, 3)
    }
}
#[doc = "External Interrupt Polarity Register\n\nYou can [`read`](crate::Reg::read) this register and get [`extpolar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extpolar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtpolarSpec;
impl crate::RegisterSpec for ExtpolarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extpolar::R`](R) reader structure"]
impl crate::Readable for ExtpolarSpec {}
#[doc = "`write(|w| ..)` method takes [`extpolar::W`](W) writer structure"]
impl crate::Writable for ExtpolarSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTPOLAR to value 0"]
impl crate::Resettable for ExtpolarSpec {
    const RESET_VALUE: u32 = 0;
}
