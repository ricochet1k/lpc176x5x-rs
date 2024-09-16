#[doc = "Register `INTCLEAR` writer"]
pub type W = crate::W<IntclearSpec>;
#[doc = "Field `RXOVERRUNINTCLR` writer - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
pub type RxoverrunintclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXERRORINTCLR` writer - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
pub type RxerrorintclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFINISHEDINTCLR` writer - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
pub type RxfinishedintclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXDONEINTCLR` writer - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
pub type RxdoneintclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUNDERRUNINTCLR` writer - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
pub type TxunderrunintclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXERRORINTCLR` writer - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
pub type TxerrorintclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFINISHEDINTCLR` writer - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
pub type TxfinishedintclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDONEINTCLR` writer - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
pub type TxdoneintclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFTINTCLR` writer - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
pub type SoftintclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUPINTCLR` writer - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
pub type WakeupintclrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    #[must_use]
    pub fn rxoverrunintclr(&mut self) -> RxoverrunintclrW<IntclearSpec> {
        RxoverrunintclrW::new(self, 0)
    }
    #[doc = "Bit 1 - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    #[must_use]
    pub fn rxerrorintclr(&mut self) -> RxerrorintclrW<IntclearSpec> {
        RxerrorintclrW::new(self, 1)
    }
    #[doc = "Bit 2 - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    #[must_use]
    pub fn rxfinishedintclr(&mut self) -> RxfinishedintclrW<IntclearSpec> {
        RxfinishedintclrW::new(self, 2)
    }
    #[doc = "Bit 3 - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    #[must_use]
    pub fn rxdoneintclr(&mut self) -> RxdoneintclrW<IntclearSpec> {
        RxdoneintclrW::new(self, 3)
    }
    #[doc = "Bit 4 - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    #[must_use]
    pub fn txunderrunintclr(&mut self) -> TxunderrunintclrW<IntclearSpec> {
        TxunderrunintclrW::new(self, 4)
    }
    #[doc = "Bit 5 - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    #[must_use]
    pub fn txerrorintclr(&mut self) -> TxerrorintclrW<IntclearSpec> {
        TxerrorintclrW::new(self, 5)
    }
    #[doc = "Bit 6 - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    #[must_use]
    pub fn txfinishedintclr(&mut self) -> TxfinishedintclrW<IntclearSpec> {
        TxfinishedintclrW::new(self, 6)
    }
    #[doc = "Bit 7 - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    #[must_use]
    pub fn txdoneintclr(&mut self) -> TxdoneintclrW<IntclearSpec> {
        TxdoneintclrW::new(self, 7)
    }
    #[doc = "Bit 12 - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    #[must_use]
    pub fn softintclr(&mut self) -> SoftintclrW<IntclearSpec> {
        SoftintclrW::new(self, 12)
    }
    #[doc = "Bit 13 - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    #[must_use]
    pub fn wakeupintclr(&mut self) -> WakeupintclrW<IntclearSpec> {
        WakeupintclrW::new(self, 13)
    }
}
#[doc = "Interrupt clear register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intclear::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntclearSpec;
impl crate::RegisterSpec for IntclearSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`intclear::W`](W) writer structure"]
impl crate::Writable for IntclearSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTCLEAR to value 0"]
impl crate::Resettable for IntclearSpec {
    const RESET_VALUE: u32 = 0;
}
