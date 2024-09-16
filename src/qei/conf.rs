#[doc = "Register `CONF` reader"]
pub type R = crate::R<ConfSpec>;
#[doc = "Register `CONF` writer"]
pub type W = crate::W<ConfSpec>;
#[doc = "Field `DIRINV` reader - Direction invert. When 1, complements the DIR bit."]
pub type DirinvR = crate::BitReader;
#[doc = "Field `DIRINV` writer - Direction invert. When 1, complements the DIR bit."]
pub type DirinvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIGMODE` reader - Signal Mode. When 0, PhA and PhB function as quadrature encoder inputs. When 1, PhA functions as the direction signal and PhB functions as the clock signal."]
pub type SigmodeR = crate::BitReader;
#[doc = "Field `SIGMODE` writer - Signal Mode. When 0, PhA and PhB function as quadrature encoder inputs. When 1, PhA functions as the direction signal and PhB functions as the clock signal."]
pub type SigmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPMODE` reader - Capture Mode. When 0, only PhA edges are counted (2X). When 1, BOTH PhA and PhB edges are counted (4X), increasing resolution but decreasing range."]
pub type CapmodeR = crate::BitReader;
#[doc = "Field `CAPMODE` writer - Capture Mode. When 0, only PhA edges are counted (2X). When 1, BOTH PhA and PhB edges are counted (4X), increasing resolution but decreasing range."]
pub type CapmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INVINX` reader - Invert Index. When 1, inverts the sense of the index input."]
pub type InvinxR = crate::BitReader;
#[doc = "Field `INVINX` writer - Invert Index. When 1, inverts the sense of the index input."]
pub type InvinxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRESPI` reader - Continuously reset the position counter on index. When 1, resets the position counter to all zeros whenever an index pulse occurs after the next position increase (recalibration)."]
pub type CrespiR = crate::BitReader;
#[doc = "Field `CRESPI` writer - Continuously reset the position counter on index. When 1, resets the position counter to all zeros whenever an index pulse occurs after the next position increase (recalibration)."]
pub type CrespiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INXGATE` reader - Index gating configuration: When INXGATE\\[16\\]
= 1, pass the index when PHA = 1 and PHB = 0, otherwise block index. When INXGATE\\[17\\]
= 1, pass the index when PHA = 1 and PHB = 1, otherwise block index. When INXGATE\\[18\\]
= 1, pass the index when PHA = 0 and PHB = 1, otherwise block index. When INXGATE\\[19\\]
= 1, pass the index when PHA = 0 and PHB = 0, otherwise block index."]
pub type InxgateR = crate::FieldReader;
#[doc = "Field `INXGATE` writer - Index gating configuration: When INXGATE\\[16\\]
= 1, pass the index when PHA = 1 and PHB = 0, otherwise block index. When INXGATE\\[17\\]
= 1, pass the index when PHA = 1 and PHB = 1, otherwise block index. When INXGATE\\[18\\]
= 1, pass the index when PHA = 0 and PHB = 1, otherwise block index. When INXGATE\\[19\\]
= 1, pass the index when PHA = 0 and PHB = 0, otherwise block index."]
pub type InxgateW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Direction invert. When 1, complements the DIR bit."]
    #[inline(always)]
    pub fn dirinv(&self) -> DirinvR {
        DirinvR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Signal Mode. When 0, PhA and PhB function as quadrature encoder inputs. When 1, PhA functions as the direction signal and PhB functions as the clock signal."]
    #[inline(always)]
    pub fn sigmode(&self) -> SigmodeR {
        SigmodeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Capture Mode. When 0, only PhA edges are counted (2X). When 1, BOTH PhA and PhB edges are counted (4X), increasing resolution but decreasing range."]
    #[inline(always)]
    pub fn capmode(&self) -> CapmodeR {
        CapmodeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Invert Index. When 1, inverts the sense of the index input."]
    #[inline(always)]
    pub fn invinx(&self) -> InvinxR {
        InvinxR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Continuously reset the position counter on index. When 1, resets the position counter to all zeros whenever an index pulse occurs after the next position increase (recalibration)."]
    #[inline(always)]
    pub fn crespi(&self) -> CrespiR {
        CrespiR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Index gating configuration: When INXGATE\\[16\\]
= 1, pass the index when PHA = 1 and PHB = 0, otherwise block index. When INXGATE\\[17\\]
= 1, pass the index when PHA = 1 and PHB = 1, otherwise block index. When INXGATE\\[18\\]
= 1, pass the index when PHA = 0 and PHB = 1, otherwise block index. When INXGATE\\[19\\]
= 1, pass the index when PHA = 0 and PHB = 0, otherwise block index."]
    #[inline(always)]
    pub fn inxgate(&self) -> InxgateR {
        InxgateR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Direction invert. When 1, complements the DIR bit."]
    #[inline(always)]
    #[must_use]
    pub fn dirinv(&mut self) -> DirinvW<ConfSpec> {
        DirinvW::new(self, 0)
    }
    #[doc = "Bit 1 - Signal Mode. When 0, PhA and PhB function as quadrature encoder inputs. When 1, PhA functions as the direction signal and PhB functions as the clock signal."]
    #[inline(always)]
    #[must_use]
    pub fn sigmode(&mut self) -> SigmodeW<ConfSpec> {
        SigmodeW::new(self, 1)
    }
    #[doc = "Bit 2 - Capture Mode. When 0, only PhA edges are counted (2X). When 1, BOTH PhA and PhB edges are counted (4X), increasing resolution but decreasing range."]
    #[inline(always)]
    #[must_use]
    pub fn capmode(&mut self) -> CapmodeW<ConfSpec> {
        CapmodeW::new(self, 2)
    }
    #[doc = "Bit 3 - Invert Index. When 1, inverts the sense of the index input."]
    #[inline(always)]
    #[must_use]
    pub fn invinx(&mut self) -> InvinxW<ConfSpec> {
        InvinxW::new(self, 3)
    }
    #[doc = "Bit 4 - Continuously reset the position counter on index. When 1, resets the position counter to all zeros whenever an index pulse occurs after the next position increase (recalibration)."]
    #[inline(always)]
    #[must_use]
    pub fn crespi(&mut self) -> CrespiW<ConfSpec> {
        CrespiW::new(self, 4)
    }
    #[doc = "Bits 16:19 - Index gating configuration: When INXGATE\\[16\\]
= 1, pass the index when PHA = 1 and PHB = 0, otherwise block index. When INXGATE\\[17\\]
= 1, pass the index when PHA = 1 and PHB = 1, otherwise block index. When INXGATE\\[18\\]
= 1, pass the index when PHA = 0 and PHB = 1, otherwise block index. When INXGATE\\[19\\]
= 1, pass the index when PHA = 0 and PHB = 0, otherwise block index."]
    #[inline(always)]
    #[must_use]
    pub fn inxgate(&mut self) -> InxgateW<ConfSpec> {
        InxgateW::new(self, 16)
    }
}
#[doc = "Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfSpec;
impl crate::RegisterSpec for ConfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf::R`](R) reader structure"]
impl crate::Readable for ConfSpec {}
#[doc = "`write(|w| ..)` method takes [`conf::W`](W) writer structure"]
impl crate::Writable for ConfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONF to value 0"]
impl crate::Resettable for ConfSpec {
    const RESET_VALUE: u32 = 0;
}
