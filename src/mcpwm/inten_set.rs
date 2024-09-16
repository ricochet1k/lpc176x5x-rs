#[doc = "Register `INTEN_SET` writer"]
pub type W = crate::W<IntenSetSpec>;
#[doc = "Field `ILIM0_SET` writer - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
pub type Ilim0SetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMAT0_SET` writer - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
pub type Imat0SetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICAP0_SET` writer - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
pub type Icap0SetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ILIM1_SET` writer - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
pub type Ilim1SetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMAT1_SET` writer - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
pub type Imat1SetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICAP1_SET` writer - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
pub type Icap1SetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ILIM2_SET` writer - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
pub type Ilim2SetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMAT2_SET` writer - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
pub type Imat2SetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICAP2_SET` writer - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
pub type Icap2SetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABORT_SET` writer - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
pub type AbortSetW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ilim0_set(&mut self) -> Ilim0SetW<IntenSetSpec> {
        Ilim0SetW::new(self, 0)
    }
    #[doc = "Bit 1 - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn imat0_set(&mut self) -> Imat0SetW<IntenSetSpec> {
        Imat0SetW::new(self, 1)
    }
    #[doc = "Bit 2 - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn icap0_set(&mut self) -> Icap0SetW<IntenSetSpec> {
        Icap0SetW::new(self, 2)
    }
    #[doc = "Bit 4 - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ilim1_set(&mut self) -> Ilim1SetW<IntenSetSpec> {
        Ilim1SetW::new(self, 4)
    }
    #[doc = "Bit 5 - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn imat1_set(&mut self) -> Imat1SetW<IntenSetSpec> {
        Imat1SetW::new(self, 5)
    }
    #[doc = "Bit 6 - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn icap1_set(&mut self) -> Icap1SetW<IntenSetSpec> {
        Icap1SetW::new(self, 6)
    }
    #[doc = "Bit 9 - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ilim2_set(&mut self) -> Ilim2SetW<IntenSetSpec> {
        Ilim2SetW::new(self, 9)
    }
    #[doc = "Bit 10 - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn imat2_set(&mut self) -> Imat2SetW<IntenSetSpec> {
        Imat2SetW::new(self, 10)
    }
    #[doc = "Bit 11 - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn icap2_set(&mut self) -> Icap2SetW<IntenSetSpec> {
        Icap2SetW::new(self, 11)
    }
    #[doc = "Bit 15 - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn abort_set(&mut self) -> AbortSetW<IntenSetSpec> {
        AbortSetW::new(self, 15)
    }
}
#[doc = "Interrupt Enable set address\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten_set::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenSetSpec;
impl crate::RegisterSpec for IntenSetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`inten_set::W`](W) writer structure"]
impl crate::Writable for IntenSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTEN_SET to value 0"]
impl crate::Resettable for IntenSetSpec {
    const RESET_VALUE: u32 = 0;
}
