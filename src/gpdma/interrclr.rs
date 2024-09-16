#[doc = "Register `INTERRCLR` writer"]
pub type W = crate::W<InterrclrSpec>;
#[doc = "Field `INTERRCLR0` writer - Writing a 1 clears the error interrupt request (IntErrStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel error interrupt."]
pub type Interrclr0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERRCLR1` writer - Writing a 1 clears the error interrupt request (IntErrStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel error interrupt."]
pub type Interrclr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERRCLR2` writer - Writing a 1 clears the error interrupt request (IntErrStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel error interrupt."]
pub type Interrclr2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERRCLR3` writer - Writing a 1 clears the error interrupt request (IntErrStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel error interrupt."]
pub type Interrclr3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERRCLR4` writer - Writing a 1 clears the error interrupt request (IntErrStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel error interrupt."]
pub type Interrclr4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERRCLR5` writer - Writing a 1 clears the error interrupt request (IntErrStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel error interrupt."]
pub type Interrclr5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERRCLR6` writer - Writing a 1 clears the error interrupt request (IntErrStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel error interrupt."]
pub type Interrclr6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERRCLR7` writer - Writing a 1 clears the error interrupt request (IntErrStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel error interrupt."]
pub type Interrclr7W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Writing a 1 clears the error interrupt request (IntErrStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel error interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn interrclr0(&mut self) -> Interrclr0W<InterrclrSpec> {
        Interrclr0W::new(self, 0)
    }
    #[doc = "Bit 1 - Writing a 1 clears the error interrupt request (IntErrStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel error interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn interrclr1(&mut self) -> Interrclr1W<InterrclrSpec> {
        Interrclr1W::new(self, 1)
    }
    #[doc = "Bit 2 - Writing a 1 clears the error interrupt request (IntErrStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel error interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn interrclr2(&mut self) -> Interrclr2W<InterrclrSpec> {
        Interrclr2W::new(self, 2)
    }
    #[doc = "Bit 3 - Writing a 1 clears the error interrupt request (IntErrStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel error interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn interrclr3(&mut self) -> Interrclr3W<InterrclrSpec> {
        Interrclr3W::new(self, 3)
    }
    #[doc = "Bit 4 - Writing a 1 clears the error interrupt request (IntErrStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel error interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn interrclr4(&mut self) -> Interrclr4W<InterrclrSpec> {
        Interrclr4W::new(self, 4)
    }
    #[doc = "Bit 5 - Writing a 1 clears the error interrupt request (IntErrStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel error interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn interrclr5(&mut self) -> Interrclr5W<InterrclrSpec> {
        Interrclr5W::new(self, 5)
    }
    #[doc = "Bit 6 - Writing a 1 clears the error interrupt request (IntErrStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel error interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn interrclr6(&mut self) -> Interrclr6W<InterrclrSpec> {
        Interrclr6W::new(self, 6)
    }
    #[doc = "Bit 7 - Writing a 1 clears the error interrupt request (IntErrStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel error interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn interrclr7(&mut self) -> Interrclr7W<InterrclrSpec> {
        Interrclr7W::new(self, 7)
    }
}
#[doc = "DMA Interrupt Error Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InterrclrSpec;
impl crate::RegisterSpec for InterrclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`interrclr::W`](W) writer structure"]
impl crate::Writable for InterrclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTERRCLR to value 0"]
impl crate::Resettable for InterrclrSpec {
    const RESET_VALUE: u32 = 0;
}
