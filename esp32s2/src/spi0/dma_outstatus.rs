#[doc = "Register `DMA_OUTSTATUS` reader"]
pub struct R(crate::R<DMA_OUTSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_OUTSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_OUTSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_OUTSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DMA_OUTDSCR_ADDR` reader - SPI dma out descriptor address."]
pub struct DMA_OUTDSCR_ADDR_R(crate::FieldReader<u32, u32>);
impl DMA_OUTDSCR_ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        DMA_OUTDSCR_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_OUTDSCR_ADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_OUTDSCR_STATE` reader - SPI dma out descriptor state."]
pub struct DMA_OUTDSCR_STATE_R(crate::FieldReader<u8, u8>);
impl DMA_OUTDSCR_STATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DMA_OUTDSCR_STATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_OUTDSCR_STATE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_OUT_STATE` reader - SPI dma out data state."]
pub struct DMA_OUT_STATE_R(crate::FieldReader<u8, u8>);
impl DMA_OUT_STATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DMA_OUT_STATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_OUT_STATE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_OUTFIFO_CNT` reader - The remains of SPI dma outfifo data."]
pub struct DMA_OUTFIFO_CNT_R(crate::FieldReader<u8, u8>);
impl DMA_OUTFIFO_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DMA_OUTFIFO_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_OUTFIFO_CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_OUTFIFO_FULL` reader - SPI dma outfifo is full."]
pub struct DMA_OUTFIFO_FULL_R(crate::FieldReader<bool, bool>);
impl DMA_OUTFIFO_FULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_OUTFIFO_FULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_OUTFIFO_FULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_OUTFIFO_EMPTY` reader - SPI dma outfifo is empty."]
pub struct DMA_OUTFIFO_EMPTY_R(crate::FieldReader<bool, bool>);
impl DMA_OUTFIFO_EMPTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_OUTFIFO_EMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_OUTFIFO_EMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:17 - SPI dma out descriptor address."]
    #[inline(always)]
    pub fn dma_outdscr_addr(&self) -> DMA_OUTDSCR_ADDR_R {
        DMA_OUTDSCR_ADDR_R::new((self.bits & 0x0003_ffff) as u32)
    }
    #[doc = "Bits 18:19 - SPI dma out descriptor state."]
    #[inline(always)]
    pub fn dma_outdscr_state(&self) -> DMA_OUTDSCR_STATE_R {
        DMA_OUTDSCR_STATE_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:22 - SPI dma out data state."]
    #[inline(always)]
    pub fn dma_out_state(&self) -> DMA_OUT_STATE_R {
        DMA_OUT_STATE_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 23:29 - The remains of SPI dma outfifo data."]
    #[inline(always)]
    pub fn dma_outfifo_cnt(&self) -> DMA_OUTFIFO_CNT_R {
        DMA_OUTFIFO_CNT_R::new(((self.bits >> 23) & 0x7f) as u8)
    }
    #[doc = "Bit 30 - SPI dma outfifo is full."]
    #[inline(always)]
    pub fn dma_outfifo_full(&self) -> DMA_OUTFIFO_FULL_R {
        DMA_OUTFIFO_FULL_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - SPI dma outfifo is empty."]
    #[inline(always)]
    pub fn dma_outfifo_empty(&self) -> DMA_OUTFIFO_EMPTY_R {
        DMA_OUTFIFO_EMPTY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "SPI DMA TX status\n\nThis register you can [`read`]
(crate::generic::Reg::read). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_outstatus]
(index.html) module"]
pub struct DMA_OUTSTATUS_SPEC;
impl crate::RegisterSpec for DMA_OUTSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_outstatus::R]
(R) reader structure"]
impl crate::Readable for DMA_OUTSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMA_OUTSTATUS to value 0x8000_0000"]
impl crate::Resettable for DMA_OUTSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000_0000
    }
}
