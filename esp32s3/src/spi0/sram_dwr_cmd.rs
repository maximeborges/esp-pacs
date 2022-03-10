#[doc = "Register `SRAM_DWR_CMD` reader"]
pub struct R(crate::R<SRAM_DWR_CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRAM_DWR_CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRAM_DWR_CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRAM_DWR_CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRAM_DWR_CMD` writer"]
pub struct W(crate::W<SRAM_DWR_CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRAM_DWR_CMD_SPEC>;
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
impl From<crate::W<SRAM_DWR_CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRAM_DWR_CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CACHE_SRAM_USR_WR_CMD_VALUE` reader - When SPI0 writes Ext_RAM, it is the command value of CMD phase."]
pub struct CACHE_SRAM_USR_WR_CMD_VALUE_R(crate::FieldReader<u16, u16>);
impl CACHE_SRAM_USR_WR_CMD_VALUE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CACHE_SRAM_USR_WR_CMD_VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CACHE_SRAM_USR_WR_CMD_VALUE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CACHE_SRAM_USR_WR_CMD_VALUE` writer - When SPI0 writes Ext_RAM, it is the command value of CMD phase."]
pub struct CACHE_SRAM_USR_WR_CMD_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHE_SRAM_USR_WR_CMD_VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `CACHE_SRAM_USR_WR_CMD_BITLEN` reader - When SPI0 writes Ext_RAM, it is the length in bits of CMD phase. The register value shall be (bit_num-1)."]
pub struct CACHE_SRAM_USR_WR_CMD_BITLEN_R(crate::FieldReader<u8, u8>);
impl CACHE_SRAM_USR_WR_CMD_BITLEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CACHE_SRAM_USR_WR_CMD_BITLEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CACHE_SRAM_USR_WR_CMD_BITLEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CACHE_SRAM_USR_WR_CMD_BITLEN` writer - When SPI0 writes Ext_RAM, it is the length in bits of CMD phase. The register value shall be (bit_num-1)."]
pub struct CACHE_SRAM_USR_WR_CMD_BITLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHE_SRAM_USR_WR_CMD_BITLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - When SPI0 writes Ext_RAM, it is the command value of CMD phase."]
    #[inline(always)]
    pub fn cache_sram_usr_wr_cmd_value(&self) -> CACHE_SRAM_USR_WR_CMD_VALUE_R {
        CACHE_SRAM_USR_WR_CMD_VALUE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 28:31 - When SPI0 writes Ext_RAM, it is the length in bits of CMD phase. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn cache_sram_usr_wr_cmd_bitlen(&self) -> CACHE_SRAM_USR_WR_CMD_BITLEN_R {
        CACHE_SRAM_USR_WR_CMD_BITLEN_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - When SPI0 writes Ext_RAM, it is the command value of CMD phase."]
    #[inline(always)]
    pub fn cache_sram_usr_wr_cmd_value(&mut self) -> CACHE_SRAM_USR_WR_CMD_VALUE_W {
        CACHE_SRAM_USR_WR_CMD_VALUE_W { w: self }
    }
    #[doc = "Bits 28:31 - When SPI0 writes Ext_RAM, it is the length in bits of CMD phase. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn cache_sram_usr_wr_cmd_bitlen(&mut self) -> CACHE_SRAM_USR_WR_CMD_BITLEN_W {
        CACHE_SRAM_USR_WR_CMD_BITLEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI0 external RAM DDR write command control register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sram_dwr_cmd]
(index.html) module"]
pub struct SRAM_DWR_CMD_SPEC;
impl crate::RegisterSpec for SRAM_DWR_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sram_dwr_cmd::R]
(R) reader structure"]
impl crate::Readable for SRAM_DWR_CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sram_dwr_cmd::W]
(W) writer structure"]
impl crate::Writable for SRAM_DWR_CMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRAM_DWR_CMD to value 0"]
impl crate::Resettable for SRAM_DWR_CMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
