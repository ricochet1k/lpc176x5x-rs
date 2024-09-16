#[doc = "Register `PINSEL7` reader"]
pub type R = crate::R<Pinsel7Spec>;
#[doc = "Register `PINSEL7` writer"]
pub type W = crate::W<Pinsel7Spec>;
#[doc = "Pin function select P3.25.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P3_25 {
    #[doc = "0: GPIO P3.25"]
    GpioP3 = 0,
    #[doc = "2: MAT0.0"]
    Mat0 = 2,
    #[doc = "3: PWM1.2"]
    Pwm1 = 3,
}
impl From<P3_25> for u8 {
    #[inline(always)]
    fn from(variant: P3_25) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P3_25 {
    type Ux = u8;
}
impl crate::IsEnum for P3_25 {}
#[doc = "Field `P3_25` reader - Pin function select P3.25."]
pub type P3_25R = crate::FieldReader<P3_25>;
impl P3_25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P3_25 {
        match self.bits {
            0 => P3_25::GpioP3,
            2 => P3_25::Mat0,
            3 => P3_25::Pwm1,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P3.25"]
    #[inline(always)]
    pub fn is_gpio_p3(&self) -> bool {
        *self == P3_25::GpioP3
    }
    #[doc = "MAT0.0"]
    #[inline(always)]
    pub fn is_mat0(&self) -> bool {
        *self == P3_25::Mat0
    }
    #[doc = "PWM1.2"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == P3_25::Pwm1
    }
}
#[doc = "Field `P3_25` writer - Pin function select P3.25."]
pub type P3_25W<'a, REG> = crate::FieldWriter<'a, REG, 2, P3_25>;
impl<'a, REG> P3_25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P3.25"]
    #[inline(always)]
    pub fn gpio_p3(self) -> &'a mut crate::W<REG> {
        self.variant(P3_25::GpioP3)
    }
    #[doc = "MAT0.0"]
    #[inline(always)]
    pub fn mat0(self) -> &'a mut crate::W<REG> {
        self.variant(P3_25::Mat0)
    }
    #[doc = "PWM1.2"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut crate::W<REG> {
        self.variant(P3_25::Pwm1)
    }
}
#[doc = "Pin function select P3.26.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P3_26 {
    #[doc = "0: GPIO P3.26"]
    GpioP3 = 0,
    #[doc = "1: STCLK"]
    Stclk = 1,
    #[doc = "2: MAT0.1"]
    Mat0 = 2,
    #[doc = "3: PWM1.3"]
    Pwm1 = 3,
}
impl From<P3_26> for u8 {
    #[inline(always)]
    fn from(variant: P3_26) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P3_26 {
    type Ux = u8;
}
impl crate::IsEnum for P3_26 {}
#[doc = "Field `P3_26` reader - Pin function select P3.26."]
pub type P3_26R = crate::FieldReader<P3_26>;
impl P3_26R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P3_26 {
        match self.bits {
            0 => P3_26::GpioP3,
            1 => P3_26::Stclk,
            2 => P3_26::Mat0,
            3 => P3_26::Pwm1,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P3.26"]
    #[inline(always)]
    pub fn is_gpio_p3(&self) -> bool {
        *self == P3_26::GpioP3
    }
    #[doc = "STCLK"]
    #[inline(always)]
    pub fn is_stclk(&self) -> bool {
        *self == P3_26::Stclk
    }
    #[doc = "MAT0.1"]
    #[inline(always)]
    pub fn is_mat0(&self) -> bool {
        *self == P3_26::Mat0
    }
    #[doc = "PWM1.3"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == P3_26::Pwm1
    }
}
#[doc = "Field `P3_26` writer - Pin function select P3.26."]
pub type P3_26W<'a, REG> = crate::FieldWriter<'a, REG, 2, P3_26, crate::Safe>;
impl<'a, REG> P3_26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P3.26"]
    #[inline(always)]
    pub fn gpio_p3(self) -> &'a mut crate::W<REG> {
        self.variant(P3_26::GpioP3)
    }
    #[doc = "STCLK"]
    #[inline(always)]
    pub fn stclk(self) -> &'a mut crate::W<REG> {
        self.variant(P3_26::Stclk)
    }
    #[doc = "MAT0.1"]
    #[inline(always)]
    pub fn mat0(self) -> &'a mut crate::W<REG> {
        self.variant(P3_26::Mat0)
    }
    #[doc = "PWM1.3"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut crate::W<REG> {
        self.variant(P3_26::Pwm1)
    }
}
impl R {
    #[doc = "Bits 18:19 - Pin function select P3.25."]
    #[inline(always)]
    pub fn p3_25(&self) -> P3_25R {
        P3_25R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Pin function select P3.26."]
    #[inline(always)]
    pub fn p3_26(&self) -> P3_26R {
        P3_26R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 18:19 - Pin function select P3.25."]
    #[inline(always)]
    #[must_use]
    pub fn p3_25(&mut self) -> P3_25W<Pinsel7Spec> {
        P3_25W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Pin function select P3.26."]
    #[inline(always)]
    #[must_use]
    pub fn p3_26(&mut self) -> P3_26W<Pinsel7Spec> {
        P3_26W::new(self, 20)
    }
}
#[doc = "Pin function select register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`pinsel7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pinsel7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pinsel7Spec;
impl crate::RegisterSpec for Pinsel7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pinsel7::R`](R) reader structure"]
impl crate::Readable for Pinsel7Spec {}
#[doc = "`write(|w| ..)` method takes [`pinsel7::W`](W) writer structure"]
impl crate::Writable for Pinsel7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PINSEL7 to value 0"]
impl crate::Resettable for Pinsel7Spec {
    const RESET_VALUE: u32 = 0;
}
