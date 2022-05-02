#[doc = "Register `SPI_SMEM_TIMING_CALI` reader"]
pub struct R(crate::R<SPI_SMEM_TIMING_CALI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_SMEM_TIMING_CALI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_SMEM_TIMING_CALI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_SMEM_TIMING_CALI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_SMEM_TIMING_CALI` writer"]
pub struct W(crate::W<SPI_SMEM_TIMING_CALI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_SMEM_TIMING_CALI_SPEC>;
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
impl From<crate::W<SPI_SMEM_TIMING_CALI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_SMEM_TIMING_CALI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_SMEM_TIMING_CLK_ENA` reader - Set this bit to power on HCLK. When PLL is powered on, the frequency of HCLK equals to that of PLL. Otherwise, the frequency equals to that of XTAL."]
pub struct SPI_SMEM_TIMING_CLK_ENA_R(crate::FieldReader<bool>);
impl SPI_SMEM_TIMING_CLK_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPI_SMEM_TIMING_CLK_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_SMEM_TIMING_CLK_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_SMEM_TIMING_CLK_ENA` writer - Set this bit to power on HCLK. When PLL is powered on, the frequency of HCLK equals to that of PLL. Otherwise, the frequency equals to that of XTAL."]
pub struct SPI_SMEM_TIMING_CLK_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SMEM_TIMING_CLK_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
#[doc = "Field `SPI_SMEM_TIMING_CALI` reader - Set this bit to add extra SPI_CLK cycles in DUMMY phase for all reading operations."]
pub struct SPI_SMEM_TIMING_CALI_R(crate::FieldReader<bool>);
impl SPI_SMEM_TIMING_CALI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPI_SMEM_TIMING_CALI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_SMEM_TIMING_CALI_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_SMEM_TIMING_CALI` writer - Set this bit to add extra SPI_CLK cycles in DUMMY phase for all reading operations."]
pub struct SPI_SMEM_TIMING_CALI_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SMEM_TIMING_CALI_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u32 & 1) << 1);
        self.w
    }
}
#[doc = "Field `SPI_SMEM_EXTRA_DUMMY_CYCLELEN` reader - Extra SPI_CLK cycles added in DUMMY phase for timing compensation, when SPI0 accesses to Ext_RAM. Active when SPI_SMEM_TIMING_CALI bit is set."]
pub struct SPI_SMEM_EXTRA_DUMMY_CYCLELEN_R(crate::FieldReader<u8>);
impl SPI_SMEM_EXTRA_DUMMY_CYCLELEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SPI_SMEM_EXTRA_DUMMY_CYCLELEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_SMEM_EXTRA_DUMMY_CYCLELEN_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_SMEM_EXTRA_DUMMY_CYCLELEN` writer - Extra SPI_CLK cycles added in DUMMY phase for timing compensation, when SPI0 accesses to Ext_RAM. Active when SPI_SMEM_TIMING_CALI bit is set."]
pub struct SPI_SMEM_EXTRA_DUMMY_CYCLELEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SMEM_EXTRA_DUMMY_CYCLELEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 2)) | ((value as u32 & 7) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Set this bit to power on HCLK. When PLL is powered on, the frequency of HCLK equals to that of PLL. Otherwise, the frequency equals to that of XTAL."]
    #[inline(always)]
    pub fn spi_smem_timing_clk_ena(&self) -> SPI_SMEM_TIMING_CLK_ENA_R {
        SPI_SMEM_TIMING_CLK_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to add extra SPI_CLK cycles in DUMMY phase for all reading operations."]
    #[inline(always)]
    pub fn spi_smem_timing_cali(&self) -> SPI_SMEM_TIMING_CALI_R {
        SPI_SMEM_TIMING_CALI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - Extra SPI_CLK cycles added in DUMMY phase for timing compensation, when SPI0 accesses to Ext_RAM. Active when SPI_SMEM_TIMING_CALI bit is set."]
    #[inline(always)]
    pub fn spi_smem_extra_dummy_cyclelen(&self) -> SPI_SMEM_EXTRA_DUMMY_CYCLELEN_R {
        SPI_SMEM_EXTRA_DUMMY_CYCLELEN_R::new(((self.bits >> 2) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to power on HCLK. When PLL is powered on, the frequency of HCLK equals to that of PLL. Otherwise, the frequency equals to that of XTAL."]
    #[inline(always)]
    pub fn spi_smem_timing_clk_ena(&mut self) -> SPI_SMEM_TIMING_CLK_ENA_W {
        SPI_SMEM_TIMING_CLK_ENA_W { w: self }
    }
    #[doc = "Bit 1 - Set this bit to add extra SPI_CLK cycles in DUMMY phase for all reading operations."]
    #[inline(always)]
    pub fn spi_smem_timing_cali(&mut self) -> SPI_SMEM_TIMING_CALI_W {
        SPI_SMEM_TIMING_CALI_W { w: self }
    }
    #[doc = "Bits 2:4 - Extra SPI_CLK cycles added in DUMMY phase for timing compensation, when SPI0 accesses to Ext_RAM. Active when SPI_SMEM_TIMING_CALI bit is set."]
    #[inline(always)]
    pub fn spi_smem_extra_dummy_cyclelen(&mut self) -> SPI_SMEM_EXTRA_DUMMY_CYCLELEN_W {
        SPI_SMEM_EXTRA_DUMMY_CYCLELEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI0 Ext_RAM timing compensation register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_smem_timing_cali](index.html) module"]
pub struct SPI_SMEM_TIMING_CALI_SPEC;
impl crate::RegisterSpec for SPI_SMEM_TIMING_CALI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_smem_timing_cali::R](R) reader structure"]
impl crate::Readable for SPI_SMEM_TIMING_CALI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_smem_timing_cali::W](W) writer structure"]
impl crate::Writable for SPI_SMEM_TIMING_CALI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_SMEM_TIMING_CALI to value 0"]
impl crate::Resettable for SPI_SMEM_TIMING_CALI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
