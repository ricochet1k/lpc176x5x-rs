#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Read mode control. Enables reading data from the OUT endpoint buffer for the endpoint specified in the LOG_ENDPOINT field using the USBRxData register. This bit is cleared by hardware when the last word of the current packet is read from USBRxData.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RdEn {
    #[doc = "0: Disabled."]
    Disabled_ = 0,
    #[doc = "1: Enabled."]
    Enabled_ = 1,
}
impl From<RdEn> for bool {
    #[inline(always)]
    fn from(variant: RdEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RD_EN` reader - Read mode control. Enables reading data from the OUT endpoint buffer for the endpoint specified in the LOG_ENDPOINT field using the USBRxData register. This bit is cleared by hardware when the last word of the current packet is read from USBRxData."]
pub type RdEnR = crate::BitReader<RdEn>;
impl RdEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RdEn {
        match self.bits {
            false => RdEn::Disabled_,
            true => RdEn::Enabled_,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled_(&self) -> bool {
        *self == RdEn::Disabled_
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_enabled_(&self) -> bool {
        *self == RdEn::Enabled_
    }
}
#[doc = "Field `RD_EN` writer - Read mode control. Enables reading data from the OUT endpoint buffer for the endpoint specified in the LOG_ENDPOINT field using the USBRxData register. This bit is cleared by hardware when the last word of the current packet is read from USBRxData."]
pub type RdEnW<'a, REG> = crate::BitWriter<'a, REG, RdEn>;
impl<'a, REG> RdEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled_(self) -> &'a mut crate::W<REG> {
        self.variant(RdEn::Disabled_)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled_(self) -> &'a mut crate::W<REG> {
        self.variant(RdEn::Enabled_)
    }
}
#[doc = "Write mode control. Enables writing data to the IN endpoint buffer for the endpoint specified in the LOG_ENDPOINT field using the USBTxData register. This bit is cleared by hardware when the number of bytes in USBTxLen have been sent.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WrEn {
    #[doc = "0: Disabled."]
    Disabled_ = 0,
    #[doc = "1: Enabled."]
    Enabled_ = 1,
}
impl From<WrEn> for bool {
    #[inline(always)]
    fn from(variant: WrEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WR_EN` reader - Write mode control. Enables writing data to the IN endpoint buffer for the endpoint specified in the LOG_ENDPOINT field using the USBTxData register. This bit is cleared by hardware when the number of bytes in USBTxLen have been sent."]
pub type WrEnR = crate::BitReader<WrEn>;
impl WrEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WrEn {
        match self.bits {
            false => WrEn::Disabled_,
            true => WrEn::Enabled_,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled_(&self) -> bool {
        *self == WrEn::Disabled_
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_enabled_(&self) -> bool {
        *self == WrEn::Enabled_
    }
}
#[doc = "Field `WR_EN` writer - Write mode control. Enables writing data to the IN endpoint buffer for the endpoint specified in the LOG_ENDPOINT field using the USBTxData register. This bit is cleared by hardware when the number of bytes in USBTxLen have been sent."]
pub type WrEnW<'a, REG> = crate::BitWriter<'a, REG, WrEn>;
impl<'a, REG> WrEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled_(self) -> &'a mut crate::W<REG> {
        self.variant(WrEn::Disabled_)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled_(self) -> &'a mut crate::W<REG> {
        self.variant(WrEn::Enabled_)
    }
}
#[doc = "Field `LOG_ENDPOINT` reader - Logical Endpoint number."]
pub type LogEndpointR = crate::FieldReader;
#[doc = "Field `LOG_ENDPOINT` writer - Logical Endpoint number."]
pub type LogEndpointW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Read mode control. Enables reading data from the OUT endpoint buffer for the endpoint specified in the LOG_ENDPOINT field using the USBRxData register. This bit is cleared by hardware when the last word of the current packet is read from USBRxData."]
    #[inline(always)]
    pub fn rd_en(&self) -> RdEnR {
        RdEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write mode control. Enables writing data to the IN endpoint buffer for the endpoint specified in the LOG_ENDPOINT field using the USBTxData register. This bit is cleared by hardware when the number of bytes in USBTxLen have been sent."]
    #[inline(always)]
    pub fn wr_en(&self) -> WrEnR {
        WrEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - Logical Endpoint number."]
    #[inline(always)]
    pub fn log_endpoint(&self) -> LogEndpointR {
        LogEndpointR::new(((self.bits >> 2) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Read mode control. Enables reading data from the OUT endpoint buffer for the endpoint specified in the LOG_ENDPOINT field using the USBRxData register. This bit is cleared by hardware when the last word of the current packet is read from USBRxData."]
    #[inline(always)]
    #[must_use]
    pub fn rd_en(&mut self) -> RdEnW<CtrlSpec> {
        RdEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Write mode control. Enables writing data to the IN endpoint buffer for the endpoint specified in the LOG_ENDPOINT field using the USBTxData register. This bit is cleared by hardware when the number of bytes in USBTxLen have been sent."]
    #[inline(always)]
    #[must_use]
    pub fn wr_en(&mut self) -> WrEnW<CtrlSpec> {
        WrEnW::new(self, 1)
    }
    #[doc = "Bits 2:5 - Logical Endpoint number."]
    #[inline(always)]
    #[must_use]
    pub fn log_endpoint(&mut self) -> LogEndpointW<CtrlSpec> {
        LogEndpointW::new(self, 2)
    }
}
#[doc = "USB Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
