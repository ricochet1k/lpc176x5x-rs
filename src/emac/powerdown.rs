#[doc = "Register `POWERDOWN` reader"]
pub type R = crate::R<PowerdownSpec>;
#[doc = "Register `POWERDOWN` writer"]
pub type W = crate::W<PowerdownSpec>;
#[doc = "Field `PD` reader - PowerDownMACAHB. If true, all AHB accesses will return a read/write error, except accesses to the Power-Down register."]
pub type PdR = crate::BitReader;
#[doc = "Field `PD` writer - PowerDownMACAHB. If true, all AHB accesses will return a read/write error, except accesses to the Power-Down register."]
pub type PdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - PowerDownMACAHB. If true, all AHB accesses will return a read/write error, except accesses to the Power-Down register."]
    #[inline(always)]
    pub fn pd(&self) -> PdR {
        PdR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - PowerDownMACAHB. If true, all AHB accesses will return a read/write error, except accesses to the Power-Down register."]
    #[inline(always)]
    #[must_use]
    pub fn pd(&mut self) -> PdW<PowerdownSpec> {
        PdW::new(self, 31)
    }
}
#[doc = "Power-down register.\n\nYou can [`read`](crate::Reg::read) this register and get [`powerdown::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`powerdown::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PowerdownSpec;
impl crate::RegisterSpec for PowerdownSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`powerdown::R`](R) reader structure"]
impl crate::Readable for PowerdownSpec {}
#[doc = "`write(|w| ..)` method takes [`powerdown::W`](W) writer structure"]
impl crate::Writable for PowerdownSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets POWERDOWN to value 0"]
impl crate::Resettable for PowerdownSpec {
    const RESET_VALUE: u32 = 0;
}
