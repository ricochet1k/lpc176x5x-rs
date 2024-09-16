#[doc = "Register `RID` reader"]
pub type R = crate::R<RidSpec>;
#[doc = "Register `RID` writer"]
pub type W = crate::W<RidSpec>;
#[doc = "Field `ID` reader - The 11-bit Identifier field of the current received message. In CAN 2.0A, these bits are called ID10-0, while in CAN 2.0B they're called ID29-18."]
pub type IdR = crate::FieldReader<u16>;
#[doc = "Field `ID` writer - The 11-bit Identifier field of the current received message. In CAN 2.0A, these bits are called ID10-0, while in CAN 2.0B they're called ID29-18."]
pub type IdW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - The 11-bit Identifier field of the current received message. In CAN 2.0A, these bits are called ID10-0, while in CAN 2.0B they're called ID29-18."]
    #[inline(always)]
    pub fn id(&self) -> IdR {
        IdR::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - The 11-bit Identifier field of the current received message. In CAN 2.0A, these bits are called ID10-0, while in CAN 2.0B they're called ID29-18."]
    #[inline(always)]
    #[must_use]
    pub fn id(&mut self) -> IdW<RidSpec> {
        IdW::new(self, 0)
    }
}
#[doc = "Received Identifier. Can only be written when RM in CANMOD is 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`rid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RidSpec;
impl crate::RegisterSpec for RidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rid::R`](R) reader structure"]
impl crate::Readable for RidSpec {}
#[doc = "`write(|w| ..)` method takes [`rid::W`](W) writer structure"]
impl crate::Writable for RidSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RID to value 0"]
impl crate::Resettable for RidSpec {
    const RESET_VALUE: u32 = 0;
}
