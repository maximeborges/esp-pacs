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
#[doc = "Field `DIN0_MODE` reader - the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
pub struct DIN0_MODE_R(crate::FieldReader<u8>);
impl DIN0_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIN0_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIN0_MODE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIN0_MODE` writer - the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
pub struct DIN0_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DIN0_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u32 & 3);
        self.w
    }
}
#[doc = "Field `DIN1_MODE` reader - the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
pub struct DIN1_MODE_R(crate::FieldReader<u8>);
impl DIN1_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIN1_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIN1_MODE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIN1_MODE` writer - the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
pub struct DIN1_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DIN1_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 2)) | ((value as u32 & 3) << 2);
        self.w
    }
}
#[doc = "Field `DIN2_MODE` reader - the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
pub struct DIN2_MODE_R(crate::FieldReader<u8>);
impl DIN2_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIN2_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIN2_MODE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIN2_MODE` writer - the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
pub struct DIN2_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DIN2_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 4)) | ((value as u32 & 3) << 4);
        self.w
    }
}
#[doc = "Field `DIN3_MODE` reader - the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
pub struct DIN3_MODE_R(crate::FieldReader<u8>);
impl DIN3_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIN3_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIN3_MODE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIN3_MODE` writer - the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
pub struct DIN3_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DIN3_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 6)) | ((value as u32 & 3) << 6);
        self.w
    }
}
#[doc = "Field `TIMING_HCLK_ACTIVE` reader - 1:enable hclk in SPI input timing module. 0: disable it. Can be configured in CONF state."]
pub struct TIMING_HCLK_ACTIVE_R(crate::FieldReader<bool>);
impl TIMING_HCLK_ACTIVE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMING_HCLK_ACTIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMING_HCLK_ACTIVE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMING_HCLK_ACTIVE` writer - 1:enable hclk in SPI input timing module. 0: disable it. Can be configured in CONF state."]
pub struct TIMING_HCLK_ACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMING_HCLK_ACTIVE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 16)) | ((value as u32 & 1) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn din0_mode(&self) -> DIN0_MODE_R {
        DIN0_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn din1_mode(&self) -> DIN1_MODE_R {
        DIN1_MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn din2_mode(&self) -> DIN2_MODE_R {
        DIN2_MODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn din3_mode(&self) -> DIN3_MODE_R {
        DIN3_MODE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 16 - 1:enable hclk in SPI input timing module. 0: disable it. Can be configured in CONF state."]
    #[inline(always)]
    pub fn timing_hclk_active(&self) -> TIMING_HCLK_ACTIVE_R {
        TIMING_HCLK_ACTIVE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn din0_mode(&mut self) -> DIN0_MODE_W {
        DIN0_MODE_W { w: self }
    }
    #[doc = "Bits 2:3 - the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn din1_mode(&mut self) -> DIN1_MODE_W {
        DIN1_MODE_W { w: self }
    }
    #[doc = "Bits 4:5 - the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn din2_mode(&mut self) -> DIN2_MODE_W {
        DIN2_MODE_W { w: self }
    }
    #[doc = "Bits 6:7 - the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn din3_mode(&mut self) -> DIN3_MODE_W {
        DIN3_MODE_W { w: self }
    }
    #[doc = "Bit 16 - 1:enable hclk in SPI input timing module. 0: disable it. Can be configured in CONF state."]
    #[inline(always)]
    pub fn timing_hclk_active(&mut self) -> TIMING_HCLK_ACTIVE_W {
        TIMING_HCLK_ACTIVE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI input delay mode configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [din_mode](index.html) module"]
pub struct DIN_MODE_SPEC;
impl crate::RegisterSpec for DIN_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [din_mode::R](R) reader structure"]
impl crate::Readable for DIN_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [din_mode::W](W) writer structure"]
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
