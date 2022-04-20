#[doc = "Register `SLV_RD_BYTE` reader"]
pub struct R(crate::R<SLV_RD_BYTE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLV_RD_BYTE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLV_RD_BYTE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLV_RD_BYTE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLV_RD_BYTE` writer"]
pub struct W(crate::W<SLV_RD_BYTE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLV_RD_BYTE_SPEC>;
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
impl From<crate::W<SLV_RD_BYTE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLV_RD_BYTE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLV_DATA_BYTELEN` reader - The full-duplex or half-duplex data byte length of the last SPI transfer in slave mode. In half-duplex mode, this value is controlled by bits \\[23:20\\]
."]
pub struct SLV_DATA_BYTELEN_R(crate::FieldReader<u32, u32>);
impl SLV_DATA_BYTELEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        SLV_DATA_BYTELEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_DATA_BYTELEN_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_DATA_BYTELEN` writer - The full-duplex or half-duplex data byte length of the last SPI transfer in slave mode. In half-duplex mode, this value is controlled by bits \\[23:20\\]
."]
pub struct SLV_DATA_BYTELEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_DATA_BYTELEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | (value as u32 & 0x000f_ffff);
        self.w
    }
}
#[doc = "Field `SLV_RDDMA_BYTELEN_EN` reader - 1: SPI_SLV_DATA_BYTELEN stores data byte length of master-read-slave data length in DMA controlled mode(Rd_DMA). 0: others"]
pub struct SLV_RDDMA_BYTELEN_EN_R(crate::FieldReader<bool, bool>);
impl SLV_RDDMA_BYTELEN_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLV_RDDMA_BYTELEN_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_RDDMA_BYTELEN_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_RDDMA_BYTELEN_EN` writer - 1: SPI_SLV_DATA_BYTELEN stores data byte length of master-read-slave data length in DMA controlled mode(Rd_DMA). 0: others"]
pub struct SLV_RDDMA_BYTELEN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_RDDMA_BYTELEN_EN_W<'a> {
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
#[doc = "Field `SLV_WRDMA_BYTELEN_EN` reader - 1: SPI_SLV_DATA_BYTELEN stores data byte length of master-write-to-slave data length in DMA controlled mode(Wr_DMA). 0: others"]
pub struct SLV_WRDMA_BYTELEN_EN_R(crate::FieldReader<bool, bool>);
impl SLV_WRDMA_BYTELEN_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLV_WRDMA_BYTELEN_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_WRDMA_BYTELEN_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_WRDMA_BYTELEN_EN` writer - 1: SPI_SLV_DATA_BYTELEN stores data byte length of master-write-to-slave data length in DMA controlled mode(Wr_DMA). 0: others"]
pub struct SLV_WRDMA_BYTELEN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_WRDMA_BYTELEN_EN_W<'a> {
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
#[doc = "Field `SLV_RDBUF_BYTELEN_EN` reader - 1: SPI_SLV_DATA_BYTELEN stores data byte length of master-read-slave data length in CPU controlled mode(Rd_BUF). 0: others"]
pub struct SLV_RDBUF_BYTELEN_EN_R(crate::FieldReader<bool, bool>);
impl SLV_RDBUF_BYTELEN_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLV_RDBUF_BYTELEN_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_RDBUF_BYTELEN_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_RDBUF_BYTELEN_EN` writer - 1: SPI_SLV_DATA_BYTELEN stores data byte length of master-read-slave data length in CPU controlled mode(Rd_BUF). 0: others"]
pub struct SLV_RDBUF_BYTELEN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_RDBUF_BYTELEN_EN_W<'a> {
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
#[doc = "Field `SLV_WRBUF_BYTELEN_EN` reader - 1: SPI_SLV_DATA_BYTELEN stores data byte length of master-write-to-slave data length in CPU controlled mode(Wr_BUF). 0: others"]
pub struct SLV_WRBUF_BYTELEN_EN_R(crate::FieldReader<bool, bool>);
impl SLV_WRBUF_BYTELEN_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLV_WRBUF_BYTELEN_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_WRBUF_BYTELEN_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_WRBUF_BYTELEN_EN` writer - 1: SPI_SLV_DATA_BYTELEN stores data byte length of master-write-to-slave data length in CPU controlled mode(Wr_BUF). 0: others"]
pub struct SLV_WRBUF_BYTELEN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_WRBUF_BYTELEN_EN_W<'a> {
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
#[doc = "Field `DMA_SEG_MAGIC_VALUE` reader - The magic value of BM table in master DMA seg-trans."]
pub struct DMA_SEG_MAGIC_VALUE_R(crate::FieldReader<u8, u8>);
impl DMA_SEG_MAGIC_VALUE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DMA_SEG_MAGIC_VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_SEG_MAGIC_VALUE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_SEG_MAGIC_VALUE` writer - The magic value of BM table in master DMA seg-trans."]
pub struct DMA_SEG_MAGIC_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_SEG_MAGIC_VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `SLV_RD_DMA_DONE` reader - The interrupt raw bit for the completion of Rd-DMA operation in the slave mode. Can not be changed by CONF_buf."]
pub struct SLV_RD_DMA_DONE_R(crate::FieldReader<bool, bool>);
impl SLV_RD_DMA_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLV_RD_DMA_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_RD_DMA_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_RD_DMA_DONE` writer - The interrupt raw bit for the completion of Rd-DMA operation in the slave mode. Can not be changed by CONF_buf."]
pub struct SLV_RD_DMA_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_RD_DMA_DONE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 30)) | ((value as u32 & 1) << 30);
        self.w
    }
}
#[doc = "Field `USR_CONF` reader - 1: Enable the DMA CONF phase of current seg-trans operation, which means seg-trans will start. 0: This is not seg-trans mode."]
pub struct USR_CONF_R(crate::FieldReader<bool, bool>);
impl USR_CONF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USR_CONF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USR_CONF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USR_CONF` writer - 1: Enable the DMA CONF phase of current seg-trans operation, which means seg-trans will start. 0: This is not seg-trans mode."]
pub struct USR_CONF_W<'a> {
    w: &'a mut W,
}
impl<'a> USR_CONF_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 31)) | ((value as u32 & 1) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19 - The full-duplex or half-duplex data byte length of the last SPI transfer in slave mode. In half-duplex mode, this value is controlled by bits \\[23:20\\]
."]
    #[inline(always)]
    pub fn slv_data_bytelen(&self) -> SLV_DATA_BYTELEN_R {
        SLV_DATA_BYTELEN_R::new((self.bits & 0x000f_ffff) as u32)
    }
    #[doc = "Bit 20 - 1: SPI_SLV_DATA_BYTELEN stores data byte length of master-read-slave data length in DMA controlled mode(Rd_DMA). 0: others"]
    #[inline(always)]
    pub fn slv_rddma_bytelen_en(&self) -> SLV_RDDMA_BYTELEN_EN_R {
        SLV_RDDMA_BYTELEN_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 1: SPI_SLV_DATA_BYTELEN stores data byte length of master-write-to-slave data length in DMA controlled mode(Wr_DMA). 0: others"]
    #[inline(always)]
    pub fn slv_wrdma_bytelen_en(&self) -> SLV_WRDMA_BYTELEN_EN_R {
        SLV_WRDMA_BYTELEN_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 1: SPI_SLV_DATA_BYTELEN stores data byte length of master-read-slave data length in CPU controlled mode(Rd_BUF). 0: others"]
    #[inline(always)]
    pub fn slv_rdbuf_bytelen_en(&self) -> SLV_RDBUF_BYTELEN_EN_R {
        SLV_RDBUF_BYTELEN_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 1: SPI_SLV_DATA_BYTELEN stores data byte length of master-write-to-slave data length in CPU controlled mode(Wr_BUF). 0: others"]
    #[inline(always)]
    pub fn slv_wrbuf_bytelen_en(&self) -> SLV_WRBUF_BYTELEN_EN_R {
        SLV_WRBUF_BYTELEN_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27 - The magic value of BM table in master DMA seg-trans."]
    #[inline(always)]
    pub fn dma_seg_magic_value(&self) -> DMA_SEG_MAGIC_VALUE_R {
        DMA_SEG_MAGIC_VALUE_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - The interrupt raw bit for the completion of Rd-DMA operation in the slave mode. Can not be changed by CONF_buf."]
    #[inline(always)]
    pub fn slv_rd_dma_done(&self) -> SLV_RD_DMA_DONE_R {
        SLV_RD_DMA_DONE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 1: Enable the DMA CONF phase of current seg-trans operation, which means seg-trans will start. 0: This is not seg-trans mode."]
    #[inline(always)]
    pub fn usr_conf(&self) -> USR_CONF_R {
        USR_CONF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:19 - The full-duplex or half-duplex data byte length of the last SPI transfer in slave mode. In half-duplex mode, this value is controlled by bits \\[23:20\\]
."]
    #[inline(always)]
    pub fn slv_data_bytelen(&mut self) -> SLV_DATA_BYTELEN_W {
        SLV_DATA_BYTELEN_W { w: self }
    }
    #[doc = "Bit 20 - 1: SPI_SLV_DATA_BYTELEN stores data byte length of master-read-slave data length in DMA controlled mode(Rd_DMA). 0: others"]
    #[inline(always)]
    pub fn slv_rddma_bytelen_en(&mut self) -> SLV_RDDMA_BYTELEN_EN_W {
        SLV_RDDMA_BYTELEN_EN_W { w: self }
    }
    #[doc = "Bit 21 - 1: SPI_SLV_DATA_BYTELEN stores data byte length of master-write-to-slave data length in DMA controlled mode(Wr_DMA). 0: others"]
    #[inline(always)]
    pub fn slv_wrdma_bytelen_en(&mut self) -> SLV_WRDMA_BYTELEN_EN_W {
        SLV_WRDMA_BYTELEN_EN_W { w: self }
    }
    #[doc = "Bit 22 - 1: SPI_SLV_DATA_BYTELEN stores data byte length of master-read-slave data length in CPU controlled mode(Rd_BUF). 0: others"]
    #[inline(always)]
    pub fn slv_rdbuf_bytelen_en(&mut self) -> SLV_RDBUF_BYTELEN_EN_W {
        SLV_RDBUF_BYTELEN_EN_W { w: self }
    }
    #[doc = "Bit 23 - 1: SPI_SLV_DATA_BYTELEN stores data byte length of master-write-to-slave data length in CPU controlled mode(Wr_BUF). 0: others"]
    #[inline(always)]
    pub fn slv_wrbuf_bytelen_en(&mut self) -> SLV_WRBUF_BYTELEN_EN_W {
        SLV_WRBUF_BYTELEN_EN_W { w: self }
    }
    #[doc = "Bits 24:27 - The magic value of BM table in master DMA seg-trans."]
    #[inline(always)]
    pub fn dma_seg_magic_value(&mut self) -> DMA_SEG_MAGIC_VALUE_W {
        DMA_SEG_MAGIC_VALUE_W { w: self }
    }
    #[doc = "Bit 30 - The interrupt raw bit for the completion of Rd-DMA operation in the slave mode. Can not be changed by CONF_buf."]
    #[inline(always)]
    pub fn slv_rd_dma_done(&mut self) -> SLV_RD_DMA_DONE_W {
        SLV_RD_DMA_DONE_W { w: self }
    }
    #[doc = "Bit 31 - 1: Enable the DMA CONF phase of current seg-trans operation, which means seg-trans will start. 0: This is not seg-trans mode."]
    #[inline(always)]
    pub fn usr_conf(&mut self) -> USR_CONF_W {
        USR_CONF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI interrupt control register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slv_rd_byte]
(index.html) module"]
pub struct SLV_RD_BYTE_SPEC;
impl crate::RegisterSpec for SLV_RD_BYTE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slv_rd_byte::R]
(R) reader structure"]
impl crate::Readable for SLV_RD_BYTE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slv_rd_byte::W]
(W) writer structure"]
impl crate::Writable for SLV_RD_BYTE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SLV_RD_BYTE to value 0x0a00_0000"]
impl crate::Resettable for SLV_RD_BYTE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0a00_0000
    }
}
