#[doc = "Register `PRIVILEGE_MODE_SEL_LOCK` reader"]
pub struct R(crate::R<PRIVILEGE_MODE_SEL_LOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRIVILEGE_MODE_SEL_LOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRIVILEGE_MODE_SEL_LOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRIVILEGE_MODE_SEL_LOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRIVILEGE_MODE_SEL_LOCK` writer"]
pub struct W(crate::W<PRIVILEGE_MODE_SEL_LOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRIVILEGE_MODE_SEL_LOCK_SPEC>;
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
impl From<crate::W<PRIVILEGE_MODE_SEL_LOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRIVILEGE_MODE_SEL_LOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRIVILEGE_MODE_SEL_LOCK` reader - privilege_mode_sel_lock"]
pub struct PRIVILEGE_MODE_SEL_LOCK_R(crate::FieldReader<bool, bool>);
impl PRIVILEGE_MODE_SEL_LOCK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRIVILEGE_MODE_SEL_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRIVILEGE_MODE_SEL_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRIVILEGE_MODE_SEL_LOCK` writer - privilege_mode_sel_lock"]
pub struct PRIVILEGE_MODE_SEL_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIVILEGE_MODE_SEL_LOCK_W<'a> {
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
    #[doc = "Bit 0 - privilege_mode_sel_lock"]
    #[inline(always)]
    pub fn privilege_mode_sel_lock(&self) -> PRIVILEGE_MODE_SEL_LOCK_R {
        PRIVILEGE_MODE_SEL_LOCK_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - privilege_mode_sel_lock"]
    #[inline(always)]
    pub fn privilege_mode_sel_lock(&mut self) -> PRIVILEGE_MODE_SEL_LOCK_W {
        PRIVILEGE_MODE_SEL_LOCK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SENSITIVE_PRIVILEGE_MODE_SEL_LOCK_REG\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [privilege_mode_sel_lock]
(index.html) module"]
pub struct PRIVILEGE_MODE_SEL_LOCK_SPEC;
impl crate::RegisterSpec for PRIVILEGE_MODE_SEL_LOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [privilege_mode_sel_lock::R]
(R) reader structure"]
impl crate::Readable for PRIVILEGE_MODE_SEL_LOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [privilege_mode_sel_lock::W]
(W) writer structure"]
impl crate::Writable for PRIVILEGE_MODE_SEL_LOCK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRIVILEGE_MODE_SEL_LOCK to value 0"]
impl crate::Resettable for PRIVILEGE_MODE_SEL_LOCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
