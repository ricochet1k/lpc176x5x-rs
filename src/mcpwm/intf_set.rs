#[doc = "Register `INTF_SET` writer"]
pub type W = crate::W<IntfSetSpec>;
#[doc = "Field `ILIM0_F_SET` writer - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
pub type Ilim0FSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMAT0_F_SET` writer - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
pub type Imat0FSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICAP0_F_SET` writer - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
pub type Icap0FSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ILIM1_F_SET` writer - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
pub type Ilim1FSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMAT1_F_SET` writer - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
pub type Imat1FSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICAP1_F_SET` writer - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
pub type Icap1FSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ILIM2_F_SET` writer - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
pub type Ilim2FSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMAT2_F_SET` writer - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
pub type Imat2FSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICAP2_F_SET` writer - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
pub type Icap2FSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABORT_F_SET` writer - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
pub type AbortFSetW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ilim0_f_set(&mut self) -> Ilim0FSetW<IntfSetSpec> {
        Ilim0FSetW::new(self, 0)
    }
    #[doc = "Bit 1 - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn imat0_f_set(&mut self) -> Imat0FSetW<IntfSetSpec> {
        Imat0FSetW::new(self, 1)
    }
    #[doc = "Bit 2 - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn icap0_f_set(&mut self) -> Icap0FSetW<IntfSetSpec> {
        Icap0FSetW::new(self, 2)
    }
    #[doc = "Bit 4 - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ilim1_f_set(&mut self) -> Ilim1FSetW<IntfSetSpec> {
        Ilim1FSetW::new(self, 4)
    }
    #[doc = "Bit 5 - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn imat1_f_set(&mut self) -> Imat1FSetW<IntfSetSpec> {
        Imat1FSetW::new(self, 5)
    }
    #[doc = "Bit 6 - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn icap1_f_set(&mut self) -> Icap1FSetW<IntfSetSpec> {
        Icap1FSetW::new(self, 6)
    }
    #[doc = "Bit 8 - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ilim2_f_set(&mut self) -> Ilim2FSetW<IntfSetSpec> {
        Ilim2FSetW::new(self, 8)
    }
    #[doc = "Bit 9 - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn imat2_f_set(&mut self) -> Imat2FSetW<IntfSetSpec> {
        Imat2FSetW::new(self, 9)
    }
    #[doc = "Bit 10 - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn icap2_f_set(&mut self) -> Icap2FSetW<IntfSetSpec> {
        Icap2FSetW::new(self, 10)
    }
    #[doc = "Bit 15 - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn abort_f_set(&mut self) -> AbortFSetW<IntfSetSpec> {
        AbortFSetW::new(self, 15)
    }
}
#[doc = "Interrupt flags set address\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_set::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntfSetSpec;
impl crate::RegisterSpec for IntfSetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`intf_set::W`](W) writer structure"]
impl crate::Writable for IntfSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTF_SET to value 0"]
impl crate::Resettable for IntfSetSpec {
    const RESET_VALUE: u32 = 0;
}
