#[doc = "Register `CONTROL%s` reader"]
pub type R = crate::R<ControlSpec>;
#[doc = "Register `CONTROL%s` writer"]
pub type W = crate::W<ControlSpec>;
#[doc = "Field `TRANSFERSIZE` reader - Transfer size. This field sets the size of the transfer when the DMA controller is the flow controller, in which case the value must be set before the channel is enabled. Transfer size is updated as data transfers are completed. A read from this field indicates the number of transfers completed on the destination bus. Reading the register when the channel is active does not give useful information because by the time that the software has processed the value read, the channel might have progressed. It is intended to be used only when a channel is enabled and then disabled. The transfer size value is not used if a peripheral is the flow controller."]
pub type TransfersizeR = crate::FieldReader<u16>;
#[doc = "Field `TRANSFERSIZE` writer - Transfer size. This field sets the size of the transfer when the DMA controller is the flow controller, in which case the value must be set before the channel is enabled. Transfer size is updated as data transfers are completed. A read from this field indicates the number of transfers completed on the destination bus. Reading the register when the channel is active does not give useful information because by the time that the software has processed the value read, the channel might have progressed. It is intended to be used only when a channel is enabled and then disabled. The transfer size value is not used if a peripheral is the flow controller."]
pub type TransfersizeW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `SBSIZE` reader - Source burst size. Indicates the number of transfers that make up a source burst. This value must be set to the burst size of the source peripheral, or if the source is memory, to the memory boundary size. The burst size is the amount of data that is transferred when the DMACBREQ signal goes active in the source peripheral. 000 - 1 001 - 4 010 - 8 011 - 16 100 - 32 101 - 64 110 - 128 111 - 256"]
pub type SbsizeR = crate::FieldReader;
#[doc = "Field `SBSIZE` writer - Source burst size. Indicates the number of transfers that make up a source burst. This value must be set to the burst size of the source peripheral, or if the source is memory, to the memory boundary size. The burst size is the amount of data that is transferred when the DMACBREQ signal goes active in the source peripheral. 000 - 1 001 - 4 010 - 8 011 - 16 100 - 32 101 - 64 110 - 128 111 - 256"]
pub type SbsizeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DBSIZE` reader - Destination burst size. Indicates the number of transfers that make up a destination burst transfer request. This value must be set to the burst size of the destination peripheral or, if the destination is memory, to the memory boundary size. The burst size is the amount of data that is transferred when the DMACBREQ signal goes active in the destination peripheral. 000 - 1 001 - 4 010 - 8 011 - 16 100 - 32 101 - 64 110 - 128 111 - 256"]
pub type DbsizeR = crate::FieldReader;
#[doc = "Field `DBSIZE` writer - Destination burst size. Indicates the number of transfers that make up a destination burst transfer request. This value must be set to the burst size of the destination peripheral or, if the destination is memory, to the memory boundary size. The burst size is the amount of data that is transferred when the DMACBREQ signal goes active in the destination peripheral. 000 - 1 001 - 4 010 - 8 011 - 16 100 - 32 101 - 64 110 - 128 111 - 256"]
pub type DbsizeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SWIDTH` reader - Source transfer width. The source and destination widths can be different from each other. The hardware automatically packs and unpacks the data as required. 000 - Byte (8-bit) 001 - Halfword (16-bit) 010 - Word (32-bit) 011 to 111 - Reserved"]
pub type SwidthR = crate::FieldReader;
#[doc = "Field `SWIDTH` writer - Source transfer width. The source and destination widths can be different from each other. The hardware automatically packs and unpacks the data as required. 000 - Byte (8-bit) 001 - Halfword (16-bit) 010 - Word (32-bit) 011 to 111 - Reserved"]
pub type SwidthW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DWIDTH` reader - Destination transfer width. The source and destination widths can be different from each other. The hardware automatically packs and unpacks the data as required. 000 - Byte (8-bit) 001 - Halfword (16-bit) 010 - Word (32-bit) 011 to 111 - Reserved"]
pub type DwidthR = crate::FieldReader;
#[doc = "Field `DWIDTH` writer - Destination transfer width. The source and destination widths can be different from each other. The hardware automatically packs and unpacks the data as required. 000 - Byte (8-bit) 001 - Halfword (16-bit) 010 - Word (32-bit) 011 to 111 - Reserved"]
pub type DwidthW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SI` reader - Source increment: 0 - the source address is not incremented after each transfer. 1 - the source address is incremented after each transfer."]
pub type SiR = crate::BitReader;
#[doc = "Field `SI` writer - Source increment: 0 - the source address is not incremented after each transfer. 1 - the source address is incremented after each transfer."]
pub type SiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DI` reader - Destination increment: 0 - the destination address is not incremented after each transfer. 1 - the destination address is incremented after each transfer."]
pub type DiR = crate::BitReader;
#[doc = "Field `DI` writer - Destination increment: 0 - the destination address is not incremented after each transfer. 1 - the destination address is incremented after each transfer."]
pub type DiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROT1` reader - This is provided to the peripheral during a DMA bus access and indicates that the access is in user mode or privileged mode. This information is not used in the LPC178x/177x. 0 - access is in user mode. 1 - access is in privileged mode."]
pub type Prot1R = crate::BitReader;
#[doc = "Field `PROT1` writer - This is provided to the peripheral during a DMA bus access and indicates that the access is in user mode or privileged mode. This information is not used in the LPC178x/177x. 0 - access is in user mode. 1 - access is in privileged mode."]
pub type Prot1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROT2` reader - This is provided to the peripheral during a DMA bus access and indicates to the peripheral that the access is bufferable or not bufferable. This information is not used in the LPC178x/177x. 0 - access is not bufferable. 1 - access is bufferable."]
pub type Prot2R = crate::BitReader;
#[doc = "Field `PROT2` writer - This is provided to the peripheral during a DMA bus access and indicates to the peripheral that the access is bufferable or not bufferable. This information is not used in the LPC178x/177x. 0 - access is not bufferable. 1 - access is bufferable."]
pub type Prot2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROT3` reader - This is provided to the peripheral during a DMA bus access and indicates to the peripheral that the access is cacheable or not cacheable. This information is not used in the LPC178x/177x. 0 - access is not cacheable. 1 - access is cacheable."]
pub type Prot3R = crate::BitReader;
#[doc = "Field `PROT3` writer - This is provided to the peripheral during a DMA bus access and indicates to the peripheral that the access is cacheable or not cacheable. This information is not used in the LPC178x/177x. 0 - access is not cacheable. 1 - access is cacheable."]
pub type Prot3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I` reader - Terminal count interrupt enable bit. 0 - the terminal count interrupt is disabled. 1 - the terminal count interrupt is enabled."]
pub type IR = crate::BitReader;
#[doc = "Field `I` writer - Terminal count interrupt enable bit. 0 - the terminal count interrupt is disabled. 1 - the terminal count interrupt is enabled."]
pub type IW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - Transfer size. This field sets the size of the transfer when the DMA controller is the flow controller, in which case the value must be set before the channel is enabled. Transfer size is updated as data transfers are completed. A read from this field indicates the number of transfers completed on the destination bus. Reading the register when the channel is active does not give useful information because by the time that the software has processed the value read, the channel might have progressed. It is intended to be used only when a channel is enabled and then disabled. The transfer size value is not used if a peripheral is the flow controller."]
    #[inline(always)]
    pub fn transfersize(&self) -> TransfersizeR {
        TransfersizeR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:14 - Source burst size. Indicates the number of transfers that make up a source burst. This value must be set to the burst size of the source peripheral, or if the source is memory, to the memory boundary size. The burst size is the amount of data that is transferred when the DMACBREQ signal goes active in the source peripheral. 000 - 1 001 - 4 010 - 8 011 - 16 100 - 32 101 - 64 110 - 128 111 - 256"]
    #[inline(always)]
    pub fn sbsize(&self) -> SbsizeR {
        SbsizeR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - Destination burst size. Indicates the number of transfers that make up a destination burst transfer request. This value must be set to the burst size of the destination peripheral or, if the destination is memory, to the memory boundary size. The burst size is the amount of data that is transferred when the DMACBREQ signal goes active in the destination peripheral. 000 - 1 001 - 4 010 - 8 011 - 16 100 - 32 101 - 64 110 - 128 111 - 256"]
    #[inline(always)]
    pub fn dbsize(&self) -> DbsizeR {
        DbsizeR::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - Source transfer width. The source and destination widths can be different from each other. The hardware automatically packs and unpacks the data as required. 000 - Byte (8-bit) 001 - Halfword (16-bit) 010 - Word (32-bit) 011 to 111 - Reserved"]
    #[inline(always)]
    pub fn swidth(&self) -> SwidthR {
        SwidthR::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - Destination transfer width. The source and destination widths can be different from each other. The hardware automatically packs and unpacks the data as required. 000 - Byte (8-bit) 001 - Halfword (16-bit) 010 - Word (32-bit) 011 to 111 - Reserved"]
    #[inline(always)]
    pub fn dwidth(&self) -> DwidthR {
        DwidthR::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bit 26 - Source increment: 0 - the source address is not incremented after each transfer. 1 - the source address is incremented after each transfer."]
    #[inline(always)]
    pub fn si(&self) -> SiR {
        SiR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Destination increment: 0 - the destination address is not incremented after each transfer. 1 - the destination address is incremented after each transfer."]
    #[inline(always)]
    pub fn di(&self) -> DiR {
        DiR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - This is provided to the peripheral during a DMA bus access and indicates that the access is in user mode or privileged mode. This information is not used in the LPC178x/177x. 0 - access is in user mode. 1 - access is in privileged mode."]
    #[inline(always)]
    pub fn prot1(&self) -> Prot1R {
        Prot1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - This is provided to the peripheral during a DMA bus access and indicates to the peripheral that the access is bufferable or not bufferable. This information is not used in the LPC178x/177x. 0 - access is not bufferable. 1 - access is bufferable."]
    #[inline(always)]
    pub fn prot2(&self) -> Prot2R {
        Prot2R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - This is provided to the peripheral during a DMA bus access and indicates to the peripheral that the access is cacheable or not cacheable. This information is not used in the LPC178x/177x. 0 - access is not cacheable. 1 - access is cacheable."]
    #[inline(always)]
    pub fn prot3(&self) -> Prot3R {
        Prot3R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Terminal count interrupt enable bit. 0 - the terminal count interrupt is disabled. 1 - the terminal count interrupt is enabled."]
    #[inline(always)]
    pub fn i(&self) -> IR {
        IR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Transfer size. This field sets the size of the transfer when the DMA controller is the flow controller, in which case the value must be set before the channel is enabled. Transfer size is updated as data transfers are completed. A read from this field indicates the number of transfers completed on the destination bus. Reading the register when the channel is active does not give useful information because by the time that the software has processed the value read, the channel might have progressed. It is intended to be used only when a channel is enabled and then disabled. The transfer size value is not used if a peripheral is the flow controller."]
    #[inline(always)]
    #[must_use]
    pub fn transfersize(&mut self) -> TransfersizeW<ControlSpec> {
        TransfersizeW::new(self, 0)
    }
    #[doc = "Bits 12:14 - Source burst size. Indicates the number of transfers that make up a source burst. This value must be set to the burst size of the source peripheral, or if the source is memory, to the memory boundary size. The burst size is the amount of data that is transferred when the DMACBREQ signal goes active in the source peripheral. 000 - 1 001 - 4 010 - 8 011 - 16 100 - 32 101 - 64 110 - 128 111 - 256"]
    #[inline(always)]
    #[must_use]
    pub fn sbsize(&mut self) -> SbsizeW<ControlSpec> {
        SbsizeW::new(self, 12)
    }
    #[doc = "Bits 15:17 - Destination burst size. Indicates the number of transfers that make up a destination burst transfer request. This value must be set to the burst size of the destination peripheral or, if the destination is memory, to the memory boundary size. The burst size is the amount of data that is transferred when the DMACBREQ signal goes active in the destination peripheral. 000 - 1 001 - 4 010 - 8 011 - 16 100 - 32 101 - 64 110 - 128 111 - 256"]
    #[inline(always)]
    #[must_use]
    pub fn dbsize(&mut self) -> DbsizeW<ControlSpec> {
        DbsizeW::new(self, 15)
    }
    #[doc = "Bits 18:20 - Source transfer width. The source and destination widths can be different from each other. The hardware automatically packs and unpacks the data as required. 000 - Byte (8-bit) 001 - Halfword (16-bit) 010 - Word (32-bit) 011 to 111 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn swidth(&mut self) -> SwidthW<ControlSpec> {
        SwidthW::new(self, 18)
    }
    #[doc = "Bits 21:23 - Destination transfer width. The source and destination widths can be different from each other. The hardware automatically packs and unpacks the data as required. 000 - Byte (8-bit) 001 - Halfword (16-bit) 010 - Word (32-bit) 011 to 111 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn dwidth(&mut self) -> DwidthW<ControlSpec> {
        DwidthW::new(self, 21)
    }
    #[doc = "Bit 26 - Source increment: 0 - the source address is not incremented after each transfer. 1 - the source address is incremented after each transfer."]
    #[inline(always)]
    #[must_use]
    pub fn si(&mut self) -> SiW<ControlSpec> {
        SiW::new(self, 26)
    }
    #[doc = "Bit 27 - Destination increment: 0 - the destination address is not incremented after each transfer. 1 - the destination address is incremented after each transfer."]
    #[inline(always)]
    #[must_use]
    pub fn di(&mut self) -> DiW<ControlSpec> {
        DiW::new(self, 27)
    }
    #[doc = "Bit 28 - This is provided to the peripheral during a DMA bus access and indicates that the access is in user mode or privileged mode. This information is not used in the LPC178x/177x. 0 - access is in user mode. 1 - access is in privileged mode."]
    #[inline(always)]
    #[must_use]
    pub fn prot1(&mut self) -> Prot1W<ControlSpec> {
        Prot1W::new(self, 28)
    }
    #[doc = "Bit 29 - This is provided to the peripheral during a DMA bus access and indicates to the peripheral that the access is bufferable or not bufferable. This information is not used in the LPC178x/177x. 0 - access is not bufferable. 1 - access is bufferable."]
    #[inline(always)]
    #[must_use]
    pub fn prot2(&mut self) -> Prot2W<ControlSpec> {
        Prot2W::new(self, 29)
    }
    #[doc = "Bit 30 - This is provided to the peripheral during a DMA bus access and indicates to the peripheral that the access is cacheable or not cacheable. This information is not used in the LPC178x/177x. 0 - access is not cacheable. 1 - access is cacheable."]
    #[inline(always)]
    #[must_use]
    pub fn prot3(&mut self) -> Prot3W<ControlSpec> {
        Prot3W::new(self, 30)
    }
    #[doc = "Bit 31 - Terminal count interrupt enable bit. 0 - the terminal count interrupt is disabled. 1 - the terminal count interrupt is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn i(&mut self) -> IW<ControlSpec> {
        IW::new(self, 31)
    }
}
#[doc = "DMA Channel 0 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ControlSpec;
impl crate::RegisterSpec for ControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`control::R`](R) reader structure"]
impl crate::Readable for ControlSpec {}
#[doc = "`write(|w| ..)` method takes [`control::W`](W) writer structure"]
impl crate::Writable for ControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONTROL%s to value 0"]
impl crate::Resettable for ControlSpec {
    const RESET_VALUE: u32 = 0;
}
