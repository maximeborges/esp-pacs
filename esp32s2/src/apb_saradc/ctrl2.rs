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
#[doc = "Field `MEAS_NUM_LIMIT` reader - Enable limit times of SAR ADC sample."]
pub struct MEAS_NUM_LIMIT_R(crate::FieldReader<bool>);
impl MEAS_NUM_LIMIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEAS_NUM_LIMIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEAS_NUM_LIMIT_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEAS_NUM_LIMIT` writer - Enable limit times of SAR ADC sample."]
pub struct MEAS_NUM_LIMIT_W<'a> {
    w: &'a mut W,
}
impl<'a> MEAS_NUM_LIMIT_W<'a> {
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
#[doc = "Field `MAX_MEAS_NUM` reader - Set maximum conversion number."]
pub struct MAX_MEAS_NUM_R(crate::FieldReader<u8>);
impl MAX_MEAS_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MAX_MEAS_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAX_MEAS_NUM_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAX_MEAS_NUM` writer - Set maximum conversion number."]
pub struct MAX_MEAS_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> MAX_MEAS_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 1)) | ((value as u32 & 0xff) << 1);
        self.w
    }
}
#[doc = "Field `SAR1_INV` reader - 1: data to DIG ADC1 CTRL is inverted, otherwise not."]
pub struct SAR1_INV_R(crate::FieldReader<bool>);
impl SAR1_INV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SAR1_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR1_INV_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR1_INV` writer - 1: data to DIG ADC1 CTRL is inverted, otherwise not."]
pub struct SAR1_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR1_INV_W<'a> {
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
#[doc = "Field `SAR2_INV` reader - 1: data to DIG ADC2 CTRL is inverted, otherwise not."]
pub struct SAR2_INV_R(crate::FieldReader<bool>);
impl SAR2_INV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SAR2_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR2_INV_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR2_INV` writer - 1: data to DIG ADC2 CTRL is inverted, otherwise not."]
pub struct SAR2_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR2_INV_W<'a> {
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
#[doc = "Field `TIMER_SEL` reader - 1: select saradc timer 0: i2s_ws trigger"]
pub struct TIMER_SEL_R(crate::FieldReader<bool>);
impl TIMER_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER_SEL_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER_SEL` writer - 1: select saradc timer 0: i2s_ws trigger"]
pub struct TIMER_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_SEL_W<'a> {
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
#[doc = "Field `TIMER_TARGET` reader - Set SAR ADC timer target."]
pub struct TIMER_TARGET_R(crate::FieldReader<u16>);
impl TIMER_TARGET_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TIMER_TARGET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER_TARGET_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER_TARGET` writer - Set SAR ADC timer target."]
pub struct TIMER_TARGET_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_TARGET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 12)) | ((value as u32 & 0x0fff) << 12);
        self.w
    }
}
#[doc = "Field `TIMER_EN` reader - Enable SAR ADC timer trigger."]
pub struct TIMER_EN_R(crate::FieldReader<bool>);
impl TIMER_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER_EN` writer - Enable SAR ADC timer trigger."]
pub struct TIMER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_EN_W<'a> {
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
    #[doc = "Bit 0 - Enable limit times of SAR ADC sample."]
    #[inline(always)]
    pub fn meas_num_limit(&self) -> MEAS_NUM_LIMIT_R {
        MEAS_NUM_LIMIT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:8 - Set maximum conversion number."]
    #[inline(always)]
    pub fn max_meas_num(&self) -> MAX_MEAS_NUM_R {
        MAX_MEAS_NUM_R::new(((self.bits >> 1) & 0xff) as u8)
    }
    #[doc = "Bit 9 - 1: data to DIG ADC1 CTRL is inverted, otherwise not."]
    #[inline(always)]
    pub fn sar1_inv(&self) -> SAR1_INV_R {
        SAR1_INV_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 1: data to DIG ADC2 CTRL is inverted, otherwise not."]
    #[inline(always)]
    pub fn sar2_inv(&self) -> SAR2_INV_R {
        SAR2_INV_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 1: select saradc timer 0: i2s_ws trigger"]
    #[inline(always)]
    pub fn timer_sel(&self) -> TIMER_SEL_R {
        TIMER_SEL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:23 - Set SAR ADC timer target."]
    #[inline(always)]
    pub fn timer_target(&self) -> TIMER_TARGET_R {
        TIMER_TARGET_R::new(((self.bits >> 12) & 0x0fff) as u16)
    }
    #[doc = "Bit 24 - Enable SAR ADC timer trigger."]
    #[inline(always)]
    pub fn timer_en(&self) -> TIMER_EN_R {
        TIMER_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable limit times of SAR ADC sample."]
    #[inline(always)]
    pub fn meas_num_limit(&mut self) -> MEAS_NUM_LIMIT_W {
        MEAS_NUM_LIMIT_W { w: self }
    }
    #[doc = "Bits 1:8 - Set maximum conversion number."]
    #[inline(always)]
    pub fn max_meas_num(&mut self) -> MAX_MEAS_NUM_W {
        MAX_MEAS_NUM_W { w: self }
    }
    #[doc = "Bit 9 - 1: data to DIG ADC1 CTRL is inverted, otherwise not."]
    #[inline(always)]
    pub fn sar1_inv(&mut self) -> SAR1_INV_W {
        SAR1_INV_W { w: self }
    }
    #[doc = "Bit 10 - 1: data to DIG ADC2 CTRL is inverted, otherwise not."]
    #[inline(always)]
    pub fn sar2_inv(&mut self) -> SAR2_INV_W {
        SAR2_INV_W { w: self }
    }
    #[doc = "Bit 11 - 1: select saradc timer 0: i2s_ws trigger"]
    #[inline(always)]
    pub fn timer_sel(&mut self) -> TIMER_SEL_W {
        TIMER_SEL_W { w: self }
    }
    #[doc = "Bits 12:23 - Set SAR ADC timer target."]
    #[inline(always)]
    pub fn timer_target(&mut self) -> TIMER_TARGET_W {
        TIMER_TARGET_W { w: self }
    }
    #[doc = "Bit 24 - Enable SAR ADC timer trigger."]
    #[inline(always)]
    pub fn timer_en(&mut self) -> TIMER_EN_W {
        TIMER_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DIG ADC common configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl2](index.html) module"]
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
