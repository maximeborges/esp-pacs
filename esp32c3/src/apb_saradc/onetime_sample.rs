#[doc = "Register `ONETIME_SAMPLE` reader"]
pub struct R(crate::R<ONETIME_SAMPLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ONETIME_SAMPLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ONETIME_SAMPLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ONETIME_SAMPLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ONETIME_SAMPLE` writer"]
pub struct W(crate::W<ONETIME_SAMPLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ONETIME_SAMPLE_SPEC>;
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
impl From<crate::W<ONETIME_SAMPLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ONETIME_SAMPLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SARADC_ONETIME_ATTEN` reader - configure onetime atten"]
pub struct SARADC_ONETIME_ATTEN_R(crate::FieldReader<u8, u8>);
impl SARADC_ONETIME_ATTEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SARADC_ONETIME_ATTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SARADC_ONETIME_ATTEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SARADC_ONETIME_ATTEN` writer - configure onetime atten"]
pub struct SARADC_ONETIME_ATTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SARADC_ONETIME_ATTEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 23)) | ((value as u32 & 3) << 23);
        self.w
    }
}
#[doc = "Field `SARADC_ONETIME_CHANNEL` reader - configure onetime channel"]
pub struct SARADC_ONETIME_CHANNEL_R(crate::FieldReader<u8, u8>);
impl SARADC_ONETIME_CHANNEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SARADC_ONETIME_CHANNEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SARADC_ONETIME_CHANNEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SARADC_ONETIME_CHANNEL` writer - configure onetime channel"]
pub struct SARADC_ONETIME_CHANNEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SARADC_ONETIME_CHANNEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 25)) | ((value as u32 & 0x0f) << 25);
        self.w
    }
}
#[doc = "Field `SARADC_ONETIME_START` reader - trigger adc onetime sample"]
pub struct SARADC_ONETIME_START_R(crate::FieldReader<bool, bool>);
impl SARADC_ONETIME_START_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SARADC_ONETIME_START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SARADC_ONETIME_START_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SARADC_ONETIME_START` writer - trigger adc onetime sample"]
pub struct SARADC_ONETIME_START_W<'a> {
    w: &'a mut W,
}
impl<'a> SARADC_ONETIME_START_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 29)) | ((value as u32 & 1) << 29);
        self.w
    }
}
#[doc = "Field `SARADC2_ONETIME_SAMPLE` reader - enable adc2 onetime sample"]
pub struct SARADC2_ONETIME_SAMPLE_R(crate::FieldReader<bool, bool>);
impl SARADC2_ONETIME_SAMPLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SARADC2_ONETIME_SAMPLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SARADC2_ONETIME_SAMPLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SARADC2_ONETIME_SAMPLE` writer - enable adc2 onetime sample"]
pub struct SARADC2_ONETIME_SAMPLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SARADC2_ONETIME_SAMPLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 30)) | ((value as u32 & 1) << 30);
        self.w
    }
}
#[doc = "Field `SARADC1_ONETIME_SAMPLE` reader - enable adc1 onetime sample"]
pub struct SARADC1_ONETIME_SAMPLE_R(crate::FieldReader<bool, bool>);
impl SARADC1_ONETIME_SAMPLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SARADC1_ONETIME_SAMPLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SARADC1_ONETIME_SAMPLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SARADC1_ONETIME_SAMPLE` writer - enable adc1 onetime sample"]
pub struct SARADC1_ONETIME_SAMPLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SARADC1_ONETIME_SAMPLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 31)) | ((value as u32 & 1) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 23:24 - configure onetime atten"]
    #[inline(always)]
    pub fn saradc_onetime_atten(&self) -> SARADC_ONETIME_ATTEN_R {
        SARADC_ONETIME_ATTEN_R::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bits 25:28 - configure onetime channel"]
    #[inline(always)]
    pub fn saradc_onetime_channel(&self) -> SARADC_ONETIME_CHANNEL_R {
        SARADC_ONETIME_CHANNEL_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
    #[doc = "Bit 29 - trigger adc onetime sample"]
    #[inline(always)]
    pub fn saradc_onetime_start(&self) -> SARADC_ONETIME_START_R {
        SARADC_ONETIME_START_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - enable adc2 onetime sample"]
    #[inline(always)]
    pub fn saradc2_onetime_sample(&self) -> SARADC2_ONETIME_SAMPLE_R {
        SARADC2_ONETIME_SAMPLE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - enable adc1 onetime sample"]
    #[inline(always)]
    pub fn saradc1_onetime_sample(&self) -> SARADC1_ONETIME_SAMPLE_R {
        SARADC1_ONETIME_SAMPLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 23:24 - configure onetime atten"]
    #[inline(always)]
    pub fn saradc_onetime_atten(&mut self) -> SARADC_ONETIME_ATTEN_W {
        SARADC_ONETIME_ATTEN_W { w: self }
    }
    #[doc = "Bits 25:28 - configure onetime channel"]
    #[inline(always)]
    pub fn saradc_onetime_channel(&mut self) -> SARADC_ONETIME_CHANNEL_W {
        SARADC_ONETIME_CHANNEL_W { w: self }
    }
    #[doc = "Bit 29 - trigger adc onetime sample"]
    #[inline(always)]
    pub fn saradc_onetime_start(&mut self) -> SARADC_ONETIME_START_W {
        SARADC_ONETIME_START_W { w: self }
    }
    #[doc = "Bit 30 - enable adc2 onetime sample"]
    #[inline(always)]
    pub fn saradc2_onetime_sample(&mut self) -> SARADC2_ONETIME_SAMPLE_W {
        SARADC2_ONETIME_SAMPLE_W { w: self }
    }
    #[doc = "Bit 31 - enable adc1 onetime sample"]
    #[inline(always)]
    pub fn saradc1_onetime_sample(&mut self) -> SARADC1_ONETIME_SAMPLE_W {
        SARADC1_ONETIME_SAMPLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "digital saradc configure register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [onetime_sample]
(index.html) module"]
pub struct ONETIME_SAMPLE_SPEC;
impl crate::RegisterSpec for ONETIME_SAMPLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [onetime_sample::R]
(R) reader structure"]
impl crate::Readable for ONETIME_SAMPLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [onetime_sample::W]
(W) writer structure"]
impl crate::Writable for ONETIME_SAMPLE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ONETIME_SAMPLE to value 0x1a00_0000"]
impl crate::Resettable for ONETIME_SAMPLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1a00_0000
    }
}
