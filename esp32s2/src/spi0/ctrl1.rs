#[doc = "Register `CTRL1` reader"]
pub struct R(crate::R<CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL1` writer"]
pub struct W(crate::W<CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL1_SPEC>;
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
impl From<crate::W<CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLK_MODE` reader - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is alwasy on. Can be configured in CONF state."]
pub struct CLK_MODE_R(crate::FieldReader<u8, u8>);
impl CLK_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CLK_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK_MODE` writer - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is alwasy on. Can be configured in CONF state."]
pub struct CLK_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `CLK_MODE_13` reader - {CPOL, CPHA},1: support spi clk mode 1 and 3, first edge output data B\\[0\\]
/B\\[7\\]
. 0: support spi clk mode 0 and 2, first edge output data B\\[1\\]
/B\\[6\\]
."]
pub struct CLK_MODE_13_R(crate::FieldReader<bool, bool>);
impl CLK_MODE_13_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_MODE_13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_MODE_13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK_MODE_13` writer - {CPOL, CPHA},1: support spi clk mode 1 and 3, first edge output data B\\[0\\]
/B\\[7\\]
. 0: support spi clk mode 0 and 2, first edge output data B\\[1\\]
/B\\[6\\]
."]
pub struct CLK_MODE_13_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_MODE_13_W<'a> {
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
#[doc = "Field `RSCK_DATA_OUT` reader - It saves half a cycle when tsck is the same as rsck. 1: output data at rsck posedge 0: output data at tsck posedge"]
pub struct RSCK_DATA_OUT_R(crate::FieldReader<bool, bool>);
impl RSCK_DATA_OUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RSCK_DATA_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSCK_DATA_OUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSCK_DATA_OUT` writer - It saves half a cycle when tsck is the same as rsck. 1: output data at rsck posedge 0: output data at tsck posedge"]
pub struct RSCK_DATA_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> RSCK_DATA_OUT_W<'a> {
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
#[doc = "Field `W16_17_WR_ENA` reader - 1:SPI_BUF16~SPI_BUF17 can be written 0:SPI_BUF16~SPI_BUF17 can not be written. Can be configured in CONF state."]
pub struct W16_17_WR_ENA_R(crate::FieldReader<bool, bool>);
impl W16_17_WR_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        W16_17_WR_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for W16_17_WR_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `W16_17_WR_ENA` writer - 1:SPI_BUF16~SPI_BUF17 can be written 0:SPI_BUF16~SPI_BUF17 can not be written. Can be configured in CONF state."]
pub struct W16_17_WR_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> W16_17_WR_ENA_W<'a> {
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
#[doc = "Field `CS_HOLD_DELAY` reader - SPI cs signal is delayed by spi clock cycles. Can be configured in CONF state."]
pub struct CS_HOLD_DELAY_R(crate::FieldReader<u8, u8>);
impl CS_HOLD_DELAY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CS_HOLD_DELAY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CS_HOLD_DELAY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CS_HOLD_DELAY` writer - SPI cs signal is delayed by spi clock cycles. Can be configured in CONF state."]
pub struct CS_HOLD_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> CS_HOLD_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 14)) | ((value as u32 & 0x3f) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is alwasy on. Can be configured in CONF state."]
    #[inline(always)]
    pub fn clk_mode(&self) -> CLK_MODE_R {
        CLK_MODE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - {CPOL, CPHA},1: support spi clk mode 1 and 3, first edge output data B\\[0\\]
/B\\[7\\]
. 0: support spi clk mode 0 and 2, first edge output data B\\[1\\]
/B\\[6\\]
."]
    #[inline(always)]
    pub fn clk_mode_13(&self) -> CLK_MODE_13_R {
        CLK_MODE_13_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - It saves half a cycle when tsck is the same as rsck. 1: output data at rsck posedge 0: output data at tsck posedge"]
    #[inline(always)]
    pub fn rsck_data_out(&self) -> RSCK_DATA_OUT_R {
        RSCK_DATA_OUT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 1:SPI_BUF16~SPI_BUF17 can be written 0:SPI_BUF16~SPI_BUF17 can not be written. Can be configured in CONF state."]
    #[inline(always)]
    pub fn w16_17_wr_ena(&self) -> W16_17_WR_ENA_R {
        W16_17_WR_ENA_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 14:19 - SPI cs signal is delayed by spi clock cycles. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cs_hold_delay(&self) -> CS_HOLD_DELAY_R {
        CS_HOLD_DELAY_R::new(((self.bits >> 14) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is alwasy on. Can be configured in CONF state."]
    #[inline(always)]
    pub fn clk_mode(&mut self) -> CLK_MODE_W {
        CLK_MODE_W { w: self }
    }
    #[doc = "Bit 2 - {CPOL, CPHA},1: support spi clk mode 1 and 3, first edge output data B\\[0\\]
/B\\[7\\]
. 0: support spi clk mode 0 and 2, first edge output data B\\[1\\]
/B\\[6\\]
."]
    #[inline(always)]
    pub fn clk_mode_13(&mut self) -> CLK_MODE_13_W {
        CLK_MODE_13_W { w: self }
    }
    #[doc = "Bit 3 - It saves half a cycle when tsck is the same as rsck. 1: output data at rsck posedge 0: output data at tsck posedge"]
    #[inline(always)]
    pub fn rsck_data_out(&mut self) -> RSCK_DATA_OUT_W {
        RSCK_DATA_OUT_W { w: self }
    }
    #[doc = "Bit 4 - 1:SPI_BUF16~SPI_BUF17 can be written 0:SPI_BUF16~SPI_BUF17 can not be written. Can be configured in CONF state."]
    #[inline(always)]
    pub fn w16_17_wr_ena(&mut self) -> W16_17_WR_ENA_W {
        W16_17_WR_ENA_W { w: self }
    }
    #[doc = "Bits 14:19 - SPI cs signal is delayed by spi clock cycles. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cs_hold_delay(&mut self) -> CS_HOLD_DELAY_W {
        CS_HOLD_DELAY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI control register 1\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl1]
(index.html) module"]
pub struct CTRL1_SPEC;
impl crate::RegisterSpec for CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl1::R]
(R) reader structure"]
impl crate::Readable for CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl1::W]
(W) writer structure"]
impl crate::Writable for CTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL1 to value 0x4010"]
impl crate::Resettable for CTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4010
    }
}
