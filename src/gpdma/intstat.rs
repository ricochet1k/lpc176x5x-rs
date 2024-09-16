#[doc = "Register `INTSTAT` reader"]
pub type R = crate::R<IntstatSpec>;
#[doc = "Field `INTSTAT0` reader - Status of DMA channel interrupts after masking. Each bit represents one channel: 0 - the corresponding channel has no active interrupt request. 1 - the corresponding channel does have an active interrupt request."]
pub type Intstat0R = crate::BitReader;
#[doc = "Field `INTSTAT1` reader - Status of DMA channel interrupts after masking. Each bit represents one channel: 0 - the corresponding channel has no active interrupt request. 1 - the corresponding channel does have an active interrupt request."]
pub type Intstat1R = crate::BitReader;
#[doc = "Field `INTSTAT2` reader - Status of DMA channel interrupts after masking. Each bit represents one channel: 0 - the corresponding channel has no active interrupt request. 1 - the corresponding channel does have an active interrupt request."]
pub type Intstat2R = crate::BitReader;
#[doc = "Field `INTSTAT3` reader - Status of DMA channel interrupts after masking. Each bit represents one channel: 0 - the corresponding channel has no active interrupt request. 1 - the corresponding channel does have an active interrupt request."]
pub type Intstat3R = crate::BitReader;
#[doc = "Field `INTSTAT4` reader - Status of DMA channel interrupts after masking. Each bit represents one channel: 0 - the corresponding channel has no active interrupt request. 1 - the corresponding channel does have an active interrupt request."]
pub type Intstat4R = crate::BitReader;
#[doc = "Field `INTSTAT5` reader - Status of DMA channel interrupts after masking. Each bit represents one channel: 0 - the corresponding channel has no active interrupt request. 1 - the corresponding channel does have an active interrupt request."]
pub type Intstat5R = crate::BitReader;
#[doc = "Field `INTSTAT6` reader - Status of DMA channel interrupts after masking. Each bit represents one channel: 0 - the corresponding channel has no active interrupt request. 1 - the corresponding channel does have an active interrupt request."]
pub type Intstat6R = crate::BitReader;
#[doc = "Field `INTSTAT7` reader - Status of DMA channel interrupts after masking. Each bit represents one channel: 0 - the corresponding channel has no active interrupt request. 1 - the corresponding channel does have an active interrupt request."]
pub type Intstat7R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Status of DMA channel interrupts after masking. Each bit represents one channel: 0 - the corresponding channel has no active interrupt request. 1 - the corresponding channel does have an active interrupt request."]
    #[inline(always)]
    pub fn intstat0(&self) -> Intstat0R {
        Intstat0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Status of DMA channel interrupts after masking. Each bit represents one channel: 0 - the corresponding channel has no active interrupt request. 1 - the corresponding channel does have an active interrupt request."]
    #[inline(always)]
    pub fn intstat1(&self) -> Intstat1R {
        Intstat1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Status of DMA channel interrupts after masking. Each bit represents one channel: 0 - the corresponding channel has no active interrupt request. 1 - the corresponding channel does have an active interrupt request."]
    #[inline(always)]
    pub fn intstat2(&self) -> Intstat2R {
        Intstat2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Status of DMA channel interrupts after masking. Each bit represents one channel: 0 - the corresponding channel has no active interrupt request. 1 - the corresponding channel does have an active interrupt request."]
    #[inline(always)]
    pub fn intstat3(&self) -> Intstat3R {
        Intstat3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Status of DMA channel interrupts after masking. Each bit represents one channel: 0 - the corresponding channel has no active interrupt request. 1 - the corresponding channel does have an active interrupt request."]
    #[inline(always)]
    pub fn intstat4(&self) -> Intstat4R {
        Intstat4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Status of DMA channel interrupts after masking. Each bit represents one channel: 0 - the corresponding channel has no active interrupt request. 1 - the corresponding channel does have an active interrupt request."]
    #[inline(always)]
    pub fn intstat5(&self) -> Intstat5R {
        Intstat5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Status of DMA channel interrupts after masking. Each bit represents one channel: 0 - the corresponding channel has no active interrupt request. 1 - the corresponding channel does have an active interrupt request."]
    #[inline(always)]
    pub fn intstat6(&self) -> Intstat6R {
        Intstat6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Status of DMA channel interrupts after masking. Each bit represents one channel: 0 - the corresponding channel has no active interrupt request. 1 - the corresponding channel does have an active interrupt request."]
    #[inline(always)]
    pub fn intstat7(&self) -> Intstat7R {
        Intstat7R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "DMA Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntstatSpec;
impl crate::RegisterSpec for IntstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intstat::R`](R) reader structure"]
impl crate::Readable for IntstatSpec {}
#[doc = "`reset()` method sets INTSTAT to value 0"]
impl crate::Resettable for IntstatSpec {
    const RESET_VALUE: u32 = 0;
}
