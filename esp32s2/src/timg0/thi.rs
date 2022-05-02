#[doc = "Register `T%sHI` reader"]
pub struct R(crate::R<THI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<THI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<THI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<THI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `T0_HI` reader - After writing to TIMG_T%sUPDATE_REG, the high 32 bits of the time-base counter of timer %s can be read here."]
pub struct T0_HI_R(crate::FieldReader<u32>);
impl T0_HI_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        T0_HI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T0_HI_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - After writing to TIMG_T%sUPDATE_REG, the high 32 bits of the time-base counter of timer %s can be read here."]
    #[inline(always)]
    pub fn t0_hi(&self) -> T0_HI_R {
        T0_HI_R::new(self.bits)
    }
}
#[doc = "Timer %s current value, high 32 bits\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [thi](index.html) module"]
pub struct THI_SPEC;
impl crate::RegisterSpec for THI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [thi::R](R) reader structure"]
impl crate::Readable for THI_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets T%sHI to value 0"]
impl crate::Resettable for THI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
