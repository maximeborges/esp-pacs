#[doc = "Register `DMA_OUT_CH1_INT_MAP` reader"]
pub struct R(crate::R<DMA_OUT_CH1_INT_MAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_OUT_CH1_INT_MAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_OUT_CH1_INT_MAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_OUT_CH1_INT_MAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_OUT_CH1_INT_MAP` writer"]
pub struct W(crate::W<DMA_OUT_CH1_INT_MAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_OUT_CH1_INT_MAP_SPEC>;
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
impl From<crate::W<DMA_OUT_CH1_INT_MAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_OUT_CH1_INT_MAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA_OUT_CH1_INT_MAP` reader - this register used to map dma_out_ch1 interrupt to one of core1's external interrupt"]
pub struct DMA_OUT_CH1_INT_MAP_R(crate::FieldReader<u8>);
impl DMA_OUT_CH1_INT_MAP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DMA_OUT_CH1_INT_MAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_OUT_CH1_INT_MAP_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_OUT_CH1_INT_MAP` writer - this register used to map dma_out_ch1 interrupt to one of core1's external interrupt"]
pub struct DMA_OUT_CH1_INT_MAP_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_OUT_CH1_INT_MAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - this register used to map dma_out_ch1 interrupt to one of core1's external interrupt"]
    #[inline(always)]
    pub fn dma_out_ch1_int_map(&self) -> DMA_OUT_CH1_INT_MAP_R {
        DMA_OUT_CH1_INT_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - this register used to map dma_out_ch1 interrupt to one of core1's external interrupt"]
    #[inline(always)]
    pub fn dma_out_ch1_int_map(&mut self) -> DMA_OUT_CH1_INT_MAP_W {
        DMA_OUT_CH1_INT_MAP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "dma_out_ch1 interrupt configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_out_ch1_int_map](index.html) module"]
pub struct DMA_OUT_CH1_INT_MAP_SPEC;
impl crate::RegisterSpec for DMA_OUT_CH1_INT_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_out_ch1_int_map::R](R) reader structure"]
impl crate::Readable for DMA_OUT_CH1_INT_MAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_out_ch1_int_map::W](W) writer structure"]
impl crate::Writable for DMA_OUT_CH1_INT_MAP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_OUT_CH1_INT_MAP to value 0x10"]
impl crate::Resettable for DMA_OUT_CH1_INT_MAP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
