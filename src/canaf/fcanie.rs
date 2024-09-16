#[doc = "Register `FCANIE` reader"]
pub type R = crate::R<FcanieSpec>;
#[doc = "Register `FCANIE` writer"]
pub type W = crate::W<FcanieSpec>;
#[doc = "Field `FCANIE` reader - Global FullCAN Interrupt Enable. When 1, this interrupt is enabled."]
pub type FcanieR = crate::BitReader;
#[doc = "Field `FCANIE` writer - Global FullCAN Interrupt Enable. When 1, this interrupt is enabled."]
pub type FcanieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Global FullCAN Interrupt Enable. When 1, this interrupt is enabled."]
    #[inline(always)]
    pub fn fcanie(&self) -> FcanieR {
        FcanieR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Global FullCAN Interrupt Enable. When 1, this interrupt is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn fcanie(&mut self) -> FcanieW<FcanieSpec> {
        FcanieW::new(self, 0)
    }
}
#[doc = "FullCAN interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`fcanie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcanie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcanieSpec;
impl crate::RegisterSpec for FcanieSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcanie::R`](R) reader structure"]
impl crate::Readable for FcanieSpec {}
#[doc = "`write(|w| ..)` method takes [`fcanie::W`](W) writer structure"]
impl crate::Writable for FcanieSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FCANIE to value 0"]
impl crate::Resettable for FcanieSpec {
    const RESET_VALUE: u32 = 0;
}
