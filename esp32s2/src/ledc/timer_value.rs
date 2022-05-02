#[doc = "Register `TIMER%s_VALUE` reader"]
pub struct R(crate::R<TIMER_VALUE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER_VALUE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER_VALUE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER_VALUE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TIMER0_CNT` reader - This register stores the current counter value of timer %s."]
pub struct TIMER0_CNT_R(crate::FieldReader<u16>);
impl TIMER0_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TIMER0_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER0_CNT_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:13 - This register stores the current counter value of timer %s."]
    #[inline(always)]
    pub fn timer0_cnt(&self) -> TIMER0_CNT_R {
        TIMER0_CNT_R::new((self.bits & 0x3fff) as u16)
    }
}
#[doc = "Timer %s current counter value\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer_value](index.html) module"]
pub struct TIMER_VALUE_SPEC;
impl crate::RegisterSpec for TIMER_VALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer_value::R](R) reader structure"]
impl crate::Readable for TIMER_VALUE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TIMER%s_VALUE to value 0"]
impl crate::Resettable for TIMER_VALUE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
