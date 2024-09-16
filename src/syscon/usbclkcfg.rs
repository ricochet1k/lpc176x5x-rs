#[doc = "Register `USBCLKCFG` reader"]
pub type R = crate::R<UsbclkcfgSpec>;
#[doc = "Register `USBCLKCFG` writer"]
pub type W = crate::W<UsbclkcfgSpec>;
#[doc = "Field `USBSEL` reader - Selects the divide value for creating the USB clock from the PLL0 output. Only the values shown below can produce even number multiples of 48 MHz from the PLL0 output. Warning: Improper setting of this value will result in incorrect operation of the USB interface. 5 = PLL0 output is divided by 6. PLL0 output must be 288 MHz. 7 = PLL0 output is divided by 8. PLL0 output must be 384 MHz. 9 = PLL0 output is divided by 10. PLL0 output must be 480 MHz."]
pub type UsbselR = crate::FieldReader;
#[doc = "Field `USBSEL` writer - Selects the divide value for creating the USB clock from the PLL0 output. Only the values shown below can produce even number multiples of 48 MHz from the PLL0 output. Warning: Improper setting of this value will result in incorrect operation of the USB interface. 5 = PLL0 output is divided by 6. PLL0 output must be 288 MHz. 7 = PLL0 output is divided by 8. PLL0 output must be 384 MHz. 9 = PLL0 output is divided by 10. PLL0 output must be 480 MHz."]
pub type UsbselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Selects the divide value for creating the USB clock from the PLL0 output. Only the values shown below can produce even number multiples of 48 MHz from the PLL0 output. Warning: Improper setting of this value will result in incorrect operation of the USB interface. 5 = PLL0 output is divided by 6. PLL0 output must be 288 MHz. 7 = PLL0 output is divided by 8. PLL0 output must be 384 MHz. 9 = PLL0 output is divided by 10. PLL0 output must be 480 MHz."]
    #[inline(always)]
    pub fn usbsel(&self) -> UsbselR {
        UsbselR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Selects the divide value for creating the USB clock from the PLL0 output. Only the values shown below can produce even number multiples of 48 MHz from the PLL0 output. Warning: Improper setting of this value will result in incorrect operation of the USB interface. 5 = PLL0 output is divided by 6. PLL0 output must be 288 MHz. 7 = PLL0 output is divided by 8. PLL0 output must be 384 MHz. 9 = PLL0 output is divided by 10. PLL0 output must be 480 MHz."]
    #[inline(always)]
    #[must_use]
    pub fn usbsel(&mut self) -> UsbselW<UsbclkcfgSpec> {
        UsbselW::new(self, 0)
    }
}
#[doc = "USB Clock Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usbclkcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbclkcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsbclkcfgSpec;
impl crate::RegisterSpec for UsbclkcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usbclkcfg::R`](R) reader structure"]
impl crate::Readable for UsbclkcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`usbclkcfg::W`](W) writer structure"]
impl crate::Writable for UsbclkcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USBCLKCFG to value 0"]
impl crate::Resettable for UsbclkcfgSpec {
    const RESET_VALUE: u32 = 0;
}
