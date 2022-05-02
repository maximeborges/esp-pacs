#[doc = "Register `T0LO` reader"]
pub struct R(crate::R<T0LO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<T0LO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<T0LO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<T0LO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `T0_LO` reader - t0_lo"]
pub struct T0_LO_R(crate::FieldReader<u32>);
impl T0_LO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        T0_LO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T0_LO_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - t0_lo"]
    #[inline(always)]
    pub fn t0_lo(&self) -> T0_LO_R {
        T0_LO_R::new(self.bits)
    }
}
#[doc = "TIMG_T0LO_REG.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t0lo](index.html) module"]
pub struct T0LO_SPEC;
impl crate::RegisterSpec for T0LO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [t0lo::R](R) reader structure"]
impl crate::Readable for T0LO_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets T0LO to value 0"]
impl crate::Resettable for T0LO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
