#[doc = "Register `RD_KEY4_DATA0` reader"]
pub struct R(crate::R<RD_KEY4_DATA0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_KEY4_DATA0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_KEY4_DATA0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_KEY4_DATA0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `KEY4_DATA0` reader - Stores the zeroth 32 bits of KEY4."]
pub struct KEY4_DATA0_R(crate::FieldReader<u32, u32>);
impl KEY4_DATA0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        KEY4_DATA0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY4_DATA0_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Stores the zeroth 32 bits of KEY4."]
    #[inline(always)]
    pub fn key4_data0(&self) -> KEY4_DATA0_R {
        KEY4_DATA0_R::new(self.bits)
    }
}
#[doc = "Register 0 of BLOCK8 (KEY4).\n\nThis register you can [`read`]
(crate::generic::Reg::read). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_key4_data0]
(index.html) module"]
pub struct RD_KEY4_DATA0_SPEC;
impl crate::RegisterSpec for RD_KEY4_DATA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_key4_data0::R]
(R) reader structure"]
impl crate::Readable for RD_KEY4_DATA0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_KEY4_DATA0 to value 0"]
impl crate::Resettable for RD_KEY4_DATA0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
