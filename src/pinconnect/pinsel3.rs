#[doc = "Register `PINSEL3` reader"]
pub type R = crate::R<Pinsel3Spec>;
#[doc = "Register `PINSEL3` writer"]
pub type W = crate::W<Pinsel3Spec>;
#[doc = "Pin function select P1.16.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_16 {
    #[doc = "0: GPIO P1.16"]
    GpioP1 = 0,
    #[doc = "1: ENET_MDC"]
    EnetMdc = 1,
}
impl From<P1_16> for u8 {
    #[inline(always)]
    fn from(variant: P1_16) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_16 {
    type Ux = u8;
}
impl crate::IsEnum for P1_16 {}
#[doc = "Field `P1_16` reader - Pin function select P1.16."]
pub type P1_16R = crate::FieldReader<P1_16>;
impl P1_16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_16 {
        match self.bits {
            0 => P1_16::GpioP1,
            1 => P1_16::EnetMdc,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P1.16"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_16::GpioP1
    }
    #[doc = "ENET_MDC"]
    #[inline(always)]
    pub fn is_enet_mdc(&self) -> bool {
        *self == P1_16::EnetMdc
    }
}
#[doc = "Field `P1_16` writer - Pin function select P1.16."]
pub type P1_16W<'a, REG> = crate::FieldWriter<'a, REG, 2, P1_16>;
impl<'a, REG> P1_16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P1.16"]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_16::GpioP1)
    }
    #[doc = "ENET_MDC"]
    #[inline(always)]
    pub fn enet_mdc(self) -> &'a mut crate::W<REG> {
        self.variant(P1_16::EnetMdc)
    }
}
#[doc = "Pin function select P1.17.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_17 {
    #[doc = "0: GPIO P1.17"]
    GpioP1 = 0,
    #[doc = "1: ENET_MDIO"]
    EnetMdio = 1,
}
impl From<P1_17> for u8 {
    #[inline(always)]
    fn from(variant: P1_17) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_17 {
    type Ux = u8;
}
impl crate::IsEnum for P1_17 {}
#[doc = "Field `P1_17` reader - Pin function select P1.17."]
pub type P1_17R = crate::FieldReader<P1_17>;
impl P1_17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_17 {
        match self.bits {
            0 => P1_17::GpioP1,
            1 => P1_17::EnetMdio,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P1.17"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_17::GpioP1
    }
    #[doc = "ENET_MDIO"]
    #[inline(always)]
    pub fn is_enet_mdio(&self) -> bool {
        *self == P1_17::EnetMdio
    }
}
#[doc = "Field `P1_17` writer - Pin function select P1.17."]
pub type P1_17W<'a, REG> = crate::FieldWriter<'a, REG, 2, P1_17>;
impl<'a, REG> P1_17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P1.17"]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_17::GpioP1)
    }
    #[doc = "ENET_MDIO"]
    #[inline(always)]
    pub fn enet_mdio(self) -> &'a mut crate::W<REG> {
        self.variant(P1_17::EnetMdio)
    }
}
#[doc = "Pin function select P1.18.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_18 {
    #[doc = "0: GPIO P1.18"]
    GpioP1 = 0,
    #[doc = "1: USB_UP_LED"]
    UsbUpLed = 1,
    #[doc = "2: PWM1.1"]
    Pwm1 = 2,
    #[doc = "3: CAP1.0"]
    Cap1 = 3,
}
impl From<P1_18> for u8 {
    #[inline(always)]
    fn from(variant: P1_18) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_18 {
    type Ux = u8;
}
impl crate::IsEnum for P1_18 {}
#[doc = "Field `P1_18` reader - Pin function select P1.18."]
pub type P1_18R = crate::FieldReader<P1_18>;
impl P1_18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_18 {
        match self.bits {
            0 => P1_18::GpioP1,
            1 => P1_18::UsbUpLed,
            2 => P1_18::Pwm1,
            3 => P1_18::Cap1,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P1.18"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_18::GpioP1
    }
    #[doc = "USB_UP_LED"]
    #[inline(always)]
    pub fn is_usb_up_led(&self) -> bool {
        *self == P1_18::UsbUpLed
    }
    #[doc = "PWM1.1"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == P1_18::Pwm1
    }
    #[doc = "CAP1.0"]
    #[inline(always)]
    pub fn is_cap1(&self) -> bool {
        *self == P1_18::Cap1
    }
}
#[doc = "Field `P1_18` writer - Pin function select P1.18."]
pub type P1_18W<'a, REG> = crate::FieldWriter<'a, REG, 2, P1_18, crate::Safe>;
impl<'a, REG> P1_18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P1.18"]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_18::GpioP1)
    }
    #[doc = "USB_UP_LED"]
    #[inline(always)]
    pub fn usb_up_led(self) -> &'a mut crate::W<REG> {
        self.variant(P1_18::UsbUpLed)
    }
    #[doc = "PWM1.1"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_18::Pwm1)
    }
    #[doc = "CAP1.0"]
    #[inline(always)]
    pub fn cap1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_18::Cap1)
    }
}
#[doc = "Pin function select P1.19.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_19 {
    #[doc = "0: GPIO P1.19."]
    GpioP1 = 0,
    #[doc = "1: MCOA0"]
    Mcoa0 = 1,
    #[doc = "2: USB_PPWR"]
    UsbPpwr = 2,
    #[doc = "3: CAP1.1"]
    Cap1 = 3,
}
impl From<P1_19> for u8 {
    #[inline(always)]
    fn from(variant: P1_19) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_19 {
    type Ux = u8;
}
impl crate::IsEnum for P1_19 {}
#[doc = "Field `P1_19` reader - Pin function select P1.19."]
pub type P1_19R = crate::FieldReader<P1_19>;
impl P1_19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_19 {
        match self.bits {
            0 => P1_19::GpioP1,
            1 => P1_19::Mcoa0,
            2 => P1_19::UsbPpwr,
            3 => P1_19::Cap1,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P1.19."]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_19::GpioP1
    }
    #[doc = "MCOA0"]
    #[inline(always)]
    pub fn is_mcoa0(&self) -> bool {
        *self == P1_19::Mcoa0
    }
    #[doc = "USB_PPWR"]
    #[inline(always)]
    pub fn is_usb_ppwr(&self) -> bool {
        *self == P1_19::UsbPpwr
    }
    #[doc = "CAP1.1"]
    #[inline(always)]
    pub fn is_cap1(&self) -> bool {
        *self == P1_19::Cap1
    }
}
#[doc = "Field `P1_19` writer - Pin function select P1.19."]
pub type P1_19W<'a, REG> = crate::FieldWriter<'a, REG, 2, P1_19, crate::Safe>;
impl<'a, REG> P1_19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P1.19."]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_19::GpioP1)
    }
    #[doc = "MCOA0"]
    #[inline(always)]
    pub fn mcoa0(self) -> &'a mut crate::W<REG> {
        self.variant(P1_19::Mcoa0)
    }
    #[doc = "USB_PPWR"]
    #[inline(always)]
    pub fn usb_ppwr(self) -> &'a mut crate::W<REG> {
        self.variant(P1_19::UsbPpwr)
    }
    #[doc = "CAP1.1"]
    #[inline(always)]
    pub fn cap1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_19::Cap1)
    }
}
#[doc = "Pin function select P1.20.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_20 {
    #[doc = "0: GPIO P1.20."]
    GpioP1 = 0,
    #[doc = "1: MCI0"]
    Mci0 = 1,
    #[doc = "2: PWM1.2"]
    Pwm1 = 2,
    #[doc = "3: SCK0"]
    Sck0 = 3,
}
impl From<P1_20> for u8 {
    #[inline(always)]
    fn from(variant: P1_20) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_20 {
    type Ux = u8;
}
impl crate::IsEnum for P1_20 {}
#[doc = "Field `P1_20` reader - Pin function select P1.20."]
pub type P1_20R = crate::FieldReader<P1_20>;
impl P1_20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_20 {
        match self.bits {
            0 => P1_20::GpioP1,
            1 => P1_20::Mci0,
            2 => P1_20::Pwm1,
            3 => P1_20::Sck0,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P1.20."]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_20::GpioP1
    }
    #[doc = "MCI0"]
    #[inline(always)]
    pub fn is_mci0(&self) -> bool {
        *self == P1_20::Mci0
    }
    #[doc = "PWM1.2"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == P1_20::Pwm1
    }
    #[doc = "SCK0"]
    #[inline(always)]
    pub fn is_sck0(&self) -> bool {
        *self == P1_20::Sck0
    }
}
#[doc = "Field `P1_20` writer - Pin function select P1.20."]
pub type P1_20W<'a, REG> = crate::FieldWriter<'a, REG, 2, P1_20, crate::Safe>;
impl<'a, REG> P1_20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P1.20."]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_20::GpioP1)
    }
    #[doc = "MCI0"]
    #[inline(always)]
    pub fn mci0(self) -> &'a mut crate::W<REG> {
        self.variant(P1_20::Mci0)
    }
    #[doc = "PWM1.2"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_20::Pwm1)
    }
    #[doc = "SCK0"]
    #[inline(always)]
    pub fn sck0(self) -> &'a mut crate::W<REG> {
        self.variant(P1_20::Sck0)
    }
}
#[doc = "Pin function select P1.21.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_21 {
    #[doc = "0: GPIO P1.21."]
    GpioP1 = 0,
    #[doc = "1: MCABORT"]
    Mcabort = 1,
    #[doc = "2: PWM1.3"]
    Pwm1 = 2,
    #[doc = "3: SSEL0"]
    Ssel0 = 3,
}
impl From<P1_21> for u8 {
    #[inline(always)]
    fn from(variant: P1_21) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_21 {
    type Ux = u8;
}
impl crate::IsEnum for P1_21 {}
#[doc = "Field `P1_21` reader - Pin function select P1.21."]
pub type P1_21R = crate::FieldReader<P1_21>;
impl P1_21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_21 {
        match self.bits {
            0 => P1_21::GpioP1,
            1 => P1_21::Mcabort,
            2 => P1_21::Pwm1,
            3 => P1_21::Ssel0,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P1.21."]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_21::GpioP1
    }
    #[doc = "MCABORT"]
    #[inline(always)]
    pub fn is_mcabort(&self) -> bool {
        *self == P1_21::Mcabort
    }
    #[doc = "PWM1.3"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == P1_21::Pwm1
    }
    #[doc = "SSEL0"]
    #[inline(always)]
    pub fn is_ssel0(&self) -> bool {
        *self == P1_21::Ssel0
    }
}
#[doc = "Field `P1_21` writer - Pin function select P1.21."]
pub type P1_21W<'a, REG> = crate::FieldWriter<'a, REG, 2, P1_21, crate::Safe>;
impl<'a, REG> P1_21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P1.21."]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_21::GpioP1)
    }
    #[doc = "MCABORT"]
    #[inline(always)]
    pub fn mcabort(self) -> &'a mut crate::W<REG> {
        self.variant(P1_21::Mcabort)
    }
    #[doc = "PWM1.3"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_21::Pwm1)
    }
    #[doc = "SSEL0"]
    #[inline(always)]
    pub fn ssel0(self) -> &'a mut crate::W<REG> {
        self.variant(P1_21::Ssel0)
    }
}
#[doc = "Pin function select P1.22\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_22 {
    #[doc = "0: GPIO P1.22."]
    GpioP1 = 0,
    #[doc = "1: MCOB0"]
    Mcob0 = 1,
    #[doc = "2: USB_PWRD"]
    UsbPwrd = 2,
    #[doc = "3: MAT1.0"]
    Mat1 = 3,
}
impl From<P1_22> for u8 {
    #[inline(always)]
    fn from(variant: P1_22) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_22 {
    type Ux = u8;
}
impl crate::IsEnum for P1_22 {}
#[doc = "Field `P1_22` reader - Pin function select P1.22"]
pub type P1_22R = crate::FieldReader<P1_22>;
impl P1_22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_22 {
        match self.bits {
            0 => P1_22::GpioP1,
            1 => P1_22::Mcob0,
            2 => P1_22::UsbPwrd,
            3 => P1_22::Mat1,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P1.22."]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_22::GpioP1
    }
    #[doc = "MCOB0"]
    #[inline(always)]
    pub fn is_mcob0(&self) -> bool {
        *self == P1_22::Mcob0
    }
    #[doc = "USB_PWRD"]
    #[inline(always)]
    pub fn is_usb_pwrd(&self) -> bool {
        *self == P1_22::UsbPwrd
    }
    #[doc = "MAT1.0"]
    #[inline(always)]
    pub fn is_mat1(&self) -> bool {
        *self == P1_22::Mat1
    }
}
#[doc = "Field `P1_22` writer - Pin function select P1.22"]
pub type P1_22W<'a, REG> = crate::FieldWriter<'a, REG, 2, P1_22, crate::Safe>;
impl<'a, REG> P1_22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P1.22."]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_22::GpioP1)
    }
    #[doc = "MCOB0"]
    #[inline(always)]
    pub fn mcob0(self) -> &'a mut crate::W<REG> {
        self.variant(P1_22::Mcob0)
    }
    #[doc = "USB_PWRD"]
    #[inline(always)]
    pub fn usb_pwrd(self) -> &'a mut crate::W<REG> {
        self.variant(P1_22::UsbPwrd)
    }
    #[doc = "MAT1.0"]
    #[inline(always)]
    pub fn mat1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_22::Mat1)
    }
}
#[doc = "Pin function select P1.23.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_23 {
    #[doc = "0: GPIO P1.23."]
    GpioP1 = 0,
    #[doc = "1: MCI1"]
    Mci1 = 1,
    #[doc = "2: PWM1.4"]
    Pwm1 = 2,
    #[doc = "3: MISO0"]
    Miso0 = 3,
}
impl From<P1_23> for u8 {
    #[inline(always)]
    fn from(variant: P1_23) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_23 {
    type Ux = u8;
}
impl crate::IsEnum for P1_23 {}
#[doc = "Field `P1_23` reader - Pin function select P1.23."]
pub type P1_23R = crate::FieldReader<P1_23>;
impl P1_23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_23 {
        match self.bits {
            0 => P1_23::GpioP1,
            1 => P1_23::Mci1,
            2 => P1_23::Pwm1,
            3 => P1_23::Miso0,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P1.23."]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_23::GpioP1
    }
    #[doc = "MCI1"]
    #[inline(always)]
    pub fn is_mci1(&self) -> bool {
        *self == P1_23::Mci1
    }
    #[doc = "PWM1.4"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == P1_23::Pwm1
    }
    #[doc = "MISO0"]
    #[inline(always)]
    pub fn is_miso0(&self) -> bool {
        *self == P1_23::Miso0
    }
}
#[doc = "Field `P1_23` writer - Pin function select P1.23."]
pub type P1_23W<'a, REG> = crate::FieldWriter<'a, REG, 2, P1_23, crate::Safe>;
impl<'a, REG> P1_23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P1.23."]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_23::GpioP1)
    }
    #[doc = "MCI1"]
    #[inline(always)]
    pub fn mci1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_23::Mci1)
    }
    #[doc = "PWM1.4"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_23::Pwm1)
    }
    #[doc = "MISO0"]
    #[inline(always)]
    pub fn miso0(self) -> &'a mut crate::W<REG> {
        self.variant(P1_23::Miso0)
    }
}
#[doc = "Pin function select P1.24.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_24 {
    #[doc = "0: GPIO P1.24."]
    GpioP1 = 0,
    #[doc = "1: MCI2"]
    Mci2 = 1,
    #[doc = "2: PWM1.5"]
    Pwm1 = 2,
    #[doc = "3: MOSI0"]
    Mosi0 = 3,
}
impl From<P1_24> for u8 {
    #[inline(always)]
    fn from(variant: P1_24) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_24 {
    type Ux = u8;
}
impl crate::IsEnum for P1_24 {}
#[doc = "Field `P1_24` reader - Pin function select P1.24."]
pub type P1_24R = crate::FieldReader<P1_24>;
impl P1_24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_24 {
        match self.bits {
            0 => P1_24::GpioP1,
            1 => P1_24::Mci2,
            2 => P1_24::Pwm1,
            3 => P1_24::Mosi0,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P1.24."]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_24::GpioP1
    }
    #[doc = "MCI2"]
    #[inline(always)]
    pub fn is_mci2(&self) -> bool {
        *self == P1_24::Mci2
    }
    #[doc = "PWM1.5"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == P1_24::Pwm1
    }
    #[doc = "MOSI0"]
    #[inline(always)]
    pub fn is_mosi0(&self) -> bool {
        *self == P1_24::Mosi0
    }
}
#[doc = "Field `P1_24` writer - Pin function select P1.24."]
pub type P1_24W<'a, REG> = crate::FieldWriter<'a, REG, 2, P1_24, crate::Safe>;
impl<'a, REG> P1_24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P1.24."]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_24::GpioP1)
    }
    #[doc = "MCI2"]
    #[inline(always)]
    pub fn mci2(self) -> &'a mut crate::W<REG> {
        self.variant(P1_24::Mci2)
    }
    #[doc = "PWM1.5"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_24::Pwm1)
    }
    #[doc = "MOSI0"]
    #[inline(always)]
    pub fn mosi0(self) -> &'a mut crate::W<REG> {
        self.variant(P1_24::Mosi0)
    }
}
#[doc = "Pin function select P1.25.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_25 {
    #[doc = "0: GPIO P1.25"]
    GpioP1 = 0,
    #[doc = "1: MCOA1"]
    Mcoa1 = 1,
    #[doc = "3: MAT1.1"]
    Mat1 = 3,
}
impl From<P1_25> for u8 {
    #[inline(always)]
    fn from(variant: P1_25) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_25 {
    type Ux = u8;
}
impl crate::IsEnum for P1_25 {}
#[doc = "Field `P1_25` reader - Pin function select P1.25."]
pub type P1_25R = crate::FieldReader<P1_25>;
impl P1_25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_25 {
        match self.bits {
            0 => P1_25::GpioP1,
            1 => P1_25::Mcoa1,
            3 => P1_25::Mat1,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P1.25"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_25::GpioP1
    }
    #[doc = "MCOA1"]
    #[inline(always)]
    pub fn is_mcoa1(&self) -> bool {
        *self == P1_25::Mcoa1
    }
    #[doc = "MAT1.1"]
    #[inline(always)]
    pub fn is_mat1(&self) -> bool {
        *self == P1_25::Mat1
    }
}
#[doc = "Field `P1_25` writer - Pin function select P1.25."]
pub type P1_25W<'a, REG> = crate::FieldWriter<'a, REG, 2, P1_25>;
impl<'a, REG> P1_25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P1.25"]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_25::GpioP1)
    }
    #[doc = "MCOA1"]
    #[inline(always)]
    pub fn mcoa1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_25::Mcoa1)
    }
    #[doc = "MAT1.1"]
    #[inline(always)]
    pub fn mat1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_25::Mat1)
    }
}
#[doc = "Pin function select P1.26.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_26 {
    #[doc = "0: GPIO P1.26"]
    GpioP1 = 0,
    #[doc = "1: MCOB1"]
    Mcob1 = 1,
    #[doc = "2: PWM1.6"]
    Pwm1 = 2,
    #[doc = "3: CAP0.0"]
    Cap0 = 3,
}
impl From<P1_26> for u8 {
    #[inline(always)]
    fn from(variant: P1_26) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_26 {
    type Ux = u8;
}
impl crate::IsEnum for P1_26 {}
#[doc = "Field `P1_26` reader - Pin function select P1.26."]
pub type P1_26R = crate::FieldReader<P1_26>;
impl P1_26R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_26 {
        match self.bits {
            0 => P1_26::GpioP1,
            1 => P1_26::Mcob1,
            2 => P1_26::Pwm1,
            3 => P1_26::Cap0,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P1.26"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_26::GpioP1
    }
    #[doc = "MCOB1"]
    #[inline(always)]
    pub fn is_mcob1(&self) -> bool {
        *self == P1_26::Mcob1
    }
    #[doc = "PWM1.6"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == P1_26::Pwm1
    }
    #[doc = "CAP0.0"]
    #[inline(always)]
    pub fn is_cap0(&self) -> bool {
        *self == P1_26::Cap0
    }
}
#[doc = "Field `P1_26` writer - Pin function select P1.26."]
pub type P1_26W<'a, REG> = crate::FieldWriter<'a, REG, 2, P1_26, crate::Safe>;
impl<'a, REG> P1_26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P1.26"]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_26::GpioP1)
    }
    #[doc = "MCOB1"]
    #[inline(always)]
    pub fn mcob1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_26::Mcob1)
    }
    #[doc = "PWM1.6"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_26::Pwm1)
    }
    #[doc = "CAP0.0"]
    #[inline(always)]
    pub fn cap0(self) -> &'a mut crate::W<REG> {
        self.variant(P1_26::Cap0)
    }
}
#[doc = "Pin function select P1.27.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_27 {
    #[doc = "0: GPIO P1.27"]
    GpioP1 = 0,
    #[doc = "1: CLKOUT"]
    Clkout = 1,
    #[doc = "2: USB_OVRCR"]
    UsbOvrcr = 2,
    #[doc = "3: CAP0.1"]
    Cap0 = 3,
}
impl From<P1_27> for u8 {
    #[inline(always)]
    fn from(variant: P1_27) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_27 {
    type Ux = u8;
}
impl crate::IsEnum for P1_27 {}
#[doc = "Field `P1_27` reader - Pin function select P1.27."]
pub type P1_27R = crate::FieldReader<P1_27>;
impl P1_27R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_27 {
        match self.bits {
            0 => P1_27::GpioP1,
            1 => P1_27::Clkout,
            2 => P1_27::UsbOvrcr,
            3 => P1_27::Cap0,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P1.27"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_27::GpioP1
    }
    #[doc = "CLKOUT"]
    #[inline(always)]
    pub fn is_clkout(&self) -> bool {
        *self == P1_27::Clkout
    }
    #[doc = "USB_OVRCR"]
    #[inline(always)]
    pub fn is_usb_ovrcr(&self) -> bool {
        *self == P1_27::UsbOvrcr
    }
    #[doc = "CAP0.1"]
    #[inline(always)]
    pub fn is_cap0(&self) -> bool {
        *self == P1_27::Cap0
    }
}
#[doc = "Field `P1_27` writer - Pin function select P1.27."]
pub type P1_27W<'a, REG> = crate::FieldWriter<'a, REG, 2, P1_27, crate::Safe>;
impl<'a, REG> P1_27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P1.27"]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_27::GpioP1)
    }
    #[doc = "CLKOUT"]
    #[inline(always)]
    pub fn clkout(self) -> &'a mut crate::W<REG> {
        self.variant(P1_27::Clkout)
    }
    #[doc = "USB_OVRCR"]
    #[inline(always)]
    pub fn usb_ovrcr(self) -> &'a mut crate::W<REG> {
        self.variant(P1_27::UsbOvrcr)
    }
    #[doc = "CAP0.1"]
    #[inline(always)]
    pub fn cap0(self) -> &'a mut crate::W<REG> {
        self.variant(P1_27::Cap0)
    }
}
#[doc = "Pin function select P1.28.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_28 {
    #[doc = "0: GPIO P1.28"]
    GpioP1 = 0,
    #[doc = "1: MCOA2"]
    Mcoa2 = 1,
    #[doc = "2: PCAP1.0"]
    Pcap1 = 2,
    #[doc = "3: MAT0.0"]
    Mat0 = 3,
}
impl From<P1_28> for u8 {
    #[inline(always)]
    fn from(variant: P1_28) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_28 {
    type Ux = u8;
}
impl crate::IsEnum for P1_28 {}
#[doc = "Field `P1_28` reader - Pin function select P1.28."]
pub type P1_28R = crate::FieldReader<P1_28>;
impl P1_28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_28 {
        match self.bits {
            0 => P1_28::GpioP1,
            1 => P1_28::Mcoa2,
            2 => P1_28::Pcap1,
            3 => P1_28::Mat0,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P1.28"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_28::GpioP1
    }
    #[doc = "MCOA2"]
    #[inline(always)]
    pub fn is_mcoa2(&self) -> bool {
        *self == P1_28::Mcoa2
    }
    #[doc = "PCAP1.0"]
    #[inline(always)]
    pub fn is_pcap1(&self) -> bool {
        *self == P1_28::Pcap1
    }
    #[doc = "MAT0.0"]
    #[inline(always)]
    pub fn is_mat0(&self) -> bool {
        *self == P1_28::Mat0
    }
}
#[doc = "Field `P1_28` writer - Pin function select P1.28."]
pub type P1_28W<'a, REG> = crate::FieldWriter<'a, REG, 2, P1_28, crate::Safe>;
impl<'a, REG> P1_28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P1.28"]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_28::GpioP1)
    }
    #[doc = "MCOA2"]
    #[inline(always)]
    pub fn mcoa2(self) -> &'a mut crate::W<REG> {
        self.variant(P1_28::Mcoa2)
    }
    #[doc = "PCAP1.0"]
    #[inline(always)]
    pub fn pcap1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_28::Pcap1)
    }
    #[doc = "MAT0.0"]
    #[inline(always)]
    pub fn mat0(self) -> &'a mut crate::W<REG> {
        self.variant(P1_28::Mat0)
    }
}
#[doc = "Pin function select P1.29\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_29 {
    #[doc = "0: GPIO P1.29"]
    GpioP1 = 0,
    #[doc = "1: MCOB2"]
    Mcob2 = 1,
    #[doc = "2: PCAP1.1"]
    Pcap1 = 2,
    #[doc = "3: MAT0.1"]
    Mat0 = 3,
}
impl From<P1_29> for u8 {
    #[inline(always)]
    fn from(variant: P1_29) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_29 {
    type Ux = u8;
}
impl crate::IsEnum for P1_29 {}
#[doc = "Field `P1_29` reader - Pin function select P1.29"]
pub type P1_29R = crate::FieldReader<P1_29>;
impl P1_29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_29 {
        match self.bits {
            0 => P1_29::GpioP1,
            1 => P1_29::Mcob2,
            2 => P1_29::Pcap1,
            3 => P1_29::Mat0,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P1.29"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_29::GpioP1
    }
    #[doc = "MCOB2"]
    #[inline(always)]
    pub fn is_mcob2(&self) -> bool {
        *self == P1_29::Mcob2
    }
    #[doc = "PCAP1.1"]
    #[inline(always)]
    pub fn is_pcap1(&self) -> bool {
        *self == P1_29::Pcap1
    }
    #[doc = "MAT0.1"]
    #[inline(always)]
    pub fn is_mat0(&self) -> bool {
        *self == P1_29::Mat0
    }
}
#[doc = "Field `P1_29` writer - Pin function select P1.29"]
pub type P1_29W<'a, REG> = crate::FieldWriter<'a, REG, 2, P1_29, crate::Safe>;
impl<'a, REG> P1_29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P1.29"]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_29::GpioP1)
    }
    #[doc = "MCOB2"]
    #[inline(always)]
    pub fn mcob2(self) -> &'a mut crate::W<REG> {
        self.variant(P1_29::Mcob2)
    }
    #[doc = "PCAP1.1"]
    #[inline(always)]
    pub fn pcap1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_29::Pcap1)
    }
    #[doc = "MAT0.1"]
    #[inline(always)]
    pub fn mat0(self) -> &'a mut crate::W<REG> {
        self.variant(P1_29::Mat0)
    }
}
#[doc = "Pin function select P1.30.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_30 {
    #[doc = "0: GPIO P1.30"]
    GpioP1 = 0,
    #[doc = "2: VBUS"]
    Vbus = 2,
    #[doc = "3: AD0.4"]
    Ad0 = 3,
}
impl From<P1_30> for u8 {
    #[inline(always)]
    fn from(variant: P1_30) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_30 {
    type Ux = u8;
}
impl crate::IsEnum for P1_30 {}
#[doc = "Field `P1_30` reader - Pin function select P1.30."]
pub type P1_30R = crate::FieldReader<P1_30>;
impl P1_30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_30 {
        match self.bits {
            0 => P1_30::GpioP1,
            2 => P1_30::Vbus,
            3 => P1_30::Ad0,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P1.30"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_30::GpioP1
    }
    #[doc = "VBUS"]
    #[inline(always)]
    pub fn is_vbus(&self) -> bool {
        *self == P1_30::Vbus
    }
    #[doc = "AD0.4"]
    #[inline(always)]
    pub fn is_ad0(&self) -> bool {
        *self == P1_30::Ad0
    }
}
#[doc = "Field `P1_30` writer - Pin function select P1.30."]
pub type P1_30W<'a, REG> = crate::FieldWriter<'a, REG, 2, P1_30>;
impl<'a, REG> P1_30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P1.30"]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_30::GpioP1)
    }
    #[doc = "VBUS"]
    #[inline(always)]
    pub fn vbus(self) -> &'a mut crate::W<REG> {
        self.variant(P1_30::Vbus)
    }
    #[doc = "AD0.4"]
    #[inline(always)]
    pub fn ad0(self) -> &'a mut crate::W<REG> {
        self.variant(P1_30::Ad0)
    }
}
#[doc = "Pin function select P1.31.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_31 {
    #[doc = "0: GPIO Port 1.31"]
    GpioPort1 = 0,
    #[doc = "2: SCK1"]
    Sck1 = 2,
    #[doc = "3: AD0.5"]
    Ad0 = 3,
}
impl From<P1_31> for u8 {
    #[inline(always)]
    fn from(variant: P1_31) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_31 {
    type Ux = u8;
}
impl crate::IsEnum for P1_31 {}
#[doc = "Field `P1_31` reader - Pin function select P1.31."]
pub type P1_31R = crate::FieldReader<P1_31>;
impl P1_31R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_31 {
        match self.bits {
            0 => P1_31::GpioPort1,
            2 => P1_31::Sck1,
            3 => P1_31::Ad0,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO Port 1.31"]
    #[inline(always)]
    pub fn is_gpio_port_1(&self) -> bool {
        *self == P1_31::GpioPort1
    }
    #[doc = "SCK1"]
    #[inline(always)]
    pub fn is_sck1(&self) -> bool {
        *self == P1_31::Sck1
    }
    #[doc = "AD0.5"]
    #[inline(always)]
    pub fn is_ad0(&self) -> bool {
        *self == P1_31::Ad0
    }
}
#[doc = "Field `P1_31` writer - Pin function select P1.31."]
pub type P1_31W<'a, REG> = crate::FieldWriter<'a, REG, 2, P1_31>;
impl<'a, REG> P1_31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO Port 1.31"]
    #[inline(always)]
    pub fn gpio_port_1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_31::GpioPort1)
    }
    #[doc = "SCK1"]
    #[inline(always)]
    pub fn sck1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_31::Sck1)
    }
    #[doc = "AD0.5"]
    #[inline(always)]
    pub fn ad0(self) -> &'a mut crate::W<REG> {
        self.variant(P1_31::Ad0)
    }
}
impl R {
    #[doc = "Bits 0:1 - Pin function select P1.16."]
    #[inline(always)]
    pub fn p1_16(&self) -> P1_16R {
        P1_16R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Pin function select P1.17."]
    #[inline(always)]
    pub fn p1_17(&self) -> P1_17R {
        P1_17R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Pin function select P1.18."]
    #[inline(always)]
    pub fn p1_18(&self) -> P1_18R {
        P1_18R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Pin function select P1.19."]
    #[inline(always)]
    pub fn p1_19(&self) -> P1_19R {
        P1_19R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Pin function select P1.20."]
    #[inline(always)]
    pub fn p1_20(&self) -> P1_20R {
        P1_20R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Pin function select P1.21."]
    #[inline(always)]
    pub fn p1_21(&self) -> P1_21R {
        P1_21R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Pin function select P1.22"]
    #[inline(always)]
    pub fn p1_22(&self) -> P1_22R {
        P1_22R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Pin function select P1.23."]
    #[inline(always)]
    pub fn p1_23(&self) -> P1_23R {
        P1_23R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Pin function select P1.24."]
    #[inline(always)]
    pub fn p1_24(&self) -> P1_24R {
        P1_24R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Pin function select P1.25."]
    #[inline(always)]
    pub fn p1_25(&self) -> P1_25R {
        P1_25R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Pin function select P1.26."]
    #[inline(always)]
    pub fn p1_26(&self) -> P1_26R {
        P1_26R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Pin function select P1.27."]
    #[inline(always)]
    pub fn p1_27(&self) -> P1_27R {
        P1_27R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Pin function select P1.28."]
    #[inline(always)]
    pub fn p1_28(&self) -> P1_28R {
        P1_28R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Pin function select P1.29"]
    #[inline(always)]
    pub fn p1_29(&self) -> P1_29R {
        P1_29R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Pin function select P1.30."]
    #[inline(always)]
    pub fn p1_30(&self) -> P1_30R {
        P1_30R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Pin function select P1.31."]
    #[inline(always)]
    pub fn p1_31(&self) -> P1_31R {
        P1_31R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Pin function select P1.16."]
    #[inline(always)]
    #[must_use]
    pub fn p1_16(&mut self) -> P1_16W<Pinsel3Spec> {
        P1_16W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Pin function select P1.17."]
    #[inline(always)]
    #[must_use]
    pub fn p1_17(&mut self) -> P1_17W<Pinsel3Spec> {
        P1_17W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Pin function select P1.18."]
    #[inline(always)]
    #[must_use]
    pub fn p1_18(&mut self) -> P1_18W<Pinsel3Spec> {
        P1_18W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Pin function select P1.19."]
    #[inline(always)]
    #[must_use]
    pub fn p1_19(&mut self) -> P1_19W<Pinsel3Spec> {
        P1_19W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Pin function select P1.20."]
    #[inline(always)]
    #[must_use]
    pub fn p1_20(&mut self) -> P1_20W<Pinsel3Spec> {
        P1_20W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Pin function select P1.21."]
    #[inline(always)]
    #[must_use]
    pub fn p1_21(&mut self) -> P1_21W<Pinsel3Spec> {
        P1_21W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Pin function select P1.22"]
    #[inline(always)]
    #[must_use]
    pub fn p1_22(&mut self) -> P1_22W<Pinsel3Spec> {
        P1_22W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Pin function select P1.23."]
    #[inline(always)]
    #[must_use]
    pub fn p1_23(&mut self) -> P1_23W<Pinsel3Spec> {
        P1_23W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Pin function select P1.24."]
    #[inline(always)]
    #[must_use]
    pub fn p1_24(&mut self) -> P1_24W<Pinsel3Spec> {
        P1_24W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Pin function select P1.25."]
    #[inline(always)]
    #[must_use]
    pub fn p1_25(&mut self) -> P1_25W<Pinsel3Spec> {
        P1_25W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Pin function select P1.26."]
    #[inline(always)]
    #[must_use]
    pub fn p1_26(&mut self) -> P1_26W<Pinsel3Spec> {
        P1_26W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Pin function select P1.27."]
    #[inline(always)]
    #[must_use]
    pub fn p1_27(&mut self) -> P1_27W<Pinsel3Spec> {
        P1_27W::new(self, 22)
    }
    #[doc = "Bits 24:25 - Pin function select P1.28."]
    #[inline(always)]
    #[must_use]
    pub fn p1_28(&mut self) -> P1_28W<Pinsel3Spec> {
        P1_28W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Pin function select P1.29"]
    #[inline(always)]
    #[must_use]
    pub fn p1_29(&mut self) -> P1_29W<Pinsel3Spec> {
        P1_29W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Pin function select P1.30."]
    #[inline(always)]
    #[must_use]
    pub fn p1_30(&mut self) -> P1_30W<Pinsel3Spec> {
        P1_30W::new(self, 28)
    }
    #[doc = "Bits 30:31 - Pin function select P1.31."]
    #[inline(always)]
    #[must_use]
    pub fn p1_31(&mut self) -> P1_31W<Pinsel3Spec> {
        P1_31W::new(self, 30)
    }
}
#[doc = "Pin function select register 3.\n\nYou can [`read`](crate::Reg::read) this register and get [`pinsel3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pinsel3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pinsel3Spec;
impl crate::RegisterSpec for Pinsel3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pinsel3::R`](R) reader structure"]
impl crate::Readable for Pinsel3Spec {}
#[doc = "`write(|w| ..)` method takes [`pinsel3::W`](W) writer structure"]
impl crate::Writable for Pinsel3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PINSEL3 to value 0"]
impl crate::Resettable for Pinsel3Spec {
    const RESET_VALUE: u32 = 0;
}
