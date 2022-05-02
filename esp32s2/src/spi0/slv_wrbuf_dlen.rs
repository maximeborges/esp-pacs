#[doc = "Register `SLV_WRBUF_DLEN` reader"]
pub struct R(crate::R<SLV_WRBUF_DLEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLV_WRBUF_DLEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLV_WRBUF_DLEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLV_WRBUF_DLEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLV_WRBUF_DLEN` writer"]
pub struct W(crate::W<SLV_WRBUF_DLEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLV_WRBUF_DLEN_SPEC>;
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
impl From<crate::W<SLV_WRBUF_DLEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLV_WRBUF_DLEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLV_WR_BUF_DONE` reader - The interrupt raw bit for the completion of write-buffer operation in the slave mode. Can not be changed by CONF_buf."]
pub struct SLV_WR_BUF_DONE_R(crate::FieldReader<bool>);
impl SLV_WR_BUF_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLV_WR_BUF_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_WR_BUF_DONE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_WR_BUF_DONE` writer - The interrupt raw bit for the completion of write-buffer operation in the slave mode. Can not be changed by CONF_buf."]
pub struct SLV_WR_BUF_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_WR_BUF_DONE_W<'a> {
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
#[doc = "Field `CONF_BASE_BITLEN` reader - The basic spi_clk cycles of CONF state. The real cycle length of CONF state, if SPI_USR_CONF is enabled, is SPI_CONF_BASE_BITLEN\\[6:0\\] + SPI_CONF_BITLEN\\[23:0\\]."]
pub struct CONF_BASE_BITLEN_R(crate::FieldReader<u8>);
impl CONF_BASE_BITLEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CONF_BASE_BITLEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CONF_BASE_BITLEN_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONF_BASE_BITLEN` writer - The basic spi_clk cycles of CONF state. The real cycle length of CONF state, if SPI_USR_CONF is enabled, is SPI_CONF_BASE_BITLEN\\[6:0\\] + SPI_CONF_BITLEN\\[23:0\\]."]
pub struct CONF_BASE_BITLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CONF_BASE_BITLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 25)) | ((value as u32 & 0x7f) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bit 24 - The interrupt raw bit for the completion of write-buffer operation in the slave mode. Can not be changed by CONF_buf."]
    #[inline(always)]
    pub fn slv_wr_buf_done(&self) -> SLV_WR_BUF_DONE_R {
        SLV_WR_BUF_DONE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:31 - The basic spi_clk cycles of CONF state. The real cycle length of CONF state, if SPI_USR_CONF is enabled, is SPI_CONF_BASE_BITLEN\\[6:0\\] + SPI_CONF_BITLEN\\[23:0\\]."]
    #[inline(always)]
    pub fn conf_base_bitlen(&self) -> CONF_BASE_BITLEN_R {
        CONF_BASE_BITLEN_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 24 - The interrupt raw bit for the completion of write-buffer operation in the slave mode. Can not be changed by CONF_buf."]
    #[inline(always)]
    pub fn slv_wr_buf_done(&mut self) -> SLV_WR_BUF_DONE_W {
        SLV_WR_BUF_DONE_W { w: self }
    }
    #[doc = "Bits 25:31 - The basic spi_clk cycles of CONF state. The real cycle length of CONF state, if SPI_USR_CONF is enabled, is SPI_CONF_BASE_BITLEN\\[6:0\\] + SPI_CONF_BITLEN\\[23:0\\]."]
    #[inline(always)]
    pub fn conf_base_bitlen(&mut self) -> CONF_BASE_BITLEN_W {
        CONF_BASE_BITLEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI slave Wr_BUF interrupt and CONF control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slv_wrbuf_dlen](index.html) module"]
pub struct SLV_WRBUF_DLEN_SPEC;
impl crate::RegisterSpec for SLV_WRBUF_DLEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slv_wrbuf_dlen::R](R) reader structure"]
impl crate::Readable for SLV_WRBUF_DLEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slv_wrbuf_dlen::W](W) writer structure"]
impl crate::Writable for SLV_WRBUF_DLEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SLV_WRBUF_DLEN to value 0xd800_0000"]
impl crate::Resettable for SLV_WRBUF_DLEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xd800_0000
    }
}
