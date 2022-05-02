#[doc = "Register `PRO_INTRUSION_STATUS` reader"]
pub struct R(crate::R<PRO_INTRUSION_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_INTRUSION_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_INTRUSION_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_INTRUSION_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PRO_INTRUSION_RECORD` reader - "]
pub struct PRO_INTRUSION_RECORD_R(crate::FieldReader<u8>);
impl PRO_INTRUSION_RECORD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PRO_INTRUSION_RECORD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_INTRUSION_RECORD_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn pro_intrusion_record(&self) -> PRO_INTRUSION_RECORD_R {
        PRO_INTRUSION_RECORD_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_intrusion_status](index.html) module"]
pub struct PRO_INTRUSION_STATUS_SPEC;
impl crate::RegisterSpec for PRO_INTRUSION_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_intrusion_status::R](R) reader structure"]
impl crate::Readable for PRO_INTRUSION_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PRO_INTRUSION_STATUS to value 0"]
impl crate::Resettable for PRO_INTRUSION_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
