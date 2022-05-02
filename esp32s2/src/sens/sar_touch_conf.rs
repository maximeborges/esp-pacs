#[doc = "Register `SAR_TOUCH_CONF` reader"]
pub struct R(crate::R<SAR_TOUCH_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_TOUCH_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_TOUCH_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_TOUCH_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_TOUCH_CONF` writer"]
pub struct W(crate::W<SAR_TOUCH_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_TOUCH_CONF_SPEC>;
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
impl From<crate::W<SAR_TOUCH_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_TOUCH_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOUCH_OUTEN` reader - Enable touch controller output."]
pub struct TOUCH_OUTEN_R(crate::FieldReader<u16>);
impl TOUCH_OUTEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TOUCH_OUTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_OUTEN_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_OUTEN` writer - Enable touch controller output."]
pub struct TOUCH_OUTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_OUTEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff) | (value as u32 & 0x7fff);
        self.w
    }
}
#[doc = "Field `TOUCH_STATUS_CLR` writer - Clear all touch active status."]
pub struct TOUCH_STATUS_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_STATUS_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 15)) | ((value as u32 & 1) << 15);
        self.w
    }
}
#[doc = "Field `TOUCH_DATA_SEL` reader - 0 and 1: touch_raw_data; 2: base_line; 3: touch_smooth_data."]
pub struct TOUCH_DATA_SEL_R(crate::FieldReader<u8>);
impl TOUCH_DATA_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TOUCH_DATA_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_DATA_SEL_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_DATA_SEL` writer - 0 and 1: touch_raw_data; 2: base_line; 3: touch_smooth_data."]
pub struct TOUCH_DATA_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_DATA_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 16)) | ((value as u32 & 3) << 16);
        self.w
    }
}
#[doc = "Field `TOUCH_DENOISE_END` reader - Touch denoise done."]
pub struct TOUCH_DENOISE_END_R(crate::FieldReader<bool>);
impl TOUCH_DENOISE_END_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOUCH_DENOISE_END_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_DENOISE_END_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_UNIT_END` reader - Indicate the completion of sampling."]
pub struct TOUCH_UNIT_END_R(crate::FieldReader<bool>);
impl TOUCH_UNIT_END_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOUCH_UNIT_END_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_UNIT_END_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_APPROACH_PAD2` reader - Indicate which pad is selected as proximity pad2"]
pub struct TOUCH_APPROACH_PAD2_R(crate::FieldReader<u8>);
impl TOUCH_APPROACH_PAD2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TOUCH_APPROACH_PAD2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_APPROACH_PAD2_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_APPROACH_PAD2` writer - Indicate which pad is selected as proximity pad2"]
pub struct TOUCH_APPROACH_PAD2_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_APPROACH_PAD2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `TOUCH_APPROACH_PAD1` reader - Indicate which pad is selected as proximity pad1"]
pub struct TOUCH_APPROACH_PAD1_R(crate::FieldReader<u8>);
impl TOUCH_APPROACH_PAD1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TOUCH_APPROACH_PAD1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_APPROACH_PAD1_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_APPROACH_PAD1` writer - Indicate which pad is selected as proximity pad1"]
pub struct TOUCH_APPROACH_PAD1_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_APPROACH_PAD1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `TOUCH_APPROACH_PAD0` reader - Indicate which pad is selected as proximity pad0"]
pub struct TOUCH_APPROACH_PAD0_R(crate::FieldReader<u8>);
impl TOUCH_APPROACH_PAD0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TOUCH_APPROACH_PAD0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_APPROACH_PAD0_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_APPROACH_PAD0` writer - Indicate which pad is selected as proximity pad0"]
pub struct TOUCH_APPROACH_PAD0_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_APPROACH_PAD0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:14 - Enable touch controller output."]
    #[inline(always)]
    pub fn touch_outen(&self) -> TOUCH_OUTEN_R {
        TOUCH_OUTEN_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:17 - 0 and 1: touch_raw_data; 2: base_line; 3: touch_smooth_data."]
    #[inline(always)]
    pub fn touch_data_sel(&self) -> TOUCH_DATA_SEL_R {
        TOUCH_DATA_SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Touch denoise done."]
    #[inline(always)]
    pub fn touch_denoise_end(&self) -> TOUCH_DENOISE_END_R {
        TOUCH_DENOISE_END_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Indicate the completion of sampling."]
    #[inline(always)]
    pub fn touch_unit_end(&self) -> TOUCH_UNIT_END_R {
        TOUCH_UNIT_END_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - Indicate which pad is selected as proximity pad2"]
    #[inline(always)]
    pub fn touch_approach_pad2(&self) -> TOUCH_APPROACH_PAD2_R {
        TOUCH_APPROACH_PAD2_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Indicate which pad is selected as proximity pad1"]
    #[inline(always)]
    pub fn touch_approach_pad1(&self) -> TOUCH_APPROACH_PAD1_R {
        TOUCH_APPROACH_PAD1_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Indicate which pad is selected as proximity pad0"]
    #[inline(always)]
    pub fn touch_approach_pad0(&self) -> TOUCH_APPROACH_PAD0_R {
        TOUCH_APPROACH_PAD0_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:14 - Enable touch controller output."]
    #[inline(always)]
    pub fn touch_outen(&mut self) -> TOUCH_OUTEN_W {
        TOUCH_OUTEN_W { w: self }
    }
    #[doc = "Bit 15 - Clear all touch active status."]
    #[inline(always)]
    pub fn touch_status_clr(&mut self) -> TOUCH_STATUS_CLR_W {
        TOUCH_STATUS_CLR_W { w: self }
    }
    #[doc = "Bits 16:17 - 0 and 1: touch_raw_data; 2: base_line; 3: touch_smooth_data."]
    #[inline(always)]
    pub fn touch_data_sel(&mut self) -> TOUCH_DATA_SEL_W {
        TOUCH_DATA_SEL_W { w: self }
    }
    #[doc = "Bits 20:23 - Indicate which pad is selected as proximity pad2"]
    #[inline(always)]
    pub fn touch_approach_pad2(&mut self) -> TOUCH_APPROACH_PAD2_W {
        TOUCH_APPROACH_PAD2_W { w: self }
    }
    #[doc = "Bits 24:27 - Indicate which pad is selected as proximity pad1"]
    #[inline(always)]
    pub fn touch_approach_pad1(&mut self) -> TOUCH_APPROACH_PAD1_W {
        TOUCH_APPROACH_PAD1_W { w: self }
    }
    #[doc = "Bits 28:31 - Indicate which pad is selected as proximity pad0"]
    #[inline(always)]
    pub fn touch_approach_pad0(&mut self) -> TOUCH_APPROACH_PAD0_W {
        TOUCH_APPROACH_PAD0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Touch sensor configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_touch_conf](index.html) module"]
pub struct SAR_TOUCH_CONF_SPEC;
impl crate::RegisterSpec for SAR_TOUCH_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_touch_conf::R](R) reader structure"]
impl crate::Readable for SAR_TOUCH_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_touch_conf::W](W) writer structure"]
impl crate::Writable for SAR_TOUCH_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAR_TOUCH_CONF to value 0xfff0_7fff"]
impl crate::Resettable for SAR_TOUCH_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xfff0_7fff
    }
}
