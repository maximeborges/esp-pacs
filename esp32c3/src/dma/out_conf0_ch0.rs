#[doc = "Register `OUT_CONF0_CH0` reader"]
pub struct R(crate::R<OUT_CONF0_CH0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUT_CONF0_CH0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUT_CONF0_CH0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUT_CONF0_CH0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUT_CONF0_CH0` writer"]
pub struct W(crate::W<OUT_CONF0_CH0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUT_CONF0_CH0_SPEC>;
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
impl From<crate::W<OUT_CONF0_CH0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUT_CONF0_CH0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUT_RST_CH0` reader - This bit is used to reset DMA channel 0 Tx FSM and Tx FIFO pointer."]
pub struct OUT_RST_CH0_R(crate::FieldReader<bool, bool>);
impl OUT_RST_CH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUT_RST_CH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_RST_CH0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_RST_CH0` writer - This bit is used to reset DMA channel 0 Tx FSM and Tx FIFO pointer."]
pub struct OUT_RST_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_RST_CH0_W<'a> {
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
#[doc = "Field `OUT_LOOP_TEST_CH0` reader - reserved"]
pub struct OUT_LOOP_TEST_CH0_R(crate::FieldReader<bool, bool>);
impl OUT_LOOP_TEST_CH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUT_LOOP_TEST_CH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_LOOP_TEST_CH0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_LOOP_TEST_CH0` writer - reserved"]
pub struct OUT_LOOP_TEST_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_LOOP_TEST_CH0_W<'a> {
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
#[doc = "Field `OUT_AUTO_WRBACK_CH0` reader - Set this bit to enable automatic outlink-writeback when all the data in tx buffer has been transmitted."]
pub struct OUT_AUTO_WRBACK_CH0_R(crate::FieldReader<bool, bool>);
impl OUT_AUTO_WRBACK_CH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUT_AUTO_WRBACK_CH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_AUTO_WRBACK_CH0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_AUTO_WRBACK_CH0` writer - Set this bit to enable automatic outlink-writeback when all the data in tx buffer has been transmitted."]
pub struct OUT_AUTO_WRBACK_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_AUTO_WRBACK_CH0_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u32 & 1) << 2);
        self.w
    }
}
#[doc = "Field `OUT_EOF_MODE_CH0` reader - EOF flag generation mode when transmitting data. 1: EOF flag for Tx channel 0 is generated when data need to transmit has been popped from FIFO in DMA"]
pub struct OUT_EOF_MODE_CH0_R(crate::FieldReader<bool, bool>);
impl OUT_EOF_MODE_CH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUT_EOF_MODE_CH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_EOF_MODE_CH0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_EOF_MODE_CH0` writer - EOF flag generation mode when transmitting data. 1: EOF flag for Tx channel 0 is generated when data need to transmit has been popped from FIFO in DMA"]
pub struct OUT_EOF_MODE_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_EOF_MODE_CH0_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u32 & 1) << 3);
        self.w
    }
}
#[doc = "Field `OUTDSCR_BURST_EN_CH0` reader - Set this bit to 1 to enable INCR burst transfer for Tx channel 0 reading link descriptor when accessing internal SRAM."]
pub struct OUTDSCR_BURST_EN_CH0_R(crate::FieldReader<bool, bool>);
impl OUTDSCR_BURST_EN_CH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUTDSCR_BURST_EN_CH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTDSCR_BURST_EN_CH0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTDSCR_BURST_EN_CH0` writer - Set this bit to 1 to enable INCR burst transfer for Tx channel 0 reading link descriptor when accessing internal SRAM."]
pub struct OUTDSCR_BURST_EN_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTDSCR_BURST_EN_CH0_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u32 & 1) << 4);
        self.w
    }
}
#[doc = "Field `OUT_DATA_BURST_EN_CH0` reader - Set this bit to 1 to enable INCR burst transfer for Tx channel 0 transmitting data when accessing internal SRAM."]
pub struct OUT_DATA_BURST_EN_CH0_R(crate::FieldReader<bool, bool>);
impl OUT_DATA_BURST_EN_CH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUT_DATA_BURST_EN_CH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_DATA_BURST_EN_CH0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_DATA_BURST_EN_CH0` writer - Set this bit to 1 to enable INCR burst transfer for Tx channel 0 transmitting data when accessing internal SRAM."]
pub struct OUT_DATA_BURST_EN_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_DATA_BURST_EN_CH0_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u32 & 1) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - This bit is used to reset DMA channel 0 Tx FSM and Tx FIFO pointer."]
    #[inline(always)]
    pub fn out_rst_ch0(&self) -> OUT_RST_CH0_R {
        OUT_RST_CH0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn out_loop_test_ch0(&self) -> OUT_LOOP_TEST_CH0_R {
        OUT_LOOP_TEST_CH0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to enable automatic outlink-writeback when all the data in tx buffer has been transmitted."]
    #[inline(always)]
    pub fn out_auto_wrback_ch0(&self) -> OUT_AUTO_WRBACK_CH0_R {
        OUT_AUTO_WRBACK_CH0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EOF flag generation mode when transmitting data. 1: EOF flag for Tx channel 0 is generated when data need to transmit has been popped from FIFO in DMA"]
    #[inline(always)]
    pub fn out_eof_mode_ch0(&self) -> OUT_EOF_MODE_CH0_R {
        OUT_EOF_MODE_CH0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit to 1 to enable INCR burst transfer for Tx channel 0 reading link descriptor when accessing internal SRAM."]
    #[inline(always)]
    pub fn outdscr_burst_en_ch0(&self) -> OUTDSCR_BURST_EN_CH0_R {
        OUTDSCR_BURST_EN_CH0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set this bit to 1 to enable INCR burst transfer for Tx channel 0 transmitting data when accessing internal SRAM."]
    #[inline(always)]
    pub fn out_data_burst_en_ch0(&self) -> OUT_DATA_BURST_EN_CH0_R {
        OUT_DATA_BURST_EN_CH0_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is used to reset DMA channel 0 Tx FSM and Tx FIFO pointer."]
    #[inline(always)]
    pub fn out_rst_ch0(&mut self) -> OUT_RST_CH0_W {
        OUT_RST_CH0_W { w: self }
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn out_loop_test_ch0(&mut self) -> OUT_LOOP_TEST_CH0_W {
        OUT_LOOP_TEST_CH0_W { w: self }
    }
    #[doc = "Bit 2 - Set this bit to enable automatic outlink-writeback when all the data in tx buffer has been transmitted."]
    #[inline(always)]
    pub fn out_auto_wrback_ch0(&mut self) -> OUT_AUTO_WRBACK_CH0_W {
        OUT_AUTO_WRBACK_CH0_W { w: self }
    }
    #[doc = "Bit 3 - EOF flag generation mode when transmitting data. 1: EOF flag for Tx channel 0 is generated when data need to transmit has been popped from FIFO in DMA"]
    #[inline(always)]
    pub fn out_eof_mode_ch0(&mut self) -> OUT_EOF_MODE_CH0_W {
        OUT_EOF_MODE_CH0_W { w: self }
    }
    #[doc = "Bit 4 - Set this bit to 1 to enable INCR burst transfer for Tx channel 0 reading link descriptor when accessing internal SRAM."]
    #[inline(always)]
    pub fn outdscr_burst_en_ch0(&mut self) -> OUTDSCR_BURST_EN_CH0_W {
        OUTDSCR_BURST_EN_CH0_W { w: self }
    }
    #[doc = "Bit 5 - Set this bit to 1 to enable INCR burst transfer for Tx channel 0 transmitting data when accessing internal SRAM."]
    #[inline(always)]
    pub fn out_data_burst_en_ch0(&mut self) -> OUT_DATA_BURST_EN_CH0_W {
        OUT_DATA_BURST_EN_CH0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA_OUT_CONF0_CH0_REG.\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_conf0_ch0]
(index.html) module"]
pub struct OUT_CONF0_CH0_SPEC;
impl crate::RegisterSpec for OUT_CONF0_CH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [out_conf0_ch0::R]
(R) reader structure"]
impl crate::Readable for OUT_CONF0_CH0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [out_conf0_ch0::W]
(W) writer structure"]
impl crate::Writable for OUT_CONF0_CH0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OUT_CONF0_CH0 to value 0x08"]
impl crate::Resettable for OUT_CONF0_CH0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x08
    }
}
