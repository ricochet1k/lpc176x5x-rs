#[doc = "Register `EMR` reader"]
pub type R = crate::R<EmrSpec>;
#[doc = "Register `EMR` writer"]
pub type W = crate::W<EmrSpec>;
#[doc = "Field `EM0` reader - External Match 0. When a match occurs between the TC and MR0, this bit can either toggle, go low, go high, or do nothing, depending on bits 5:4 of this register. This bit can be driven onto a MATn.0 pin, in a positive-logic manner (0 = low, 1 = high)."]
pub type Em0R = crate::BitReader;
#[doc = "Field `EM0` writer - External Match 0. When a match occurs between the TC and MR0, this bit can either toggle, go low, go high, or do nothing, depending on bits 5:4 of this register. This bit can be driven onto a MATn.0 pin, in a positive-logic manner (0 = low, 1 = high)."]
pub type Em0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM1` reader - External Match 1. When a match occurs between the TC and MR1, this bit can either toggle, go low, go high, or do nothing, depending on bits 7:6 of this register. This bit can be driven onto a MATn.1 pin, in a positive-logic manner (0 = low, 1 = high)."]
pub type Em1R = crate::BitReader;
#[doc = "Field `EM1` writer - External Match 1. When a match occurs between the TC and MR1, this bit can either toggle, go low, go high, or do nothing, depending on bits 7:6 of this register. This bit can be driven onto a MATn.1 pin, in a positive-logic manner (0 = low, 1 = high)."]
pub type Em1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM2` reader - External Match 2. When a match occurs between the TC and MR2, this bit can either toggle, go low, go high, or do nothing, depending on bits 9:8 of this register. This bit can be driven onto a MATn.0 pin, in a positive-logic manner (0 = low, 1 = high)."]
pub type Em2R = crate::BitReader;
#[doc = "Field `EM2` writer - External Match 2. When a match occurs between the TC and MR2, this bit can either toggle, go low, go high, or do nothing, depending on bits 9:8 of this register. This bit can be driven onto a MATn.0 pin, in a positive-logic manner (0 = low, 1 = high)."]
pub type Em2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM3` reader - External Match 3. When a match occurs between the TC and MR3, this bit can either toggle, go low, go high, or do nothing, depending on bits 11:10 of this register. This bit can be driven onto a MATn.0 pin, in a positive-logic manner (0 = low, 1 = high)."]
pub type Em3R = crate::BitReader;
#[doc = "Field `EM3` writer - External Match 3. When a match occurs between the TC and MR3, this bit can either toggle, go low, go high, or do nothing, depending on bits 11:10 of this register. This bit can be driven onto a MATn.0 pin, in a positive-logic manner (0 = low, 1 = high)."]
pub type Em3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "External Match Control 0. Determines the functionality of External Match 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Emc0 {
    #[doc = "0: Do Nothing."]
    DoNothing_ = 0,
    #[doc = "1: Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)."]
    ClearTheCorrespond = 1,
    #[doc = "2: Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)."]
    SetTheCorrespondin = 2,
    #[doc = "3: Toggle the corresponding External Match bit/output."]
    ToggleTheCorrespon = 3,
}
impl From<Emc0> for u8 {
    #[inline(always)]
    fn from(variant: Emc0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Emc0 {
    type Ux = u8;
}
impl crate::IsEnum for Emc0 {}
#[doc = "Field `EMC0` reader - External Match Control 0. Determines the functionality of External Match 0."]
pub type Emc0R = crate::FieldReader<Emc0>;
impl Emc0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Emc0 {
        match self.bits {
            0 => Emc0::DoNothing_,
            1 => Emc0::ClearTheCorrespond,
            2 => Emc0::SetTheCorrespondin,
            3 => Emc0::ToggleTheCorrespon,
            _ => unreachable!(),
        }
    }
    #[doc = "Do Nothing."]
    #[inline(always)]
    pub fn is_do_nothing_(&self) -> bool {
        *self == Emc0::DoNothing_
    }
    #[doc = "Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)."]
    #[inline(always)]
    pub fn is_clear_the_correspond(&self) -> bool {
        *self == Emc0::ClearTheCorrespond
    }
    #[doc = "Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)."]
    #[inline(always)]
    pub fn is_set_the_correspondin(&self) -> bool {
        *self == Emc0::SetTheCorrespondin
    }
    #[doc = "Toggle the corresponding External Match bit/output."]
    #[inline(always)]
    pub fn is_toggle_the_correspon(&self) -> bool {
        *self == Emc0::ToggleTheCorrespon
    }
}
#[doc = "Field `EMC0` writer - External Match Control 0. Determines the functionality of External Match 0."]
pub type Emc0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Emc0, crate::Safe>;
impl<'a, REG> Emc0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Do Nothing."]
    #[inline(always)]
    pub fn do_nothing_(self) -> &'a mut crate::W<REG> {
        self.variant(Emc0::DoNothing_)
    }
    #[doc = "Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)."]
    #[inline(always)]
    pub fn clear_the_correspond(self) -> &'a mut crate::W<REG> {
        self.variant(Emc0::ClearTheCorrespond)
    }
    #[doc = "Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)."]
    #[inline(always)]
    pub fn set_the_correspondin(self) -> &'a mut crate::W<REG> {
        self.variant(Emc0::SetTheCorrespondin)
    }
    #[doc = "Toggle the corresponding External Match bit/output."]
    #[inline(always)]
    pub fn toggle_the_correspon(self) -> &'a mut crate::W<REG> {
        self.variant(Emc0::ToggleTheCorrespon)
    }
}
#[doc = "External Match Control 1. Determines the functionality of External Match 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Emc1 {
    #[doc = "0: Do Nothing."]
    DoNothing_ = 0,
    #[doc = "1: Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)."]
    ClearTheCorrespond = 1,
    #[doc = "2: Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)."]
    SetTheCorrespondin = 2,
    #[doc = "3: Toggle the corresponding External Match bit/output."]
    ToggleTheCorrespon = 3,
}
impl From<Emc1> for u8 {
    #[inline(always)]
    fn from(variant: Emc1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Emc1 {
    type Ux = u8;
}
impl crate::IsEnum for Emc1 {}
#[doc = "Field `EMC1` reader - External Match Control 1. Determines the functionality of External Match 1."]
pub type Emc1R = crate::FieldReader<Emc1>;
impl Emc1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Emc1 {
        match self.bits {
            0 => Emc1::DoNothing_,
            1 => Emc1::ClearTheCorrespond,
            2 => Emc1::SetTheCorrespondin,
            3 => Emc1::ToggleTheCorrespon,
            _ => unreachable!(),
        }
    }
    #[doc = "Do Nothing."]
    #[inline(always)]
    pub fn is_do_nothing_(&self) -> bool {
        *self == Emc1::DoNothing_
    }
    #[doc = "Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)."]
    #[inline(always)]
    pub fn is_clear_the_correspond(&self) -> bool {
        *self == Emc1::ClearTheCorrespond
    }
    #[doc = "Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)."]
    #[inline(always)]
    pub fn is_set_the_correspondin(&self) -> bool {
        *self == Emc1::SetTheCorrespondin
    }
    #[doc = "Toggle the corresponding External Match bit/output."]
    #[inline(always)]
    pub fn is_toggle_the_correspon(&self) -> bool {
        *self == Emc1::ToggleTheCorrespon
    }
}
#[doc = "Field `EMC1` writer - External Match Control 1. Determines the functionality of External Match 1."]
pub type Emc1W<'a, REG> = crate::FieldWriter<'a, REG, 2, Emc1, crate::Safe>;
impl<'a, REG> Emc1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Do Nothing."]
    #[inline(always)]
    pub fn do_nothing_(self) -> &'a mut crate::W<REG> {
        self.variant(Emc1::DoNothing_)
    }
    #[doc = "Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)."]
    #[inline(always)]
    pub fn clear_the_correspond(self) -> &'a mut crate::W<REG> {
        self.variant(Emc1::ClearTheCorrespond)
    }
    #[doc = "Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)."]
    #[inline(always)]
    pub fn set_the_correspondin(self) -> &'a mut crate::W<REG> {
        self.variant(Emc1::SetTheCorrespondin)
    }
    #[doc = "Toggle the corresponding External Match bit/output."]
    #[inline(always)]
    pub fn toggle_the_correspon(self) -> &'a mut crate::W<REG> {
        self.variant(Emc1::ToggleTheCorrespon)
    }
}
#[doc = "External Match Control 2. Determines the functionality of External Match 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Emc2 {
    #[doc = "0: Do Nothing."]
    DoNothing_ = 0,
    #[doc = "1: Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)."]
    ClearTheCorrespond = 1,
    #[doc = "2: Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)."]
    SetTheCorrespondin = 2,
    #[doc = "3: Toggle the corresponding External Match bit/output."]
    ToggleTheCorrespon = 3,
}
impl From<Emc2> for u8 {
    #[inline(always)]
    fn from(variant: Emc2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Emc2 {
    type Ux = u8;
}
impl crate::IsEnum for Emc2 {}
#[doc = "Field `EMC2` reader - External Match Control 2. Determines the functionality of External Match 2."]
pub type Emc2R = crate::FieldReader<Emc2>;
impl Emc2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Emc2 {
        match self.bits {
            0 => Emc2::DoNothing_,
            1 => Emc2::ClearTheCorrespond,
            2 => Emc2::SetTheCorrespondin,
            3 => Emc2::ToggleTheCorrespon,
            _ => unreachable!(),
        }
    }
    #[doc = "Do Nothing."]
    #[inline(always)]
    pub fn is_do_nothing_(&self) -> bool {
        *self == Emc2::DoNothing_
    }
    #[doc = "Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)."]
    #[inline(always)]
    pub fn is_clear_the_correspond(&self) -> bool {
        *self == Emc2::ClearTheCorrespond
    }
    #[doc = "Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)."]
    #[inline(always)]
    pub fn is_set_the_correspondin(&self) -> bool {
        *self == Emc2::SetTheCorrespondin
    }
    #[doc = "Toggle the corresponding External Match bit/output."]
    #[inline(always)]
    pub fn is_toggle_the_correspon(&self) -> bool {
        *self == Emc2::ToggleTheCorrespon
    }
}
#[doc = "Field `EMC2` writer - External Match Control 2. Determines the functionality of External Match 2."]
pub type Emc2W<'a, REG> = crate::FieldWriter<'a, REG, 2, Emc2, crate::Safe>;
impl<'a, REG> Emc2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Do Nothing."]
    #[inline(always)]
    pub fn do_nothing_(self) -> &'a mut crate::W<REG> {
        self.variant(Emc2::DoNothing_)
    }
    #[doc = "Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)."]
    #[inline(always)]
    pub fn clear_the_correspond(self) -> &'a mut crate::W<REG> {
        self.variant(Emc2::ClearTheCorrespond)
    }
    #[doc = "Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)."]
    #[inline(always)]
    pub fn set_the_correspondin(self) -> &'a mut crate::W<REG> {
        self.variant(Emc2::SetTheCorrespondin)
    }
    #[doc = "Toggle the corresponding External Match bit/output."]
    #[inline(always)]
    pub fn toggle_the_correspon(self) -> &'a mut crate::W<REG> {
        self.variant(Emc2::ToggleTheCorrespon)
    }
}
#[doc = "External Match Control 3. Determines the functionality of External Match 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Emc3 {
    #[doc = "0: Do Nothing."]
    DoNothing_ = 0,
    #[doc = "1: Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)."]
    ClearTheCorrespond = 1,
    #[doc = "2: Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)."]
    SetTheCorrespondin = 2,
    #[doc = "3: Toggle the corresponding External Match bit/output."]
    ToggleTheCorrespon = 3,
}
impl From<Emc3> for u8 {
    #[inline(always)]
    fn from(variant: Emc3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Emc3 {
    type Ux = u8;
}
impl crate::IsEnum for Emc3 {}
#[doc = "Field `EMC3` reader - External Match Control 3. Determines the functionality of External Match 3."]
pub type Emc3R = crate::FieldReader<Emc3>;
impl Emc3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Emc3 {
        match self.bits {
            0 => Emc3::DoNothing_,
            1 => Emc3::ClearTheCorrespond,
            2 => Emc3::SetTheCorrespondin,
            3 => Emc3::ToggleTheCorrespon,
            _ => unreachable!(),
        }
    }
    #[doc = "Do Nothing."]
    #[inline(always)]
    pub fn is_do_nothing_(&self) -> bool {
        *self == Emc3::DoNothing_
    }
    #[doc = "Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)."]
    #[inline(always)]
    pub fn is_clear_the_correspond(&self) -> bool {
        *self == Emc3::ClearTheCorrespond
    }
    #[doc = "Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)."]
    #[inline(always)]
    pub fn is_set_the_correspondin(&self) -> bool {
        *self == Emc3::SetTheCorrespondin
    }
    #[doc = "Toggle the corresponding External Match bit/output."]
    #[inline(always)]
    pub fn is_toggle_the_correspon(&self) -> bool {
        *self == Emc3::ToggleTheCorrespon
    }
}
#[doc = "Field `EMC3` writer - External Match Control 3. Determines the functionality of External Match 3."]
pub type Emc3W<'a, REG> = crate::FieldWriter<'a, REG, 2, Emc3, crate::Safe>;
impl<'a, REG> Emc3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Do Nothing."]
    #[inline(always)]
    pub fn do_nothing_(self) -> &'a mut crate::W<REG> {
        self.variant(Emc3::DoNothing_)
    }
    #[doc = "Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)."]
    #[inline(always)]
    pub fn clear_the_correspond(self) -> &'a mut crate::W<REG> {
        self.variant(Emc3::ClearTheCorrespond)
    }
    #[doc = "Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)."]
    #[inline(always)]
    pub fn set_the_correspondin(self) -> &'a mut crate::W<REG> {
        self.variant(Emc3::SetTheCorrespondin)
    }
    #[doc = "Toggle the corresponding External Match bit/output."]
    #[inline(always)]
    pub fn toggle_the_correspon(self) -> &'a mut crate::W<REG> {
        self.variant(Emc3::ToggleTheCorrespon)
    }
}
impl R {
    #[doc = "Bit 0 - External Match 0. When a match occurs between the TC and MR0, this bit can either toggle, go low, go high, or do nothing, depending on bits 5:4 of this register. This bit can be driven onto a MATn.0 pin, in a positive-logic manner (0 = low, 1 = high)."]
    #[inline(always)]
    pub fn em0(&self) -> Em0R {
        Em0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External Match 1. When a match occurs between the TC and MR1, this bit can either toggle, go low, go high, or do nothing, depending on bits 7:6 of this register. This bit can be driven onto a MATn.1 pin, in a positive-logic manner (0 = low, 1 = high)."]
    #[inline(always)]
    pub fn em1(&self) -> Em1R {
        Em1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External Match 2. When a match occurs between the TC and MR2, this bit can either toggle, go low, go high, or do nothing, depending on bits 9:8 of this register. This bit can be driven onto a MATn.0 pin, in a positive-logic manner (0 = low, 1 = high)."]
    #[inline(always)]
    pub fn em2(&self) -> Em2R {
        Em2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - External Match 3. When a match occurs between the TC and MR3, this bit can either toggle, go low, go high, or do nothing, depending on bits 11:10 of this register. This bit can be driven onto a MATn.0 pin, in a positive-logic manner (0 = low, 1 = high)."]
    #[inline(always)]
    pub fn em3(&self) -> Em3R {
        Em3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - External Match Control 0. Determines the functionality of External Match 0."]
    #[inline(always)]
    pub fn emc0(&self) -> Emc0R {
        Emc0R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - External Match Control 1. Determines the functionality of External Match 1."]
    #[inline(always)]
    pub fn emc1(&self) -> Emc1R {
        Emc1R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - External Match Control 2. Determines the functionality of External Match 2."]
    #[inline(always)]
    pub fn emc2(&self) -> Emc2R {
        Emc2R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - External Match Control 3. Determines the functionality of External Match 3."]
    #[inline(always)]
    pub fn emc3(&self) -> Emc3R {
        Emc3R::new(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - External Match 0. When a match occurs between the TC and MR0, this bit can either toggle, go low, go high, or do nothing, depending on bits 5:4 of this register. This bit can be driven onto a MATn.0 pin, in a positive-logic manner (0 = low, 1 = high)."]
    #[inline(always)]
    #[must_use]
    pub fn em0(&mut self) -> Em0W<EmrSpec> {
        Em0W::new(self, 0)
    }
    #[doc = "Bit 1 - External Match 1. When a match occurs between the TC and MR1, this bit can either toggle, go low, go high, or do nothing, depending on bits 7:6 of this register. This bit can be driven onto a MATn.1 pin, in a positive-logic manner (0 = low, 1 = high)."]
    #[inline(always)]
    #[must_use]
    pub fn em1(&mut self) -> Em1W<EmrSpec> {
        Em1W::new(self, 1)
    }
    #[doc = "Bit 2 - External Match 2. When a match occurs between the TC and MR2, this bit can either toggle, go low, go high, or do nothing, depending on bits 9:8 of this register. This bit can be driven onto a MATn.0 pin, in a positive-logic manner (0 = low, 1 = high)."]
    #[inline(always)]
    #[must_use]
    pub fn em2(&mut self) -> Em2W<EmrSpec> {
        Em2W::new(self, 2)
    }
    #[doc = "Bit 3 - External Match 3. When a match occurs between the TC and MR3, this bit can either toggle, go low, go high, or do nothing, depending on bits 11:10 of this register. This bit can be driven onto a MATn.0 pin, in a positive-logic manner (0 = low, 1 = high)."]
    #[inline(always)]
    #[must_use]
    pub fn em3(&mut self) -> Em3W<EmrSpec> {
        Em3W::new(self, 3)
    }
    #[doc = "Bits 4:5 - External Match Control 0. Determines the functionality of External Match 0."]
    #[inline(always)]
    #[must_use]
    pub fn emc0(&mut self) -> Emc0W<EmrSpec> {
        Emc0W::new(self, 4)
    }
    #[doc = "Bits 6:7 - External Match Control 1. Determines the functionality of External Match 1."]
    #[inline(always)]
    #[must_use]
    pub fn emc1(&mut self) -> Emc1W<EmrSpec> {
        Emc1W::new(self, 6)
    }
    #[doc = "Bits 8:9 - External Match Control 2. Determines the functionality of External Match 2."]
    #[inline(always)]
    #[must_use]
    pub fn emc2(&mut self) -> Emc2W<EmrSpec> {
        Emc2W::new(self, 8)
    }
    #[doc = "Bits 10:11 - External Match Control 3. Determines the functionality of External Match 3."]
    #[inline(always)]
    #[must_use]
    pub fn emc3(&mut self) -> Emc3W<EmrSpec> {
        Emc3W::new(self, 10)
    }
}
#[doc = "External Match Register. The EMR controls the external match pins.\n\nYou can [`read`](crate::Reg::read) this register and get [`emr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmrSpec;
impl crate::RegisterSpec for EmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emr::R`](R) reader structure"]
impl crate::Readable for EmrSpec {}
#[doc = "`write(|w| ..)` method takes [`emr::W`](W) writer structure"]
impl crate::Writable for EmrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EMR to value 0"]
impl crate::Resettable for EmrSpec {
    const RESET_VALUE: u32 = 0;
}
