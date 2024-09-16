#[doc = "Register `RXRATE` reader"]
pub type R = crate::R<RxrateSpec>;
#[doc = "Register `RXRATE` writer"]
pub type W = crate::W<RxrateSpec>;
#[doc = "Field `Y_DIVIDER` reader - I2S receive MCLK rate denominator. This value is used to divide PCLK to produce the receive MCLK. Eight bits of fractional divide supports a wide range of possibilities. A value of 0 stops the clock."]
pub type YDividerR = crate::FieldReader;
#[doc = "Field `Y_DIVIDER` writer - I2S receive MCLK rate denominator. This value is used to divide PCLK to produce the receive MCLK. Eight bits of fractional divide supports a wide range of possibilities. A value of 0 stops the clock."]
pub type YDividerW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `X_DIVIDER` reader - I2S receive MCLK rate numerator. This value is used to multiply PCLK by to produce the receive MCLK. A value of 0 stops the clock. Eight bits of fractional divide supports a wide range of possibilities. Note: the resulting ratio X/Y is divided by 2."]
pub type XDividerR = crate::FieldReader;
#[doc = "Field `X_DIVIDER` writer - I2S receive MCLK rate numerator. This value is used to multiply PCLK by to produce the receive MCLK. A value of 0 stops the clock. Eight bits of fractional divide supports a wide range of possibilities. Note: the resulting ratio X/Y is divided by 2."]
pub type XDividerW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - I2S receive MCLK rate denominator. This value is used to divide PCLK to produce the receive MCLK. Eight bits of fractional divide supports a wide range of possibilities. A value of 0 stops the clock."]
    #[inline(always)]
    pub fn y_divider(&self) -> YDividerR {
        YDividerR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - I2S receive MCLK rate numerator. This value is used to multiply PCLK by to produce the receive MCLK. A value of 0 stops the clock. Eight bits of fractional divide supports a wide range of possibilities. Note: the resulting ratio X/Y is divided by 2."]
    #[inline(always)]
    pub fn x_divider(&self) -> XDividerR {
        XDividerR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - I2S receive MCLK rate denominator. This value is used to divide PCLK to produce the receive MCLK. Eight bits of fractional divide supports a wide range of possibilities. A value of 0 stops the clock."]
    #[inline(always)]
    #[must_use]
    pub fn y_divider(&mut self) -> YDividerW<RxrateSpec> {
        YDividerW::new(self, 0)
    }
    #[doc = "Bits 8:15 - I2S receive MCLK rate numerator. This value is used to multiply PCLK by to produce the receive MCLK. A value of 0 stops the clock. Eight bits of fractional divide supports a wide range of possibilities. Note: the resulting ratio X/Y is divided by 2."]
    #[inline(always)]
    #[must_use]
    pub fn x_divider(&mut self) -> XDividerW<RxrateSpec> {
        XDividerW::new(self, 8)
    }
}
#[doc = "I2S Receive MCLK divider. This register determines the I2S RX MCLK rate by specifying the value to divide PCLK by in order to produce MCLK.\n\nYou can [`read`](crate::Reg::read) this register and get [`rxrate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxrate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxrateSpec;
impl crate::RegisterSpec for RxrateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxrate::R`](R) reader structure"]
impl crate::Readable for RxrateSpec {}
#[doc = "`write(|w| ..)` method takes [`rxrate::W`](W) writer structure"]
impl crate::Writable for RxrateSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXRATE to value 0"]
impl crate::Resettable for RxrateSpec {
    const RESET_VALUE: u32 = 0;
}
