#[doc = "Register `SPI_SMEM_DIN_NUM` reader"]
pub struct R(crate::R<SPI_SMEM_DIN_NUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_SMEM_DIN_NUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_SMEM_DIN_NUM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_SMEM_DIN_NUM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_SMEM_DIN_NUM` writer"]
pub struct W(crate::W<SPI_SMEM_DIN_NUM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_SMEM_DIN_NUM_SPEC>;
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
impl From<crate::W<SPI_SMEM_DIN_NUM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_SMEM_DIN_NUM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_SMEM_DIN0_NUM` reader - SPI_D input delay number."]
pub struct SPI_SMEM_DIN0_NUM_R(crate::FieldReader<u8>);
impl SPI_SMEM_DIN0_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SPI_SMEM_DIN0_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_SMEM_DIN0_NUM_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_SMEM_DIN0_NUM` writer - SPI_D input delay number."]
pub struct SPI_SMEM_DIN0_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SMEM_DIN0_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u32 & 3);
        self.w
    }
}
#[doc = "Field `SPI_SMEM_DIN1_NUM` reader - SPI_Q input delay number."]
pub struct SPI_SMEM_DIN1_NUM_R(crate::FieldReader<u8>);
impl SPI_SMEM_DIN1_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SPI_SMEM_DIN1_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_SMEM_DIN1_NUM_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_SMEM_DIN1_NUM` writer - SPI_Q input delay number."]
pub struct SPI_SMEM_DIN1_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SMEM_DIN1_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 2)) | ((value as u32 & 3) << 2);
        self.w
    }
}
#[doc = "Field `SPI_SMEM_DIN2_NUM` reader - SPI_WP input delay number."]
pub struct SPI_SMEM_DIN2_NUM_R(crate::FieldReader<u8>);
impl SPI_SMEM_DIN2_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SPI_SMEM_DIN2_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_SMEM_DIN2_NUM_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_SMEM_DIN2_NUM` writer - SPI_WP input delay number."]
pub struct SPI_SMEM_DIN2_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SMEM_DIN2_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 4)) | ((value as u32 & 3) << 4);
        self.w
    }
}
#[doc = "Field `SPI_SMEM_DIN3_NUM` reader - SPI_HD input delay number."]
pub struct SPI_SMEM_DIN3_NUM_R(crate::FieldReader<u8>);
impl SPI_SMEM_DIN3_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SPI_SMEM_DIN3_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_SMEM_DIN3_NUM_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_SMEM_DIN3_NUM` writer - SPI_HD input delay number."]
pub struct SPI_SMEM_DIN3_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SMEM_DIN3_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 6)) | ((value as u32 & 3) << 6);
        self.w
    }
}
#[doc = "Field `SPI_SMEM_DIN4_NUM` reader - SPI_IO4 input delay number."]
pub struct SPI_SMEM_DIN4_NUM_R(crate::FieldReader<u8>);
impl SPI_SMEM_DIN4_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SPI_SMEM_DIN4_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_SMEM_DIN4_NUM_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_SMEM_DIN4_NUM` writer - SPI_IO4 input delay number."]
pub struct SPI_SMEM_DIN4_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SMEM_DIN4_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 8)) | ((value as u32 & 3) << 8);
        self.w
    }
}
#[doc = "Field `SPI_SMEM_DIN5_NUM` reader - SPI_IO5 input delay number."]
pub struct SPI_SMEM_DIN5_NUM_R(crate::FieldReader<u8>);
impl SPI_SMEM_DIN5_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SPI_SMEM_DIN5_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_SMEM_DIN5_NUM_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_SMEM_DIN5_NUM` writer - SPI_IO5 input delay number."]
pub struct SPI_SMEM_DIN5_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SMEM_DIN5_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 10)) | ((value as u32 & 3) << 10);
        self.w
    }
}
#[doc = "Field `SPI_SMEM_DIN6_NUM` reader - SPI_IO6 input delay number."]
pub struct SPI_SMEM_DIN6_NUM_R(crate::FieldReader<u8>);
impl SPI_SMEM_DIN6_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SPI_SMEM_DIN6_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_SMEM_DIN6_NUM_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_SMEM_DIN6_NUM` writer - SPI_IO6 input delay number."]
pub struct SPI_SMEM_DIN6_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SMEM_DIN6_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 12)) | ((value as u32 & 3) << 12);
        self.w
    }
}
#[doc = "Field `SPI_SMEM_DIN7_NUM` reader - SPI_IO7 input delay number."]
pub struct SPI_SMEM_DIN7_NUM_R(crate::FieldReader<u8>);
impl SPI_SMEM_DIN7_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SPI_SMEM_DIN7_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_SMEM_DIN7_NUM_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_SMEM_DIN7_NUM` writer - SPI_IO7 input delay number."]
pub struct SPI_SMEM_DIN7_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SMEM_DIN7_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 14)) | ((value as u32 & 3) << 14);
        self.w
    }
}
#[doc = "Field `SPI_SMEM_DINS_NUM` reader - SPI_DQS input delay number."]
pub struct SPI_SMEM_DINS_NUM_R(crate::FieldReader<u8>);
impl SPI_SMEM_DINS_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SPI_SMEM_DINS_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_SMEM_DINS_NUM_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_SMEM_DINS_NUM` writer - SPI_DQS input delay number."]
pub struct SPI_SMEM_DINS_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SMEM_DINS_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 16)) | ((value as u32 & 3) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - SPI_D input delay number."]
    #[inline(always)]
    pub fn spi_smem_din0_num(&self) -> SPI_SMEM_DIN0_NUM_R {
        SPI_SMEM_DIN0_NUM_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - SPI_Q input delay number."]
    #[inline(always)]
    pub fn spi_smem_din1_num(&self) -> SPI_SMEM_DIN1_NUM_R {
        SPI_SMEM_DIN1_NUM_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - SPI_WP input delay number."]
    #[inline(always)]
    pub fn spi_smem_din2_num(&self) -> SPI_SMEM_DIN2_NUM_R {
        SPI_SMEM_DIN2_NUM_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - SPI_HD input delay number."]
    #[inline(always)]
    pub fn spi_smem_din3_num(&self) -> SPI_SMEM_DIN3_NUM_R {
        SPI_SMEM_DIN3_NUM_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - SPI_IO4 input delay number."]
    #[inline(always)]
    pub fn spi_smem_din4_num(&self) -> SPI_SMEM_DIN4_NUM_R {
        SPI_SMEM_DIN4_NUM_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - SPI_IO5 input delay number."]
    #[inline(always)]
    pub fn spi_smem_din5_num(&self) -> SPI_SMEM_DIN5_NUM_R {
        SPI_SMEM_DIN5_NUM_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - SPI_IO6 input delay number."]
    #[inline(always)]
    pub fn spi_smem_din6_num(&self) -> SPI_SMEM_DIN6_NUM_R {
        SPI_SMEM_DIN6_NUM_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - SPI_IO7 input delay number."]
    #[inline(always)]
    pub fn spi_smem_din7_num(&self) -> SPI_SMEM_DIN7_NUM_R {
        SPI_SMEM_DIN7_NUM_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - SPI_DQS input delay number."]
    #[inline(always)]
    pub fn spi_smem_dins_num(&self) -> SPI_SMEM_DINS_NUM_R {
        SPI_SMEM_DINS_NUM_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - SPI_D input delay number."]
    #[inline(always)]
    pub fn spi_smem_din0_num(&mut self) -> SPI_SMEM_DIN0_NUM_W {
        SPI_SMEM_DIN0_NUM_W { w: self }
    }
    #[doc = "Bits 2:3 - SPI_Q input delay number."]
    #[inline(always)]
    pub fn spi_smem_din1_num(&mut self) -> SPI_SMEM_DIN1_NUM_W {
        SPI_SMEM_DIN1_NUM_W { w: self }
    }
    #[doc = "Bits 4:5 - SPI_WP input delay number."]
    #[inline(always)]
    pub fn spi_smem_din2_num(&mut self) -> SPI_SMEM_DIN2_NUM_W {
        SPI_SMEM_DIN2_NUM_W { w: self }
    }
    #[doc = "Bits 6:7 - SPI_HD input delay number."]
    #[inline(always)]
    pub fn spi_smem_din3_num(&mut self) -> SPI_SMEM_DIN3_NUM_W {
        SPI_SMEM_DIN3_NUM_W { w: self }
    }
    #[doc = "Bits 8:9 - SPI_IO4 input delay number."]
    #[inline(always)]
    pub fn spi_smem_din4_num(&mut self) -> SPI_SMEM_DIN4_NUM_W {
        SPI_SMEM_DIN4_NUM_W { w: self }
    }
    #[doc = "Bits 10:11 - SPI_IO5 input delay number."]
    #[inline(always)]
    pub fn spi_smem_din5_num(&mut self) -> SPI_SMEM_DIN5_NUM_W {
        SPI_SMEM_DIN5_NUM_W { w: self }
    }
    #[doc = "Bits 12:13 - SPI_IO6 input delay number."]
    #[inline(always)]
    pub fn spi_smem_din6_num(&mut self) -> SPI_SMEM_DIN6_NUM_W {
        SPI_SMEM_DIN6_NUM_W { w: self }
    }
    #[doc = "Bits 14:15 - SPI_IO7 input delay number."]
    #[inline(always)]
    pub fn spi_smem_din7_num(&mut self) -> SPI_SMEM_DIN7_NUM_W {
        SPI_SMEM_DIN7_NUM_W { w: self }
    }
    #[doc = "Bits 16:17 - SPI_DQS input delay number."]
    #[inline(always)]
    pub fn spi_smem_dins_num(&mut self) -> SPI_SMEM_DINS_NUM_W {
        SPI_SMEM_DINS_NUM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MSPI input timing delay number control register when accesses to Ext_RAM.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_smem_din_num](index.html) module"]
pub struct SPI_SMEM_DIN_NUM_SPEC;
impl crate::RegisterSpec for SPI_SMEM_DIN_NUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_smem_din_num::R](R) reader structure"]
impl crate::Readable for SPI_SMEM_DIN_NUM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_smem_din_num::W](W) writer structure"]
impl crate::Writable for SPI_SMEM_DIN_NUM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_SMEM_DIN_NUM to value 0"]
impl crate::Resettable for SPI_SMEM_DIN_NUM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
