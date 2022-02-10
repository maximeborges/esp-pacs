#[doc = "Register `T_STRING` reader"]
pub struct R(crate::R<T_STRING_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<T_STRING_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<T_STRING_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<T_STRING_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `T_STRING` writer"]
pub struct W(crate::W<T_STRING_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<T_STRING_SPEC>;
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
impl From<crate::W<T_STRING_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<T_STRING_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T_STRING` reader - Sha t_string (used if and only if mode == SHA_512/t)."]
pub struct T_STRING_R(crate::FieldReader<u32, u32>);
impl T_STRING_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        T_STRING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T_STRING_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T_STRING` writer - Sha t_string (used if and only if mode == SHA_512/t)."]
pub struct T_STRING_W<'a> {
    w: &'a mut W,
}
impl<'a> T_STRING_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Sha t_string (used if and only if mode == SHA_512/t)."]
    #[inline(always)]
    pub fn t_string(&self) -> T_STRING_R {
        T_STRING_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Sha t_string (used if and only if mode == SHA_512/t)."]
    #[inline(always)]
    pub fn t_string(&mut self) -> T_STRING_W {
        T_STRING_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SHA 512/t configuration register 0.\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t_string]
(index.html) module"]
pub struct T_STRING_SPEC;
impl crate::RegisterSpec for T_STRING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [t_string::R]
(R) reader structure"]
impl crate::Readable for T_STRING_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [t_string::W]
(W) writer structure"]
impl crate::Writable for T_STRING_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets T_STRING to value 0"]
impl crate::Resettable for T_STRING_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
