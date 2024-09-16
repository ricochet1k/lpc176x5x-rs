#[doc = "Register `USBCLKCTRL` reader"]
pub type R = crate::R<UsbclkctrlSpec>;
#[doc = "Register `USBCLKCTRL` writer"]
pub type W = crate::W<UsbclkctrlSpec>;
#[doc = "Field `DEV_CLK_EN` reader - Device clock enable. Enables the usbclk input to the device controller"]
pub type DevClkEnR = crate::BitReader;
#[doc = "Field `DEV_CLK_EN` writer - Device clock enable. Enables the usbclk input to the device controller"]
pub type DevClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORTSEL_CLK_EN` reader - Port select register clock enable."]
pub type PortselClkEnR = crate::BitReader;
#[doc = "Field `PORTSEL_CLK_EN` writer - Port select register clock enable."]
pub type PortselClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHB_CLK_EN` reader - AHB clock enable"]
pub type AhbClkEnR = crate::BitReader;
#[doc = "Field `AHB_CLK_EN` writer - AHB clock enable"]
pub type AhbClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Device clock enable. Enables the usbclk input to the device controller"]
    #[inline(always)]
    pub fn dev_clk_en(&self) -> DevClkEnR {
        DevClkEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Port select register clock enable."]
    #[inline(always)]
    pub fn portsel_clk_en(&self) -> PortselClkEnR {
        PortselClkEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AHB clock enable"]
    #[inline(always)]
    pub fn ahb_clk_en(&self) -> AhbClkEnR {
        AhbClkEnR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Device clock enable. Enables the usbclk input to the device controller"]
    #[inline(always)]
    #[must_use]
    pub fn dev_clk_en(&mut self) -> DevClkEnW<UsbclkctrlSpec> {
        DevClkEnW::new(self, 1)
    }
    #[doc = "Bit 3 - Port select register clock enable."]
    #[inline(always)]
    #[must_use]
    pub fn portsel_clk_en(&mut self) -> PortselClkEnW<UsbclkctrlSpec> {
        PortselClkEnW::new(self, 3)
    }
    #[doc = "Bit 4 - AHB clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn ahb_clk_en(&mut self) -> AhbClkEnW<UsbclkctrlSpec> {
        AhbClkEnW::new(self, 4)
    }
}
#[doc = "USB Clock Control\n\nYou can [`read`](crate::Reg::read) this register and get [`usbclkctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbclkctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsbclkctrlSpec;
impl crate::RegisterSpec for UsbclkctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usbclkctrl::R`](R) reader structure"]
impl crate::Readable for UsbclkctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`usbclkctrl::W`](W) writer structure"]
impl crate::Writable for UsbclkctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USBCLKCTRL to value 0"]
impl crate::Resettable for UsbclkctrlSpec {
    const RESET_VALUE: u32 = 0;
}
