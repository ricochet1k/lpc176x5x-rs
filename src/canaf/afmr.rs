#[doc = "Register `AFMR` reader"]
pub type R = crate::R<AfmrSpec>;
#[doc = "Register `AFMR` writer"]
pub type W = crate::W<AfmrSpec>;
#[doc = "Field `ACCOFF` reader - if AccBP is 0, the Acceptance Filter is not operational. All Rx messages on all CAN buses are ignored."]
pub type AccoffR = crate::BitReader;
#[doc = "Field `ACCOFF` writer - if AccBP is 0, the Acceptance Filter is not operational. All Rx messages on all CAN buses are ignored."]
pub type AccoffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACCBP` reader - All Rx messages are accepted on enabled CAN controllers. Software must set this bit before modifying the contents of any of the registers described below, and before modifying the contents of Lookup Table RAM in any way other than setting or clearing Disable bits in Standard Identifier entries. When both this bit and AccOff are 0, the Acceptance filter operates to screen received CAN Identifiers."]
pub type AccbpR = crate::BitReader;
#[doc = "Field `ACCBP` writer - All Rx messages are accepted on enabled CAN controllers. Software must set this bit before modifying the contents of any of the registers described below, and before modifying the contents of Lookup Table RAM in any way other than setting or clearing Disable bits in Standard Identifier entries. When both this bit and AccOff are 0, the Acceptance filter operates to screen received CAN Identifiers."]
pub type AccbpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "FullCAN mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Efcan {
    #[doc = "0: Software must read all messages for all enabled IDs on all enabled CAN buses, from the receiving CAN controllers."]
    SoftwareMustReadA = 0,
    #[doc = "1: The Acceptance Filter itself will take care of receiving and storing messages for selected Standard ID values on selected CAN buses. See Section 21.16 FullCAN mode on page 576."]
    TheAcceptanceFilte = 1,
}
impl From<Efcan> for bool {
    #[inline(always)]
    fn from(variant: Efcan) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EFCAN` reader - FullCAN mode"]
pub type EfcanR = crate::BitReader<Efcan>;
impl EfcanR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Efcan {
        match self.bits {
            false => Efcan::SoftwareMustReadA,
            true => Efcan::TheAcceptanceFilte,
        }
    }
    #[doc = "Software must read all messages for all enabled IDs on all enabled CAN buses, from the receiving CAN controllers."]
    #[inline(always)]
    pub fn is_software_must_read_a(&self) -> bool {
        *self == Efcan::SoftwareMustReadA
    }
    #[doc = "The Acceptance Filter itself will take care of receiving and storing messages for selected Standard ID values on selected CAN buses. See Section 21.16 FullCAN mode on page 576."]
    #[inline(always)]
    pub fn is_the_acceptance_filte(&self) -> bool {
        *self == Efcan::TheAcceptanceFilte
    }
}
#[doc = "Field `EFCAN` writer - FullCAN mode"]
pub type EfcanW<'a, REG> = crate::BitWriter<'a, REG, Efcan>;
impl<'a, REG> EfcanW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software must read all messages for all enabled IDs on all enabled CAN buses, from the receiving CAN controllers."]
    #[inline(always)]
    pub fn software_must_read_a(self) -> &'a mut crate::W<REG> {
        self.variant(Efcan::SoftwareMustReadA)
    }
    #[doc = "The Acceptance Filter itself will take care of receiving and storing messages for selected Standard ID values on selected CAN buses. See Section 21.16 FullCAN mode on page 576."]
    #[inline(always)]
    pub fn the_acceptance_filte(self) -> &'a mut crate::W<REG> {
        self.variant(Efcan::TheAcceptanceFilte)
    }
}
impl R {
    #[doc = "Bit 0 - if AccBP is 0, the Acceptance Filter is not operational. All Rx messages on all CAN buses are ignored."]
    #[inline(always)]
    pub fn accoff(&self) -> AccoffR {
        AccoffR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - All Rx messages are accepted on enabled CAN controllers. Software must set this bit before modifying the contents of any of the registers described below, and before modifying the contents of Lookup Table RAM in any way other than setting or clearing Disable bits in Standard Identifier entries. When both this bit and AccOff are 0, the Acceptance filter operates to screen received CAN Identifiers."]
    #[inline(always)]
    pub fn accbp(&self) -> AccbpR {
        AccbpR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FullCAN mode"]
    #[inline(always)]
    pub fn efcan(&self) -> EfcanR {
        EfcanR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - if AccBP is 0, the Acceptance Filter is not operational. All Rx messages on all CAN buses are ignored."]
    #[inline(always)]
    #[must_use]
    pub fn accoff(&mut self) -> AccoffW<AfmrSpec> {
        AccoffW::new(self, 0)
    }
    #[doc = "Bit 1 - All Rx messages are accepted on enabled CAN controllers. Software must set this bit before modifying the contents of any of the registers described below, and before modifying the contents of Lookup Table RAM in any way other than setting or clearing Disable bits in Standard Identifier entries. When both this bit and AccOff are 0, the Acceptance filter operates to screen received CAN Identifiers."]
    #[inline(always)]
    #[must_use]
    pub fn accbp(&mut self) -> AccbpW<AfmrSpec> {
        AccbpW::new(self, 1)
    }
    #[doc = "Bit 2 - FullCAN mode"]
    #[inline(always)]
    #[must_use]
    pub fn efcan(&mut self) -> EfcanW<AfmrSpec> {
        EfcanW::new(self, 2)
    }
}
#[doc = "Acceptance Filter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`afmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AfmrSpec;
impl crate::RegisterSpec for AfmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afmr::R`](R) reader structure"]
impl crate::Readable for AfmrSpec {}
#[doc = "`write(|w| ..)` method takes [`afmr::W`](W) writer structure"]
impl crate::Writable for AfmrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AFMR to value 0"]
impl crate::Resettable for AfmrSpec {
    const RESET_VALUE: u32 = 0;
}
