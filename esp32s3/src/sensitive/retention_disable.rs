#[doc = "Register `RETENTION_DISABLE` reader"]
pub struct R(crate::R<RETENTION_DISABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RETENTION_DISABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RETENTION_DISABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RETENTION_DISABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RETENTION_DISABLE` writer"]
pub struct W(crate::W<RETENTION_DISABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RETENTION_DISABLE_SPEC>;
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
impl From<crate::W<RETENTION_DISABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RETENTION_DISABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RETENTION_DISABLE` reader - Set 1 to disable retention function and lock disable state."]
pub struct RETENTION_DISABLE_R(crate::FieldReader<bool, bool>);
impl RETENTION_DISABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RETENTION_DISABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RETENTION_DISABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RETENTION_DISABLE` writer - Set 1 to disable retention function and lock disable state."]
pub struct RETENTION_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> RETENTION_DISABLE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Set 1 to disable retention function and lock disable state."]
    #[inline(always)]
    pub fn retention_disable(&self) -> RETENTION_DISABLE_R {
        RETENTION_DISABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to disable retention function and lock disable state."]
    #[inline(always)]
    pub fn retention_disable(&mut self) -> RETENTION_DISABLE_W {
        RETENTION_DISABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Retention configuration register.\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [retention_disable]
(index.html) module"]
pub struct RETENTION_DISABLE_SPEC;
impl crate::RegisterSpec for RETENTION_DISABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [retention_disable::R]
(R) reader structure"]
impl crate::Readable for RETENTION_DISABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [retention_disable::W]
(W) writer structure"]
impl crate::Writable for RETENTION_DISABLE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RETENTION_DISABLE to value 0"]
impl crate::Resettable for RETENTION_DISABLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
