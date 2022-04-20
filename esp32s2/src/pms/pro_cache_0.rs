#[doc = "Register `PRO_CACHE_0` reader"]
pub struct R(crate::R<PRO_CACHE_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_CACHE_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_CACHE_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_CACHE_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRO_CACHE_0` writer"]
pub struct W(crate::W<PRO_CACHE_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRO_CACHE_0_SPEC>;
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
impl From<crate::W<PRO_CACHE_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRO_CACHE_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRO_CACHE_LOCK` reader - Lock register. Setting to 1 locks cache permission control registers."]
pub struct PRO_CACHE_LOCK_R(crate::FieldReader<bool, bool>);
impl PRO_CACHE_LOCK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_CACHE_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_CACHE_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_CACHE_LOCK` writer - Lock register. Setting to 1 locks cache permission control registers."]
pub struct PRO_CACHE_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_CACHE_LOCK_W<'a> {
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
    #[doc = "Bit 0 - Lock register. Setting to 1 locks cache permission control registers."]
    #[inline(always)]
    pub fn pro_cache_lock(&self) -> PRO_CACHE_LOCK_R {
        PRO_CACHE_LOCK_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Lock register. Setting to 1 locks cache permission control registers."]
    #[inline(always)]
    pub fn pro_cache_lock(&mut self) -> PRO_CACHE_LOCK_W {
        PRO_CACHE_LOCK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cache permission control register 0.\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_cache_0]
(index.html) module"]
pub struct PRO_CACHE_0_SPEC;
impl crate::RegisterSpec for PRO_CACHE_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_cache_0::R]
(R) reader structure"]
impl crate::Readable for PRO_CACHE_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pro_cache_0::W]
(W) writer structure"]
impl crate::Writable for PRO_CACHE_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRO_CACHE_0 to value 0"]
impl crate::Resettable for PRO_CACHE_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
