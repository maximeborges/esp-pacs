#[doc = "Register `INTR_STATUS_0` reader"]
pub struct R(crate::R<INTR_STATUS_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_STATUS_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_STATUS_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_STATUS_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INTR_STATUS_0` reader - this register store the status of the first 32 interrupt source"]
pub struct INTR_STATUS_0_R(crate::FieldReader<u32, u32>);
impl INTR_STATUS_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        INTR_STATUS_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTR_STATUS_0_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - this register store the status of the first 32 interrupt source"]
    #[inline(always)]
    pub fn intr_status_0(&self) -> INTR_STATUS_0_R {
        INTR_STATUS_0_R::new(self.bits)
    }
}
#[doc = "interrupt status register\n\nThis register you can [`read`]
(crate::generic::Reg::read). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_status_0]
(index.html) module"]
pub struct INTR_STATUS_0_SPEC;
impl crate::RegisterSpec for INTR_STATUS_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_status_0::R]
(R) reader structure"]
impl crate::Readable for INTR_STATUS_0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTR_STATUS_0 to value 0"]
impl crate::Resettable for INTR_STATUS_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
