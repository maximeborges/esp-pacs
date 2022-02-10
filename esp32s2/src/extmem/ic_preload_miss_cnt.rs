#[doc = "Register `IC_PRELOAD_MISS_CNT` reader"]
pub struct R(crate::R<IC_PRELOAD_MISS_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IC_PRELOAD_MISS_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IC_PRELOAD_MISS_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IC_PRELOAD_MISS_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IC_PRELOAD_MISS_CNT` reader - The bits are used to count the number of missed pre-load which include manual pre-load and conditional pre-load."]
pub struct IC_PRELOAD_MISS_CNT_R(crate::FieldReader<u16, u16>);
impl IC_PRELOAD_MISS_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        IC_PRELOAD_MISS_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC_PRELOAD_MISS_CNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - The bits are used to count the number of missed pre-load which include manual pre-load and conditional pre-load."]
    #[inline(always)]
    pub fn ic_preload_miss_cnt(&self) -> IC_PRELOAD_MISS_CNT_R {
        IC_PRELOAD_MISS_CNT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "register description\n\nThis register you can [`read`]
(crate::generic::Reg::read). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic_preload_miss_cnt]
(index.html) module"]
pub struct IC_PRELOAD_MISS_CNT_SPEC;
impl crate::RegisterSpec for IC_PRELOAD_MISS_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ic_preload_miss_cnt::R]
(R) reader structure"]
impl crate::Readable for IC_PRELOAD_MISS_CNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IC_PRELOAD_MISS_CNT to value 0"]
impl crate::Resettable for IC_PRELOAD_MISS_CNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
