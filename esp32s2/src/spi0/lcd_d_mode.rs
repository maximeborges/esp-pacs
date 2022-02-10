#[doc = "Register `LCD_D_MODE` reader"]
pub struct R(crate::R<LCD_D_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCD_D_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCD_D_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCD_D_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCD_D_MODE` writer"]
pub struct W(crate::W<LCD_D_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCD_D_MODE_SPEC>;
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
impl From<crate::W<LCD_D_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCD_D_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `D_DQS_MODE` reader - the output spi_dqs is delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
pub struct D_DQS_MODE_R(crate::FieldReader<u8, u8>);
impl D_DQS_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        D_DQS_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for D_DQS_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `D_DQS_MODE` writer - the output spi_dqs is delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
pub struct D_DQS_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> D_DQS_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `D_CD_MODE` reader - the output spi_cd is delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
pub struct D_CD_MODE_R(crate::FieldReader<u8, u8>);
impl D_CD_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        D_CD_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for D_CD_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `D_CD_MODE` writer - the output spi_cd is delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
pub struct D_CD_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> D_CD_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | ((value as u32 & 0x07) << 3);
        self.w
    }
}
#[doc = "Field `D_DE_MODE` reader - the output spi_de is delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
pub struct D_DE_MODE_R(crate::FieldReader<u8, u8>);
impl D_DE_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        D_DE_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for D_DE_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `D_DE_MODE` writer - the output spi_de is delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
pub struct D_DE_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> D_DE_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | ((value as u32 & 0x07) << 6);
        self.w
    }
}
#[doc = "Field `D_HSYNC_MODE` reader - the output spi_hsync is delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
pub struct D_HSYNC_MODE_R(crate::FieldReader<u8, u8>);
impl D_HSYNC_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        D_HSYNC_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for D_HSYNC_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `D_HSYNC_MODE` writer - the output spi_hsync is delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
pub struct D_HSYNC_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> D_HSYNC_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | ((value as u32 & 0x07) << 9);
        self.w
    }
}
#[doc = "Field `D_VSYNC_MODE` reader - the output spi_vsync is delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
pub struct D_VSYNC_MODE_R(crate::FieldReader<u8, u8>);
impl D_VSYNC_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        D_VSYNC_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for D_VSYNC_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `D_VSYNC_MODE` writer - the output spi_vsync is delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
pub struct D_VSYNC_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> D_VSYNC_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "Field `DE_IDLE_POL` reader - It is the idle value of spi_de."]
pub struct DE_IDLE_POL_R(crate::FieldReader<bool, bool>);
impl DE_IDLE_POL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DE_IDLE_POL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DE_IDLE_POL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DE_IDLE_POL` writer - It is the idle value of spi_de."]
pub struct DE_IDLE_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> DE_IDLE_POL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `HS_BLANK_EN` reader - 1: The pulse of spi_hsync is out in vertical blanking lines in seg-trans or one trans. 0: spi_hsync pulse is valid only in active region lines in seg-trans."]
pub struct HS_BLANK_EN_R(crate::FieldReader<bool, bool>);
impl HS_BLANK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HS_BLANK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HS_BLANK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HS_BLANK_EN` writer - 1: The pulse of spi_hsync is out in vertical blanking lines in seg-trans or one trans. 0: spi_hsync pulse is valid only in active region lines in seg-trans."]
pub struct HS_BLANK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> HS_BLANK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - the output spi_dqs is delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn d_dqs_mode(&self) -> D_DQS_MODE_R {
        D_DQS_MODE_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:5 - the output spi_cd is delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn d_cd_mode(&self) -> D_CD_MODE_R {
        D_CD_MODE_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 6:8 - the output spi_de is delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn d_de_mode(&self) -> D_DE_MODE_R {
        D_DE_MODE_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bits 9:11 - the output spi_hsync is delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn d_hsync_mode(&self) -> D_HSYNC_MODE_R {
        D_HSYNC_MODE_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - the output spi_vsync is delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn d_vsync_mode(&self) -> D_VSYNC_MODE_R {
        D_VSYNC_MODE_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 15 - It is the idle value of spi_de."]
    #[inline(always)]
    pub fn de_idle_pol(&self) -> DE_IDLE_POL_R {
        DE_IDLE_POL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - 1: The pulse of spi_hsync is out in vertical blanking lines in seg-trans or one trans. 0: spi_hsync pulse is valid only in active region lines in seg-trans."]
    #[inline(always)]
    pub fn hs_blank_en(&self) -> HS_BLANK_EN_R {
        HS_BLANK_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - the output spi_dqs is delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn d_dqs_mode(&mut self) -> D_DQS_MODE_W {
        D_DQS_MODE_W { w: self }
    }
    #[doc = "Bits 3:5 - the output spi_cd is delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn d_cd_mode(&mut self) -> D_CD_MODE_W {
        D_CD_MODE_W { w: self }
    }
    #[doc = "Bits 6:8 - the output spi_de is delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn d_de_mode(&mut self) -> D_DE_MODE_W {
        D_DE_MODE_W { w: self }
    }
    #[doc = "Bits 9:11 - the output spi_hsync is delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn d_hsync_mode(&mut self) -> D_HSYNC_MODE_W {
        D_HSYNC_MODE_W { w: self }
    }
    #[doc = "Bits 12:14 - the output spi_vsync is delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn d_vsync_mode(&mut self) -> D_VSYNC_MODE_W {
        D_VSYNC_MODE_W { w: self }
    }
    #[doc = "Bit 15 - It is the idle value of spi_de."]
    #[inline(always)]
    pub fn de_idle_pol(&mut self) -> DE_IDLE_POL_W {
        DE_IDLE_POL_W { w: self }
    }
    #[doc = "Bit 16 - 1: The pulse of spi_hsync is out in vertical blanking lines in seg-trans or one trans. 0: spi_hsync pulse is valid only in active region lines in seg-trans."]
    #[inline(always)]
    pub fn hs_blank_en(&mut self) -> HS_BLANK_EN_W {
        HS_BLANK_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD delay number\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcd_d_mode]
(index.html) module"]
pub struct LCD_D_MODE_SPEC;
impl crate::RegisterSpec for LCD_D_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcd_d_mode::R]
(R) reader structure"]
impl crate::Readable for LCD_D_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcd_d_mode::W]
(W) writer structure"]
impl crate::Writable for LCD_D_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LCD_D_MODE to value 0"]
impl crate::Resettable for LCD_D_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
