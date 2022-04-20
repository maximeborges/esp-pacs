#[doc = "Register `SLV_RDBUF_DLEN` reader"]
pub struct R(crate::R<SLV_RDBUF_DLEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLV_RDBUF_DLEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLV_RDBUF_DLEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLV_RDBUF_DLEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLV_RDBUF_DLEN` writer"]
pub struct W(crate::W<SLV_RDBUF_DLEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLV_RDBUF_DLEN_SPEC>;
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
impl From<crate::W<SLV_RDBUF_DLEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLV_RDBUF_DLEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLV_DMA_RD_BYTELEN` reader - In the slave mode it is the length in bytes for read operations. The register value shall be byte_num."]
pub struct SLV_DMA_RD_BYTELEN_R(crate::FieldReader<u32, u32>);
impl SLV_DMA_RD_BYTELEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        SLV_DMA_RD_BYTELEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_DMA_RD_BYTELEN_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_DMA_RD_BYTELEN` writer - In the slave mode it is the length in bytes for read operations. The register value shall be byte_num."]
pub struct SLV_DMA_RD_BYTELEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_DMA_RD_BYTELEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | (value as u32 & 0x000f_ffff);
        self.w
    }
}
#[doc = "Field `SLV_RD_BUF_DONE` reader - The interrupt raw bit for the completion of read-buffer operation in the slave mode. Can not be changed by CONF_buf."]
pub struct SLV_RD_BUF_DONE_R(crate::FieldReader<bool, bool>);
impl SLV_RD_BUF_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLV_RD_BUF_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_RD_BUF_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_RD_BUF_DONE` writer - The interrupt raw bit for the completion of read-buffer operation in the slave mode. Can not be changed by CONF_buf."]
pub struct SLV_RD_BUF_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_RD_BUF_DONE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 24)) | ((value as u32 & 1) << 24);
        self.w
    }
}
#[doc = "Field `SEG_MAGIC_ERR` reader - 1: The recent magic value in CONF buffer is not right in master DMA seg-trans mode. 0: others."]
pub struct SEG_MAGIC_ERR_R(crate::FieldReader<bool, bool>);
impl SEG_MAGIC_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SEG_MAGIC_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEG_MAGIC_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEG_MAGIC_ERR` writer - 1: The recent magic value in CONF buffer is not right in master DMA seg-trans mode. 0: others."]
pub struct SEG_MAGIC_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> SEG_MAGIC_ERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 25)) | ((value as u32 & 1) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19 - In the slave mode it is the length in bytes for read operations. The register value shall be byte_num."]
    #[inline(always)]
    pub fn slv_dma_rd_bytelen(&self) -> SLV_DMA_RD_BYTELEN_R {
        SLV_DMA_RD_BYTELEN_R::new((self.bits & 0x000f_ffff) as u32)
    }
    #[doc = "Bit 24 - The interrupt raw bit for the completion of read-buffer operation in the slave mode. Can not be changed by CONF_buf."]
    #[inline(always)]
    pub fn slv_rd_buf_done(&self) -> SLV_RD_BUF_DONE_R {
        SLV_RD_BUF_DONE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 1: The recent magic value in CONF buffer is not right in master DMA seg-trans mode. 0: others."]
    #[inline(always)]
    pub fn seg_magic_err(&self) -> SEG_MAGIC_ERR_R {
        SEG_MAGIC_ERR_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:19 - In the slave mode it is the length in bytes for read operations. The register value shall be byte_num."]
    #[inline(always)]
    pub fn slv_dma_rd_bytelen(&mut self) -> SLV_DMA_RD_BYTELEN_W {
        SLV_DMA_RD_BYTELEN_W { w: self }
    }
    #[doc = "Bit 24 - The interrupt raw bit for the completion of read-buffer operation in the slave mode. Can not be changed by CONF_buf."]
    #[inline(always)]
    pub fn slv_rd_buf_done(&mut self) -> SLV_RD_BUF_DONE_W {
        SLV_RD_BUF_DONE_W { w: self }
    }
    #[doc = "Bit 25 - 1: The recent magic value in CONF buffer is not right in master DMA seg-trans mode. 0: others."]
    #[inline(always)]
    pub fn seg_magic_err(&mut self) -> SEG_MAGIC_ERR_W {
        SEG_MAGIC_ERR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI magic error and slave control register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slv_rdbuf_dlen]
(index.html) module"]
pub struct SLV_RDBUF_DLEN_SPEC;
impl crate::RegisterSpec for SLV_RDBUF_DLEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slv_rdbuf_dlen::R]
(R) reader structure"]
impl crate::Readable for SLV_RDBUF_DLEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slv_rdbuf_dlen::W]
(W) writer structure"]
impl crate::Writable for SLV_RDBUF_DLEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SLV_RDBUF_DLEN to value 0"]
impl crate::Resettable for SLV_RDBUF_DLEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
