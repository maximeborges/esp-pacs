#[doc = "Register `TOUCH_DAC` reader"]
pub struct R(crate::R<TOUCH_DAC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOUCH_DAC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TOUCH_DAC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TOUCH_DAC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TOUCH_DAC` writer"]
pub struct W(crate::W<TOUCH_DAC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TOUCH_DAC_SPEC>;
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
impl From<crate::W<TOUCH_DAC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TOUCH_DAC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOUCH_PAD9_DAC` reader - configure touch pad dac9"]
pub struct TOUCH_PAD9_DAC_R(crate::FieldReader<u8, u8>);
impl TOUCH_PAD9_DAC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TOUCH_PAD9_DAC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_PAD9_DAC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_PAD9_DAC` writer - configure touch pad dac9"]
pub struct TOUCH_PAD9_DAC_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_PAD9_DAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | ((value as u32 & 0x07) << 2);
        self.w
    }
}
#[doc = "Field `TOUCH_PAD8_DAC` reader - configure touch pad dac8"]
pub struct TOUCH_PAD8_DAC_R(crate::FieldReader<u8, u8>);
impl TOUCH_PAD8_DAC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TOUCH_PAD8_DAC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_PAD8_DAC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_PAD8_DAC` writer - configure touch pad dac8"]
pub struct TOUCH_PAD8_DAC_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_PAD8_DAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | ((value as u32 & 0x07) << 5);
        self.w
    }
}
#[doc = "Field `TOUCH_PAD7_DAC` reader - configure touch pad dac7"]
pub struct TOUCH_PAD7_DAC_R(crate::FieldReader<u8, u8>);
impl TOUCH_PAD7_DAC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TOUCH_PAD7_DAC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_PAD7_DAC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_PAD7_DAC` writer - configure touch pad dac7"]
pub struct TOUCH_PAD7_DAC_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_PAD7_DAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `TOUCH_PAD6_DAC` reader - configure touch pad dac6"]
pub struct TOUCH_PAD6_DAC_R(crate::FieldReader<u8, u8>);
impl TOUCH_PAD6_DAC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TOUCH_PAD6_DAC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_PAD6_DAC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_PAD6_DAC` writer - configure touch pad dac6"]
pub struct TOUCH_PAD6_DAC_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_PAD6_DAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | ((value as u32 & 0x07) << 11);
        self.w
    }
}
#[doc = "Field `TOUCH_PAD5_DAC` reader - configure touch pad dac5"]
pub struct TOUCH_PAD5_DAC_R(crate::FieldReader<u8, u8>);
impl TOUCH_PAD5_DAC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TOUCH_PAD5_DAC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_PAD5_DAC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_PAD5_DAC` writer - configure touch pad dac5"]
pub struct TOUCH_PAD5_DAC_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_PAD5_DAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 14)) | ((value as u32 & 0x07) << 14);
        self.w
    }
}
#[doc = "Field `TOUCH_PAD4_DAC` reader - configure touch pad dac4"]
pub struct TOUCH_PAD4_DAC_R(crate::FieldReader<u8, u8>);
impl TOUCH_PAD4_DAC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TOUCH_PAD4_DAC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_PAD4_DAC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_PAD4_DAC` writer - configure touch pad dac4"]
pub struct TOUCH_PAD4_DAC_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_PAD4_DAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 17)) | ((value as u32 & 0x07) << 17);
        self.w
    }
}
#[doc = "Field `TOUCH_PAD3_DAC` reader - configure touch pad dac3"]
pub struct TOUCH_PAD3_DAC_R(crate::FieldReader<u8, u8>);
impl TOUCH_PAD3_DAC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TOUCH_PAD3_DAC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_PAD3_DAC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_PAD3_DAC` writer - configure touch pad dac3"]
pub struct TOUCH_PAD3_DAC_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_PAD3_DAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | ((value as u32 & 0x07) << 20);
        self.w
    }
}
#[doc = "Field `TOUCH_PAD2_DAC` reader - configure touch pad dac2"]
pub struct TOUCH_PAD2_DAC_R(crate::FieldReader<u8, u8>);
impl TOUCH_PAD2_DAC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TOUCH_PAD2_DAC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_PAD2_DAC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_PAD2_DAC` writer - configure touch pad dac2"]
pub struct TOUCH_PAD2_DAC_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_PAD2_DAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 23)) | ((value as u32 & 0x07) << 23);
        self.w
    }
}
#[doc = "Field `TOUCH_PAD1_DAC` reader - configure touch pad dac1"]
pub struct TOUCH_PAD1_DAC_R(crate::FieldReader<u8, u8>);
impl TOUCH_PAD1_DAC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TOUCH_PAD1_DAC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_PAD1_DAC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_PAD1_DAC` writer - configure touch pad dac1"]
pub struct TOUCH_PAD1_DAC_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_PAD1_DAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 26)) | ((value as u32 & 0x07) << 26);
        self.w
    }
}
#[doc = "Field `TOUCH_PAD0_DAC` reader - configure touch pad dac0"]
pub struct TOUCH_PAD0_DAC_R(crate::FieldReader<u8, u8>);
impl TOUCH_PAD0_DAC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TOUCH_PAD0_DAC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_PAD0_DAC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_PAD0_DAC` writer - configure touch pad dac0"]
pub struct TOUCH_PAD0_DAC_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_PAD0_DAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 29)) | ((value as u32 & 0x07) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:4 - configure touch pad dac9"]
    #[inline(always)]
    pub fn touch_pad9_dac(&self) -> TOUCH_PAD9_DAC_R {
        TOUCH_PAD9_DAC_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bits 5:7 - configure touch pad dac8"]
    #[inline(always)]
    pub fn touch_pad8_dac(&self) -> TOUCH_PAD8_DAC_R {
        TOUCH_PAD8_DAC_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - configure touch pad dac7"]
    #[inline(always)]
    pub fn touch_pad7_dac(&self) -> TOUCH_PAD7_DAC_R {
        TOUCH_PAD7_DAC_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 11:13 - configure touch pad dac6"]
    #[inline(always)]
    pub fn touch_pad6_dac(&self) -> TOUCH_PAD6_DAC_R {
        TOUCH_PAD6_DAC_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bits 14:16 - configure touch pad dac5"]
    #[inline(always)]
    pub fn touch_pad5_dac(&self) -> TOUCH_PAD5_DAC_R {
        TOUCH_PAD5_DAC_R::new(((self.bits >> 14) & 0x07) as u8)
    }
    #[doc = "Bits 17:19 - configure touch pad dac4"]
    #[inline(always)]
    pub fn touch_pad4_dac(&self) -> TOUCH_PAD4_DAC_R {
        TOUCH_PAD4_DAC_R::new(((self.bits >> 17) & 0x07) as u8)
    }
    #[doc = "Bits 20:22 - configure touch pad dac3"]
    #[inline(always)]
    pub fn touch_pad3_dac(&self) -> TOUCH_PAD3_DAC_R {
        TOUCH_PAD3_DAC_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 23:25 - configure touch pad dac2"]
    #[inline(always)]
    pub fn touch_pad2_dac(&self) -> TOUCH_PAD2_DAC_R {
        TOUCH_PAD2_DAC_R::new(((self.bits >> 23) & 0x07) as u8)
    }
    #[doc = "Bits 26:28 - configure touch pad dac1"]
    #[inline(always)]
    pub fn touch_pad1_dac(&self) -> TOUCH_PAD1_DAC_R {
        TOUCH_PAD1_DAC_R::new(((self.bits >> 26) & 0x07) as u8)
    }
    #[doc = "Bits 29:31 - configure touch pad dac0"]
    #[inline(always)]
    pub fn touch_pad0_dac(&self) -> TOUCH_PAD0_DAC_R {
        TOUCH_PAD0_DAC_R::new(((self.bits >> 29) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 2:4 - configure touch pad dac9"]
    #[inline(always)]
    pub fn touch_pad9_dac(&mut self) -> TOUCH_PAD9_DAC_W {
        TOUCH_PAD9_DAC_W { w: self }
    }
    #[doc = "Bits 5:7 - configure touch pad dac8"]
    #[inline(always)]
    pub fn touch_pad8_dac(&mut self) -> TOUCH_PAD8_DAC_W {
        TOUCH_PAD8_DAC_W { w: self }
    }
    #[doc = "Bits 8:10 - configure touch pad dac7"]
    #[inline(always)]
    pub fn touch_pad7_dac(&mut self) -> TOUCH_PAD7_DAC_W {
        TOUCH_PAD7_DAC_W { w: self }
    }
    #[doc = "Bits 11:13 - configure touch pad dac6"]
    #[inline(always)]
    pub fn touch_pad6_dac(&mut self) -> TOUCH_PAD6_DAC_W {
        TOUCH_PAD6_DAC_W { w: self }
    }
    #[doc = "Bits 14:16 - configure touch pad dac5"]
    #[inline(always)]
    pub fn touch_pad5_dac(&mut self) -> TOUCH_PAD5_DAC_W {
        TOUCH_PAD5_DAC_W { w: self }
    }
    #[doc = "Bits 17:19 - configure touch pad dac4"]
    #[inline(always)]
    pub fn touch_pad4_dac(&mut self) -> TOUCH_PAD4_DAC_W {
        TOUCH_PAD4_DAC_W { w: self }
    }
    #[doc = "Bits 20:22 - configure touch pad dac3"]
    #[inline(always)]
    pub fn touch_pad3_dac(&mut self) -> TOUCH_PAD3_DAC_W {
        TOUCH_PAD3_DAC_W { w: self }
    }
    #[doc = "Bits 23:25 - configure touch pad dac2"]
    #[inline(always)]
    pub fn touch_pad2_dac(&mut self) -> TOUCH_PAD2_DAC_W {
        TOUCH_PAD2_DAC_W { w: self }
    }
    #[doc = "Bits 26:28 - configure touch pad dac1"]
    #[inline(always)]
    pub fn touch_pad1_dac(&mut self) -> TOUCH_PAD1_DAC_W {
        TOUCH_PAD1_DAC_W { w: self }
    }
    #[doc = "Bits 29:31 - configure touch pad dac0"]
    #[inline(always)]
    pub fn touch_pad0_dac(&mut self) -> TOUCH_PAD0_DAC_W {
        TOUCH_PAD0_DAC_W { w: self }
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
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [touch_dac]
(index.html) module"]
pub struct TOUCH_DAC_SPEC;
impl crate::RegisterSpec for TOUCH_DAC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [touch_dac::R]
(R) reader structure"]
impl crate::Readable for TOUCH_DAC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [touch_dac::W]
(W) writer structure"]
impl crate::Writable for TOUCH_DAC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TOUCH_DAC to value 0"]
impl crate::Resettable for TOUCH_DAC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
