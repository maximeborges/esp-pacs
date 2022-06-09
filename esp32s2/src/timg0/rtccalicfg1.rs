#[doc = "Register `RTCCALICFG1` reader"]
pub struct R(crate::R<RTCCALICFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCCALICFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCCALICFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCCALICFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RTC_CALI_CYCLING_DATA_VLD` reader - Periodic calibration valid signal."]
pub type RTC_CALI_CYCLING_DATA_VLD_R = crate::BitReader<bool>;
#[doc = "Field `RTC_CALI_VALUE` reader - Calibration value when cycles of clock to be calibrated reach TIMG_RTC_CALI_MAX, in unit of XTAL_CLK clock cycles."]
pub type RTC_CALI_VALUE_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bit 0 - Periodic calibration valid signal."]
    #[inline(always)]
    pub fn rtc_cali_cycling_data_vld(&self) -> RTC_CALI_CYCLING_DATA_VLD_R {
        RTC_CALI_CYCLING_DATA_VLD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 7:31 - Calibration value when cycles of clock to be calibrated reach TIMG_RTC_CALI_MAX, in unit of XTAL_CLK clock cycles."]
    #[inline(always)]
    pub fn rtc_cali_value(&self) -> RTC_CALI_VALUE_R {
        RTC_CALI_VALUE_R::new(((self.bits >> 7) & 0x01ff_ffff) as u32)
    }
}
#[doc = "RTC calibration configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtccalicfg1](index.html) module"]
pub struct RTCCALICFG1_SPEC;
impl crate::RegisterSpec for RTCCALICFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtccalicfg1::R](R) reader structure"]
impl crate::Readable for RTCCALICFG1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RTCCALICFG1 to value 0"]
impl crate::Resettable for RTCCALICFG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
