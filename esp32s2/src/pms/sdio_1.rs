#[doc = "Register `SDIO_1` reader"]
pub struct R(crate::R<SDIO_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDIO_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDIO_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDIO_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDIO_1` writer"]
pub struct W(crate::W<SDIO_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDIO_1_SPEC>;
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
impl From<crate::W<SDIO_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDIO_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDIO_DISABLE` reader - Setting to 1 disables the SDIO function."]
pub struct SDIO_DISABLE_R(crate::FieldReader<bool, bool>);
impl SDIO_DISABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDIO_DISABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIO_DISABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIO_DISABLE` writer - Setting to 1 disables the SDIO function."]
pub struct SDIO_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_DISABLE_W<'a> {
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
    #[doc = "Bit 0 - Setting to 1 disables the SDIO function."]
    #[inline(always)]
    pub fn sdio_disable(&self) -> SDIO_DISABLE_R {
        SDIO_DISABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Setting to 1 disables the SDIO function."]
    #[inline(always)]
    pub fn sdio_disable(&mut self) -> SDIO_DISABLE_W {
        SDIO_DISABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDIO permission control register 1.\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdio_1]
(index.html) module"]
pub struct SDIO_1_SPEC;
impl crate::RegisterSpec for SDIO_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdio_1::R]
(R) reader structure"]
impl crate::Readable for SDIO_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdio_1::W]
(W) writer structure"]
impl crate::Writable for SDIO_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SDIO_1 to value 0"]
impl crate::Resettable for SDIO_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
