#[doc = "Register `PCLKSEL1` reader"]
pub type R = crate::R<Pclksel1Spec>;
#[doc = "Register `PCLKSEL1` writer"]
pub type W = crate::W<Pclksel1Spec>;
#[doc = "Peripheral clock selection for the Quadrature Encoder Interface.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PclkQei {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CclkDiv4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    Cclk = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CclkDiv2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CclkDiv8 = 3,
}
impl From<PclkQei> for u8 {
    #[inline(always)]
    fn from(variant: PclkQei) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PclkQei {
    type Ux = u8;
}
impl crate::IsEnum for PclkQei {}
#[doc = "Field `PCLK_QEI` reader - Peripheral clock selection for the Quadrature Encoder Interface."]
pub type PclkQeiR = crate::FieldReader<PclkQei>;
impl PclkQeiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PclkQei {
        match self.bits {
            0 => PclkQei::CclkDiv4,
            1 => PclkQei::Cclk,
            2 => PclkQei::CclkDiv2,
            3 => PclkQei::CclkDiv8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PclkQei::CclkDiv4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PclkQei::Cclk
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PclkQei::CclkDiv2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PclkQei::CclkDiv8
    }
}
#[doc = "Field `PCLK_QEI` writer - Peripheral clock selection for the Quadrature Encoder Interface."]
pub type PclkQeiW<'a, REG> = crate::FieldWriter<'a, REG, 2, PclkQei, crate::Safe>;
impl<'a, REG> PclkQeiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(PclkQei::CclkDiv4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(PclkQei::Cclk)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(PclkQei::CclkDiv2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(PclkQei::CclkDiv8)
    }
}
#[doc = "Peripheral clock selection for GPIO interrupts.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PclkGpioint {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CclkDiv4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    Cclk = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CclkDiv2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CclkDiv8 = 3,
}
impl From<PclkGpioint> for u8 {
    #[inline(always)]
    fn from(variant: PclkGpioint) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PclkGpioint {
    type Ux = u8;
}
impl crate::IsEnum for PclkGpioint {}
#[doc = "Field `PCLK_GPIOINT` reader - Peripheral clock selection for GPIO interrupts."]
pub type PclkGpiointR = crate::FieldReader<PclkGpioint>;
impl PclkGpiointR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PclkGpioint {
        match self.bits {
            0 => PclkGpioint::CclkDiv4,
            1 => PclkGpioint::Cclk,
            2 => PclkGpioint::CclkDiv2,
            3 => PclkGpioint::CclkDiv8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PclkGpioint::CclkDiv4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PclkGpioint::Cclk
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PclkGpioint::CclkDiv2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PclkGpioint::CclkDiv8
    }
}
#[doc = "Field `PCLK_GPIOINT` writer - Peripheral clock selection for GPIO interrupts."]
pub type PclkGpiointW<'a, REG> = crate::FieldWriter<'a, REG, 2, PclkGpioint, crate::Safe>;
impl<'a, REG> PclkGpiointW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(PclkGpioint::CclkDiv4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(PclkGpioint::Cclk)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(PclkGpioint::CclkDiv2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(PclkGpioint::CclkDiv8)
    }
}
#[doc = "Peripheral clock selection for the Pin Connect block.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PclkPcb {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CclkDiv4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    Cclk = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CclkDiv2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CclkDiv8 = 3,
}
impl From<PclkPcb> for u8 {
    #[inline(always)]
    fn from(variant: PclkPcb) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PclkPcb {
    type Ux = u8;
}
impl crate::IsEnum for PclkPcb {}
#[doc = "Field `PCLK_PCB` reader - Peripheral clock selection for the Pin Connect block."]
pub type PclkPcbR = crate::FieldReader<PclkPcb>;
impl PclkPcbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PclkPcb {
        match self.bits {
            0 => PclkPcb::CclkDiv4,
            1 => PclkPcb::Cclk,
            2 => PclkPcb::CclkDiv2,
            3 => PclkPcb::CclkDiv8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PclkPcb::CclkDiv4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PclkPcb::Cclk
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PclkPcb::CclkDiv2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PclkPcb::CclkDiv8
    }
}
#[doc = "Field `PCLK_PCB` writer - Peripheral clock selection for the Pin Connect block."]
pub type PclkPcbW<'a, REG> = crate::FieldWriter<'a, REG, 2, PclkPcb, crate::Safe>;
impl<'a, REG> PclkPcbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(PclkPcb::CclkDiv4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(PclkPcb::Cclk)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(PclkPcb::CclkDiv2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(PclkPcb::CclkDiv8)
    }
}
#[doc = "Peripheral clock selection for I2C1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PclkI2c1 {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CclkDiv4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    Cclk = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CclkDiv2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CclkDiv8 = 3,
}
impl From<PclkI2c1> for u8 {
    #[inline(always)]
    fn from(variant: PclkI2c1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PclkI2c1 {
    type Ux = u8;
}
impl crate::IsEnum for PclkI2c1 {}
#[doc = "Field `PCLK_I2C1` reader - Peripheral clock selection for I2C1."]
pub type PclkI2c1R = crate::FieldReader<PclkI2c1>;
impl PclkI2c1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PclkI2c1 {
        match self.bits {
            0 => PclkI2c1::CclkDiv4,
            1 => PclkI2c1::Cclk,
            2 => PclkI2c1::CclkDiv2,
            3 => PclkI2c1::CclkDiv8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PclkI2c1::CclkDiv4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PclkI2c1::Cclk
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PclkI2c1::CclkDiv2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PclkI2c1::CclkDiv8
    }
}
#[doc = "Field `PCLK_I2C1` writer - Peripheral clock selection for I2C1."]
pub type PclkI2c1W<'a, REG> = crate::FieldWriter<'a, REG, 2, PclkI2c1, crate::Safe>;
impl<'a, REG> PclkI2c1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(PclkI2c1::CclkDiv4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(PclkI2c1::Cclk)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(PclkI2c1::CclkDiv2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(PclkI2c1::CclkDiv8)
    }
}
#[doc = "Peripheral clock selection for SSP0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PclkSsp0 {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CclkDiv4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    Cclk = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CclkDiv2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CclkDiv8 = 3,
}
impl From<PclkSsp0> for u8 {
    #[inline(always)]
    fn from(variant: PclkSsp0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PclkSsp0 {
    type Ux = u8;
}
impl crate::IsEnum for PclkSsp0 {}
#[doc = "Field `PCLK_SSP0` reader - Peripheral clock selection for SSP0."]
pub type PclkSsp0R = crate::FieldReader<PclkSsp0>;
impl PclkSsp0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PclkSsp0 {
        match self.bits {
            0 => PclkSsp0::CclkDiv4,
            1 => PclkSsp0::Cclk,
            2 => PclkSsp0::CclkDiv2,
            3 => PclkSsp0::CclkDiv8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PclkSsp0::CclkDiv4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PclkSsp0::Cclk
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PclkSsp0::CclkDiv2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PclkSsp0::CclkDiv8
    }
}
#[doc = "Field `PCLK_SSP0` writer - Peripheral clock selection for SSP0."]
pub type PclkSsp0W<'a, REG> = crate::FieldWriter<'a, REG, 2, PclkSsp0, crate::Safe>;
impl<'a, REG> PclkSsp0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(PclkSsp0::CclkDiv4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(PclkSsp0::Cclk)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(PclkSsp0::CclkDiv2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(PclkSsp0::CclkDiv8)
    }
}
#[doc = "Peripheral clock selection for TIMER2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PclkTimer2 {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CclkDiv4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    Cclk = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CclkDiv2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CclkDiv8 = 3,
}
impl From<PclkTimer2> for u8 {
    #[inline(always)]
    fn from(variant: PclkTimer2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PclkTimer2 {
    type Ux = u8;
}
impl crate::IsEnum for PclkTimer2 {}
#[doc = "Field `PCLK_TIMER2` reader - Peripheral clock selection for TIMER2."]
pub type PclkTimer2R = crate::FieldReader<PclkTimer2>;
impl PclkTimer2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PclkTimer2 {
        match self.bits {
            0 => PclkTimer2::CclkDiv4,
            1 => PclkTimer2::Cclk,
            2 => PclkTimer2::CclkDiv2,
            3 => PclkTimer2::CclkDiv8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PclkTimer2::CclkDiv4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PclkTimer2::Cclk
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PclkTimer2::CclkDiv2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PclkTimer2::CclkDiv8
    }
}
#[doc = "Field `PCLK_TIMER2` writer - Peripheral clock selection for TIMER2."]
pub type PclkTimer2W<'a, REG> = crate::FieldWriter<'a, REG, 2, PclkTimer2, crate::Safe>;
impl<'a, REG> PclkTimer2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(PclkTimer2::CclkDiv4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(PclkTimer2::Cclk)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(PclkTimer2::CclkDiv2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(PclkTimer2::CclkDiv8)
    }
}
#[doc = "Peripheral clock selection for TIMER3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PclkTimer3 {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CclkDiv4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    Cclk = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CclkDiv2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CclkDiv8 = 3,
}
impl From<PclkTimer3> for u8 {
    #[inline(always)]
    fn from(variant: PclkTimer3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PclkTimer3 {
    type Ux = u8;
}
impl crate::IsEnum for PclkTimer3 {}
#[doc = "Field `PCLK_TIMER3` reader - Peripheral clock selection for TIMER3."]
pub type PclkTimer3R = crate::FieldReader<PclkTimer3>;
impl PclkTimer3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PclkTimer3 {
        match self.bits {
            0 => PclkTimer3::CclkDiv4,
            1 => PclkTimer3::Cclk,
            2 => PclkTimer3::CclkDiv2,
            3 => PclkTimer3::CclkDiv8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PclkTimer3::CclkDiv4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PclkTimer3::Cclk
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PclkTimer3::CclkDiv2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PclkTimer3::CclkDiv8
    }
}
#[doc = "Field `PCLK_TIMER3` writer - Peripheral clock selection for TIMER3."]
pub type PclkTimer3W<'a, REG> = crate::FieldWriter<'a, REG, 2, PclkTimer3, crate::Safe>;
impl<'a, REG> PclkTimer3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(PclkTimer3::CclkDiv4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(PclkTimer3::Cclk)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(PclkTimer3::CclkDiv2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(PclkTimer3::CclkDiv8)
    }
}
#[doc = "Peripheral clock selection for UART2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PclkUart2 {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CclkDiv4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    Cclk = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CclkDiv2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CclkDiv8 = 3,
}
impl From<PclkUart2> for u8 {
    #[inline(always)]
    fn from(variant: PclkUart2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PclkUart2 {
    type Ux = u8;
}
impl crate::IsEnum for PclkUart2 {}
#[doc = "Field `PCLK_UART2` reader - Peripheral clock selection for UART2."]
pub type PclkUart2R = crate::FieldReader<PclkUart2>;
impl PclkUart2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PclkUart2 {
        match self.bits {
            0 => PclkUart2::CclkDiv4,
            1 => PclkUart2::Cclk,
            2 => PclkUart2::CclkDiv2,
            3 => PclkUart2::CclkDiv8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PclkUart2::CclkDiv4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PclkUart2::Cclk
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PclkUart2::CclkDiv2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PclkUart2::CclkDiv8
    }
}
#[doc = "Field `PCLK_UART2` writer - Peripheral clock selection for UART2."]
pub type PclkUart2W<'a, REG> = crate::FieldWriter<'a, REG, 2, PclkUart2, crate::Safe>;
impl<'a, REG> PclkUart2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(PclkUart2::CclkDiv4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(PclkUart2::Cclk)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(PclkUart2::CclkDiv2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(PclkUart2::CclkDiv8)
    }
}
#[doc = "Peripheral clock selection for UART3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PclkUart3 {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CclkDiv4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    Cclk = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CclkDiv2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CclkDiv8 = 3,
}
impl From<PclkUart3> for u8 {
    #[inline(always)]
    fn from(variant: PclkUart3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PclkUart3 {
    type Ux = u8;
}
impl crate::IsEnum for PclkUart3 {}
#[doc = "Field `PCLK_UART3` reader - Peripheral clock selection for UART3."]
pub type PclkUart3R = crate::FieldReader<PclkUart3>;
impl PclkUart3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PclkUart3 {
        match self.bits {
            0 => PclkUart3::CclkDiv4,
            1 => PclkUart3::Cclk,
            2 => PclkUart3::CclkDiv2,
            3 => PclkUart3::CclkDiv8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PclkUart3::CclkDiv4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PclkUart3::Cclk
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PclkUart3::CclkDiv2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PclkUart3::CclkDiv8
    }
}
#[doc = "Field `PCLK_UART3` writer - Peripheral clock selection for UART3."]
pub type PclkUart3W<'a, REG> = crate::FieldWriter<'a, REG, 2, PclkUart3, crate::Safe>;
impl<'a, REG> PclkUart3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(PclkUart3::CclkDiv4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(PclkUart3::Cclk)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(PclkUart3::CclkDiv2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(PclkUart3::CclkDiv8)
    }
}
#[doc = "Peripheral clock selection for I2C2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PclkI2c2 {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CclkDiv4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    Cclk = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CclkDiv2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CclkDiv8 = 3,
}
impl From<PclkI2c2> for u8 {
    #[inline(always)]
    fn from(variant: PclkI2c2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PclkI2c2 {
    type Ux = u8;
}
impl crate::IsEnum for PclkI2c2 {}
#[doc = "Field `PCLK_I2C2` reader - Peripheral clock selection for I2C2."]
pub type PclkI2c2R = crate::FieldReader<PclkI2c2>;
impl PclkI2c2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PclkI2c2 {
        match self.bits {
            0 => PclkI2c2::CclkDiv4,
            1 => PclkI2c2::Cclk,
            2 => PclkI2c2::CclkDiv2,
            3 => PclkI2c2::CclkDiv8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PclkI2c2::CclkDiv4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PclkI2c2::Cclk
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PclkI2c2::CclkDiv2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PclkI2c2::CclkDiv8
    }
}
#[doc = "Field `PCLK_I2C2` writer - Peripheral clock selection for I2C2."]
pub type PclkI2c2W<'a, REG> = crate::FieldWriter<'a, REG, 2, PclkI2c2, crate::Safe>;
impl<'a, REG> PclkI2c2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(PclkI2c2::CclkDiv4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(PclkI2c2::Cclk)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(PclkI2c2::CclkDiv2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(PclkI2c2::CclkDiv8)
    }
}
#[doc = "Peripheral clock selection for I2S.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PclkI2s {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CclkDiv4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    Cclk = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CclkDiv2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CclkDiv8 = 3,
}
impl From<PclkI2s> for u8 {
    #[inline(always)]
    fn from(variant: PclkI2s) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PclkI2s {
    type Ux = u8;
}
impl crate::IsEnum for PclkI2s {}
#[doc = "Field `PCLK_I2S` reader - Peripheral clock selection for I2S."]
pub type PclkI2sR = crate::FieldReader<PclkI2s>;
impl PclkI2sR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PclkI2s {
        match self.bits {
            0 => PclkI2s::CclkDiv4,
            1 => PclkI2s::Cclk,
            2 => PclkI2s::CclkDiv2,
            3 => PclkI2s::CclkDiv8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PclkI2s::CclkDiv4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PclkI2s::Cclk
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PclkI2s::CclkDiv2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PclkI2s::CclkDiv8
    }
}
#[doc = "Field `PCLK_I2S` writer - Peripheral clock selection for I2S."]
pub type PclkI2sW<'a, REG> = crate::FieldWriter<'a, REG, 2, PclkI2s, crate::Safe>;
impl<'a, REG> PclkI2sW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(PclkI2s::CclkDiv4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(PclkI2s::Cclk)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(PclkI2s::CclkDiv2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(PclkI2s::CclkDiv8)
    }
}
#[doc = "Peripheral clock selection for Repetitive Interrupt Timer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PclkRit {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CclkDiv4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    Cclk = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CclkDiv2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CclkDiv8 = 3,
}
impl From<PclkRit> for u8 {
    #[inline(always)]
    fn from(variant: PclkRit) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PclkRit {
    type Ux = u8;
}
impl crate::IsEnum for PclkRit {}
#[doc = "Field `PCLK_RIT` reader - Peripheral clock selection for Repetitive Interrupt Timer."]
pub type PclkRitR = crate::FieldReader<PclkRit>;
impl PclkRitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PclkRit {
        match self.bits {
            0 => PclkRit::CclkDiv4,
            1 => PclkRit::Cclk,
            2 => PclkRit::CclkDiv2,
            3 => PclkRit::CclkDiv8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PclkRit::CclkDiv4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PclkRit::Cclk
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PclkRit::CclkDiv2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PclkRit::CclkDiv8
    }
}
#[doc = "Field `PCLK_RIT` writer - Peripheral clock selection for Repetitive Interrupt Timer."]
pub type PclkRitW<'a, REG> = crate::FieldWriter<'a, REG, 2, PclkRit, crate::Safe>;
impl<'a, REG> PclkRitW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(PclkRit::CclkDiv4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(PclkRit::Cclk)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(PclkRit::CclkDiv2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(PclkRit::CclkDiv8)
    }
}
#[doc = "Peripheral clock selection for the System Control block.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PclkSyscon {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CclkDiv4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    Cclk = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CclkDiv2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CclkDiv8 = 3,
}
impl From<PclkSyscon> for u8 {
    #[inline(always)]
    fn from(variant: PclkSyscon) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PclkSyscon {
    type Ux = u8;
}
impl crate::IsEnum for PclkSyscon {}
#[doc = "Field `PCLK_SYSCON` reader - Peripheral clock selection for the System Control block."]
pub type PclkSysconR = crate::FieldReader<PclkSyscon>;
impl PclkSysconR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PclkSyscon {
        match self.bits {
            0 => PclkSyscon::CclkDiv4,
            1 => PclkSyscon::Cclk,
            2 => PclkSyscon::CclkDiv2,
            3 => PclkSyscon::CclkDiv8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PclkSyscon::CclkDiv4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PclkSyscon::Cclk
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PclkSyscon::CclkDiv2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PclkSyscon::CclkDiv8
    }
}
#[doc = "Field `PCLK_SYSCON` writer - Peripheral clock selection for the System Control block."]
pub type PclkSysconW<'a, REG> = crate::FieldWriter<'a, REG, 2, PclkSyscon, crate::Safe>;
impl<'a, REG> PclkSysconW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(PclkSyscon::CclkDiv4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(PclkSyscon::Cclk)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(PclkSyscon::CclkDiv2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(PclkSyscon::CclkDiv8)
    }
}
#[doc = "Peripheral clock selection for the Motor Control PWM.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PclkMc {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CclkDiv4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    Cclk = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CclkDiv2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CclkDiv8 = 3,
}
impl From<PclkMc> for u8 {
    #[inline(always)]
    fn from(variant: PclkMc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PclkMc {
    type Ux = u8;
}
impl crate::IsEnum for PclkMc {}
#[doc = "Field `PCLK_MC` reader - Peripheral clock selection for the Motor Control PWM."]
pub type PclkMcR = crate::FieldReader<PclkMc>;
impl PclkMcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PclkMc {
        match self.bits {
            0 => PclkMc::CclkDiv4,
            1 => PclkMc::Cclk,
            2 => PclkMc::CclkDiv2,
            3 => PclkMc::CclkDiv8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PclkMc::CclkDiv4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PclkMc::Cclk
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PclkMc::CclkDiv2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PclkMc::CclkDiv8
    }
}
#[doc = "Field `PCLK_MC` writer - Peripheral clock selection for the Motor Control PWM."]
pub type PclkMcW<'a, REG> = crate::FieldWriter<'a, REG, 2, PclkMc, crate::Safe>;
impl<'a, REG> PclkMcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(PclkMc::CclkDiv4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(PclkMc::Cclk)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(PclkMc::CclkDiv2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(PclkMc::CclkDiv8)
    }
}
impl R {
    #[doc = "Bits 0:1 - Peripheral clock selection for the Quadrature Encoder Interface."]
    #[inline(always)]
    pub fn pclk_qei(&self) -> PclkQeiR {
        PclkQeiR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Peripheral clock selection for GPIO interrupts."]
    #[inline(always)]
    pub fn pclk_gpioint(&self) -> PclkGpiointR {
        PclkGpiointR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Peripheral clock selection for the Pin Connect block."]
    #[inline(always)]
    pub fn pclk_pcb(&self) -> PclkPcbR {
        PclkPcbR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Peripheral clock selection for I2C1."]
    #[inline(always)]
    pub fn pclk_i2c1(&self) -> PclkI2c1R {
        PclkI2c1R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Peripheral clock selection for SSP0."]
    #[inline(always)]
    pub fn pclk_ssp0(&self) -> PclkSsp0R {
        PclkSsp0R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Peripheral clock selection for TIMER2."]
    #[inline(always)]
    pub fn pclk_timer2(&self) -> PclkTimer2R {
        PclkTimer2R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Peripheral clock selection for TIMER3."]
    #[inline(always)]
    pub fn pclk_timer3(&self) -> PclkTimer3R {
        PclkTimer3R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Peripheral clock selection for UART2."]
    #[inline(always)]
    pub fn pclk_uart2(&self) -> PclkUart2R {
        PclkUart2R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Peripheral clock selection for UART3."]
    #[inline(always)]
    pub fn pclk_uart3(&self) -> PclkUart3R {
        PclkUart3R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Peripheral clock selection for I2C2."]
    #[inline(always)]
    pub fn pclk_i2c2(&self) -> PclkI2c2R {
        PclkI2c2R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Peripheral clock selection for I2S."]
    #[inline(always)]
    pub fn pclk_i2s(&self) -> PclkI2sR {
        PclkI2sR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Peripheral clock selection for Repetitive Interrupt Timer."]
    #[inline(always)]
    pub fn pclk_rit(&self) -> PclkRitR {
        PclkRitR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Peripheral clock selection for the System Control block."]
    #[inline(always)]
    pub fn pclk_syscon(&self) -> PclkSysconR {
        PclkSysconR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Peripheral clock selection for the Motor Control PWM."]
    #[inline(always)]
    pub fn pclk_mc(&self) -> PclkMcR {
        PclkMcR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Peripheral clock selection for the Quadrature Encoder Interface."]
    #[inline(always)]
    #[must_use]
    pub fn pclk_qei(&mut self) -> PclkQeiW<Pclksel1Spec> {
        PclkQeiW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Peripheral clock selection for GPIO interrupts."]
    #[inline(always)]
    #[must_use]
    pub fn pclk_gpioint(&mut self) -> PclkGpiointW<Pclksel1Spec> {
        PclkGpiointW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Peripheral clock selection for the Pin Connect block."]
    #[inline(always)]
    #[must_use]
    pub fn pclk_pcb(&mut self) -> PclkPcbW<Pclksel1Spec> {
        PclkPcbW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Peripheral clock selection for I2C1."]
    #[inline(always)]
    #[must_use]
    pub fn pclk_i2c1(&mut self) -> PclkI2c1W<Pclksel1Spec> {
        PclkI2c1W::new(self, 6)
    }
    #[doc = "Bits 10:11 - Peripheral clock selection for SSP0."]
    #[inline(always)]
    #[must_use]
    pub fn pclk_ssp0(&mut self) -> PclkSsp0W<Pclksel1Spec> {
        PclkSsp0W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Peripheral clock selection for TIMER2."]
    #[inline(always)]
    #[must_use]
    pub fn pclk_timer2(&mut self) -> PclkTimer2W<Pclksel1Spec> {
        PclkTimer2W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Peripheral clock selection for TIMER3."]
    #[inline(always)]
    #[must_use]
    pub fn pclk_timer3(&mut self) -> PclkTimer3W<Pclksel1Spec> {
        PclkTimer3W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Peripheral clock selection for UART2."]
    #[inline(always)]
    #[must_use]
    pub fn pclk_uart2(&mut self) -> PclkUart2W<Pclksel1Spec> {
        PclkUart2W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Peripheral clock selection for UART3."]
    #[inline(always)]
    #[must_use]
    pub fn pclk_uart3(&mut self) -> PclkUart3W<Pclksel1Spec> {
        PclkUart3W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Peripheral clock selection for I2C2."]
    #[inline(always)]
    #[must_use]
    pub fn pclk_i2c2(&mut self) -> PclkI2c2W<Pclksel1Spec> {
        PclkI2c2W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Peripheral clock selection for I2S."]
    #[inline(always)]
    #[must_use]
    pub fn pclk_i2s(&mut self) -> PclkI2sW<Pclksel1Spec> {
        PclkI2sW::new(self, 22)
    }
    #[doc = "Bits 26:27 - Peripheral clock selection for Repetitive Interrupt Timer."]
    #[inline(always)]
    #[must_use]
    pub fn pclk_rit(&mut self) -> PclkRitW<Pclksel1Spec> {
        PclkRitW::new(self, 26)
    }
    #[doc = "Bits 28:29 - Peripheral clock selection for the System Control block."]
    #[inline(always)]
    #[must_use]
    pub fn pclk_syscon(&mut self) -> PclkSysconW<Pclksel1Spec> {
        PclkSysconW::new(self, 28)
    }
    #[doc = "Bits 30:31 - Peripheral clock selection for the Motor Control PWM."]
    #[inline(always)]
    #[must_use]
    pub fn pclk_mc(&mut self) -> PclkMcW<Pclksel1Spec> {
        PclkMcW::new(self, 30)
    }
}
#[doc = "Peripheral Clock Selection register 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`pclksel1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pclksel1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pclksel1Spec;
impl crate::RegisterSpec for Pclksel1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pclksel1::R`](R) reader structure"]
impl crate::Readable for Pclksel1Spec {}
#[doc = "`write(|w| ..)` method takes [`pclksel1::W`](W) writer structure"]
impl crate::Writable for Pclksel1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCLKSEL1 to value 0"]
impl crate::Resettable for Pclksel1Spec {
    const RESET_VALUE: u32 = 0;
}
