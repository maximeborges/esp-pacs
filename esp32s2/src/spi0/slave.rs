#[doc = "Register `SLAVE` reader"]
pub struct R(crate::R<SLAVE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLAVE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLAVE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLAVE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLAVE` writer"]
pub struct W(crate::W<SLAVE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLAVE_SPEC>;
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
impl From<crate::W<SLAVE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLAVE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRANS_DONE` reader - The interrupt raw bit for the completion of any operation in both the master mode and the slave mode. Can not be changed by CONF_buf."]
pub struct TRANS_DONE_R(crate::FieldReader<bool, bool>);
impl TRANS_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRANS_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRANS_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRANS_DONE` writer - The interrupt raw bit for the completion of any operation in both the master mode and the slave mode. Can not be changed by CONF_buf."]
pub struct TRANS_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> TRANS_DONE_W<'a> {
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
#[doc = "Field `INT_RD_BUF_DONE_EN` reader - SPI_SLV_RD_BUF_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state."]
pub struct INT_RD_BUF_DONE_EN_R(crate::FieldReader<bool, bool>);
impl INT_RD_BUF_DONE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INT_RD_BUF_DONE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT_RD_BUF_DONE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT_RD_BUF_DONE_EN` writer - SPI_SLV_RD_BUF_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state."]
pub struct INT_RD_BUF_DONE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_RD_BUF_DONE_EN_W<'a> {
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
#[doc = "Field `INT_WR_BUF_DONE_EN` reader - SPI_SLV_WR_BUF_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state."]
pub struct INT_WR_BUF_DONE_EN_R(crate::FieldReader<bool, bool>);
impl INT_WR_BUF_DONE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INT_WR_BUF_DONE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT_WR_BUF_DONE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT_WR_BUF_DONE_EN` writer - SPI_SLV_WR_BUF_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state."]
pub struct INT_WR_BUF_DONE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_WR_BUF_DONE_EN_W<'a> {
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
#[doc = "Field `INT_RD_DMA_DONE_EN` reader - SPI_SLV_RD_DMA_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state."]
pub struct INT_RD_DMA_DONE_EN_R(crate::FieldReader<bool, bool>);
impl INT_RD_DMA_DONE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INT_RD_DMA_DONE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT_RD_DMA_DONE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT_RD_DMA_DONE_EN` writer - SPI_SLV_RD_DMA_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state."]
pub struct INT_RD_DMA_DONE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_RD_DMA_DONE_EN_W<'a> {
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
#[doc = "Field `INT_WR_DMA_DONE_EN` reader - SPI_SLV_WR_DMA_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state."]
pub struct INT_WR_DMA_DONE_EN_R(crate::FieldReader<bool, bool>);
impl INT_WR_DMA_DONE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INT_WR_DMA_DONE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT_WR_DMA_DONE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT_WR_DMA_DONE_EN` writer - SPI_SLV_WR_DMA_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state."]
pub struct INT_WR_DMA_DONE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_WR_DMA_DONE_EN_W<'a> {
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
#[doc = "Field `INT_TRANS_DONE_EN` reader - SPI_TRANS_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state."]
pub struct INT_TRANS_DONE_EN_R(crate::FieldReader<bool, bool>);
impl INT_TRANS_DONE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INT_TRANS_DONE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT_TRANS_DONE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT_TRANS_DONE_EN` writer - SPI_TRANS_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state."]
pub struct INT_TRANS_DONE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_TRANS_DONE_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `INT_DMA_SEG_TRANS_EN` reader - SPI_DMA_SEG_TRANS_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state."]
pub struct INT_DMA_SEG_TRANS_EN_R(crate::FieldReader<bool, bool>);
impl INT_DMA_SEG_TRANS_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INT_DMA_SEG_TRANS_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT_DMA_SEG_TRANS_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT_DMA_SEG_TRANS_EN` writer - SPI_DMA_SEG_TRANS_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state."]
pub struct INT_DMA_SEG_TRANS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_DMA_SEG_TRANS_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `SEG_MAGIC_ERR_INT_EN` reader - 1: Enable seg magic value error interrupt. 0: Others. Can be configured in CONF state."]
pub struct SEG_MAGIC_ERR_INT_EN_R(crate::FieldReader<bool, bool>);
impl SEG_MAGIC_ERR_INT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SEG_MAGIC_ERR_INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEG_MAGIC_ERR_INT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEG_MAGIC_ERR_INT_EN` writer - 1: Enable seg magic value error interrupt. 0: Others. Can be configured in CONF state."]
pub struct SEG_MAGIC_ERR_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SEG_MAGIC_ERR_INT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `TRANS_CNT` reader - The operations counter in both the master mode and the slave mode."]
pub struct TRANS_CNT_R(crate::FieldReader<u8, u8>);
impl TRANS_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TRANS_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRANS_CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRANS_DONE_AUTO_CLR_EN` reader - SPI_TRANS_DONE auto clear enable, clear it 3 apb cycles after the pos edge of SPI_TRANS_DONE. 0:disable. 1: enable. Can be configured in CONF state."]
pub struct TRANS_DONE_AUTO_CLR_EN_R(crate::FieldReader<bool, bool>);
impl TRANS_DONE_AUTO_CLR_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRANS_DONE_AUTO_CLR_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRANS_DONE_AUTO_CLR_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRANS_DONE_AUTO_CLR_EN` writer - SPI_TRANS_DONE auto clear enable, clear it 3 apb cycles after the pos edge of SPI_TRANS_DONE. 0:disable. 1: enable. Can be configured in CONF state."]
pub struct TRANS_DONE_AUTO_CLR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TRANS_DONE_AUTO_CLR_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `MODE` reader - Set SPI work mode. 1: slave mode 0: master mode."]
pub struct MODE_R(crate::FieldReader<bool, bool>);
impl MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE` writer - Set SPI work mode. 1: slave mode 0: master mode."]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `SOFT_RESET` reader - Software reset enable, reset the spi clock line cs line and data lines. Can be configured in CONF state."]
pub struct SOFT_RESET_R(crate::FieldReader<bool, bool>);
impl SOFT_RESET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SOFT_RESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFT_RESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFT_RESET` writer - Software reset enable, reset the spi clock line cs line and data lines. Can be configured in CONF state."]
pub struct SOFT_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFT_RESET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 4 - The interrupt raw bit for the completion of any operation in both the master mode and the slave mode. Can not be changed by CONF_buf."]
    #[inline(always)]
    pub fn trans_done(&self) -> TRANS_DONE_R {
        TRANS_DONE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SPI_SLV_RD_BUF_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn int_rd_buf_done_en(&self) -> INT_RD_BUF_DONE_EN_R {
        INT_RD_BUF_DONE_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SPI_SLV_WR_BUF_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn int_wr_buf_done_en(&self) -> INT_WR_BUF_DONE_EN_R {
        INT_WR_BUF_DONE_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - SPI_SLV_RD_DMA_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn int_rd_dma_done_en(&self) -> INT_RD_DMA_DONE_EN_R {
        INT_RD_DMA_DONE_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SPI_SLV_WR_DMA_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn int_wr_dma_done_en(&self) -> INT_WR_DMA_DONE_EN_R {
        INT_WR_DMA_DONE_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SPI_TRANS_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn int_trans_done_en(&self) -> INT_TRANS_DONE_EN_R {
        INT_TRANS_DONE_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - SPI_DMA_SEG_TRANS_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn int_dma_seg_trans_en(&self) -> INT_DMA_SEG_TRANS_EN_R {
        INT_DMA_SEG_TRANS_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 1: Enable seg magic value error interrupt. 0: Others. Can be configured in CONF state."]
    #[inline(always)]
    pub fn seg_magic_err_int_en(&self) -> SEG_MAGIC_ERR_INT_EN_R {
        SEG_MAGIC_ERR_INT_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 23:26 - The operations counter in both the master mode and the slave mode."]
    #[inline(always)]
    pub fn trans_cnt(&self) -> TRANS_CNT_R {
        TRANS_CNT_R::new(((self.bits >> 23) & 0x0f) as u8)
    }
    #[doc = "Bit 29 - SPI_TRANS_DONE auto clear enable, clear it 3 apb cycles after the pos edge of SPI_TRANS_DONE. 0:disable. 1: enable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn trans_done_auto_clr_en(&self) -> TRANS_DONE_AUTO_CLR_EN_R {
        TRANS_DONE_AUTO_CLR_EN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Set SPI work mode. 1: slave mode 0: master mode."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Software reset enable, reset the spi clock line cs line and data lines. Can be configured in CONF state."]
    #[inline(always)]
    pub fn soft_reset(&self) -> SOFT_RESET_R {
        SOFT_RESET_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - The interrupt raw bit for the completion of any operation in both the master mode and the slave mode. Can not be changed by CONF_buf."]
    #[inline(always)]
    pub fn trans_done(&mut self) -> TRANS_DONE_W {
        TRANS_DONE_W { w: self }
    }
    #[doc = "Bit 5 - SPI_SLV_RD_BUF_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn int_rd_buf_done_en(&mut self) -> INT_RD_BUF_DONE_EN_W {
        INT_RD_BUF_DONE_EN_W { w: self }
    }
    #[doc = "Bit 6 - SPI_SLV_WR_BUF_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn int_wr_buf_done_en(&mut self) -> INT_WR_BUF_DONE_EN_W {
        INT_WR_BUF_DONE_EN_W { w: self }
    }
    #[doc = "Bit 7 - SPI_SLV_RD_DMA_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn int_rd_dma_done_en(&mut self) -> INT_RD_DMA_DONE_EN_W {
        INT_RD_DMA_DONE_EN_W { w: self }
    }
    #[doc = "Bit 8 - SPI_SLV_WR_DMA_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn int_wr_dma_done_en(&mut self) -> INT_WR_DMA_DONE_EN_W {
        INT_WR_DMA_DONE_EN_W { w: self }
    }
    #[doc = "Bit 9 - SPI_TRANS_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn int_trans_done_en(&mut self) -> INT_TRANS_DONE_EN_W {
        INT_TRANS_DONE_EN_W { w: self }
    }
    #[doc = "Bit 10 - SPI_DMA_SEG_TRANS_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn int_dma_seg_trans_en(&mut self) -> INT_DMA_SEG_TRANS_EN_W {
        INT_DMA_SEG_TRANS_EN_W { w: self }
    }
    #[doc = "Bit 11 - 1: Enable seg magic value error interrupt. 0: Others. Can be configured in CONF state."]
    #[inline(always)]
    pub fn seg_magic_err_int_en(&mut self) -> SEG_MAGIC_ERR_INT_EN_W {
        SEG_MAGIC_ERR_INT_EN_W { w: self }
    }
    #[doc = "Bit 29 - SPI_TRANS_DONE auto clear enable, clear it 3 apb cycles after the pos edge of SPI_TRANS_DONE. 0:disable. 1: enable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn trans_done_auto_clr_en(&mut self) -> TRANS_DONE_AUTO_CLR_EN_W {
        TRANS_DONE_AUTO_CLR_EN_W { w: self }
    }
    #[doc = "Bit 30 - Set SPI work mode. 1: slave mode 0: master mode."]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 31 - Software reset enable, reset the spi clock line cs line and data lines. Can be configured in CONF state."]
    #[inline(always)]
    pub fn soft_reset(&mut self) -> SOFT_RESET_W {
        SOFT_RESET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI slave control register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slave]
(index.html) module"]
pub struct SLAVE_SPEC;
impl crate::RegisterSpec for SLAVE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slave::R]
(R) reader structure"]
impl crate::Readable for SLAVE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slave::W]
(W) writer structure"]
impl crate::Writable for SLAVE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SLAVE to value 0x0200"]
impl crate::Resettable for SLAVE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200
    }
}
