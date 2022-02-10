#[doc = "Register `RTC_DIAG0` reader"]
pub struct R(crate::R<RTC_DIAG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_DIAG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_DIAG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_DIAG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RTC_LOW_POWER_DIAG1` reader - "]
pub struct RTC_LOW_POWER_DIAG1_R(crate::FieldReader<u32, u32>);
impl RTC_LOW_POWER_DIAG1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        RTC_LOW_POWER_DIAG1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_LOW_POWER_DIAG1_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rtc_low_power_diag1(&self) -> RTC_LOW_POWER_DIAG1_R {
        RTC_LOW_POWER_DIAG1_R::new(self.bits)
    }
}
#[doc = "rtc configure register\n\nThis register you can [`read`]
(crate::generic::Reg::read). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_diag0]
(index.html) module"]
pub struct RTC_DIAG0_SPEC;
impl crate::RegisterSpec for RTC_DIAG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_diag0::R]
(R) reader structure"]
impl crate::Readable for RTC_DIAG0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RTC_DIAG0 to value 0"]
impl crate::Resettable for RTC_DIAG0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
