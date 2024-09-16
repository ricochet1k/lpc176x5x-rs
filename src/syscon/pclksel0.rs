#[doc = "Register `PCLKSEL0` reader"]
pub type R = crate::R<Pclksel0Spec>;
#[doc = "Register `PCLKSEL0` writer"]
pub type W = crate::W<Pclksel0Spec>;
#[doc = "Peripheral clock selection for WDT.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PclkWdt {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CclkDiv4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    Cclk = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CclkDiv2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CclkDiv8 = 3,
}
impl From<PclkWdt> for u8 {
    #[inline(always)]
    fn from(variant: PclkWdt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PclkWdt {
    type Ux = u8;
}
impl crate::IsEnum for PclkWdt {}
#[doc = "Field `PCLK_WDT` reader - Peripheral clock selection for WDT."]
pub type PclkWdtR = crate::FieldReader<PclkWdt>;
impl PclkWdtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PclkWdt {
        match self.bits {
            0 => PclkWdt::CclkDiv4,
            1 => PclkWdt::Cclk,
            2 => PclkWdt::CclkDiv2,
            3 => PclkWdt::CclkDiv8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PclkWdt::CclkDiv4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PclkWdt::Cclk
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PclkWdt::CclkDiv2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PclkWdt::CclkDiv8
    }
}
#[doc = "Field `PCLK_WDT` writer - Peripheral clock selection for WDT."]
pub type PclkWdtW<'a, REG> = crate::FieldWriter<'a, REG, 2, PclkWdt, crate::Safe>;
impl<'a, REG> PclkWdtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(PclkWdt::CclkDiv4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(PclkWdt::Cclk)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(PclkWdt::CclkDiv2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(PclkWdt::CclkDiv8)
    }
}
#[doc = "Peripheral clock selection for TIMER0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PclkTimer0 {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CclkDiv4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    Cclk = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CclkDiv2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CclkDiv8 = 3,
}
impl From<PclkTimer0> for u8 {
    #[inline(always)]
    fn from(variant: PclkTimer0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PclkTimer0 {
    type Ux = u8;
}
impl crate::IsEnum for PclkTimer0 {}
#[doc = "Field `PCLK_TIMER0` reader - Peripheral clock selection for TIMER0."]
pub type PclkTimer0R = crate::FieldReader<PclkTimer0>;
impl PclkTimer0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PclkTimer0 {
        match self.bits {
            0 => PclkTimer0::CclkDiv4,
            1 => PclkTimer0::Cclk,
            2 => PclkTimer0::CclkDiv2,
            3 => PclkTimer0::CclkDiv8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PclkTimer0::CclkDiv4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PclkTimer0::Cclk
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PclkTimer0::CclkDiv2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PclkTimer0::CclkDiv8
    }
}
#[doc = "Field `PCLK_TIMER0` writer - Peripheral clock selection for TIMER0."]
pub type PclkTimer0W<'a, REG> = crate::FieldWriter<'a, REG, 2, PclkTimer0, crate::Safe>;
impl<'a, REG> PclkTimer0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(PclkTimer0::CclkDiv4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(PclkTimer0::Cclk)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(PclkTimer0::CclkDiv2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(PclkTimer0::CclkDiv8)
    }
}
#[doc = "Peripheral clock selection for TIMER1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PclkTimer1 {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CclkDiv4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    Cclk = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CclkDiv2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CclkDiv8 = 3,
}
impl From<PclkTimer1> for u8 {
    #[inline(always)]
    fn from(variant: PclkTimer1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PclkTimer1 {
    type Ux = u8;
}
impl crate::IsEnum for PclkTimer1 {}
#[doc = "Field `PCLK_TIMER1` reader - Peripheral clock selection for TIMER1."]
pub type PclkTimer1R = crate::FieldReader<PclkTimer1>;
impl PclkTimer1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PclkTimer1 {
        match self.bits {
            0 => PclkTimer1::CclkDiv4,
            1 => PclkTimer1::Cclk,
            2 => PclkTimer1::CclkDiv2,
            3 => PclkTimer1::CclkDiv8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PclkTimer1::CclkDiv4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PclkTimer1::Cclk
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PclkTimer1::CclkDiv2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PclkTimer1::CclkDiv8
    }
}
#[doc = "Field `PCLK_TIMER1` writer - Peripheral clock selection for TIMER1."]
pub type PclkTimer1W<'a, REG> = crate::FieldWriter<'a, REG, 2, PclkTimer1, crate::Safe>;
impl<'a, REG> PclkTimer1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(PclkTimer1::CclkDiv4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(PclkTimer1::Cclk)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(PclkTimer1::CclkDiv2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(PclkTimer1::CclkDiv8)
    }
}
#[doc = "Peripheral clock selection for UART0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PclkUart0 {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CclkDiv4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    Cclk = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CclkDiv2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CclkDiv8 = 3,
}
impl From<PclkUart0> for u8 {
    #[inline(always)]
    fn from(variant: PclkUart0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PclkUart0 {
    type Ux = u8;
}
impl crate::IsEnum for PclkUart0 {}
#[doc = "Field `PCLK_UART0` reader - Peripheral clock selection for UART0."]
pub type PclkUart0R = crate::FieldReader<PclkUart0>;
impl PclkUart0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PclkUart0 {
        match self.bits {
            0 => PclkUart0::CclkDiv4,
            1 => PclkUart0::Cclk,
            2 => PclkUart0::CclkDiv2,
            3 => PclkUart0::CclkDiv8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PclkUart0::CclkDiv4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PclkUart0::Cclk
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PclkUart0::CclkDiv2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PclkUart0::CclkDiv8
    }
}
#[doc = "Field `PCLK_UART0` writer - Peripheral clock selection for UART0."]
pub type PclkUart0W<'a, REG> = crate::FieldWriter<'a, REG, 2, PclkUart0, crate::Safe>;
impl<'a, REG> PclkUart0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(PclkUart0::CclkDiv4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(PclkUart0::Cclk)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(PclkUart0::CclkDiv2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(PclkUart0::CclkDiv8)
    }
}
#[doc = "Peripheral clock selection for UART1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PclkUart1 {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CclkDiv4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    Cclk = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CclkDiv2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CclkDiv8 = 3,
}
impl From<PclkUart1> for u8 {
    #[inline(always)]
    fn from(variant: PclkUart1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PclkUart1 {
    type Ux = u8;
}
impl crate::IsEnum for PclkUart1 {}
#[doc = "Field `PCLK_UART1` reader - Peripheral clock selection for UART1."]
pub type PclkUart1R = crate::FieldReader<PclkUart1>;
impl PclkUart1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PclkUart1 {
        match self.bits {
            0 => PclkUart1::CclkDiv4,
            1 => PclkUart1::Cclk,
            2 => PclkUart1::CclkDiv2,
            3 => PclkUart1::CclkDiv8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PclkUart1::CclkDiv4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PclkUart1::Cclk
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PclkUart1::CclkDiv2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PclkUart1::CclkDiv8
    }
}
#[doc = "Field `PCLK_UART1` writer - Peripheral clock selection for UART1."]
pub type PclkUart1W<'a, REG> = crate::FieldWriter<'a, REG, 2, PclkUart1, crate::Safe>;
impl<'a, REG> PclkUart1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(PclkUart1::CclkDiv4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(PclkUart1::Cclk)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(PclkUart1::CclkDiv2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(PclkUart1::CclkDiv8)
    }
}
#[doc = "Peripheral clock selection for PWM1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PclkPwm1 {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CclkDiv4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    Cclk = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CclkDiv2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CclkDiv8 = 3,
}
impl From<PclkPwm1> for u8 {
    #[inline(always)]
    fn from(variant: PclkPwm1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PclkPwm1 {
    type Ux = u8;
}
impl crate::IsEnum for PclkPwm1 {}
#[doc = "Field `PCLK_PWM1` reader - Peripheral clock selection for PWM1."]
pub type PclkPwm1R = crate::FieldReader<PclkPwm1>;
impl PclkPwm1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PclkPwm1 {
        match self.bits {
            0 => PclkPwm1::CclkDiv4,
            1 => PclkPwm1::Cclk,
            2 => PclkPwm1::CclkDiv2,
            3 => PclkPwm1::CclkDiv8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PclkPwm1::CclkDiv4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PclkPwm1::Cclk
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PclkPwm1::CclkDiv2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PclkPwm1::CclkDiv8
    }
}
#[doc = "Field `PCLK_PWM1` writer - Peripheral clock selection for PWM1."]
pub type PclkPwm1W<'a, REG> = crate::FieldWriter<'a, REG, 2, PclkPwm1, crate::Safe>;
impl<'a, REG> PclkPwm1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(PclkPwm1::CclkDiv4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(PclkPwm1::Cclk)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(PclkPwm1::CclkDiv2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(PclkPwm1::CclkDiv8)
    }
}
#[doc = "Peripheral clock selection for I2C0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PclkI2c0 {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CclkDiv4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    Cclk = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CclkDiv2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CclkDiv8 = 3,
}
impl From<PclkI2c0> for u8 {
    #[inline(always)]
    fn from(variant: PclkI2c0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PclkI2c0 {
    type Ux = u8;
}
impl crate::IsEnum for PclkI2c0 {}
#[doc = "Field `PCLK_I2C0` reader - Peripheral clock selection for I2C0."]
pub type PclkI2c0R = crate::FieldReader<PclkI2c0>;
impl PclkI2c0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PclkI2c0 {
        match self.bits {
            0 => PclkI2c0::CclkDiv4,
            1 => PclkI2c0::Cclk,
            2 => PclkI2c0::CclkDiv2,
            3 => PclkI2c0::CclkDiv8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PclkI2c0::CclkDiv4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PclkI2c0::Cclk
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PclkI2c0::CclkDiv2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PclkI2c0::CclkDiv8
    }
}
#[doc = "Field `PCLK_I2C0` writer - Peripheral clock selection for I2C0."]
pub type PclkI2c0W<'a, REG> = crate::FieldWriter<'a, REG, 2, PclkI2c0, crate::Safe>;
impl<'a, REG> PclkI2c0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(PclkI2c0::CclkDiv4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(PclkI2c0::Cclk)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(PclkI2c0::CclkDiv2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(PclkI2c0::CclkDiv8)
    }
}
#[doc = "Peripheral clock selection for SPI.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PclkSpi {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CclkDiv4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    Cclk = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CclkDiv2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CclkDiv8 = 3,
}
impl From<PclkSpi> for u8 {
    #[inline(always)]
    fn from(variant: PclkSpi) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PclkSpi {
    type Ux = u8;
}
impl crate::IsEnum for PclkSpi {}
#[doc = "Field `PCLK_SPI` reader - Peripheral clock selection for SPI."]
pub type PclkSpiR = crate::FieldReader<PclkSpi>;
impl PclkSpiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PclkSpi {
        match self.bits {
            0 => PclkSpi::CclkDiv4,
            1 => PclkSpi::Cclk,
            2 => PclkSpi::CclkDiv2,
            3 => PclkSpi::CclkDiv8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PclkSpi::CclkDiv4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PclkSpi::Cclk
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PclkSpi::CclkDiv2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PclkSpi::CclkDiv8
    }
}
#[doc = "Field `PCLK_SPI` writer - Peripheral clock selection for SPI."]
pub type PclkSpiW<'a, REG> = crate::FieldWriter<'a, REG, 2, PclkSpi, crate::Safe>;
impl<'a, REG> PclkSpiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(PclkSpi::CclkDiv4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(PclkSpi::Cclk)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(PclkSpi::CclkDiv2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(PclkSpi::CclkDiv8)
    }
}
#[doc = "Peripheral clock selection for SSP1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PclkSsp1 {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CclkDiv4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    Cclk = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CclkDiv2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CclkDiv8 = 3,
}
impl From<PclkSsp1> for u8 {
    #[inline(always)]
    fn from(variant: PclkSsp1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PclkSsp1 {
    type Ux = u8;
}
impl crate::IsEnum for PclkSsp1 {}
#[doc = "Field `PCLK_SSP1` reader - Peripheral clock selection for SSP1."]
pub type PclkSsp1R = crate::FieldReader<PclkSsp1>;
impl PclkSsp1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PclkSsp1 {
        match self.bits {
            0 => PclkSsp1::CclkDiv4,
            1 => PclkSsp1::Cclk,
            2 => PclkSsp1::CclkDiv2,
            3 => PclkSsp1::CclkDiv8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PclkSsp1::CclkDiv4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PclkSsp1::Cclk
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PclkSsp1::CclkDiv2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PclkSsp1::CclkDiv8
    }
}
#[doc = "Field `PCLK_SSP1` writer - Peripheral clock selection for SSP1."]
pub type PclkSsp1W<'a, REG> = crate::FieldWriter<'a, REG, 2, PclkSsp1, crate::Safe>;
impl<'a, REG> PclkSsp1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(PclkSsp1::CclkDiv4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(PclkSsp1::Cclk)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(PclkSsp1::CclkDiv2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(PclkSsp1::CclkDiv8)
    }
}
#[doc = "Peripheral clock selection for DAC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PclkDac {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CclkDiv4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    Cclk = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CclkDiv2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CclkDiv8 = 3,
}
impl From<PclkDac> for u8 {
    #[inline(always)]
    fn from(variant: PclkDac) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PclkDac {
    type Ux = u8;
}
impl crate::IsEnum for PclkDac {}
#[doc = "Field `PCLK_DAC` reader - Peripheral clock selection for DAC."]
pub type PclkDacR = crate::FieldReader<PclkDac>;
impl PclkDacR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PclkDac {
        match self.bits {
            0 => PclkDac::CclkDiv4,
            1 => PclkDac::Cclk,
            2 => PclkDac::CclkDiv2,
            3 => PclkDac::CclkDiv8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PclkDac::CclkDiv4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PclkDac::Cclk
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PclkDac::CclkDiv2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PclkDac::CclkDiv8
    }
}
#[doc = "Field `PCLK_DAC` writer - Peripheral clock selection for DAC."]
pub type PclkDacW<'a, REG> = crate::FieldWriter<'a, REG, 2, PclkDac, crate::Safe>;
impl<'a, REG> PclkDacW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(PclkDac::CclkDiv4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(PclkDac::Cclk)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(PclkDac::CclkDiv2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(PclkDac::CclkDiv8)
    }
}
#[doc = "Peripheral clock selection for ADC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PclkAdc {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CclkDiv4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    Cclk = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CclkDiv2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CclkDiv8 = 3,
}
impl From<PclkAdc> for u8 {
    #[inline(always)]
    fn from(variant: PclkAdc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PclkAdc {
    type Ux = u8;
}
impl crate::IsEnum for PclkAdc {}
#[doc = "Field `PCLK_ADC` reader - Peripheral clock selection for ADC."]
pub type PclkAdcR = crate::FieldReader<PclkAdc>;
impl PclkAdcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PclkAdc {
        match self.bits {
            0 => PclkAdc::CclkDiv4,
            1 => PclkAdc::Cclk,
            2 => PclkAdc::CclkDiv2,
            3 => PclkAdc::CclkDiv8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PclkAdc::CclkDiv4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PclkAdc::Cclk
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PclkAdc::CclkDiv2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PclkAdc::CclkDiv8
    }
}
#[doc = "Field `PCLK_ADC` writer - Peripheral clock selection for ADC."]
pub type PclkAdcW<'a, REG> = crate::FieldWriter<'a, REG, 2, PclkAdc, crate::Safe>;
impl<'a, REG> PclkAdcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(PclkAdc::CclkDiv4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(PclkAdc::Cclk)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(PclkAdc::CclkDiv2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(PclkAdc::CclkDiv8)
    }
}
#[doc = "Peripheral clock selection for CAN1.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PclkCan1 {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CclkDiv4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    Cclk = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CclkDiv2 = 2,
    #[doc = "3: CCLK div 6. PCLK_peripheral = CCLK/6."]
    CclkDiv6 = 3,
}
impl From<PclkCan1> for u8 {
    #[inline(always)]
    fn from(variant: PclkCan1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PclkCan1 {
    type Ux = u8;
}
impl crate::IsEnum for PclkCan1 {}
#[doc = "Field `PCLK_CAN1` reader - Peripheral clock selection for CAN1.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used."]
pub type PclkCan1R = crate::FieldReader<PclkCan1>;
impl PclkCan1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PclkCan1 {
        match self.bits {
            0 => PclkCan1::CclkDiv4,
            1 => PclkCan1::Cclk,
            2 => PclkCan1::CclkDiv2,
            3 => PclkCan1::CclkDiv6,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PclkCan1::CclkDiv4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PclkCan1::Cclk
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PclkCan1::CclkDiv2
    }
    #[doc = "CCLK div 6. PCLK_peripheral = CCLK/6."]
    #[inline(always)]
    pub fn is_cclk_div_6(&self) -> bool {
        *self == PclkCan1::CclkDiv6
    }
}
#[doc = "Field `PCLK_CAN1` writer - Peripheral clock selection for CAN1.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used."]
pub type PclkCan1W<'a, REG> = crate::FieldWriter<'a, REG, 2, PclkCan1, crate::Safe>;
impl<'a, REG> PclkCan1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(PclkCan1::CclkDiv4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(PclkCan1::Cclk)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(PclkCan1::CclkDiv2)
    }
    #[doc = "CCLK div 6. PCLK_peripheral = CCLK/6."]
    #[inline(always)]
    pub fn cclk_div_6(self) -> &'a mut crate::W<REG> {
        self.variant(PclkCan1::CclkDiv6)
    }
}
#[doc = "Peripheral clock selection for CAN2.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PclkCan2 {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CclkDiv4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    Cclk = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CclkDiv2 = 2,
    #[doc = "3: CCLK div 6. PCLK_peripheral = CCLK/6,"]
    CclkDiv6 = 3,
}
impl From<PclkCan2> for u8 {
    #[inline(always)]
    fn from(variant: PclkCan2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PclkCan2 {
    type Ux = u8;
}
impl crate::IsEnum for PclkCan2 {}
#[doc = "Field `PCLK_CAN2` reader - Peripheral clock selection for CAN2.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used."]
pub type PclkCan2R = crate::FieldReader<PclkCan2>;
impl PclkCan2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PclkCan2 {
        match self.bits {
            0 => PclkCan2::CclkDiv4,
            1 => PclkCan2::Cclk,
            2 => PclkCan2::CclkDiv2,
            3 => PclkCan2::CclkDiv6,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PclkCan2::CclkDiv4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PclkCan2::Cclk
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PclkCan2::CclkDiv2
    }
    #[doc = "CCLK div 6. PCLK_peripheral = CCLK/6,"]
    #[inline(always)]
    pub fn is_cclk_div_6(&self) -> bool {
        *self == PclkCan2::CclkDiv6
    }
}
#[doc = "Field `PCLK_CAN2` writer - Peripheral clock selection for CAN2.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used."]
pub type PclkCan2W<'a, REG> = crate::FieldWriter<'a, REG, 2, PclkCan2, crate::Safe>;
impl<'a, REG> PclkCan2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(PclkCan2::CclkDiv4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(PclkCan2::Cclk)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(PclkCan2::CclkDiv2)
    }
    #[doc = "CCLK div 6. PCLK_peripheral = CCLK/6,"]
    #[inline(always)]
    pub fn cclk_div_6(self) -> &'a mut crate::W<REG> {
        self.variant(PclkCan2::CclkDiv6)
    }
}
#[doc = "Peripheral clock selection for CAN acceptance filtering.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PclkAcf {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CclkDiv4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    Cclk = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CclkDiv2 = 2,
    #[doc = "3: CCLK div 6. PCLK_peripheral = CCLK/6"]
    CclkDiv6 = 3,
}
impl From<PclkAcf> for u8 {
    #[inline(always)]
    fn from(variant: PclkAcf) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PclkAcf {
    type Ux = u8;
}
impl crate::IsEnum for PclkAcf {}
#[doc = "Field `PCLK_ACF` reader - Peripheral clock selection for CAN acceptance filtering.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used."]
pub type PclkAcfR = crate::FieldReader<PclkAcf>;
impl PclkAcfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PclkAcf {
        match self.bits {
            0 => PclkAcf::CclkDiv4,
            1 => PclkAcf::Cclk,
            2 => PclkAcf::CclkDiv2,
            3 => PclkAcf::CclkDiv6,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PclkAcf::CclkDiv4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PclkAcf::Cclk
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PclkAcf::CclkDiv2
    }
    #[doc = "CCLK div 6. PCLK_peripheral = CCLK/6"]
    #[inline(always)]
    pub fn is_cclk_div_6(&self) -> bool {
        *self == PclkAcf::CclkDiv6
    }
}
#[doc = "Field `PCLK_ACF` writer - Peripheral clock selection for CAN acceptance filtering.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used."]
pub type PclkAcfW<'a, REG> = crate::FieldWriter<'a, REG, 2, PclkAcf, crate::Safe>;
impl<'a, REG> PclkAcfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(PclkAcf::CclkDiv4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(PclkAcf::Cclk)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(PclkAcf::CclkDiv2)
    }
    #[doc = "CCLK div 6. PCLK_peripheral = CCLK/6"]
    #[inline(always)]
    pub fn cclk_div_6(self) -> &'a mut crate::W<REG> {
        self.variant(PclkAcf::CclkDiv6)
    }
}
impl R {
    #[doc = "Bits 0:1 - Peripheral clock selection for WDT."]
    #[inline(always)]
    pub fn pclk_wdt(&self) -> PclkWdtR {
        PclkWdtR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Peripheral clock selection for TIMER0."]
    #[inline(always)]
    pub fn pclk_timer0(&self) -> PclkTimer0R {
        PclkTimer0R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Peripheral clock selection for TIMER1."]
    #[inline(always)]
    pub fn pclk_timer1(&self) -> PclkTimer1R {
        PclkTimer1R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Peripheral clock selection for UART0."]
    #[inline(always)]
    pub fn pclk_uart0(&self) -> PclkUart0R {
        PclkUart0R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Peripheral clock selection for UART1."]
    #[inline(always)]
    pub fn pclk_uart1(&self) -> PclkUart1R {
        PclkUart1R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Peripheral clock selection for PWM1."]
    #[inline(always)]
    pub fn pclk_pwm1(&self) -> PclkPwm1R {
        PclkPwm1R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Peripheral clock selection for I2C0."]
    #[inline(always)]
    pub fn pclk_i2c0(&self) -> PclkI2c0R {
        PclkI2c0R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Peripheral clock selection for SPI."]
    #[inline(always)]
    pub fn pclk_spi(&self) -> PclkSpiR {
        PclkSpiR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Peripheral clock selection for SSP1."]
    #[inline(always)]
    pub fn pclk_ssp1(&self) -> PclkSsp1R {
        PclkSsp1R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Peripheral clock selection for DAC."]
    #[inline(always)]
    pub fn pclk_dac(&self) -> PclkDacR {
        PclkDacR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Peripheral clock selection for ADC."]
    #[inline(always)]
    pub fn pclk_adc(&self) -> PclkAdcR {
        PclkAdcR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Peripheral clock selection for CAN1.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used."]
    #[inline(always)]
    pub fn pclk_can1(&self) -> PclkCan1R {
        PclkCan1R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Peripheral clock selection for CAN2.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used."]
    #[inline(always)]
    pub fn pclk_can2(&self) -> PclkCan2R {
        PclkCan2R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Peripheral clock selection for CAN acceptance filtering.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used."]
    #[inline(always)]
    pub fn pclk_acf(&self) -> PclkAcfR {
        PclkAcfR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Peripheral clock selection for WDT."]
    #[inline(always)]
    #[must_use]
    pub fn pclk_wdt(&mut self) -> PclkWdtW<Pclksel0Spec> {
        PclkWdtW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Peripheral clock selection for TIMER0."]
    #[inline(always)]
    #[must_use]
    pub fn pclk_timer0(&mut self) -> PclkTimer0W<Pclksel0Spec> {
        PclkTimer0W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Peripheral clock selection for TIMER1."]
    #[inline(always)]
    #[must_use]
    pub fn pclk_timer1(&mut self) -> PclkTimer1W<Pclksel0Spec> {
        PclkTimer1W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Peripheral clock selection for UART0."]
    #[inline(always)]
    #[must_use]
    pub fn pclk_uart0(&mut self) -> PclkUart0W<Pclksel0Spec> {
        PclkUart0W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Peripheral clock selection for UART1."]
    #[inline(always)]
    #[must_use]
    pub fn pclk_uart1(&mut self) -> PclkUart1W<Pclksel0Spec> {
        PclkUart1W::new(self, 8)
    }
    #[doc = "Bits 12:13 - Peripheral clock selection for PWM1."]
    #[inline(always)]
    #[must_use]
    pub fn pclk_pwm1(&mut self) -> PclkPwm1W<Pclksel0Spec> {
        PclkPwm1W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Peripheral clock selection for I2C0."]
    #[inline(always)]
    #[must_use]
    pub fn pclk_i2c0(&mut self) -> PclkI2c0W<Pclksel0Spec> {
        PclkI2c0W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Peripheral clock selection for SPI."]
    #[inline(always)]
    #[must_use]
    pub fn pclk_spi(&mut self) -> PclkSpiW<Pclksel0Spec> {
        PclkSpiW::new(self, 16)
    }
    #[doc = "Bits 20:21 - Peripheral clock selection for SSP1."]
    #[inline(always)]
    #[must_use]
    pub fn pclk_ssp1(&mut self) -> PclkSsp1W<Pclksel0Spec> {
        PclkSsp1W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Peripheral clock selection for DAC."]
    #[inline(always)]
    #[must_use]
    pub fn pclk_dac(&mut self) -> PclkDacW<Pclksel0Spec> {
        PclkDacW::new(self, 22)
    }
    #[doc = "Bits 24:25 - Peripheral clock selection for ADC."]
    #[inline(always)]
    #[must_use]
    pub fn pclk_adc(&mut self) -> PclkAdcW<Pclksel0Spec> {
        PclkAdcW::new(self, 24)
    }
    #[doc = "Bits 26:27 - Peripheral clock selection for CAN1.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used."]
    #[inline(always)]
    #[must_use]
    pub fn pclk_can1(&mut self) -> PclkCan1W<Pclksel0Spec> {
        PclkCan1W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Peripheral clock selection for CAN2.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used."]
    #[inline(always)]
    #[must_use]
    pub fn pclk_can2(&mut self) -> PclkCan2W<Pclksel0Spec> {
        PclkCan2W::new(self, 28)
    }
    #[doc = "Bits 30:31 - Peripheral clock selection for CAN acceptance filtering.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used."]
    #[inline(always)]
    #[must_use]
    pub fn pclk_acf(&mut self) -> PclkAcfW<Pclksel0Spec> {
        PclkAcfW::new(self, 30)
    }
}
#[doc = "Peripheral Clock Selection register 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`pclksel0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pclksel0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pclksel0Spec;
impl crate::RegisterSpec for Pclksel0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pclksel0::R`](R) reader structure"]
impl crate::Readable for Pclksel0Spec {}
#[doc = "`write(|w| ..)` method takes [`pclksel0::W`](W) writer structure"]
impl crate::Writable for Pclksel0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCLKSEL0 to value 0"]
impl crate::Resettable for Pclksel0Spec {
    const RESET_VALUE: u32 = 0;
}
