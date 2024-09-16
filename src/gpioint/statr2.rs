#[doc = "Register `STATR2` reader"]
pub type R = crate::R<Statr2Spec>;
#[doc = "Field `P2_0REI` reader - Status of Rising Edge Interrupt for P2\\[0\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub type P2_0reiR = crate::BitReader;
#[doc = "Field `P2_1REI` reader - Status of Rising Edge Interrupt for P2\\[1\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub type P2_1reiR = crate::BitReader;
#[doc = "Field `P2_2REI` reader - Status of Rising Edge Interrupt for P2\\[2\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub type P2_2reiR = crate::BitReader;
#[doc = "Field `P2_3REI` reader - Status of Rising Edge Interrupt for P2\\[3\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub type P2_3reiR = crate::BitReader;
#[doc = "Field `P2_4REI` reader - Status of Rising Edge Interrupt for P2\\[4\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub type P2_4reiR = crate::BitReader;
#[doc = "Field `P2_5REI` reader - Status of Rising Edge Interrupt for P2\\[5\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub type P2_5reiR = crate::BitReader;
#[doc = "Field `P2_6REI` reader - Status of Rising Edge Interrupt for P2\\[6\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub type P2_6reiR = crate::BitReader;
#[doc = "Field `P2_7REI` reader - Status of Rising Edge Interrupt for P2\\[7\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub type P2_7reiR = crate::BitReader;
#[doc = "Field `P2_8REI` reader - Status of Rising Edge Interrupt for P2\\[8\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub type P2_8reiR = crate::BitReader;
#[doc = "Field `P2_9REI` reader - Status of Rising Edge Interrupt for P2\\[9\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub type P2_9reiR = crate::BitReader;
#[doc = "Field `P2_10REI` reader - Status of Rising Edge Interrupt for P2\\[10\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub type P2_10reiR = crate::BitReader;
#[doc = "Field `P2_11REI` reader - Status of Rising Edge Interrupt for P2\\[11\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub type P2_11reiR = crate::BitReader;
#[doc = "Field `P2_12REI` reader - Status of Rising Edge Interrupt for P2\\[12\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub type P2_12reiR = crate::BitReader;
#[doc = "Field `P2_13REI` reader - Status of Rising Edge Interrupt for P2\\[13\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub type P2_13reiR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Status of Rising Edge Interrupt for P2\\[0\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_0rei(&self) -> P2_0reiR {
        P2_0reiR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Status of Rising Edge Interrupt for P2\\[1\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_1rei(&self) -> P2_1reiR {
        P2_1reiR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Status of Rising Edge Interrupt for P2\\[2\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_2rei(&self) -> P2_2reiR {
        P2_2reiR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Status of Rising Edge Interrupt for P2\\[3\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_3rei(&self) -> P2_3reiR {
        P2_3reiR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Status of Rising Edge Interrupt for P2\\[4\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_4rei(&self) -> P2_4reiR {
        P2_4reiR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Status of Rising Edge Interrupt for P2\\[5\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_5rei(&self) -> P2_5reiR {
        P2_5reiR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Status of Rising Edge Interrupt for P2\\[6\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_6rei(&self) -> P2_6reiR {
        P2_6reiR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Status of Rising Edge Interrupt for P2\\[7\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_7rei(&self) -> P2_7reiR {
        P2_7reiR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Status of Rising Edge Interrupt for P2\\[8\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_8rei(&self) -> P2_8reiR {
        P2_8reiR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Status of Rising Edge Interrupt for P2\\[9\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_9rei(&self) -> P2_9reiR {
        P2_9reiR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Status of Rising Edge Interrupt for P2\\[10\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_10rei(&self) -> P2_10reiR {
        P2_10reiR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Status of Rising Edge Interrupt for P2\\[11\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_11rei(&self) -> P2_11reiR {
        P2_11reiR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Status of Rising Edge Interrupt for P2\\[12\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_12rei(&self) -> P2_12reiR {
        P2_12reiR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Status of Rising Edge Interrupt for P2\\[13\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_13rei(&self) -> P2_13reiR {
        P2_13reiR::new(((self.bits >> 13) & 1) != 0)
    }
}
#[doc = "GPIO Interrupt Status for Rising edge for Port 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`statr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Statr2Spec;
impl crate::RegisterSpec for Statr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`statr2::R`](R) reader structure"]
impl crate::Readable for Statr2Spec {}
#[doc = "`reset()` method sets STATR2 to value 0"]
impl crate::Resettable for Statr2Spec {
    const RESET_VALUE: u32 = 0;
}
