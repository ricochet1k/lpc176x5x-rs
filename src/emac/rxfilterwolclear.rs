#[doc = "Register `RXFILTERWOLCLEAR` writer"]
pub type W = crate::W<RxfilterwolclearSpec>;
#[doc = "Field `AUWCLR` writer - AcceptUnicastWoLClr. When a 1 is written, the corresponding status bit in the RxFilterWoLStatus register is cleared."]
pub type AuwclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABWCLR` writer - AcceptBroadcastWoLClr. When a 1 is written, the corresponding status bit in the RxFilterWoLStatus register is cleared."]
pub type AbwclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AMWCLR` writer - AcceptMulticastWoLClr. When a 1 is written, the corresponding status bit in the RxFilterWoLStatus register is cleared."]
pub type AmwclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUHWCLR` writer - AcceptUnicastHashWoLClr. When a 1 is written, the corresponding status bit in the RxFilterWoLStatus register is cleared."]
pub type AuhwclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AMHWCLR` writer - AcceptMulticastHashWoLClr. When a 1 is written, the corresponding status bit in the RxFilterWoLStatus register is cleared."]
pub type AmhwclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APWCLR` writer - AcceptPerfectWoLClr. When a 1 is written, the corresponding status bit in the RxFilterWoLStatus register is cleared."]
pub type ApwclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFWCLR` writer - RxFilterWoLClr. When a 1 is written, the corresponding status bit in the RxFilterWoLStatus register is cleared."]
pub type RfwclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPWCLR` writer - MagicPacketWoLClr. When a 1 is written, the corresponding status bit in the RxFilterWoLStatus register is cleared."]
pub type MpwclrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - AcceptUnicastWoLClr. When a 1 is written, the corresponding status bit in the RxFilterWoLStatus register is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn auwclr(&mut self) -> AuwclrW<RxfilterwolclearSpec> {
        AuwclrW::new(self, 0)
    }
    #[doc = "Bit 1 - AcceptBroadcastWoLClr. When a 1 is written, the corresponding status bit in the RxFilterWoLStatus register is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn abwclr(&mut self) -> AbwclrW<RxfilterwolclearSpec> {
        AbwclrW::new(self, 1)
    }
    #[doc = "Bit 2 - AcceptMulticastWoLClr. When a 1 is written, the corresponding status bit in the RxFilterWoLStatus register is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn amwclr(&mut self) -> AmwclrW<RxfilterwolclearSpec> {
        AmwclrW::new(self, 2)
    }
    #[doc = "Bit 3 - AcceptUnicastHashWoLClr. When a 1 is written, the corresponding status bit in the RxFilterWoLStatus register is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn auhwclr(&mut self) -> AuhwclrW<RxfilterwolclearSpec> {
        AuhwclrW::new(self, 3)
    }
    #[doc = "Bit 4 - AcceptMulticastHashWoLClr. When a 1 is written, the corresponding status bit in the RxFilterWoLStatus register is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn amhwclr(&mut self) -> AmhwclrW<RxfilterwolclearSpec> {
        AmhwclrW::new(self, 4)
    }
    #[doc = "Bit 5 - AcceptPerfectWoLClr. When a 1 is written, the corresponding status bit in the RxFilterWoLStatus register is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn apwclr(&mut self) -> ApwclrW<RxfilterwolclearSpec> {
        ApwclrW::new(self, 5)
    }
    #[doc = "Bit 7 - RxFilterWoLClr. When a 1 is written, the corresponding status bit in the RxFilterWoLStatus register is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn rfwclr(&mut self) -> RfwclrW<RxfilterwolclearSpec> {
        RfwclrW::new(self, 7)
    }
    #[doc = "Bit 8 - MagicPacketWoLClr. When a 1 is written, the corresponding status bit in the RxFilterWoLStatus register is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn mpwclr(&mut self) -> MpwclrW<RxfilterwolclearSpec> {
        MpwclrW::new(self, 8)
    }
}
#[doc = "Receive filter WoL clear register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxfilterwolclear::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxfilterwolclearSpec;
impl crate::RegisterSpec for RxfilterwolclearSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`rxfilterwolclear::W`](W) writer structure"]
impl crate::Writable for RxfilterwolclearSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXFILTERWOLCLEAR to value 0"]
impl crate::Resettable for RxfilterwolclearSpec {
    const RESET_VALUE: u32 = 0;
}
