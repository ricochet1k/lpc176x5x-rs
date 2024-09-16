#[doc = "Register `DR[%s]` reader"]
pub type R = crate::R<DrSpec>;
#[doc = "Field `RESULT` reader - When DONE is 1, this field contains a binary fraction representing the voltage on the AD0\\[n\\]
pin, as it falls within the range of VREFP to V SS. Zero in the field indicates that the voltage on the input pin was less than, equal to, or close to that on VSS, while 0xFFF indicates that the voltage on the input was close to, equal to, or greater than that on VREFP."]
pub type ResultR = crate::FieldReader<u16>;
#[doc = "Field `OVERRUN` reader - This bit is 1 in burst mode if the results of one or more conversions was (were) lost and overwritten before the conversion that produced the result in the RESULT bits.This bit is cleared by reading this register."]
pub type OverrunR = crate::BitReader;
#[doc = "Field `DONE` reader - This bit is set to 1 when an A/D conversion completes. It is cleared when this register is read."]
pub type DoneR = crate::BitReader;
impl R {
    #[doc = "Bits 4:15 - When DONE is 1, this field contains a binary fraction representing the voltage on the AD0\\[n\\]
pin, as it falls within the range of VREFP to V SS. Zero in the field indicates that the voltage on the input pin was less than, equal to, or close to that on VSS, while 0xFFF indicates that the voltage on the input was close to, equal to, or greater than that on VREFP."]
    #[inline(always)]
    pub fn result(&self) -> ResultR {
        ResultR::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bit 30 - This bit is 1 in burst mode if the results of one or more conversions was (were) lost and overwritten before the conversion that produced the result in the RESULT bits.This bit is cleared by reading this register."]
    #[inline(always)]
    pub fn overrun(&self) -> OverrunR {
        OverrunR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - This bit is set to 1 when an A/D conversion completes. It is cleared when this register is read."]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "A/D Channel 0 Data Register. This register contains the result of the most recent conversion completed on channel 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`dr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DrSpec;
impl crate::RegisterSpec for DrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr::R`](R) reader structure"]
impl crate::Readable for DrSpec {}
#[doc = "`reset()` method sets DR[%s]
to value 0"]
impl crate::Resettable for DrSpec {
    const RESET_VALUE: u32 = 0;
}
