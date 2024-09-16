#[doc = "Register `MCMD` reader"]
pub type R = crate::R<McmdSpec>;
#[doc = "Register `MCMD` writer"]
pub type W = crate::W<McmdSpec>;
#[doc = "Field `READ` reader - This bit causes the MII Management hardware to perform a single Read cycle. The Read data is returned in Register MRDD (MII Mgmt Read Data)."]
pub type ReadR = crate::BitReader;
#[doc = "Field `READ` writer - This bit causes the MII Management hardware to perform a single Read cycle. The Read data is returned in Register MRDD (MII Mgmt Read Data)."]
pub type ReadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCAN` reader - This bit causes the MII Management hardware to perform Read cycles continuously. This is useful for monitoring Link Fail for example."]
pub type ScanR = crate::BitReader;
#[doc = "Field `SCAN` writer - This bit causes the MII Management hardware to perform Read cycles continuously. This is useful for monitoring Link Fail for example."]
pub type ScanW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This bit causes the MII Management hardware to perform a single Read cycle. The Read data is returned in Register MRDD (MII Mgmt Read Data)."]
    #[inline(always)]
    pub fn read(&self) -> ReadR {
        ReadR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit causes the MII Management hardware to perform Read cycles continuously. This is useful for monitoring Link Fail for example."]
    #[inline(always)]
    pub fn scan(&self) -> ScanR {
        ScanR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit causes the MII Management hardware to perform a single Read cycle. The Read data is returned in Register MRDD (MII Mgmt Read Data)."]
    #[inline(always)]
    #[must_use]
    pub fn read(&mut self) -> ReadW<McmdSpec> {
        ReadW::new(self, 0)
    }
    #[doc = "Bit 1 - This bit causes the MII Management hardware to perform Read cycles continuously. This is useful for monitoring Link Fail for example."]
    #[inline(always)]
    #[must_use]
    pub fn scan(&mut self) -> ScanW<McmdSpec> {
        ScanW::new(self, 1)
    }
}
#[doc = "MII Mgmt Command register.\n\nYou can [`read`](crate::Reg::read) this register and get [`mcmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McmdSpec;
impl crate::RegisterSpec for McmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcmd::R`](R) reader structure"]
impl crate::Readable for McmdSpec {}
#[doc = "`write(|w| ..)` method takes [`mcmd::W`](W) writer structure"]
impl crate::Writable for McmdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCMD to value 0"]
impl crate::Resettable for McmdSpec {
    const RESET_VALUE: u32 = 0;
}
