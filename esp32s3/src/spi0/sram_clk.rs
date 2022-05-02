#[doc = "Register `SRAM_CLK` reader"]
pub struct R(crate::R<SRAM_CLK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRAM_CLK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRAM_CLK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRAM_CLK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRAM_CLK` writer"]
pub struct W(crate::W<SRAM_CLK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRAM_CLK_SPEC>;
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
impl From<crate::W<SRAM_CLK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRAM_CLK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCLKCNT_L` reader - It must equal to the value of SPI_MEM_SCLKCNT_N."]
pub struct SCLKCNT_L_R(crate::FieldReader<u8>);
impl SCLKCNT_L_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SCLKCNT_L_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCLKCNT_L_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCLKCNT_L` writer - It must equal to the value of SPI_MEM_SCLKCNT_N."]
pub struct SCLKCNT_L_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLKCNT_L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `SCLKCNT_H` reader - It must be a floor value of ((SPI_MEM_SCLKCNT_N+1)/2-1)."]
pub struct SCLKCNT_H_R(crate::FieldReader<u8>);
impl SCLKCNT_H_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SCLKCNT_H_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCLKCNT_H_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCLKCNT_H` writer - It must be a floor value of ((SPI_MEM_SCLKCNT_N+1)/2-1)."]
pub struct SCLKCNT_H_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLKCNT_H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `SCLKCNT_N` reader - When SPI0 accesses to Ext_RAM, f_SPI_CLK = f_MSPI_CORE_CLK/(SPI_MEM_SCLKCNT_N+1)"]
pub struct SCLKCNT_N_R(crate::FieldReader<u8>);
impl SCLKCNT_N_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SCLKCNT_N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCLKCNT_N_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCLKCNT_N` writer - When SPI0 accesses to Ext_RAM, f_SPI_CLK = f_MSPI_CORE_CLK/(SPI_MEM_SCLKCNT_N+1)"]
pub struct SCLKCNT_N_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLKCNT_N_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `SCLK_EQU_SYSCLK` reader - When SPI0 accesses to Ext_RAM, set this bit in 1-division mode, f_SPI_CLK = f_MSPI_CORE_CLK."]
pub struct SCLK_EQU_SYSCLK_R(crate::FieldReader<bool>);
impl SCLK_EQU_SYSCLK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCLK_EQU_SYSCLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCLK_EQU_SYSCLK_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCLK_EQU_SYSCLK` writer - When SPI0 accesses to Ext_RAM, set this bit in 1-division mode, f_SPI_CLK = f_MSPI_CORE_CLK."]
pub struct SCLK_EQU_SYSCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLK_EQU_SYSCLK_W<'a> {
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
    #[doc = "Bits 0:7 - It must equal to the value of SPI_MEM_SCLKCNT_N."]
    #[inline(always)]
    pub fn sclkcnt_l(&self) -> SCLKCNT_L_R {
        SCLKCNT_L_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - It must be a floor value of ((SPI_MEM_SCLKCNT_N+1)/2-1)."]
    #[inline(always)]
    pub fn sclkcnt_h(&self) -> SCLKCNT_H_R {
        SCLKCNT_H_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - When SPI0 accesses to Ext_RAM, f_SPI_CLK = f_MSPI_CORE_CLK/(SPI_MEM_SCLKCNT_N+1)"]
    #[inline(always)]
    pub fn sclkcnt_n(&self) -> SCLKCNT_N_R {
        SCLKCNT_N_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 31 - When SPI0 accesses to Ext_RAM, set this bit in 1-division mode, f_SPI_CLK = f_MSPI_CORE_CLK."]
    #[inline(always)]
    pub fn sclk_equ_sysclk(&self) -> SCLK_EQU_SYSCLK_R {
        SCLK_EQU_SYSCLK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - It must equal to the value of SPI_MEM_SCLKCNT_N."]
    #[inline(always)]
    pub fn sclkcnt_l(&mut self) -> SCLKCNT_L_W {
        SCLKCNT_L_W { w: self }
    }
    #[doc = "Bits 8:15 - It must be a floor value of ((SPI_MEM_SCLKCNT_N+1)/2-1)."]
    #[inline(always)]
    pub fn sclkcnt_h(&mut self) -> SCLKCNT_H_W {
        SCLKCNT_H_W { w: self }
    }
    #[doc = "Bits 16:23 - When SPI0 accesses to Ext_RAM, f_SPI_CLK = f_MSPI_CORE_CLK/(SPI_MEM_SCLKCNT_N+1)"]
    #[inline(always)]
    pub fn sclkcnt_n(&mut self) -> SCLKCNT_N_W {
        SCLKCNT_N_W { w: self }
    }
    #[doc = "Bit 31 - When SPI0 accesses to Ext_RAM, set this bit in 1-division mode, f_SPI_CLK = f_MSPI_CORE_CLK."]
    #[inline(always)]
    pub fn sclk_equ_sysclk(&mut self) -> SCLK_EQU_SYSCLK_W {
        SCLK_EQU_SYSCLK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI_CLK clock division register when SPI0 accesses to Ext_RAM.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sram_clk](index.html) module"]
pub struct SRAM_CLK_SPEC;
impl crate::RegisterSpec for SRAM_CLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sram_clk::R](R) reader structure"]
impl crate::Readable for SRAM_CLK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sram_clk::W](W) writer structure"]
impl crate::Writable for SRAM_CLK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRAM_CLK to value 0x0003_0103"]
impl crate::Resettable for SRAM_CLK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0003_0103
    }
}
