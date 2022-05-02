#[doc = "Register `PRIVILEGE_MODE_SEL` reader"]
pub struct R(crate::R<PRIVILEGE_MODE_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRIVILEGE_MODE_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRIVILEGE_MODE_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRIVILEGE_MODE_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRIVILEGE_MODE_SEL` writer"]
pub struct W(crate::W<PRIVILEGE_MODE_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRIVILEGE_MODE_SEL_SPEC>;
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
impl From<crate::W<PRIVILEGE_MODE_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRIVILEGE_MODE_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRIVILEGE_MODE_SEL` reader - privilege_mode_sel"]
pub struct PRIVILEGE_MODE_SEL_R(crate::FieldReader<bool>);
impl PRIVILEGE_MODE_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRIVILEGE_MODE_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRIVILEGE_MODE_SEL_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRIVILEGE_MODE_SEL` writer - privilege_mode_sel"]
pub struct PRIVILEGE_MODE_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIVILEGE_MODE_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - privilege_mode_sel"]
    #[inline(always)]
    pub fn privilege_mode_sel(&self) -> PRIVILEGE_MODE_SEL_R {
        PRIVILEGE_MODE_SEL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - privilege_mode_sel"]
    #[inline(always)]
    pub fn privilege_mode_sel(&mut self) -> PRIVILEGE_MODE_SEL_W {
        PRIVILEGE_MODE_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SENSITIVE_PRIVILEGE_MODE_SEL_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [privilege_mode_sel](index.html) module"]
pub struct PRIVILEGE_MODE_SEL_SPEC;
impl crate::RegisterSpec for PRIVILEGE_MODE_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [privilege_mode_sel::R](R) reader structure"]
impl crate::Readable for PRIVILEGE_MODE_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [privilege_mode_sel::W](W) writer structure"]
impl crate::Writable for PRIVILEGE_MODE_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRIVILEGE_MODE_SEL to value 0"]
impl crate::Resettable for PRIVILEGE_MODE_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
