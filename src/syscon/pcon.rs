#[doc = "Register `PCON` reader"]
pub type R = crate::R<PconSpec>;
#[doc = "Register `PCON` writer"]
pub type W = crate::W<PconSpec>;
#[doc = "Field `PM0` reader - Power mode control bit 0. This bit controls entry to the Power-down mode."]
pub type Pm0R = crate::BitReader;
#[doc = "Field `PM0` writer - Power mode control bit 0. This bit controls entry to the Power-down mode."]
pub type Pm0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PM1` reader - Power mode control bit 1. This bit controls entry to the Deep Power-down mode."]
pub type Pm1R = crate::BitReader;
#[doc = "Field `PM1` writer - Power mode control bit 1. This bit controls entry to the Deep Power-down mode."]
pub type Pm1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BODRPM` reader - Brown-Out Reduced Power Mode. When BODRPM is 1, the Brown-Out Detect circuitry will be turned off when chip Power-down mode or Deep Sleep mode is entered, resulting in a further reduction in power usage. However, the possibility of using Brown-Out Detect as a wake-up source from the reduced power mode will be lost. When 0, the Brown-Out Detect function remains active during Power-down and Deep Sleep modes. See the System Control Block chapter for details of Brown-Out detection."]
pub type BodrpmR = crate::BitReader;
#[doc = "Field `BODRPM` writer - Brown-Out Reduced Power Mode. When BODRPM is 1, the Brown-Out Detect circuitry will be turned off when chip Power-down mode or Deep Sleep mode is entered, resulting in a further reduction in power usage. However, the possibility of using Brown-Out Detect as a wake-up source from the reduced power mode will be lost. When 0, the Brown-Out Detect function remains active during Power-down and Deep Sleep modes. See the System Control Block chapter for details of Brown-Out detection."]
pub type BodrpmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOGD` reader - Brown-Out Global Disable. When BOGD is 1, the Brown-Out Detect circuitry is fully disabled at all times, and does not consume power. When 0, the Brown-Out Detect circuitry is enabled. See the System Control Block chapter for details of Brown-Out detection. Note: the Brown-Out Reset Disable (BORD, in this register) and the Brown-Out Interrupt (xx) must be disabled when software changes the value of this bit."]
pub type BogdR = crate::BitReader;
#[doc = "Field `BOGD` writer - Brown-Out Global Disable. When BOGD is 1, the Brown-Out Detect circuitry is fully disabled at all times, and does not consume power. When 0, the Brown-Out Detect circuitry is enabled. See the System Control Block chapter for details of Brown-Out detection. Note: the Brown-Out Reset Disable (BORD, in this register) and the Brown-Out Interrupt (xx) must be disabled when software changes the value of this bit."]
pub type BogdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BORD` reader - Brown-Out Reset Disable. When BORD is 1, the BOD will not reset the device when the VDD(REG)(3V3) voltage dips goes below the BOD reset trip level. The Brown-Out interrupt is not affected. When BORD is 0, the BOD reset is enabled."]
pub type BordR = crate::BitReader;
#[doc = "Field `BORD` writer - Brown-Out Reset Disable. When BORD is 1, the BOD will not reset the device when the VDD(REG)(3V3) voltage dips goes below the BOD reset trip level. The Brown-Out interrupt is not affected. When BORD is 0, the BOD reset is enabled."]
pub type BordW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMFLAG` reader - Sleep Mode entry flag. Set when the Sleep mode is successfully entered. Cleared by software writing a one to this bit."]
pub type SmflagR = crate::BitReader;
#[doc = "Field `SMFLAG` writer - Sleep Mode entry flag. Set when the Sleep mode is successfully entered. Cleared by software writing a one to this bit."]
pub type SmflagW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSFLAG` reader - Deep Sleep entry flag. Set when the Deep Sleep mode is successfully entered. Cleared by software writing a one to this bit."]
pub type DsflagR = crate::BitReader;
#[doc = "Field `DSFLAG` writer - Deep Sleep entry flag. Set when the Deep Sleep mode is successfully entered. Cleared by software writing a one to this bit."]
pub type DsflagW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDFLAG` reader - Power-down entry flag. Set when the Power-down mode is successfully entered. Cleared by software writing a one to this bit."]
pub type PdflagR = crate::BitReader;
#[doc = "Field `PDFLAG` writer - Power-down entry flag. Set when the Power-down mode is successfully entered. Cleared by software writing a one to this bit."]
pub type PdflagW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPDFLAG` reader - Deep Power-down entry flag. Set when the Deep Power-down mode is successfully entered. Cleared by software writing a one to this bit."]
pub type DpdflagR = crate::BitReader;
#[doc = "Field `DPDFLAG` writer - Deep Power-down entry flag. Set when the Deep Power-down mode is successfully entered. Cleared by software writing a one to this bit."]
pub type DpdflagW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Power mode control bit 0. This bit controls entry to the Power-down mode."]
    #[inline(always)]
    pub fn pm0(&self) -> Pm0R {
        Pm0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Power mode control bit 1. This bit controls entry to the Deep Power-down mode."]
    #[inline(always)]
    pub fn pm1(&self) -> Pm1R {
        Pm1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Brown-Out Reduced Power Mode. When BODRPM is 1, the Brown-Out Detect circuitry will be turned off when chip Power-down mode or Deep Sleep mode is entered, resulting in a further reduction in power usage. However, the possibility of using Brown-Out Detect as a wake-up source from the reduced power mode will be lost. When 0, the Brown-Out Detect function remains active during Power-down and Deep Sleep modes. See the System Control Block chapter for details of Brown-Out detection."]
    #[inline(always)]
    pub fn bodrpm(&self) -> BodrpmR {
        BodrpmR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Brown-Out Global Disable. When BOGD is 1, the Brown-Out Detect circuitry is fully disabled at all times, and does not consume power. When 0, the Brown-Out Detect circuitry is enabled. See the System Control Block chapter for details of Brown-Out detection. Note: the Brown-Out Reset Disable (BORD, in this register) and the Brown-Out Interrupt (xx) must be disabled when software changes the value of this bit."]
    #[inline(always)]
    pub fn bogd(&self) -> BogdR {
        BogdR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Brown-Out Reset Disable. When BORD is 1, the BOD will not reset the device when the VDD(REG)(3V3) voltage dips goes below the BOD reset trip level. The Brown-Out interrupt is not affected. When BORD is 0, the BOD reset is enabled."]
    #[inline(always)]
    pub fn bord(&self) -> BordR {
        BordR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Sleep Mode entry flag. Set when the Sleep mode is successfully entered. Cleared by software writing a one to this bit."]
    #[inline(always)]
    pub fn smflag(&self) -> SmflagR {
        SmflagR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Deep Sleep entry flag. Set when the Deep Sleep mode is successfully entered. Cleared by software writing a one to this bit."]
    #[inline(always)]
    pub fn dsflag(&self) -> DsflagR {
        DsflagR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Power-down entry flag. Set when the Power-down mode is successfully entered. Cleared by software writing a one to this bit."]
    #[inline(always)]
    pub fn pdflag(&self) -> PdflagR {
        PdflagR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Deep Power-down entry flag. Set when the Deep Power-down mode is successfully entered. Cleared by software writing a one to this bit."]
    #[inline(always)]
    pub fn dpdflag(&self) -> DpdflagR {
        DpdflagR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power mode control bit 0. This bit controls entry to the Power-down mode."]
    #[inline(always)]
    #[must_use]
    pub fn pm0(&mut self) -> Pm0W<PconSpec> {
        Pm0W::new(self, 0)
    }
    #[doc = "Bit 1 - Power mode control bit 1. This bit controls entry to the Deep Power-down mode."]
    #[inline(always)]
    #[must_use]
    pub fn pm1(&mut self) -> Pm1W<PconSpec> {
        Pm1W::new(self, 1)
    }
    #[doc = "Bit 2 - Brown-Out Reduced Power Mode. When BODRPM is 1, the Brown-Out Detect circuitry will be turned off when chip Power-down mode or Deep Sleep mode is entered, resulting in a further reduction in power usage. However, the possibility of using Brown-Out Detect as a wake-up source from the reduced power mode will be lost. When 0, the Brown-Out Detect function remains active during Power-down and Deep Sleep modes. See the System Control Block chapter for details of Brown-Out detection."]
    #[inline(always)]
    #[must_use]
    pub fn bodrpm(&mut self) -> BodrpmW<PconSpec> {
        BodrpmW::new(self, 2)
    }
    #[doc = "Bit 3 - Brown-Out Global Disable. When BOGD is 1, the Brown-Out Detect circuitry is fully disabled at all times, and does not consume power. When 0, the Brown-Out Detect circuitry is enabled. See the System Control Block chapter for details of Brown-Out detection. Note: the Brown-Out Reset Disable (BORD, in this register) and the Brown-Out Interrupt (xx) must be disabled when software changes the value of this bit."]
    #[inline(always)]
    #[must_use]
    pub fn bogd(&mut self) -> BogdW<PconSpec> {
        BogdW::new(self, 3)
    }
    #[doc = "Bit 4 - Brown-Out Reset Disable. When BORD is 1, the BOD will not reset the device when the VDD(REG)(3V3) voltage dips goes below the BOD reset trip level. The Brown-Out interrupt is not affected. When BORD is 0, the BOD reset is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn bord(&mut self) -> BordW<PconSpec> {
        BordW::new(self, 4)
    }
    #[doc = "Bit 8 - Sleep Mode entry flag. Set when the Sleep mode is successfully entered. Cleared by software writing a one to this bit."]
    #[inline(always)]
    #[must_use]
    pub fn smflag(&mut self) -> SmflagW<PconSpec> {
        SmflagW::new(self, 8)
    }
    #[doc = "Bit 9 - Deep Sleep entry flag. Set when the Deep Sleep mode is successfully entered. Cleared by software writing a one to this bit."]
    #[inline(always)]
    #[must_use]
    pub fn dsflag(&mut self) -> DsflagW<PconSpec> {
        DsflagW::new(self, 9)
    }
    #[doc = "Bit 10 - Power-down entry flag. Set when the Power-down mode is successfully entered. Cleared by software writing a one to this bit."]
    #[inline(always)]
    #[must_use]
    pub fn pdflag(&mut self) -> PdflagW<PconSpec> {
        PdflagW::new(self, 10)
    }
    #[doc = "Bit 11 - Deep Power-down entry flag. Set when the Deep Power-down mode is successfully entered. Cleared by software writing a one to this bit."]
    #[inline(always)]
    #[must_use]
    pub fn dpdflag(&mut self) -> DpdflagW<PconSpec> {
        DpdflagW::new(self, 11)
    }
}
#[doc = "Power Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pcon::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcon::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PconSpec;
impl crate::RegisterSpec for PconSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcon::R`](R) reader structure"]
impl crate::Readable for PconSpec {}
#[doc = "`write(|w| ..)` method takes [`pcon::W`](W) writer structure"]
impl crate::Writable for PconSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCON to value 0"]
impl crate::Resettable for PconSpec {
    const RESET_VALUE: u32 = 0;
}
