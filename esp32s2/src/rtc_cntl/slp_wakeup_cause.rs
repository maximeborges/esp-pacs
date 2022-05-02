#[doc = "Register `SLP_WAKEUP_CAUSE` reader"]
pub struct R(crate::R<SLP_WAKEUP_CAUSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLP_WAKEUP_CAUSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLP_WAKEUP_CAUSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLP_WAKEUP_CAUSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WAKEUP_CAUSE` reader - Stores the wakeup cause."]
pub struct WAKEUP_CAUSE_R(crate::FieldReader<u32>);
impl WAKEUP_CAUSE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        WAKEUP_CAUSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAKEUP_CAUSE_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:16 - Stores the wakeup cause."]
    #[inline(always)]
    pub fn wakeup_cause(&self) -> WAKEUP_CAUSE_R {
        WAKEUP_CAUSE_R::new((self.bits & 0x0001_ffff) as u32)
    }
}
#[doc = "Stores the sleep-to-wakeup cause.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slp_wakeup_cause](index.html) module"]
pub struct SLP_WAKEUP_CAUSE_SPEC;
impl crate::RegisterSpec for SLP_WAKEUP_CAUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slp_wakeup_cause::R](R) reader structure"]
impl crate::Readable for SLP_WAKEUP_CAUSE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SLP_WAKEUP_CAUSE to value 0"]
impl crate::Resettable for SLP_WAKEUP_CAUSE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
