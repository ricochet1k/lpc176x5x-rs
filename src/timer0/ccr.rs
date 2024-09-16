#[doc = "Register `CCR` reader"]
pub type R = crate::R<CcrSpec>;
#[doc = "Register `CCR` writer"]
pub type W = crate::W<CcrSpec>;
#[doc = "Capture on CAPn.0 rising edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cap0re {
    #[doc = "1: A sequence of 0 then 1 on CAPn.0 will cause CR0 to be loaded with the contents of TC."]
    Enable = 1,
    #[doc = "0: This feature is disabled."]
    Disable = 0,
}
impl From<Cap0re> for bool {
    #[inline(always)]
    fn from(variant: Cap0re) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAP0RE` reader - Capture on CAPn.0 rising edge"]
pub type Cap0reR = crate::BitReader<Cap0re>;
impl Cap0reR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cap0re {
        match self.bits {
            true => Cap0re::Enable,
            false => Cap0re::Disable,
        }
    }
    #[doc = "A sequence of 0 then 1 on CAPn.0 will cause CR0 to be loaded with the contents of TC."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cap0re::Enable
    }
    #[doc = "This feature is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cap0re::Disable
    }
}
#[doc = "Field `CAP0RE` writer - Capture on CAPn.0 rising edge"]
pub type Cap0reW<'a, REG> = crate::BitWriter<'a, REG, Cap0re>;
impl<'a, REG> Cap0reW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A sequence of 0 then 1 on CAPn.0 will cause CR0 to be loaded with the contents of TC."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cap0re::Enable)
    }
    #[doc = "This feature is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cap0re::Disable)
    }
}
#[doc = "Capture on CAPn.0 falling edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cap0fe {
    #[doc = "1: A sequence of 1 then 0 on CAPn.0 will cause CR0 to be loaded with the contents of TC."]
    Enable = 1,
    #[doc = "0: This feature is disabled."]
    Disable = 0,
}
impl From<Cap0fe> for bool {
    #[inline(always)]
    fn from(variant: Cap0fe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAP0FE` reader - Capture on CAPn.0 falling edge"]
pub type Cap0feR = crate::BitReader<Cap0fe>;
impl Cap0feR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cap0fe {
        match self.bits {
            true => Cap0fe::Enable,
            false => Cap0fe::Disable,
        }
    }
    #[doc = "A sequence of 1 then 0 on CAPn.0 will cause CR0 to be loaded with the contents of TC."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cap0fe::Enable
    }
    #[doc = "This feature is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cap0fe::Disable
    }
}
#[doc = "Field `CAP0FE` writer - Capture on CAPn.0 falling edge"]
pub type Cap0feW<'a, REG> = crate::BitWriter<'a, REG, Cap0fe>;
impl<'a, REG> Cap0feW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A sequence of 1 then 0 on CAPn.0 will cause CR0 to be loaded with the contents of TC."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cap0fe::Enable)
    }
    #[doc = "This feature is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cap0fe::Disable)
    }
}
#[doc = "Interrupt on CAPn.0 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cap0i {
    #[doc = "1: A CR0 load due to a CAPn.0 event will generate an interrupt."]
    Enable = 1,
    #[doc = "0: This feature is disabled."]
    Disable = 0,
}
impl From<Cap0i> for bool {
    #[inline(always)]
    fn from(variant: Cap0i) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAP0I` reader - Interrupt on CAPn.0 event"]
pub type Cap0iR = crate::BitReader<Cap0i>;
impl Cap0iR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cap0i {
        match self.bits {
            true => Cap0i::Enable,
            false => Cap0i::Disable,
        }
    }
    #[doc = "A CR0 load due to a CAPn.0 event will generate an interrupt."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cap0i::Enable
    }
    #[doc = "This feature is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cap0i::Disable
    }
}
#[doc = "Field `CAP0I` writer - Interrupt on CAPn.0 event"]
pub type Cap0iW<'a, REG> = crate::BitWriter<'a, REG, Cap0i>;
impl<'a, REG> Cap0iW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A CR0 load due to a CAPn.0 event will generate an interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cap0i::Enable)
    }
    #[doc = "This feature is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cap0i::Disable)
    }
}
#[doc = "Capture on CAPn.1 rising edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cap1re {
    #[doc = "1: A sequence of 0 then 1 on CAPn.1 will cause CR1 to be loaded with the contents of TC."]
    Enable = 1,
    #[doc = "0: This feature is disabled."]
    Disable = 0,
}
impl From<Cap1re> for bool {
    #[inline(always)]
    fn from(variant: Cap1re) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAP1RE` reader - Capture on CAPn.1 rising edge"]
pub type Cap1reR = crate::BitReader<Cap1re>;
impl Cap1reR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cap1re {
        match self.bits {
            true => Cap1re::Enable,
            false => Cap1re::Disable,
        }
    }
    #[doc = "A sequence of 0 then 1 on CAPn.1 will cause CR1 to be loaded with the contents of TC."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cap1re::Enable
    }
    #[doc = "This feature is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cap1re::Disable
    }
}
#[doc = "Field `CAP1RE` writer - Capture on CAPn.1 rising edge"]
pub type Cap1reW<'a, REG> = crate::BitWriter<'a, REG, Cap1re>;
impl<'a, REG> Cap1reW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A sequence of 0 then 1 on CAPn.1 will cause CR1 to be loaded with the contents of TC."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cap1re::Enable)
    }
    #[doc = "This feature is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cap1re::Disable)
    }
}
#[doc = "Capture on CAPn.1 falling edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cap1fe {
    #[doc = "1: A sequence of 1 then 0 on CAPn.1 will cause CR1 to be loaded with the contents of TC."]
    Enable = 1,
    #[doc = "0: This feature is disabled."]
    Disable = 0,
}
impl From<Cap1fe> for bool {
    #[inline(always)]
    fn from(variant: Cap1fe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAP1FE` reader - Capture on CAPn.1 falling edge"]
pub type Cap1feR = crate::BitReader<Cap1fe>;
impl Cap1feR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cap1fe {
        match self.bits {
            true => Cap1fe::Enable,
            false => Cap1fe::Disable,
        }
    }
    #[doc = "A sequence of 1 then 0 on CAPn.1 will cause CR1 to be loaded with the contents of TC."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cap1fe::Enable
    }
    #[doc = "This feature is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cap1fe::Disable
    }
}
#[doc = "Field `CAP1FE` writer - Capture on CAPn.1 falling edge"]
pub type Cap1feW<'a, REG> = crate::BitWriter<'a, REG, Cap1fe>;
impl<'a, REG> Cap1feW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A sequence of 1 then 0 on CAPn.1 will cause CR1 to be loaded with the contents of TC."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cap1fe::Enable)
    }
    #[doc = "This feature is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cap1fe::Disable)
    }
}
#[doc = "Interrupt on CAPn.1 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cap1i {
    #[doc = "1: A CR1 load due to a CAPn.1 event will generate an interrupt."]
    Enable = 1,
    #[doc = "0: This feature is disabled."]
    Disable = 0,
}
impl From<Cap1i> for bool {
    #[inline(always)]
    fn from(variant: Cap1i) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAP1I` reader - Interrupt on CAPn.1 event"]
pub type Cap1iR = crate::BitReader<Cap1i>;
impl Cap1iR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cap1i {
        match self.bits {
            true => Cap1i::Enable,
            false => Cap1i::Disable,
        }
    }
    #[doc = "A CR1 load due to a CAPn.1 event will generate an interrupt."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cap1i::Enable
    }
    #[doc = "This feature is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cap1i::Disable
    }
}
#[doc = "Field `CAP1I` writer - Interrupt on CAPn.1 event"]
pub type Cap1iW<'a, REG> = crate::BitWriter<'a, REG, Cap1i>;
impl<'a, REG> Cap1iW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A CR1 load due to a CAPn.1 event will generate an interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cap1i::Enable)
    }
    #[doc = "This feature is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cap1i::Disable)
    }
}
impl R {
    #[doc = "Bit 0 - Capture on CAPn.0 rising edge"]
    #[inline(always)]
    pub fn cap0re(&self) -> Cap0reR {
        Cap0reR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Capture on CAPn.0 falling edge"]
    #[inline(always)]
    pub fn cap0fe(&self) -> Cap0feR {
        Cap0feR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt on CAPn.0 event"]
    #[inline(always)]
    pub fn cap0i(&self) -> Cap0iR {
        Cap0iR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture on CAPn.1 rising edge"]
    #[inline(always)]
    pub fn cap1re(&self) -> Cap1reR {
        Cap1reR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Capture on CAPn.1 falling edge"]
    #[inline(always)]
    pub fn cap1fe(&self) -> Cap1feR {
        Cap1feR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt on CAPn.1 event"]
    #[inline(always)]
    pub fn cap1i(&self) -> Cap1iR {
        Cap1iR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Capture on CAPn.0 rising edge"]
    #[inline(always)]
    #[must_use]
    pub fn cap0re(&mut self) -> Cap0reW<CcrSpec> {
        Cap0reW::new(self, 0)
    }
    #[doc = "Bit 1 - Capture on CAPn.0 falling edge"]
    #[inline(always)]
    #[must_use]
    pub fn cap0fe(&mut self) -> Cap0feW<CcrSpec> {
        Cap0feW::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt on CAPn.0 event"]
    #[inline(always)]
    #[must_use]
    pub fn cap0i(&mut self) -> Cap0iW<CcrSpec> {
        Cap0iW::new(self, 2)
    }
    #[doc = "Bit 3 - Capture on CAPn.1 rising edge"]
    #[inline(always)]
    #[must_use]
    pub fn cap1re(&mut self) -> Cap1reW<CcrSpec> {
        Cap1reW::new(self, 3)
    }
    #[doc = "Bit 4 - Capture on CAPn.1 falling edge"]
    #[inline(always)]
    #[must_use]
    pub fn cap1fe(&mut self) -> Cap1feW<CcrSpec> {
        Cap1feW::new(self, 4)
    }
    #[doc = "Bit 5 - Interrupt on CAPn.1 event"]
    #[inline(always)]
    #[must_use]
    pub fn cap1i(&mut self) -> Cap1iW<CcrSpec> {
        Cap1iW::new(self, 5)
    }
}
#[doc = "Capture Control Register. The CCR controls which edges of the capture inputs are used to load the Capture Registers and whether or not an interrupt is generated when a capture takes place.\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcrSpec;
impl crate::RegisterSpec for CcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr::R`](R) reader structure"]
impl crate::Readable for CcrSpec {}
#[doc = "`write(|w| ..)` method takes [`ccr::W`](W) writer structure"]
impl crate::Writable for CcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCR to value 0"]
impl crate::Resettable for CcrSpec {
    const RESET_VALUE: u32 = 0;
}
