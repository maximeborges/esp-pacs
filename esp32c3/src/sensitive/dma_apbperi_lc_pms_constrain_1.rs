#[doc = "Register `DMA_APBPERI_LC_PMS_CONSTRAIN_1` reader"]
pub struct R(crate::R<DMA_APBPERI_LC_PMS_CONSTRAIN_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_APBPERI_LC_PMS_CONSTRAIN_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_APBPERI_LC_PMS_CONSTRAIN_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_APBPERI_LC_PMS_CONSTRAIN_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_APBPERI_LC_PMS_CONSTRAIN_1` writer"]
pub struct W(crate::W<DMA_APBPERI_LC_PMS_CONSTRAIN_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_APBPERI_LC_PMS_CONSTRAIN_1_SPEC>;
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
impl From<crate::W<DMA_APBPERI_LC_PMS_CONSTRAIN_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_APBPERI_LC_PMS_CONSTRAIN_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0` reader - dma_apbperi_lc_pms_constrain_sram_world_0_pms_0"]
pub struct DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0_R(crate::FieldReader<u8>);
impl DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0` writer - dma_apbperi_lc_pms_constrain_sram_world_0_pms_0"]
pub struct DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u32 & 3);
        self.w
    }
}
#[doc = "Field `DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1` reader - dma_apbperi_lc_pms_constrain_sram_world_0_pms_1"]
pub struct DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1_R(crate::FieldReader<u8>);
impl DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1` writer - dma_apbperi_lc_pms_constrain_sram_world_0_pms_1"]
pub struct DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 2)) | ((value as u32 & 3) << 2);
        self.w
    }
}
#[doc = "Field `DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2` reader - dma_apbperi_lc_pms_constrain_sram_world_0_pms_2"]
pub struct DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2_R(crate::FieldReader<u8>);
impl DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2` writer - dma_apbperi_lc_pms_constrain_sram_world_0_pms_2"]
pub struct DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 4)) | ((value as u32 & 3) << 4);
        self.w
    }
}
#[doc = "Field `DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3` reader - dma_apbperi_lc_pms_constrain_sram_world_0_pms_3"]
pub struct DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3_R(crate::FieldReader<u8>);
impl DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3` writer - dma_apbperi_lc_pms_constrain_sram_world_0_pms_3"]
pub struct DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 6)) | ((value as u32 & 3) << 6);
        self.w
    }
}
#[doc = "Field `DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_0` reader - dma_apbperi_lc_pms_constrain_sram_world_1_pms_0"]
pub struct DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_0_R(crate::FieldReader<u8>);
impl DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_0_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_0` writer - dma_apbperi_lc_pms_constrain_sram_world_1_pms_0"]
pub struct DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_0_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 12)) | ((value as u32 & 3) << 12);
        self.w
    }
}
#[doc = "Field `DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_1` reader - dma_apbperi_lc_pms_constrain_sram_world_1_pms_1"]
pub struct DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_1_R(crate::FieldReader<u8>);
impl DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_1_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_1` writer - dma_apbperi_lc_pms_constrain_sram_world_1_pms_1"]
pub struct DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_1_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 14)) | ((value as u32 & 3) << 14);
        self.w
    }
}
#[doc = "Field `DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_2` reader - dma_apbperi_lc_pms_constrain_sram_world_1_pms_2"]
pub struct DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_2_R(crate::FieldReader<u8>);
impl DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_2_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_2` writer - dma_apbperi_lc_pms_constrain_sram_world_1_pms_2"]
pub struct DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_2_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 16)) | ((value as u32 & 3) << 16);
        self.w
    }
}
#[doc = "Field `DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_3` reader - dma_apbperi_lc_pms_constrain_sram_world_1_pms_3"]
pub struct DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_3_R(crate::FieldReader<u8>);
impl DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_3_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_3` writer - dma_apbperi_lc_pms_constrain_sram_world_1_pms_3"]
pub struct DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_3_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 18)) | ((value as u32 & 3) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - dma_apbperi_lc_pms_constrain_sram_world_0_pms_0"]
    #[inline(always)]
    pub fn dma_apbperi_lc_pms_constrain_sram_world_0_pms_0(
        &self,
    ) -> DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0_R {
        DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - dma_apbperi_lc_pms_constrain_sram_world_0_pms_1"]
    #[inline(always)]
    pub fn dma_apbperi_lc_pms_constrain_sram_world_0_pms_1(
        &self,
    ) -> DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1_R {
        DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - dma_apbperi_lc_pms_constrain_sram_world_0_pms_2"]
    #[inline(always)]
    pub fn dma_apbperi_lc_pms_constrain_sram_world_0_pms_2(
        &self,
    ) -> DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2_R {
        DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - dma_apbperi_lc_pms_constrain_sram_world_0_pms_3"]
    #[inline(always)]
    pub fn dma_apbperi_lc_pms_constrain_sram_world_0_pms_3(
        &self,
    ) -> DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3_R {
        DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 12:13 - dma_apbperi_lc_pms_constrain_sram_world_1_pms_0"]
    #[inline(always)]
    pub fn dma_apbperi_lc_pms_constrain_sram_world_1_pms_0(
        &self,
    ) -> DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_0_R {
        DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_0_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - dma_apbperi_lc_pms_constrain_sram_world_1_pms_1"]
    #[inline(always)]
    pub fn dma_apbperi_lc_pms_constrain_sram_world_1_pms_1(
        &self,
    ) -> DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_1_R {
        DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_1_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - dma_apbperi_lc_pms_constrain_sram_world_1_pms_2"]
    #[inline(always)]
    pub fn dma_apbperi_lc_pms_constrain_sram_world_1_pms_2(
        &self,
    ) -> DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_2_R {
        DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_2_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - dma_apbperi_lc_pms_constrain_sram_world_1_pms_3"]
    #[inline(always)]
    pub fn dma_apbperi_lc_pms_constrain_sram_world_1_pms_3(
        &self,
    ) -> DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_3_R {
        DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_3_R::new(((self.bits >> 18) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - dma_apbperi_lc_pms_constrain_sram_world_0_pms_0"]
    #[inline(always)]
    pub fn dma_apbperi_lc_pms_constrain_sram_world_0_pms_0(
        &mut self,
    ) -> DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0_W {
        DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0_W { w: self }
    }
    #[doc = "Bits 2:3 - dma_apbperi_lc_pms_constrain_sram_world_0_pms_1"]
    #[inline(always)]
    pub fn dma_apbperi_lc_pms_constrain_sram_world_0_pms_1(
        &mut self,
    ) -> DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1_W {
        DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1_W { w: self }
    }
    #[doc = "Bits 4:5 - dma_apbperi_lc_pms_constrain_sram_world_0_pms_2"]
    #[inline(always)]
    pub fn dma_apbperi_lc_pms_constrain_sram_world_0_pms_2(
        &mut self,
    ) -> DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2_W {
        DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2_W { w: self }
    }
    #[doc = "Bits 6:7 - dma_apbperi_lc_pms_constrain_sram_world_0_pms_3"]
    #[inline(always)]
    pub fn dma_apbperi_lc_pms_constrain_sram_world_0_pms_3(
        &mut self,
    ) -> DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3_W {
        DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3_W { w: self }
    }
    #[doc = "Bits 12:13 - dma_apbperi_lc_pms_constrain_sram_world_1_pms_0"]
    #[inline(always)]
    pub fn dma_apbperi_lc_pms_constrain_sram_world_1_pms_0(
        &mut self,
    ) -> DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_0_W {
        DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_0_W { w: self }
    }
    #[doc = "Bits 14:15 - dma_apbperi_lc_pms_constrain_sram_world_1_pms_1"]
    #[inline(always)]
    pub fn dma_apbperi_lc_pms_constrain_sram_world_1_pms_1(
        &mut self,
    ) -> DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_1_W {
        DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_1_W { w: self }
    }
    #[doc = "Bits 16:17 - dma_apbperi_lc_pms_constrain_sram_world_1_pms_2"]
    #[inline(always)]
    pub fn dma_apbperi_lc_pms_constrain_sram_world_1_pms_2(
        &mut self,
    ) -> DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_2_W {
        DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_2_W { w: self }
    }
    #[doc = "Bits 18:19 - dma_apbperi_lc_pms_constrain_sram_world_1_pms_3"]
    #[inline(always)]
    pub fn dma_apbperi_lc_pms_constrain_sram_world_1_pms_3(
        &mut self,
    ) -> DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_3_W {
        DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SENSITIVE_DMA_APBPERI_LC_PMS_CONSTRAIN_1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_apbperi_lc_pms_constrain_1](index.html) module"]
pub struct DMA_APBPERI_LC_PMS_CONSTRAIN_1_SPEC;
impl crate::RegisterSpec for DMA_APBPERI_LC_PMS_CONSTRAIN_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_apbperi_lc_pms_constrain_1::R](R) reader structure"]
impl crate::Readable for DMA_APBPERI_LC_PMS_CONSTRAIN_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_apbperi_lc_pms_constrain_1::W](W) writer structure"]
impl crate::Writable for DMA_APBPERI_LC_PMS_CONSTRAIN_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_APBPERI_LC_PMS_CONSTRAIN_1 to value 0x000f_f0ff"]
impl crate::Resettable for DMA_APBPERI_LC_PMS_CONSTRAIN_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x000f_f0ff
    }
}
