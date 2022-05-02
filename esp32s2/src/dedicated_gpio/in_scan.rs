#[doc = "Register `IN_SCAN` reader"]
pub struct R(crate::R<IN_SCAN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IN_SCAN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IN_SCAN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IN_SCAN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IN_STATUS` reader - GPIO input value after configured by DEDIC_GPIO_IN_DLY_REG."]
pub struct IN_STATUS_R(crate::FieldReader<u8>);
impl IN_STATUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IN_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN_STATUS_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - GPIO input value after configured by DEDIC_GPIO_IN_DLY_REG."]
    #[inline(always)]
    pub fn in_status(&self) -> IN_STATUS_R {
        IN_STATUS_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Dedicated GPIO input status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [in_scan](index.html) module"]
pub struct IN_SCAN_SPEC;
impl crate::RegisterSpec for IN_SCAN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [in_scan::R](R) reader structure"]
impl crate::Readable for IN_SCAN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IN_SCAN to value 0"]
impl crate::Resettable for IN_SCAN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
