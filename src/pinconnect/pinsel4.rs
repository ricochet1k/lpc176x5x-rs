#[doc = "Register `PINSEL4` reader"]
pub type R = crate::R<Pinsel4Spec>;
#[doc = "Register `PINSEL4` writer"]
pub type W = crate::W<Pinsel4Spec>;
#[doc = "Pin function select P2.0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P2_0 {
    #[doc = "0: GPIO P2.0"]
    GpioP2 = 0,
    #[doc = "1: PWM1.1"]
    Pwm1 = 1,
    #[doc = "2: TXD1"]
    Txd1 = 2,
}
impl From<P2_0> for u8 {
    #[inline(always)]
    fn from(variant: P2_0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P2_0 {
    type Ux = u8;
}
impl crate::IsEnum for P2_0 {}
#[doc = "Field `P2_0` reader - Pin function select P2.0."]
pub type P2_0R = crate::FieldReader<P2_0>;
impl P2_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P2_0 {
        match self.bits {
            0 => P2_0::GpioP2,
            1 => P2_0::Pwm1,
            2 => P2_0::Txd1,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P2.0"]
    #[inline(always)]
    pub fn is_gpio_p2(&self) -> bool {
        *self == P2_0::GpioP2
    }
    #[doc = "PWM1.1"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == P2_0::Pwm1
    }
    #[doc = "TXD1"]
    #[inline(always)]
    pub fn is_txd1(&self) -> bool {
        *self == P2_0::Txd1
    }
}
#[doc = "Field `P2_0` writer - Pin function select P2.0."]
pub type P2_0W<'a, REG> = crate::FieldWriter<'a, REG, 2, P2_0>;
impl<'a, REG> P2_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P2.0"]
    #[inline(always)]
    pub fn gpio_p2(self) -> &'a mut crate::W<REG> {
        self.variant(P2_0::GpioP2)
    }
    #[doc = "PWM1.1"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut crate::W<REG> {
        self.variant(P2_0::Pwm1)
    }
    #[doc = "TXD1"]
    #[inline(always)]
    pub fn txd1(self) -> &'a mut crate::W<REG> {
        self.variant(P2_0::Txd1)
    }
}
#[doc = "Pin function select P2.1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P2_1 {
    #[doc = "0: GPIO P2.1"]
    GpioP2 = 0,
    #[doc = "1: PWM1.2"]
    Pwm1 = 1,
    #[doc = "2: RXD1"]
    Rxd1 = 2,
}
impl From<P2_1> for u8 {
    #[inline(always)]
    fn from(variant: P2_1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P2_1 {
    type Ux = u8;
}
impl crate::IsEnum for P2_1 {}
#[doc = "Field `P2_1` reader - Pin function select P2.1."]
pub type P2_1R = crate::FieldReader<P2_1>;
impl P2_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P2_1 {
        match self.bits {
            0 => P2_1::GpioP2,
            1 => P2_1::Pwm1,
            2 => P2_1::Rxd1,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P2.1"]
    #[inline(always)]
    pub fn is_gpio_p2(&self) -> bool {
        *self == P2_1::GpioP2
    }
    #[doc = "PWM1.2"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == P2_1::Pwm1
    }
    #[doc = "RXD1"]
    #[inline(always)]
    pub fn is_rxd1(&self) -> bool {
        *self == P2_1::Rxd1
    }
}
#[doc = "Field `P2_1` writer - Pin function select P2.1."]
pub type P2_1W<'a, REG> = crate::FieldWriter<'a, REG, 2, P2_1>;
impl<'a, REG> P2_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P2.1"]
    #[inline(always)]
    pub fn gpio_p2(self) -> &'a mut crate::W<REG> {
        self.variant(P2_1::GpioP2)
    }
    #[doc = "PWM1.2"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut crate::W<REG> {
        self.variant(P2_1::Pwm1)
    }
    #[doc = "RXD1"]
    #[inline(always)]
    pub fn rxd1(self) -> &'a mut crate::W<REG> {
        self.variant(P2_1::Rxd1)
    }
}
#[doc = "Pin function select P2.2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P2_2 {
    #[doc = "0: GPIO P2.2"]
    GpioP2 = 0,
    #[doc = "1: PWM1.3"]
    Pwm1 = 1,
    #[doc = "2: CTS1"]
    Cts1 = 2,
}
impl From<P2_2> for u8 {
    #[inline(always)]
    fn from(variant: P2_2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P2_2 {
    type Ux = u8;
}
impl crate::IsEnum for P2_2 {}
#[doc = "Field `P2_2` reader - Pin function select P2.2."]
pub type P2_2R = crate::FieldReader<P2_2>;
impl P2_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P2_2 {
        match self.bits {
            0 => P2_2::GpioP2,
            1 => P2_2::Pwm1,
            2 => P2_2::Cts1,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P2.2"]
    #[inline(always)]
    pub fn is_gpio_p2(&self) -> bool {
        *self == P2_2::GpioP2
    }
    #[doc = "PWM1.3"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == P2_2::Pwm1
    }
    #[doc = "CTS1"]
    #[inline(always)]
    pub fn is_cts1(&self) -> bool {
        *self == P2_2::Cts1
    }
}
#[doc = "Field `P2_2` writer - Pin function select P2.2."]
pub type P2_2W<'a, REG> = crate::FieldWriter<'a, REG, 2, P2_2>;
impl<'a, REG> P2_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P2.2"]
    #[inline(always)]
    pub fn gpio_p2(self) -> &'a mut crate::W<REG> {
        self.variant(P2_2::GpioP2)
    }
    #[doc = "PWM1.3"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut crate::W<REG> {
        self.variant(P2_2::Pwm1)
    }
    #[doc = "CTS1"]
    #[inline(always)]
    pub fn cts1(self) -> &'a mut crate::W<REG> {
        self.variant(P2_2::Cts1)
    }
}
#[doc = "Pin function select P2.3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P2_3 {
    #[doc = "0: GPIO P2.3."]
    GpioP2 = 0,
    #[doc = "1: PWM1.4"]
    Pwm1 = 1,
    #[doc = "2: DCD1"]
    Dcd1 = 2,
}
impl From<P2_3> for u8 {
    #[inline(always)]
    fn from(variant: P2_3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P2_3 {
    type Ux = u8;
}
impl crate::IsEnum for P2_3 {}
#[doc = "Field `P2_3` reader - Pin function select P2.3."]
pub type P2_3R = crate::FieldReader<P2_3>;
impl P2_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P2_3 {
        match self.bits {
            0 => P2_3::GpioP2,
            1 => P2_3::Pwm1,
            2 => P2_3::Dcd1,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P2.3."]
    #[inline(always)]
    pub fn is_gpio_p2(&self) -> bool {
        *self == P2_3::GpioP2
    }
    #[doc = "PWM1.4"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == P2_3::Pwm1
    }
    #[doc = "DCD1"]
    #[inline(always)]
    pub fn is_dcd1(&self) -> bool {
        *self == P2_3::Dcd1
    }
}
#[doc = "Field `P2_3` writer - Pin function select P2.3."]
pub type P2_3W<'a, REG> = crate::FieldWriter<'a, REG, 2, P2_3>;
impl<'a, REG> P2_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P2.3."]
    #[inline(always)]
    pub fn gpio_p2(self) -> &'a mut crate::W<REG> {
        self.variant(P2_3::GpioP2)
    }
    #[doc = "PWM1.4"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut crate::W<REG> {
        self.variant(P2_3::Pwm1)
    }
    #[doc = "DCD1"]
    #[inline(always)]
    pub fn dcd1(self) -> &'a mut crate::W<REG> {
        self.variant(P2_3::Dcd1)
    }
}
#[doc = "Pin function select P2.4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P2_4 {
    #[doc = "0: GPIO P2.4."]
    GpioP2 = 0,
    #[doc = "1: PWM1.5"]
    Pwm1 = 1,
    #[doc = "2: DSR1"]
    Dsr1 = 2,
}
impl From<P2_4> for u8 {
    #[inline(always)]
    fn from(variant: P2_4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P2_4 {
    type Ux = u8;
}
impl crate::IsEnum for P2_4 {}
#[doc = "Field `P2_4` reader - Pin function select P2.4."]
pub type P2_4R = crate::FieldReader<P2_4>;
impl P2_4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P2_4 {
        match self.bits {
            0 => P2_4::GpioP2,
            1 => P2_4::Pwm1,
            2 => P2_4::Dsr1,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P2.4."]
    #[inline(always)]
    pub fn is_gpio_p2(&self) -> bool {
        *self == P2_4::GpioP2
    }
    #[doc = "PWM1.5"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == P2_4::Pwm1
    }
    #[doc = "DSR1"]
    #[inline(always)]
    pub fn is_dsr1(&self) -> bool {
        *self == P2_4::Dsr1
    }
}
#[doc = "Field `P2_4` writer - Pin function select P2.4."]
pub type P2_4W<'a, REG> = crate::FieldWriter<'a, REG, 2, P2_4>;
impl<'a, REG> P2_4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P2.4."]
    #[inline(always)]
    pub fn gpio_p2(self) -> &'a mut crate::W<REG> {
        self.variant(P2_4::GpioP2)
    }
    #[doc = "PWM1.5"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut crate::W<REG> {
        self.variant(P2_4::Pwm1)
    }
    #[doc = "DSR1"]
    #[inline(always)]
    pub fn dsr1(self) -> &'a mut crate::W<REG> {
        self.variant(P2_4::Dsr1)
    }
}
#[doc = "Pin function select P2.5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P2_5 {
    #[doc = "0: GPIO P2.5."]
    GpioP2 = 0,
    #[doc = "1: PWM1.6"]
    Pwm1 = 1,
    #[doc = "2: DTR1"]
    Dtr1 = 2,
}
impl From<P2_5> for u8 {
    #[inline(always)]
    fn from(variant: P2_5) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P2_5 {
    type Ux = u8;
}
impl crate::IsEnum for P2_5 {}
#[doc = "Field `P2_5` reader - Pin function select P2.5."]
pub type P2_5R = crate::FieldReader<P2_5>;
impl P2_5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P2_5 {
        match self.bits {
            0 => P2_5::GpioP2,
            1 => P2_5::Pwm1,
            2 => P2_5::Dtr1,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P2.5."]
    #[inline(always)]
    pub fn is_gpio_p2(&self) -> bool {
        *self == P2_5::GpioP2
    }
    #[doc = "PWM1.6"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == P2_5::Pwm1
    }
    #[doc = "DTR1"]
    #[inline(always)]
    pub fn is_dtr1(&self) -> bool {
        *self == P2_5::Dtr1
    }
}
#[doc = "Field `P2_5` writer - Pin function select P2.5."]
pub type P2_5W<'a, REG> = crate::FieldWriter<'a, REG, 2, P2_5>;
impl<'a, REG> P2_5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P2.5."]
    #[inline(always)]
    pub fn gpio_p2(self) -> &'a mut crate::W<REG> {
        self.variant(P2_5::GpioP2)
    }
    #[doc = "PWM1.6"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut crate::W<REG> {
        self.variant(P2_5::Pwm1)
    }
    #[doc = "DTR1"]
    #[inline(always)]
    pub fn dtr1(self) -> &'a mut crate::W<REG> {
        self.variant(P2_5::Dtr1)
    }
}
#[doc = "Pin function select P2.6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P2_6 {
    #[doc = "0: GPIO P2.6."]
    GpioP2 = 0,
    #[doc = "1: PCAP1.0"]
    Pcap1 = 1,
    #[doc = "2: RI1"]
    Ri1 = 2,
}
impl From<P2_6> for u8 {
    #[inline(always)]
    fn from(variant: P2_6) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P2_6 {
    type Ux = u8;
}
impl crate::IsEnum for P2_6 {}
#[doc = "Field `P2_6` reader - Pin function select P2.6."]
pub type P2_6R = crate::FieldReader<P2_6>;
impl P2_6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P2_6 {
        match self.bits {
            0 => P2_6::GpioP2,
            1 => P2_6::Pcap1,
            2 => P2_6::Ri1,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P2.6."]
    #[inline(always)]
    pub fn is_gpio_p2(&self) -> bool {
        *self == P2_6::GpioP2
    }
    #[doc = "PCAP1.0"]
    #[inline(always)]
    pub fn is_pcap1(&self) -> bool {
        *self == P2_6::Pcap1
    }
    #[doc = "RI1"]
    #[inline(always)]
    pub fn is_ri1(&self) -> bool {
        *self == P2_6::Ri1
    }
}
#[doc = "Field `P2_6` writer - Pin function select P2.6."]
pub type P2_6W<'a, REG> = crate::FieldWriter<'a, REG, 2, P2_6>;
impl<'a, REG> P2_6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P2.6."]
    #[inline(always)]
    pub fn gpio_p2(self) -> &'a mut crate::W<REG> {
        self.variant(P2_6::GpioP2)
    }
    #[doc = "PCAP1.0"]
    #[inline(always)]
    pub fn pcap1(self) -> &'a mut crate::W<REG> {
        self.variant(P2_6::Pcap1)
    }
    #[doc = "RI1"]
    #[inline(always)]
    pub fn ri1(self) -> &'a mut crate::W<REG> {
        self.variant(P2_6::Ri1)
    }
}
#[doc = "Pin function select P2.7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P2_7 {
    #[doc = "0: GPIO P2.7."]
    GpioP2 = 0,
    #[doc = "1: RD2"]
    Rd2 = 1,
    #[doc = "2: RTS1"]
    Rts1 = 2,
}
impl From<P2_7> for u8 {
    #[inline(always)]
    fn from(variant: P2_7) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P2_7 {
    type Ux = u8;
}
impl crate::IsEnum for P2_7 {}
#[doc = "Field `P2_7` reader - Pin function select P2.7."]
pub type P2_7R = crate::FieldReader<P2_7>;
impl P2_7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P2_7 {
        match self.bits {
            0 => P2_7::GpioP2,
            1 => P2_7::Rd2,
            2 => P2_7::Rts1,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P2.7."]
    #[inline(always)]
    pub fn is_gpio_p2(&self) -> bool {
        *self == P2_7::GpioP2
    }
    #[doc = "RD2"]
    #[inline(always)]
    pub fn is_rd2(&self) -> bool {
        *self == P2_7::Rd2
    }
    #[doc = "RTS1"]
    #[inline(always)]
    pub fn is_rts1(&self) -> bool {
        *self == P2_7::Rts1
    }
}
#[doc = "Field `P2_7` writer - Pin function select P2.7."]
pub type P2_7W<'a, REG> = crate::FieldWriter<'a, REG, 2, P2_7>;
impl<'a, REG> P2_7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P2.7."]
    #[inline(always)]
    pub fn gpio_p2(self) -> &'a mut crate::W<REG> {
        self.variant(P2_7::GpioP2)
    }
    #[doc = "RD2"]
    #[inline(always)]
    pub fn rd2(self) -> &'a mut crate::W<REG> {
        self.variant(P2_7::Rd2)
    }
    #[doc = "RTS1"]
    #[inline(always)]
    pub fn rts1(self) -> &'a mut crate::W<REG> {
        self.variant(P2_7::Rts1)
    }
}
#[doc = "Pin function select P2.8.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P2_8 {
    #[doc = "0: GPIO P2.8."]
    GpioP2 = 0,
    #[doc = "1: TD2"]
    Td2 = 1,
    #[doc = "2: TXD2"]
    Txd2 = 2,
    #[doc = "3: ENET_MDC"]
    EnetMdc = 3,
}
impl From<P2_8> for u8 {
    #[inline(always)]
    fn from(variant: P2_8) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P2_8 {
    type Ux = u8;
}
impl crate::IsEnum for P2_8 {}
#[doc = "Field `P2_8` reader - Pin function select P2.8."]
pub type P2_8R = crate::FieldReader<P2_8>;
impl P2_8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P2_8 {
        match self.bits {
            0 => P2_8::GpioP2,
            1 => P2_8::Td2,
            2 => P2_8::Txd2,
            3 => P2_8::EnetMdc,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P2.8."]
    #[inline(always)]
    pub fn is_gpio_p2(&self) -> bool {
        *self == P2_8::GpioP2
    }
    #[doc = "TD2"]
    #[inline(always)]
    pub fn is_td2(&self) -> bool {
        *self == P2_8::Td2
    }
    #[doc = "TXD2"]
    #[inline(always)]
    pub fn is_txd2(&self) -> bool {
        *self == P2_8::Txd2
    }
    #[doc = "ENET_MDC"]
    #[inline(always)]
    pub fn is_enet_mdc(&self) -> bool {
        *self == P2_8::EnetMdc
    }
}
#[doc = "Field `P2_8` writer - Pin function select P2.8."]
pub type P2_8W<'a, REG> = crate::FieldWriter<'a, REG, 2, P2_8, crate::Safe>;
impl<'a, REG> P2_8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P2.8."]
    #[inline(always)]
    pub fn gpio_p2(self) -> &'a mut crate::W<REG> {
        self.variant(P2_8::GpioP2)
    }
    #[doc = "TD2"]
    #[inline(always)]
    pub fn td2(self) -> &'a mut crate::W<REG> {
        self.variant(P2_8::Td2)
    }
    #[doc = "TXD2"]
    #[inline(always)]
    pub fn txd2(self) -> &'a mut crate::W<REG> {
        self.variant(P2_8::Txd2)
    }
    #[doc = "ENET_MDC"]
    #[inline(always)]
    pub fn enet_mdc(self) -> &'a mut crate::W<REG> {
        self.variant(P2_8::EnetMdc)
    }
}
#[doc = "Pin function select P2.9.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P2_9 {
    #[doc = "0: GPIO P2.9"]
    GpioP2 = 0,
    #[doc = "1: USB_CONNECT"]
    UsbConnect = 1,
    #[doc = "2: RXD2"]
    Rxd2 = 2,
    #[doc = "3: ENET_MDIO"]
    EnetMdio = 3,
}
impl From<P2_9> for u8 {
    #[inline(always)]
    fn from(variant: P2_9) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P2_9 {
    type Ux = u8;
}
impl crate::IsEnum for P2_9 {}
#[doc = "Field `P2_9` reader - Pin function select P2.9."]
pub type P2_9R = crate::FieldReader<P2_9>;
impl P2_9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P2_9 {
        match self.bits {
            0 => P2_9::GpioP2,
            1 => P2_9::UsbConnect,
            2 => P2_9::Rxd2,
            3 => P2_9::EnetMdio,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P2.9"]
    #[inline(always)]
    pub fn is_gpio_p2(&self) -> bool {
        *self == P2_9::GpioP2
    }
    #[doc = "USB_CONNECT"]
    #[inline(always)]
    pub fn is_usb_connect(&self) -> bool {
        *self == P2_9::UsbConnect
    }
    #[doc = "RXD2"]
    #[inline(always)]
    pub fn is_rxd2(&self) -> bool {
        *self == P2_9::Rxd2
    }
    #[doc = "ENET_MDIO"]
    #[inline(always)]
    pub fn is_enet_mdio(&self) -> bool {
        *self == P2_9::EnetMdio
    }
}
#[doc = "Field `P2_9` writer - Pin function select P2.9."]
pub type P2_9W<'a, REG> = crate::FieldWriter<'a, REG, 2, P2_9, crate::Safe>;
impl<'a, REG> P2_9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P2.9"]
    #[inline(always)]
    pub fn gpio_p2(self) -> &'a mut crate::W<REG> {
        self.variant(P2_9::GpioP2)
    }
    #[doc = "USB_CONNECT"]
    #[inline(always)]
    pub fn usb_connect(self) -> &'a mut crate::W<REG> {
        self.variant(P2_9::UsbConnect)
    }
    #[doc = "RXD2"]
    #[inline(always)]
    pub fn rxd2(self) -> &'a mut crate::W<REG> {
        self.variant(P2_9::Rxd2)
    }
    #[doc = "ENET_MDIO"]
    #[inline(always)]
    pub fn enet_mdio(self) -> &'a mut crate::W<REG> {
        self.variant(P2_9::EnetMdio)
    }
}
#[doc = "Pin function select P2.10.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P2_10 {
    #[doc = "0: GPIO P2.10"]
    GpioP2 = 0,
    #[doc = "1: EINT0"]
    Eint0 = 1,
    #[doc = "2: NMI"]
    Nmi = 2,
}
impl From<P2_10> for u8 {
    #[inline(always)]
    fn from(variant: P2_10) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P2_10 {
    type Ux = u8;
}
impl crate::IsEnum for P2_10 {}
#[doc = "Field `P2_10` reader - Pin function select P2.10."]
pub type P2_10R = crate::FieldReader<P2_10>;
impl P2_10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P2_10 {
        match self.bits {
            0 => P2_10::GpioP2,
            1 => P2_10::Eint0,
            2 => P2_10::Nmi,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P2.10"]
    #[inline(always)]
    pub fn is_gpio_p2(&self) -> bool {
        *self == P2_10::GpioP2
    }
    #[doc = "EINT0"]
    #[inline(always)]
    pub fn is_eint0(&self) -> bool {
        *self == P2_10::Eint0
    }
    #[doc = "NMI"]
    #[inline(always)]
    pub fn is_nmi(&self) -> bool {
        *self == P2_10::Nmi
    }
}
#[doc = "Field `P2_10` writer - Pin function select P2.10."]
pub type P2_10W<'a, REG> = crate::FieldWriter<'a, REG, 2, P2_10>;
impl<'a, REG> P2_10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P2.10"]
    #[inline(always)]
    pub fn gpio_p2(self) -> &'a mut crate::W<REG> {
        self.variant(P2_10::GpioP2)
    }
    #[doc = "EINT0"]
    #[inline(always)]
    pub fn eint0(self) -> &'a mut crate::W<REG> {
        self.variant(P2_10::Eint0)
    }
    #[doc = "NMI"]
    #[inline(always)]
    pub fn nmi(self) -> &'a mut crate::W<REG> {
        self.variant(P2_10::Nmi)
    }
}
#[doc = "Pin function select P2.11.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P2_11 {
    #[doc = "0: GPIO P2.11"]
    GpioP2 = 0,
    #[doc = "1: EINT1"]
    Eint1 = 1,
    #[doc = "3: I2STX_CLK"]
    I2stxClk = 3,
}
impl From<P2_11> for u8 {
    #[inline(always)]
    fn from(variant: P2_11) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P2_11 {
    type Ux = u8;
}
impl crate::IsEnum for P2_11 {}
#[doc = "Field `P2_11` reader - Pin function select P2.11."]
pub type P2_11R = crate::FieldReader<P2_11>;
impl P2_11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P2_11 {
        match self.bits {
            0 => P2_11::GpioP2,
            1 => P2_11::Eint1,
            3 => P2_11::I2stxClk,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P2.11"]
    #[inline(always)]
    pub fn is_gpio_p2(&self) -> bool {
        *self == P2_11::GpioP2
    }
    #[doc = "EINT1"]
    #[inline(always)]
    pub fn is_eint1(&self) -> bool {
        *self == P2_11::Eint1
    }
    #[doc = "I2STX_CLK"]
    #[inline(always)]
    pub fn is_i2stx_clk(&self) -> bool {
        *self == P2_11::I2stxClk
    }
}
#[doc = "Field `P2_11` writer - Pin function select P2.11."]
pub type P2_11W<'a, REG> = crate::FieldWriter<'a, REG, 2, P2_11>;
impl<'a, REG> P2_11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P2.11"]
    #[inline(always)]
    pub fn gpio_p2(self) -> &'a mut crate::W<REG> {
        self.variant(P2_11::GpioP2)
    }
    #[doc = "EINT1"]
    #[inline(always)]
    pub fn eint1(self) -> &'a mut crate::W<REG> {
        self.variant(P2_11::Eint1)
    }
    #[doc = "I2STX_CLK"]
    #[inline(always)]
    pub fn i2stx_clk(self) -> &'a mut crate::W<REG> {
        self.variant(P2_11::I2stxClk)
    }
}
#[doc = "Pin function select P2.12.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P2_12 {
    #[doc = "0: GPIO P2.12"]
    GpioP2 = 0,
    #[doc = "1: EINT2"]
    Eint2 = 1,
    #[doc = "3: I2STX_WS"]
    I2stxWs = 3,
}
impl From<P2_12> for u8 {
    #[inline(always)]
    fn from(variant: P2_12) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P2_12 {
    type Ux = u8;
}
impl crate::IsEnum for P2_12 {}
#[doc = "Field `P2_12` reader - Pin function select P2.12."]
pub type P2_12R = crate::FieldReader<P2_12>;
impl P2_12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P2_12 {
        match self.bits {
            0 => P2_12::GpioP2,
            1 => P2_12::Eint2,
            3 => P2_12::I2stxWs,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P2.12"]
    #[inline(always)]
    pub fn is_gpio_p2(&self) -> bool {
        *self == P2_12::GpioP2
    }
    #[doc = "EINT2"]
    #[inline(always)]
    pub fn is_eint2(&self) -> bool {
        *self == P2_12::Eint2
    }
    #[doc = "I2STX_WS"]
    #[inline(always)]
    pub fn is_i2stx_ws(&self) -> bool {
        *self == P2_12::I2stxWs
    }
}
#[doc = "Field `P2_12` writer - Pin function select P2.12."]
pub type P2_12W<'a, REG> = crate::FieldWriter<'a, REG, 2, P2_12>;
impl<'a, REG> P2_12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P2.12"]
    #[inline(always)]
    pub fn gpio_p2(self) -> &'a mut crate::W<REG> {
        self.variant(P2_12::GpioP2)
    }
    #[doc = "EINT2"]
    #[inline(always)]
    pub fn eint2(self) -> &'a mut crate::W<REG> {
        self.variant(P2_12::Eint2)
    }
    #[doc = "I2STX_WS"]
    #[inline(always)]
    pub fn i2stx_ws(self) -> &'a mut crate::W<REG> {
        self.variant(P2_12::I2stxWs)
    }
}
#[doc = "Pin function select P2.13.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P2_13 {
    #[doc = "0: GPIO P2.13"]
    GpioP2 = 0,
    #[doc = "1: EINT3"]
    Eint3 = 1,
    #[doc = "3: I2STX_SDA"]
    I2stxSda = 3,
}
impl From<P2_13> for u8 {
    #[inline(always)]
    fn from(variant: P2_13) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P2_13 {
    type Ux = u8;
}
impl crate::IsEnum for P2_13 {}
#[doc = "Field `P2_13` reader - Pin function select P2.13."]
pub type P2_13R = crate::FieldReader<P2_13>;
impl P2_13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P2_13 {
        match self.bits {
            0 => P2_13::GpioP2,
            1 => P2_13::Eint3,
            3 => P2_13::I2stxSda,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P2.13"]
    #[inline(always)]
    pub fn is_gpio_p2(&self) -> bool {
        *self == P2_13::GpioP2
    }
    #[doc = "EINT3"]
    #[inline(always)]
    pub fn is_eint3(&self) -> bool {
        *self == P2_13::Eint3
    }
    #[doc = "I2STX_SDA"]
    #[inline(always)]
    pub fn is_i2stx_sda(&self) -> bool {
        *self == P2_13::I2stxSda
    }
}
#[doc = "Field `P2_13` writer - Pin function select P2.13."]
pub type P2_13W<'a, REG> = crate::FieldWriter<'a, REG, 2, P2_13>;
impl<'a, REG> P2_13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P2.13"]
    #[inline(always)]
    pub fn gpio_p2(self) -> &'a mut crate::W<REG> {
        self.variant(P2_13::GpioP2)
    }
    #[doc = "EINT3"]
    #[inline(always)]
    pub fn eint3(self) -> &'a mut crate::W<REG> {
        self.variant(P2_13::Eint3)
    }
    #[doc = "I2STX_SDA"]
    #[inline(always)]
    pub fn i2stx_sda(self) -> &'a mut crate::W<REG> {
        self.variant(P2_13::I2stxSda)
    }
}
impl R {
    #[doc = "Bits 0:1 - Pin function select P2.0."]
    #[inline(always)]
    pub fn p2_0(&self) -> P2_0R {
        P2_0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Pin function select P2.1."]
    #[inline(always)]
    pub fn p2_1(&self) -> P2_1R {
        P2_1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Pin function select P2.2."]
    #[inline(always)]
    pub fn p2_2(&self) -> P2_2R {
        P2_2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Pin function select P2.3."]
    #[inline(always)]
    pub fn p2_3(&self) -> P2_3R {
        P2_3R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Pin function select P2.4."]
    #[inline(always)]
    pub fn p2_4(&self) -> P2_4R {
        P2_4R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Pin function select P2.5."]
    #[inline(always)]
    pub fn p2_5(&self) -> P2_5R {
        P2_5R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Pin function select P2.6."]
    #[inline(always)]
    pub fn p2_6(&self) -> P2_6R {
        P2_6R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Pin function select P2.7."]
    #[inline(always)]
    pub fn p2_7(&self) -> P2_7R {
        P2_7R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Pin function select P2.8."]
    #[inline(always)]
    pub fn p2_8(&self) -> P2_8R {
        P2_8R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Pin function select P2.9."]
    #[inline(always)]
    pub fn p2_9(&self) -> P2_9R {
        P2_9R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Pin function select P2.10."]
    #[inline(always)]
    pub fn p2_10(&self) -> P2_10R {
        P2_10R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Pin function select P2.11."]
    #[inline(always)]
    pub fn p2_11(&self) -> P2_11R {
        P2_11R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Pin function select P2.12."]
    #[inline(always)]
    pub fn p2_12(&self) -> P2_12R {
        P2_12R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Pin function select P2.13."]
    #[inline(always)]
    pub fn p2_13(&self) -> P2_13R {
        P2_13R::new(((self.bits >> 26) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Pin function select P2.0."]
    #[inline(always)]
    #[must_use]
    pub fn p2_0(&mut self) -> P2_0W<Pinsel4Spec> {
        P2_0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Pin function select P2.1."]
    #[inline(always)]
    #[must_use]
    pub fn p2_1(&mut self) -> P2_1W<Pinsel4Spec> {
        P2_1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Pin function select P2.2."]
    #[inline(always)]
    #[must_use]
    pub fn p2_2(&mut self) -> P2_2W<Pinsel4Spec> {
        P2_2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Pin function select P2.3."]
    #[inline(always)]
    #[must_use]
    pub fn p2_3(&mut self) -> P2_3W<Pinsel4Spec> {
        P2_3W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Pin function select P2.4."]
    #[inline(always)]
    #[must_use]
    pub fn p2_4(&mut self) -> P2_4W<Pinsel4Spec> {
        P2_4W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Pin function select P2.5."]
    #[inline(always)]
    #[must_use]
    pub fn p2_5(&mut self) -> P2_5W<Pinsel4Spec> {
        P2_5W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Pin function select P2.6."]
    #[inline(always)]
    #[must_use]
    pub fn p2_6(&mut self) -> P2_6W<Pinsel4Spec> {
        P2_6W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Pin function select P2.7."]
    #[inline(always)]
    #[must_use]
    pub fn p2_7(&mut self) -> P2_7W<Pinsel4Spec> {
        P2_7W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Pin function select P2.8."]
    #[inline(always)]
    #[must_use]
    pub fn p2_8(&mut self) -> P2_8W<Pinsel4Spec> {
        P2_8W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Pin function select P2.9."]
    #[inline(always)]
    #[must_use]
    pub fn p2_9(&mut self) -> P2_9W<Pinsel4Spec> {
        P2_9W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Pin function select P2.10."]
    #[inline(always)]
    #[must_use]
    pub fn p2_10(&mut self) -> P2_10W<Pinsel4Spec> {
        P2_10W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Pin function select P2.11."]
    #[inline(always)]
    #[must_use]
    pub fn p2_11(&mut self) -> P2_11W<Pinsel4Spec> {
        P2_11W::new(self, 22)
    }
    #[doc = "Bits 24:25 - Pin function select P2.12."]
    #[inline(always)]
    #[must_use]
    pub fn p2_12(&mut self) -> P2_12W<Pinsel4Spec> {
        P2_12W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Pin function select P2.13."]
    #[inline(always)]
    #[must_use]
    pub fn p2_13(&mut self) -> P2_13W<Pinsel4Spec> {
        P2_13W::new(self, 26)
    }
}
#[doc = "Pin function select register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`pinsel4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pinsel4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pinsel4Spec;
impl crate::RegisterSpec for Pinsel4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pinsel4::R`](R) reader structure"]
impl crate::Readable for Pinsel4Spec {}
#[doc = "`write(|w| ..)` method takes [`pinsel4::W`](W) writer structure"]
impl crate::Writable for Pinsel4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PINSEL4 to value 0"]
impl crate::Resettable for Pinsel4Spec {
    const RESET_VALUE: u32 = 0;
}
