#[doc = "Register `DMACREQSEL` reader"]
pub type R = crate::R<DmacreqselSpec>;
#[doc = "Register `DMACREQSEL` writer"]
pub type W = crate::W<DmacreqselSpec>;
#[doc = "Field `DMASEL08` reader - Selects the DMA request for GPDMA input 8: 0 - uart0 tx 1 - Timer 0 match 0 is selected."]
pub type Dmasel08R = crate::BitReader;
#[doc = "Field `DMASEL08` writer - Selects the DMA request for GPDMA input 8: 0 - uart0 tx 1 - Timer 0 match 0 is selected."]
pub type Dmasel08W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMASEL09` reader - Selects the DMA request for GPDMA input 9: 0 - uart0 rx 1 - Timer 0 match 1 is selected."]
pub type Dmasel09R = crate::BitReader;
#[doc = "Field `DMASEL09` writer - Selects the DMA request for GPDMA input 9: 0 - uart0 rx 1 - Timer 0 match 1 is selected."]
pub type Dmasel09W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMASEL10` reader - Selects the DMA request for GPDMA input 10: 0 - uart1 tx is selected. 1 - Timer 1 match 0 is selected."]
pub type Dmasel10R = crate::BitReader;
#[doc = "Field `DMASEL10` writer - Selects the DMA request for GPDMA input 10: 0 - uart1 tx is selected. 1 - Timer 1 match 0 is selected."]
pub type Dmasel10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMASEL11` reader - Selects the DMA request for GPDMA input 11: 0 - uart1 rx is selected. 1 - Timer 1 match 1 is selected."]
pub type Dmasel11R = crate::BitReader;
#[doc = "Field `DMASEL11` writer - Selects the DMA request for GPDMA input 11: 0 - uart1 rx is selected. 1 - Timer 1 match 1 is selected."]
pub type Dmasel11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMASEL12` reader - Selects the DMA request for GPDMA input 12: 0 - uart2 tx is selected. 1 - Timer 2 match 0 is selected."]
pub type Dmasel12R = crate::BitReader;
#[doc = "Field `DMASEL12` writer - Selects the DMA request for GPDMA input 12: 0 - uart2 tx is selected. 1 - Timer 2 match 0 is selected."]
pub type Dmasel12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMASEL13` reader - Selects the DMA request for GPDMA input 13: 0 - uart2 rx is selected. 1 - Timer 2 match 1 is selected."]
pub type Dmasel13R = crate::BitReader;
#[doc = "Field `DMASEL13` writer - Selects the DMA request for GPDMA input 13: 0 - uart2 rx is selected. 1 - Timer 2 match 1 is selected."]
pub type Dmasel13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMASEL14` reader - Selects the DMA request for GPDMA input 14: 0 - uart3 tx is selected. 1 - I2S channel 0 is selected."]
pub type Dmasel14R = crate::BitReader;
#[doc = "Field `DMASEL14` writer - Selects the DMA request for GPDMA input 14: 0 - uart3 tx is selected. 1 - I2S channel 0 is selected."]
pub type Dmasel14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMASEL15` reader - Selects the DMA request for GPDMA input 15: 0 - uart3 rx is selected. 1 - I2S channel 1 is selected."]
pub type Dmasel15R = crate::BitReader;
#[doc = "Field `DMASEL15` writer - Selects the DMA request for GPDMA input 15: 0 - uart3 rx is selected. 1 - I2S channel 1 is selected."]
pub type Dmasel15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Selects the DMA request for GPDMA input 8: 0 - uart0 tx 1 - Timer 0 match 0 is selected."]
    #[inline(always)]
    pub fn dmasel08(&self) -> Dmasel08R {
        Dmasel08R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Selects the DMA request for GPDMA input 9: 0 - uart0 rx 1 - Timer 0 match 1 is selected."]
    #[inline(always)]
    pub fn dmasel09(&self) -> Dmasel09R {
        Dmasel09R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Selects the DMA request for GPDMA input 10: 0 - uart1 tx is selected. 1 - Timer 1 match 0 is selected."]
    #[inline(always)]
    pub fn dmasel10(&self) -> Dmasel10R {
        Dmasel10R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Selects the DMA request for GPDMA input 11: 0 - uart1 rx is selected. 1 - Timer 1 match 1 is selected."]
    #[inline(always)]
    pub fn dmasel11(&self) -> Dmasel11R {
        Dmasel11R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Selects the DMA request for GPDMA input 12: 0 - uart2 tx is selected. 1 - Timer 2 match 0 is selected."]
    #[inline(always)]
    pub fn dmasel12(&self) -> Dmasel12R {
        Dmasel12R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Selects the DMA request for GPDMA input 13: 0 - uart2 rx is selected. 1 - Timer 2 match 1 is selected."]
    #[inline(always)]
    pub fn dmasel13(&self) -> Dmasel13R {
        Dmasel13R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Selects the DMA request for GPDMA input 14: 0 - uart3 tx is selected. 1 - I2S channel 0 is selected."]
    #[inline(always)]
    pub fn dmasel14(&self) -> Dmasel14R {
        Dmasel14R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Selects the DMA request for GPDMA input 15: 0 - uart3 rx is selected. 1 - I2S channel 1 is selected."]
    #[inline(always)]
    pub fn dmasel15(&self) -> Dmasel15R {
        Dmasel15R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects the DMA request for GPDMA input 8: 0 - uart0 tx 1 - Timer 0 match 0 is selected."]
    #[inline(always)]
    #[must_use]
    pub fn dmasel08(&mut self) -> Dmasel08W<DmacreqselSpec> {
        Dmasel08W::new(self, 0)
    }
    #[doc = "Bit 1 - Selects the DMA request for GPDMA input 9: 0 - uart0 rx 1 - Timer 0 match 1 is selected."]
    #[inline(always)]
    #[must_use]
    pub fn dmasel09(&mut self) -> Dmasel09W<DmacreqselSpec> {
        Dmasel09W::new(self, 1)
    }
    #[doc = "Bit 2 - Selects the DMA request for GPDMA input 10: 0 - uart1 tx is selected. 1 - Timer 1 match 0 is selected."]
    #[inline(always)]
    #[must_use]
    pub fn dmasel10(&mut self) -> Dmasel10W<DmacreqselSpec> {
        Dmasel10W::new(self, 2)
    }
    #[doc = "Bit 3 - Selects the DMA request for GPDMA input 11: 0 - uart1 rx is selected. 1 - Timer 1 match 1 is selected."]
    #[inline(always)]
    #[must_use]
    pub fn dmasel11(&mut self) -> Dmasel11W<DmacreqselSpec> {
        Dmasel11W::new(self, 3)
    }
    #[doc = "Bit 4 - Selects the DMA request for GPDMA input 12: 0 - uart2 tx is selected. 1 - Timer 2 match 0 is selected."]
    #[inline(always)]
    #[must_use]
    pub fn dmasel12(&mut self) -> Dmasel12W<DmacreqselSpec> {
        Dmasel12W::new(self, 4)
    }
    #[doc = "Bit 5 - Selects the DMA request for GPDMA input 13: 0 - uart2 rx is selected. 1 - Timer 2 match 1 is selected."]
    #[inline(always)]
    #[must_use]
    pub fn dmasel13(&mut self) -> Dmasel13W<DmacreqselSpec> {
        Dmasel13W::new(self, 5)
    }
    #[doc = "Bit 6 - Selects the DMA request for GPDMA input 14: 0 - uart3 tx is selected. 1 - I2S channel 0 is selected."]
    #[inline(always)]
    #[must_use]
    pub fn dmasel14(&mut self) -> Dmasel14W<DmacreqselSpec> {
        Dmasel14W::new(self, 6)
    }
    #[doc = "Bit 7 - Selects the DMA request for GPDMA input 15: 0 - uart3 rx is selected. 1 - I2S channel 1 is selected."]
    #[inline(always)]
    #[must_use]
    pub fn dmasel15(&mut self) -> Dmasel15W<DmacreqselSpec> {
        Dmasel15W::new(self, 7)
    }
}
#[doc = "Selects between alternative requests on DMA channels 0 through 7 and 10 through 15\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacreqsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacreqsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacreqselSpec;
impl crate::RegisterSpec for DmacreqselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacreqsel::R`](R) reader structure"]
impl crate::Readable for DmacreqselSpec {}
#[doc = "`write(|w| ..)` method takes [`dmacreqsel::W`](W) writer structure"]
impl crate::Writable for DmacreqselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMACREQSEL to value 0"]
impl crate::Resettable for DmacreqselSpec {
    const RESET_VALUE: u32 = 0;
}
