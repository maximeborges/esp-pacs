#[doc = "Register `DATE` reader"]
pub struct R(crate::R<DATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATE` writer"]
pub struct W(crate::W<DATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATE_SPEC>;
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
impl From<crate::W<DATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_SMEM_SPICLK_FUN_DRV` reader - The driver of SPI_CLK PAD is controlled by the bits SPI_SMEM_SPICLK_FUN_DRV\\[1:0\\]
 when the bit SPI_SPICLK_PAD_DRV_CTL_EN is set and MSPI accesses to external RAM."]
pub struct SPI_SMEM_SPICLK_FUN_DRV_R(crate::FieldReader<u8, u8>);
impl SPI_SMEM_SPICLK_FUN_DRV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SPI_SMEM_SPICLK_FUN_DRV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_SMEM_SPICLK_FUN_DRV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_SMEM_SPICLK_FUN_DRV` writer - The driver of SPI_CLK PAD is controlled by the bits SPI_SMEM_SPICLK_FUN_DRV\\[1:0\\]
 when the bit SPI_SPICLK_PAD_DRV_CTL_EN is set and MSPI accesses to external RAM."]
pub struct SPI_SMEM_SPICLK_FUN_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SMEM_SPICLK_FUN_DRV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `SPI_FMEM_SPICLK_FUN_DRV` reader - The driver of SPI_CLK PAD is controlled by the bits SPI_FMEM_SPICLK_FUN_DRV\\[1:0\\]
 when the bit SPI_SPICLK_PAD_DRV_CTL_EN is set and MSPI accesses to flash."]
pub struct SPI_FMEM_SPICLK_FUN_DRV_R(crate::FieldReader<u8, u8>);
impl SPI_FMEM_SPICLK_FUN_DRV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SPI_FMEM_SPICLK_FUN_DRV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_FMEM_SPICLK_FUN_DRV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_FMEM_SPICLK_FUN_DRV` writer - The driver of SPI_CLK PAD is controlled by the bits SPI_FMEM_SPICLK_FUN_DRV\\[1:0\\]
 when the bit SPI_SPICLK_PAD_DRV_CTL_EN is set and MSPI accesses to flash."]
pub struct SPI_FMEM_SPICLK_FUN_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_FMEM_SPICLK_FUN_DRV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `SPI_SPICLK_PAD_DRV_CTL_EN` reader - SPI_CLK PAD driver control signal. 1: The driver of SPI_CLK PAD is controlled by the bits SPI_FMEM_SPICLK_FUN_DRV\\[1:0\\]
 and SPI_SMEM_SPICLK_FUN_DRV\\[1:0\\]
. 0: The driver of SPI_CLK PAD is controlled by the bits IO_MUX_FUNC_DRV\\[1:0\\]
 of SPICLK PAD."]
pub struct SPI_SPICLK_PAD_DRV_CTL_EN_R(crate::FieldReader<bool, bool>);
impl SPI_SPICLK_PAD_DRV_CTL_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPI_SPICLK_PAD_DRV_CTL_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_SPICLK_PAD_DRV_CTL_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_SPICLK_PAD_DRV_CTL_EN` writer - SPI_CLK PAD driver control signal. 1: The driver of SPI_CLK PAD is controlled by the bits SPI_FMEM_SPICLK_FUN_DRV\\[1:0\\]
 and SPI_SMEM_SPICLK_FUN_DRV\\[1:0\\]
. 0: The driver of SPI_CLK PAD is controlled by the bits IO_MUX_FUNC_DRV\\[1:0\\]
 of SPICLK PAD."]
pub struct SPI_SPICLK_PAD_DRV_CTL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SPICLK_PAD_DRV_CTL_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `DATE` reader - SPI register version."]
pub struct DATE_R(crate::FieldReader<u32, u32>);
impl DATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        DATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATE` writer - SPI register version."]
pub struct DATE_W<'a> {
    w: &'a mut W,
}
impl<'a> DATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x007f_ffff << 5)) | ((value as u32 & 0x007f_ffff) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - The driver of SPI_CLK PAD is controlled by the bits SPI_SMEM_SPICLK_FUN_DRV\\[1:0\\]
 when the bit SPI_SPICLK_PAD_DRV_CTL_EN is set and MSPI accesses to external RAM."]
    #[inline(always)]
    pub fn spi_smem_spiclk_fun_drv(&self) -> SPI_SMEM_SPICLK_FUN_DRV_R {
        SPI_SMEM_SPICLK_FUN_DRV_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - The driver of SPI_CLK PAD is controlled by the bits SPI_FMEM_SPICLK_FUN_DRV\\[1:0\\]
 when the bit SPI_SPICLK_PAD_DRV_CTL_EN is set and MSPI accesses to flash."]
    #[inline(always)]
    pub fn spi_fmem_spiclk_fun_drv(&self) -> SPI_FMEM_SPICLK_FUN_DRV_R {
        SPI_FMEM_SPICLK_FUN_DRV_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - SPI_CLK PAD driver control signal. 1: The driver of SPI_CLK PAD is controlled by the bits SPI_FMEM_SPICLK_FUN_DRV\\[1:0\\]
 and SPI_SMEM_SPICLK_FUN_DRV\\[1:0\\]
. 0: The driver of SPI_CLK PAD is controlled by the bits IO_MUX_FUNC_DRV\\[1:0\\]
 of SPICLK PAD."]
    #[inline(always)]
    pub fn spi_spiclk_pad_drv_ctl_en(&self) -> SPI_SPICLK_PAD_DRV_CTL_EN_R {
        SPI_SPICLK_PAD_DRV_CTL_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:27 - SPI register version."]
    #[inline(always)]
    pub fn date(&self) -> DATE_R {
        DATE_R::new(((self.bits >> 5) & 0x007f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:1 - The driver of SPI_CLK PAD is controlled by the bits SPI_SMEM_SPICLK_FUN_DRV\\[1:0\\]
 when the bit SPI_SPICLK_PAD_DRV_CTL_EN is set and MSPI accesses to external RAM."]
    #[inline(always)]
    pub fn spi_smem_spiclk_fun_drv(&mut self) -> SPI_SMEM_SPICLK_FUN_DRV_W {
        SPI_SMEM_SPICLK_FUN_DRV_W { w: self }
    }
    #[doc = "Bits 2:3 - The driver of SPI_CLK PAD is controlled by the bits SPI_FMEM_SPICLK_FUN_DRV\\[1:0\\]
 when the bit SPI_SPICLK_PAD_DRV_CTL_EN is set and MSPI accesses to flash."]
    #[inline(always)]
    pub fn spi_fmem_spiclk_fun_drv(&mut self) -> SPI_FMEM_SPICLK_FUN_DRV_W {
        SPI_FMEM_SPICLK_FUN_DRV_W { w: self }
    }
    #[doc = "Bit 4 - SPI_CLK PAD driver control signal. 1: The driver of SPI_CLK PAD is controlled by the bits SPI_FMEM_SPICLK_FUN_DRV\\[1:0\\]
 and SPI_SMEM_SPICLK_FUN_DRV\\[1:0\\]
. 0: The driver of SPI_CLK PAD is controlled by the bits IO_MUX_FUNC_DRV\\[1:0\\]
 of SPICLK PAD."]
    #[inline(always)]
    pub fn spi_spiclk_pad_drv_ctl_en(&mut self) -> SPI_SPICLK_PAD_DRV_CTL_EN_W {
        SPI_SPICLK_PAD_DRV_CTL_EN_W { w: self }
    }
    #[doc = "Bits 5:27 - SPI register version."]
    #[inline(always)]
    pub fn date(&mut self) -> DATE_W {
        DATE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI0 version control register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [date]
(index.html) module"]
pub struct DATE_SPEC;
impl crate::RegisterSpec for DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [date::R]
(R) reader structure"]
impl crate::Readable for DATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [date::W]
(W) writer structure"]
impl crate::Writable for DATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATE to value 0x0210_1040"]
impl crate::Resettable for DATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0210_1040
    }
}
