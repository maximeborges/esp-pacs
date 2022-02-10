#[doc = "Register `MODE` reader"]
pub struct R(crate::R<MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MODE` writer"]
pub struct W(crate::W<MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MODE_SPEC>;
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
impl From<crate::W<MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODE` reader - Defines the operation type of the AES Accelerator operating under the Typical AES working mode. & 0x0(AES_EN_128): AES-EN-128 # 0x1(AES_EN_192): AES-EN-192 # 0x2(AES_EN_256): AES-EN-256 # 0x4(AES_DE_128): AES-DE-128 # 0x5(AES_DE_192): AES-DE-192 # 0x6(AES_DE_256): AES-DE-256 &"]
pub struct MODE_R(crate::FieldReader<u8, u8>);
impl MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE` writer - Defines the operation type of the AES Accelerator operating under the Typical AES working mode. & 0x0(AES_EN_128): AES-EN-128 # 0x1(AES_EN_192): AES-EN-192 # 0x2(AES_EN_256): AES-EN-256 # 0x4(AES_DE_128): AES-DE-128 # 0x5(AES_DE_192): AES-DE-192 # 0x6(AES_DE_256): AES-DE-256 &"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Defines the operation type of the AES Accelerator operating under the Typical AES working mode. & 0x0(AES_EN_128): AES-EN-128 # 0x1(AES_EN_192): AES-EN-192 # 0x2(AES_EN_256): AES-EN-256 # 0x4(AES_DE_128): AES-DE-128 # 0x5(AES_DE_192): AES-DE-192 # 0x6(AES_DE_256): AES-DE-256 &"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Defines the operation type of the AES Accelerator operating under the Typical AES working mode. & 0x0(AES_EN_128): AES-EN-128 # 0x1(AES_EN_192): AES-EN-192 # 0x2(AES_EN_256): AES-EN-256 # 0x4(AES_DE_128): AES-DE-128 # 0x5(AES_DE_192): AES-DE-192 # 0x6(AES_DE_256): AES-DE-256 &"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AES working mode configuration register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode]
(index.html) module"]
pub struct MODE_SPEC;
impl crate::RegisterSpec for MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mode::R]
(R) reader structure"]
impl crate::Readable for MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mode::W]
(W) writer structure"]
impl crate::Writable for MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MODE to value 0"]
impl crate::Resettable for MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
