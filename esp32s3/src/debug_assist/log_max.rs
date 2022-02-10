#[doc = "Register `LOG_MAX` reader"]
pub struct R(crate::R<LOG_MAX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOG_MAX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOG_MAX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOG_MAX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LOG_MAX` writer"]
pub struct W(crate::W<LOG_MAX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LOG_MAX_SPEC>;
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
impl From<crate::W<LOG_MAX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LOG_MAX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOG_MAX` reader - check region max addr"]
pub struct LOG_MAX_R(crate::FieldReader<u32, u32>);
impl LOG_MAX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        LOG_MAX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOG_MAX_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOG_MAX` writer - check region max addr"]
pub struct LOG_MAX_W<'a> {
    w: &'a mut W,
}
impl<'a> LOG_MAX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - check region max addr"]
    #[inline(always)]
    pub fn log_max(&self) -> LOG_MAX_R {
        LOG_MAX_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - check region max addr"]
    #[inline(always)]
    pub fn log_max(&mut self) -> LOG_MAX_W {
        LOG_MAX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "log check region configuration register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [log_max]
(index.html) module"]
pub struct LOG_MAX_SPEC;
impl crate::RegisterSpec for LOG_MAX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [log_max::R]
(R) reader structure"]
impl crate::Readable for LOG_MAX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [log_max::W]
(W) writer structure"]
impl crate::Writable for LOG_MAX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LOG_MAX to value 0"]
impl crate::Resettable for LOG_MAX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
