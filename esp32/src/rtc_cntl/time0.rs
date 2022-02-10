#[doc = "Register `TIME0` reader"]
pub struct R(crate::R<TIME0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIME0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIME0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIME0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TIME_LO` reader - RTC timer low 32 bits"]
pub struct TIME_LO_R(crate::FieldReader<u32, u32>);
impl TIME_LO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        TIME_LO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIME_LO_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - RTC timer low 32 bits"]
    #[inline(always)]
    pub fn time_lo(&self) -> TIME_LO_R {
        TIME_LO_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`]
(crate::generic::Reg::read). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [time0]
(index.html) module"]
pub struct TIME0_SPEC;
impl crate::RegisterSpec for TIME0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [time0::R]
(R) reader structure"]
impl crate::Readable for TIME0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TIME0 to value 0"]
impl crate::Resettable for TIME0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
