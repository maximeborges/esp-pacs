#[doc = "Register `CLOCK` reader"]
pub struct R(crate::R<CLOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLOCK` writer"]
pub struct W(crate::W<CLOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLOCK_SPEC>;
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
impl From<crate::W<CLOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKCNT_L` reader - It must equal to the value of SPI_MEM_CLKCNT_N."]
pub struct CLKCNT_L_R(crate::FieldReader<u8, u8>);
impl CLKCNT_L_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CLKCNT_L_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKCNT_L_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKCNT_L` writer - It must equal to the value of SPI_MEM_CLKCNT_N."]
pub struct CLKCNT_L_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKCNT_L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `CLKCNT_H` reader - It must be a floor value of ((SPI_MEM_CLKCNT_N+1)/2-1)."]
pub struct CLKCNT_H_R(crate::FieldReader<u8, u8>);
impl CLKCNT_H_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CLKCNT_H_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKCNT_H_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKCNT_H` writer - It must be a floor value of ((SPI_MEM_CLKCNT_N+1)/2-1)."]
pub struct CLKCNT_H_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKCNT_H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `CLKCNT_N` reader - When SPI1 accesses to flash or Ext_RAM, f_SPI_CLK = f_MSPI_CORE_CLK/(SPI_MEM_CLKCNT_N+1)"]
pub struct CLKCNT_N_R(crate::FieldReader<u8, u8>);
impl CLKCNT_N_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CLKCNT_N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKCNT_N_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKCNT_N` writer - When SPI1 accesses to flash or Ext_RAM, f_SPI_CLK = f_MSPI_CORE_CLK/(SPI_MEM_CLKCNT_N+1)"]
pub struct CLKCNT_N_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKCNT_N_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `CLK_EQU_SYSCLK` reader - When SPI1 access to flash or Ext_RAM, set this bit in 1-division mode, f_SPI_CLK = f_MSPI_CORE_CLK."]
pub struct CLK_EQU_SYSCLK_R(crate::FieldReader<bool, bool>);
impl CLK_EQU_SYSCLK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_EQU_SYSCLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_EQU_SYSCLK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK_EQU_SYSCLK` writer - When SPI1 access to flash or Ext_RAM, set this bit in 1-division mode, f_SPI_CLK = f_MSPI_CORE_CLK."]
pub struct CLK_EQU_SYSCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_EQU_SYSCLK_W<'a> {
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
    #[doc = "Bits 0:7 - It must equal to the value of SPI_MEM_CLKCNT_N."]
    #[inline(always)]
    pub fn clkcnt_l(&self) -> CLKCNT_L_R {
        CLKCNT_L_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - It must be a floor value of ((SPI_MEM_CLKCNT_N+1)/2-1)."]
    #[inline(always)]
    pub fn clkcnt_h(&self) -> CLKCNT_H_R {
        CLKCNT_H_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - When SPI1 accesses to flash or Ext_RAM, f_SPI_CLK = f_MSPI_CORE_CLK/(SPI_MEM_CLKCNT_N+1)"]
    #[inline(always)]
    pub fn clkcnt_n(&self) -> CLKCNT_N_R {
        CLKCNT_N_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 31 - When SPI1 access to flash or Ext_RAM, set this bit in 1-division mode, f_SPI_CLK = f_MSPI_CORE_CLK."]
    #[inline(always)]
    pub fn clk_equ_sysclk(&self) -> CLK_EQU_SYSCLK_R {
        CLK_EQU_SYSCLK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - It must equal to the value of SPI_MEM_CLKCNT_N."]
    #[inline(always)]
    pub fn clkcnt_l(&mut self) -> CLKCNT_L_W {
        CLKCNT_L_W { w: self }
    }
    #[doc = "Bits 8:15 - It must be a floor value of ((SPI_MEM_CLKCNT_N+1)/2-1)."]
    #[inline(always)]
    pub fn clkcnt_h(&mut self) -> CLKCNT_H_W {
        CLKCNT_H_W { w: self }
    }
    #[doc = "Bits 16:23 - When SPI1 accesses to flash or Ext_RAM, f_SPI_CLK = f_MSPI_CORE_CLK/(SPI_MEM_CLKCNT_N+1)"]
    #[inline(always)]
    pub fn clkcnt_n(&mut self) -> CLKCNT_N_W {
        CLKCNT_N_W { w: self }
    }
    #[doc = "Bit 31 - When SPI1 access to flash or Ext_RAM, set this bit in 1-division mode, f_SPI_CLK = f_MSPI_CORE_CLK."]
    #[inline(always)]
    pub fn clk_equ_sysclk(&mut self) -> CLK_EQU_SYSCLK_W {
        CLK_EQU_SYSCLK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI_CLK clock division register when SPI1 accesses to flash or Ext_RAM.\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clock]
(index.html) module"]
pub struct CLOCK_SPEC;
impl crate::RegisterSpec for CLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clock::R]
(R) reader structure"]
impl crate::Readable for CLOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clock::W]
(W) writer structure"]
impl crate::Writable for CLOCK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLOCK to value 0x0003_0103"]
impl crate::Resettable for CLOCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0003_0103
    }
}
