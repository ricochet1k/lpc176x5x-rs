#[doc = "Register `INTTCSTAT` reader"]
pub type R = crate::R<InttcstatSpec>;
#[doc = "Field `INTTCSTAT0` reader - Terminal count interrupt request status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
pub type Inttcstat0R = crate::BitReader;
#[doc = "Field `INTTCSTAT1` reader - Terminal count interrupt request status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
pub type Inttcstat1R = crate::BitReader;
#[doc = "Field `INTTCSTAT2` reader - Terminal count interrupt request status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
pub type Inttcstat2R = crate::BitReader;
#[doc = "Field `INTTCSTAT3` reader - Terminal count interrupt request status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
pub type Inttcstat3R = crate::BitReader;
#[doc = "Field `INTTCSTAT4` reader - Terminal count interrupt request status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
pub type Inttcstat4R = crate::BitReader;
#[doc = "Field `INTTCSTAT5` reader - Terminal count interrupt request status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
pub type Inttcstat5R = crate::BitReader;
#[doc = "Field `INTTCSTAT6` reader - Terminal count interrupt request status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
pub type Inttcstat6R = crate::BitReader;
#[doc = "Field `INTTCSTAT7` reader - Terminal count interrupt request status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
pub type Inttcstat7R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Terminal count interrupt request status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
    #[inline(always)]
    pub fn inttcstat0(&self) -> Inttcstat0R {
        Inttcstat0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Terminal count interrupt request status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
    #[inline(always)]
    pub fn inttcstat1(&self) -> Inttcstat1R {
        Inttcstat1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Terminal count interrupt request status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
    #[inline(always)]
    pub fn inttcstat2(&self) -> Inttcstat2R {
        Inttcstat2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Terminal count interrupt request status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
    #[inline(always)]
    pub fn inttcstat3(&self) -> Inttcstat3R {
        Inttcstat3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Terminal count interrupt request status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
    #[inline(always)]
    pub fn inttcstat4(&self) -> Inttcstat4R {
        Inttcstat4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Terminal count interrupt request status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
    #[inline(always)]
    pub fn inttcstat5(&self) -> Inttcstat5R {
        Inttcstat5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Terminal count interrupt request status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
    #[inline(always)]
    pub fn inttcstat6(&self) -> Inttcstat6R {
        Inttcstat6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Terminal count interrupt request status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
    #[inline(always)]
    pub fn inttcstat7(&self) -> Inttcstat7R {
        Inttcstat7R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "DMA Interrupt Terminal Count Request Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`inttcstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InttcstatSpec;
impl crate::RegisterSpec for InttcstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inttcstat::R`](R) reader structure"]
impl crate::Readable for InttcstatSpec {}
#[doc = "`reset()` method sets INTTCSTAT to value 0"]
impl crate::Resettable for InttcstatSpec {
    const RESET_VALUE: u32 = 0;
}
