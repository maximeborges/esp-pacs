#[doc = "Register `SEARCH_ENABLE` reader"]
pub struct R(crate::R<SEARCH_ENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEARCH_ENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEARCH_ENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEARCH_ENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEARCH_ENABLE` writer"]
pub struct W(crate::W<SEARCH_ENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEARCH_ENABLE_SPEC>;
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
impl From<crate::W<SEARCH_ENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEARCH_ENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEARCH_ENABLE` reader - Configure this bit to 1 for acceleration. 1: with acceleration, 0: without acceleration(default). This option should be used together with RSA_SEARCH_POS."]
pub struct SEARCH_ENABLE_R(crate::FieldReader<bool>);
impl SEARCH_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SEARCH_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEARCH_ENABLE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEARCH_ENABLE` writer - Configure this bit to 1 for acceleration. 1: with acceleration, 0: without acceleration(default). This option should be used together with RSA_SEARCH_POS."]
pub struct SEARCH_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SEARCH_ENABLE_W<'a> {
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
    #[doc = "Bit 0 - Configure this bit to 1 for acceleration. 1: with acceleration, 0: without acceleration(default). This option should be used together with RSA_SEARCH_POS."]
    #[inline(always)]
    pub fn search_enable(&self) -> SEARCH_ENABLE_R {
        SEARCH_ENABLE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Configure this bit to 1 for acceleration. 1: with acceleration, 0: without acceleration(default). This option should be used together with RSA_SEARCH_POS."]
    #[inline(always)]
    pub fn search_enable(&mut self) -> SEARCH_ENABLE_W {
        SEARCH_ENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RSA search option\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [search_enable](index.html) module"]
pub struct SEARCH_ENABLE_SPEC;
impl crate::RegisterSpec for SEARCH_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [search_enable::R](R) reader structure"]
impl crate::Readable for SEARCH_ENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [search_enable::W](W) writer structure"]
impl crate::Writable for SEARCH_ENABLE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SEARCH_ENABLE to value 0"]
impl crate::Resettable for SEARCH_ENABLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
