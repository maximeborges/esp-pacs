#[doc = "Register `ACPU_INT` reader"]
pub struct R(crate::R<ACPU_INT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACPU_INT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACPU_INT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACPU_INT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `APPCPU_INT` reader - GPIO0~31 APP CPU interrupt status"]
pub struct APPCPU_INT_R(crate::FieldReader<u32>);
impl APPCPU_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        APPCPU_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APPCPU_INT_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - GPIO0~31 APP CPU interrupt status"]
    #[inline(always)]
    pub fn appcpu_int(&self) -> APPCPU_INT_R {
        APPCPU_INT_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acpu_int](index.html) module"]
pub struct ACPU_INT_SPEC;
impl crate::RegisterSpec for ACPU_INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acpu_int::R](R) reader structure"]
impl crate::Readable for ACPU_INT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ACPU_INT to value 0"]
impl crate::Resettable for ACPU_INT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
