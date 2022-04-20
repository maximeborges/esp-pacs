#[doc = "Register `DMA_CONF` reader"]
pub struct R(crate::R<DMA_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_CONF` writer"]
pub struct W(crate::W<DMA_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_CONF_SPEC>;
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
impl From<crate::W<DMA_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IN_RST` reader - The bit is used to reset in dma fsm and in data fifo pointer."]
pub struct IN_RST_R(crate::FieldReader<bool, bool>);
impl IN_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IN_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN_RST` writer - The bit is used to reset in dma fsm and in data fifo pointer."]
pub struct IN_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_RST_W<'a> {
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
#[doc = "Field `OUT_RST` reader - The bit is used to reset out dma fsm and out data fifo pointer."]
pub struct OUT_RST_R(crate::FieldReader<bool, bool>);
impl OUT_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUT_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_RST` writer - The bit is used to reset out dma fsm and out data fifo pointer."]
pub struct OUT_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_RST_W<'a> {
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
#[doc = "Field `AHBM_FIFO_RST` reader - Reset spi dma ahb master fifo pointer."]
pub struct AHBM_FIFO_RST_R(crate::FieldReader<bool, bool>);
impl AHBM_FIFO_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AHBM_FIFO_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AHBM_FIFO_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AHBM_FIFO_RST` writer - Reset spi dma ahb master fifo pointer."]
pub struct AHBM_FIFO_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> AHBM_FIFO_RST_W<'a> {
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
#[doc = "Field `AHBM_RST` reader - Reset spi dma ahb master."]
pub struct AHBM_RST_R(crate::FieldReader<bool, bool>);
impl AHBM_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AHBM_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AHBM_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AHBM_RST` writer - Reset spi dma ahb master."]
pub struct AHBM_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> AHBM_RST_W<'a> {
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
#[doc = "Field `IN_LOOP_TEST` reader - Set bit to test in link."]
pub struct IN_LOOP_TEST_R(crate::FieldReader<bool, bool>);
impl IN_LOOP_TEST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IN_LOOP_TEST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN_LOOP_TEST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN_LOOP_TEST` writer - Set bit to test in link."]
pub struct IN_LOOP_TEST_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_LOOP_TEST_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 6)) | ((value as u32 & 1) << 6);
        self.w
    }
}
#[doc = "Field `OUT_LOOP_TEST` reader - Set bit to test out link."]
pub struct OUT_LOOP_TEST_R(crate::FieldReader<bool, bool>);
impl OUT_LOOP_TEST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUT_LOOP_TEST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_LOOP_TEST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_LOOP_TEST` writer - Set bit to test out link."]
pub struct OUT_LOOP_TEST_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_LOOP_TEST_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 7)) | ((value as u32 & 1) << 7);
        self.w
    }
}
#[doc = "Field `OUT_AUTO_WRBACK` reader - when the bit is set, DMA continue to use the next inlink node when the length of inlink is 0."]
pub struct OUT_AUTO_WRBACK_R(crate::FieldReader<bool, bool>);
impl OUT_AUTO_WRBACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUT_AUTO_WRBACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_AUTO_WRBACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_AUTO_WRBACK` writer - when the bit is set, DMA continue to use the next inlink node when the length of inlink is 0."]
pub struct OUT_AUTO_WRBACK_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_AUTO_WRBACK_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 8)) | ((value as u32 & 1) << 8);
        self.w
    }
}
#[doc = "Field `OUT_EOF_MODE` reader - out eof flag generation mode . 1: when dma pop all data from fifo 0:when ahb push all data to fifo."]
pub struct OUT_EOF_MODE_R(crate::FieldReader<bool, bool>);
impl OUT_EOF_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUT_EOF_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_EOF_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_EOF_MODE` writer - out eof flag generation mode . 1: when dma pop all data from fifo 0:when ahb push all data to fifo."]
pub struct OUT_EOF_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_EOF_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 9)) | ((value as u32 & 1) << 9);
        self.w
    }
}
#[doc = "Field `OUTDSCR_BURST_EN` reader - read descriptor use burst mode when read data for memory."]
pub struct OUTDSCR_BURST_EN_R(crate::FieldReader<bool, bool>);
impl OUTDSCR_BURST_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUTDSCR_BURST_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTDSCR_BURST_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTDSCR_BURST_EN` writer - read descriptor use burst mode when read data for memory."]
pub struct OUTDSCR_BURST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTDSCR_BURST_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 10)) | ((value as u32 & 1) << 10);
        self.w
    }
}
#[doc = "Field `INDSCR_BURST_EN` reader - read descriptor use burst mode when write data to memory."]
pub struct INDSCR_BURST_EN_R(crate::FieldReader<bool, bool>);
impl INDSCR_BURST_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INDSCR_BURST_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INDSCR_BURST_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INDSCR_BURST_EN` writer - read descriptor use burst mode when write data to memory."]
pub struct INDSCR_BURST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> INDSCR_BURST_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 11)) | ((value as u32 & 1) << 11);
        self.w
    }
}
#[doc = "Field `OUT_DATA_BURST_EN` reader - spi dma read data from memory in burst mode."]
pub struct OUT_DATA_BURST_EN_R(crate::FieldReader<bool, bool>);
impl OUT_DATA_BURST_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUT_DATA_BURST_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_DATA_BURST_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_DATA_BURST_EN` writer - spi dma read data from memory in burst mode."]
pub struct OUT_DATA_BURST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_DATA_BURST_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 12)) | ((value as u32 & 1) << 12);
        self.w
    }
}
#[doc = "Field `MEM_TRANS_EN` reader - 1: Internal memory data transfer enable bit. Send SPI DMA RX buffer data to SPI DMA TX buffer. 0: Disable this function."]
pub struct MEM_TRANS_EN_R(crate::FieldReader<bool, bool>);
impl MEM_TRANS_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEM_TRANS_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM_TRANS_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM_TRANS_EN` writer - 1: Internal memory data transfer enable bit. Send SPI DMA RX buffer data to SPI DMA TX buffer. 0: Disable this function."]
pub struct MEM_TRANS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_TRANS_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 13)) | ((value as u32 & 1) << 13);
        self.w
    }
}
#[doc = "Field `DMA_RX_STOP` reader - spi dma read data stop when in continue tx/rx mode."]
pub struct DMA_RX_STOP_R(crate::FieldReader<bool, bool>);
impl DMA_RX_STOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_RX_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_RX_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_RX_STOP` writer - spi dma read data stop when in continue tx/rx mode."]
pub struct DMA_RX_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_RX_STOP_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 14)) | ((value as u32 & 1) << 14);
        self.w
    }
}
#[doc = "Field `DMA_TX_STOP` reader - spi dma write data stop when in continue tx/rx mode."]
pub struct DMA_TX_STOP_R(crate::FieldReader<bool, bool>);
impl DMA_TX_STOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_TX_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_TX_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_TX_STOP` writer - spi dma write data stop when in continue tx/rx mode."]
pub struct DMA_TX_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_TX_STOP_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 15)) | ((value as u32 & 1) << 15);
        self.w
    }
}
#[doc = "Field `DMA_CONTINUE` reader - spi dma continue tx/rx data."]
pub struct DMA_CONTINUE_R(crate::FieldReader<bool, bool>);
impl DMA_CONTINUE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_CONTINUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_CONTINUE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_CONTINUE` writer - spi dma continue tx/rx data."]
pub struct DMA_CONTINUE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_CONTINUE_W<'a> {
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
#[doc = "Field `SLV_LAST_SEG_POP_CLR` reader - 1: Clear spi_slv_seg_frt_pop_mask. 0 : others"]
pub struct SLV_LAST_SEG_POP_CLR_R(crate::FieldReader<bool, bool>);
impl SLV_LAST_SEG_POP_CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLV_LAST_SEG_POP_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_LAST_SEG_POP_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_LAST_SEG_POP_CLR` writer - 1: Clear spi_slv_seg_frt_pop_mask. 0 : others"]
pub struct SLV_LAST_SEG_POP_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_LAST_SEG_POP_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 17)) | ((value as u32 & 1) << 17);
        self.w
    }
}
#[doc = "Field `DMA_SLV_SEG_TRANS_EN` reader - Enable dma segment transfer in spi dma half slave mode. 1: enable. 0: disable."]
pub struct DMA_SLV_SEG_TRANS_EN_R(crate::FieldReader<bool, bool>);
impl DMA_SLV_SEG_TRANS_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_SLV_SEG_TRANS_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_SLV_SEG_TRANS_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_SLV_SEG_TRANS_EN` writer - Enable dma segment transfer in spi dma half slave mode. 1: enable. 0: disable."]
pub struct DMA_SLV_SEG_TRANS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_SLV_SEG_TRANS_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 18)) | ((value as u32 & 1) << 18);
        self.w
    }
}
#[doc = "Field `SLV_RX_SEG_TRANS_CLR_EN` reader - 1: spi_dma_infifo_full_vld is cleared by spi slave CMD5. 0: spi_dma_infifo_full_vld is cleared by SPI_TRANS_DONE."]
pub struct SLV_RX_SEG_TRANS_CLR_EN_R(crate::FieldReader<bool, bool>);
impl SLV_RX_SEG_TRANS_CLR_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLV_RX_SEG_TRANS_CLR_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_RX_SEG_TRANS_CLR_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_RX_SEG_TRANS_CLR_EN` writer - 1: spi_dma_infifo_full_vld is cleared by spi slave CMD5. 0: spi_dma_infifo_full_vld is cleared by SPI_TRANS_DONE."]
pub struct SLV_RX_SEG_TRANS_CLR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_RX_SEG_TRANS_CLR_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 19)) | ((value as u32 & 1) << 19);
        self.w
    }
}
#[doc = "Field `SLV_TX_SEG_TRANS_CLR_EN` reader - 1: spi_dma_outfifo_empty_vld is cleared by spi slave CMD6. 0: spi_dma_outfifo_empty_vld is cleared by SPI_TRANS_DONE."]
pub struct SLV_TX_SEG_TRANS_CLR_EN_R(crate::FieldReader<bool, bool>);
impl SLV_TX_SEG_TRANS_CLR_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLV_TX_SEG_TRANS_CLR_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_TX_SEG_TRANS_CLR_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_TX_SEG_TRANS_CLR_EN` writer - 1: spi_dma_outfifo_empty_vld is cleared by spi slave CMD6. 0: spi_dma_outfifo_empty_vld is cleared by SPI_TRANS_DONE."]
pub struct SLV_TX_SEG_TRANS_CLR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_TX_SEG_TRANS_CLR_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 20)) | ((value as u32 & 1) << 20);
        self.w
    }
}
#[doc = "Field `RX_EOF_EN` reader - 1: SPI_IN_SUC_EOF_INT_RAW is set when the number of dma pushed data bytes is equal to the value of SPI_SLV_DMA_RD_BYTELEN\\[19:0\\]
/ SPI_MST_DMA_RD_BYTELEN\\[19:0\\]
 in spi dma transition. 0: SPI_IN_SUC_EOF_INT_RAW is set by SPI_TRANS_DONE in non-seg-trans or SPI_DMA_SEG_TRANS_DONE in seg-trans."]
pub struct RX_EOF_EN_R(crate::FieldReader<bool, bool>);
impl RX_EOF_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_EOF_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_EOF_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_EOF_EN` writer - 1: SPI_IN_SUC_EOF_INT_RAW is set when the number of dma pushed data bytes is equal to the value of SPI_SLV_DMA_RD_BYTELEN\\[19:0\\]
/ SPI_MST_DMA_RD_BYTELEN\\[19:0\\]
 in spi dma transition. 0: SPI_IN_SUC_EOF_INT_RAW is set by SPI_TRANS_DONE in non-seg-trans or SPI_DMA_SEG_TRANS_DONE in seg-trans."]
pub struct RX_EOF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_EOF_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 21)) | ((value as u32 & 1) << 21);
        self.w
    }
}
#[doc = "Field `DMA_INFIFO_FULL_CLR` reader - 1:Clear spi_dma_infifo_full_vld. 0: Do not control it."]
pub struct DMA_INFIFO_FULL_CLR_R(crate::FieldReader<bool, bool>);
impl DMA_INFIFO_FULL_CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_INFIFO_FULL_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_INFIFO_FULL_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_INFIFO_FULL_CLR` writer - 1:Clear spi_dma_infifo_full_vld. 0: Do not control it."]
pub struct DMA_INFIFO_FULL_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_INFIFO_FULL_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 22)) | ((value as u32 & 1) << 22);
        self.w
    }
}
#[doc = "Field `DMA_OUTFIFO_EMPTY_CLR` reader - 1:Clear spi_dma_outfifo_empty_vld. 0: Do not control it."]
pub struct DMA_OUTFIFO_EMPTY_CLR_R(crate::FieldReader<bool, bool>);
impl DMA_OUTFIFO_EMPTY_CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_OUTFIFO_EMPTY_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_OUTFIFO_EMPTY_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_OUTFIFO_EMPTY_CLR` writer - 1:Clear spi_dma_outfifo_empty_vld. 0: Do not control it."]
pub struct DMA_OUTFIFO_EMPTY_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_OUTFIFO_EMPTY_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 23)) | ((value as u32 & 1) << 23);
        self.w
    }
}
#[doc = "Field `EXT_MEM_BK_SIZE` reader - Select the external memory block size."]
pub struct EXT_MEM_BK_SIZE_R(crate::FieldReader<u8, u8>);
impl EXT_MEM_BK_SIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EXT_MEM_BK_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXT_MEM_BK_SIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXT_MEM_BK_SIZE` writer - Select the external memory block size."]
pub struct EXT_MEM_BK_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> EXT_MEM_BK_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 26)) | ((value as u32 & 3) << 26);
        self.w
    }
}
#[doc = "Field `DMA_SEG_TRANS_CLR` reader - 1: End slave seg-trans, which acts as 0x05 command. 2 or more end seg-trans signals will induce error in DMA RX. 0: others. Will be cleared in 1 APB CLK cycles by hardware.."]
pub struct DMA_SEG_TRANS_CLR_R(crate::FieldReader<bool, bool>);
impl DMA_SEG_TRANS_CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_SEG_TRANS_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_SEG_TRANS_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_SEG_TRANS_CLR` writer - 1: End slave seg-trans, which acts as 0x05 command. 2 or more end seg-trans signals will induce error in DMA RX. 0: others. Will be cleared in 1 APB CLK cycles by hardware.."]
pub struct DMA_SEG_TRANS_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_SEG_TRANS_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 28)) | ((value as u32 & 1) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - The bit is used to reset in dma fsm and in data fifo pointer."]
    #[inline(always)]
    pub fn in_rst(&self) -> IN_RST_R {
        IN_RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The bit is used to reset out dma fsm and out data fifo pointer."]
    #[inline(always)]
    pub fn out_rst(&self) -> OUT_RST_R {
        OUT_RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reset spi dma ahb master fifo pointer."]
    #[inline(always)]
    pub fn ahbm_fifo_rst(&self) -> AHBM_FIFO_RST_R {
        AHBM_FIFO_RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Reset spi dma ahb master."]
    #[inline(always)]
    pub fn ahbm_rst(&self) -> AHBM_RST_R {
        AHBM_RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set bit to test in link."]
    #[inline(always)]
    pub fn in_loop_test(&self) -> IN_LOOP_TEST_R {
        IN_LOOP_TEST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Set bit to test out link."]
    #[inline(always)]
    pub fn out_loop_test(&self) -> OUT_LOOP_TEST_R {
        OUT_LOOP_TEST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - when the bit is set, DMA continue to use the next inlink node when the length of inlink is 0."]
    #[inline(always)]
    pub fn out_auto_wrback(&self) -> OUT_AUTO_WRBACK_R {
        OUT_AUTO_WRBACK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - out eof flag generation mode . 1: when dma pop all data from fifo 0:when ahb push all data to fifo."]
    #[inline(always)]
    pub fn out_eof_mode(&self) -> OUT_EOF_MODE_R {
        OUT_EOF_MODE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - read descriptor use burst mode when read data for memory."]
    #[inline(always)]
    pub fn outdscr_burst_en(&self) -> OUTDSCR_BURST_EN_R {
        OUTDSCR_BURST_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - read descriptor use burst mode when write data to memory."]
    #[inline(always)]
    pub fn indscr_burst_en(&self) -> INDSCR_BURST_EN_R {
        INDSCR_BURST_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - spi dma read data from memory in burst mode."]
    #[inline(always)]
    pub fn out_data_burst_en(&self) -> OUT_DATA_BURST_EN_R {
        OUT_DATA_BURST_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 1: Internal memory data transfer enable bit. Send SPI DMA RX buffer data to SPI DMA TX buffer. 0: Disable this function."]
    #[inline(always)]
    pub fn mem_trans_en(&self) -> MEM_TRANS_EN_R {
        MEM_TRANS_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - spi dma read data stop when in continue tx/rx mode."]
    #[inline(always)]
    pub fn dma_rx_stop(&self) -> DMA_RX_STOP_R {
        DMA_RX_STOP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - spi dma write data stop when in continue tx/rx mode."]
    #[inline(always)]
    pub fn dma_tx_stop(&self) -> DMA_TX_STOP_R {
        DMA_TX_STOP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - spi dma continue tx/rx data."]
    #[inline(always)]
    pub fn dma_continue(&self) -> DMA_CONTINUE_R {
        DMA_CONTINUE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 1: Clear spi_slv_seg_frt_pop_mask. 0 : others"]
    #[inline(always)]
    pub fn slv_last_seg_pop_clr(&self) -> SLV_LAST_SEG_POP_CLR_R {
        SLV_LAST_SEG_POP_CLR_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable dma segment transfer in spi dma half slave mode. 1: enable. 0: disable."]
    #[inline(always)]
    pub fn dma_slv_seg_trans_en(&self) -> DMA_SLV_SEG_TRANS_EN_R {
        DMA_SLV_SEG_TRANS_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 1: spi_dma_infifo_full_vld is cleared by spi slave CMD5. 0: spi_dma_infifo_full_vld is cleared by SPI_TRANS_DONE."]
    #[inline(always)]
    pub fn slv_rx_seg_trans_clr_en(&self) -> SLV_RX_SEG_TRANS_CLR_EN_R {
        SLV_RX_SEG_TRANS_CLR_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 1: spi_dma_outfifo_empty_vld is cleared by spi slave CMD6. 0: spi_dma_outfifo_empty_vld is cleared by SPI_TRANS_DONE."]
    #[inline(always)]
    pub fn slv_tx_seg_trans_clr_en(&self) -> SLV_TX_SEG_TRANS_CLR_EN_R {
        SLV_TX_SEG_TRANS_CLR_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 1: SPI_IN_SUC_EOF_INT_RAW is set when the number of dma pushed data bytes is equal to the value of SPI_SLV_DMA_RD_BYTELEN\\[19:0\\]
/ SPI_MST_DMA_RD_BYTELEN\\[19:0\\]
 in spi dma transition. 0: SPI_IN_SUC_EOF_INT_RAW is set by SPI_TRANS_DONE in non-seg-trans or SPI_DMA_SEG_TRANS_DONE in seg-trans."]
    #[inline(always)]
    pub fn rx_eof_en(&self) -> RX_EOF_EN_R {
        RX_EOF_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 1:Clear spi_dma_infifo_full_vld. 0: Do not control it."]
    #[inline(always)]
    pub fn dma_infifo_full_clr(&self) -> DMA_INFIFO_FULL_CLR_R {
        DMA_INFIFO_FULL_CLR_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 1:Clear spi_dma_outfifo_empty_vld. 0: Do not control it."]
    #[inline(always)]
    pub fn dma_outfifo_empty_clr(&self) -> DMA_OUTFIFO_EMPTY_CLR_R {
        DMA_OUTFIFO_EMPTY_CLR_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 26:27 - Select the external memory block size."]
    #[inline(always)]
    pub fn ext_mem_bk_size(&self) -> EXT_MEM_BK_SIZE_R {
        EXT_MEM_BK_SIZE_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 28 - 1: End slave seg-trans, which acts as 0x05 command. 2 or more end seg-trans signals will induce error in DMA RX. 0: others. Will be cleared in 1 APB CLK cycles by hardware.."]
    #[inline(always)]
    pub fn dma_seg_trans_clr(&self) -> DMA_SEG_TRANS_CLR_R {
        DMA_SEG_TRANS_CLR_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - The bit is used to reset in dma fsm and in data fifo pointer."]
    #[inline(always)]
    pub fn in_rst(&mut self) -> IN_RST_W {
        IN_RST_W { w: self }
    }
    #[doc = "Bit 3 - The bit is used to reset out dma fsm and out data fifo pointer."]
    #[inline(always)]
    pub fn out_rst(&mut self) -> OUT_RST_W {
        OUT_RST_W { w: self }
    }
    #[doc = "Bit 4 - Reset spi dma ahb master fifo pointer."]
    #[inline(always)]
    pub fn ahbm_fifo_rst(&mut self) -> AHBM_FIFO_RST_W {
        AHBM_FIFO_RST_W { w: self }
    }
    #[doc = "Bit 5 - Reset spi dma ahb master."]
    #[inline(always)]
    pub fn ahbm_rst(&mut self) -> AHBM_RST_W {
        AHBM_RST_W { w: self }
    }
    #[doc = "Bit 6 - Set bit to test in link."]
    #[inline(always)]
    pub fn in_loop_test(&mut self) -> IN_LOOP_TEST_W {
        IN_LOOP_TEST_W { w: self }
    }
    #[doc = "Bit 7 - Set bit to test out link."]
    #[inline(always)]
    pub fn out_loop_test(&mut self) -> OUT_LOOP_TEST_W {
        OUT_LOOP_TEST_W { w: self }
    }
    #[doc = "Bit 8 - when the bit is set, DMA continue to use the next inlink node when the length of inlink is 0."]
    #[inline(always)]
    pub fn out_auto_wrback(&mut self) -> OUT_AUTO_WRBACK_W {
        OUT_AUTO_WRBACK_W { w: self }
    }
    #[doc = "Bit 9 - out eof flag generation mode . 1: when dma pop all data from fifo 0:when ahb push all data to fifo."]
    #[inline(always)]
    pub fn out_eof_mode(&mut self) -> OUT_EOF_MODE_W {
        OUT_EOF_MODE_W { w: self }
    }
    #[doc = "Bit 10 - read descriptor use burst mode when read data for memory."]
    #[inline(always)]
    pub fn outdscr_burst_en(&mut self) -> OUTDSCR_BURST_EN_W {
        OUTDSCR_BURST_EN_W { w: self }
    }
    #[doc = "Bit 11 - read descriptor use burst mode when write data to memory."]
    #[inline(always)]
    pub fn indscr_burst_en(&mut self) -> INDSCR_BURST_EN_W {
        INDSCR_BURST_EN_W { w: self }
    }
    #[doc = "Bit 12 - spi dma read data from memory in burst mode."]
    #[inline(always)]
    pub fn out_data_burst_en(&mut self) -> OUT_DATA_BURST_EN_W {
        OUT_DATA_BURST_EN_W { w: self }
    }
    #[doc = "Bit 13 - 1: Internal memory data transfer enable bit. Send SPI DMA RX buffer data to SPI DMA TX buffer. 0: Disable this function."]
    #[inline(always)]
    pub fn mem_trans_en(&mut self) -> MEM_TRANS_EN_W {
        MEM_TRANS_EN_W { w: self }
    }
    #[doc = "Bit 14 - spi dma read data stop when in continue tx/rx mode."]
    #[inline(always)]
    pub fn dma_rx_stop(&mut self) -> DMA_RX_STOP_W {
        DMA_RX_STOP_W { w: self }
    }
    #[doc = "Bit 15 - spi dma write data stop when in continue tx/rx mode."]
    #[inline(always)]
    pub fn dma_tx_stop(&mut self) -> DMA_TX_STOP_W {
        DMA_TX_STOP_W { w: self }
    }
    #[doc = "Bit 16 - spi dma continue tx/rx data."]
    #[inline(always)]
    pub fn dma_continue(&mut self) -> DMA_CONTINUE_W {
        DMA_CONTINUE_W { w: self }
    }
    #[doc = "Bit 17 - 1: Clear spi_slv_seg_frt_pop_mask. 0 : others"]
    #[inline(always)]
    pub fn slv_last_seg_pop_clr(&mut self) -> SLV_LAST_SEG_POP_CLR_W {
        SLV_LAST_SEG_POP_CLR_W { w: self }
    }
    #[doc = "Bit 18 - Enable dma segment transfer in spi dma half slave mode. 1: enable. 0: disable."]
    #[inline(always)]
    pub fn dma_slv_seg_trans_en(&mut self) -> DMA_SLV_SEG_TRANS_EN_W {
        DMA_SLV_SEG_TRANS_EN_W { w: self }
    }
    #[doc = "Bit 19 - 1: spi_dma_infifo_full_vld is cleared by spi slave CMD5. 0: spi_dma_infifo_full_vld is cleared by SPI_TRANS_DONE."]
    #[inline(always)]
    pub fn slv_rx_seg_trans_clr_en(&mut self) -> SLV_RX_SEG_TRANS_CLR_EN_W {
        SLV_RX_SEG_TRANS_CLR_EN_W { w: self }
    }
    #[doc = "Bit 20 - 1: spi_dma_outfifo_empty_vld is cleared by spi slave CMD6. 0: spi_dma_outfifo_empty_vld is cleared by SPI_TRANS_DONE."]
    #[inline(always)]
    pub fn slv_tx_seg_trans_clr_en(&mut self) -> SLV_TX_SEG_TRANS_CLR_EN_W {
        SLV_TX_SEG_TRANS_CLR_EN_W { w: self }
    }
    #[doc = "Bit 21 - 1: SPI_IN_SUC_EOF_INT_RAW is set when the number of dma pushed data bytes is equal to the value of SPI_SLV_DMA_RD_BYTELEN\\[19:0\\]
/ SPI_MST_DMA_RD_BYTELEN\\[19:0\\]
 in spi dma transition. 0: SPI_IN_SUC_EOF_INT_RAW is set by SPI_TRANS_DONE in non-seg-trans or SPI_DMA_SEG_TRANS_DONE in seg-trans."]
    #[inline(always)]
    pub fn rx_eof_en(&mut self) -> RX_EOF_EN_W {
        RX_EOF_EN_W { w: self }
    }
    #[doc = "Bit 22 - 1:Clear spi_dma_infifo_full_vld. 0: Do not control it."]
    #[inline(always)]
    pub fn dma_infifo_full_clr(&mut self) -> DMA_INFIFO_FULL_CLR_W {
        DMA_INFIFO_FULL_CLR_W { w: self }
    }
    #[doc = "Bit 23 - 1:Clear spi_dma_outfifo_empty_vld. 0: Do not control it."]
    #[inline(always)]
    pub fn dma_outfifo_empty_clr(&mut self) -> DMA_OUTFIFO_EMPTY_CLR_W {
        DMA_OUTFIFO_EMPTY_CLR_W { w: self }
    }
    #[doc = "Bits 26:27 - Select the external memory block size."]
    #[inline(always)]
    pub fn ext_mem_bk_size(&mut self) -> EXT_MEM_BK_SIZE_W {
        EXT_MEM_BK_SIZE_W { w: self }
    }
    #[doc = "Bit 28 - 1: End slave seg-trans, which acts as 0x05 command. 2 or more end seg-trans signals will induce error in DMA RX. 0: others. Will be cleared in 1 APB CLK cycles by hardware.."]
    #[inline(always)]
    pub fn dma_seg_trans_clr(&mut self) -> DMA_SEG_TRANS_CLR_W {
        DMA_SEG_TRANS_CLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI DMA control register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_conf]
(index.html) module"]
pub struct DMA_CONF_SPEC;
impl crate::RegisterSpec for DMA_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_conf::R]
(R) reader structure"]
impl crate::Readable for DMA_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_conf::W]
(W) writer structure"]
impl crate::Writable for DMA_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_CONF to value 0x0200"]
impl crate::Resettable for DMA_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200
    }
}
