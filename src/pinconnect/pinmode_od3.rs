#[doc = "Register `PINMODE_OD3` reader"]
pub type R = crate::R<PinmodeOd3Spec>;
#[doc = "Register `PINMODE_OD3` writer"]
pub type W = crate::W<PinmodeOd3Spec>;
#[doc = "Port 3 pin 25 open drain mode control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P3_25od {
    #[doc = "0: Normal. P3.25 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P3.25 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<P3_25od> for bool {
    #[inline(always)]
    fn from(variant: P3_25od) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P3_25OD` reader - Port 3 pin 25 open drain mode control."]
pub type P3_25odR = crate::BitReader<P3_25od>;
impl P3_25odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P3_25od {
        match self.bits {
            false => P3_25od::Normal,
            true => P3_25od::OpenDrain,
        }
    }
    #[doc = "Normal. P3.25 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P3_25od::Normal
    }
    #[doc = "Open-drain. P3.25 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P3_25od::OpenDrain
    }
}
#[doc = "Field `P3_25OD` writer - Port 3 pin 25 open drain mode control."]
pub type P3_25odW<'a, REG> = crate::BitWriter<'a, REG, P3_25od>;
impl<'a, REG> P3_25odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P3.25 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(P3_25od::Normal)
    }
    #[doc = "Open-drain. P3.25 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(P3_25od::OpenDrain)
    }
}
#[doc = "Field `P3_26OD` reader - Port 3 pin 26 open drain mode control, see P3.25OD"]
pub type P3_26odR = crate::BitReader;
#[doc = "Field `P3_26OD` writer - Port 3 pin 26 open drain mode control, see P3.25OD"]
pub type P3_26odW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 25 - Port 3 pin 25 open drain mode control."]
    #[inline(always)]
    pub fn p3_25od(&self) -> P3_25odR {
        P3_25odR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Port 3 pin 26 open drain mode control, see P3.25OD"]
    #[inline(always)]
    pub fn p3_26od(&self) -> P3_26odR {
        P3_26odR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 25 - Port 3 pin 25 open drain mode control."]
    #[inline(always)]
    #[must_use]
    pub fn p3_25od(&mut self) -> P3_25odW<PinmodeOd3Spec> {
        P3_25odW::new(self, 25)
    }
    #[doc = "Bit 26 - Port 3 pin 26 open drain mode control, see P3.25OD"]
    #[inline(always)]
    #[must_use]
    pub fn p3_26od(&mut self) -> P3_26odW<PinmodeOd3Spec> {
        P3_26odW::new(self, 26)
    }
}
#[doc = "Open drain mode control register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`pinmode_od3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pinmode_od3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PinmodeOd3Spec;
impl crate::RegisterSpec for PinmodeOd3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pinmode_od3::R`](R) reader structure"]
impl crate::Readable for PinmodeOd3Spec {}
#[doc = "`write(|w| ..)` method takes [`pinmode_od3::W`](W) writer structure"]
impl crate::Writable for PinmodeOd3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PINMODE_OD3 to value 0"]
impl crate::Resettable for PinmodeOd3Spec {
    const RESET_VALUE: u32 = 0;
}
