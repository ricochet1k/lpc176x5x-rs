#[doc = "Register `INTENABLE` reader"]
pub type R = crate::R<IntenableSpec>;
#[doc = "Register `INTENABLE` writer"]
pub type W = crate::W<IntenableSpec>;
#[doc = "Field `RXOVERRUNINTEN` reader - Enable for interrupt trigger on receive buffer overrun or descriptor underrun situations."]
pub type RxoverrunintenR = crate::BitReader;
#[doc = "Field `RXOVERRUNINTEN` writer - Enable for interrupt trigger on receive buffer overrun or descriptor underrun situations."]
pub type RxoverrunintenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXERRORINTEN` reader - Enable for interrupt trigger on receive errors."]
pub type RxerrorintenR = crate::BitReader;
#[doc = "Field `RXERRORINTEN` writer - Enable for interrupt trigger on receive errors."]
pub type RxerrorintenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFINISHEDINTEN` reader - Enable for interrupt triggered when all receive descriptors have been processed i.e. on the transition to the situation where ProduceIndex == ConsumeIndex."]
pub type RxfinishedintenR = crate::BitReader;
#[doc = "Field `RXFINISHEDINTEN` writer - Enable for interrupt triggered when all receive descriptors have been processed i.e. on the transition to the situation where ProduceIndex == ConsumeIndex."]
pub type RxfinishedintenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXDONEINTEN` reader - Enable for interrupt triggered when a receive descriptor has been processed while the Interrupt bit in the Control field of the descriptor was set."]
pub type RxdoneintenR = crate::BitReader;
#[doc = "Field `RXDONEINTEN` writer - Enable for interrupt triggered when a receive descriptor has been processed while the Interrupt bit in the Control field of the descriptor was set."]
pub type RxdoneintenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUNDERRUNINTEN` reader - Enable for interrupt trigger on transmit buffer or descriptor underrun situations."]
pub type TxunderrunintenR = crate::BitReader;
#[doc = "Field `TXUNDERRUNINTEN` writer - Enable for interrupt trigger on transmit buffer or descriptor underrun situations."]
pub type TxunderrunintenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXERRORINTEN` reader - Enable for interrupt trigger on transmit errors."]
pub type TxerrorintenR = crate::BitReader;
#[doc = "Field `TXERRORINTEN` writer - Enable for interrupt trigger on transmit errors."]
pub type TxerrorintenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFINISHEDINTEN` reader - Enable for interrupt triggered when all transmit descriptors have been processed i.e. on the transition to the situation where ProduceIndex == ConsumeIndex."]
pub type TxfinishedintenR = crate::BitReader;
#[doc = "Field `TXFINISHEDINTEN` writer - Enable for interrupt triggered when all transmit descriptors have been processed i.e. on the transition to the situation where ProduceIndex == ConsumeIndex."]
pub type TxfinishedintenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDONEINTEN` reader - Enable for interrupt triggered when a descriptor has been transmitted while the Interrupt bit in the Control field of the descriptor was set."]
pub type TxdoneintenR = crate::BitReader;
#[doc = "Field `TXDONEINTEN` writer - Enable for interrupt triggered when a descriptor has been transmitted while the Interrupt bit in the Control field of the descriptor was set."]
pub type TxdoneintenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFTINTEN` reader - Enable for interrupt triggered by the SoftInt bit in the IntStatus register, caused by software writing a 1 to the SoftIntSet bit in the IntSet register."]
pub type SoftintenR = crate::BitReader;
#[doc = "Field `SOFTINTEN` writer - Enable for interrupt triggered by the SoftInt bit in the IntStatus register, caused by software writing a 1 to the SoftIntSet bit in the IntSet register."]
pub type SoftintenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUPINTEN` reader - Enable for interrupt triggered by a Wake-up event detected by the receive filter."]
pub type WakeupintenR = crate::BitReader;
#[doc = "Field `WAKEUPINTEN` writer - Enable for interrupt triggered by a Wake-up event detected by the receive filter."]
pub type WakeupintenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable for interrupt trigger on receive buffer overrun or descriptor underrun situations."]
    #[inline(always)]
    pub fn rxoverruninten(&self) -> RxoverrunintenR {
        RxoverrunintenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable for interrupt trigger on receive errors."]
    #[inline(always)]
    pub fn rxerrorinten(&self) -> RxerrorintenR {
        RxerrorintenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable for interrupt triggered when all receive descriptors have been processed i.e. on the transition to the situation where ProduceIndex == ConsumeIndex."]
    #[inline(always)]
    pub fn rxfinishedinten(&self) -> RxfinishedintenR {
        RxfinishedintenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable for interrupt triggered when a receive descriptor has been processed while the Interrupt bit in the Control field of the descriptor was set."]
    #[inline(always)]
    pub fn rxdoneinten(&self) -> RxdoneintenR {
        RxdoneintenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable for interrupt trigger on transmit buffer or descriptor underrun situations."]
    #[inline(always)]
    pub fn txunderruninten(&self) -> TxunderrunintenR {
        TxunderrunintenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable for interrupt trigger on transmit errors."]
    #[inline(always)]
    pub fn txerrorinten(&self) -> TxerrorintenR {
        TxerrorintenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable for interrupt triggered when all transmit descriptors have been processed i.e. on the transition to the situation where ProduceIndex == ConsumeIndex."]
    #[inline(always)]
    pub fn txfinishedinten(&self) -> TxfinishedintenR {
        TxfinishedintenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable for interrupt triggered when a descriptor has been transmitted while the Interrupt bit in the Control field of the descriptor was set."]
    #[inline(always)]
    pub fn txdoneinten(&self) -> TxdoneintenR {
        TxdoneintenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable for interrupt triggered by the SoftInt bit in the IntStatus register, caused by software writing a 1 to the SoftIntSet bit in the IntSet register."]
    #[inline(always)]
    pub fn softinten(&self) -> SoftintenR {
        SoftintenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable for interrupt triggered by a Wake-up event detected by the receive filter."]
    #[inline(always)]
    pub fn wakeupinten(&self) -> WakeupintenR {
        WakeupintenR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable for interrupt trigger on receive buffer overrun or descriptor underrun situations."]
    #[inline(always)]
    #[must_use]
    pub fn rxoverruninten(&mut self) -> RxoverrunintenW<IntenableSpec> {
        RxoverrunintenW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable for interrupt trigger on receive errors."]
    #[inline(always)]
    #[must_use]
    pub fn rxerrorinten(&mut self) -> RxerrorintenW<IntenableSpec> {
        RxerrorintenW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable for interrupt triggered when all receive descriptors have been processed i.e. on the transition to the situation where ProduceIndex == ConsumeIndex."]
    #[inline(always)]
    #[must_use]
    pub fn rxfinishedinten(&mut self) -> RxfinishedintenW<IntenableSpec> {
        RxfinishedintenW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable for interrupt triggered when a receive descriptor has been processed while the Interrupt bit in the Control field of the descriptor was set."]
    #[inline(always)]
    #[must_use]
    pub fn rxdoneinten(&mut self) -> RxdoneintenW<IntenableSpec> {
        RxdoneintenW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable for interrupt trigger on transmit buffer or descriptor underrun situations."]
    #[inline(always)]
    #[must_use]
    pub fn txunderruninten(&mut self) -> TxunderrunintenW<IntenableSpec> {
        TxunderrunintenW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable for interrupt trigger on transmit errors."]
    #[inline(always)]
    #[must_use]
    pub fn txerrorinten(&mut self) -> TxerrorintenW<IntenableSpec> {
        TxerrorintenW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable for interrupt triggered when all transmit descriptors have been processed i.e. on the transition to the situation where ProduceIndex == ConsumeIndex."]
    #[inline(always)]
    #[must_use]
    pub fn txfinishedinten(&mut self) -> TxfinishedintenW<IntenableSpec> {
        TxfinishedintenW::new(self, 6)
    }
    #[doc = "Bit 7 - Enable for interrupt triggered when a descriptor has been transmitted while the Interrupt bit in the Control field of the descriptor was set."]
    #[inline(always)]
    #[must_use]
    pub fn txdoneinten(&mut self) -> TxdoneintenW<IntenableSpec> {
        TxdoneintenW::new(self, 7)
    }
    #[doc = "Bit 12 - Enable for interrupt triggered by the SoftInt bit in the IntStatus register, caused by software writing a 1 to the SoftIntSet bit in the IntSet register."]
    #[inline(always)]
    #[must_use]
    pub fn softinten(&mut self) -> SoftintenW<IntenableSpec> {
        SoftintenW::new(self, 12)
    }
    #[doc = "Bit 13 - Enable for interrupt triggered by a Wake-up event detected by the receive filter."]
    #[inline(always)]
    #[must_use]
    pub fn wakeupinten(&mut self) -> WakeupintenW<IntenableSpec> {
        WakeupintenW::new(self, 13)
    }
}
#[doc = "Interrupt enable register.\n\nYou can [`read`](crate::Reg::read) this register and get [`intenable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenableSpec;
impl crate::RegisterSpec for IntenableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenable::R`](R) reader structure"]
impl crate::Readable for IntenableSpec {}
#[doc = "`write(|w| ..)` method takes [`intenable::W`](W) writer structure"]
impl crate::Writable for IntenableSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTENABLE to value 0"]
impl crate::Resettable for IntenableSpec {
    const RESET_VALUE: u32 = 0;
}
