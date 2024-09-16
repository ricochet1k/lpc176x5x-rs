#[doc = "Register `CLRT` reader"]
pub type R = crate::R<ClrtSpec>;
#[doc = "Register `CLRT` writer"]
pub type W = crate::W<ClrtSpec>;
#[doc = "Field `RETRANSMAX` reader - RETRANSMISSION MAXIMUM.This is a programmable field specifying the number of retransmission attempts following a collision before aborting the packet due to excessive collisions. The Standard specifies the attemptLimit to be 0xF (15d). See IEEE 802.3/4.2.3.2.5."]
pub type RetransmaxR = crate::FieldReader;
#[doc = "Field `RETRANSMAX` writer - RETRANSMISSION MAXIMUM.This is a programmable field specifying the number of retransmission attempts following a collision before aborting the packet due to excessive collisions. The Standard specifies the attemptLimit to be 0xF (15d). See IEEE 802.3/4.2.3.2.5."]
pub type RetransmaxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `COLLWIN` reader - COLLISION WINDOW. This is a programmable field representing the slot time or collision window during which collisions occur in properly configured networks. The default value of 0x37 (55d) represents a 56 byte window following the preamble and SFD."]
pub type CollwinR = crate::FieldReader;
#[doc = "Field `COLLWIN` writer - COLLISION WINDOW. This is a programmable field representing the slot time or collision window during which collisions occur in properly configured networks. The default value of 0x37 (55d) represents a 56 byte window following the preamble and SFD."]
pub type CollwinW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:3 - RETRANSMISSION MAXIMUM.This is a programmable field specifying the number of retransmission attempts following a collision before aborting the packet due to excessive collisions. The Standard specifies the attemptLimit to be 0xF (15d). See IEEE 802.3/4.2.3.2.5."]
    #[inline(always)]
    pub fn retransmax(&self) -> RetransmaxR {
        RetransmaxR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:13 - COLLISION WINDOW. This is a programmable field representing the slot time or collision window during which collisions occur in properly configured networks. The default value of 0x37 (55d) represents a 56 byte window following the preamble and SFD."]
    #[inline(always)]
    pub fn collwin(&self) -> CollwinR {
        CollwinR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - RETRANSMISSION MAXIMUM.This is a programmable field specifying the number of retransmission attempts following a collision before aborting the packet due to excessive collisions. The Standard specifies the attemptLimit to be 0xF (15d). See IEEE 802.3/4.2.3.2.5."]
    #[inline(always)]
    #[must_use]
    pub fn retransmax(&mut self) -> RetransmaxW<ClrtSpec> {
        RetransmaxW::new(self, 0)
    }
    #[doc = "Bits 8:13 - COLLISION WINDOW. This is a programmable field representing the slot time or collision window during which collisions occur in properly configured networks. The default value of 0x37 (55d) represents a 56 byte window following the preamble and SFD."]
    #[inline(always)]
    #[must_use]
    pub fn collwin(&mut self) -> CollwinW<ClrtSpec> {
        CollwinW::new(self, 8)
    }
}
#[doc = "Collision window / Retry register.\n\nYou can [`read`](crate::Reg::read) this register and get [`clrt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clrt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClrtSpec;
impl crate::RegisterSpec for ClrtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clrt::R`](R) reader structure"]
impl crate::Readable for ClrtSpec {}
#[doc = "`write(|w| ..)` method takes [`clrt::W`](W) writer structure"]
impl crate::Writable for ClrtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLRT to value 0x370f"]
impl crate::Resettable for ClrtSpec {
    const RESET_VALUE: u32 = 0x370f;
}
