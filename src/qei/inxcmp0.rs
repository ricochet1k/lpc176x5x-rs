#[doc = "Register `INXCMP0` reader"]
pub type R = crate::R<Inxcmp0Spec>;
#[doc = "Register `INXCMP0` writer"]
pub type W = crate::W<Inxcmp0Spec>;
#[doc = "Field `ICMP0` reader - Index compare value 0."]
pub type Icmp0R = crate::FieldReader<u32>;
#[doc = "Field `ICMP0` writer - Index compare value 0."]
pub type Icmp0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Index compare value 0."]
    #[inline(always)]
    pub fn icmp0(&self) -> Icmp0R {
        Icmp0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Index compare value 0."]
    #[inline(always)]
    #[must_use]
    pub fn icmp0(&mut self) -> Icmp0W<Inxcmp0Spec> {
        Icmp0W::new(self, 0)
    }
}
#[doc = "Index compare register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`inxcmp0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inxcmp0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Inxcmp0Spec;
impl crate::RegisterSpec for Inxcmp0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inxcmp0::R`](R) reader structure"]
impl crate::Readable for Inxcmp0Spec {}
#[doc = "`write(|w| ..)` method takes [`inxcmp0::W`](W) writer structure"]
impl crate::Writable for Inxcmp0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INXCMP0 to value 0xffff_ffff"]
impl crate::Resettable for Inxcmp0Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
