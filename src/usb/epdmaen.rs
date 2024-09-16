#[doc = "Register `EPDMAEN` writer"]
pub type W = crate::W<EpdmaenSpec>;
#[doc = "Field `EP_DMA_EN0` writer - Control endpoint OUT (DMA cannot be enabled for this endpoint and the EP0_DMA_ENABLE bit value must be 0)."]
pub type EpDmaEn0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP_DMA_EN1` writer - Control endpoint IN (DMA cannot be enabled for this endpoint and the EP1_DMA_ENABLE bit must be 0)."]
pub type EpDmaEn1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP_DMA_EN` writer - Endpoint xx(2 &lt;= xx &lt;= 31) DMA enable control bit. 0 = No effect. 1 = Enable the DMA operation for endpoint EPxx."]
pub type EpDmaEnW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl W {
    #[doc = "Bit 0 - Control endpoint OUT (DMA cannot be enabled for this endpoint and the EP0_DMA_ENABLE bit value must be 0)."]
    #[inline(always)]
    #[must_use]
    pub fn ep_dma_en0(&mut self) -> EpDmaEn0W<EpdmaenSpec> {
        EpDmaEn0W::new(self, 0)
    }
    #[doc = "Bit 1 - Control endpoint IN (DMA cannot be enabled for this endpoint and the EP1_DMA_ENABLE bit must be 0)."]
    #[inline(always)]
    #[must_use]
    pub fn ep_dma_en1(&mut self) -> EpDmaEn1W<EpdmaenSpec> {
        EpDmaEn1W::new(self, 1)
    }
    #[doc = "Bits 2:31 - Endpoint xx(2 &lt;= xx &lt;= 31) DMA enable control bit. 0 = No effect. 1 = Enable the DMA operation for endpoint EPxx."]
    #[inline(always)]
    #[must_use]
    pub fn ep_dma_en(&mut self) -> EpDmaEnW<EpdmaenSpec> {
        EpDmaEnW::new(self, 2)
    }
}
#[doc = "USB Endpoint DMA Enable\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epdmaen::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EpdmaenSpec;
impl crate::RegisterSpec for EpdmaenSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`epdmaen::W`](W) writer structure"]
impl crate::Writable for EpdmaenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EPDMAEN to value 0"]
impl crate::Resettable for EpdmaenSpec {
    const RESET_VALUE: u32 = 0;
}
