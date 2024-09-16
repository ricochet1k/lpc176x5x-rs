#[doc = "Register `RXFILTERCTRL` reader"]
pub type R = crate::R<RxfilterctrlSpec>;
#[doc = "Register `RXFILTERCTRL` writer"]
pub type W = crate::W<RxfilterctrlSpec>;
#[doc = "Field `AUE` reader - AcceptUnicastEn. When set to 1, all unicast frames are accepted."]
pub type AueR = crate::BitReader;
#[doc = "Field `AUE` writer - AcceptUnicastEn. When set to 1, all unicast frames are accepted."]
pub type AueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABE` reader - AcceptBroadcastEn. When set to 1, all broadcast frames are accepted."]
pub type AbeR = crate::BitReader;
#[doc = "Field `ABE` writer - AcceptBroadcastEn. When set to 1, all broadcast frames are accepted."]
pub type AbeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AME` reader - AcceptMulticastEn. When set to 1, all multicast frames are accepted."]
pub type AmeR = crate::BitReader;
#[doc = "Field `AME` writer - AcceptMulticastEn. When set to 1, all multicast frames are accepted."]
pub type AmeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUHE` reader - AcceptUnicastHashEn. When set to 1, unicast frames that pass the imperfect hash filter are accepted."]
pub type AuheR = crate::BitReader;
#[doc = "Field `AUHE` writer - AcceptUnicastHashEn. When set to 1, unicast frames that pass the imperfect hash filter are accepted."]
pub type AuheW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AMHE` reader - AcceptMulticastHashEn. When set to 1, multicast frames that pass the imperfect hash filter are accepted."]
pub type AmheR = crate::BitReader;
#[doc = "Field `AMHE` writer - AcceptMulticastHashEn. When set to 1, multicast frames that pass the imperfect hash filter are accepted."]
pub type AmheW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APE` reader - AcceptPerfectEn. When set to 1, the frames with a destination address identical to the station address are accepted."]
pub type ApeR = crate::BitReader;
#[doc = "Field `APE` writer - AcceptPerfectEn. When set to 1, the frames with a destination address identical to the station address are accepted."]
pub type ApeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPEW` reader - MagicPacketEnWoL. When set to 1, the result of the magic packet filter will generate a WoL interrupt when there is a match."]
pub type MpewR = crate::BitReader;
#[doc = "Field `MPEW` writer - MagicPacketEnWoL. When set to 1, the result of the magic packet filter will generate a WoL interrupt when there is a match."]
pub type MpewW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFEW` reader - RxFilterEnWoL. When set to 1, the result of the perfect address matching filter and the imperfect hash filter will generate a WoL interrupt when there is a match."]
pub type RfewR = crate::BitReader;
#[doc = "Field `RFEW` writer - RxFilterEnWoL. When set to 1, the result of the perfect address matching filter and the imperfect hash filter will generate a WoL interrupt when there is a match."]
pub type RfewW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - AcceptUnicastEn. When set to 1, all unicast frames are accepted."]
    #[inline(always)]
    pub fn aue(&self) -> AueR {
        AueR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AcceptBroadcastEn. When set to 1, all broadcast frames are accepted."]
    #[inline(always)]
    pub fn abe(&self) -> AbeR {
        AbeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AcceptMulticastEn. When set to 1, all multicast frames are accepted."]
    #[inline(always)]
    pub fn ame(&self) -> AmeR {
        AmeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AcceptUnicastHashEn. When set to 1, unicast frames that pass the imperfect hash filter are accepted."]
    #[inline(always)]
    pub fn auhe(&self) -> AuheR {
        AuheR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AcceptMulticastHashEn. When set to 1, multicast frames that pass the imperfect hash filter are accepted."]
    #[inline(always)]
    pub fn amhe(&self) -> AmheR {
        AmheR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AcceptPerfectEn. When set to 1, the frames with a destination address identical to the station address are accepted."]
    #[inline(always)]
    pub fn ape(&self) -> ApeR {
        ApeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 12 - MagicPacketEnWoL. When set to 1, the result of the magic packet filter will generate a WoL interrupt when there is a match."]
    #[inline(always)]
    pub fn mpew(&self) -> MpewR {
        MpewR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - RxFilterEnWoL. When set to 1, the result of the perfect address matching filter and the imperfect hash filter will generate a WoL interrupt when there is a match."]
    #[inline(always)]
    pub fn rfew(&self) -> RfewR {
        RfewR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AcceptUnicastEn. When set to 1, all unicast frames are accepted."]
    #[inline(always)]
    #[must_use]
    pub fn aue(&mut self) -> AueW<RxfilterctrlSpec> {
        AueW::new(self, 0)
    }
    #[doc = "Bit 1 - AcceptBroadcastEn. When set to 1, all broadcast frames are accepted."]
    #[inline(always)]
    #[must_use]
    pub fn abe(&mut self) -> AbeW<RxfilterctrlSpec> {
        AbeW::new(self, 1)
    }
    #[doc = "Bit 2 - AcceptMulticastEn. When set to 1, all multicast frames are accepted."]
    #[inline(always)]
    #[must_use]
    pub fn ame(&mut self) -> AmeW<RxfilterctrlSpec> {
        AmeW::new(self, 2)
    }
    #[doc = "Bit 3 - AcceptUnicastHashEn. When set to 1, unicast frames that pass the imperfect hash filter are accepted."]
    #[inline(always)]
    #[must_use]
    pub fn auhe(&mut self) -> AuheW<RxfilterctrlSpec> {
        AuheW::new(self, 3)
    }
    #[doc = "Bit 4 - AcceptMulticastHashEn. When set to 1, multicast frames that pass the imperfect hash filter are accepted."]
    #[inline(always)]
    #[must_use]
    pub fn amhe(&mut self) -> AmheW<RxfilterctrlSpec> {
        AmheW::new(self, 4)
    }
    #[doc = "Bit 5 - AcceptPerfectEn. When set to 1, the frames with a destination address identical to the station address are accepted."]
    #[inline(always)]
    #[must_use]
    pub fn ape(&mut self) -> ApeW<RxfilterctrlSpec> {
        ApeW::new(self, 5)
    }
    #[doc = "Bit 12 - MagicPacketEnWoL. When set to 1, the result of the magic packet filter will generate a WoL interrupt when there is a match."]
    #[inline(always)]
    #[must_use]
    pub fn mpew(&mut self) -> MpewW<RxfilterctrlSpec> {
        MpewW::new(self, 12)
    }
    #[doc = "Bit 13 - RxFilterEnWoL. When set to 1, the result of the perfect address matching filter and the imperfect hash filter will generate a WoL interrupt when there is a match."]
    #[inline(always)]
    #[must_use]
    pub fn rfew(&mut self) -> RfewW<RxfilterctrlSpec> {
        RfewW::new(self, 13)
    }
}
#[doc = "Receive filter control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rxfilterctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxfilterctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxfilterctrlSpec;
impl crate::RegisterSpec for RxfilterctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxfilterctrl::R`](R) reader structure"]
impl crate::Readable for RxfilterctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`rxfilterctrl::W`](W) writer structure"]
impl crate::Writable for RxfilterctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXFILTERCTRL to value 0"]
impl crate::Resettable for RxfilterctrlSpec {
    const RESET_VALUE: u32 = 0;
}
