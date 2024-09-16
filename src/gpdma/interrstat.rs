#[doc = "Register `INTERRSTAT` reader"]
pub type R = crate::R<InterrstatSpec>;
#[doc = "Field `INTERRSTAT0` reader - Interrupt error status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
pub type Interrstat0R = crate::BitReader;
#[doc = "Field `INTERRSTAT1` reader - Interrupt error status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
pub type Interrstat1R = crate::BitReader;
#[doc = "Field `INTERRSTAT2` reader - Interrupt error status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
pub type Interrstat2R = crate::BitReader;
#[doc = "Field `INTERRSTAT3` reader - Interrupt error status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
pub type Interrstat3R = crate::BitReader;
#[doc = "Field `INTERRSTAT4` reader - Interrupt error status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
pub type Interrstat4R = crate::BitReader;
#[doc = "Field `INTERRSTAT5` reader - Interrupt error status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
pub type Interrstat5R = crate::BitReader;
#[doc = "Field `INTERRSTAT6` reader - Interrupt error status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
pub type Interrstat6R = crate::BitReader;
#[doc = "Field `INTERRSTAT7` reader - Interrupt error status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
pub type Interrstat7R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Interrupt error status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn interrstat0(&self) -> Interrstat0R {
        Interrstat0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt error status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn interrstat1(&self) -> Interrstat1R {
        Interrstat1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt error status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn interrstat2(&self) -> Interrstat2R {
        Interrstat2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt error status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn interrstat3(&self) -> Interrstat3R {
        Interrstat3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt error status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn interrstat4(&self) -> Interrstat4R {
        Interrstat4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt error status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn interrstat5(&self) -> Interrstat5R {
        Interrstat5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt error status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn interrstat6(&self) -> Interrstat6R {
        Interrstat6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt error status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn interrstat7(&self) -> Interrstat7R {
        Interrstat7R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "DMA Interrupt Error Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`interrstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InterrstatSpec;
impl crate::RegisterSpec for InterrstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interrstat::R`](R) reader structure"]
impl crate::Readable for InterrstatSpec {}
#[doc = "`reset()` method sets INTERRSTAT to value 0"]
impl crate::Resettable for InterrstatSpec {
    const RESET_VALUE: u32 = 0;
}
