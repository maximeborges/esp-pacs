#[doc = "Register `DMA_APBPERI_SDIO_PMS_CONSTRAIN_1` reader"]
pub struct R(crate::R<DMA_APBPERI_SDIO_PMS_CONSTRAIN_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_APBPERI_SDIO_PMS_CONSTRAIN_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_APBPERI_SDIO_PMS_CONSTRAIN_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_APBPERI_SDIO_PMS_CONSTRAIN_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_APBPERI_SDIO_PMS_CONSTRAIN_1` writer"]
pub struct W(crate::W<DMA_APBPERI_SDIO_PMS_CONSTRAIN_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_APBPERI_SDIO_PMS_CONSTRAIN_1_SPEC>;
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
impl From<crate::W<DMA_APBPERI_SDIO_PMS_CONSTRAIN_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_APBPERI_SDIO_PMS_CONSTRAIN_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_PMS_0` reader - sdio's permission(store,load) in data region0 of SRAM"]
pub struct DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_PMS_0_R(crate::FieldReader<u8, u8>);
impl DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_PMS_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_PMS_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_PMS_0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_PMS_0` writer - sdio's permission(store,load) in data region0 of SRAM"]
pub struct DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_PMS_0_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_PMS_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u32 & 3);
        self.w
    }
}
#[doc = "Field `DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_PMS_1` reader - sdio's permission(store,load) in data region1 of SRAM"]
pub struct DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_PMS_1_R(crate::FieldReader<u8, u8>);
impl DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_PMS_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_PMS_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_PMS_1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_PMS_1` writer - sdio's permission(store,load) in data region1 of SRAM"]
pub struct DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_PMS_1_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_PMS_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 2)) | ((value as u32 & 3) << 2);
        self.w
    }
}
#[doc = "Field `DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_PMS_2` reader - sdio's permission(store,load) in data region2 of SRAM"]
pub struct DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_PMS_2_R(crate::FieldReader<u8, u8>);
impl DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_PMS_2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_PMS_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_PMS_2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_PMS_2` writer - sdio's permission(store,load) in data region2 of SRAM"]
pub struct DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_PMS_2_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_PMS_2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 4)) | ((value as u32 & 3) << 4);
        self.w
    }
}
#[doc = "Field `DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_PMS_3` reader - sdio's permission(store,load) in data region3 of SRAM"]
pub struct DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_PMS_3_R(crate::FieldReader<u8, u8>);
impl DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_PMS_3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_PMS_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_PMS_3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_PMS_3` writer - sdio's permission(store,load) in data region3 of SRAM"]
pub struct DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_PMS_3_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_PMS_3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 6)) | ((value as u32 & 3) << 6);
        self.w
    }
}
#[doc = "Field `DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_0` reader - sdio's permission(store,load) in dcache data sram block0"]
pub struct DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_0_R(crate::FieldReader<u8, u8>);
impl DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_0` writer - sdio's permission(store,load) in dcache data sram block0"]
pub struct DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_0_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 8)) | ((value as u32 & 3) << 8);
        self.w
    }
}
#[doc = "Field `DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_1` reader - sdio's permission(store,load) in dcache data sram block1"]
pub struct DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_1_R(crate::FieldReader<u8, u8>);
impl DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_1` writer - sdio's permission(store,load) in dcache data sram block1"]
pub struct DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_1_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 10)) | ((value as u32 & 3) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - sdio's permission(store,load) in data region0 of SRAM"]
    #[inline(always)]
    pub fn dma_apbperi_sdio_pms_constrain_sram_pms_0(
        &self,
    ) -> DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_PMS_0_R {
        DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_PMS_0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - sdio's permission(store,load) in data region1 of SRAM"]
    #[inline(always)]
    pub fn dma_apbperi_sdio_pms_constrain_sram_pms_1(
        &self,
    ) -> DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_PMS_1_R {
        DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_PMS_1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - sdio's permission(store,load) in data region2 of SRAM"]
    #[inline(always)]
    pub fn dma_apbperi_sdio_pms_constrain_sram_pms_2(
        &self,
    ) -> DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_PMS_2_R {
        DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_PMS_2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - sdio's permission(store,load) in data region3 of SRAM"]
    #[inline(always)]
    pub fn dma_apbperi_sdio_pms_constrain_sram_pms_3(
        &self,
    ) -> DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_PMS_3_R {
        DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_PMS_3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - sdio's permission(store,load) in dcache data sram block0"]
    #[inline(always)]
    pub fn dma_apbperi_sdio_pms_constrain_sram_cachedataarray_pms_0(
        &self,
    ) -> DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_0_R {
        DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_0_R::new(
            ((self.bits >> 8) & 3) as u8,
        )
    }
    #[doc = "Bits 10:11 - sdio's permission(store,load) in dcache data sram block1"]
    #[inline(always)]
    pub fn dma_apbperi_sdio_pms_constrain_sram_cachedataarray_pms_1(
        &self,
    ) -> DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_1_R {
        DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_1_R::new(
            ((self.bits >> 10) & 3) as u8,
        )
    }
}
impl W {
    #[doc = "Bits 0:1 - sdio's permission(store,load) in data region0 of SRAM"]
    #[inline(always)]
    pub fn dma_apbperi_sdio_pms_constrain_sram_pms_0(
        &mut self,
    ) -> DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_PMS_0_W {
        DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_PMS_0_W { w: self }
    }
    #[doc = "Bits 2:3 - sdio's permission(store,load) in data region1 of SRAM"]
    #[inline(always)]
    pub fn dma_apbperi_sdio_pms_constrain_sram_pms_1(
        &mut self,
    ) -> DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_PMS_1_W {
        DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_PMS_1_W { w: self }
    }
    #[doc = "Bits 4:5 - sdio's permission(store,load) in data region2 of SRAM"]
    #[inline(always)]
    pub fn dma_apbperi_sdio_pms_constrain_sram_pms_2(
        &mut self,
    ) -> DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_PMS_2_W {
        DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_PMS_2_W { w: self }
    }
    #[doc = "Bits 6:7 - sdio's permission(store,load) in data region3 of SRAM"]
    #[inline(always)]
    pub fn dma_apbperi_sdio_pms_constrain_sram_pms_3(
        &mut self,
    ) -> DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_PMS_3_W {
        DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_PMS_3_W { w: self }
    }
    #[doc = "Bits 8:9 - sdio's permission(store,load) in dcache data sram block0"]
    #[inline(always)]
    pub fn dma_apbperi_sdio_pms_constrain_sram_cachedataarray_pms_0(
        &mut self,
    ) -> DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_0_W {
        DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_0_W { w: self }
    }
    #[doc = "Bits 10:11 - sdio's permission(store,load) in dcache data sram block1"]
    #[inline(always)]
    pub fn dma_apbperi_sdio_pms_constrain_sram_cachedataarray_pms_1(
        &mut self,
    ) -> DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_1_W {
        DMA_APBPERI_SDIO_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sdio dma permission configuration register 1.\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_apbperi_sdio_pms_constrain_1]
(index.html) module"]
pub struct DMA_APBPERI_SDIO_PMS_CONSTRAIN_1_SPEC;
impl crate::RegisterSpec for DMA_APBPERI_SDIO_PMS_CONSTRAIN_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_apbperi_sdio_pms_constrain_1::R]
(R) reader structure"]
impl crate::Readable for DMA_APBPERI_SDIO_PMS_CONSTRAIN_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_apbperi_sdio_pms_constrain_1::W]
(W) writer structure"]
impl crate::Writable for DMA_APBPERI_SDIO_PMS_CONSTRAIN_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_APBPERI_SDIO_PMS_CONSTRAIN_1 to value 0x0fff"]
impl crate::Resettable for DMA_APBPERI_SDIO_PMS_CONSTRAIN_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0fff
    }
}
