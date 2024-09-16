#[doc = "Register `RAWINTTCSTAT` reader"]
pub type R = crate::R<RawinttcstatSpec>;
#[doc = "Field `RAWINTTCSTAT0` reader - Status of the terminal count interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
pub type Rawinttcstat0R = crate::BitReader;
#[doc = "Field `RAWINTTCSTAT1` reader - Status of the terminal count interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
pub type Rawinttcstat1R = crate::BitReader;
#[doc = "Field `RAWINTTCSTAT2` reader - Status of the terminal count interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
pub type Rawinttcstat2R = crate::BitReader;
#[doc = "Field `RAWINTTCSTAT3` reader - Status of the terminal count interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
pub type Rawinttcstat3R = crate::BitReader;
#[doc = "Field `RAWINTTCSTAT4` reader - Status of the terminal count interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
pub type Rawinttcstat4R = crate::BitReader;
#[doc = "Field `RAWINTTCSTAT5` reader - Status of the terminal count interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
pub type Rawinttcstat5R = crate::BitReader;
#[doc = "Field `RAWINTTCSTAT6` reader - Status of the terminal count interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
pub type Rawinttcstat6R = crate::BitReader;
#[doc = "Field `RAWINTTCSTAT7` reader - Status of the terminal count interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
pub type Rawinttcstat7R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Status of the terminal count interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
    #[inline(always)]
    pub fn rawinttcstat0(&self) -> Rawinttcstat0R {
        Rawinttcstat0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Status of the terminal count interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
    #[inline(always)]
    pub fn rawinttcstat1(&self) -> Rawinttcstat1R {
        Rawinttcstat1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Status of the terminal count interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
    #[inline(always)]
    pub fn rawinttcstat2(&self) -> Rawinttcstat2R {
        Rawinttcstat2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Status of the terminal count interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
    #[inline(always)]
    pub fn rawinttcstat3(&self) -> Rawinttcstat3R {
        Rawinttcstat3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Status of the terminal count interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
    #[inline(always)]
    pub fn rawinttcstat4(&self) -> Rawinttcstat4R {
        Rawinttcstat4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Status of the terminal count interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
    #[inline(always)]
    pub fn rawinttcstat5(&self) -> Rawinttcstat5R {
        Rawinttcstat5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Status of the terminal count interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
    #[inline(always)]
    pub fn rawinttcstat6(&self) -> Rawinttcstat6R {
        Rawinttcstat6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Status of the terminal count interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
    #[inline(always)]
    pub fn rawinttcstat7(&self) -> Rawinttcstat7R {
        Rawinttcstat7R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "DMA Raw Interrupt Terminal Count Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rawinttcstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RawinttcstatSpec;
impl crate::RegisterSpec for RawinttcstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rawinttcstat::R`](R) reader structure"]
impl crate::Readable for RawinttcstatSpec {}
#[doc = "`reset()` method sets RAWINTTCSTAT to value 0"]
impl crate::Resettable for RawinttcstatSpec {
    const RESET_VALUE: u32 = 0;
}
