#[doc = "Register `RTC_SLP_REJECT_CAUSE` reader"]
pub struct R(crate::R<RTC_SLP_REJECT_CAUSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_SLP_REJECT_CAUSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_SLP_REJECT_CAUSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_SLP_REJECT_CAUSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `REJECT_CAUSE` reader - sleep reject cause"]
pub struct REJECT_CAUSE_R(crate::FieldReader<u32, u32>);
impl REJECT_CAUSE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        REJECT_CAUSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REJECT_CAUSE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:17 - sleep reject cause"]
    #[inline(always)]
    pub fn reject_cause(&self) -> REJECT_CAUSE_R {
        REJECT_CAUSE_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
#[doc = "get reject casue\n\nThis register you can [`read`]
(crate::generic::Reg::read). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_slp_reject_cause]
(index.html) module"]
pub struct RTC_SLP_REJECT_CAUSE_SPEC;
impl crate::RegisterSpec for RTC_SLP_REJECT_CAUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_slp_reject_cause::R]
(R) reader structure"]
impl crate::Readable for RTC_SLP_REJECT_CAUSE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RTC_SLP_REJECT_CAUSE to value 0"]
impl crate::Resettable for RTC_SLP_REJECT_CAUSE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
