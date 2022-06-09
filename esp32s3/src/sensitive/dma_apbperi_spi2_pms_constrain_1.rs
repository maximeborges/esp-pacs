#[doc = "Register `DMA_APBPERI_SPI2_PMS_CONSTRAIN_1` reader"]
pub struct R(crate::R<DMA_APBPERI_SPI2_PMS_CONSTRAIN_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_APBPERI_SPI2_PMS_CONSTRAIN_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_APBPERI_SPI2_PMS_CONSTRAIN_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_APBPERI_SPI2_PMS_CONSTRAIN_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_APBPERI_SPI2_PMS_CONSTRAIN_1` writer"]
pub struct W(crate::W<DMA_APBPERI_SPI2_PMS_CONSTRAIN_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_APBPERI_SPI2_PMS_CONSTRAIN_1_SPEC>;
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
impl From<crate::W<DMA_APBPERI_SPI2_PMS_CONSTRAIN_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_APBPERI_SPI2_PMS_CONSTRAIN_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA_APBPERI_SPI2_PMS_CONSTRAIN_SRAM_PMS_0` reader - spi2's permission(store,load) in data region0 of SRAM"]
pub type DMA_APBPERI_SPI2_PMS_CONSTRAIN_SRAM_PMS_0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMA_APBPERI_SPI2_PMS_CONSTRAIN_SRAM_PMS_0` writer - spi2's permission(store,load) in data region0 of SRAM"]
pub type DMA_APBPERI_SPI2_PMS_CONSTRAIN_SRAM_PMS_0_W<'a> =
    crate::FieldWriter<'a, u32, DMA_APBPERI_SPI2_PMS_CONSTRAIN_1_SPEC, u8, u8, 2, 0>;
#[doc = "Field `DMA_APBPERI_SPI2_PMS_CONSTRAIN_SRAM_PMS_1` reader - spi2's permission(store,load) in data region1 of SRAM"]
pub type DMA_APBPERI_SPI2_PMS_CONSTRAIN_SRAM_PMS_1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMA_APBPERI_SPI2_PMS_CONSTRAIN_SRAM_PMS_1` writer - spi2's permission(store,load) in data region1 of SRAM"]
pub type DMA_APBPERI_SPI2_PMS_CONSTRAIN_SRAM_PMS_1_W<'a> =
    crate::FieldWriter<'a, u32, DMA_APBPERI_SPI2_PMS_CONSTRAIN_1_SPEC, u8, u8, 2, 2>;
#[doc = "Field `DMA_APBPERI_SPI2_PMS_CONSTRAIN_SRAM_PMS_2` reader - spi2's permission(store,load) in data region2 of SRAM"]
pub type DMA_APBPERI_SPI2_PMS_CONSTRAIN_SRAM_PMS_2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMA_APBPERI_SPI2_PMS_CONSTRAIN_SRAM_PMS_2` writer - spi2's permission(store,load) in data region2 of SRAM"]
pub type DMA_APBPERI_SPI2_PMS_CONSTRAIN_SRAM_PMS_2_W<'a> =
    crate::FieldWriter<'a, u32, DMA_APBPERI_SPI2_PMS_CONSTRAIN_1_SPEC, u8, u8, 2, 4>;
#[doc = "Field `DMA_APBPERI_SPI2_PMS_CONSTRAIN_SRAM_PMS_3` reader - spi2's permission(store,load) in data region3 of SRAM"]
pub type DMA_APBPERI_SPI2_PMS_CONSTRAIN_SRAM_PMS_3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMA_APBPERI_SPI2_PMS_CONSTRAIN_SRAM_PMS_3` writer - spi2's permission(store,load) in data region3 of SRAM"]
pub type DMA_APBPERI_SPI2_PMS_CONSTRAIN_SRAM_PMS_3_W<'a> =
    crate::FieldWriter<'a, u32, DMA_APBPERI_SPI2_PMS_CONSTRAIN_1_SPEC, u8, u8, 2, 6>;
#[doc = "Field `DMA_APBPERI_SPI2_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_0` reader - spi2's permission(store,load) in dcache data sram block0"]
pub type DMA_APBPERI_SPI2_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMA_APBPERI_SPI2_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_0` writer - spi2's permission(store,load) in dcache data sram block0"]
pub type DMA_APBPERI_SPI2_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_0_W<'a> =
    crate::FieldWriter<'a, u32, DMA_APBPERI_SPI2_PMS_CONSTRAIN_1_SPEC, u8, u8, 2, 8>;
#[doc = "Field `DMA_APBPERI_SPI2_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_1` reader - spi2's permission(store,load) in dcache data sram block1"]
pub type DMA_APBPERI_SPI2_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMA_APBPERI_SPI2_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_1` writer - spi2's permission(store,load) in dcache data sram block1"]
pub type DMA_APBPERI_SPI2_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_1_W<'a> =
    crate::FieldWriter<'a, u32, DMA_APBPERI_SPI2_PMS_CONSTRAIN_1_SPEC, u8, u8, 2, 10>;
impl R {
    #[doc = "Bits 0:1 - spi2's permission(store,load) in data region0 of SRAM"]
    #[inline(always)]
    pub fn dma_apbperi_spi2_pms_constrain_sram_pms_0(
        &self,
    ) -> DMA_APBPERI_SPI2_PMS_CONSTRAIN_SRAM_PMS_0_R {
        DMA_APBPERI_SPI2_PMS_CONSTRAIN_SRAM_PMS_0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - spi2's permission(store,load) in data region1 of SRAM"]
    #[inline(always)]
    pub fn dma_apbperi_spi2_pms_constrain_sram_pms_1(
        &self,
    ) -> DMA_APBPERI_SPI2_PMS_CONSTRAIN_SRAM_PMS_1_R {
        DMA_APBPERI_SPI2_PMS_CONSTRAIN_SRAM_PMS_1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - spi2's permission(store,load) in data region2 of SRAM"]
    #[inline(always)]
    pub fn dma_apbperi_spi2_pms_constrain_sram_pms_2(
        &self,
    ) -> DMA_APBPERI_SPI2_PMS_CONSTRAIN_SRAM_PMS_2_R {
        DMA_APBPERI_SPI2_PMS_CONSTRAIN_SRAM_PMS_2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - spi2's permission(store,load) in data region3 of SRAM"]
    #[inline(always)]
    pub fn dma_apbperi_spi2_pms_constrain_sram_pms_3(
        &self,
    ) -> DMA_APBPERI_SPI2_PMS_CONSTRAIN_SRAM_PMS_3_R {
        DMA_APBPERI_SPI2_PMS_CONSTRAIN_SRAM_PMS_3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - spi2's permission(store,load) in dcache data sram block0"]
    #[inline(always)]
    pub fn dma_apbperi_spi2_pms_constrain_sram_cachedataarray_pms_0(
        &self,
    ) -> DMA_APBPERI_SPI2_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_0_R {
        DMA_APBPERI_SPI2_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_0_R::new(
            ((self.bits >> 8) & 3) as u8,
        )
    }
    #[doc = "Bits 10:11 - spi2's permission(store,load) in dcache data sram block1"]
    #[inline(always)]
    pub fn dma_apbperi_spi2_pms_constrain_sram_cachedataarray_pms_1(
        &self,
    ) -> DMA_APBPERI_SPI2_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_1_R {
        DMA_APBPERI_SPI2_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_1_R::new(
            ((self.bits >> 10) & 3) as u8,
        )
    }
}
impl W {
    #[doc = "Bits 0:1 - spi2's permission(store,load) in data region0 of SRAM"]
    #[inline(always)]
    pub fn dma_apbperi_spi2_pms_constrain_sram_pms_0(
        &mut self,
    ) -> DMA_APBPERI_SPI2_PMS_CONSTRAIN_SRAM_PMS_0_W {
        DMA_APBPERI_SPI2_PMS_CONSTRAIN_SRAM_PMS_0_W::new(self)
    }
    #[doc = "Bits 2:3 - spi2's permission(store,load) in data region1 of SRAM"]
    #[inline(always)]
    pub fn dma_apbperi_spi2_pms_constrain_sram_pms_1(
        &mut self,
    ) -> DMA_APBPERI_SPI2_PMS_CONSTRAIN_SRAM_PMS_1_W {
        DMA_APBPERI_SPI2_PMS_CONSTRAIN_SRAM_PMS_1_W::new(self)
    }
    #[doc = "Bits 4:5 - spi2's permission(store,load) in data region2 of SRAM"]
    #[inline(always)]
    pub fn dma_apbperi_spi2_pms_constrain_sram_pms_2(
        &mut self,
    ) -> DMA_APBPERI_SPI2_PMS_CONSTRAIN_SRAM_PMS_2_W {
        DMA_APBPERI_SPI2_PMS_CONSTRAIN_SRAM_PMS_2_W::new(self)
    }
    #[doc = "Bits 6:7 - spi2's permission(store,load) in data region3 of SRAM"]
    #[inline(always)]
    pub fn dma_apbperi_spi2_pms_constrain_sram_pms_3(
        &mut self,
    ) -> DMA_APBPERI_SPI2_PMS_CONSTRAIN_SRAM_PMS_3_W {
        DMA_APBPERI_SPI2_PMS_CONSTRAIN_SRAM_PMS_3_W::new(self)
    }
    #[doc = "Bits 8:9 - spi2's permission(store,load) in dcache data sram block0"]
    #[inline(always)]
    pub fn dma_apbperi_spi2_pms_constrain_sram_cachedataarray_pms_0(
        &mut self,
    ) -> DMA_APBPERI_SPI2_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_0_W {
        DMA_APBPERI_SPI2_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_0_W::new(self)
    }
    #[doc = "Bits 10:11 - spi2's permission(store,load) in dcache data sram block1"]
    #[inline(always)]
    pub fn dma_apbperi_spi2_pms_constrain_sram_cachedataarray_pms_1(
        &mut self,
    ) -> DMA_APBPERI_SPI2_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_1_W {
        DMA_APBPERI_SPI2_PMS_CONSTRAIN_SRAM_CACHEDATAARRAY_PMS_1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "spi2 dma permission configuration register 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_apbperi_spi2_pms_constrain_1](index.html) module"]
pub struct DMA_APBPERI_SPI2_PMS_CONSTRAIN_1_SPEC;
impl crate::RegisterSpec for DMA_APBPERI_SPI2_PMS_CONSTRAIN_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_apbperi_spi2_pms_constrain_1::R](R) reader structure"]
impl crate::Readable for DMA_APBPERI_SPI2_PMS_CONSTRAIN_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_apbperi_spi2_pms_constrain_1::W](W) writer structure"]
impl crate::Writable for DMA_APBPERI_SPI2_PMS_CONSTRAIN_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_APBPERI_SPI2_PMS_CONSTRAIN_1 to value 0x0fff"]
impl crate::Resettable for DMA_APBPERI_SPI2_PMS_CONSTRAIN_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0fff
    }
}
