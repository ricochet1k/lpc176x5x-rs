#[doc = "Register `LLI%s` reader"]
pub type R = crate::R<LliSpec>;
#[doc = "Register `LLI%s` writer"]
pub type W = crate::W<LliSpec>;
#[doc = "Field `LLI` reader - Linked list item. Bits \\[31:2\\]
of the address for the next LLI. Address bits \\[1:0\\]
are 0."]
pub type LliR = crate::FieldReader<u32>;
#[doc = "Field `LLI` writer - Linked list item. Bits \\[31:2\\]
of the address for the next LLI. Address bits \\[1:0\\]
are 0."]
pub type LliW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - Linked list item. Bits \\[31:2\\]
of the address for the next LLI. Address bits \\[1:0\\]
are 0."]
    #[inline(always)]
    pub fn lli(&self) -> LliR {
        LliR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Linked list item. Bits \\[31:2\\]
of the address for the next LLI. Address bits \\[1:0\\]
are 0."]
    #[inline(always)]
    #[must_use]
    pub fn lli(&mut self) -> LliW<LliSpec> {
        LliW::new(self, 2)
    }
}
#[doc = "DMA Channel 0 Linked List Item Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lli::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lli::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LliSpec;
impl crate::RegisterSpec for LliSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lli::R`](R) reader structure"]
impl crate::Readable for LliSpec {}
#[doc = "`write(|w| ..)` method takes [`lli::W`](W) writer structure"]
impl crate::Writable for LliSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LLI%s to value 0"]
impl crate::Resettable for LliSpec {
    const RESET_VALUE: u32 = 0;
}
