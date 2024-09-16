#[doc = "Register `PINSEL0` reader"]
pub type R = crate::R<Pinsel0Spec>;
#[doc = "Register `PINSEL0` writer"]
pub type W = crate::W<Pinsel0Spec>;
#[doc = "Pin function select P0.0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P0_0 {
    #[doc = "0: GPIO P0.0"]
    GpioP0 = 0,
    #[doc = "1: RD1"]
    Rd1 = 1,
    #[doc = "2: TXD3"]
    Txd3 = 2,
    #[doc = "3: SDA1"]
    Sda1 = 3,
}
impl From<P0_0> for u8 {
    #[inline(always)]
    fn from(variant: P0_0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P0_0 {
    type Ux = u8;
}
impl crate::IsEnum for P0_0 {}
#[doc = "Field `P0_0` reader - Pin function select P0.0."]
pub type P0_0R = crate::FieldReader<P0_0>;
impl P0_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_0 {
        match self.bits {
            0 => P0_0::GpioP0,
            1 => P0_0::Rd1,
            2 => P0_0::Txd3,
            3 => P0_0::Sda1,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P0.0"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_0::GpioP0
    }
    #[doc = "RD1"]
    #[inline(always)]
    pub fn is_rd1(&self) -> bool {
        *self == P0_0::Rd1
    }
    #[doc = "TXD3"]
    #[inline(always)]
    pub fn is_txd3(&self) -> bool {
        *self == P0_0::Txd3
    }
    #[doc = "SDA1"]
    #[inline(always)]
    pub fn is_sda1(&self) -> bool {
        *self == P0_0::Sda1
    }
}
#[doc = "Field `P0_0` writer - Pin function select P0.0."]
pub type P0_0W<'a, REG> = crate::FieldWriter<'a, REG, 2, P0_0, crate::Safe>;
impl<'a, REG> P0_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P0.0"]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut crate::W<REG> {
        self.variant(P0_0::GpioP0)
    }
    #[doc = "RD1"]
    #[inline(always)]
    pub fn rd1(self) -> &'a mut crate::W<REG> {
        self.variant(P0_0::Rd1)
    }
    #[doc = "TXD3"]
    #[inline(always)]
    pub fn txd3(self) -> &'a mut crate::W<REG> {
        self.variant(P0_0::Txd3)
    }
    #[doc = "SDA1"]
    #[inline(always)]
    pub fn sda1(self) -> &'a mut crate::W<REG> {
        self.variant(P0_0::Sda1)
    }
}
#[doc = "Pin function select P0.1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P0_1 {
    #[doc = "0: GPIO P0.1"]
    GpioP0 = 0,
    #[doc = "1: TD1"]
    Td1 = 1,
    #[doc = "2: RXD3"]
    Rxd3 = 2,
    #[doc = "3: SCL1"]
    Scl1 = 3,
}
impl From<P0_1> for u8 {
    #[inline(always)]
    fn from(variant: P0_1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P0_1 {
    type Ux = u8;
}
impl crate::IsEnum for P0_1 {}
#[doc = "Field `P0_1` reader - Pin function select P0.1."]
pub type P0_1R = crate::FieldReader<P0_1>;
impl P0_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_1 {
        match self.bits {
            0 => P0_1::GpioP0,
            1 => P0_1::Td1,
            2 => P0_1::Rxd3,
            3 => P0_1::Scl1,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P0.1"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_1::GpioP0
    }
    #[doc = "TD1"]
    #[inline(always)]
    pub fn is_td1(&self) -> bool {
        *self == P0_1::Td1
    }
    #[doc = "RXD3"]
    #[inline(always)]
    pub fn is_rxd3(&self) -> bool {
        *self == P0_1::Rxd3
    }
    #[doc = "SCL1"]
    #[inline(always)]
    pub fn is_scl1(&self) -> bool {
        *self == P0_1::Scl1
    }
}
#[doc = "Field `P0_1` writer - Pin function select P0.1."]
pub type P0_1W<'a, REG> = crate::FieldWriter<'a, REG, 2, P0_1, crate::Safe>;
impl<'a, REG> P0_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P0.1"]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut crate::W<REG> {
        self.variant(P0_1::GpioP0)
    }
    #[doc = "TD1"]
    #[inline(always)]
    pub fn td1(self) -> &'a mut crate::W<REG> {
        self.variant(P0_1::Td1)
    }
    #[doc = "RXD3"]
    #[inline(always)]
    pub fn rxd3(self) -> &'a mut crate::W<REG> {
        self.variant(P0_1::Rxd3)
    }
    #[doc = "SCL1"]
    #[inline(always)]
    pub fn scl1(self) -> &'a mut crate::W<REG> {
        self.variant(P0_1::Scl1)
    }
}
#[doc = "Pin function select P0.2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P0_2 {
    #[doc = "0: GPIO P0.2"]
    GpioP0 = 0,
    #[doc = "1: TXD0"]
    Txd0 = 1,
    #[doc = "2: AD0.7"]
    Ad0 = 2,
}
impl From<P0_2> for u8 {
    #[inline(always)]
    fn from(variant: P0_2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P0_2 {
    type Ux = u8;
}
impl crate::IsEnum for P0_2 {}
#[doc = "Field `P0_2` reader - Pin function select P0.2."]
pub type P0_2R = crate::FieldReader<P0_2>;
impl P0_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_2 {
        match self.bits {
            0 => P0_2::GpioP0,
            1 => P0_2::Txd0,
            2 => P0_2::Ad0,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P0.2"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_2::GpioP0
    }
    #[doc = "TXD0"]
    #[inline(always)]
    pub fn is_txd0(&self) -> bool {
        *self == P0_2::Txd0
    }
    #[doc = "AD0.7"]
    #[inline(always)]
    pub fn is_ad0(&self) -> bool {
        *self == P0_2::Ad0
    }
}
#[doc = "Field `P0_2` writer - Pin function select P0.2."]
pub type P0_2W<'a, REG> = crate::FieldWriter<'a, REG, 2, P0_2>;
impl<'a, REG> P0_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P0.2"]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut crate::W<REG> {
        self.variant(P0_2::GpioP0)
    }
    #[doc = "TXD0"]
    #[inline(always)]
    pub fn txd0(self) -> &'a mut crate::W<REG> {
        self.variant(P0_2::Txd0)
    }
    #[doc = "AD0.7"]
    #[inline(always)]
    pub fn ad0(self) -> &'a mut crate::W<REG> {
        self.variant(P0_2::Ad0)
    }
}
#[doc = "Pin function select P0.3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P0_3 {
    #[doc = "0: GPIO P0.3."]
    GpioP0 = 0,
    #[doc = "1: RXD0"]
    Rxd0 = 1,
    #[doc = "2: AD0.6"]
    Ad0 = 2,
}
impl From<P0_3> for u8 {
    #[inline(always)]
    fn from(variant: P0_3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P0_3 {
    type Ux = u8;
}
impl crate::IsEnum for P0_3 {}
#[doc = "Field `P0_3` reader - Pin function select P0.3."]
pub type P0_3R = crate::FieldReader<P0_3>;
impl P0_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_3 {
        match self.bits {
            0 => P0_3::GpioP0,
            1 => P0_3::Rxd0,
            2 => P0_3::Ad0,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P0.3."]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_3::GpioP0
    }
    #[doc = "RXD0"]
    #[inline(always)]
    pub fn is_rxd0(&self) -> bool {
        *self == P0_3::Rxd0
    }
    #[doc = "AD0.6"]
    #[inline(always)]
    pub fn is_ad0(&self) -> bool {
        *self == P0_3::Ad0
    }
}
#[doc = "Field `P0_3` writer - Pin function select P0.3."]
pub type P0_3W<'a, REG> = crate::FieldWriter<'a, REG, 2, P0_3>;
impl<'a, REG> P0_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P0.3."]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut crate::W<REG> {
        self.variant(P0_3::GpioP0)
    }
    #[doc = "RXD0"]
    #[inline(always)]
    pub fn rxd0(self) -> &'a mut crate::W<REG> {
        self.variant(P0_3::Rxd0)
    }
    #[doc = "AD0.6"]
    #[inline(always)]
    pub fn ad0(self) -> &'a mut crate::W<REG> {
        self.variant(P0_3::Ad0)
    }
}
#[doc = "Pin function select P0.4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P0_4 {
    #[doc = "0: GPIO P0.4."]
    GpioP0 = 0,
    #[doc = "1: I2SRX_CLK"]
    I2srxClk = 1,
    #[doc = "2: RD2"]
    Rd2 = 2,
    #[doc = "3: CAP2.0"]
    Cap2 = 3,
}
impl From<P0_4> for u8 {
    #[inline(always)]
    fn from(variant: P0_4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P0_4 {
    type Ux = u8;
}
impl crate::IsEnum for P0_4 {}
#[doc = "Field `P0_4` reader - Pin function select P0.4."]
pub type P0_4R = crate::FieldReader<P0_4>;
impl P0_4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_4 {
        match self.bits {
            0 => P0_4::GpioP0,
            1 => P0_4::I2srxClk,
            2 => P0_4::Rd2,
            3 => P0_4::Cap2,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P0.4."]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_4::GpioP0
    }
    #[doc = "I2SRX_CLK"]
    #[inline(always)]
    pub fn is_i2srx_clk(&self) -> bool {
        *self == P0_4::I2srxClk
    }
    #[doc = "RD2"]
    #[inline(always)]
    pub fn is_rd2(&self) -> bool {
        *self == P0_4::Rd2
    }
    #[doc = "CAP2.0"]
    #[inline(always)]
    pub fn is_cap2(&self) -> bool {
        *self == P0_4::Cap2
    }
}
#[doc = "Field `P0_4` writer - Pin function select P0.4."]
pub type P0_4W<'a, REG> = crate::FieldWriter<'a, REG, 2, P0_4, crate::Safe>;
impl<'a, REG> P0_4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P0.4."]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut crate::W<REG> {
        self.variant(P0_4::GpioP0)
    }
    #[doc = "I2SRX_CLK"]
    #[inline(always)]
    pub fn i2srx_clk(self) -> &'a mut crate::W<REG> {
        self.variant(P0_4::I2srxClk)
    }
    #[doc = "RD2"]
    #[inline(always)]
    pub fn rd2(self) -> &'a mut crate::W<REG> {
        self.variant(P0_4::Rd2)
    }
    #[doc = "CAP2.0"]
    #[inline(always)]
    pub fn cap2(self) -> &'a mut crate::W<REG> {
        self.variant(P0_4::Cap2)
    }
}
#[doc = "Pin function select P0.5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P0_5 {
    #[doc = "0: GPIO P0.5."]
    GpioP0 = 0,
    #[doc = "1: I2SRX_WS"]
    I2srxWs = 1,
    #[doc = "2: TD2"]
    Td2 = 2,
    #[doc = "3: CAP2.1"]
    Cap2 = 3,
}
impl From<P0_5> for u8 {
    #[inline(always)]
    fn from(variant: P0_5) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P0_5 {
    type Ux = u8;
}
impl crate::IsEnum for P0_5 {}
#[doc = "Field `P0_5` reader - Pin function select P0.5."]
pub type P0_5R = crate::FieldReader<P0_5>;
impl P0_5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_5 {
        match self.bits {
            0 => P0_5::GpioP0,
            1 => P0_5::I2srxWs,
            2 => P0_5::Td2,
            3 => P0_5::Cap2,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P0.5."]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_5::GpioP0
    }
    #[doc = "I2SRX_WS"]
    #[inline(always)]
    pub fn is_i2srx_ws(&self) -> bool {
        *self == P0_5::I2srxWs
    }
    #[doc = "TD2"]
    #[inline(always)]
    pub fn is_td2(&self) -> bool {
        *self == P0_5::Td2
    }
    #[doc = "CAP2.1"]
    #[inline(always)]
    pub fn is_cap2(&self) -> bool {
        *self == P0_5::Cap2
    }
}
#[doc = "Field `P0_5` writer - Pin function select P0.5."]
pub type P0_5W<'a, REG> = crate::FieldWriter<'a, REG, 2, P0_5, crate::Safe>;
impl<'a, REG> P0_5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P0.5."]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut crate::W<REG> {
        self.variant(P0_5::GpioP0)
    }
    #[doc = "I2SRX_WS"]
    #[inline(always)]
    pub fn i2srx_ws(self) -> &'a mut crate::W<REG> {
        self.variant(P0_5::I2srxWs)
    }
    #[doc = "TD2"]
    #[inline(always)]
    pub fn td2(self) -> &'a mut crate::W<REG> {
        self.variant(P0_5::Td2)
    }
    #[doc = "CAP2.1"]
    #[inline(always)]
    pub fn cap2(self) -> &'a mut crate::W<REG> {
        self.variant(P0_5::Cap2)
    }
}
#[doc = "Pin function select P0.6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P0_6 {
    #[doc = "0: GPIO P0.6."]
    GpioP0 = 0,
    #[doc = "1: I2SRX_SDA"]
    I2srxSda = 1,
    #[doc = "2: SSEL1"]
    Ssel1 = 2,
    #[doc = "3: MAT2.0"]
    Mat2 = 3,
}
impl From<P0_6> for u8 {
    #[inline(always)]
    fn from(variant: P0_6) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P0_6 {
    type Ux = u8;
}
impl crate::IsEnum for P0_6 {}
#[doc = "Field `P0_6` reader - Pin function select P0.6."]
pub type P0_6R = crate::FieldReader<P0_6>;
impl P0_6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_6 {
        match self.bits {
            0 => P0_6::GpioP0,
            1 => P0_6::I2srxSda,
            2 => P0_6::Ssel1,
            3 => P0_6::Mat2,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P0.6."]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_6::GpioP0
    }
    #[doc = "I2SRX_SDA"]
    #[inline(always)]
    pub fn is_i2srx_sda(&self) -> bool {
        *self == P0_6::I2srxSda
    }
    #[doc = "SSEL1"]
    #[inline(always)]
    pub fn is_ssel1(&self) -> bool {
        *self == P0_6::Ssel1
    }
    #[doc = "MAT2.0"]
    #[inline(always)]
    pub fn is_mat2(&self) -> bool {
        *self == P0_6::Mat2
    }
}
#[doc = "Field `P0_6` writer - Pin function select P0.6."]
pub type P0_6W<'a, REG> = crate::FieldWriter<'a, REG, 2, P0_6, crate::Safe>;
impl<'a, REG> P0_6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P0.6."]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut crate::W<REG> {
        self.variant(P0_6::GpioP0)
    }
    #[doc = "I2SRX_SDA"]
    #[inline(always)]
    pub fn i2srx_sda(self) -> &'a mut crate::W<REG> {
        self.variant(P0_6::I2srxSda)
    }
    #[doc = "SSEL1"]
    #[inline(always)]
    pub fn ssel1(self) -> &'a mut crate::W<REG> {
        self.variant(P0_6::Ssel1)
    }
    #[doc = "MAT2.0"]
    #[inline(always)]
    pub fn mat2(self) -> &'a mut crate::W<REG> {
        self.variant(P0_6::Mat2)
    }
}
#[doc = "Pin function select P0.7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P0_7 {
    #[doc = "0: GPIO P0.7."]
    GpioP0 = 0,
    #[doc = "1: I2STX_CLK"]
    I2stxClk = 1,
    #[doc = "2: SCK1"]
    Sck1 = 2,
    #[doc = "3: MAT2.1"]
    Mat2 = 3,
}
impl From<P0_7> for u8 {
    #[inline(always)]
    fn from(variant: P0_7) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P0_7 {
    type Ux = u8;
}
impl crate::IsEnum for P0_7 {}
#[doc = "Field `P0_7` reader - Pin function select P0.7."]
pub type P0_7R = crate::FieldReader<P0_7>;
impl P0_7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_7 {
        match self.bits {
            0 => P0_7::GpioP0,
            1 => P0_7::I2stxClk,
            2 => P0_7::Sck1,
            3 => P0_7::Mat2,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P0.7."]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_7::GpioP0
    }
    #[doc = "I2STX_CLK"]
    #[inline(always)]
    pub fn is_i2stx_clk(&self) -> bool {
        *self == P0_7::I2stxClk
    }
    #[doc = "SCK1"]
    #[inline(always)]
    pub fn is_sck1(&self) -> bool {
        *self == P0_7::Sck1
    }
    #[doc = "MAT2.1"]
    #[inline(always)]
    pub fn is_mat2(&self) -> bool {
        *self == P0_7::Mat2
    }
}
#[doc = "Field `P0_7` writer - Pin function select P0.7."]
pub type P0_7W<'a, REG> = crate::FieldWriter<'a, REG, 2, P0_7, crate::Safe>;
impl<'a, REG> P0_7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P0.7."]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut crate::W<REG> {
        self.variant(P0_7::GpioP0)
    }
    #[doc = "I2STX_CLK"]
    #[inline(always)]
    pub fn i2stx_clk(self) -> &'a mut crate::W<REG> {
        self.variant(P0_7::I2stxClk)
    }
    #[doc = "SCK1"]
    #[inline(always)]
    pub fn sck1(self) -> &'a mut crate::W<REG> {
        self.variant(P0_7::Sck1)
    }
    #[doc = "MAT2.1"]
    #[inline(always)]
    pub fn mat2(self) -> &'a mut crate::W<REG> {
        self.variant(P0_7::Mat2)
    }
}
#[doc = "Pin function select P0.8.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P0_8 {
    #[doc = "0: GPIO P0.8."]
    GpioP0 = 0,
    #[doc = "1: I2STX_WS"]
    I2stxWs = 1,
    #[doc = "2: MISO1"]
    Miso1 = 2,
    #[doc = "3: MAT2.2"]
    Mat2 = 3,
}
impl From<P0_8> for u8 {
    #[inline(always)]
    fn from(variant: P0_8) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P0_8 {
    type Ux = u8;
}
impl crate::IsEnum for P0_8 {}
#[doc = "Field `P0_8` reader - Pin function select P0.8."]
pub type P0_8R = crate::FieldReader<P0_8>;
impl P0_8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_8 {
        match self.bits {
            0 => P0_8::GpioP0,
            1 => P0_8::I2stxWs,
            2 => P0_8::Miso1,
            3 => P0_8::Mat2,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P0.8."]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_8::GpioP0
    }
    #[doc = "I2STX_WS"]
    #[inline(always)]
    pub fn is_i2stx_ws(&self) -> bool {
        *self == P0_8::I2stxWs
    }
    #[doc = "MISO1"]
    #[inline(always)]
    pub fn is_miso1(&self) -> bool {
        *self == P0_8::Miso1
    }
    #[doc = "MAT2.2"]
    #[inline(always)]
    pub fn is_mat2(&self) -> bool {
        *self == P0_8::Mat2
    }
}
#[doc = "Field `P0_8` writer - Pin function select P0.8."]
pub type P0_8W<'a, REG> = crate::FieldWriter<'a, REG, 2, P0_8, crate::Safe>;
impl<'a, REG> P0_8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P0.8."]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut crate::W<REG> {
        self.variant(P0_8::GpioP0)
    }
    #[doc = "I2STX_WS"]
    #[inline(always)]
    pub fn i2stx_ws(self) -> &'a mut crate::W<REG> {
        self.variant(P0_8::I2stxWs)
    }
    #[doc = "MISO1"]
    #[inline(always)]
    pub fn miso1(self) -> &'a mut crate::W<REG> {
        self.variant(P0_8::Miso1)
    }
    #[doc = "MAT2.2"]
    #[inline(always)]
    pub fn mat2(self) -> &'a mut crate::W<REG> {
        self.variant(P0_8::Mat2)
    }
}
#[doc = "Pin function select P0.9.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P0_9 {
    #[doc = "0: GPIO P0.9"]
    GpioP0 = 0,
    #[doc = "1: I2STX_SDA"]
    I2stxSda = 1,
    #[doc = "2: MOSI1"]
    Mosi1 = 2,
    #[doc = "3: MAT2.3"]
    Mat2 = 3,
}
impl From<P0_9> for u8 {
    #[inline(always)]
    fn from(variant: P0_9) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P0_9 {
    type Ux = u8;
}
impl crate::IsEnum for P0_9 {}
#[doc = "Field `P0_9` reader - Pin function select P0.9."]
pub type P0_9R = crate::FieldReader<P0_9>;
impl P0_9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_9 {
        match self.bits {
            0 => P0_9::GpioP0,
            1 => P0_9::I2stxSda,
            2 => P0_9::Mosi1,
            3 => P0_9::Mat2,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P0.9"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_9::GpioP0
    }
    #[doc = "I2STX_SDA"]
    #[inline(always)]
    pub fn is_i2stx_sda(&self) -> bool {
        *self == P0_9::I2stxSda
    }
    #[doc = "MOSI1"]
    #[inline(always)]
    pub fn is_mosi1(&self) -> bool {
        *self == P0_9::Mosi1
    }
    #[doc = "MAT2.3"]
    #[inline(always)]
    pub fn is_mat2(&self) -> bool {
        *self == P0_9::Mat2
    }
}
#[doc = "Field `P0_9` writer - Pin function select P0.9."]
pub type P0_9W<'a, REG> = crate::FieldWriter<'a, REG, 2, P0_9, crate::Safe>;
impl<'a, REG> P0_9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P0.9"]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut crate::W<REG> {
        self.variant(P0_9::GpioP0)
    }
    #[doc = "I2STX_SDA"]
    #[inline(always)]
    pub fn i2stx_sda(self) -> &'a mut crate::W<REG> {
        self.variant(P0_9::I2stxSda)
    }
    #[doc = "MOSI1"]
    #[inline(always)]
    pub fn mosi1(self) -> &'a mut crate::W<REG> {
        self.variant(P0_9::Mosi1)
    }
    #[doc = "MAT2.3"]
    #[inline(always)]
    pub fn mat2(self) -> &'a mut crate::W<REG> {
        self.variant(P0_9::Mat2)
    }
}
#[doc = "Pin function select P0.10.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P0_10 {
    #[doc = "0: GPIO P0.10"]
    GpioP0 = 0,
    #[doc = "1: TXD2"]
    Txd2 = 1,
    #[doc = "2: SDA2"]
    Sda2 = 2,
    #[doc = "3: MAT3.0"]
    Mat3 = 3,
}
impl From<P0_10> for u8 {
    #[inline(always)]
    fn from(variant: P0_10) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P0_10 {
    type Ux = u8;
}
impl crate::IsEnum for P0_10 {}
#[doc = "Field `P0_10` reader - Pin function select P0.10."]
pub type P0_10R = crate::FieldReader<P0_10>;
impl P0_10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_10 {
        match self.bits {
            0 => P0_10::GpioP0,
            1 => P0_10::Txd2,
            2 => P0_10::Sda2,
            3 => P0_10::Mat3,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P0.10"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_10::GpioP0
    }
    #[doc = "TXD2"]
    #[inline(always)]
    pub fn is_txd2(&self) -> bool {
        *self == P0_10::Txd2
    }
    #[doc = "SDA2"]
    #[inline(always)]
    pub fn is_sda2(&self) -> bool {
        *self == P0_10::Sda2
    }
    #[doc = "MAT3.0"]
    #[inline(always)]
    pub fn is_mat3(&self) -> bool {
        *self == P0_10::Mat3
    }
}
#[doc = "Field `P0_10` writer - Pin function select P0.10."]
pub type P0_10W<'a, REG> = crate::FieldWriter<'a, REG, 2, P0_10, crate::Safe>;
impl<'a, REG> P0_10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P0.10"]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut crate::W<REG> {
        self.variant(P0_10::GpioP0)
    }
    #[doc = "TXD2"]
    #[inline(always)]
    pub fn txd2(self) -> &'a mut crate::W<REG> {
        self.variant(P0_10::Txd2)
    }
    #[doc = "SDA2"]
    #[inline(always)]
    pub fn sda2(self) -> &'a mut crate::W<REG> {
        self.variant(P0_10::Sda2)
    }
    #[doc = "MAT3.0"]
    #[inline(always)]
    pub fn mat3(self) -> &'a mut crate::W<REG> {
        self.variant(P0_10::Mat3)
    }
}
#[doc = "Pin function select P0.11.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P0_11 {
    #[doc = "0: GPIO P0.11"]
    GpioP0 = 0,
    #[doc = "1: RXD2"]
    Rxd2 = 1,
    #[doc = "2: SCL2"]
    Scl2 = 2,
    #[doc = "3: MAT3.1"]
    Mat3 = 3,
}
impl From<P0_11> for u8 {
    #[inline(always)]
    fn from(variant: P0_11) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P0_11 {
    type Ux = u8;
}
impl crate::IsEnum for P0_11 {}
#[doc = "Field `P0_11` reader - Pin function select P0.11."]
pub type P0_11R = crate::FieldReader<P0_11>;
impl P0_11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_11 {
        match self.bits {
            0 => P0_11::GpioP0,
            1 => P0_11::Rxd2,
            2 => P0_11::Scl2,
            3 => P0_11::Mat3,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P0.11"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_11::GpioP0
    }
    #[doc = "RXD2"]
    #[inline(always)]
    pub fn is_rxd2(&self) -> bool {
        *self == P0_11::Rxd2
    }
    #[doc = "SCL2"]
    #[inline(always)]
    pub fn is_scl2(&self) -> bool {
        *self == P0_11::Scl2
    }
    #[doc = "MAT3.1"]
    #[inline(always)]
    pub fn is_mat3(&self) -> bool {
        *self == P0_11::Mat3
    }
}
#[doc = "Field `P0_11` writer - Pin function select P0.11."]
pub type P0_11W<'a, REG> = crate::FieldWriter<'a, REG, 2, P0_11, crate::Safe>;
impl<'a, REG> P0_11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P0.11"]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut crate::W<REG> {
        self.variant(P0_11::GpioP0)
    }
    #[doc = "RXD2"]
    #[inline(always)]
    pub fn rxd2(self) -> &'a mut crate::W<REG> {
        self.variant(P0_11::Rxd2)
    }
    #[doc = "SCL2"]
    #[inline(always)]
    pub fn scl2(self) -> &'a mut crate::W<REG> {
        self.variant(P0_11::Scl2)
    }
    #[doc = "MAT3.1"]
    #[inline(always)]
    pub fn mat3(self) -> &'a mut crate::W<REG> {
        self.variant(P0_11::Mat3)
    }
}
#[doc = "Pin function select P0.15.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P0_15 {
    #[doc = "0: GPIO P0.15"]
    GpioP0 = 0,
    #[doc = "1: TXD1"]
    Txd1 = 1,
    #[doc = "2: SCK0"]
    Sck0 = 2,
    #[doc = "3: SCK"]
    Sck = 3,
}
impl From<P0_15> for u8 {
    #[inline(always)]
    fn from(variant: P0_15) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P0_15 {
    type Ux = u8;
}
impl crate::IsEnum for P0_15 {}
#[doc = "Field `P0_15` reader - Pin function select P0.15."]
pub type P0_15R = crate::FieldReader<P0_15>;
impl P0_15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_15 {
        match self.bits {
            0 => P0_15::GpioP0,
            1 => P0_15::Txd1,
            2 => P0_15::Sck0,
            3 => P0_15::Sck,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P0.15"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_15::GpioP0
    }
    #[doc = "TXD1"]
    #[inline(always)]
    pub fn is_txd1(&self) -> bool {
        *self == P0_15::Txd1
    }
    #[doc = "SCK0"]
    #[inline(always)]
    pub fn is_sck0(&self) -> bool {
        *self == P0_15::Sck0
    }
    #[doc = "SCK"]
    #[inline(always)]
    pub fn is_sck(&self) -> bool {
        *self == P0_15::Sck
    }
}
#[doc = "Field `P0_15` writer - Pin function select P0.15."]
pub type P0_15W<'a, REG> = crate::FieldWriter<'a, REG, 2, P0_15, crate::Safe>;
impl<'a, REG> P0_15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P0.15"]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut crate::W<REG> {
        self.variant(P0_15::GpioP0)
    }
    #[doc = "TXD1"]
    #[inline(always)]
    pub fn txd1(self) -> &'a mut crate::W<REG> {
        self.variant(P0_15::Txd1)
    }
    #[doc = "SCK0"]
    #[inline(always)]
    pub fn sck0(self) -> &'a mut crate::W<REG> {
        self.variant(P0_15::Sck0)
    }
    #[doc = "SCK"]
    #[inline(always)]
    pub fn sck(self) -> &'a mut crate::W<REG> {
        self.variant(P0_15::Sck)
    }
}
impl R {
    #[doc = "Bits 0:1 - Pin function select P0.0."]
    #[inline(always)]
    pub fn p0_0(&self) -> P0_0R {
        P0_0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Pin function select P0.1."]
    #[inline(always)]
    pub fn p0_1(&self) -> P0_1R {
        P0_1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Pin function select P0.2."]
    #[inline(always)]
    pub fn p0_2(&self) -> P0_2R {
        P0_2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Pin function select P0.3."]
    #[inline(always)]
    pub fn p0_3(&self) -> P0_3R {
        P0_3R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Pin function select P0.4."]
    #[inline(always)]
    pub fn p0_4(&self) -> P0_4R {
        P0_4R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Pin function select P0.5."]
    #[inline(always)]
    pub fn p0_5(&self) -> P0_5R {
        P0_5R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Pin function select P0.6."]
    #[inline(always)]
    pub fn p0_6(&self) -> P0_6R {
        P0_6R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Pin function select P0.7."]
    #[inline(always)]
    pub fn p0_7(&self) -> P0_7R {
        P0_7R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Pin function select P0.8."]
    #[inline(always)]
    pub fn p0_8(&self) -> P0_8R {
        P0_8R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Pin function select P0.9."]
    #[inline(always)]
    pub fn p0_9(&self) -> P0_9R {
        P0_9R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Pin function select P0.10."]
    #[inline(always)]
    pub fn p0_10(&self) -> P0_10R {
        P0_10R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Pin function select P0.11."]
    #[inline(always)]
    pub fn p0_11(&self) -> P0_11R {
        P0_11R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Pin function select P0.15."]
    #[inline(always)]
    pub fn p0_15(&self) -> P0_15R {
        P0_15R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Pin function select P0.0."]
    #[inline(always)]
    #[must_use]
    pub fn p0_0(&mut self) -> P0_0W<Pinsel0Spec> {
        P0_0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Pin function select P0.1."]
    #[inline(always)]
    #[must_use]
    pub fn p0_1(&mut self) -> P0_1W<Pinsel0Spec> {
        P0_1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Pin function select P0.2."]
    #[inline(always)]
    #[must_use]
    pub fn p0_2(&mut self) -> P0_2W<Pinsel0Spec> {
        P0_2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Pin function select P0.3."]
    #[inline(always)]
    #[must_use]
    pub fn p0_3(&mut self) -> P0_3W<Pinsel0Spec> {
        P0_3W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Pin function select P0.4."]
    #[inline(always)]
    #[must_use]
    pub fn p0_4(&mut self) -> P0_4W<Pinsel0Spec> {
        P0_4W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Pin function select P0.5."]
    #[inline(always)]
    #[must_use]
    pub fn p0_5(&mut self) -> P0_5W<Pinsel0Spec> {
        P0_5W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Pin function select P0.6."]
    #[inline(always)]
    #[must_use]
    pub fn p0_6(&mut self) -> P0_6W<Pinsel0Spec> {
        P0_6W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Pin function select P0.7."]
    #[inline(always)]
    #[must_use]
    pub fn p0_7(&mut self) -> P0_7W<Pinsel0Spec> {
        P0_7W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Pin function select P0.8."]
    #[inline(always)]
    #[must_use]
    pub fn p0_8(&mut self) -> P0_8W<Pinsel0Spec> {
        P0_8W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Pin function select P0.9."]
    #[inline(always)]
    #[must_use]
    pub fn p0_9(&mut self) -> P0_9W<Pinsel0Spec> {
        P0_9W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Pin function select P0.10."]
    #[inline(always)]
    #[must_use]
    pub fn p0_10(&mut self) -> P0_10W<Pinsel0Spec> {
        P0_10W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Pin function select P0.11."]
    #[inline(always)]
    #[must_use]
    pub fn p0_11(&mut self) -> P0_11W<Pinsel0Spec> {
        P0_11W::new(self, 22)
    }
    #[doc = "Bits 30:31 - Pin function select P0.15."]
    #[inline(always)]
    #[must_use]
    pub fn p0_15(&mut self) -> P0_15W<Pinsel0Spec> {
        P0_15W::new(self, 30)
    }
}
#[doc = "Pin function select register 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`pinsel0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pinsel0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pinsel0Spec;
impl crate::RegisterSpec for Pinsel0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pinsel0::R`](R) reader structure"]
impl crate::Readable for Pinsel0Spec {}
#[doc = "`write(|w| ..)` method takes [`pinsel0::W`](W) writer structure"]
impl crate::Writable for Pinsel0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PINSEL0 to value 0"]
impl crate::Resettable for Pinsel0Spec {
    const RESET_VALUE: u32 = 0;
}
