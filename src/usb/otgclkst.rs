#[doc = "Register `OTGCLKST` reader"]
pub type R = crate::R<OtgclkstSpec>;
#[doc = "Host clock status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HostClkOn {
    #[doc = "0: Host clock is not available."]
    HostClockIsNotAv = 0,
    #[doc = "1: Host clock is available."]
    HostClockIsAvaila = 1,
}
impl From<HostClkOn> for bool {
    #[inline(always)]
    fn from(variant: HostClkOn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HOST_CLK_ON` reader - Host clock status."]
pub type HostClkOnR = crate::BitReader<HostClkOn>;
impl HostClkOnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HostClkOn {
        match self.bits {
            false => HostClkOn::HostClockIsNotAv,
            true => HostClkOn::HostClockIsAvaila,
        }
    }
    #[doc = "Host clock is not available."]
    #[inline(always)]
    pub fn is_host_clock_is_not_av(&self) -> bool {
        *self == HostClkOn::HostClockIsNotAv
    }
    #[doc = "Host clock is available."]
    #[inline(always)]
    pub fn is_host_clock_is_availa(&self) -> bool {
        *self == HostClkOn::HostClockIsAvaila
    }
}
#[doc = "Device clock status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DevClkOn {
    #[doc = "0: Device clock is not available."]
    DeviceClockIsNot_ = 0,
    #[doc = "1: Device clock is available."]
    DeviceClockIsAvai = 1,
}
impl From<DevClkOn> for bool {
    #[inline(always)]
    fn from(variant: DevClkOn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEV_CLK_ON` reader - Device clock status."]
pub type DevClkOnR = crate::BitReader<DevClkOn>;
impl DevClkOnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DevClkOn {
        match self.bits {
            false => DevClkOn::DeviceClockIsNot_,
            true => DevClkOn::DeviceClockIsAvai,
        }
    }
    #[doc = "Device clock is not available."]
    #[inline(always)]
    pub fn is_device_clock_is_not_(&self) -> bool {
        *self == DevClkOn::DeviceClockIsNot_
    }
    #[doc = "Device clock is available."]
    #[inline(always)]
    pub fn is_device_clock_is_avai(&self) -> bool {
        *self == DevClkOn::DeviceClockIsAvai
    }
}
#[doc = "I2C clock status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2cClkOn {
    #[doc = "0: I2C clock is not available."]
    I2cClockIsNotAva = 0,
    #[doc = "1: I2C clock is available."]
    I2cClockIsAvailab = 1,
}
impl From<I2cClkOn> for bool {
    #[inline(always)]
    fn from(variant: I2cClkOn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C_CLK_ON` reader - I2C clock status."]
pub type I2cClkOnR = crate::BitReader<I2cClkOn>;
impl I2cClkOnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2cClkOn {
        match self.bits {
            false => I2cClkOn::I2cClockIsNotAva,
            true => I2cClkOn::I2cClockIsAvailab,
        }
    }
    #[doc = "I2C clock is not available."]
    #[inline(always)]
    pub fn is_i2c_clock_is_not_ava(&self) -> bool {
        *self == I2cClkOn::I2cClockIsNotAva
    }
    #[doc = "I2C clock is available."]
    #[inline(always)]
    pub fn is_i2c_clock_is_availab(&self) -> bool {
        *self == I2cClkOn::I2cClockIsAvailab
    }
}
#[doc = "OTG clock status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OtgClkOn {
    #[doc = "0: OTG clock is not available."]
    OtgClockIsNotAva = 0,
    #[doc = "1: OTG clock is available."]
    OtgClockIsAvailab = 1,
}
impl From<OtgClkOn> for bool {
    #[inline(always)]
    fn from(variant: OtgClkOn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OTG_CLK_ON` reader - OTG clock status."]
pub type OtgClkOnR = crate::BitReader<OtgClkOn>;
impl OtgClkOnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OtgClkOn {
        match self.bits {
            false => OtgClkOn::OtgClockIsNotAva,
            true => OtgClkOn::OtgClockIsAvailab,
        }
    }
    #[doc = "OTG clock is not available."]
    #[inline(always)]
    pub fn is_otg_clock_is_not_ava(&self) -> bool {
        *self == OtgClkOn::OtgClockIsNotAva
    }
    #[doc = "OTG clock is available."]
    #[inline(always)]
    pub fn is_otg_clock_is_availab(&self) -> bool {
        *self == OtgClkOn::OtgClockIsAvailab
    }
}
#[doc = "AHB master clock status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AhbClkOn {
    #[doc = "0: AHB clock is not available."]
    AhbClockIsNotAva = 0,
    #[doc = "1: AHB clock is available."]
    AhbClockIsAvailab = 1,
}
impl From<AhbClkOn> for bool {
    #[inline(always)]
    fn from(variant: AhbClkOn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHB_CLK_ON` reader - AHB master clock status."]
pub type AhbClkOnR = crate::BitReader<AhbClkOn>;
impl AhbClkOnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AhbClkOn {
        match self.bits {
            false => AhbClkOn::AhbClockIsNotAva,
            true => AhbClkOn::AhbClockIsAvailab,
        }
    }
    #[doc = "AHB clock is not available."]
    #[inline(always)]
    pub fn is_ahb_clock_is_not_ava(&self) -> bool {
        *self == AhbClkOn::AhbClockIsNotAva
    }
    #[doc = "AHB clock is available."]
    #[inline(always)]
    pub fn is_ahb_clock_is_availab(&self) -> bool {
        *self == AhbClkOn::AhbClockIsAvailab
    }
}
impl R {
    #[doc = "Bit 0 - Host clock status."]
    #[inline(always)]
    pub fn host_clk_on(&self) -> HostClkOnR {
        HostClkOnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Device clock status."]
    #[inline(always)]
    pub fn dev_clk_on(&self) -> DevClkOnR {
        DevClkOnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I2C clock status."]
    #[inline(always)]
    pub fn i2c_clk_on(&self) -> I2cClkOnR {
        I2cClkOnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OTG clock status."]
    #[inline(always)]
    pub fn otg_clk_on(&self) -> OtgClkOnR {
        OtgClkOnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AHB master clock status."]
    #[inline(always)]
    pub fn ahb_clk_on(&self) -> AhbClkOnR {
        AhbClkOnR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "OTG clock status\n\nYou can [`read`](crate::Reg::read) this register and get [`otgclkst::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtgclkstSpec;
impl crate::RegisterSpec for OtgclkstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otgclkst::R`](R) reader structure"]
impl crate::Readable for OtgclkstSpec {}
#[doc = "`reset()` method sets OTGCLKST to value 0"]
impl crate::Resettable for OtgclkstSpec {
    const RESET_VALUE: u32 = 0;
}
