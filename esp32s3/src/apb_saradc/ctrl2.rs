#[doc = "Register `CTRL2` reader"]
pub struct R(crate::R<CTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL2` writer"]
pub struct W(crate::W<CTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL2_SPEC>;
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
impl From<crate::W<CTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SARADC_MEAS_NUM_LIMIT` reader - enable apb saradc limit the sample num"]
pub struct SARADC_MEAS_NUM_LIMIT_R(crate::FieldReader<bool>);
impl SARADC_MEAS_NUM_LIMIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SARADC_MEAS_NUM_LIMIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SARADC_MEAS_NUM_LIMIT_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SARADC_MEAS_NUM_LIMIT` writer - enable apb saradc limit the sample num"]
pub struct SARADC_MEAS_NUM_LIMIT_W<'a> {
    w: &'a mut W,
}
impl<'a> SARADC_MEAS_NUM_LIMIT_W<'a> {
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
#[doc = "Field `SARADC_MAX_MEAS_NUM` reader - max conversion number"]
pub struct SARADC_MAX_MEAS_NUM_R(crate::FieldReader<u8>);
impl SARADC_MAX_MEAS_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SARADC_MAX_MEAS_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SARADC_MAX_MEAS_NUM_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SARADC_MAX_MEAS_NUM` writer - max conversion number"]
pub struct SARADC_MAX_MEAS_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> SARADC_MAX_MEAS_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 1)) | ((value as u32 & 0xff) << 1);
        self.w
    }
}
#[doc = "Field `SARADC_SAR1_INV` reader - 1: data to DIG ADC1 CTRL is inverted, otherwise not"]
pub struct SARADC_SAR1_INV_R(crate::FieldReader<bool>);
impl SARADC_SAR1_INV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SARADC_SAR1_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SARADC_SAR1_INV_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SARADC_SAR1_INV` writer - 1: data to DIG ADC1 CTRL is inverted, otherwise not"]
pub struct SARADC_SAR1_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> SARADC_SAR1_INV_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 9)) | ((value as u32 & 1) << 9);
        self.w
    }
}
#[doc = "Field `SARADC_SAR2_INV` reader - 1: data to DIG ADC2 CTRL is inverted, otherwise not"]
pub struct SARADC_SAR2_INV_R(crate::FieldReader<bool>);
impl SARADC_SAR2_INV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SARADC_SAR2_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SARADC_SAR2_INV_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SARADC_SAR2_INV` writer - 1: data to DIG ADC2 CTRL is inverted, otherwise not"]
pub struct SARADC_SAR2_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> SARADC_SAR2_INV_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 10)) | ((value as u32 & 1) << 10);
        self.w
    }
}
#[doc = "Field `SARADC_TIMER_SEL` reader - 1: select saradc timer 0: i2s_ws trigger"]
pub struct SARADC_TIMER_SEL_R(crate::FieldReader<bool>);
impl SARADC_TIMER_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SARADC_TIMER_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SARADC_TIMER_SEL_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SARADC_TIMER_SEL` writer - 1: select saradc timer 0: i2s_ws trigger"]
pub struct SARADC_TIMER_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SARADC_TIMER_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 11)) | ((value as u32 & 1) << 11);
        self.w
    }
}
#[doc = "Field `SARADC_TIMER_TARGET` reader - to set saradc timer target"]
pub struct SARADC_TIMER_TARGET_R(crate::FieldReader<u16>);
impl SARADC_TIMER_TARGET_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        SARADC_TIMER_TARGET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SARADC_TIMER_TARGET_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SARADC_TIMER_TARGET` writer - to set saradc timer target"]
pub struct SARADC_TIMER_TARGET_W<'a> {
    w: &'a mut W,
}
impl<'a> SARADC_TIMER_TARGET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 12)) | ((value as u32 & 0x0fff) << 12);
        self.w
    }
}
#[doc = "Field `SARADC_TIMER_EN` reader - to enable saradc timer trigger"]
pub struct SARADC_TIMER_EN_R(crate::FieldReader<bool>);
impl SARADC_TIMER_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SARADC_TIMER_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SARADC_TIMER_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SARADC_TIMER_EN` writer - to enable saradc timer trigger"]
pub struct SARADC_TIMER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SARADC_TIMER_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 24)) | ((value as u32 & 1) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - enable apb saradc limit the sample num"]
    #[inline(always)]
    pub fn saradc_meas_num_limit(&self) -> SARADC_MEAS_NUM_LIMIT_R {
        SARADC_MEAS_NUM_LIMIT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:8 - max conversion number"]
    #[inline(always)]
    pub fn saradc_max_meas_num(&self) -> SARADC_MAX_MEAS_NUM_R {
        SARADC_MAX_MEAS_NUM_R::new(((self.bits >> 1) & 0xff) as u8)
    }
    #[doc = "Bit 9 - 1: data to DIG ADC1 CTRL is inverted, otherwise not"]
    #[inline(always)]
    pub fn saradc_sar1_inv(&self) -> SARADC_SAR1_INV_R {
        SARADC_SAR1_INV_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 1: data to DIG ADC2 CTRL is inverted, otherwise not"]
    #[inline(always)]
    pub fn saradc_sar2_inv(&self) -> SARADC_SAR2_INV_R {
        SARADC_SAR2_INV_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 1: select saradc timer 0: i2s_ws trigger"]
    #[inline(always)]
    pub fn saradc_timer_sel(&self) -> SARADC_TIMER_SEL_R {
        SARADC_TIMER_SEL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:23 - to set saradc timer target"]
    #[inline(always)]
    pub fn saradc_timer_target(&self) -> SARADC_TIMER_TARGET_R {
        SARADC_TIMER_TARGET_R::new(((self.bits >> 12) & 0x0fff) as u16)
    }
    #[doc = "Bit 24 - to enable saradc timer trigger"]
    #[inline(always)]
    pub fn saradc_timer_en(&self) -> SARADC_TIMER_EN_R {
        SARADC_TIMER_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - enable apb saradc limit the sample num"]
    #[inline(always)]
    pub fn saradc_meas_num_limit(&mut self) -> SARADC_MEAS_NUM_LIMIT_W {
        SARADC_MEAS_NUM_LIMIT_W { w: self }
    }
    #[doc = "Bits 1:8 - max conversion number"]
    #[inline(always)]
    pub fn saradc_max_meas_num(&mut self) -> SARADC_MAX_MEAS_NUM_W {
        SARADC_MAX_MEAS_NUM_W { w: self }
    }
    #[doc = "Bit 9 - 1: data to DIG ADC1 CTRL is inverted, otherwise not"]
    #[inline(always)]
    pub fn saradc_sar1_inv(&mut self) -> SARADC_SAR1_INV_W {
        SARADC_SAR1_INV_W { w: self }
    }
    #[doc = "Bit 10 - 1: data to DIG ADC2 CTRL is inverted, otherwise not"]
    #[inline(always)]
    pub fn saradc_sar2_inv(&mut self) -> SARADC_SAR2_INV_W {
        SARADC_SAR2_INV_W { w: self }
    }
    #[doc = "Bit 11 - 1: select saradc timer 0: i2s_ws trigger"]
    #[inline(always)]
    pub fn saradc_timer_sel(&mut self) -> SARADC_TIMER_SEL_W {
        SARADC_TIMER_SEL_W { w: self }
    }
    #[doc = "Bits 12:23 - to set saradc timer target"]
    #[inline(always)]
    pub fn saradc_timer_target(&mut self) -> SARADC_TIMER_TARGET_W {
        SARADC_TIMER_TARGET_W { w: self }
    }
    #[doc = "Bit 24 - to enable saradc timer trigger"]
    #[inline(always)]
    pub fn saradc_timer_en(&mut self) -> SARADC_TIMER_EN_W {
        SARADC_TIMER_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure apb saradc controller\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl2](index.html) module"]
pub struct CTRL2_SPEC;
impl crate::RegisterSpec for CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl2::R](R) reader structure"]
impl crate::Readable for CTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl2::W](W) writer structure"]
impl crate::Writable for CTRL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL2 to value 0xa1fe"]
impl crate::Resettable for CTRL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xa1fe
    }
}
