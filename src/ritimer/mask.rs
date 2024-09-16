#[doc = "Register `MASK` reader"]
pub type R = crate::R<MaskSpec>;
#[doc = "Register `MASK` writer"]
pub type W = crate::W<MaskSpec>;
#[doc = "Field `RIMASK` reader - Mask register. This register holds the 32-bit mask value. A one written to any bit overrides the result of the comparison for the corresponding bit of the counter and compare register (causes the comparison of the register bits to be always true)."]
pub type RimaskR = crate::FieldReader<u32>;
#[doc = "Field `RIMASK` writer - Mask register. This register holds the 32-bit mask value. A one written to any bit overrides the result of the comparison for the corresponding bit of the counter and compare register (causes the comparison of the register bits to be always true)."]
pub type RimaskW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Mask register. This register holds the 32-bit mask value. A one written to any bit overrides the result of the comparison for the corresponding bit of the counter and compare register (causes the comparison of the register bits to be always true)."]
    #[inline(always)]
    pub fn rimask(&self) -> RimaskR {
        RimaskR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask register. This register holds the 32-bit mask value. A one written to any bit overrides the result of the comparison for the corresponding bit of the counter and compare register (causes the comparison of the register bits to be always true)."]
    #[inline(always)]
    #[must_use]
    pub fn rimask(&mut self) -> RimaskW<MaskSpec> {
        RimaskW::new(self, 0)
    }
}
#[doc = "Mask register. This register holds the 32-bit mask value. A 1 written to any bit will force a compare on the corresponding bit of the counter and compare register.\n\nYou can [`read`](crate::Reg::read) this register and get [`mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MaskSpec;
impl crate::RegisterSpec for MaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mask::R`](R) reader structure"]
impl crate::Readable for MaskSpec {}
#[doc = "`write(|w| ..)` method takes [`mask::W`](W) writer structure"]
impl crate::Writable for MaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MASK to value 0"]
impl crate::Resettable for MaskSpec {
    const RESET_VALUE: u32 = 0;
}
