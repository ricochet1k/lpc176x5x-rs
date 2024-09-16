#[doc = "Register `RXFILTERWOLSTATUS` reader"]
pub type R = crate::R<RxfilterwolstatusSpec>;
#[doc = "Field `AUW` reader - AcceptUnicastWoL. When the value is 1, a unicast frames caused WoL."]
pub type AuwR = crate::BitReader;
#[doc = "Field `ABW` reader - AcceptBroadcastWoL. When the value is 1, a broadcast frame caused WoL."]
pub type AbwR = crate::BitReader;
#[doc = "Field `AMW` reader - AcceptMulticastWoL. When the value is 1, a multicast frame caused WoL."]
pub type AmwR = crate::BitReader;
#[doc = "Field `AUHW` reader - AcceptUnicastHashWoL. When the value is 1, a unicast frame that passes the imperfect hash filter caused WoL."]
pub type AuhwR = crate::BitReader;
#[doc = "Field `AMHW` reader - AcceptMulticastHashWoL. When the value is 1, a multicast frame that passes the imperfect hash filter caused WoL."]
pub type AmhwR = crate::BitReader;
#[doc = "Field `APW` reader - AcceptPerfectWoL. When the value is 1, the perfect address matching filter caused WoL."]
pub type ApwR = crate::BitReader;
#[doc = "Field `RFW` reader - RxFilterWoL. When the value is 1, the receive filter caused WoL."]
pub type RfwR = crate::BitReader;
#[doc = "Field `MPW` reader - MagicPacketWoL. When the value is 1, the magic packet filter caused WoL."]
pub type MpwR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - AcceptUnicastWoL. When the value is 1, a unicast frames caused WoL."]
    #[inline(always)]
    pub fn auw(&self) -> AuwR {
        AuwR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AcceptBroadcastWoL. When the value is 1, a broadcast frame caused WoL."]
    #[inline(always)]
    pub fn abw(&self) -> AbwR {
        AbwR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AcceptMulticastWoL. When the value is 1, a multicast frame caused WoL."]
    #[inline(always)]
    pub fn amw(&self) -> AmwR {
        AmwR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AcceptUnicastHashWoL. When the value is 1, a unicast frame that passes the imperfect hash filter caused WoL."]
    #[inline(always)]
    pub fn auhw(&self) -> AuhwR {
        AuhwR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AcceptMulticastHashWoL. When the value is 1, a multicast frame that passes the imperfect hash filter caused WoL."]
    #[inline(always)]
    pub fn amhw(&self) -> AmhwR {
        AmhwR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AcceptPerfectWoL. When the value is 1, the perfect address matching filter caused WoL."]
    #[inline(always)]
    pub fn apw(&self) -> ApwR {
        ApwR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - RxFilterWoL. When the value is 1, the receive filter caused WoL."]
    #[inline(always)]
    pub fn rfw(&self) -> RfwR {
        RfwR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - MagicPacketWoL. When the value is 1, the magic packet filter caused WoL."]
    #[inline(always)]
    pub fn mpw(&self) -> MpwR {
        MpwR::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "Receive filter WoL status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rxfilterwolstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxfilterwolstatusSpec;
impl crate::RegisterSpec for RxfilterwolstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxfilterwolstatus::R`](R) reader structure"]
impl crate::Readable for RxfilterwolstatusSpec {}
#[doc = "`reset()` method sets RXFILTERWOLSTATUS to value 0"]
impl crate::Resettable for RxfilterwolstatusSpec {
    const RESET_VALUE: u32 = 0;
}
