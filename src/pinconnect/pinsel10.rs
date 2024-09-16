#[doc = "Register `PINSEL10` reader"]
pub type R = crate::R<Pinsel10Spec>;
#[doc = "Register `PINSEL10` writer"]
pub type W = crate::W<Pinsel10Spec>;
#[doc = "TPIU interface pins control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tpiuctrl {
    #[doc = "0: Disabled. TPIU interface is disabled."]
    Disabled = 0,
    #[doc = "1: Enabled. TPIU interface is enabled. TPIU signals are available on the pins hosting them regardless of the PINSEL4 content."]
    Enabled = 1,
}
impl From<Tpiuctrl> for bool {
    #[inline(always)]
    fn from(variant: Tpiuctrl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TPIUCTRL` reader - TPIU interface pins control."]
pub type TpiuctrlR = crate::BitReader<Tpiuctrl>;
impl TpiuctrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tpiuctrl {
        match self.bits {
            false => Tpiuctrl::Disabled,
            true => Tpiuctrl::Enabled,
        }
    }
    #[doc = "Disabled. TPIU interface is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Tpiuctrl::Disabled
    }
    #[doc = "Enabled. TPIU interface is enabled. TPIU signals are available on the pins hosting them regardless of the PINSEL4 content."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Tpiuctrl::Enabled
    }
}
#[doc = "Field `TPIUCTRL` writer - TPIU interface pins control."]
pub type TpiuctrlW<'a, REG> = crate::BitWriter<'a, REG, Tpiuctrl>;
impl<'a, REG> TpiuctrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. TPIU interface is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Tpiuctrl::Disabled)
    }
    #[doc = "Enabled. TPIU interface is enabled. TPIU signals are available on the pins hosting them regardless of the PINSEL4 content."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Tpiuctrl::Enabled)
    }
}
impl R {
    #[doc = "Bit 3 - TPIU interface pins control."]
    #[inline(always)]
    pub fn tpiuctrl(&self) -> TpiuctrlR {
        TpiuctrlR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - TPIU interface pins control."]
    #[inline(always)]
    #[must_use]
    pub fn tpiuctrl(&mut self) -> TpiuctrlW<Pinsel10Spec> {
        TpiuctrlW::new(self, 3)
    }
}
#[doc = "Pin function select register 10\n\nYou can [`read`](crate::Reg::read) this register and get [`pinsel10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pinsel10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pinsel10Spec;
impl crate::RegisterSpec for Pinsel10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pinsel10::R`](R) reader structure"]
impl crate::Readable for Pinsel10Spec {}
#[doc = "`write(|w| ..)` method takes [`pinsel10::W`](W) writer structure"]
impl crate::Writable for Pinsel10Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PINSEL10 to value 0"]
impl crate::Resettable for Pinsel10Spec {
    const RESET_VALUE: u32 = 0;
}
