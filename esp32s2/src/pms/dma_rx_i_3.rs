#[doc = "Register `DMA_RX_I_3` reader"]
pub struct R(crate::R<DMA_RX_I_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_RX_I_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_RX_I_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_RX_I_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DMA_RX_I_ILG_ST` reader - Record the illegitimate information of RX Copy DMA. \\[22:6\\]: store the bits \\[18:2\\] of address. \\[5\\]: if bits \\[31:19\\] of address are 0x7ff, then the bit value is 1, otherwise it is 0. \\[4\\]: 1 means write operation, 0 means read operation. \\[3:0\\]: RX Copy DMA bus byte enables."]
pub struct DMA_RX_I_ILG_ST_R(crate::FieldReader<u32>);
impl DMA_RX_I_ILG_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        DMA_RX_I_ILG_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_RX_I_ILG_ST_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:22 - Record the illegitimate information of RX Copy DMA. \\[22:6\\]: store the bits \\[18:2\\] of address. \\[5\\]: if bits \\[31:19\\] of address are 0x7ff, then the bit value is 1, otherwise it is 0. \\[4\\]: 1 means write operation, 0 means read operation. \\[3:0\\]: RX Copy DMA bus byte enables."]
    #[inline(always)]
    pub fn dma_rx_i_ilg_st(&self) -> DMA_RX_I_ILG_ST_R {
        DMA_RX_I_ILG_ST_R::new((self.bits & 0x007f_ffff) as u32)
    }
}
#[doc = "RX Copy DMA status register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_rx_i_3](index.html) module"]
pub struct DMA_RX_I_3_SPEC;
impl crate::RegisterSpec for DMA_RX_I_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_rx_i_3::R](R) reader structure"]
impl crate::Readable for DMA_RX_I_3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMA_RX_I_3 to value 0"]
impl crate::Resettable for DMA_RX_I_3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
