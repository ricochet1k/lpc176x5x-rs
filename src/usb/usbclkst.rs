#[doc = "Register `USBCLKST` reader"]
pub type R = crate::R<UsbclkstSpec>;
#[doc = "Field `DEV_CLK_ON` reader - Device clock on. The usbclk input to the device controller is active ."]
pub type DevClkOnR = crate::BitReader;
#[doc = "Field `PORTSEL_CLK_ON` reader - Port select register clock on."]
pub type PortselClkOnR = crate::BitReader;
#[doc = "Field `AHB_CLK_ON` reader - AHB clock on."]
pub type AhbClkOnR = crate::BitReader;
impl R {
    #[doc = "Bit 1 - Device clock on. The usbclk input to the device controller is active ."]
    #[inline(always)]
    pub fn dev_clk_on(&self) -> DevClkOnR {
        DevClkOnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Port select register clock on."]
    #[inline(always)]
    pub fn portsel_clk_on(&self) -> PortselClkOnR {
        PortselClkOnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AHB clock on."]
    #[inline(always)]
    pub fn ahb_clk_on(&self) -> AhbClkOnR {
        AhbClkOnR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "USB Clock Status\n\nYou can [`read`](crate::Reg::read) this register and get [`usbclkst::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsbclkstSpec;
impl crate::RegisterSpec for UsbclkstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usbclkst::R`](R) reader structure"]
impl crate::Readable for UsbclkstSpec {}
#[doc = "`reset()` method sets USBCLKST to value 0"]
impl crate::Resettable for UsbclkstSpec {
    const RESET_VALUE: u32 = 0;
}
