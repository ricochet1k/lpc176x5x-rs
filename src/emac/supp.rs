#[doc = "Register `SUPP` reader"]
pub type R = crate::R<SuppSpec>;
#[doc = "Register `SUPP` writer"]
pub type W = crate::W<SuppSpec>;
#[doc = "Field `SPEED` reader - This bit configures the Reduced MII logic for the current operating speed. When set, 100 Mbps mode is selected. When cleared, 10 Mbps mode is selected."]
pub type SpeedR = crate::BitReader;
#[doc = "Field `SPEED` writer - This bit configures the Reduced MII logic for the current operating speed. When set, 100 Mbps mode is selected. When cleared, 10 Mbps mode is selected."]
pub type SpeedW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 8 - This bit configures the Reduced MII logic for the current operating speed. When set, 100 Mbps mode is selected. When cleared, 10 Mbps mode is selected."]
    #[inline(always)]
    pub fn speed(&self) -> SpeedR {
        SpeedR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - This bit configures the Reduced MII logic for the current operating speed. When set, 100 Mbps mode is selected. When cleared, 10 Mbps mode is selected."]
    #[inline(always)]
    #[must_use]
    pub fn speed(&mut self) -> SpeedW<SuppSpec> {
        SpeedW::new(self, 8)
    }
}
#[doc = "PHY Support register.\n\nYou can [`read`](crate::Reg::read) this register and get [`supp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`supp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SuppSpec;
impl crate::RegisterSpec for SuppSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`supp::R`](R) reader structure"]
impl crate::Readable for SuppSpec {}
#[doc = "`write(|w| ..)` method takes [`supp::W`](W) writer structure"]
impl crate::Writable for SuppSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SUPP to value 0"]
impl crate::Resettable for SuppSpec {
    const RESET_VALUE: u32 = 0;
}
