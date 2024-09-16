#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Field `ABRT` reader - Slave abort. When 1, this bit indicates that a slave abort has occurred. This bit is cleared by reading this register."]
pub type AbrtR = crate::BitReader;
#[doc = "Field `MODF` reader - Mode fault. when 1, this bit indicates that a Mode fault error has occurred. This bit is cleared by reading this register, then writing the SPI0 control register."]
pub type ModfR = crate::BitReader;
#[doc = "Field `ROVR` reader - Read overrun. When 1, this bit indicates that a read overrun has occurred. This bit is cleared by reading this register."]
pub type RovrR = crate::BitReader;
#[doc = "Field `WCOL` reader - Write collision. When 1, this bit indicates that a write collision has occurred. This bit is cleared by reading this register, then accessing the SPI Data Register."]
pub type WcolR = crate::BitReader;
#[doc = "Field `SPIF` reader - SPI transfer complete flag. When 1, this bit indicates when a SPI data transfer is complete. When a master, this bit is set at the end of the last cycle of the transfer. When a slave, this bit is set on the last data sampling edge of the SCK. This bit is cleared by first reading this register, then accessing the SPI Data Register. Note: this is not the SPI interrupt flag. This flag is found in the SPINT register."]
pub type SpifR = crate::BitReader;
impl R {
    #[doc = "Bit 3 - Slave abort. When 1, this bit indicates that a slave abort has occurred. This bit is cleared by reading this register."]
    #[inline(always)]
    pub fn abrt(&self) -> AbrtR {
        AbrtR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Mode fault. when 1, this bit indicates that a Mode fault error has occurred. This bit is cleared by reading this register, then writing the SPI0 control register."]
    #[inline(always)]
    pub fn modf(&self) -> ModfR {
        ModfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Read overrun. When 1, this bit indicates that a read overrun has occurred. This bit is cleared by reading this register."]
    #[inline(always)]
    pub fn rovr(&self) -> RovrR {
        RovrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write collision. When 1, this bit indicates that a write collision has occurred. This bit is cleared by reading this register, then accessing the SPI Data Register."]
    #[inline(always)]
    pub fn wcol(&self) -> WcolR {
        WcolR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SPI transfer complete flag. When 1, this bit indicates when a SPI data transfer is complete. When a master, this bit is set at the end of the last cycle of the transfer. When a slave, this bit is set on the last data sampling edge of the SCK. This bit is cleared by first reading this register, then accessing the SPI Data Register. Note: this is not the SPI interrupt flag. This flag is found in the SPINT register."]
    #[inline(always)]
    pub fn spif(&self) -> SpifR {
        SpifR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "SPI Status Register. This register shows the status of the SPI.\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SrSpec {
    const RESET_VALUE: u32 = 0;
}
