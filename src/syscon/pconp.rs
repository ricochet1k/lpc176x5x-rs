#[doc = "Register `PCONP` reader"]
pub type R = crate::R<PconpSpec>;
#[doc = "Register `PCONP` writer"]
pub type W = crate::W<PconpSpec>;
#[doc = "Field `PCTIM0` reader - Timer/Counter 0 power/clock control bit."]
pub type Pctim0R = crate::BitReader;
#[doc = "Field `PCTIM0` writer - Timer/Counter 0 power/clock control bit."]
pub type Pctim0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCTIM1` reader - Timer/Counter 1 power/clock control bit."]
pub type Pctim1R = crate::BitReader;
#[doc = "Field `PCTIM1` writer - Timer/Counter 1 power/clock control bit."]
pub type Pctim1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCUART0` reader - UART0 power/clock control bit."]
pub type Pcuart0R = crate::BitReader;
#[doc = "Field `PCUART0` writer - UART0 power/clock control bit."]
pub type Pcuart0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCUART1` reader - UART1 power/clock control bit."]
pub type Pcuart1R = crate::BitReader;
#[doc = "Field `PCUART1` writer - UART1 power/clock control bit."]
pub type Pcuart1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCPWM1` reader - PWM1 power/clock control bit."]
pub type Pcpwm1R = crate::BitReader;
#[doc = "Field `PCPWM1` writer - PWM1 power/clock control bit."]
pub type Pcpwm1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCI2C0` reader - The I2C0 interface power/clock control bit."]
pub type Pci2c0R = crate::BitReader;
#[doc = "Field `PCI2C0` writer - The I2C0 interface power/clock control bit."]
pub type Pci2c0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCSPI` reader - The SPI interface power/clock control bit."]
pub type PcspiR = crate::BitReader;
#[doc = "Field `PCSPI` writer - The SPI interface power/clock control bit."]
pub type PcspiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCRTC` reader - The RTC power/clock control bit."]
pub type PcrtcR = crate::BitReader;
#[doc = "Field `PCRTC` writer - The RTC power/clock control bit."]
pub type PcrtcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCSSP1` reader - The SSP 1 interface power/clock control bit."]
pub type Pcssp1R = crate::BitReader;
#[doc = "Field `PCSSP1` writer - The SSP 1 interface power/clock control bit."]
pub type Pcssp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCADC` reader - A/D converter (ADC) power/clock control bit. Note: Clear the PDN bit in the AD0CR before clearing this bit, and set this bit before setting PDN."]
pub type PcadcR = crate::BitReader;
#[doc = "Field `PCADC` writer - A/D converter (ADC) power/clock control bit. Note: Clear the PDN bit in the AD0CR before clearing this bit, and set this bit before setting PDN."]
pub type PcadcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCCAN1` reader - CAN Controller 1 power/clock control bit."]
pub type Pccan1R = crate::BitReader;
#[doc = "Field `PCCAN1` writer - CAN Controller 1 power/clock control bit."]
pub type Pccan1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCCAN2` reader - CAN Controller 2 power/clock control bit."]
pub type Pccan2R = crate::BitReader;
#[doc = "Field `PCCAN2` writer - CAN Controller 2 power/clock control bit."]
pub type Pccan2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCGPIO` reader - Power/clock control bit for IOCON, GPIO, and GPIO interrupts."]
pub type PcgpioR = crate::BitReader;
#[doc = "Field `PCGPIO` writer - Power/clock control bit for IOCON, GPIO, and GPIO interrupts."]
pub type PcgpioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCRIT` reader - Repetitive Interrupt Timer power/clock control bit."]
pub type PcritR = crate::BitReader;
#[doc = "Field `PCRIT` writer - Repetitive Interrupt Timer power/clock control bit."]
pub type PcritW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCMCPWM` reader - Motor Control PWM"]
pub type PcmcpwmR = crate::BitReader;
#[doc = "Field `PCMCPWM` writer - Motor Control PWM"]
pub type PcmcpwmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCQEI` reader - Quadrature Encoder Interface power/clock control bit."]
pub type PcqeiR = crate::BitReader;
#[doc = "Field `PCQEI` writer - Quadrature Encoder Interface power/clock control bit."]
pub type PcqeiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCI2C1` reader - The I2C1 interface power/clock control bit."]
pub type Pci2c1R = crate::BitReader;
#[doc = "Field `PCI2C1` writer - The I2C1 interface power/clock control bit."]
pub type Pci2c1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCSSP0` reader - The SSP0 interface power/clock control bit."]
pub type Pcssp0R = crate::BitReader;
#[doc = "Field `PCSSP0` writer - The SSP0 interface power/clock control bit."]
pub type Pcssp0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCTIM2` reader - Timer 2 power/clock control bit."]
pub type Pctim2R = crate::BitReader;
#[doc = "Field `PCTIM2` writer - Timer 2 power/clock control bit."]
pub type Pctim2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCTIM3` reader - Timer 3 power/clock control bit."]
pub type Pctim3R = crate::BitReader;
#[doc = "Field `PCTIM3` writer - Timer 3 power/clock control bit."]
pub type Pctim3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCUART2` reader - UART 2 power/clock control bit."]
pub type Pcuart2R = crate::BitReader;
#[doc = "Field `PCUART2` writer - UART 2 power/clock control bit."]
pub type Pcuart2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCUART3` reader - UART 3 power/clock control bit."]
pub type Pcuart3R = crate::BitReader;
#[doc = "Field `PCUART3` writer - UART 3 power/clock control bit."]
pub type Pcuart3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCI2C2` reader - I2C interface 2 power/clock control bit."]
pub type Pci2c2R = crate::BitReader;
#[doc = "Field `PCI2C2` writer - I2C interface 2 power/clock control bit."]
pub type Pci2c2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCI2S` reader - I2S interface power/clock control bit."]
pub type Pci2sR = crate::BitReader;
#[doc = "Field `PCI2S` writer - I2S interface power/clock control bit."]
pub type Pci2sW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCGPDMA` reader - GPDMA function power/clock control bit."]
pub type PcgpdmaR = crate::BitReader;
#[doc = "Field `PCGPDMA` writer - GPDMA function power/clock control bit."]
pub type PcgpdmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCENET` reader - Ethernet block power/clock control bit."]
pub type PcenetR = crate::BitReader;
#[doc = "Field `PCENET` writer - Ethernet block power/clock control bit."]
pub type PcenetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCUSB` reader - USB interface power/clock control bit."]
pub type PcusbR = crate::BitReader;
#[doc = "Field `PCUSB` writer - USB interface power/clock control bit."]
pub type PcusbW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Timer/Counter 0 power/clock control bit."]
    #[inline(always)]
    pub fn pctim0(&self) -> Pctim0R {
        Pctim0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer/Counter 1 power/clock control bit."]
    #[inline(always)]
    pub fn pctim1(&self) -> Pctim1R {
        Pctim1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UART0 power/clock control bit."]
    #[inline(always)]
    pub fn pcuart0(&self) -> Pcuart0R {
        Pcuart0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - UART1 power/clock control bit."]
    #[inline(always)]
    pub fn pcuart1(&self) -> Pcuart1R {
        Pcuart1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - PWM1 power/clock control bit."]
    #[inline(always)]
    pub fn pcpwm1(&self) -> Pcpwm1R {
        Pcpwm1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The I2C0 interface power/clock control bit."]
    #[inline(always)]
    pub fn pci2c0(&self) -> Pci2c0R {
        Pci2c0R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The SPI interface power/clock control bit."]
    #[inline(always)]
    pub fn pcspi(&self) -> PcspiR {
        PcspiR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The RTC power/clock control bit."]
    #[inline(always)]
    pub fn pcrtc(&self) -> PcrtcR {
        PcrtcR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The SSP 1 interface power/clock control bit."]
    #[inline(always)]
    pub fn pcssp1(&self) -> Pcssp1R {
        Pcssp1R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - A/D converter (ADC) power/clock control bit. Note: Clear the PDN bit in the AD0CR before clearing this bit, and set this bit before setting PDN."]
    #[inline(always)]
    pub fn pcadc(&self) -> PcadcR {
        PcadcR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CAN Controller 1 power/clock control bit."]
    #[inline(always)]
    pub fn pccan1(&self) -> Pccan1R {
        Pccan1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CAN Controller 2 power/clock control bit."]
    #[inline(always)]
    pub fn pccan2(&self) -> Pccan2R {
        Pccan2R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Power/clock control bit for IOCON, GPIO, and GPIO interrupts."]
    #[inline(always)]
    pub fn pcgpio(&self) -> PcgpioR {
        PcgpioR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Repetitive Interrupt Timer power/clock control bit."]
    #[inline(always)]
    pub fn pcrit(&self) -> PcritR {
        PcritR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Motor Control PWM"]
    #[inline(always)]
    pub fn pcmcpwm(&self) -> PcmcpwmR {
        PcmcpwmR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Quadrature Encoder Interface power/clock control bit."]
    #[inline(always)]
    pub fn pcqei(&self) -> PcqeiR {
        PcqeiR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The I2C1 interface power/clock control bit."]
    #[inline(always)]
    pub fn pci2c1(&self) -> Pci2c1R {
        Pci2c1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - The SSP0 interface power/clock control bit."]
    #[inline(always)]
    pub fn pcssp0(&self) -> Pcssp0R {
        Pcssp0R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Timer 2 power/clock control bit."]
    #[inline(always)]
    pub fn pctim2(&self) -> Pctim2R {
        Pctim2R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Timer 3 power/clock control bit."]
    #[inline(always)]
    pub fn pctim3(&self) -> Pctim3R {
        Pctim3R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - UART 2 power/clock control bit."]
    #[inline(always)]
    pub fn pcuart2(&self) -> Pcuart2R {
        Pcuart2R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - UART 3 power/clock control bit."]
    #[inline(always)]
    pub fn pcuart3(&self) -> Pcuart3R {
        Pcuart3R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - I2C interface 2 power/clock control bit."]
    #[inline(always)]
    pub fn pci2c2(&self) -> Pci2c2R {
        Pci2c2R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - I2S interface power/clock control bit."]
    #[inline(always)]
    pub fn pci2s(&self) -> Pci2sR {
        Pci2sR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - GPDMA function power/clock control bit."]
    #[inline(always)]
    pub fn pcgpdma(&self) -> PcgpdmaR {
        PcgpdmaR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Ethernet block power/clock control bit."]
    #[inline(always)]
    pub fn pcenet(&self) -> PcenetR {
        PcenetR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - USB interface power/clock control bit."]
    #[inline(always)]
    pub fn pcusb(&self) -> PcusbR {
        PcusbR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Timer/Counter 0 power/clock control bit."]
    #[inline(always)]
    #[must_use]
    pub fn pctim0(&mut self) -> Pctim0W<PconpSpec> {
        Pctim0W::new(self, 1)
    }
    #[doc = "Bit 2 - Timer/Counter 1 power/clock control bit."]
    #[inline(always)]
    #[must_use]
    pub fn pctim1(&mut self) -> Pctim1W<PconpSpec> {
        Pctim1W::new(self, 2)
    }
    #[doc = "Bit 3 - UART0 power/clock control bit."]
    #[inline(always)]
    #[must_use]
    pub fn pcuart0(&mut self) -> Pcuart0W<PconpSpec> {
        Pcuart0W::new(self, 3)
    }
    #[doc = "Bit 4 - UART1 power/clock control bit."]
    #[inline(always)]
    #[must_use]
    pub fn pcuart1(&mut self) -> Pcuart1W<PconpSpec> {
        Pcuart1W::new(self, 4)
    }
    #[doc = "Bit 6 - PWM1 power/clock control bit."]
    #[inline(always)]
    #[must_use]
    pub fn pcpwm1(&mut self) -> Pcpwm1W<PconpSpec> {
        Pcpwm1W::new(self, 6)
    }
    #[doc = "Bit 7 - The I2C0 interface power/clock control bit."]
    #[inline(always)]
    #[must_use]
    pub fn pci2c0(&mut self) -> Pci2c0W<PconpSpec> {
        Pci2c0W::new(self, 7)
    }
    #[doc = "Bit 8 - The SPI interface power/clock control bit."]
    #[inline(always)]
    #[must_use]
    pub fn pcspi(&mut self) -> PcspiW<PconpSpec> {
        PcspiW::new(self, 8)
    }
    #[doc = "Bit 9 - The RTC power/clock control bit."]
    #[inline(always)]
    #[must_use]
    pub fn pcrtc(&mut self) -> PcrtcW<PconpSpec> {
        PcrtcW::new(self, 9)
    }
    #[doc = "Bit 10 - The SSP 1 interface power/clock control bit."]
    #[inline(always)]
    #[must_use]
    pub fn pcssp1(&mut self) -> Pcssp1W<PconpSpec> {
        Pcssp1W::new(self, 10)
    }
    #[doc = "Bit 12 - A/D converter (ADC) power/clock control bit. Note: Clear the PDN bit in the AD0CR before clearing this bit, and set this bit before setting PDN."]
    #[inline(always)]
    #[must_use]
    pub fn pcadc(&mut self) -> PcadcW<PconpSpec> {
        PcadcW::new(self, 12)
    }
    #[doc = "Bit 13 - CAN Controller 1 power/clock control bit."]
    #[inline(always)]
    #[must_use]
    pub fn pccan1(&mut self) -> Pccan1W<PconpSpec> {
        Pccan1W::new(self, 13)
    }
    #[doc = "Bit 14 - CAN Controller 2 power/clock control bit."]
    #[inline(always)]
    #[must_use]
    pub fn pccan2(&mut self) -> Pccan2W<PconpSpec> {
        Pccan2W::new(self, 14)
    }
    #[doc = "Bit 15 - Power/clock control bit for IOCON, GPIO, and GPIO interrupts."]
    #[inline(always)]
    #[must_use]
    pub fn pcgpio(&mut self) -> PcgpioW<PconpSpec> {
        PcgpioW::new(self, 15)
    }
    #[doc = "Bit 16 - Repetitive Interrupt Timer power/clock control bit."]
    #[inline(always)]
    #[must_use]
    pub fn pcrit(&mut self) -> PcritW<PconpSpec> {
        PcritW::new(self, 16)
    }
    #[doc = "Bit 17 - Motor Control PWM"]
    #[inline(always)]
    #[must_use]
    pub fn pcmcpwm(&mut self) -> PcmcpwmW<PconpSpec> {
        PcmcpwmW::new(self, 17)
    }
    #[doc = "Bit 18 - Quadrature Encoder Interface power/clock control bit."]
    #[inline(always)]
    #[must_use]
    pub fn pcqei(&mut self) -> PcqeiW<PconpSpec> {
        PcqeiW::new(self, 18)
    }
    #[doc = "Bit 19 - The I2C1 interface power/clock control bit."]
    #[inline(always)]
    #[must_use]
    pub fn pci2c1(&mut self) -> Pci2c1W<PconpSpec> {
        Pci2c1W::new(self, 19)
    }
    #[doc = "Bit 21 - The SSP0 interface power/clock control bit."]
    #[inline(always)]
    #[must_use]
    pub fn pcssp0(&mut self) -> Pcssp0W<PconpSpec> {
        Pcssp0W::new(self, 21)
    }
    #[doc = "Bit 22 - Timer 2 power/clock control bit."]
    #[inline(always)]
    #[must_use]
    pub fn pctim2(&mut self) -> Pctim2W<PconpSpec> {
        Pctim2W::new(self, 22)
    }
    #[doc = "Bit 23 - Timer 3 power/clock control bit."]
    #[inline(always)]
    #[must_use]
    pub fn pctim3(&mut self) -> Pctim3W<PconpSpec> {
        Pctim3W::new(self, 23)
    }
    #[doc = "Bit 24 - UART 2 power/clock control bit."]
    #[inline(always)]
    #[must_use]
    pub fn pcuart2(&mut self) -> Pcuart2W<PconpSpec> {
        Pcuart2W::new(self, 24)
    }
    #[doc = "Bit 25 - UART 3 power/clock control bit."]
    #[inline(always)]
    #[must_use]
    pub fn pcuart3(&mut self) -> Pcuart3W<PconpSpec> {
        Pcuart3W::new(self, 25)
    }
    #[doc = "Bit 26 - I2C interface 2 power/clock control bit."]
    #[inline(always)]
    #[must_use]
    pub fn pci2c2(&mut self) -> Pci2c2W<PconpSpec> {
        Pci2c2W::new(self, 26)
    }
    #[doc = "Bit 27 - I2S interface power/clock control bit."]
    #[inline(always)]
    #[must_use]
    pub fn pci2s(&mut self) -> Pci2sW<PconpSpec> {
        Pci2sW::new(self, 27)
    }
    #[doc = "Bit 29 - GPDMA function power/clock control bit."]
    #[inline(always)]
    #[must_use]
    pub fn pcgpdma(&mut self) -> PcgpdmaW<PconpSpec> {
        PcgpdmaW::new(self, 29)
    }
    #[doc = "Bit 30 - Ethernet block power/clock control bit."]
    #[inline(always)]
    #[must_use]
    pub fn pcenet(&mut self) -> PcenetW<PconpSpec> {
        PcenetW::new(self, 30)
    }
    #[doc = "Bit 31 - USB interface power/clock control bit."]
    #[inline(always)]
    #[must_use]
    pub fn pcusb(&mut self) -> PcusbW<PconpSpec> {
        PcusbW::new(self, 31)
    }
}
#[doc = "Power Control for Peripherals Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pconp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pconp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PconpSpec;
impl crate::RegisterSpec for PconpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pconp::R`](R) reader structure"]
impl crate::Readable for PconpSpec {}
#[doc = "`write(|w| ..)` method takes [`pconp::W`](W) writer structure"]
impl crate::Writable for PconpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCONP to value 0x03be"]
impl crate::Resettable for PconpSpec {
    const RESET_VALUE: u32 = 0x03be;
}
