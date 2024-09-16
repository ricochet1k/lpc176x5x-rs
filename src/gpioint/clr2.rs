#[doc = "Register `CLR2` writer"]
pub type W = crate::W<Clr2Spec>;
#[doc = "Field `P2_0CI` writer - Clear GPIO port Interrupts for P2\\[0\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
pub type P2_0ciW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2_1CI` writer - Clear GPIO port Interrupts for P2\\[1\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
pub type P2_1ciW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2_2CI` writer - Clear GPIO port Interrupts for P2\\[2\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
pub type P2_2ciW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2_3CI` writer - Clear GPIO port Interrupts for P2\\[3\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
pub type P2_3ciW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2_4CI` writer - Clear GPIO port Interrupts for P2\\[4\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
pub type P2_4ciW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2_5CI` writer - Clear GPIO port Interrupts for P2\\[5\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
pub type P2_5ciW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2_6CI` writer - Clear GPIO port Interrupts for P2\\[6\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
pub type P2_6ciW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2_7CI` writer - Clear GPIO port Interrupts for P2\\[7\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
pub type P2_7ciW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2_8CI` writer - Clear GPIO port Interrupts for P2\\[8\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
pub type P2_8ciW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2_9CI` writer - Clear GPIO port Interrupts for P2\\[9\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
pub type P2_9ciW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2_10CI` writer - Clear GPIO port Interrupts for P2\\[10\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
pub type P2_10ciW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2_11CI` writer - Clear GPIO port Interrupts for P2\\[11\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
pub type P2_11ciW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2_12CI` writer - Clear GPIO port Interrupts for P2\\[12\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
pub type P2_12ciW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2_13CI` writer - Clear GPIO port Interrupts for P2\\[13\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
pub type P2_13ciW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear GPIO port Interrupts for P2\\[0\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    #[must_use]
    pub fn p2_0ci(&mut self) -> P2_0ciW<Clr2Spec> {
        P2_0ciW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear GPIO port Interrupts for P2\\[1\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    #[must_use]
    pub fn p2_1ci(&mut self) -> P2_1ciW<Clr2Spec> {
        P2_1ciW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear GPIO port Interrupts for P2\\[2\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    #[must_use]
    pub fn p2_2ci(&mut self) -> P2_2ciW<Clr2Spec> {
        P2_2ciW::new(self, 2)
    }
    #[doc = "Bit 3 - Clear GPIO port Interrupts for P2\\[3\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    #[must_use]
    pub fn p2_3ci(&mut self) -> P2_3ciW<Clr2Spec> {
        P2_3ciW::new(self, 3)
    }
    #[doc = "Bit 4 - Clear GPIO port Interrupts for P2\\[4\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    #[must_use]
    pub fn p2_4ci(&mut self) -> P2_4ciW<Clr2Spec> {
        P2_4ciW::new(self, 4)
    }
    #[doc = "Bit 5 - Clear GPIO port Interrupts for P2\\[5\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    #[must_use]
    pub fn p2_5ci(&mut self) -> P2_5ciW<Clr2Spec> {
        P2_5ciW::new(self, 5)
    }
    #[doc = "Bit 6 - Clear GPIO port Interrupts for P2\\[6\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    #[must_use]
    pub fn p2_6ci(&mut self) -> P2_6ciW<Clr2Spec> {
        P2_6ciW::new(self, 6)
    }
    #[doc = "Bit 7 - Clear GPIO port Interrupts for P2\\[7\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    #[must_use]
    pub fn p2_7ci(&mut self) -> P2_7ciW<Clr2Spec> {
        P2_7ciW::new(self, 7)
    }
    #[doc = "Bit 8 - Clear GPIO port Interrupts for P2\\[8\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    #[must_use]
    pub fn p2_8ci(&mut self) -> P2_8ciW<Clr2Spec> {
        P2_8ciW::new(self, 8)
    }
    #[doc = "Bit 9 - Clear GPIO port Interrupts for P2\\[9\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    #[must_use]
    pub fn p2_9ci(&mut self) -> P2_9ciW<Clr2Spec> {
        P2_9ciW::new(self, 9)
    }
    #[doc = "Bit 10 - Clear GPIO port Interrupts for P2\\[10\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    #[must_use]
    pub fn p2_10ci(&mut self) -> P2_10ciW<Clr2Spec> {
        P2_10ciW::new(self, 10)
    }
    #[doc = "Bit 11 - Clear GPIO port Interrupts for P2\\[11\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    #[must_use]
    pub fn p2_11ci(&mut self) -> P2_11ciW<Clr2Spec> {
        P2_11ciW::new(self, 11)
    }
    #[doc = "Bit 12 - Clear GPIO port Interrupts for P2\\[12\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    #[must_use]
    pub fn p2_12ci(&mut self) -> P2_12ciW<Clr2Spec> {
        P2_12ciW::new(self, 12)
    }
    #[doc = "Bit 13 - Clear GPIO port Interrupts for P2\\[13\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    #[must_use]
    pub fn p2_13ci(&mut self) -> P2_13ciW<Clr2Spec> {
        P2_13ciW::new(self, 13)
    }
}
#[doc = "GPIO Interrupt Clear.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clr2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Clr2Spec;
impl crate::RegisterSpec for Clr2Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`clr2::W`](W) writer structure"]
impl crate::Writable for Clr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLR2 to value 0"]
impl crate::Resettable for Clr2Spec {
    const RESET_VALUE: u32 = 0;
}
