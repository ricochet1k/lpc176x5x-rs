#[doc = "Register `TID%s` reader"]
pub type R = crate::R<TidSpec>;
#[doc = "Register `TID%s` writer"]
pub type W = crate::W<TidSpec>;
#[doc = "Field `ID` reader - The 11-bit Identifier to be sent in the next transmit message."]
pub type IdR = crate::FieldReader<u16>;
#[doc = "Field `ID` writer - The 11-bit Identifier to be sent in the next transmit message."]
pub type IdW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - The 11-bit Identifier to be sent in the next transmit message."]
    #[inline(always)]
    pub fn id(&self) -> IdR {
        IdR::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - The 11-bit Identifier to be sent in the next transmit message."]
    #[inline(always)]
    #[must_use]
    pub fn id(&mut self) -> IdW<TidSpec> {
        IdW::new(self, 0)
    }
}
#[doc = "Transmit Identifier (Tx Buffer)\n\nYou can [`read`](crate::Reg::read) this register and get [`tid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TidSpec;
impl crate::RegisterSpec for TidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tid::R`](R) reader structure"]
impl crate::Readable for TidSpec {}
#[doc = "`write(|w| ..)` method takes [`tid::W`](W) writer structure"]
impl crate::Writable for TidSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TID%s to value 0"]
impl crate::Resettable for TidSpec {
    const RESET_VALUE: u32 = 0;
}
