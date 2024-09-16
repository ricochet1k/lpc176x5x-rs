#[doc = "Register `INTSET` writer"]
pub type W = crate::W<IntsetSpec>;
#[doc = "Field `RXOVERRUNINTSET` writer - Writing a 1 to one sets the corresponding status bit in interrupt status register IntStatus."]
pub type RxoverrunintsetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXERRORINTSET` writer - Writing a 1 to one sets the corresponding status bit in interrupt status register IntStatus."]
pub type RxerrorintsetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFINISHEDINTSET` writer - Writing a 1 to one sets the corresponding status bit in interrupt status register IntStatus."]
pub type RxfinishedintsetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXDONEINTSET` writer - Writing a 1 to one sets the corresponding status bit in interrupt status register IntStatus."]
pub type RxdoneintsetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUNDERRUNINTSET` writer - Writing a 1 to one sets the corresponding status bit in interrupt status register IntStatus."]
pub type TxunderrunintsetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXERRORINTSET` writer - Writing a 1 to one sets the corresponding status bit in interrupt status register IntStatus."]
pub type TxerrorintsetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFINISHEDINTSET` writer - Writing a 1 to one sets the corresponding status bit in interrupt status register IntStatus."]
pub type TxfinishedintsetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDONEINTSET` writer - Writing a 1 to one sets the corresponding status bit in interrupt status register IntStatus."]
pub type TxdoneintsetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFTINTSET` writer - Writing a 1 to one sets the corresponding status bit in interrupt status register IntStatus."]
pub type SoftintsetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUPINTSET` writer - Writing a 1 to one sets the corresponding status bit in interrupt status register IntStatus."]
pub type WakeupintsetW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Writing a 1 to one sets the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    #[must_use]
    pub fn rxoverrunintset(&mut self) -> RxoverrunintsetW<IntsetSpec> {
        RxoverrunintsetW::new(self, 0)
    }
    #[doc = "Bit 1 - Writing a 1 to one sets the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    #[must_use]
    pub fn rxerrorintset(&mut self) -> RxerrorintsetW<IntsetSpec> {
        RxerrorintsetW::new(self, 1)
    }
    #[doc = "Bit 2 - Writing a 1 to one sets the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    #[must_use]
    pub fn rxfinishedintset(&mut self) -> RxfinishedintsetW<IntsetSpec> {
        RxfinishedintsetW::new(self, 2)
    }
    #[doc = "Bit 3 - Writing a 1 to one sets the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    #[must_use]
    pub fn rxdoneintset(&mut self) -> RxdoneintsetW<IntsetSpec> {
        RxdoneintsetW::new(self, 3)
    }
    #[doc = "Bit 4 - Writing a 1 to one sets the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    #[must_use]
    pub fn txunderrunintset(&mut self) -> TxunderrunintsetW<IntsetSpec> {
        TxunderrunintsetW::new(self, 4)
    }
    #[doc = "Bit 5 - Writing a 1 to one sets the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    #[must_use]
    pub fn txerrorintset(&mut self) -> TxerrorintsetW<IntsetSpec> {
        TxerrorintsetW::new(self, 5)
    }
    #[doc = "Bit 6 - Writing a 1 to one sets the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    #[must_use]
    pub fn txfinishedintset(&mut self) -> TxfinishedintsetW<IntsetSpec> {
        TxfinishedintsetW::new(self, 6)
    }
    #[doc = "Bit 7 - Writing a 1 to one sets the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    #[must_use]
    pub fn txdoneintset(&mut self) -> TxdoneintsetW<IntsetSpec> {
        TxdoneintsetW::new(self, 7)
    }
    #[doc = "Bit 12 - Writing a 1 to one sets the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    #[must_use]
    pub fn softintset(&mut self) -> SoftintsetW<IntsetSpec> {
        SoftintsetW::new(self, 12)
    }
    #[doc = "Bit 13 - Writing a 1 to one sets the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    #[must_use]
    pub fn wakeupintset(&mut self) -> WakeupintsetW<IntsetSpec> {
        WakeupintsetW::new(self, 13)
    }
}
#[doc = "Interrupt set register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntsetSpec;
impl crate::RegisterSpec for IntsetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`intset::W`](W) writer structure"]
impl crate::Writable for IntsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTSET to value 0"]
impl crate::Resettable for IntsetSpec {
    const RESET_VALUE: u32 = 0;
}
