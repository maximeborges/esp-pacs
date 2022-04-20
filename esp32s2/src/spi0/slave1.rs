#[doc = "Register `SLAVE1` reader"]
pub struct R(crate::R<SLAVE1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLAVE1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLAVE1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLAVE1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLAVE1` writer"]
pub struct W(crate::W<SLAVE1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLAVE1_SPEC>;
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
impl From<crate::W<SLAVE1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLAVE1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLV_ADDR_ERR_CLR` reader - 1: Clear SPI_SLV_ADDR_ERR. 0: not valid. Can be changed by CONF_buf."]
pub struct SLV_ADDR_ERR_CLR_R(crate::FieldReader<bool, bool>);
impl SLV_ADDR_ERR_CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLV_ADDR_ERR_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_ADDR_ERR_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_ADDR_ERR_CLR` writer - 1: Clear SPI_SLV_ADDR_ERR. 0: not valid. Can be changed by CONF_buf."]
pub struct SLV_ADDR_ERR_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_ADDR_ERR_CLR_W<'a> {
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
#[doc = "Field `SLV_CMD_ERR_CLR` reader - 1: Clear SPI_SLV_CMD_ERR. 0: not valid. Can be changed by CONF_buf."]
pub struct SLV_CMD_ERR_CLR_R(crate::FieldReader<bool, bool>);
impl SLV_CMD_ERR_CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLV_CMD_ERR_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_CMD_ERR_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_CMD_ERR_CLR` writer - 1: Clear SPI_SLV_CMD_ERR. 0: not valid. Can be changed by CONF_buf."]
pub struct SLV_CMD_ERR_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_CMD_ERR_CLR_W<'a> {
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
#[doc = "Field `SLV_NO_QPI_EN` reader - 1: spi slave QPI mode is not supported. 0: spi slave QPI mode is supported."]
pub struct SLV_NO_QPI_EN_R(crate::FieldReader<bool, bool>);
impl SLV_NO_QPI_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLV_NO_QPI_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_NO_QPI_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_NO_QPI_EN` writer - 1: spi slave QPI mode is not supported. 0: spi slave QPI mode is supported."]
pub struct SLV_NO_QPI_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_NO_QPI_EN_W<'a> {
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
#[doc = "Field `SLV_ADDR_ERR` reader - 1: The address value of the last SPI transfer is not supported by SPI slave. 0: The address value is supported or no address value is received."]
pub struct SLV_ADDR_ERR_R(crate::FieldReader<bool, bool>);
impl SLV_ADDR_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLV_ADDR_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_ADDR_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_CMD_ERR` reader - 1: The command value of the last SPI transfer is not supported by SPI slave. 0: The command value is supported or no command value is received."]
pub struct SLV_CMD_ERR_R(crate::FieldReader<bool, bool>);
impl SLV_CMD_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLV_CMD_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_CMD_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_WR_DMA_DONE` reader - The interrupt raw bit for the completion of dma write operation in the slave mode. Can not be changed by CONF_buf."]
pub struct SLV_WR_DMA_DONE_R(crate::FieldReader<bool, bool>);
impl SLV_WR_DMA_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLV_WR_DMA_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_WR_DMA_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_WR_DMA_DONE` writer - The interrupt raw bit for the completion of dma write operation in the slave mode. Can not be changed by CONF_buf."]
pub struct SLV_WR_DMA_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_WR_DMA_DONE_W<'a> {
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
#[doc = "Field `SLV_LAST_COMMAND` reader - In the slave mode it is the value of command."]
pub struct SLV_LAST_COMMAND_R(crate::FieldReader<u8, u8>);
impl SLV_LAST_COMMAND_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SLV_LAST_COMMAND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_LAST_COMMAND_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_LAST_COMMAND` writer - In the slave mode it is the value of command."]
pub struct SLV_LAST_COMMAND_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_LAST_COMMAND_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `SLV_LAST_ADDR` reader - In the slave mode it is the value of address."]
pub struct SLV_LAST_ADDR_R(crate::FieldReader<u8, u8>);
impl SLV_LAST_ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SLV_LAST_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_LAST_ADDR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_LAST_ADDR` writer - In the slave mode it is the value of address."]
pub struct SLV_LAST_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_LAST_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 10 - 1: Clear SPI_SLV_ADDR_ERR. 0: not valid. Can be changed by CONF_buf."]
    #[inline(always)]
    pub fn slv_addr_err_clr(&self) -> SLV_ADDR_ERR_CLR_R {
        SLV_ADDR_ERR_CLR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 1: Clear SPI_SLV_CMD_ERR. 0: not valid. Can be changed by CONF_buf."]
    #[inline(always)]
    pub fn slv_cmd_err_clr(&self) -> SLV_CMD_ERR_CLR_R {
        SLV_CMD_ERR_CLR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 1: spi slave QPI mode is not supported. 0: spi slave QPI mode is supported."]
    #[inline(always)]
    pub fn slv_no_qpi_en(&self) -> SLV_NO_QPI_EN_R {
        SLV_NO_QPI_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 1: The address value of the last SPI transfer is not supported by SPI slave. 0: The address value is supported or no address value is received."]
    #[inline(always)]
    pub fn slv_addr_err(&self) -> SLV_ADDR_ERR_R {
        SLV_ADDR_ERR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 1: The command value of the last SPI transfer is not supported by SPI slave. 0: The command value is supported or no command value is received."]
    #[inline(always)]
    pub fn slv_cmd_err(&self) -> SLV_CMD_ERR_R {
        SLV_CMD_ERR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The interrupt raw bit for the completion of dma write operation in the slave mode. Can not be changed by CONF_buf."]
    #[inline(always)]
    pub fn slv_wr_dma_done(&self) -> SLV_WR_DMA_DONE_R {
        SLV_WR_DMA_DONE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - In the slave mode it is the value of command."]
    #[inline(always)]
    pub fn slv_last_command(&self) -> SLV_LAST_COMMAND_R {
        SLV_LAST_COMMAND_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - In the slave mode it is the value of address."]
    #[inline(always)]
    pub fn slv_last_addr(&self) -> SLV_LAST_ADDR_R {
        SLV_LAST_ADDR_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 10 - 1: Clear SPI_SLV_ADDR_ERR. 0: not valid. Can be changed by CONF_buf."]
    #[inline(always)]
    pub fn slv_addr_err_clr(&mut self) -> SLV_ADDR_ERR_CLR_W {
        SLV_ADDR_ERR_CLR_W { w: self }
    }
    #[doc = "Bit 11 - 1: Clear SPI_SLV_CMD_ERR. 0: not valid. Can be changed by CONF_buf."]
    #[inline(always)]
    pub fn slv_cmd_err_clr(&mut self) -> SLV_CMD_ERR_CLR_W {
        SLV_CMD_ERR_CLR_W { w: self }
    }
    #[doc = "Bit 12 - 1: spi slave QPI mode is not supported. 0: spi slave QPI mode is supported."]
    #[inline(always)]
    pub fn slv_no_qpi_en(&mut self) -> SLV_NO_QPI_EN_W {
        SLV_NO_QPI_EN_W { w: self }
    }
    #[doc = "Bit 15 - The interrupt raw bit for the completion of dma write operation in the slave mode. Can not be changed by CONF_buf."]
    #[inline(always)]
    pub fn slv_wr_dma_done(&mut self) -> SLV_WR_DMA_DONE_W {
        SLV_WR_DMA_DONE_W { w: self }
    }
    #[doc = "Bits 16:23 - In the slave mode it is the value of command."]
    #[inline(always)]
    pub fn slv_last_command(&mut self) -> SLV_LAST_COMMAND_W {
        SLV_LAST_COMMAND_W { w: self }
    }
    #[doc = "Bits 24:31 - In the slave mode it is the value of address."]
    #[inline(always)]
    pub fn slv_last_addr(&mut self) -> SLV_LAST_ADDR_W {
        SLV_LAST_ADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI slave control register 1\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slave1]
(index.html) module"]
pub struct SLAVE1_SPEC;
impl crate::RegisterSpec for SLAVE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slave1::R]
(R) reader structure"]
impl crate::Readable for SLAVE1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slave1::W]
(W) writer structure"]
impl crate::Writable for SLAVE1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SLAVE1 to value 0"]
impl crate::Resettable for SLAVE1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
