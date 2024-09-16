#[doc = "Register `PINSEL2` reader"]
pub type R = crate::R<Pinsel2Spec>;
#[doc = "Register `PINSEL2` writer"]
pub type W = crate::W<Pinsel2Spec>;
#[doc = "Pin function select P1.0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_0 {
    #[doc = "0: GPIO P1.0"]
    GpioP1 = 0,
    #[doc = "1: ENET_TXD0"]
    EnetTxd0 = 1,
}
impl From<P1_0> for u8 {
    #[inline(always)]
    fn from(variant: P1_0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_0 {
    type Ux = u8;
}
impl crate::IsEnum for P1_0 {}
#[doc = "Field `P1_0` reader - Pin function select P1.0."]
pub type P1_0R = crate::FieldReader<P1_0>;
impl P1_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_0 {
        match self.bits {
            0 => P1_0::GpioP1,
            1 => P1_0::EnetTxd0,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P1.0"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_0::GpioP1
    }
    #[doc = "ENET_TXD0"]
    #[inline(always)]
    pub fn is_enet_txd0(&self) -> bool {
        *self == P1_0::EnetTxd0
    }
}
#[doc = "Field `P1_0` writer - Pin function select P1.0."]
pub type P1_0W<'a, REG> = crate::FieldWriter<'a, REG, 2, P1_0>;
impl<'a, REG> P1_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P1.0"]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_0::GpioP1)
    }
    #[doc = "ENET_TXD0"]
    #[inline(always)]
    pub fn enet_txd0(self) -> &'a mut crate::W<REG> {
        self.variant(P1_0::EnetTxd0)
    }
}
#[doc = "Pin function select P1.1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_1 {
    #[doc = "0: GPIO P1.1"]
    GpioP1 = 0,
    #[doc = "1: ENET_TXD1"]
    EnetTxd1 = 1,
}
impl From<P1_1> for u8 {
    #[inline(always)]
    fn from(variant: P1_1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_1 {
    type Ux = u8;
}
impl crate::IsEnum for P1_1 {}
#[doc = "Field `P1_1` reader - Pin function select P1.1."]
pub type P1_1R = crate::FieldReader<P1_1>;
impl P1_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_1 {
        match self.bits {
            0 => P1_1::GpioP1,
            1 => P1_1::EnetTxd1,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P1.1"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_1::GpioP1
    }
    #[doc = "ENET_TXD1"]
    #[inline(always)]
    pub fn is_enet_txd1(&self) -> bool {
        *self == P1_1::EnetTxd1
    }
}
#[doc = "Field `P1_1` writer - Pin function select P1.1."]
pub type P1_1W<'a, REG> = crate::FieldWriter<'a, REG, 2, P1_1>;
impl<'a, REG> P1_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P1.1"]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_1::GpioP1)
    }
    #[doc = "ENET_TXD1"]
    #[inline(always)]
    pub fn enet_txd1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_1::EnetTxd1)
    }
}
#[doc = "Pin function select P1.4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_4 {
    #[doc = "0: GPIO P1.4."]
    GpioP1 = 0,
    #[doc = "1: ENET_TX_EN"]
    EnetTxEn = 1,
}
impl From<P1_4> for u8 {
    #[inline(always)]
    fn from(variant: P1_4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_4 {
    type Ux = u8;
}
impl crate::IsEnum for P1_4 {}
#[doc = "Field `P1_4` reader - Pin function select P1.4."]
pub type P1_4R = crate::FieldReader<P1_4>;
impl P1_4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_4 {
        match self.bits {
            0 => P1_4::GpioP1,
            1 => P1_4::EnetTxEn,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P1.4."]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_4::GpioP1
    }
    #[doc = "ENET_TX_EN"]
    #[inline(always)]
    pub fn is_enet_tx_en(&self) -> bool {
        *self == P1_4::EnetTxEn
    }
}
#[doc = "Field `P1_4` writer - Pin function select P1.4."]
pub type P1_4W<'a, REG> = crate::FieldWriter<'a, REG, 2, P1_4>;
impl<'a, REG> P1_4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P1.4."]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_4::GpioP1)
    }
    #[doc = "ENET_TX_EN"]
    #[inline(always)]
    pub fn enet_tx_en(self) -> &'a mut crate::W<REG> {
        self.variant(P1_4::EnetTxEn)
    }
}
#[doc = "Pin function select P1.8.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_8 {
    #[doc = "0: GPIO P1.8."]
    GpioP1 = 0,
    #[doc = "1: ENET_CRS"]
    EnetCrs = 1,
}
impl From<P1_8> for u8 {
    #[inline(always)]
    fn from(variant: P1_8) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_8 {
    type Ux = u8;
}
impl crate::IsEnum for P1_8 {}
#[doc = "Field `P1_8` reader - Pin function select P1.8."]
pub type P1_8R = crate::FieldReader<P1_8>;
impl P1_8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_8 {
        match self.bits {
            0 => P1_8::GpioP1,
            1 => P1_8::EnetCrs,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P1.8."]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_8::GpioP1
    }
    #[doc = "ENET_CRS"]
    #[inline(always)]
    pub fn is_enet_crs(&self) -> bool {
        *self == P1_8::EnetCrs
    }
}
#[doc = "Field `P1_8` writer - Pin function select P1.8."]
pub type P1_8W<'a, REG> = crate::FieldWriter<'a, REG, 2, P1_8>;
impl<'a, REG> P1_8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P1.8."]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_8::GpioP1)
    }
    #[doc = "ENET_CRS"]
    #[inline(always)]
    pub fn enet_crs(self) -> &'a mut crate::W<REG> {
        self.variant(P1_8::EnetCrs)
    }
}
#[doc = "Pin function select P1.9.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_9 {
    #[doc = "0: GPIO Port 1.9"]
    GpioPort1 = 0,
    #[doc = "1: ENET_RXD0"]
    EnetRxd0 = 1,
}
impl From<P1_9> for u8 {
    #[inline(always)]
    fn from(variant: P1_9) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_9 {
    type Ux = u8;
}
impl crate::IsEnum for P1_9 {}
#[doc = "Field `P1_9` reader - Pin function select P1.9."]
pub type P1_9R = crate::FieldReader<P1_9>;
impl P1_9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_9 {
        match self.bits {
            0 => P1_9::GpioPort1,
            1 => P1_9::EnetRxd0,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO Port 1.9"]
    #[inline(always)]
    pub fn is_gpio_port_1(&self) -> bool {
        *self == P1_9::GpioPort1
    }
    #[doc = "ENET_RXD0"]
    #[inline(always)]
    pub fn is_enet_rxd0(&self) -> bool {
        *self == P1_9::EnetRxd0
    }
}
#[doc = "Field `P1_9` writer - Pin function select P1.9."]
pub type P1_9W<'a, REG> = crate::FieldWriter<'a, REG, 2, P1_9>;
impl<'a, REG> P1_9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO Port 1.9"]
    #[inline(always)]
    pub fn gpio_port_1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_9::GpioPort1)
    }
    #[doc = "ENET_RXD0"]
    #[inline(always)]
    pub fn enet_rxd0(self) -> &'a mut crate::W<REG> {
        self.variant(P1_9::EnetRxd0)
    }
}
#[doc = "Pin function select P1.10.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_10 {
    #[doc = "0: GPIO P1.10"]
    GpioP1 = 0,
    #[doc = "1: ENET_RXD1"]
    EnetRxd1 = 1,
}
impl From<P1_10> for u8 {
    #[inline(always)]
    fn from(variant: P1_10) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_10 {
    type Ux = u8;
}
impl crate::IsEnum for P1_10 {}
#[doc = "Field `P1_10` reader - Pin function select P1.10."]
pub type P1_10R = crate::FieldReader<P1_10>;
impl P1_10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_10 {
        match self.bits {
            0 => P1_10::GpioP1,
            1 => P1_10::EnetRxd1,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P1.10"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_10::GpioP1
    }
    #[doc = "ENET_RXD1"]
    #[inline(always)]
    pub fn is_enet_rxd1(&self) -> bool {
        *self == P1_10::EnetRxd1
    }
}
#[doc = "Field `P1_10` writer - Pin function select P1.10."]
pub type P1_10W<'a, REG> = crate::FieldWriter<'a, REG, 2, P1_10>;
impl<'a, REG> P1_10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P1.10"]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_10::GpioP1)
    }
    #[doc = "ENET_RXD1"]
    #[inline(always)]
    pub fn enet_rxd1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_10::EnetRxd1)
    }
}
#[doc = "Pin function select P1.14.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_14 {
    #[doc = "0: GPIO P1.14"]
    GpioP1 = 0,
    #[doc = "1: ENET_RX_ER"]
    EnetRxEr = 1,
}
impl From<P1_14> for u8 {
    #[inline(always)]
    fn from(variant: P1_14) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_14 {
    type Ux = u8;
}
impl crate::IsEnum for P1_14 {}
#[doc = "Field `P1_14` reader - Pin function select P1.14."]
pub type P1_14R = crate::FieldReader<P1_14>;
impl P1_14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_14 {
        match self.bits {
            0 => P1_14::GpioP1,
            1 => P1_14::EnetRxEr,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P1.14"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_14::GpioP1
    }
    #[doc = "ENET_RX_ER"]
    #[inline(always)]
    pub fn is_enet_rx_er(&self) -> bool {
        *self == P1_14::EnetRxEr
    }
}
#[doc = "Field `P1_14` writer - Pin function select P1.14."]
pub type P1_14W<'a, REG> = crate::FieldWriter<'a, REG, 2, P1_14>;
impl<'a, REG> P1_14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P1.14"]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_14::GpioP1)
    }
    #[doc = "ENET_RX_ER"]
    #[inline(always)]
    pub fn enet_rx_er(self) -> &'a mut crate::W<REG> {
        self.variant(P1_14::EnetRxEr)
    }
}
#[doc = "Pin function select P1.15.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_15 {
    #[doc = "0: GPIO P1.15"]
    GpioP1 = 0,
    #[doc = "1: ENET_REF_CLK"]
    EnetRefClk = 1,
}
impl From<P1_15> for u8 {
    #[inline(always)]
    fn from(variant: P1_15) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_15 {
    type Ux = u8;
}
impl crate::IsEnum for P1_15 {}
#[doc = "Field `P1_15` reader - Pin function select P1.15."]
pub type P1_15R = crate::FieldReader<P1_15>;
impl P1_15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_15 {
        match self.bits {
            0 => P1_15::GpioP1,
            1 => P1_15::EnetRefClk,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P1.15"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_15::GpioP1
    }
    #[doc = "ENET_REF_CLK"]
    #[inline(always)]
    pub fn is_enet_ref_clk(&self) -> bool {
        *self == P1_15::EnetRefClk
    }
}
#[doc = "Field `P1_15` writer - Pin function select P1.15."]
pub type P1_15W<'a, REG> = crate::FieldWriter<'a, REG, 2, P1_15>;
impl<'a, REG> P1_15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P1.15"]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_15::GpioP1)
    }
    #[doc = "ENET_REF_CLK"]
    #[inline(always)]
    pub fn enet_ref_clk(self) -> &'a mut crate::W<REG> {
        self.variant(P1_15::EnetRefClk)
    }
}
impl R {
    #[doc = "Bits 0:1 - Pin function select P1.0."]
    #[inline(always)]
    pub fn p1_0(&self) -> P1_0R {
        P1_0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Pin function select P1.1."]
    #[inline(always)]
    pub fn p1_1(&self) -> P1_1R {
        P1_1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Pin function select P1.4."]
    #[inline(always)]
    pub fn p1_4(&self) -> P1_4R {
        P1_4R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Pin function select P1.8."]
    #[inline(always)]
    pub fn p1_8(&self) -> P1_8R {
        P1_8R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Pin function select P1.9."]
    #[inline(always)]
    pub fn p1_9(&self) -> P1_9R {
        P1_9R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Pin function select P1.10."]
    #[inline(always)]
    pub fn p1_10(&self) -> P1_10R {
        P1_10R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Pin function select P1.14."]
    #[inline(always)]
    pub fn p1_14(&self) -> P1_14R {
        P1_14R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Pin function select P1.15."]
    #[inline(always)]
    pub fn p1_15(&self) -> P1_15R {
        P1_15R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Pin function select P1.0."]
    #[inline(always)]
    #[must_use]
    pub fn p1_0(&mut self) -> P1_0W<Pinsel2Spec> {
        P1_0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Pin function select P1.1."]
    #[inline(always)]
    #[must_use]
    pub fn p1_1(&mut self) -> P1_1W<Pinsel2Spec> {
        P1_1W::new(self, 2)
    }
    #[doc = "Bits 8:9 - Pin function select P1.4."]
    #[inline(always)]
    #[must_use]
    pub fn p1_4(&mut self) -> P1_4W<Pinsel2Spec> {
        P1_4W::new(self, 8)
    }
    #[doc = "Bits 16:17 - Pin function select P1.8."]
    #[inline(always)]
    #[must_use]
    pub fn p1_8(&mut self) -> P1_8W<Pinsel2Spec> {
        P1_8W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Pin function select P1.9."]
    #[inline(always)]
    #[must_use]
    pub fn p1_9(&mut self) -> P1_9W<Pinsel2Spec> {
        P1_9W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Pin function select P1.10."]
    #[inline(always)]
    #[must_use]
    pub fn p1_10(&mut self) -> P1_10W<Pinsel2Spec> {
        P1_10W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Pin function select P1.14."]
    #[inline(always)]
    #[must_use]
    pub fn p1_14(&mut self) -> P1_14W<Pinsel2Spec> {
        P1_14W::new(self, 22)
    }
    #[doc = "Bits 30:31 - Pin function select P1.15."]
    #[inline(always)]
    #[must_use]
    pub fn p1_15(&mut self) -> P1_15W<Pinsel2Spec> {
        P1_15W::new(self, 30)
    }
}
#[doc = "Pin function select register 2.\n\nYou can [`read`](crate::Reg::read) this register and get [`pinsel2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pinsel2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pinsel2Spec;
impl crate::RegisterSpec for Pinsel2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pinsel2::R`](R) reader structure"]
impl crate::Readable for Pinsel2Spec {}
#[doc = "`write(|w| ..)` method takes [`pinsel2::W`](W) writer structure"]
impl crate::Writable for Pinsel2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PINSEL2 to value 0"]
impl crate::Resettable for Pinsel2Spec {
    const RESET_VALUE: u32 = 0;
}
