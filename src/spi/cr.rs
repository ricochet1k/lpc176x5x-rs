#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "The SPI controller sends and receives 8 bits of data per transfer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bitenable {
    #[doc = "1: The SPI controller sends and receives the number of bits selected by bits 11:8."]
    TheSpiControllerS = 1,
}
impl From<Bitenable> for bool {
    #[inline(always)]
    fn from(variant: Bitenable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BITENABLE` reader - The SPI controller sends and receives 8 bits of data per transfer."]
pub type BitenableR = crate::BitReader<Bitenable>;
impl BitenableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Bitenable> {
        match self.bits {
            true => Some(Bitenable::TheSpiControllerS),
            _ => None,
        }
    }
    #[doc = "The SPI controller sends and receives the number of bits selected by bits 11:8."]
    #[inline(always)]
    pub fn is_the_spi_controller_s(&self) -> bool {
        *self == Bitenable::TheSpiControllerS
    }
}
#[doc = "Field `BITENABLE` writer - The SPI controller sends and receives 8 bits of data per transfer."]
pub type BitenableW<'a, REG> = crate::BitWriter<'a, REG, Bitenable>;
impl<'a, REG> BitenableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The SPI controller sends and receives the number of bits selected by bits 11:8."]
    #[inline(always)]
    pub fn the_spi_controller_s(self) -> &'a mut crate::W<REG> {
        self.variant(Bitenable::TheSpiControllerS)
    }
}
#[doc = "Clock phase control determines the relationship between the data and the clock on SPI transfers, and controls when a slave transfer is defined as starting and ending.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpha {
    #[doc = "0: Data is sampled on the first clock edge of SCK. A transfer starts and ends with activation and deactivation of the SSEL signal."]
    FirstEdge = 0,
    #[doc = "1: Data is sampled on the second clock edge of the SCK. A transfer starts with the first clock edge, and ends with the last sampling edge when the SSEL signal is active."]
    SecondEdge = 1,
}
impl From<Cpha> for bool {
    #[inline(always)]
    fn from(variant: Cpha) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPHA` reader - Clock phase control determines the relationship between the data and the clock on SPI transfers, and controls when a slave transfer is defined as starting and ending."]
pub type CphaR = crate::BitReader<Cpha>;
impl CphaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpha {
        match self.bits {
            false => Cpha::FirstEdge,
            true => Cpha::SecondEdge,
        }
    }
    #[doc = "Data is sampled on the first clock edge of SCK. A transfer starts and ends with activation and deactivation of the SSEL signal."]
    #[inline(always)]
    pub fn is_first_edge(&self) -> bool {
        *self == Cpha::FirstEdge
    }
    #[doc = "Data is sampled on the second clock edge of the SCK. A transfer starts with the first clock edge, and ends with the last sampling edge when the SSEL signal is active."]
    #[inline(always)]
    pub fn is_second_edge(&self) -> bool {
        *self == Cpha::SecondEdge
    }
}
#[doc = "Field `CPHA` writer - Clock phase control determines the relationship between the data and the clock on SPI transfers, and controls when a slave transfer is defined as starting and ending."]
pub type CphaW<'a, REG> = crate::BitWriter<'a, REG, Cpha>;
impl<'a, REG> CphaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data is sampled on the first clock edge of SCK. A transfer starts and ends with activation and deactivation of the SSEL signal."]
    #[inline(always)]
    pub fn first_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Cpha::FirstEdge)
    }
    #[doc = "Data is sampled on the second clock edge of the SCK. A transfer starts with the first clock edge, and ends with the last sampling edge when the SSEL signal is active."]
    #[inline(always)]
    pub fn second_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Cpha::SecondEdge)
    }
}
#[doc = "Clock polarity control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpol {
    #[doc = "0: SCK is active high."]
    SckIsActiveHigh_ = 0,
    #[doc = "1: SCK is active low."]
    SckIsActiveLow_ = 1,
}
impl From<Cpol> for bool {
    #[inline(always)]
    fn from(variant: Cpol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPOL` reader - Clock polarity control."]
pub type CpolR = crate::BitReader<Cpol>;
impl CpolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpol {
        match self.bits {
            false => Cpol::SckIsActiveHigh_,
            true => Cpol::SckIsActiveLow_,
        }
    }
    #[doc = "SCK is active high."]
    #[inline(always)]
    pub fn is_sck_is_active_high_(&self) -> bool {
        *self == Cpol::SckIsActiveHigh_
    }
    #[doc = "SCK is active low."]
    #[inline(always)]
    pub fn is_sck_is_active_low_(&self) -> bool {
        *self == Cpol::SckIsActiveLow_
    }
}
#[doc = "Field `CPOL` writer - Clock polarity control."]
pub type CpolW<'a, REG> = crate::BitWriter<'a, REG, Cpol>;
impl<'a, REG> CpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SCK is active high."]
    #[inline(always)]
    pub fn sck_is_active_high_(self) -> &'a mut crate::W<REG> {
        self.variant(Cpol::SckIsActiveHigh_)
    }
    #[doc = "SCK is active low."]
    #[inline(always)]
    pub fn sck_is_active_low_(self) -> &'a mut crate::W<REG> {
        self.variant(Cpol::SckIsActiveLow_)
    }
}
#[doc = "Master mode select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstr {
    #[doc = "0: The SPI operates in Slave mode."]
    Slave = 0,
    #[doc = "1: The SPI operates in Master mode."]
    Master = 1,
}
impl From<Mstr> for bool {
    #[inline(always)]
    fn from(variant: Mstr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTR` reader - Master mode select."]
pub type MstrR = crate::BitReader<Mstr>;
impl MstrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mstr {
        match self.bits {
            false => Mstr::Slave,
            true => Mstr::Master,
        }
    }
    #[doc = "The SPI operates in Slave mode."]
    #[inline(always)]
    pub fn is_slave(&self) -> bool {
        *self == Mstr::Slave
    }
    #[doc = "The SPI operates in Master mode."]
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        *self == Mstr::Master
    }
}
#[doc = "Field `MSTR` writer - Master mode select."]
pub type MstrW<'a, REG> = crate::BitWriter<'a, REG, Mstr>;
impl<'a, REG> MstrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The SPI operates in Slave mode."]
    #[inline(always)]
    pub fn slave(self) -> &'a mut crate::W<REG> {
        self.variant(Mstr::Slave)
    }
    #[doc = "The SPI operates in Master mode."]
    #[inline(always)]
    pub fn master(self) -> &'a mut crate::W<REG> {
        self.variant(Mstr::Master)
    }
}
#[doc = "LSB First controls which direction each byte is shifted when transferred.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lsbf {
    #[doc = "0: SPI data is transferred MSB (bit 7) first."]
    Msb = 0,
    #[doc = "1: SPI data is transferred LSB (bit 0) first."]
    Lsb = 1,
}
impl From<Lsbf> for bool {
    #[inline(always)]
    fn from(variant: Lsbf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSBF` reader - LSB First controls which direction each byte is shifted when transferred."]
pub type LsbfR = crate::BitReader<Lsbf>;
impl LsbfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lsbf {
        match self.bits {
            false => Lsbf::Msb,
            true => Lsbf::Lsb,
        }
    }
    #[doc = "SPI data is transferred MSB (bit 7) first."]
    #[inline(always)]
    pub fn is_msb(&self) -> bool {
        *self == Lsbf::Msb
    }
    #[doc = "SPI data is transferred LSB (bit 0) first."]
    #[inline(always)]
    pub fn is_lsb(&self) -> bool {
        *self == Lsbf::Lsb
    }
}
#[doc = "Field `LSBF` writer - LSB First controls which direction each byte is shifted when transferred."]
pub type LsbfW<'a, REG> = crate::BitWriter<'a, REG, Lsbf>;
impl<'a, REG> LsbfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI data is transferred MSB (bit 7) first."]
    #[inline(always)]
    pub fn msb(self) -> &'a mut crate::W<REG> {
        self.variant(Lsbf::Msb)
    }
    #[doc = "SPI data is transferred LSB (bit 0) first."]
    #[inline(always)]
    pub fn lsb(self) -> &'a mut crate::W<REG> {
        self.variant(Lsbf::Lsb)
    }
}
#[doc = "Serial peripheral interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spie {
    #[doc = "0: SPI interrupts are inhibited."]
    Intblock = 0,
    #[doc = "1: A hardware interrupt is generated each time the SPIF or MODF bits are activated."]
    Hwint = 1,
}
impl From<Spie> for bool {
    #[inline(always)]
    fn from(variant: Spie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPIE` reader - Serial peripheral interrupt enable."]
pub type SpieR = crate::BitReader<Spie>;
impl SpieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spie {
        match self.bits {
            false => Spie::Intblock,
            true => Spie::Hwint,
        }
    }
    #[doc = "SPI interrupts are inhibited."]
    #[inline(always)]
    pub fn is_intblock(&self) -> bool {
        *self == Spie::Intblock
    }
    #[doc = "A hardware interrupt is generated each time the SPIF or MODF bits are activated."]
    #[inline(always)]
    pub fn is_hwint(&self) -> bool {
        *self == Spie::Hwint
    }
}
#[doc = "Field `SPIE` writer - Serial peripheral interrupt enable."]
pub type SpieW<'a, REG> = crate::BitWriter<'a, REG, Spie>;
impl<'a, REG> SpieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI interrupts are inhibited."]
    #[inline(always)]
    pub fn intblock(self) -> &'a mut crate::W<REG> {
        self.variant(Spie::Intblock)
    }
    #[doc = "A hardware interrupt is generated each time the SPIF or MODF bits are activated."]
    #[inline(always)]
    pub fn hwint(self) -> &'a mut crate::W<REG> {
        self.variant(Spie::Hwint)
    }
}
#[doc = "When bit 2 of this register is 1, this field controls the number of bits per transfer:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bits {
    #[doc = "8: 8 bits per transfer"]
    _8BitsPerTransfer = 8,
    #[doc = "9: 9 bits per transfer"]
    _9BitsPerTransfer = 9,
    #[doc = "10: 10 bits per transfer"]
    _10BitsPerTransfer = 10,
    #[doc = "11: 11 bits per transfer"]
    _11BitsPerTransfer = 11,
    #[doc = "12: 12 bits per transfer"]
    _12BitsPerTransfer = 12,
    #[doc = "13: 13 bits per transfer"]
    _13BitsPerTransfer = 13,
    #[doc = "14: 14 bits per transfer"]
    _14BitsPerTransfer = 14,
    #[doc = "15: 15 bits per transfer"]
    _15BitsPerTransfer = 15,
    #[doc = "0: 16 bits per transfer"]
    _16BitsPerTransfer = 0,
}
impl From<Bits> for u8 {
    #[inline(always)]
    fn from(variant: Bits) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bits {
    type Ux = u8;
}
impl crate::IsEnum for Bits {}
#[doc = "Field `BITS` reader - When bit 2 of this register is 1, this field controls the number of bits per transfer:"]
pub type BitsR = crate::FieldReader<Bits>;
impl BitsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Bits> {
        match self.bits {
            8 => Some(Bits::_8BitsPerTransfer),
            9 => Some(Bits::_9BitsPerTransfer),
            10 => Some(Bits::_10BitsPerTransfer),
            11 => Some(Bits::_11BitsPerTransfer),
            12 => Some(Bits::_12BitsPerTransfer),
            13 => Some(Bits::_13BitsPerTransfer),
            14 => Some(Bits::_14BitsPerTransfer),
            15 => Some(Bits::_15BitsPerTransfer),
            0 => Some(Bits::_16BitsPerTransfer),
            _ => None,
        }
    }
    #[doc = "8 bits per transfer"]
    #[inline(always)]
    pub fn is_8_bits_per_transfer(&self) -> bool {
        *self == Bits::_8BitsPerTransfer
    }
    #[doc = "9 bits per transfer"]
    #[inline(always)]
    pub fn is_9_bits_per_transfer(&self) -> bool {
        *self == Bits::_9BitsPerTransfer
    }
    #[doc = "10 bits per transfer"]
    #[inline(always)]
    pub fn is_10_bits_per_transfer(&self) -> bool {
        *self == Bits::_10BitsPerTransfer
    }
    #[doc = "11 bits per transfer"]
    #[inline(always)]
    pub fn is_11_bits_per_transfer(&self) -> bool {
        *self == Bits::_11BitsPerTransfer
    }
    #[doc = "12 bits per transfer"]
    #[inline(always)]
    pub fn is_12_bits_per_transfer(&self) -> bool {
        *self == Bits::_12BitsPerTransfer
    }
    #[doc = "13 bits per transfer"]
    #[inline(always)]
    pub fn is_13_bits_per_transfer(&self) -> bool {
        *self == Bits::_13BitsPerTransfer
    }
    #[doc = "14 bits per transfer"]
    #[inline(always)]
    pub fn is_14_bits_per_transfer(&self) -> bool {
        *self == Bits::_14BitsPerTransfer
    }
    #[doc = "15 bits per transfer"]
    #[inline(always)]
    pub fn is_15_bits_per_transfer(&self) -> bool {
        *self == Bits::_15BitsPerTransfer
    }
    #[doc = "16 bits per transfer"]
    #[inline(always)]
    pub fn is_16_bits_per_transfer(&self) -> bool {
        *self == Bits::_16BitsPerTransfer
    }
}
#[doc = "Field `BITS` writer - When bit 2 of this register is 1, this field controls the number of bits per transfer:"]
pub type BitsW<'a, REG> = crate::FieldWriter<'a, REG, 4, Bits>;
impl<'a, REG> BitsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 bits per transfer"]
    #[inline(always)]
    pub fn _8_bits_per_transfer(self) -> &'a mut crate::W<REG> {
        self.variant(Bits::_8BitsPerTransfer)
    }
    #[doc = "9 bits per transfer"]
    #[inline(always)]
    pub fn _9_bits_per_transfer(self) -> &'a mut crate::W<REG> {
        self.variant(Bits::_9BitsPerTransfer)
    }
    #[doc = "10 bits per transfer"]
    #[inline(always)]
    pub fn _10_bits_per_transfer(self) -> &'a mut crate::W<REG> {
        self.variant(Bits::_10BitsPerTransfer)
    }
    #[doc = "11 bits per transfer"]
    #[inline(always)]
    pub fn _11_bits_per_transfer(self) -> &'a mut crate::W<REG> {
        self.variant(Bits::_11BitsPerTransfer)
    }
    #[doc = "12 bits per transfer"]
    #[inline(always)]
    pub fn _12_bits_per_transfer(self) -> &'a mut crate::W<REG> {
        self.variant(Bits::_12BitsPerTransfer)
    }
    #[doc = "13 bits per transfer"]
    #[inline(always)]
    pub fn _13_bits_per_transfer(self) -> &'a mut crate::W<REG> {
        self.variant(Bits::_13BitsPerTransfer)
    }
    #[doc = "14 bits per transfer"]
    #[inline(always)]
    pub fn _14_bits_per_transfer(self) -> &'a mut crate::W<REG> {
        self.variant(Bits::_14BitsPerTransfer)
    }
    #[doc = "15 bits per transfer"]
    #[inline(always)]
    pub fn _15_bits_per_transfer(self) -> &'a mut crate::W<REG> {
        self.variant(Bits::_15BitsPerTransfer)
    }
    #[doc = "16 bits per transfer"]
    #[inline(always)]
    pub fn _16_bits_per_transfer(self) -> &'a mut crate::W<REG> {
        self.variant(Bits::_16BitsPerTransfer)
    }
}
impl R {
    #[doc = "Bit 2 - The SPI controller sends and receives 8 bits of data per transfer."]
    #[inline(always)]
    pub fn bitenable(&self) -> BitenableR {
        BitenableR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clock phase control determines the relationship between the data and the clock on SPI transfers, and controls when a slave transfer is defined as starting and ending."]
    #[inline(always)]
    pub fn cpha(&self) -> CphaR {
        CphaR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Clock polarity control."]
    #[inline(always)]
    pub fn cpol(&self) -> CpolR {
        CpolR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Master mode select."]
    #[inline(always)]
    pub fn mstr(&self) -> MstrR {
        MstrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LSB First controls which direction each byte is shifted when transferred."]
    #[inline(always)]
    pub fn lsbf(&self) -> LsbfR {
        LsbfR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Serial peripheral interrupt enable."]
    #[inline(always)]
    pub fn spie(&self) -> SpieR {
        SpieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - When bit 2 of this register is 1, this field controls the number of bits per transfer:"]
    #[inline(always)]
    pub fn bits_(&self) -> BitsR {
        BitsR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - The SPI controller sends and receives 8 bits of data per transfer."]
    #[inline(always)]
    #[must_use]
    pub fn bitenable(&mut self) -> BitenableW<CrSpec> {
        BitenableW::new(self, 2)
    }
    #[doc = "Bit 3 - Clock phase control determines the relationship between the data and the clock on SPI transfers, and controls when a slave transfer is defined as starting and ending."]
    #[inline(always)]
    #[must_use]
    pub fn cpha(&mut self) -> CphaW<CrSpec> {
        CphaW::new(self, 3)
    }
    #[doc = "Bit 4 - Clock polarity control."]
    #[inline(always)]
    #[must_use]
    pub fn cpol(&mut self) -> CpolW<CrSpec> {
        CpolW::new(self, 4)
    }
    #[doc = "Bit 5 - Master mode select."]
    #[inline(always)]
    #[must_use]
    pub fn mstr(&mut self) -> MstrW<CrSpec> {
        MstrW::new(self, 5)
    }
    #[doc = "Bit 6 - LSB First controls which direction each byte is shifted when transferred."]
    #[inline(always)]
    #[must_use]
    pub fn lsbf(&mut self) -> LsbfW<CrSpec> {
        LsbfW::new(self, 6)
    }
    #[doc = "Bit 7 - Serial peripheral interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn spie(&mut self) -> SpieW<CrSpec> {
        SpieW::new(self, 7)
    }
    #[doc = "Bits 8:11 - When bit 2 of this register is 1, this field controls the number of bits per transfer:"]
    #[inline(always)]
    #[must_use]
    pub fn bits_(&mut self) -> BitsW<CrSpec> {
        BitsW::new(self, 8)
    }
}
#[doc = "SPI Control Register. This register controls the operation of the SPI.\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {
    const RESET_VALUE: u32 = 0;
}
