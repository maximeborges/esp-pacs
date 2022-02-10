#[doc = "Register `SDIO_SELECT` reader"]
pub struct R(crate::R<SDIO_SELECT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDIO_SELECT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDIO_SELECT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDIO_SELECT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDIO_SELECT` writer"]
pub struct W(crate::W<SDIO_SELECT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDIO_SELECT_SPEC>;
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
impl From<crate::W<SDIO_SELECT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDIO_SELECT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDIO_SEL` reader - GPIO sdio select register"]
pub struct SDIO_SEL_R(crate::FieldReader<u8, u8>);
impl SDIO_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SDIO_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIO_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIO_SEL` writer - GPIO sdio select register"]
pub struct SDIO_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - GPIO sdio select register"]
    #[inline(always)]
    pub fn sdio_sel(&self) -> SDIO_SEL_R {
        SDIO_SEL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO sdio select register"]
    #[inline(always)]
    pub fn sdio_sel(&mut self) -> SDIO_SEL_W {
        SDIO_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO sdio select register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdio_select]
(index.html) module"]
pub struct SDIO_SELECT_SPEC;
impl crate::RegisterSpec for SDIO_SELECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdio_select::R]
(R) reader structure"]
impl crate::Readable for SDIO_SELECT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdio_select::W]
(W) writer structure"]
impl crate::Writable for SDIO_SELECT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SDIO_SELECT to value 0"]
impl crate::Resettable for SDIO_SELECT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
