#[doc = "Register `HASHFILTERL` reader"]
pub type R = crate::R<HashfilterlSpec>;
#[doc = "Register `HASHFILTERL` writer"]
pub type W = crate::W<HashfilterlSpec>;
#[doc = "Field `HFL` reader - HashFilterL. Bits 31:0 of the imperfect filter hash table for receive filtering."]
pub type HflR = crate::FieldReader<u32>;
#[doc = "Field `HFL` writer - HashFilterL. Bits 31:0 of the imperfect filter hash table for receive filtering."]
pub type HflW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - HashFilterL. Bits 31:0 of the imperfect filter hash table for receive filtering."]
    #[inline(always)]
    pub fn hfl(&self) -> HflR {
        HflR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - HashFilterL. Bits 31:0 of the imperfect filter hash table for receive filtering."]
    #[inline(always)]
    #[must_use]
    pub fn hfl(&mut self) -> HflW<HashfilterlSpec> {
        HflW::new(self, 0)
    }
}
#[doc = "Hash filter table LSBs register.\n\nYou can [`read`](crate::Reg::read) this register and get [`hashfilterl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hashfilterl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HashfilterlSpec;
impl crate::RegisterSpec for HashfilterlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hashfilterl::R`](R) reader structure"]
impl crate::Readable for HashfilterlSpec {}
#[doc = "`write(|w| ..)` method takes [`hashfilterl::W`](W) writer structure"]
impl crate::Writable for HashfilterlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASHFILTERL to value 0"]
impl crate::Resettable for HashfilterlSpec {
    const RESET_VALUE: u32 = 0;
}
