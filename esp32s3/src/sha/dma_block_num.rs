#[doc = "Register `DMA_BLOCK_NUM` reader"]
pub struct R(crate::R<DMA_BLOCK_NUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_BLOCK_NUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_BLOCK_NUM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_BLOCK_NUM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_BLOCK_NUM` writer"]
pub struct W(crate::W<DMA_BLOCK_NUM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_BLOCK_NUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DMA_BLOCK_NUM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_BLOCK_NUM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA_BLOCK_NUM` reader - dma-sha block number"]
pub struct DMA_BLOCK_NUM_R(crate::FieldReader<u8, u8>);
impl DMA_BLOCK_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DMA_BLOCK_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_BLOCK_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_BLOCK_NUM` writer - dma-sha block number"]
pub struct DMA_BLOCK_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_BLOCK_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - dma-sha block number"]
    #[inline(always)]
    pub fn dma_block_num(&self) -> DMA_BLOCK_NUM_R {
        DMA_BLOCK_NUM_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - dma-sha block number"]
    #[inline(always)]
    pub fn dma_block_num(&mut self) -> DMA_BLOCK_NUM_W {
        DMA_BLOCK_NUM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA configuration register 0.\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_block_num]
(index.html) module"]
pub struct DMA_BLOCK_NUM_SPEC;
impl crate::RegisterSpec for DMA_BLOCK_NUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_block_num::R]
(R) reader structure"]
impl crate::Readable for DMA_BLOCK_NUM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_block_num::W]
(W) writer structure"]
impl crate::Writable for DMA_BLOCK_NUM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_BLOCK_NUM to value 0"]
impl crate::Resettable for DMA_BLOCK_NUM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
