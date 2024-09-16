#[doc = "Register `LER` reader"]
pub type R = crate::R<LerSpec>;
#[doc = "Register `LER` writer"]
pub type W = crate::W<LerSpec>;
#[doc = "Field `MAT0LATCHEN` reader - Enable PWM Match 0 Latch. PWM MR0 register update control. Writing a one to this bit allows the last value written to the PWM Match Register 0 to be become effective when the timer is next reset by a PWM Match event. See Section 27.6.7."]
pub type Mat0latchenR = crate::BitReader;
#[doc = "Field `MAT0LATCHEN` writer - Enable PWM Match 0 Latch. PWM MR0 register update control. Writing a one to this bit allows the last value written to the PWM Match Register 0 to be become effective when the timer is next reset by a PWM Match event. See Section 27.6.7."]
pub type Mat0latchenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAT1LATCHEN` reader - Enable PWM Match 1 Latch. PWM MR1 register update control. See bit 0 for details."]
pub type Mat1latchenR = crate::BitReader;
#[doc = "Field `MAT1LATCHEN` writer - Enable PWM Match 1 Latch. PWM MR1 register update control. See bit 0 for details."]
pub type Mat1latchenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAT2LATCHEN` reader - Enable PWM Match 2 Latch. PWM MR2 register update control. See bit 0 for details."]
pub type Mat2latchenR = crate::BitReader;
#[doc = "Field `MAT2LATCHEN` writer - Enable PWM Match 2 Latch. PWM MR2 register update control. See bit 0 for details."]
pub type Mat2latchenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAT3LATCHEN` reader - Enable PWM Match 3 Latch. PWM MR3 register update control. See bit 0 for details."]
pub type Mat3latchenR = crate::BitReader;
#[doc = "Field `MAT3LATCHEN` writer - Enable PWM Match 3 Latch. PWM MR3 register update control. See bit 0 for details."]
pub type Mat3latchenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAT4LATCHEN` reader - Enable PWM Match 4 Latch. PWM MR4 register update control. See bit 0 for details."]
pub type Mat4latchenR = crate::BitReader;
#[doc = "Field `MAT4LATCHEN` writer - Enable PWM Match 4 Latch. PWM MR4 register update control. See bit 0 for details."]
pub type Mat4latchenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAT5LATCHEN` reader - Enable PWM Match 5 Latch. PWM MR5 register update control. See bit 0 for details."]
pub type Mat5latchenR = crate::BitReader;
#[doc = "Field `MAT5LATCHEN` writer - Enable PWM Match 5 Latch. PWM MR5 register update control. See bit 0 for details."]
pub type Mat5latchenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAT6LATCHEN` reader - Enable PWM Match 6 Latch. PWM MR6 register update control. See bit 0 for details."]
pub type Mat6latchenR = crate::BitReader;
#[doc = "Field `MAT6LATCHEN` writer - Enable PWM Match 6 Latch. PWM MR6 register update control. See bit 0 for details."]
pub type Mat6latchenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable PWM Match 0 Latch. PWM MR0 register update control. Writing a one to this bit allows the last value written to the PWM Match Register 0 to be become effective when the timer is next reset by a PWM Match event. See Section 27.6.7."]
    #[inline(always)]
    pub fn mat0latchen(&self) -> Mat0latchenR {
        Mat0latchenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable PWM Match 1 Latch. PWM MR1 register update control. See bit 0 for details."]
    #[inline(always)]
    pub fn mat1latchen(&self) -> Mat1latchenR {
        Mat1latchenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable PWM Match 2 Latch. PWM MR2 register update control. See bit 0 for details."]
    #[inline(always)]
    pub fn mat2latchen(&self) -> Mat2latchenR {
        Mat2latchenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable PWM Match 3 Latch. PWM MR3 register update control. See bit 0 for details."]
    #[inline(always)]
    pub fn mat3latchen(&self) -> Mat3latchenR {
        Mat3latchenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable PWM Match 4 Latch. PWM MR4 register update control. See bit 0 for details."]
    #[inline(always)]
    pub fn mat4latchen(&self) -> Mat4latchenR {
        Mat4latchenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable PWM Match 5 Latch. PWM MR5 register update control. See bit 0 for details."]
    #[inline(always)]
    pub fn mat5latchen(&self) -> Mat5latchenR {
        Mat5latchenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable PWM Match 6 Latch. PWM MR6 register update control. See bit 0 for details."]
    #[inline(always)]
    pub fn mat6latchen(&self) -> Mat6latchenR {
        Mat6latchenR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable PWM Match 0 Latch. PWM MR0 register update control. Writing a one to this bit allows the last value written to the PWM Match Register 0 to be become effective when the timer is next reset by a PWM Match event. See Section 27.6.7."]
    #[inline(always)]
    #[must_use]
    pub fn mat0latchen(&mut self) -> Mat0latchenW<LerSpec> {
        Mat0latchenW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable PWM Match 1 Latch. PWM MR1 register update control. See bit 0 for details."]
    #[inline(always)]
    #[must_use]
    pub fn mat1latchen(&mut self) -> Mat1latchenW<LerSpec> {
        Mat1latchenW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable PWM Match 2 Latch. PWM MR2 register update control. See bit 0 for details."]
    #[inline(always)]
    #[must_use]
    pub fn mat2latchen(&mut self) -> Mat2latchenW<LerSpec> {
        Mat2latchenW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable PWM Match 3 Latch. PWM MR3 register update control. See bit 0 for details."]
    #[inline(always)]
    #[must_use]
    pub fn mat3latchen(&mut self) -> Mat3latchenW<LerSpec> {
        Mat3latchenW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable PWM Match 4 Latch. PWM MR4 register update control. See bit 0 for details."]
    #[inline(always)]
    #[must_use]
    pub fn mat4latchen(&mut self) -> Mat4latchenW<LerSpec> {
        Mat4latchenW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable PWM Match 5 Latch. PWM MR5 register update control. See bit 0 for details."]
    #[inline(always)]
    #[must_use]
    pub fn mat5latchen(&mut self) -> Mat5latchenW<LerSpec> {
        Mat5latchenW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable PWM Match 6 Latch. PWM MR6 register update control. See bit 0 for details."]
    #[inline(always)]
    #[must_use]
    pub fn mat6latchen(&mut self) -> Mat6latchenW<LerSpec> {
        Mat6latchenW::new(self, 6)
    }
}
#[doc = "Load Enable Register. Enables use of updated PWM match values.\n\nYou can [`read`](crate::Reg::read) this register and get [`ler::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ler::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LerSpec;
impl crate::RegisterSpec for LerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ler::R`](R) reader structure"]
impl crate::Readable for LerSpec {}
#[doc = "`write(|w| ..)` method takes [`ler::W`](W) writer structure"]
impl crate::Writable for LerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LER to value 0"]
impl crate::Resettable for LerSpec {
    const RESET_VALUE: u32 = 0;
}
