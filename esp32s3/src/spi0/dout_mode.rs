#[doc = "Register `DOUT_MODE` reader"]
pub struct R(crate::R<DOUT_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOUT_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOUT_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOUT_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOUT_MODE` writer"]
pub struct W(crate::W<DOUT_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOUT_MODE_SPEC>;
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
impl From<crate::W<DOUT_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOUT_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DOUT0_MODE` reader - SPI_D output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
pub struct DOUT0_MODE_R(crate::FieldReader<bool, bool>);
impl DOUT0_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DOUT0_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOUT0_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOUT0_MODE` writer - SPI_D output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
pub struct DOUT0_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT0_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `DOUT1_MODE` reader - SPI_Q output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
pub struct DOUT1_MODE_R(crate::FieldReader<bool, bool>);
impl DOUT1_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DOUT1_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOUT1_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOUT1_MODE` writer - SPI_Q output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
pub struct DOUT1_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT1_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `DOUT2_MODE` reader - SPI_WP output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
pub struct DOUT2_MODE_R(crate::FieldReader<bool, bool>);
impl DOUT2_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DOUT2_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOUT2_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOUT2_MODE` writer - SPI_WP output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
pub struct DOUT2_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT2_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `DOUT3_MODE` reader - SPI_HD output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
pub struct DOUT3_MODE_R(crate::FieldReader<bool, bool>);
impl DOUT3_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DOUT3_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOUT3_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOUT3_MODE` writer - SPI_HD output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
pub struct DOUT3_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT3_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `DOUT4_MODE` reader - SPI_IO4 output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
pub struct DOUT4_MODE_R(crate::FieldReader<bool, bool>);
impl DOUT4_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DOUT4_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOUT4_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOUT4_MODE` writer - SPI_IO4 output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
pub struct DOUT4_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT4_MODE_W<'a> {
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
#[doc = "Field `DOUT5_MODE` reader - SPI_IO5 output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
pub struct DOUT5_MODE_R(crate::FieldReader<bool, bool>);
impl DOUT5_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DOUT5_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOUT5_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOUT5_MODE` writer - SPI_IO5 output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
pub struct DOUT5_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT5_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `DOUT6_MODE` reader - SPI_IO6 output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
pub struct DOUT6_MODE_R(crate::FieldReader<bool, bool>);
impl DOUT6_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DOUT6_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOUT6_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOUT6_MODE` writer - SPI_IO6 output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
pub struct DOUT6_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT6_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `DOUT7_MODE` reader - SPI_IO7 output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
pub struct DOUT7_MODE_R(crate::FieldReader<bool, bool>);
impl DOUT7_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DOUT7_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOUT7_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOUT7_MODE` writer - SPI_IO7 output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
pub struct DOUT7_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT7_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `DOUTS_MODE` reader - SPI_DQS output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
pub struct DOUTS_MODE_R(crate::FieldReader<bool, bool>);
impl DOUTS_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DOUTS_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOUTS_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOUTS_MODE` writer - SPI_DQS output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
pub struct DOUTS_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUTS_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - SPI_D output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    pub fn dout0_mode(&self) -> DOUT0_MODE_R {
        DOUT0_MODE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SPI_Q output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    pub fn dout1_mode(&self) -> DOUT1_MODE_R {
        DOUT1_MODE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SPI_WP output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    pub fn dout2_mode(&self) -> DOUT2_MODE_R {
        DOUT2_MODE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SPI_HD output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    pub fn dout3_mode(&self) -> DOUT3_MODE_R {
        DOUT3_MODE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SPI_IO4 output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    pub fn dout4_mode(&self) -> DOUT4_MODE_R {
        DOUT4_MODE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SPI_IO5 output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    pub fn dout5_mode(&self) -> DOUT5_MODE_R {
        DOUT5_MODE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SPI_IO6 output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    pub fn dout6_mode(&self) -> DOUT6_MODE_R {
        DOUT6_MODE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - SPI_IO7 output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    pub fn dout7_mode(&self) -> DOUT7_MODE_R {
        DOUT7_MODE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SPI_DQS output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    pub fn douts_mode(&self) -> DOUTS_MODE_R {
        DOUTS_MODE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI_D output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    pub fn dout0_mode(&mut self) -> DOUT0_MODE_W {
        DOUT0_MODE_W { w: self }
    }
    #[doc = "Bit 1 - SPI_Q output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    pub fn dout1_mode(&mut self) -> DOUT1_MODE_W {
        DOUT1_MODE_W { w: self }
    }
    #[doc = "Bit 2 - SPI_WP output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    pub fn dout2_mode(&mut self) -> DOUT2_MODE_W {
        DOUT2_MODE_W { w: self }
    }
    #[doc = "Bit 3 - SPI_HD output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    pub fn dout3_mode(&mut self) -> DOUT3_MODE_W {
        DOUT3_MODE_W { w: self }
    }
    #[doc = "Bit 4 - SPI_IO4 output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    pub fn dout4_mode(&mut self) -> DOUT4_MODE_W {
        DOUT4_MODE_W { w: self }
    }
    #[doc = "Bit 5 - SPI_IO5 output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    pub fn dout5_mode(&mut self) -> DOUT5_MODE_W {
        DOUT5_MODE_W { w: self }
    }
    #[doc = "Bit 6 - SPI_IO6 output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    pub fn dout6_mode(&mut self) -> DOUT6_MODE_W {
        DOUT6_MODE_W { w: self }
    }
    #[doc = "Bit 7 - SPI_IO7 output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    pub fn dout7_mode(&mut self) -> DOUT7_MODE_W {
        DOUT7_MODE_W { w: self }
    }
    #[doc = "Bit 8 - SPI_DQS output delay mode. 0: No delay. 1: Delay one cycle at MSPI_CORE_CLK negative edge."]
    #[inline(always)]
    pub fn douts_mode(&mut self) -> DOUTS_MODE_W {
        DOUTS_MODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MSPI output timing delay mode control register when accesses to flash.\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dout_mode]
(index.html) module"]
pub struct DOUT_MODE_SPEC;
impl crate::RegisterSpec for DOUT_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dout_mode::R]
(R) reader structure"]
impl crate::Readable for DOUT_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dout_mode::W]
(W) writer structure"]
impl crate::Writable for DOUT_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DOUT_MODE to value 0"]
impl crate::Resettable for DOUT_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
