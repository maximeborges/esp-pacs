#[doc = "Register `TOUCH_DAC1` reader"]
pub struct R(crate::R<TOUCH_DAC1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOUCH_DAC1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TOUCH_DAC1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TOUCH_DAC1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TOUCH_DAC1` writer"]
pub struct W(crate::W<TOUCH_DAC1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TOUCH_DAC1_SPEC>;
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
impl From<crate::W<TOUCH_DAC1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TOUCH_DAC1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOUCH_PAD14_DAC` reader - configure touch pad dac14"]
pub struct TOUCH_PAD14_DAC_R(crate::FieldReader<u8, u8>);
impl TOUCH_PAD14_DAC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TOUCH_PAD14_DAC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_PAD14_DAC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_PAD14_DAC` writer - configure touch pad dac14"]
pub struct TOUCH_PAD14_DAC_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_PAD14_DAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 17)) | ((value as u32 & 0x07) << 17);
        self.w
    }
}
#[doc = "Field `TOUCH_PAD13_DAC` reader - configure touch pad dac13"]
pub struct TOUCH_PAD13_DAC_R(crate::FieldReader<u8, u8>);
impl TOUCH_PAD13_DAC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TOUCH_PAD13_DAC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_PAD13_DAC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_PAD13_DAC` writer - configure touch pad dac13"]
pub struct TOUCH_PAD13_DAC_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_PAD13_DAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | ((value as u32 & 0x07) << 20);
        self.w
    }
}
#[doc = "Field `TOUCH_PAD12_DAC` reader - configure touch pad dac12"]
pub struct TOUCH_PAD12_DAC_R(crate::FieldReader<u8, u8>);
impl TOUCH_PAD12_DAC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TOUCH_PAD12_DAC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_PAD12_DAC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_PAD12_DAC` writer - configure touch pad dac12"]
pub struct TOUCH_PAD12_DAC_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_PAD12_DAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 23)) | ((value as u32 & 0x07) << 23);
        self.w
    }
}
#[doc = "Field `TOUCH_PAD11_DAC` reader - configure touch pad dac11"]
pub struct TOUCH_PAD11_DAC_R(crate::FieldReader<u8, u8>);
impl TOUCH_PAD11_DAC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TOUCH_PAD11_DAC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_PAD11_DAC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_PAD11_DAC` writer - configure touch pad dac11"]
pub struct TOUCH_PAD11_DAC_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_PAD11_DAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 26)) | ((value as u32 & 0x07) << 26);
        self.w
    }
}
#[doc = "Field `TOUCH_PAD10_DAC` reader - configure touch pad dac10"]
pub struct TOUCH_PAD10_DAC_R(crate::FieldReader<u8, u8>);
impl TOUCH_PAD10_DAC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TOUCH_PAD10_DAC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_PAD10_DAC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_PAD10_DAC` writer - configure touch pad dac10"]
pub struct TOUCH_PAD10_DAC_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_PAD10_DAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 29)) | ((value as u32 & 0x07) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bits 17:19 - configure touch pad dac14"]
    #[inline(always)]
    pub fn touch_pad14_dac(&self) -> TOUCH_PAD14_DAC_R {
        TOUCH_PAD14_DAC_R::new(((self.bits >> 17) & 0x07) as u8)
    }
    #[doc = "Bits 20:22 - configure touch pad dac13"]
    #[inline(always)]
    pub fn touch_pad13_dac(&self) -> TOUCH_PAD13_DAC_R {
        TOUCH_PAD13_DAC_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 23:25 - configure touch pad dac12"]
    #[inline(always)]
    pub fn touch_pad12_dac(&self) -> TOUCH_PAD12_DAC_R {
        TOUCH_PAD12_DAC_R::new(((self.bits >> 23) & 0x07) as u8)
    }
    #[doc = "Bits 26:28 - configure touch pad dac11"]
    #[inline(always)]
    pub fn touch_pad11_dac(&self) -> TOUCH_PAD11_DAC_R {
        TOUCH_PAD11_DAC_R::new(((self.bits >> 26) & 0x07) as u8)
    }
    #[doc = "Bits 29:31 - configure touch pad dac10"]
    #[inline(always)]
    pub fn touch_pad10_dac(&self) -> TOUCH_PAD10_DAC_R {
        TOUCH_PAD10_DAC_R::new(((self.bits >> 29) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 17:19 - configure touch pad dac14"]
    #[inline(always)]
    pub fn touch_pad14_dac(&mut self) -> TOUCH_PAD14_DAC_W {
        TOUCH_PAD14_DAC_W { w: self }
    }
    #[doc = "Bits 20:22 - configure touch pad dac13"]
    #[inline(always)]
    pub fn touch_pad13_dac(&mut self) -> TOUCH_PAD13_DAC_W {
        TOUCH_PAD13_DAC_W { w: self }
    }
    #[doc = "Bits 23:25 - configure touch pad dac12"]
    #[inline(always)]
    pub fn touch_pad12_dac(&mut self) -> TOUCH_PAD12_DAC_W {
        TOUCH_PAD12_DAC_W { w: self }
    }
    #[doc = "Bits 26:28 - configure touch pad dac11"]
    #[inline(always)]
    pub fn touch_pad11_dac(&mut self) -> TOUCH_PAD11_DAC_W {
        TOUCH_PAD11_DAC_W { w: self }
    }
    #[doc = "Bits 29:31 - configure touch pad dac10"]
    #[inline(always)]
    pub fn touch_pad10_dac(&mut self) -> TOUCH_PAD10_DAC_W {
        TOUCH_PAD10_DAC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure touch dac\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [touch_dac1]
(index.html) module"]
pub struct TOUCH_DAC1_SPEC;
impl crate::RegisterSpec for TOUCH_DAC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [touch_dac1::R]
(R) reader structure"]
impl crate::Readable for TOUCH_DAC1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [touch_dac1::W]
(W) writer structure"]
impl crate::Writable for TOUCH_DAC1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TOUCH_DAC1 to value 0"]
impl crate::Resettable for TOUCH_DAC1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
