#[doc = "Register `RXSTATUS` reader"]
pub type R = crate::R<RxstatusSpec>;
#[doc = "Register `RXSTATUS` writer"]
pub type W = crate::W<RxstatusSpec>;
#[doc = "Field `RXSTATUS` reader - MSBs of receive status base address."]
pub type RxstatusR = crate::FieldReader<u32>;
#[doc = "Field `RXSTATUS` writer - MSBs of receive status base address."]
pub type RxstatusW<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 3:31 - MSBs of receive status base address."]
    #[inline(always)]
    pub fn rxstatus(&self) -> RxstatusR {
        RxstatusR::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 3:31 - MSBs of receive status base address."]
    #[inline(always)]
    #[must_use]
    pub fn rxstatus(&mut self) -> RxstatusW<RxstatusSpec> {
        RxstatusW::new(self, 3)
    }
}
#[doc = "Receive status base address register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rxstatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxstatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxstatusSpec;
impl crate::RegisterSpec for RxstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxstatus::R`](R) reader structure"]
impl crate::Readable for RxstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`rxstatus::W`](W) writer structure"]
impl crate::Writable for RxstatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXSTATUS to value 0"]
impl crate::Resettable for RxstatusSpec {
    const RESET_VALUE: u32 = 0;
}
