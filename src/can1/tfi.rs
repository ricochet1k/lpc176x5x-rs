#[doc = "Register `TFI%s` reader"]
pub type R = crate::R<TfiSpec>;
#[doc = "Register `TFI%s` writer"]
pub type W = crate::W<TfiSpec>;
#[doc = "Field `PRIO` reader - If the TPM (Transmit Priority Mode) bit in the CANxMOD register is set to 1, enabled Tx Buffers contend for the right to send their messages based on this field. The buffer with the lowest TX Priority value wins the prioritization and is sent first."]
pub type PrioR = crate::FieldReader;
#[doc = "Field `PRIO` writer - If the TPM (Transmit Priority Mode) bit in the CANxMOD register is set to 1, enabled Tx Buffers contend for the right to send their messages based on this field. The buffer with the lowest TX Priority value wins the prioritization and is sent first."]
pub type PrioW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DLC` reader - Data Length Code. This value is sent in the DLC field of the next transmit message. In addition, if RTR = 0, this value controls the number of Data bytes sent in the next transmit message, from the CANxTDA and CANxTDB registers: 0000-0111 = 0-7 bytes 1xxx = 8 bytes"]
pub type DlcR = crate::FieldReader;
#[doc = "Field `DLC` writer - Data Length Code. This value is sent in the DLC field of the next transmit message. In addition, if RTR = 0, this value controls the number of Data bytes sent in the next transmit message, from the CANxTDA and CANxTDB registers: 0000-0111 = 0-7 bytes 1xxx = 8 bytes"]
pub type DlcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RTR` reader - This value is sent in the RTR bit of the next transmit message. If this bit is 0, the number of data bytes called out by the DLC field are sent from the CANxTDA and CANxTDB registers. If this bit is 1, a Remote Frame is sent, containing a request for that number of bytes."]
pub type RtrR = crate::BitReader;
#[doc = "Field `RTR` writer - This value is sent in the RTR bit of the next transmit message. If this bit is 0, the number of data bytes called out by the DLC field are sent from the CANxTDA and CANxTDB registers. If this bit is 1, a Remote Frame is sent, containing a request for that number of bytes."]
pub type RtrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FF` reader - If this bit is 0, the next transmit message will be sent with an 11-bit Identifier (standard frame format), while if it's 1, the message will be sent with a 29-bit Identifier (extended frame format)."]
pub type FfR = crate::BitReader;
#[doc = "Field `FF` writer - If this bit is 0, the next transmit message will be sent with an 11-bit Identifier (standard frame format), while if it's 1, the message will be sent with a 29-bit Identifier (extended frame format)."]
pub type FfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - If the TPM (Transmit Priority Mode) bit in the CANxMOD register is set to 1, enabled Tx Buffers contend for the right to send their messages based on this field. The buffer with the lowest TX Priority value wins the prioritization and is sent first."]
    #[inline(always)]
    pub fn prio(&self) -> PrioR {
        PrioR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Data Length Code. This value is sent in the DLC field of the next transmit message. In addition, if RTR = 0, this value controls the number of Data bytes sent in the next transmit message, from the CANxTDA and CANxTDB registers: 0000-0111 = 0-7 bytes 1xxx = 8 bytes"]
    #[inline(always)]
    pub fn dlc(&self) -> DlcR {
        DlcR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - This value is sent in the RTR bit of the next transmit message. If this bit is 0, the number of data bytes called out by the DLC field are sent from the CANxTDA and CANxTDB registers. If this bit is 1, a Remote Frame is sent, containing a request for that number of bytes."]
    #[inline(always)]
    pub fn rtr(&self) -> RtrR {
        RtrR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - If this bit is 0, the next transmit message will be sent with an 11-bit Identifier (standard frame format), while if it's 1, the message will be sent with a 29-bit Identifier (extended frame format)."]
    #[inline(always)]
    pub fn ff(&self) -> FfR {
        FfR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - If the TPM (Transmit Priority Mode) bit in the CANxMOD register is set to 1, enabled Tx Buffers contend for the right to send their messages based on this field. The buffer with the lowest TX Priority value wins the prioritization and is sent first."]
    #[inline(always)]
    #[must_use]
    pub fn prio(&mut self) -> PrioW<TfiSpec> {
        PrioW::new(self, 0)
    }
    #[doc = "Bits 16:19 - Data Length Code. This value is sent in the DLC field of the next transmit message. In addition, if RTR = 0, this value controls the number of Data bytes sent in the next transmit message, from the CANxTDA and CANxTDB registers: 0000-0111 = 0-7 bytes 1xxx = 8 bytes"]
    #[inline(always)]
    #[must_use]
    pub fn dlc(&mut self) -> DlcW<TfiSpec> {
        DlcW::new(self, 16)
    }
    #[doc = "Bit 30 - This value is sent in the RTR bit of the next transmit message. If this bit is 0, the number of data bytes called out by the DLC field are sent from the CANxTDA and CANxTDB registers. If this bit is 1, a Remote Frame is sent, containing a request for that number of bytes."]
    #[inline(always)]
    #[must_use]
    pub fn rtr(&mut self) -> RtrW<TfiSpec> {
        RtrW::new(self, 30)
    }
    #[doc = "Bit 31 - If this bit is 0, the next transmit message will be sent with an 11-bit Identifier (standard frame format), while if it's 1, the message will be sent with a 29-bit Identifier (extended frame format)."]
    #[inline(always)]
    #[must_use]
    pub fn ff(&mut self) -> FfW<TfiSpec> {
        FfW::new(self, 31)
    }
}
#[doc = "Transmit frame info (Tx Buffer )\n\nYou can [`read`](crate::Reg::read) this register and get [`tfi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tfi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TfiSpec;
impl crate::RegisterSpec for TfiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tfi::R`](R) reader structure"]
impl crate::Readable for TfiSpec {}
#[doc = "`write(|w| ..)` method takes [`tfi::W`](W) writer structure"]
impl crate::Writable for TfiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TFI%s to value 0"]
impl crate::Resettable for TfiSpec {
    const RESET_VALUE: u32 = 0;
}
