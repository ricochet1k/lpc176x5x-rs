#[doc = "Register `STATF2` reader"]
pub type R = crate::R<Statf2Spec>;
#[doc = "Field `P2_0FEI` reader - Status of Falling Edge Interrupt for P2\\[0\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub type P2_0feiR = crate::BitReader;
#[doc = "Field `P2_1FEI` reader - Status of Falling Edge Interrupt for P2\\[1\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub type P2_1feiR = crate::BitReader;
#[doc = "Field `P2_2FEI` reader - Status of Falling Edge Interrupt for P2\\[2\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub type P2_2feiR = crate::BitReader;
#[doc = "Field `P2_3FEI` reader - Status of Falling Edge Interrupt for P2\\[3\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub type P2_3feiR = crate::BitReader;
#[doc = "Field `P2_4FEI` reader - Status of Falling Edge Interrupt for P2\\[4\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub type P2_4feiR = crate::BitReader;
#[doc = "Field `P2_5FEI` reader - Status of Falling Edge Interrupt for P2\\[5\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub type P2_5feiR = crate::BitReader;
#[doc = "Field `P2_6FEI` reader - Status of Falling Edge Interrupt for P2\\[6\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub type P2_6feiR = crate::BitReader;
#[doc = "Field `P2_7FEI` reader - Status of Falling Edge Interrupt for P2\\[7\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub type P2_7feiR = crate::BitReader;
#[doc = "Field `P2_8FEI` reader - Status of Falling Edge Interrupt for P2\\[8\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub type P2_8feiR = crate::BitReader;
#[doc = "Field `P2_9FEI` reader - Status of Falling Edge Interrupt for P2\\[9\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub type P2_9feiR = crate::BitReader;
#[doc = "Field `P2_10FEI` reader - Status of Falling Edge Interrupt for P2\\[10\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub type P2_10feiR = crate::BitReader;
#[doc = "Field `P2_11FEI` reader - Status of Falling Edge Interrupt for P2\\[11\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub type P2_11feiR = crate::BitReader;
#[doc = "Field `P2_12FEI` reader - Status of Falling Edge Interrupt for P2\\[12\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub type P2_12feiR = crate::BitReader;
#[doc = "Field `P2_13FEI` reader - Status of Falling Edge Interrupt for P2\\[13\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub type P2_13feiR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Status of Falling Edge Interrupt for P2\\[0\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_0fei(&self) -> P2_0feiR {
        P2_0feiR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Status of Falling Edge Interrupt for P2\\[1\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_1fei(&self) -> P2_1feiR {
        P2_1feiR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Status of Falling Edge Interrupt for P2\\[2\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_2fei(&self) -> P2_2feiR {
        P2_2feiR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Status of Falling Edge Interrupt for P2\\[3\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_3fei(&self) -> P2_3feiR {
        P2_3feiR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Status of Falling Edge Interrupt for P2\\[4\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_4fei(&self) -> P2_4feiR {
        P2_4feiR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Status of Falling Edge Interrupt for P2\\[5\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_5fei(&self) -> P2_5feiR {
        P2_5feiR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Status of Falling Edge Interrupt for P2\\[6\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_6fei(&self) -> P2_6feiR {
        P2_6feiR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Status of Falling Edge Interrupt for P2\\[7\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_7fei(&self) -> P2_7feiR {
        P2_7feiR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Status of Falling Edge Interrupt for P2\\[8\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_8fei(&self) -> P2_8feiR {
        P2_8feiR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Status of Falling Edge Interrupt for P2\\[9\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_9fei(&self) -> P2_9feiR {
        P2_9feiR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Status of Falling Edge Interrupt for P2\\[10\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_10fei(&self) -> P2_10feiR {
        P2_10feiR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Status of Falling Edge Interrupt for P2\\[11\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_11fei(&self) -> P2_11feiR {
        P2_11feiR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Status of Falling Edge Interrupt for P2\\[12\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_12fei(&self) -> P2_12feiR {
        P2_12feiR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Status of Falling Edge Interrupt for P2\\[13\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_13fei(&self) -> P2_13feiR {
        P2_13feiR::new(((self.bits >> 13) & 1) != 0)
    }
}
#[doc = "GPIO Interrupt Status for Falling edge for Port 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`statf2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Statf2Spec;
impl crate::RegisterSpec for Statf2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`statf2::R`](R) reader structure"]
impl crate::Readable for Statf2Spec {}
#[doc = "`reset()` method sets STATF2 to value 0"]
impl crate::Resettable for Statf2Spec {
    const RESET_VALUE: u32 = 0;
}
