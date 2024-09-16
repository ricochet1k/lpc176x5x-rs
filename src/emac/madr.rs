#[doc = "Register `MADR` reader"]
pub type R = crate::R<MadrSpec>;
#[doc = "Register `MADR` writer"]
pub type W = crate::W<MadrSpec>;
#[doc = "Field `REGADDR` reader - REGISTER ADDRESS. This field represents the 5-bit Register Address field of Mgmt cycles. Up to 32 registers can be accessed."]
pub type RegaddrR = crate::FieldReader;
#[doc = "Field `REGADDR` writer - REGISTER ADDRESS. This field represents the 5-bit Register Address field of Mgmt cycles. Up to 32 registers can be accessed."]
pub type RegaddrW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHYADDR` reader - PHY ADDRESS. This field represents the 5-bit PHY Address field of Mgmt cycles. Up to 31 PHYs can be addressed (0 is reserved)."]
pub type PhyaddrR = crate::FieldReader;
#[doc = "Field `PHYADDR` writer - PHY ADDRESS. This field represents the 5-bit PHY Address field of Mgmt cycles. Up to 31 PHYs can be addressed (0 is reserved)."]
pub type PhyaddrW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - REGISTER ADDRESS. This field represents the 5-bit Register Address field of Mgmt cycles. Up to 32 registers can be accessed."]
    #[inline(always)]
    pub fn regaddr(&self) -> RegaddrR {
        RegaddrR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - PHY ADDRESS. This field represents the 5-bit PHY Address field of Mgmt cycles. Up to 31 PHYs can be addressed (0 is reserved)."]
    #[inline(always)]
    pub fn phyaddr(&self) -> PhyaddrR {
        PhyaddrR::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - REGISTER ADDRESS. This field represents the 5-bit Register Address field of Mgmt cycles. Up to 32 registers can be accessed."]
    #[inline(always)]
    #[must_use]
    pub fn regaddr(&mut self) -> RegaddrW<MadrSpec> {
        RegaddrW::new(self, 0)
    }
    #[doc = "Bits 8:12 - PHY ADDRESS. This field represents the 5-bit PHY Address field of Mgmt cycles. Up to 31 PHYs can be addressed (0 is reserved)."]
    #[inline(always)]
    #[must_use]
    pub fn phyaddr(&mut self) -> PhyaddrW<MadrSpec> {
        PhyaddrW::new(self, 8)
    }
}
#[doc = "MII Mgmt Address register.\n\nYou can [`read`](crate::Reg::read) this register and get [`madr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`madr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MadrSpec;
impl crate::RegisterSpec for MadrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`madr::R`](R) reader structure"]
impl crate::Readable for MadrSpec {}
#[doc = "`write(|w| ..)` method takes [`madr::W`](W) writer structure"]
impl crate::Writable for MadrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MADR to value 0"]
impl crate::Resettable for MadrSpec {
    const RESET_VALUE: u32 = 0;
}
