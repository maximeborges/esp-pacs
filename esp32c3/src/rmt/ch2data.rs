#[doc = "Register `CH2DATA` reader"]
pub struct R(crate::R<CH2DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH2DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH2DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH2DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH2DATA` writer"]
pub struct W(crate::W<CH2DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH2DATA_SPEC>;
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
impl From<crate::W<CH2DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH2DATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - Reserved."]
pub struct DATA_R(crate::FieldReader<u32>);
impl DATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA` writer - Reserved."]
pub struct DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Reserved."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Reserved."]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W {
        DATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RMT_CH2DATA_REG.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2data](index.html) module"]
pub struct CH2DATA_SPEC;
impl crate::RegisterSpec for CH2DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch2data::R](R) reader structure"]
impl crate::Readable for CH2DATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch2data::W](W) writer structure"]
impl crate::Writable for CH2DATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CH2DATA to value 0"]
impl crate::Resettable for CH2DATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
