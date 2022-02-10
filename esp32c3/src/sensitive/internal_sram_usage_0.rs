#[doc = "Register `INTERNAL_SRAM_USAGE_0` reader"]
pub struct R(crate::R<INTERNAL_SRAM_USAGE_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTERNAL_SRAM_USAGE_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTERNAL_SRAM_USAGE_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTERNAL_SRAM_USAGE_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTERNAL_SRAM_USAGE_0` writer"]
pub struct W(crate::W<INTERNAL_SRAM_USAGE_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTERNAL_SRAM_USAGE_0_SPEC>;
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
impl From<crate::W<INTERNAL_SRAM_USAGE_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTERNAL_SRAM_USAGE_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTERNAL_SRAM_USAGE_LOCK` reader - internal_sram_usage_lock"]
pub struct INTERNAL_SRAM_USAGE_LOCK_R(crate::FieldReader<bool, bool>);
impl INTERNAL_SRAM_USAGE_LOCK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTERNAL_SRAM_USAGE_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTERNAL_SRAM_USAGE_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTERNAL_SRAM_USAGE_LOCK` writer - internal_sram_usage_lock"]
pub struct INTERNAL_SRAM_USAGE_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> INTERNAL_SRAM_USAGE_LOCK_W<'a> {
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
    #[doc = "Bit 0 - internal_sram_usage_lock"]
    #[inline(always)]
    pub fn internal_sram_usage_lock(&self) -> INTERNAL_SRAM_USAGE_LOCK_R {
        INTERNAL_SRAM_USAGE_LOCK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - internal_sram_usage_lock"]
    #[inline(always)]
    pub fn internal_sram_usage_lock(&mut self) -> INTERNAL_SRAM_USAGE_LOCK_W {
        INTERNAL_SRAM_USAGE_LOCK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SENSITIVE_INTERNAL_SRAM_USAGE_0_REG\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [internal_sram_usage_0]
(index.html) module"]
pub struct INTERNAL_SRAM_USAGE_0_SPEC;
impl crate::RegisterSpec for INTERNAL_SRAM_USAGE_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [internal_sram_usage_0::R]
(R) reader structure"]
impl crate::Readable for INTERNAL_SRAM_USAGE_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [internal_sram_usage_0::W]
(W) writer structure"]
impl crate::Writable for INTERNAL_SRAM_USAGE_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTERNAL_SRAM_USAGE_0 to value 0"]
impl crate::Resettable for INTERNAL_SRAM_USAGE_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
