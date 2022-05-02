#[doc = "Register `RTCCALICFG2` reader"]
pub struct R(crate::R<RTCCALICFG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCCALICFG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCCALICFG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCCALICFG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCCALICFG2` writer"]
pub struct W(crate::W<RTCCALICFG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCCALICFG2_SPEC>;
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
impl From<crate::W<RTCCALICFG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCCALICFG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_CALI_TIMEOUT` reader - timeoutindicator"]
pub struct RTC_CALI_TIMEOUT_R(crate::FieldReader<bool>);
impl RTC_CALI_TIMEOUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_CALI_TIMEOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_CALI_TIMEOUT_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_CALI_TIMEOUT_RST_CNT` reader - reg_rtc_cali_timeout_rst_cnt.Cyclesthatreleasecalibrationtimeoutreset"]
pub struct RTC_CALI_TIMEOUT_RST_CNT_R(crate::FieldReader<u8>);
impl RTC_CALI_TIMEOUT_RST_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RTC_CALI_TIMEOUT_RST_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_CALI_TIMEOUT_RST_CNT_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_CALI_TIMEOUT_RST_CNT` writer - reg_rtc_cali_timeout_rst_cnt.Cyclesthatreleasecalibrationtimeoutreset"]
pub struct RTC_CALI_TIMEOUT_RST_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CALI_TIMEOUT_RST_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 3)) | ((value as u32 & 0x0f) << 3);
        self.w
    }
}
#[doc = "Field `RTC_CALI_TIMEOUT_THRES` reader - reg_rtc_cali_timeout_thres.timeoutifcalivaluecountsoverthreshold"]
pub struct RTC_CALI_TIMEOUT_THRES_R(crate::FieldReader<u32>);
impl RTC_CALI_TIMEOUT_THRES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        RTC_CALI_TIMEOUT_THRES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_CALI_TIMEOUT_THRES_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_CALI_TIMEOUT_THRES` writer - reg_rtc_cali_timeout_thres.timeoutifcalivaluecountsoverthreshold"]
pub struct RTC_CALI_TIMEOUT_THRES_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CALI_TIMEOUT_THRES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff_ffff << 7)) | ((value as u32 & 0x01ff_ffff) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - timeoutindicator"]
    #[inline(always)]
    pub fn rtc_cali_timeout(&self) -> RTC_CALI_TIMEOUT_R {
        RTC_CALI_TIMEOUT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 3:6 - reg_rtc_cali_timeout_rst_cnt.Cyclesthatreleasecalibrationtimeoutreset"]
    #[inline(always)]
    pub fn rtc_cali_timeout_rst_cnt(&self) -> RTC_CALI_TIMEOUT_RST_CNT_R {
        RTC_CALI_TIMEOUT_RST_CNT_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bits 7:31 - reg_rtc_cali_timeout_thres.timeoutifcalivaluecountsoverthreshold"]
    #[inline(always)]
    pub fn rtc_cali_timeout_thres(&self) -> RTC_CALI_TIMEOUT_THRES_R {
        RTC_CALI_TIMEOUT_THRES_R::new(((self.bits >> 7) & 0x01ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 3:6 - reg_rtc_cali_timeout_rst_cnt.Cyclesthatreleasecalibrationtimeoutreset"]
    #[inline(always)]
    pub fn rtc_cali_timeout_rst_cnt(&mut self) -> RTC_CALI_TIMEOUT_RST_CNT_W {
        RTC_CALI_TIMEOUT_RST_CNT_W { w: self }
    }
    #[doc = "Bits 7:31 - reg_rtc_cali_timeout_thres.timeoutifcalivaluecountsoverthreshold"]
    #[inline(always)]
    pub fn rtc_cali_timeout_thres(&mut self) -> RTC_CALI_TIMEOUT_THRES_W {
        RTC_CALI_TIMEOUT_THRES_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIMG_RTCCALICFG2_REG.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtccalicfg2](index.html) module"]
pub struct RTCCALICFG2_SPEC;
impl crate::RegisterSpec for RTCCALICFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtccalicfg2::R](R) reader structure"]
impl crate::Readable for RTCCALICFG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtccalicfg2::W](W) writer structure"]
impl crate::Writable for RTCCALICFG2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTCCALICFG2 to value 0xffff_ff98"]
impl crate::Resettable for RTCCALICFG2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ff98
    }
}
