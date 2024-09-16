#[doc = "Register `DR` reader"]
pub type R = crate::R<DrSpec>;
#[doc = "Register `DR` writer"]
pub type W = crate::W<DrSpec>;
#[doc = "Field `DATALOW` reader - SPI Bi-directional data port."]
pub type DatalowR = crate::FieldReader;
#[doc = "Field `DATALOW` writer - SPI Bi-directional data port."]
pub type DatalowW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATAHIGH` reader - If bit 2 of the SPCR is 1 and bits 11:8 are other than 1000, some or all of these bits contain the additional transmit and receive bits. When less than 16 bits are selected, the more significant among these bits read as zeroes."]
pub type DatahighR = crate::FieldReader;
#[doc = "Field `DATAHIGH` writer - If bit 2 of the SPCR is 1 and bits 11:8 are other than 1000, some or all of these bits contain the additional transmit and receive bits. When less than 16 bits are selected, the more significant among these bits read as zeroes."]
pub type DatahighW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - SPI Bi-directional data port."]
    #[inline(always)]
    pub fn datalow(&self) -> DatalowR {
        DatalowR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - If bit 2 of the SPCR is 1 and bits 11:8 are other than 1000, some or all of these bits contain the additional transmit and receive bits. When less than 16 bits are selected, the more significant among these bits read as zeroes."]
    #[inline(always)]
    pub fn datahigh(&self) -> DatahighR {
        DatahighR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SPI Bi-directional data port."]
    #[inline(always)]
    #[must_use]
    pub fn datalow(&mut self) -> DatalowW<DrSpec> {
        DatalowW::new(self, 0)
    }
    #[doc = "Bits 8:15 - If bit 2 of the SPCR is 1 and bits 11:8 are other than 1000, some or all of these bits contain the additional transmit and receive bits. When less than 16 bits are selected, the more significant among these bits read as zeroes."]
    #[inline(always)]
    #[must_use]
    pub fn datahigh(&mut self) -> DatahighW<DrSpec> {
        DatahighW::new(self, 8)
    }
}
#[doc = "SPI Data Register. This bi-directional register provides the transmit and receive data for the SPI. Transmit data is provided to the SPI0 by writing to this register. Data received by the SPI0 can be read from this register.\n\nYou can [`read`](crate::Reg::read) this register and get [`dr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">The register is <b>modified</b> in some way after a read operation.</div>"]
pub struct DrSpec;
impl crate::RegisterSpec for DrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr::R`](R) reader structure"]
impl crate::Readable for DrSpec {}
#[doc = "`write(|w| ..)` method takes [`dr::W`](W) writer structure"]
impl crate::Writable for DrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DR to value 0"]
impl crate::Resettable for DrSpec {
    const RESET_VALUE: u32 = 0;
}
