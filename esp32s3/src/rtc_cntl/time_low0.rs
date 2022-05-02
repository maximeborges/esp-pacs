#[doc = "Register `TIME_LOW0` reader"]
pub struct R(crate::R<TIME_LOW0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIME_LOW0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIME_LOW0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIME_LOW0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RTC_TIMER_VALUE0_LOW` reader - RTC timer low 32 bits"]
pub struct RTC_TIMER_VALUE0_LOW_R(crate::FieldReader<u32>);
impl RTC_TIMER_VALUE0_LOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        RTC_TIMER_VALUE0_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_TIMER_VALUE0_LOW_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - RTC timer low 32 bits"]
    #[inline(always)]
    pub fn rtc_timer_value0_low(&self) -> RTC_TIMER_VALUE0_LOW_R {
        RTC_TIMER_VALUE0_LOW_R::new(self.bits)
    }
}
#[doc = "read rtc_main timer low bits\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [time_low0](index.html) module"]
pub struct TIME_LOW0_SPEC;
impl crate::RegisterSpec for TIME_LOW0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [time_low0::R](R) reader structure"]
impl crate::Readable for TIME_LOW0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TIME_LOW0 to value 0"]
impl crate::Resettable for TIME_LOW0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
