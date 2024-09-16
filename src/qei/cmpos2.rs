#[doc = "Register `CMPOS2` reader"]
pub type R = crate::R<Cmpos2Spec>;
#[doc = "Register `CMPOS2` writer"]
pub type W = crate::W<Cmpos2Spec>;
#[doc = "Field `PCMP2` reader - Position compare value 2."]
pub type Pcmp2R = crate::FieldReader<u32>;
#[doc = "Field `PCMP2` writer - Position compare value 2."]
pub type Pcmp2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Position compare value 2."]
    #[inline(always)]
    pub fn pcmp2(&self) -> Pcmp2R {
        Pcmp2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Position compare value 2."]
    #[inline(always)]
    #[must_use]
    pub fn pcmp2(&mut self) -> Pcmp2W<Cmpos2Spec> {
        Pcmp2W::new(self, 0)
    }
}
#[doc = "Position compare register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpos2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpos2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cmpos2Spec;
impl crate::RegisterSpec for Cmpos2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmpos2::R`](R) reader structure"]
impl crate::Readable for Cmpos2Spec {}
#[doc = "`write(|w| ..)` method takes [`cmpos2::W`](W) writer structure"]
impl crate::Writable for Cmpos2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMPOS2 to value 0xffff_ffff"]
impl crate::Resettable for Cmpos2Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
