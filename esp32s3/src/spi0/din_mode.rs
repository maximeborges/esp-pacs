#[doc = "Register `DIN_MODE` reader"]
pub struct R(crate::R<DIN_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIN_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIN_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIN_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIN_MODE` writer"]
pub struct W(crate::W<DIN_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIN_MODE_SPEC>;
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
impl From<crate::W<DIN_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIN_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIN0_MODE` reader - SPI_D input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DIN0_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DIN0_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DIN0_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DIN0_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DIN0_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
pub struct DIN0_MODE_R(crate::FieldReader<u8, u8>);
impl DIN0_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIN0_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIN0_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIN0_MODE` writer - SPI_D input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DIN0_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DIN0_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DIN0_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DIN0_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DIN0_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
pub struct DIN0_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DIN0_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !7) | (value as u32 & 7);
        self.w
    }
}
#[doc = "Field `DIN1_MODE` reader - SPI_Q input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DIN3_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DIN3_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DIN3_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DIN3_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DIN3_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
pub struct DIN1_MODE_R(crate::FieldReader<u8, u8>);
impl DIN1_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIN1_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIN1_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIN1_MODE` writer - SPI_Q input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DIN3_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DIN3_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DIN3_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DIN3_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DIN3_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
pub struct DIN1_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DIN1_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 3)) | ((value as u32 & 7) << 3);
        self.w
    }
}
#[doc = "Field `DIN2_MODE` reader - SPI_WP input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DIN6_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DIN6_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DIN6_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DIN6_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DIN6_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
pub struct DIN2_MODE_R(crate::FieldReader<u8, u8>);
impl DIN2_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIN2_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIN2_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIN2_MODE` writer - SPI_WP input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DIN6_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DIN6_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DIN6_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DIN6_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DIN6_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
pub struct DIN2_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DIN2_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 6)) | ((value as u32 & 7) << 6);
        self.w
    }
}
#[doc = "Field `DIN3_MODE` reader - SPI_HD input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DIN9_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DIN9_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DIN9_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DIN9_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DIN9_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
pub struct DIN3_MODE_R(crate::FieldReader<u8, u8>);
impl DIN3_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIN3_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIN3_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIN3_MODE` writer - SPI_HD input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DIN9_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DIN9_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DIN9_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DIN9_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DIN9_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
pub struct DIN3_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DIN3_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 9)) | ((value as u32 & 7) << 9);
        self.w
    }
}
#[doc = "Field `DIN4_MODE` reader - SPI_IO4 input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DIN12_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DIN12_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DIN12_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DIN12_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DIN12_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
pub struct DIN4_MODE_R(crate::FieldReader<u8, u8>);
impl DIN4_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIN4_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIN4_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIN4_MODE` writer - SPI_IO4 input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DIN12_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DIN12_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DIN12_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DIN12_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DIN12_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
pub struct DIN4_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DIN4_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 12)) | ((value as u32 & 7) << 12);
        self.w
    }
}
#[doc = "Field `DIN5_MODE` reader - SPI_IO5 input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DIN15_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DIN15_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DIN15_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DIN15_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DIN15_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
pub struct DIN5_MODE_R(crate::FieldReader<u8, u8>);
impl DIN5_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIN5_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIN5_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIN5_MODE` writer - SPI_IO5 input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DIN15_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DIN15_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DIN15_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DIN15_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DIN15_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
pub struct DIN5_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DIN5_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 15)) | ((value as u32 & 7) << 15);
        self.w
    }
}
#[doc = "Field `DIN6_MODE` reader - SPI_IO6 input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DIN18_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DIN18_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DIN18_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DIN18_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DIN18_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
pub struct DIN6_MODE_R(crate::FieldReader<u8, u8>);
impl DIN6_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIN6_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIN6_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIN6_MODE` writer - SPI_IO6 input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DIN18_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DIN18_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DIN18_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DIN18_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DIN18_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
pub struct DIN6_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DIN6_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 18)) | ((value as u32 & 7) << 18);
        self.w
    }
}
#[doc = "Field `DIN7_MODE` reader - SPI_IO7 input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DIN21_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DIN21_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DIN21_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DIN21_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DIN21_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
pub struct DIN7_MODE_R(crate::FieldReader<u8, u8>);
impl DIN7_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIN7_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIN7_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIN7_MODE` writer - SPI_IO7 input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DIN21_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DIN21_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DIN21_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DIN21_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DIN21_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
pub struct DIN7_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DIN7_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 21)) | ((value as u32 & 7) << 21);
        self.w
    }
}
#[doc = "Field `DINS_MODE` reader - SPI_DQS input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DINS_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DINS_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DINS_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DINS_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DINS_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
pub struct DINS_MODE_R(crate::FieldReader<u8, u8>);
impl DINS_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DINS_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DINS_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DINS_MODE` writer - SPI_DQS input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DINS_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DINS_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DINS_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DINS_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DINS_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
pub struct DINS_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DINS_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 24)) | ((value as u32 & 7) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - SPI_D input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DIN0_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DIN0_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DIN0_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DIN0_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DIN0_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    pub fn din0_mode(&self) -> DIN0_MODE_R {
        DIN0_MODE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - SPI_Q input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DIN3_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DIN3_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DIN3_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DIN3_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DIN3_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    pub fn din1_mode(&self) -> DIN1_MODE_R {
        DIN1_MODE_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - SPI_WP input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DIN6_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DIN6_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DIN6_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DIN6_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DIN6_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    pub fn din2_mode(&self) -> DIN2_MODE_R {
        DIN2_MODE_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - SPI_HD input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DIN9_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DIN9_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DIN9_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DIN9_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DIN9_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    pub fn din3_mode(&self) -> DIN3_MODE_R {
        DIN3_MODE_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - SPI_IO4 input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DIN12_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DIN12_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DIN12_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DIN12_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DIN12_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    pub fn din4_mode(&self) -> DIN4_MODE_R {
        DIN4_MODE_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - SPI_IO5 input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DIN15_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DIN15_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DIN15_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DIN15_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DIN15_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    pub fn din5_mode(&self) -> DIN5_MODE_R {
        DIN5_MODE_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - SPI_IO6 input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DIN18_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DIN18_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DIN18_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DIN18_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DIN18_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    pub fn din6_mode(&self) -> DIN6_MODE_R {
        DIN6_MODE_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - SPI_IO7 input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DIN21_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DIN21_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DIN21_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DIN21_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DIN21_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    pub fn din7_mode(&self) -> DIN7_MODE_R {
        DIN7_MODE_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26 - SPI_DQS input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DINS_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DINS_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DINS_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DINS_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DINS_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    pub fn dins_mode(&self) -> DINS_MODE_R {
        DINS_MODE_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SPI_D input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DIN0_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DIN0_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DIN0_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DIN0_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DIN0_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    pub fn din0_mode(&mut self) -> DIN0_MODE_W {
        DIN0_MODE_W { w: self }
    }
    #[doc = "Bits 3:5 - SPI_Q input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DIN3_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DIN3_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DIN3_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DIN3_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DIN3_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    pub fn din1_mode(&mut self) -> DIN1_MODE_W {
        DIN1_MODE_W { w: self }
    }
    #[doc = "Bits 6:8 - SPI_WP input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DIN6_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DIN6_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DIN6_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DIN6_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DIN6_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    pub fn din2_mode(&mut self) -> DIN2_MODE_W {
        DIN2_MODE_W { w: self }
    }
    #[doc = "Bits 9:11 - SPI_HD input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DIN9_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DIN9_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DIN9_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DIN9_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DIN9_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    pub fn din3_mode(&mut self) -> DIN3_MODE_W {
        DIN3_MODE_W { w: self }
    }
    #[doc = "Bits 12:14 - SPI_IO4 input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DIN12_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DIN12_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DIN12_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DIN12_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DIN12_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    pub fn din4_mode(&mut self) -> DIN4_MODE_W {
        DIN4_MODE_W { w: self }
    }
    #[doc = "Bits 15:17 - SPI_IO5 input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DIN15_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DIN15_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DIN15_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DIN15_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DIN15_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    pub fn din5_mode(&mut self) -> DIN5_MODE_W {
        DIN5_MODE_W { w: self }
    }
    #[doc = "Bits 18:20 - SPI_IO6 input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DIN18_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DIN18_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DIN18_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DIN18_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DIN18_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    pub fn din6_mode(&mut self) -> DIN6_MODE_W {
        DIN6_MODE_W { w: self }
    }
    #[doc = "Bits 21:23 - SPI_IO7 input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DIN21_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DIN21_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DIN21_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DIN21_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DIN21_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    pub fn din7_mode(&mut self) -> DIN7_MODE_W {
        DIN7_MODE_W { w: self }
    }
    #[doc = "Bits 24:26 - SPI_DQS input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DINS_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DINS_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DINS_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DINS_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DINS_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    pub fn dins_mode(&mut self) -> DINS_MODE_W {
        DINS_MODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MSPI input timing delay mode control register when accesses to flash.\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [din_mode]
(index.html) module"]
pub struct DIN_MODE_SPEC;
impl crate::RegisterSpec for DIN_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [din_mode::R]
(R) reader structure"]
impl crate::Readable for DIN_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [din_mode::W]
(W) writer structure"]
impl crate::Writable for DIN_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIN_MODE to value 0"]
impl crate::Resettable for DIN_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
