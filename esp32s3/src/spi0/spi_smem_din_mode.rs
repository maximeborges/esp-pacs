#[doc = "Register `SPI_SMEM_DIN_MODE` reader"]
pub struct R(crate::R<SPI_SMEM_DIN_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_SMEM_DIN_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_SMEM_DIN_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_SMEM_DIN_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_SMEM_DIN_MODE` writer"]
pub struct W(crate::W<SPI_SMEM_DIN_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_SMEM_DIN_MODE_SPEC>;
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
impl From<crate::W<SPI_SMEM_DIN_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_SMEM_DIN_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_SMEM_DIN0_MODE` reader - SPI_D input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DIN0_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DIN0_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DIN0_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DIN0_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DIN0_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
pub struct SPI_SMEM_DIN0_MODE_R(crate::FieldReader<u8, u8>);
impl SPI_SMEM_DIN0_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SPI_SMEM_DIN0_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_SMEM_DIN0_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_SMEM_DIN0_MODE` writer - SPI_D input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DIN0_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DIN0_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DIN0_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DIN0_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DIN0_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
pub struct SPI_SMEM_DIN0_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SMEM_DIN0_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `SPI_SMEM_DIN1_MODE` reader - SPI_Q input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DIN3_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DIN3_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DIN3_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DIN3_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DIN3_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
pub struct SPI_SMEM_DIN1_MODE_R(crate::FieldReader<u8, u8>);
impl SPI_SMEM_DIN1_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SPI_SMEM_DIN1_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_SMEM_DIN1_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_SMEM_DIN1_MODE` writer - SPI_Q input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DIN3_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DIN3_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DIN3_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DIN3_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DIN3_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
pub struct SPI_SMEM_DIN1_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SMEM_DIN1_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | ((value as u32 & 0x07) << 3);
        self.w
    }
}
#[doc = "Field `SPI_SMEM_DIN2_MODE` reader - SPI_WP input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DIN6_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DIN6_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DIN6_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DIN6_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DIN6_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
pub struct SPI_SMEM_DIN2_MODE_R(crate::FieldReader<u8, u8>);
impl SPI_SMEM_DIN2_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SPI_SMEM_DIN2_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_SMEM_DIN2_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_SMEM_DIN2_MODE` writer - SPI_WP input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DIN6_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DIN6_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DIN6_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DIN6_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DIN6_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
pub struct SPI_SMEM_DIN2_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SMEM_DIN2_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | ((value as u32 & 0x07) << 6);
        self.w
    }
}
#[doc = "Field `SPI_SMEM_DIN3_MODE` reader - SPI_HD input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DIN9_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DIN9_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DIN9_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DIN9_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DIN9_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
pub struct SPI_SMEM_DIN3_MODE_R(crate::FieldReader<u8, u8>);
impl SPI_SMEM_DIN3_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SPI_SMEM_DIN3_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_SMEM_DIN3_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_SMEM_DIN3_MODE` writer - SPI_HD input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DIN9_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DIN9_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DIN9_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DIN9_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DIN9_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
pub struct SPI_SMEM_DIN3_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SMEM_DIN3_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | ((value as u32 & 0x07) << 9);
        self.w
    }
}
#[doc = "Field `SPI_SMEM_DIN4_MODE` reader - SPI_IO4 input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DIN12_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DIN12_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DIN12_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DIN12_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DIN12_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
pub struct SPI_SMEM_DIN4_MODE_R(crate::FieldReader<u8, u8>);
impl SPI_SMEM_DIN4_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SPI_SMEM_DIN4_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_SMEM_DIN4_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_SMEM_DIN4_MODE` writer - SPI_IO4 input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DIN12_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DIN12_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DIN12_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DIN12_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DIN12_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
pub struct SPI_SMEM_DIN4_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SMEM_DIN4_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "Field `SPI_SMEM_DIN5_MODE` reader - SPI_IO5 input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DIN15_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DIN15_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DIN15_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DIN15_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DIN15_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
pub struct SPI_SMEM_DIN5_MODE_R(crate::FieldReader<u8, u8>);
impl SPI_SMEM_DIN5_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SPI_SMEM_DIN5_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_SMEM_DIN5_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_SMEM_DIN5_MODE` writer - SPI_IO5 input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DIN15_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DIN15_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DIN15_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DIN15_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DIN15_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
pub struct SPI_SMEM_DIN5_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SMEM_DIN5_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 15)) | ((value as u32 & 0x07) << 15);
        self.w
    }
}
#[doc = "Field `SPI_SMEM_DIN6_MODE` reader - SPI_IO6 input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DIN18_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DIN18_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DIN18_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DIN18_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DIN18_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
pub struct SPI_SMEM_DIN6_MODE_R(crate::FieldReader<u8, u8>);
impl SPI_SMEM_DIN6_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SPI_SMEM_DIN6_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_SMEM_DIN6_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_SMEM_DIN6_MODE` writer - SPI_IO6 input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DIN18_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DIN18_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DIN18_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DIN18_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DIN18_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
pub struct SPI_SMEM_DIN6_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SMEM_DIN6_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | ((value as u32 & 0x07) << 18);
        self.w
    }
}
#[doc = "Field `SPI_SMEM_DIN7_MODE` reader - SPI_IO7 input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DIN21_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DIN21_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DIN21_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DIN21_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DIN21_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
pub struct SPI_SMEM_DIN7_MODE_R(crate::FieldReader<u8, u8>);
impl SPI_SMEM_DIN7_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SPI_SMEM_DIN7_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_SMEM_DIN7_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_SMEM_DIN7_MODE` writer - SPI_IO7 input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DIN21_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DIN21_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DIN21_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DIN21_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DIN21_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
pub struct SPI_SMEM_DIN7_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SMEM_DIN7_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 21)) | ((value as u32 & 0x07) << 21);
        self.w
    }
}
#[doc = "Field `SPI_SMEM_DINS_MODE` reader - SPI_DQS input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DINS_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DINS_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DINS_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DINS_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DINS_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
pub struct SPI_SMEM_DINS_MODE_R(crate::FieldReader<u8, u8>);
impl SPI_SMEM_DINS_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SPI_SMEM_DINS_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_SMEM_DINS_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_SMEM_DINS_MODE` writer - SPI_DQS input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DINS_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DINS_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DINS_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DINS_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DINS_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
pub struct SPI_SMEM_DINS_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SMEM_DINS_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | ((value as u32 & 0x07) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - SPI_D input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DIN0_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DIN0_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DIN0_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DIN0_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DIN0_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    pub fn spi_smem_din0_mode(&self) -> SPI_SMEM_DIN0_MODE_R {
        SPI_SMEM_DIN0_MODE_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:5 - SPI_Q input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DIN3_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DIN3_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DIN3_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DIN3_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DIN3_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    pub fn spi_smem_din1_mode(&self) -> SPI_SMEM_DIN1_MODE_R {
        SPI_SMEM_DIN1_MODE_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 6:8 - SPI_WP input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DIN6_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DIN6_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DIN6_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DIN6_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DIN6_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    pub fn spi_smem_din2_mode(&self) -> SPI_SMEM_DIN2_MODE_R {
        SPI_SMEM_DIN2_MODE_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bits 9:11 - SPI_HD input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DIN9_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DIN9_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DIN9_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DIN9_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DIN9_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    pub fn spi_smem_din3_mode(&self) -> SPI_SMEM_DIN3_MODE_R {
        SPI_SMEM_DIN3_MODE_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - SPI_IO4 input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DIN12_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DIN12_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DIN12_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DIN12_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DIN12_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    pub fn spi_smem_din4_mode(&self) -> SPI_SMEM_DIN4_MODE_R {
        SPI_SMEM_DIN4_MODE_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 15:17 - SPI_IO5 input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DIN15_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DIN15_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DIN15_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DIN15_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DIN15_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    pub fn spi_smem_din5_mode(&self) -> SPI_SMEM_DIN5_MODE_R {
        SPI_SMEM_DIN5_MODE_R::new(((self.bits >> 15) & 0x07) as u8)
    }
    #[doc = "Bits 18:20 - SPI_IO6 input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DIN18_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DIN18_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DIN18_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DIN18_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DIN18_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    pub fn spi_smem_din6_mode(&self) -> SPI_SMEM_DIN6_MODE_R {
        SPI_SMEM_DIN6_MODE_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bits 21:23 - SPI_IO7 input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DIN21_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DIN21_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DIN21_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DIN21_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DIN21_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    pub fn spi_smem_din7_mode(&self) -> SPI_SMEM_DIN7_MODE_R {
        SPI_SMEM_DIN7_MODE_R::new(((self.bits >> 21) & 0x07) as u8)
    }
    #[doc = "Bits 24:26 - SPI_DQS input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DINS_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DINS_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DINS_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DINS_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DINS_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    pub fn spi_smem_dins_mode(&self) -> SPI_SMEM_DINS_MODE_R {
        SPI_SMEM_DINS_MODE_R::new(((self.bits >> 24) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SPI_D input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DIN0_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DIN0_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DIN0_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DIN0_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DIN0_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    pub fn spi_smem_din0_mode(&mut self) -> SPI_SMEM_DIN0_MODE_W {
        SPI_SMEM_DIN0_MODE_W { w: self }
    }
    #[doc = "Bits 3:5 - SPI_Q input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DIN3_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DIN3_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DIN3_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DIN3_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DIN3_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    pub fn spi_smem_din1_mode(&mut self) -> SPI_SMEM_DIN1_MODE_W {
        SPI_SMEM_DIN1_MODE_W { w: self }
    }
    #[doc = "Bits 6:8 - SPI_WP input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DIN6_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DIN6_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DIN6_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DIN6_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DIN6_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    pub fn spi_smem_din2_mode(&mut self) -> SPI_SMEM_DIN2_MODE_W {
        SPI_SMEM_DIN2_MODE_W { w: self }
    }
    #[doc = "Bits 9:11 - SPI_HD input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DIN9_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DIN9_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DIN9_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DIN9_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DIN9_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    pub fn spi_smem_din3_mode(&mut self) -> SPI_SMEM_DIN3_MODE_W {
        SPI_SMEM_DIN3_MODE_W { w: self }
    }
    #[doc = "Bits 12:14 - SPI_IO4 input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DIN12_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DIN12_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DIN12_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DIN12_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DIN12_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    pub fn spi_smem_din4_mode(&mut self) -> SPI_SMEM_DIN4_MODE_W {
        SPI_SMEM_DIN4_MODE_W { w: self }
    }
    #[doc = "Bits 15:17 - SPI_IO5 input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DIN15_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DIN15_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DIN15_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DIN15_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DIN15_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    pub fn spi_smem_din5_mode(&mut self) -> SPI_SMEM_DIN5_MODE_W {
        SPI_SMEM_DIN5_MODE_W { w: self }
    }
    #[doc = "Bits 18:20 - SPI_IO6 input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DIN18_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DIN18_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DIN18_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DIN18_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DIN18_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    pub fn spi_smem_din6_mode(&mut self) -> SPI_SMEM_DIN6_MODE_W {
        SPI_SMEM_DIN6_MODE_W { w: self }
    }
    #[doc = "Bits 21:23 - SPI_IO7 input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DIN21_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DIN21_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DIN21_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DIN21_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DIN21_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    pub fn spi_smem_din7_mode(&mut self) -> SPI_SMEM_DIN7_MODE_W {
        SPI_SMEM_DIN7_MODE_W { w: self }
    }
    #[doc = "Bits 24:26 - SPI_DQS input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DINS_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DINS_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DINS_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DINS_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DINS_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    pub fn spi_smem_dins_mode(&mut self) -> SPI_SMEM_DINS_MODE_W {
        SPI_SMEM_DINS_MODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MSPI input timing delay mode control register when accesses to Ext_RAM.\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_smem_din_mode]
(index.html) module"]
pub struct SPI_SMEM_DIN_MODE_SPEC;
impl crate::RegisterSpec for SPI_SMEM_DIN_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_smem_din_mode::R]
(R) reader structure"]
impl crate::Readable for SPI_SMEM_DIN_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_smem_din_mode::W]
(W) writer structure"]
impl crate::Writable for SPI_SMEM_DIN_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_SMEM_DIN_MODE to value 0"]
impl crate::Resettable for SPI_SMEM_DIN_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
