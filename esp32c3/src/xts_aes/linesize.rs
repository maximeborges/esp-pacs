#[doc = "Register `LINESIZE` reader"]
pub struct R(crate::R<LINESIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LINESIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LINESIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LINESIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LINESIZE` writer"]
pub struct W(crate::W<LINESIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LINESIZE_SPEC>;
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
impl From<crate::W<LINESIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LINESIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LINESIZE` reader - This bit stores the line size parameter. 0: 16Byte, 1: 32Byte."]
pub struct LINESIZE_R(crate::FieldReader<bool, bool>);
impl LINESIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LINESIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINESIZE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINESIZE` writer - This bit stores the line size parameter. 0: 16Byte, 1: 32Byte."]
pub struct LINESIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> LINESIZE_W<'a> {
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
    #[doc = "Bit 0 - This bit stores the line size parameter. 0: 16Byte, 1: 32Byte."]
    #[inline(always)]
    pub fn linesize(&self) -> LINESIZE_R {
        LINESIZE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit stores the line size parameter. 0: 16Byte, 1: 32Byte."]
    #[inline(always)]
    pub fn linesize(&mut self) -> LINESIZE_W {
        LINESIZE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "XTS-AES line-size register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [linesize]
(index.html) module"]
pub struct LINESIZE_SPEC;
impl crate::RegisterSpec for LINESIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [linesize::R]
(R) reader structure"]
impl crate::Readable for LINESIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [linesize::W]
(W) writer structure"]
impl crate::Writable for LINESIZE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LINESIZE to value 0"]
impl crate::Resettable for LINESIZE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
