#[doc = "Register `RS485DLY` reader"]
pub type R = crate::R<Rs485dlySpec>;
#[doc = "Register `RS485DLY` writer"]
pub type W = crate::W<Rs485dlySpec>;
#[doc = "Field `DLY` reader - Contains the direction control (UnOE) delay value. This register works in conjunction with an 8-bit counter."]
pub type DlyR = crate::FieldReader;
#[doc = "Field `DLY` writer - Contains the direction control (UnOE) delay value. This register works in conjunction with an 8-bit counter."]
pub type DlyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Contains the direction control (UnOE) delay value. This register works in conjunction with an 8-bit counter."]
    #[inline(always)]
    pub fn dly(&self) -> DlyR {
        DlyR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Contains the direction control (UnOE) delay value. This register works in conjunction with an 8-bit counter."]
    #[inline(always)]
    #[must_use]
    pub fn dly(&mut self) -> DlyW<Rs485dlySpec> {
        DlyW::new(self, 0)
    }
}
#[doc = "RS-485/EIA-485 direction control delay.\n\nYou can [`read`](crate::Reg::read) this register and get [`rs485dly::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rs485dly::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rs485dlySpec;
impl crate::RegisterSpec for Rs485dlySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rs485dly::R`](R) reader structure"]
impl crate::Readable for Rs485dlySpec {}
#[doc = "`write(|w| ..)` method takes [`rs485dly::W`](W) writer structure"]
impl crate::Writable for Rs485dlySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RS485DLY to value 0"]
impl crate::Resettable for Rs485dlySpec {
    const RESET_VALUE: u32 = 0;
}
