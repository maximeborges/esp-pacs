#[doc = "Register `SIGMADELTA1` reader"]
pub struct R(crate::R<SIGMADELTA1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIGMADELTA1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIGMADELTA1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIGMADELTA1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SIGMADELTA1` writer"]
pub struct W(crate::W<SIGMADELTA1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIGMADELTA1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SIGMADELTA1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIGMADELTA1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_SD1_IN` reader - "]
pub type GPIO_SD1_IN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GPIO_SD1_IN` writer - "]
pub type GPIO_SD1_IN_W<'a> = crate::FieldWriter<'a, u32, SIGMADELTA1_SPEC, u8, u8, 8, 0>;
#[doc = "Field `GPIO_SD1_PRESCALE` reader - "]
pub type GPIO_SD1_PRESCALE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GPIO_SD1_PRESCALE` writer - "]
pub type GPIO_SD1_PRESCALE_W<'a> = crate::FieldWriter<'a, u32, SIGMADELTA1_SPEC, u8, u8, 8, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn gpio_sd1_in(&self) -> GPIO_SD1_IN_R {
        GPIO_SD1_IN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn gpio_sd1_prescale(&self) -> GPIO_SD1_PRESCALE_R {
        GPIO_SD1_PRESCALE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn gpio_sd1_in(&mut self) -> GPIO_SD1_IN_W {
        GPIO_SD1_IN_W::new(self)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn gpio_sd1_prescale(&mut self) -> GPIO_SD1_PRESCALE_W {
        GPIO_SD1_PRESCALE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sigmadelta1](index.html) module"]
pub struct SIGMADELTA1_SPEC;
impl crate::RegisterSpec for SIGMADELTA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sigmadelta1::R](R) reader structure"]
impl crate::Readable for SIGMADELTA1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sigmadelta1::W](W) writer structure"]
impl crate::Writable for SIGMADELTA1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SIGMADELTA1 to value 0xff00"]
impl crate::Resettable for SIGMADELTA1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff00
    }
}
