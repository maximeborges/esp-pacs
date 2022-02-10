#[doc = "Register `APB_ADC_ARB_CTRL` reader"]
pub struct R(crate::R<APB_ADC_ARB_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB_ADC_ARB_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB_ADC_ARB_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB_ADC_ARB_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB_ADC_ARB_CTRL` writer"]
pub struct W(crate::W<APB_ADC_ARB_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB_ADC_ARB_CTRL_SPEC>;
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
impl From<crate::W<APB_ADC_ARB_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB_ADC_ARB_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC_ARB_APB_FORCE` reader - adc2 arbiter force to enableapb controller"]
pub struct ADC_ARB_APB_FORCE_R(crate::FieldReader<bool, bool>);
impl ADC_ARB_APB_FORCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC_ARB_APB_FORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_ARB_APB_FORCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_ARB_APB_FORCE` writer - adc2 arbiter force to enableapb controller"]
pub struct ADC_ARB_APB_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_ARB_APB_FORCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `ADC_ARB_RTC_FORCE` reader - adc2 arbiter force to enable rtc controller"]
pub struct ADC_ARB_RTC_FORCE_R(crate::FieldReader<bool, bool>);
impl ADC_ARB_RTC_FORCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC_ARB_RTC_FORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_ARB_RTC_FORCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_ARB_RTC_FORCE` writer - adc2 arbiter force to enable rtc controller"]
pub struct ADC_ARB_RTC_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_ARB_RTC_FORCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `ADC_ARB_WIFI_FORCE` reader - adc2 arbiter force to enable wifi controller"]
pub struct ADC_ARB_WIFI_FORCE_R(crate::FieldReader<bool, bool>);
impl ADC_ARB_WIFI_FORCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC_ARB_WIFI_FORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_ARB_WIFI_FORCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_ARB_WIFI_FORCE` writer - adc2 arbiter force to enable wifi controller"]
pub struct ADC_ARB_WIFI_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_ARB_WIFI_FORCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `ADC_ARB_GRANT_FORCE` reader - adc2 arbiter force grant"]
pub struct ADC_ARB_GRANT_FORCE_R(crate::FieldReader<bool, bool>);
impl ADC_ARB_GRANT_FORCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC_ARB_GRANT_FORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_ARB_GRANT_FORCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_ARB_GRANT_FORCE` writer - adc2 arbiter force grant"]
pub struct ADC_ARB_GRANT_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_ARB_GRANT_FORCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `ADC_ARB_APB_PRIORITY` reader - Set adc2 arbiterapb priority"]
pub struct ADC_ARB_APB_PRIORITY_R(crate::FieldReader<u8, u8>);
impl ADC_ARB_APB_PRIORITY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ADC_ARB_APB_PRIORITY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_ARB_APB_PRIORITY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_ARB_APB_PRIORITY` writer - Set adc2 arbiterapb priority"]
pub struct ADC_ARB_APB_PRIORITY_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_ARB_APB_PRIORITY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `ADC_ARB_RTC_PRIORITY` reader - Set adc2 arbiter rtc priority"]
pub struct ADC_ARB_RTC_PRIORITY_R(crate::FieldReader<u8, u8>);
impl ADC_ARB_RTC_PRIORITY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ADC_ARB_RTC_PRIORITY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_ARB_RTC_PRIORITY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_ARB_RTC_PRIORITY` writer - Set adc2 arbiter rtc priority"]
pub struct ADC_ARB_RTC_PRIORITY_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_ARB_RTC_PRIORITY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `ADC_ARB_WIFI_PRIORITY` reader - Set adc2 arbiter wifi priority"]
pub struct ADC_ARB_WIFI_PRIORITY_R(crate::FieldReader<u8, u8>);
impl ADC_ARB_WIFI_PRIORITY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ADC_ARB_WIFI_PRIORITY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_ARB_WIFI_PRIORITY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_ARB_WIFI_PRIORITY` writer - Set adc2 arbiter wifi priority"]
pub struct ADC_ARB_WIFI_PRIORITY_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_ARB_WIFI_PRIORITY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `ADC_ARB_FIX_PRIORITY` reader - adc2 arbiter uses fixed priority"]
pub struct ADC_ARB_FIX_PRIORITY_R(crate::FieldReader<bool, bool>);
impl ADC_ARB_FIX_PRIORITY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC_ARB_FIX_PRIORITY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_ARB_FIX_PRIORITY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_ARB_FIX_PRIORITY` writer - adc2 arbiter uses fixed priority"]
pub struct ADC_ARB_FIX_PRIORITY_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_ARB_FIX_PRIORITY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - adc2 arbiter force to enableapb controller"]
    #[inline(always)]
    pub fn adc_arb_apb_force(&self) -> ADC_ARB_APB_FORCE_R {
        ADC_ARB_APB_FORCE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - adc2 arbiter force to enable rtc controller"]
    #[inline(always)]
    pub fn adc_arb_rtc_force(&self) -> ADC_ARB_RTC_FORCE_R {
        ADC_ARB_RTC_FORCE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - adc2 arbiter force to enable wifi controller"]
    #[inline(always)]
    pub fn adc_arb_wifi_force(&self) -> ADC_ARB_WIFI_FORCE_R {
        ADC_ARB_WIFI_FORCE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - adc2 arbiter force grant"]
    #[inline(always)]
    pub fn adc_arb_grant_force(&self) -> ADC_ARB_GRANT_FORCE_R {
        ADC_ARB_GRANT_FORCE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Set adc2 arbiterapb priority"]
    #[inline(always)]
    pub fn adc_arb_apb_priority(&self) -> ADC_ARB_APB_PRIORITY_R {
        ADC_ARB_APB_PRIORITY_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Set adc2 arbiter rtc priority"]
    #[inline(always)]
    pub fn adc_arb_rtc_priority(&self) -> ADC_ARB_RTC_PRIORITY_R {
        ADC_ARB_RTC_PRIORITY_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Set adc2 arbiter wifi priority"]
    #[inline(always)]
    pub fn adc_arb_wifi_priority(&self) -> ADC_ARB_WIFI_PRIORITY_R {
        ADC_ARB_WIFI_PRIORITY_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 12 - adc2 arbiter uses fixed priority"]
    #[inline(always)]
    pub fn adc_arb_fix_priority(&self) -> ADC_ARB_FIX_PRIORITY_R {
        ADC_ARB_FIX_PRIORITY_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - adc2 arbiter force to enableapb controller"]
    #[inline(always)]
    pub fn adc_arb_apb_force(&mut self) -> ADC_ARB_APB_FORCE_W {
        ADC_ARB_APB_FORCE_W { w: self }
    }
    #[doc = "Bit 3 - adc2 arbiter force to enable rtc controller"]
    #[inline(always)]
    pub fn adc_arb_rtc_force(&mut self) -> ADC_ARB_RTC_FORCE_W {
        ADC_ARB_RTC_FORCE_W { w: self }
    }
    #[doc = "Bit 4 - adc2 arbiter force to enable wifi controller"]
    #[inline(always)]
    pub fn adc_arb_wifi_force(&mut self) -> ADC_ARB_WIFI_FORCE_W {
        ADC_ARB_WIFI_FORCE_W { w: self }
    }
    #[doc = "Bit 5 - adc2 arbiter force grant"]
    #[inline(always)]
    pub fn adc_arb_grant_force(&mut self) -> ADC_ARB_GRANT_FORCE_W {
        ADC_ARB_GRANT_FORCE_W { w: self }
    }
    #[doc = "Bits 6:7 - Set adc2 arbiterapb priority"]
    #[inline(always)]
    pub fn adc_arb_apb_priority(&mut self) -> ADC_ARB_APB_PRIORITY_W {
        ADC_ARB_APB_PRIORITY_W { w: self }
    }
    #[doc = "Bits 8:9 - Set adc2 arbiter rtc priority"]
    #[inline(always)]
    pub fn adc_arb_rtc_priority(&mut self) -> ADC_ARB_RTC_PRIORITY_W {
        ADC_ARB_RTC_PRIORITY_W { w: self }
    }
    #[doc = "Bits 10:11 - Set adc2 arbiter wifi priority"]
    #[inline(always)]
    pub fn adc_arb_wifi_priority(&mut self) -> ADC_ARB_WIFI_PRIORITY_W {
        ADC_ARB_WIFI_PRIORITY_W { w: self }
    }
    #[doc = "Bit 12 - adc2 arbiter uses fixed priority"]
    #[inline(always)]
    pub fn adc_arb_fix_priority(&mut self) -> ADC_ARB_FIX_PRIORITY_W {
        ADC_ARB_FIX_PRIORITY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure apb saradc arbit\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_adc_arb_ctrl]
(index.html) module"]
pub struct APB_ADC_ARB_CTRL_SPEC;
impl crate::RegisterSpec for APB_ADC_ARB_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb_adc_arb_ctrl::R]
(R) reader structure"]
impl crate::Readable for APB_ADC_ARB_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb_adc_arb_ctrl::W]
(W) writer structure"]
impl crate::Writable for APB_ADC_ARB_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB_ADC_ARB_CTRL to value 0x0900"]
impl crate::Resettable for APB_ADC_ARB_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0900
    }
}
