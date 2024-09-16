#[doc = "Register `LUTERR` reader"]
pub type R = crate::R<LuterrSpec>;
#[doc = "Field `LUTERR` reader - This read-only bit is set to 1 if the Acceptance Filter encounters an error in the content of the tables in AF RAM. It is cleared when software reads the LUTerrAd register. This condition is ORed with the other CAN interrupts from the CAN controllers, to produce the request that is connected to the NVIC."]
pub type LuterrR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This read-only bit is set to 1 if the Acceptance Filter encounters an error in the content of the tables in AF RAM. It is cleared when software reads the LUTerrAd register. This condition is ORed with the other CAN interrupts from the CAN controllers, to produce the request that is connected to the NVIC."]
    #[inline(always)]
    pub fn luterr(&self) -> LuterrR {
        LuterrR::new((self.bits & 1) != 0)
    }
}
#[doc = "LUT Error Register\n\nYou can [`read`](crate::Reg::read) this register and get [`luterr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LuterrSpec;
impl crate::RegisterSpec for LuterrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`luterr::R`](R) reader structure"]
impl crate::Readable for LuterrSpec {}
#[doc = "`reset()` method sets LUTERR to value 0"]
impl crate::Resettable for LuterrSpec {
    const RESET_VALUE: u32 = 0;
}
