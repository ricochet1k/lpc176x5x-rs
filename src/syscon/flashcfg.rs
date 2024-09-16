#[doc = "Register `FLASHCFG` reader"]
pub type R = crate::R<FlashcfgSpec>;
#[doc = "Register `FLASHCFG` writer"]
pub type W = crate::W<FlashcfgSpec>;
#[doc = "Flash access time. The value of this field plus 1 gives the number of CPU clocks used for a flash access. Warning: improper setting of this value may result in incorrect operation of the device. Other values are reserved.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Flashtim {
    #[doc = "0: Flash accesses use 1 CPU clock. Use for up to 20 MHz CPU clock."]
    _1clk = 0,
    #[doc = "1: Flash accesses use 2 CPU clocks. Use for up to 40 MHz CPU clock."]
    _2clk = 1,
    #[doc = "2: Flash accesses use 3 CPU clocks. Use for up to 60 MHz CPU clock."]
    _3clk = 2,
    #[doc = "3: Flash accesses use 4 CPU clocks. Use for up to 80 MHz CPU clock."]
    _4clk = 3,
    #[doc = "4: Flash accesses use 5 CPU clocks. Use for up to 100 MHz CPU clock. Use for up to 120 Mhz for LPC1759 and LPC1769 only."]
    _5clk = 4,
    #[doc = "5: Flash accesses use 6 CPU clocks. This safe setting will work under any conditions."]
    _6clk = 5,
}
impl From<Flashtim> for u8 {
    #[inline(always)]
    fn from(variant: Flashtim) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Flashtim {
    type Ux = u8;
}
impl crate::IsEnum for Flashtim {}
#[doc = "Field `FLASHTIM` reader - Flash access time. The value of this field plus 1 gives the number of CPU clocks used for a flash access. Warning: improper setting of this value may result in incorrect operation of the device. Other values are reserved."]
pub type FlashtimR = crate::FieldReader<Flashtim>;
impl FlashtimR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Flashtim> {
        match self.bits {
            0 => Some(Flashtim::_1clk),
            1 => Some(Flashtim::_2clk),
            2 => Some(Flashtim::_3clk),
            3 => Some(Flashtim::_4clk),
            4 => Some(Flashtim::_5clk),
            5 => Some(Flashtim::_6clk),
            _ => None,
        }
    }
    #[doc = "Flash accesses use 1 CPU clock. Use for up to 20 MHz CPU clock."]
    #[inline(always)]
    pub fn is_1clk(&self) -> bool {
        *self == Flashtim::_1clk
    }
    #[doc = "Flash accesses use 2 CPU clocks. Use for up to 40 MHz CPU clock."]
    #[inline(always)]
    pub fn is_2clk(&self) -> bool {
        *self == Flashtim::_2clk
    }
    #[doc = "Flash accesses use 3 CPU clocks. Use for up to 60 MHz CPU clock."]
    #[inline(always)]
    pub fn is_3clk(&self) -> bool {
        *self == Flashtim::_3clk
    }
    #[doc = "Flash accesses use 4 CPU clocks. Use for up to 80 MHz CPU clock."]
    #[inline(always)]
    pub fn is_4clk(&self) -> bool {
        *self == Flashtim::_4clk
    }
    #[doc = "Flash accesses use 5 CPU clocks. Use for up to 100 MHz CPU clock. Use for up to 120 Mhz for LPC1759 and LPC1769 only."]
    #[inline(always)]
    pub fn is_5clk(&self) -> bool {
        *self == Flashtim::_5clk
    }
    #[doc = "Flash accesses use 6 CPU clocks. This safe setting will work under any conditions."]
    #[inline(always)]
    pub fn is_6clk(&self) -> bool {
        *self == Flashtim::_6clk
    }
}
#[doc = "Field `FLASHTIM` writer - Flash access time. The value of this field plus 1 gives the number of CPU clocks used for a flash access. Warning: improper setting of this value may result in incorrect operation of the device. Other values are reserved."]
pub type FlashtimW<'a, REG> = crate::FieldWriter<'a, REG, 4, Flashtim>;
impl<'a, REG> FlashtimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Flash accesses use 1 CPU clock. Use for up to 20 MHz CPU clock."]
    #[inline(always)]
    pub fn _1clk(self) -> &'a mut crate::W<REG> {
        self.variant(Flashtim::_1clk)
    }
    #[doc = "Flash accesses use 2 CPU clocks. Use for up to 40 MHz CPU clock."]
    #[inline(always)]
    pub fn _2clk(self) -> &'a mut crate::W<REG> {
        self.variant(Flashtim::_2clk)
    }
    #[doc = "Flash accesses use 3 CPU clocks. Use for up to 60 MHz CPU clock."]
    #[inline(always)]
    pub fn _3clk(self) -> &'a mut crate::W<REG> {
        self.variant(Flashtim::_3clk)
    }
    #[doc = "Flash accesses use 4 CPU clocks. Use for up to 80 MHz CPU clock."]
    #[inline(always)]
    pub fn _4clk(self) -> &'a mut crate::W<REG> {
        self.variant(Flashtim::_4clk)
    }
    #[doc = "Flash accesses use 5 CPU clocks. Use for up to 100 MHz CPU clock. Use for up to 120 Mhz for LPC1759 and LPC1769 only."]
    #[inline(always)]
    pub fn _5clk(self) -> &'a mut crate::W<REG> {
        self.variant(Flashtim::_5clk)
    }
    #[doc = "Flash accesses use 6 CPU clocks. This safe setting will work under any conditions."]
    #[inline(always)]
    pub fn _6clk(self) -> &'a mut crate::W<REG> {
        self.variant(Flashtim::_6clk)
    }
}
impl R {
    #[doc = "Bits 12:15 - Flash access time. The value of this field plus 1 gives the number of CPU clocks used for a flash access. Warning: improper setting of this value may result in incorrect operation of the device. Other values are reserved."]
    #[inline(always)]
    pub fn flashtim(&self) -> FlashtimR {
        FlashtimR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 12:15 - Flash access time. The value of this field plus 1 gives the number of CPU clocks used for a flash access. Warning: improper setting of this value may result in incorrect operation of the device. Other values are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn flashtim(&mut self) -> FlashtimW<FlashcfgSpec> {
        FlashtimW::new(self, 12)
    }
}
#[doc = "Flash Accelerator Configuration Register. Controls flash access timing.\n\nYou can [`read`](crate::Reg::read) this register and get [`flashcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashcfgSpec;
impl crate::RegisterSpec for FlashcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flashcfg::R`](R) reader structure"]
impl crate::Readable for FlashcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`flashcfg::W`](W) writer structure"]
impl crate::Writable for FlashcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASHCFG to value 0x303a"]
impl crate::Resettable for FlashcfgSpec {
    const RESET_VALUE: u32 = 0x303a;
}
