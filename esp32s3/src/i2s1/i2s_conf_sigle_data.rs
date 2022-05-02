#[doc = "Register `I2S_CONF_SIGLE_DATA` reader"]
pub struct R(crate::R<I2S_CONF_SIGLE_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2S_CONF_SIGLE_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2S_CONF_SIGLE_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2S_CONF_SIGLE_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2S_CONF_SIGLE_DATA` writer"]
pub struct W(crate::W<I2S_CONF_SIGLE_DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2S_CONF_SIGLE_DATA_SPEC>;
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
impl From<crate::W<I2S_CONF_SIGLE_DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2S_CONF_SIGLE_DATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2S_SINGLE_DATA` reader - The configured constant channel data to be sent out."]
pub struct I2S_SINGLE_DATA_R(crate::FieldReader<u32>);
impl I2S_SINGLE_DATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        I2S_SINGLE_DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2S_SINGLE_DATA_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2S_SINGLE_DATA` writer - The configured constant channel data to be sent out."]
pub struct I2S_SINGLE_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_SINGLE_DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - The configured constant channel data to be sent out."]
    #[inline(always)]
    pub fn i2s_single_data(&self) -> I2S_SINGLE_DATA_R {
        I2S_SINGLE_DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The configured constant channel data to be sent out."]
    #[inline(always)]
    pub fn i2s_single_data(&mut self) -> I2S_SINGLE_DATA_W {
        I2S_SINGLE_DATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S signal data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_conf_sigle_data](index.html) module"]
pub struct I2S_CONF_SIGLE_DATA_SPEC;
impl crate::RegisterSpec for I2S_CONF_SIGLE_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2s_conf_sigle_data::R](R) reader structure"]
impl crate::Readable for I2S_CONF_SIGLE_DATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2s_conf_sigle_data::W](W) writer structure"]
impl crate::Writable for I2S_CONF_SIGLE_DATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2S_CONF_SIGLE_DATA to value 0"]
impl crate::Resettable for I2S_CONF_SIGLE_DATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
