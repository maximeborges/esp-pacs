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
#[doc = "Field `RTC_CALI_CYCLING_DATA_VLD` reader - rtc_cali_cycling_data_vld"]
pub struct RTC_CALI_CYCLING_DATA_VLD_R(crate::FieldReader<bool>);
impl RTC_CALI_CYCLING_DATA_VLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_CALI_CYCLING_DATA_VLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_CALI_CYCLING_DATA_VLD_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_CALI_VALUE` reader - rtc_cali_value"]
pub struct RTC_CALI_VALUE_R(crate::FieldReader<u32>);
impl RTC_CALI_VALUE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        RTC_CALI_VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_CALI_VALUE_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - rtc_cali_cycling_data_vld"]
    #[inline(always)]
    pub fn rtc_cali_cycling_data_vld(&self) -> RTC_CALI_CYCLING_DATA_VLD_R {
        RTC_CALI_CYCLING_DATA_VLD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 7:31 - rtc_cali_value"]
    #[inline(always)]
    pub fn rtc_cali_value(&self) -> RTC_CALI_VALUE_R {
        RTC_CALI_VALUE_R::new(((self.bits >> 7) & 0x01ff_ffff) as u32)
    }
}
#[doc = "TIMG_RTCCALICFG1_REG.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtccalicfg1](index.html) module"]
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
