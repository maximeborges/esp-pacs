#[doc = "Register `DC_PRELOAD_EVICT_CNT` reader"]
pub struct R(crate::R<DC_PRELOAD_EVICT_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DC_PRELOAD_EVICT_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DC_PRELOAD_EVICT_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DC_PRELOAD_EVICT_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DC_PRELOAD_EVICT_CNT` reader - The bits are used to count the number of cache evictions by pre-load which include manual pre-load and conditional pre-load."]
pub struct DC_PRELOAD_EVICT_CNT_R(crate::FieldReader<u16>);
impl DC_PRELOAD_EVICT_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        DC_PRELOAD_EVICT_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DC_PRELOAD_EVICT_CNT_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - The bits are used to count the number of cache evictions by pre-load which include manual pre-load and conditional pre-load."]
    #[inline(always)]
    pub fn dc_preload_evict_cnt(&self) -> DC_PRELOAD_EVICT_CNT_R {
        DC_PRELOAD_EVICT_CNT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc_preload_evict_cnt](index.html) module"]
pub struct DC_PRELOAD_EVICT_CNT_SPEC;
impl crate::RegisterSpec for DC_PRELOAD_EVICT_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dc_preload_evict_cnt::R](R) reader structure"]
impl crate::Readable for DC_PRELOAD_EVICT_CNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DC_PRELOAD_EVICT_CNT to value 0"]
impl crate::Resettable for DC_PRELOAD_EVICT_CNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
