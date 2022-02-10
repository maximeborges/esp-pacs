#[doc = "Register `TEXT_IN_%s` reader"]
pub struct R(crate::R<TEXT_IN__SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TEXT_IN__SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TEXT_IN__SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TEXT_IN__SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TEXT_IN_%s` writer"]
pub struct W(crate::W<TEXT_IN__SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TEXT_IN__SPEC>;
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
impl From<crate::W<TEXT_IN__SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TEXT_IN__SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TEXT_IN_0` reader - Stores the source data when the AES Accelerator operates in the Typical AES working mode."]
pub struct TEXT_IN_0_R(crate::FieldReader<u32, u32>);
impl TEXT_IN_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        TEXT_IN_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEXT_IN_0_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEXT_IN_0` writer - Stores the source data when the AES Accelerator operates in the Typical AES working mode."]
pub struct TEXT_IN_0_W<'a> {
    w: &'a mut W,
}
impl<'a> TEXT_IN_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Stores the source data when the AES Accelerator operates in the Typical AES working mode."]
    #[inline(always)]
    pub fn text_in_0(&self) -> TEXT_IN_0_R {
        TEXT_IN_0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Stores the source data when the AES Accelerator operates in the Typical AES working mode."]
    #[inline(always)]
    pub fn text_in_0(&mut self) -> TEXT_IN_0_W {
        TEXT_IN_0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Source data register %s\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [text_in_]
(index.html) module"]
pub struct TEXT_IN__SPEC;
impl crate::RegisterSpec for TEXT_IN__SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [text_in_::R]
(R) reader structure"]
impl crate::Readable for TEXT_IN__SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [text_in_::W]
(W) writer structure"]
impl crate::Writable for TEXT_IN__SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TEXT_IN_%s to value 0"]
impl crate::Resettable for TEXT_IN__SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
