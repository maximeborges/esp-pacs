#[doc = "Register `IN_CONF0_CH0` reader"]
pub struct R(crate::R<IN_CONF0_CH0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IN_CONF0_CH0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IN_CONF0_CH0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IN_CONF0_CH0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IN_CONF0_CH0` writer"]
pub struct W(crate::W<IN_CONF0_CH0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IN_CONF0_CH0_SPEC>;
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
impl From<crate::W<IN_CONF0_CH0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IN_CONF0_CH0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IN_RST_CH0` reader - This bit is used to reset DMA channel 0 Rx FSM and Rx FIFO pointer."]
pub struct IN_RST_CH0_R(crate::FieldReader<bool, bool>);
impl IN_RST_CH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IN_RST_CH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN_RST_CH0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN_RST_CH0` writer - This bit is used to reset DMA channel 0 Rx FSM and Rx FIFO pointer."]
pub struct IN_RST_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_RST_CH0_W<'a> {
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
#[doc = "Field `IN_LOOP_TEST_CH0` reader - reserved"]
pub struct IN_LOOP_TEST_CH0_R(crate::FieldReader<bool, bool>);
impl IN_LOOP_TEST_CH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IN_LOOP_TEST_CH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN_LOOP_TEST_CH0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN_LOOP_TEST_CH0` writer - reserved"]
pub struct IN_LOOP_TEST_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_LOOP_TEST_CH0_W<'a> {
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
#[doc = "Field `INDSCR_BURST_EN_CH0` reader - Set this bit to 1 to enable INCR burst transfer for Rx channel 0 reading link descriptor when accessing internal SRAM."]
pub struct INDSCR_BURST_EN_CH0_R(crate::FieldReader<bool, bool>);
impl INDSCR_BURST_EN_CH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INDSCR_BURST_EN_CH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INDSCR_BURST_EN_CH0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INDSCR_BURST_EN_CH0` writer - Set this bit to 1 to enable INCR burst transfer for Rx channel 0 reading link descriptor when accessing internal SRAM."]
pub struct INDSCR_BURST_EN_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> INDSCR_BURST_EN_CH0_W<'a> {
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
#[doc = "Field `IN_DATA_BURST_EN_CH0` reader - Set this bit to 1 to enable INCR burst transfer for Rx channel 0 receiving data when accessing internal SRAM."]
pub struct IN_DATA_BURST_EN_CH0_R(crate::FieldReader<bool, bool>);
impl IN_DATA_BURST_EN_CH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IN_DATA_BURST_EN_CH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN_DATA_BURST_EN_CH0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN_DATA_BURST_EN_CH0` writer - Set this bit to 1 to enable INCR burst transfer for Rx channel 0 receiving data when accessing internal SRAM."]
pub struct IN_DATA_BURST_EN_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_DATA_BURST_EN_CH0_W<'a> {
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
#[doc = "Field `MEM_TRANS_EN_CH0` reader - Set this bit 1 to enable automatic transmitting data from memory to memory via DMA."]
pub struct MEM_TRANS_EN_CH0_R(crate::FieldReader<bool, bool>);
impl MEM_TRANS_EN_CH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEM_TRANS_EN_CH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM_TRANS_EN_CH0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM_TRANS_EN_CH0` writer - Set this bit 1 to enable automatic transmitting data from memory to memory via DMA."]
pub struct MEM_TRANS_EN_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_TRANS_EN_CH0_W<'a> {
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
impl R {
    #[doc = "Bit 0 - This bit is used to reset DMA channel 0 Rx FSM and Rx FIFO pointer."]
    #[inline(always)]
    pub fn in_rst_ch0(&self) -> IN_RST_CH0_R {
        IN_RST_CH0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn in_loop_test_ch0(&self) -> IN_LOOP_TEST_CH0_R {
        IN_LOOP_TEST_CH0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Set this bit to 1 to enable INCR burst transfer for Rx channel 0 reading link descriptor when accessing internal SRAM."]
    #[inline(always)]
    pub fn indscr_burst_en_ch0(&self) -> INDSCR_BURST_EN_CH0_R {
        INDSCR_BURST_EN_CH0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Set this bit to 1 to enable INCR burst transfer for Rx channel 0 receiving data when accessing internal SRAM."]
    #[inline(always)]
    pub fn in_data_burst_en_ch0(&self) -> IN_DATA_BURST_EN_CH0_R {
        IN_DATA_BURST_EN_CH0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Set this bit 1 to enable automatic transmitting data from memory to memory via DMA."]
    #[inline(always)]
    pub fn mem_trans_en_ch0(&self) -> MEM_TRANS_EN_CH0_R {
        MEM_TRANS_EN_CH0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is used to reset DMA channel 0 Rx FSM and Rx FIFO pointer."]
    #[inline(always)]
    pub fn in_rst_ch0(&mut self) -> IN_RST_CH0_W {
        IN_RST_CH0_W { w: self }
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn in_loop_test_ch0(&mut self) -> IN_LOOP_TEST_CH0_W {
        IN_LOOP_TEST_CH0_W { w: self }
    }
    #[doc = "Bit 2 - Set this bit to 1 to enable INCR burst transfer for Rx channel 0 reading link descriptor when accessing internal SRAM."]
    #[inline(always)]
    pub fn indscr_burst_en_ch0(&mut self) -> INDSCR_BURST_EN_CH0_W {
        INDSCR_BURST_EN_CH0_W { w: self }
    }
    #[doc = "Bit 3 - Set this bit to 1 to enable INCR burst transfer for Rx channel 0 receiving data when accessing internal SRAM."]
    #[inline(always)]
    pub fn in_data_burst_en_ch0(&mut self) -> IN_DATA_BURST_EN_CH0_W {
        IN_DATA_BURST_EN_CH0_W { w: self }
    }
    #[doc = "Bit 4 - Set this bit 1 to enable automatic transmitting data from memory to memory via DMA."]
    #[inline(always)]
    pub fn mem_trans_en_ch0(&mut self) -> MEM_TRANS_EN_CH0_W {
        MEM_TRANS_EN_CH0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA_IN_CONF0_CH0_REG.\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [in_conf0_ch0]
(index.html) module"]
pub struct IN_CONF0_CH0_SPEC;
impl crate::RegisterSpec for IN_CONF0_CH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [in_conf0_ch0::R]
(R) reader structure"]
impl crate::Readable for IN_CONF0_CH0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [in_conf0_ch0::W]
(W) writer structure"]
impl crate::Writable for IN_CONF0_CH0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IN_CONF0_CH0 to value 0"]
impl crate::Resettable for IN_CONF0_CH0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
