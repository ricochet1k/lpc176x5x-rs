#[doc = "Register `DEVINTPRI` writer"]
pub type W = crate::W<DevintpriSpec>;
#[doc = "Frame interrupt routing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Frame {
    #[doc = "0: FRAME interrupt is routed to USB_INT_REQ_LP."]
    Lp = 0,
    #[doc = "1: FRAME interrupt is routed to USB_INT_REQ_HP."]
    Hp = 1,
}
impl From<Frame> for bool {
    #[inline(always)]
    fn from(variant: Frame) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRAME` writer - Frame interrupt routing"]
pub type FrameW<'a, REG> = crate::BitWriter<'a, REG, Frame>;
impl<'a, REG> FrameW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FRAME interrupt is routed to USB_INT_REQ_LP."]
    #[inline(always)]
    pub fn lp(self) -> &'a mut crate::W<REG> {
        self.variant(Frame::Lp)
    }
    #[doc = "FRAME interrupt is routed to USB_INT_REQ_HP."]
    #[inline(always)]
    pub fn hp(self) -> &'a mut crate::W<REG> {
        self.variant(Frame::Hp)
    }
}
#[doc = "Fast endpoint interrupt routing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EpFast {
    #[doc = "0: EP_FAST interrupt is routed to USB_INT_REQ_LP."]
    Lp = 0,
    #[doc = "1: EP_FAST interrupt is routed to USB_INT_REQ_HP."]
    Hp = 1,
}
impl From<EpFast> for bool {
    #[inline(always)]
    fn from(variant: EpFast) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP_FAST` writer - Fast endpoint interrupt routing"]
pub type EpFastW<'a, REG> = crate::BitWriter<'a, REG, EpFast>;
impl<'a, REG> EpFastW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "EP_FAST interrupt is routed to USB_INT_REQ_LP."]
    #[inline(always)]
    pub fn lp(self) -> &'a mut crate::W<REG> {
        self.variant(EpFast::Lp)
    }
    #[doc = "EP_FAST interrupt is routed to USB_INT_REQ_HP."]
    #[inline(always)]
    pub fn hp(self) -> &'a mut crate::W<REG> {
        self.variant(EpFast::Hp)
    }
}
impl W {
    #[doc = "Bit 0 - Frame interrupt routing"]
    #[inline(always)]
    #[must_use]
    pub fn frame(&mut self) -> FrameW<DevintpriSpec> {
        FrameW::new(self, 0)
    }
    #[doc = "Bit 1 - Fast endpoint interrupt routing"]
    #[inline(always)]
    #[must_use]
    pub fn ep_fast(&mut self) -> EpFastW<DevintpriSpec> {
        EpFastW::new(self, 1)
    }
}
#[doc = "USB Device Interrupt Priority\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devintpri::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevintpriSpec;
impl crate::RegisterSpec for DevintpriSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`devintpri::W`](W) writer structure"]
impl crate::Writable for DevintpriSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEVINTPRI to value 0"]
impl crate::Resettable for DevintpriSpec {
    const RESET_VALUE: u32 = 0;
}
