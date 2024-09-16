#[doc = "Register `MASK[%s]` reader"]
pub type R = crate::R<MaskSpec>;
#[doc = "Register `MASK[%s]` writer"]
pub type W = crate::W<MaskSpec>;
#[doc = "Field `MASK` reader - CAN AF RAM mask"]
pub type MaskR = crate::FieldReader<u32>;
#[doc = "Field `MASK` writer - CAN AF RAM mask"]
pub type MaskW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CAN AF RAM mask"]
    #[inline(always)]
    pub fn mask(&self) -> MaskR {
        MaskR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CAN AF RAM mask"]
    #[inline(always)]
    #[must_use]
    pub fn mask(&mut self) -> MaskW<MaskSpec> {
        MaskW::new(self, 0)
    }
}
#[doc = "CAN AF ram access register\n\nYou can [`read`](crate::Reg::read) this register and get [`mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets MASK[%s]
to value 0"]
impl crate::Resettable for MaskSpec {
    const RESET_VALUE: u32 = 0;
}
