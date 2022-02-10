#[doc = "Register `LCD_D_NUM` reader"]
pub struct R(crate::R<LCD_D_NUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCD_D_NUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCD_D_NUM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCD_D_NUM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCD_D_NUM` writer"]
pub struct W(crate::W<LCD_D_NUM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCD_D_NUM_SPEC>;
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
impl From<crate::W<LCD_D_NUM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCD_D_NUM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `D_DQS_NUM` reader - the output spi_dqs is delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub struct D_DQS_NUM_R(crate::FieldReader<u8, u8>);
impl D_DQS_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        D_DQS_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for D_DQS_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `D_DQS_NUM` writer - the output spi_dqs is delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub struct D_DQS_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> D_DQS_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `D_CD_NUM` reader - the output spi_cd is delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub struct D_CD_NUM_R(crate::FieldReader<u8, u8>);
impl D_CD_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        D_CD_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for D_CD_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `D_CD_NUM` writer - the output spi_cd is delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub struct D_CD_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> D_CD_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `D_DE_NUM` reader - the output spi_de is delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub struct D_DE_NUM_R(crate::FieldReader<u8, u8>);
impl D_DE_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        D_DE_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for D_DE_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `D_DE_NUM` writer - the output spi_de is delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub struct D_DE_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> D_DE_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `D_HSYNC_NUM` reader - the output spi_hsync is delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub struct D_HSYNC_NUM_R(crate::FieldReader<u8, u8>);
impl D_HSYNC_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        D_HSYNC_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for D_HSYNC_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `D_HSYNC_NUM` writer - the output spi_hsync is delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub struct D_HSYNC_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> D_HSYNC_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `D_VSYNC_NUM` reader - the output spi_vsync is delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub struct D_VSYNC_NUM_R(crate::FieldReader<u8, u8>);
impl D_VSYNC_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        D_VSYNC_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for D_VSYNC_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `D_VSYNC_NUM` writer - the output spi_vsync is delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub struct D_VSYNC_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> D_VSYNC_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - the output spi_dqs is delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn d_dqs_num(&self) -> D_DQS_NUM_R {
        D_DQS_NUM_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - the output spi_cd is delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn d_cd_num(&self) -> D_CD_NUM_R {
        D_CD_NUM_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - the output spi_de is delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn d_de_num(&self) -> D_DE_NUM_R {
        D_DE_NUM_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - the output spi_hsync is delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn d_hsync_num(&self) -> D_HSYNC_NUM_R {
        D_HSYNC_NUM_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - the output spi_vsync is delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn d_vsync_num(&self) -> D_VSYNC_NUM_R {
        D_VSYNC_NUM_R::new(((self.bits >> 8) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - the output spi_dqs is delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn d_dqs_num(&mut self) -> D_DQS_NUM_W {
        D_DQS_NUM_W { w: self }
    }
    #[doc = "Bits 2:3 - the output spi_cd is delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn d_cd_num(&mut self) -> D_CD_NUM_W {
        D_CD_NUM_W { w: self }
    }
    #[doc = "Bits 4:5 - the output spi_de is delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn d_de_num(&mut self) -> D_DE_NUM_W {
        D_DE_NUM_W { w: self }
    }
    #[doc = "Bits 6:7 - the output spi_hsync is delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn d_hsync_num(&mut self) -> D_HSYNC_NUM_W {
        D_HSYNC_NUM_W { w: self }
    }
    #[doc = "Bits 8:9 - the output spi_vsync is delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn d_vsync_num(&mut self) -> D_VSYNC_NUM_W {
        D_VSYNC_NUM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD delay mode\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcd_d_num]
(index.html) module"]
pub struct LCD_D_NUM_SPEC;
impl crate::RegisterSpec for LCD_D_NUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcd_d_num::R]
(R) reader structure"]
impl crate::Readable for LCD_D_NUM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcd_d_num::W]
(W) writer structure"]
impl crate::Writable for LCD_D_NUM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LCD_D_NUM to value 0"]
impl crate::Resettable for LCD_D_NUM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
