#[doc = "Register `SAR_MEAS2_MUX` reader"]
pub struct R(crate::R<SAR_MEAS2_MUX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_MEAS2_MUX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_MEAS2_MUX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_MEAS2_MUX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_MEAS2_MUX` writer"]
pub struct W(crate::W<SAR_MEAS2_MUX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_MEAS2_MUX_SPEC>;
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
impl From<crate::W<SAR_MEAS2_MUX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_MEAS2_MUX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAR2_PWDET_CCT` reader - SAR2_PWDET_CCT, PA power detector capacitance tuning."]
pub struct SAR2_PWDET_CCT_R(crate::FieldReader<u8, u8>);
impl SAR2_PWDET_CCT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SAR2_PWDET_CCT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR2_PWDET_CCT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR2_PWDET_CCT` writer - SAR2_PWDET_CCT, PA power detector capacitance tuning."]
pub struct SAR2_PWDET_CCT_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR2_PWDET_CCT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 28)) | ((value as u32 & 7) << 28);
        self.w
    }
}
#[doc = "Field `SAR2_RTC_FORCE` reader - In sleep, force to use RTC to control ADC."]
pub struct SAR2_RTC_FORCE_R(crate::FieldReader<bool, bool>);
impl SAR2_RTC_FORCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SAR2_RTC_FORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR2_RTC_FORCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR2_RTC_FORCE` writer - In sleep, force to use RTC to control ADC."]
pub struct SAR2_RTC_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR2_RTC_FORCE_W<'a> {
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
    #[doc = "Bits 28:30 - SAR2_PWDET_CCT, PA power detector capacitance tuning."]
    #[inline(always)]
    pub fn sar2_pwdet_cct(&self) -> SAR2_PWDET_CCT_R {
        SAR2_PWDET_CCT_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - In sleep, force to use RTC to control ADC."]
    #[inline(always)]
    pub fn sar2_rtc_force(&self) -> SAR2_RTC_FORCE_R {
        SAR2_RTC_FORCE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 28:30 - SAR2_PWDET_CCT, PA power detector capacitance tuning."]
    #[inline(always)]
    pub fn sar2_pwdet_cct(&mut self) -> SAR2_PWDET_CCT_W {
        SAR2_PWDET_CCT_W { w: self }
    }
    #[doc = "Bit 31 - In sleep, force to use RTC to control ADC."]
    #[inline(always)]
    pub fn sar2_rtc_force(&mut self) -> SAR2_RTC_FORCE_W {
        SAR2_RTC_FORCE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Select the controller for SAR ADC2\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_meas2_mux]
(index.html) module"]
pub struct SAR_MEAS2_MUX_SPEC;
impl crate::RegisterSpec for SAR_MEAS2_MUX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_meas2_mux::R]
(R) reader structure"]
impl crate::Readable for SAR_MEAS2_MUX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_meas2_mux::W]
(W) writer structure"]
impl crate::Writable for SAR_MEAS2_MUX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAR_MEAS2_MUX to value 0"]
impl crate::Resettable for SAR_MEAS2_MUX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
