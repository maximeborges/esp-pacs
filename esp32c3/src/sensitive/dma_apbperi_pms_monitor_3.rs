#[doc = "Register `DMA_APBPERI_PMS_MONITOR_3` reader"]
pub struct R(crate::R<DMA_APBPERI_PMS_MONITOR_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_APBPERI_PMS_MONITOR_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_APBPERI_PMS_MONITOR_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_APBPERI_PMS_MONITOR_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_WR` reader - dma_apbperi_pms_monitor_violate_status_wr"]
pub struct DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_WR_R(crate::FieldReader<bool, bool>);
impl DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_WR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_WR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_WR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_BYTEEN` reader - dma_apbperi_pms_monitor_violate_status_byteen"]
pub struct DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_BYTEEN_R(crate::FieldReader<u8, u8>);
impl DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_BYTEEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_BYTEEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_BYTEEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - dma_apbperi_pms_monitor_violate_status_wr"]
    #[inline(always)]
    pub fn dma_apbperi_pms_monitor_violate_status_wr(
        &self,
    ) -> DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_WR_R {
        DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_WR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - dma_apbperi_pms_monitor_violate_status_byteen"]
    #[inline(always)]
    pub fn dma_apbperi_pms_monitor_violate_status_byteen(
        &self,
    ) -> DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_BYTEEN_R {
        DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_BYTEEN_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
}
#[doc = "SENSITIVE_DMA_APBPERI_PMS_MONITOR_3_REG\n\nThis register you can [`read`]
(crate::generic::Reg::read). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_apbperi_pms_monitor_3]
(index.html) module"]
pub struct DMA_APBPERI_PMS_MONITOR_3_SPEC;
impl crate::RegisterSpec for DMA_APBPERI_PMS_MONITOR_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_apbperi_pms_monitor_3::R]
(R) reader structure"]
impl crate::Readable for DMA_APBPERI_PMS_MONITOR_3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMA_APBPERI_PMS_MONITOR_3 to value 0"]
impl crate::Resettable for DMA_APBPERI_PMS_MONITOR_3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
