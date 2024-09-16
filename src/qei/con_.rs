#[doc = "Register `CON` writer"]
pub type W = crate::W<ConSpec>;
#[doc = "Field `RESP` writer - Reset position counter. When set = 1, resets the position counter to all zeros. Autoclears when the position counter is cleared."]
pub type RespW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESPI` writer - Reset position counter on index. When set = 1, resets the position counter to all zeros once only the first time an index pulse occurs. Autoclears when the position counter is cleared."]
pub type RespiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESV` writer - Reset velocity. When set = 1, resets the velocity counter to all zeros, reloads the velocity timer, and presets the velocity compare register. Autoclears when the velocity counter is cleared."]
pub type ResvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESI` writer - Reset index counter. When set = 1, resets the index counter to all zeros. Autoclears when the index counter is cleared."]
pub type ResiW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Reset position counter. When set = 1, resets the position counter to all zeros. Autoclears when the position counter is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn resp(&mut self) -> RespW<ConSpec> {
        RespW::new(self, 0)
    }
    #[doc = "Bit 1 - Reset position counter on index. When set = 1, resets the position counter to all zeros once only the first time an index pulse occurs. Autoclears when the position counter is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn respi(&mut self) -> RespiW<ConSpec> {
        RespiW::new(self, 1)
    }
    #[doc = "Bit 2 - Reset velocity. When set = 1, resets the velocity counter to all zeros, reloads the velocity timer, and presets the velocity compare register. Autoclears when the velocity counter is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn resv(&mut self) -> ResvW<ConSpec> {
        ResvW::new(self, 2)
    }
    #[doc = "Bit 3 - Reset index counter. When set = 1, resets the index counter to all zeros. Autoclears when the index counter is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn resi(&mut self) -> ResiW<ConSpec> {
        ResiW::new(self, 3)
    }
}
#[doc = "Control register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`con::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConSpec;
impl crate::RegisterSpec for ConSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`con::W`](W) writer structure"]
impl crate::Writable for ConSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CON to value 0"]
impl crate::Resettable for ConSpec {
    const RESET_VALUE: u32 = 0;
}
