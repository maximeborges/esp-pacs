#[doc = "Register `CAM_CTRL1` reader"]
pub struct R(crate::R<CAM_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAM_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAM_CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAM_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAM_CTRL1` writer"]
pub struct W(crate::W<CAM_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAM_CTRL1_SPEC>;
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
impl From<crate::W<CAM_CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAM_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAM_REC_DATA_BYTELEN` reader - Camera receive data byte length minus 1 to set DMA in_suc_eof_int."]
pub struct CAM_REC_DATA_BYTELEN_R(crate::FieldReader<u16, u16>);
impl CAM_REC_DATA_BYTELEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CAM_REC_DATA_BYTELEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAM_REC_DATA_BYTELEN_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAM_REC_DATA_BYTELEN` writer - Camera receive data byte length minus 1 to set DMA in_suc_eof_int."]
pub struct CAM_REC_DATA_BYTELEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CAM_REC_DATA_BYTELEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `CAM_LINE_INT_NUM` reader - The line number minus 1 to generate cam_hs_int."]
pub struct CAM_LINE_INT_NUM_R(crate::FieldReader<u8, u8>);
impl CAM_LINE_INT_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CAM_LINE_INT_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAM_LINE_INT_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAM_LINE_INT_NUM` writer - The line number minus 1 to generate cam_hs_int."]
pub struct CAM_LINE_INT_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> CAM_LINE_INT_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
#[doc = "Field `CAM_CLK_INV` reader - 1: Invert the input signal CAM_PCLK. 0: Not invert."]
pub struct CAM_CLK_INV_R(crate::FieldReader<bool, bool>);
impl CAM_CLK_INV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CAM_CLK_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAM_CLK_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAM_CLK_INV` writer - 1: Invert the input signal CAM_PCLK. 0: Not invert."]
pub struct CAM_CLK_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> CAM_CLK_INV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `CAM_VSYNC_FILTER_EN` reader - 1: Enable CAM_VSYNC filter function. 0: bypass."]
pub struct CAM_VSYNC_FILTER_EN_R(crate::FieldReader<bool, bool>);
impl CAM_VSYNC_FILTER_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CAM_VSYNC_FILTER_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAM_VSYNC_FILTER_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAM_VSYNC_FILTER_EN` writer - 1: Enable CAM_VSYNC filter function. 0: bypass."]
pub struct CAM_VSYNC_FILTER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CAM_VSYNC_FILTER_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `CAM_2BYTE_EN` reader - 1: The bit number of input data is 9~16. 0: The bit number of input data is 0~8."]
pub struct CAM_2BYTE_EN_R(crate::FieldReader<bool, bool>);
impl CAM_2BYTE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CAM_2BYTE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAM_2BYTE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAM_2BYTE_EN` writer - 1: The bit number of input data is 9~16. 0: The bit number of input data is 0~8."]
pub struct CAM_2BYTE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CAM_2BYTE_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `CAM_DE_INV` reader - CAM_DE invert enable signal, valid in high level."]
pub struct CAM_DE_INV_R(crate::FieldReader<bool, bool>);
impl CAM_DE_INV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CAM_DE_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAM_DE_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAM_DE_INV` writer - CAM_DE invert enable signal, valid in high level."]
pub struct CAM_DE_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> CAM_DE_INV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `CAM_HSYNC_INV` reader - CAM_HSYNC invert enable signal, valid in high level."]
pub struct CAM_HSYNC_INV_R(crate::FieldReader<bool, bool>);
impl CAM_HSYNC_INV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CAM_HSYNC_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAM_HSYNC_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAM_HSYNC_INV` writer - CAM_HSYNC invert enable signal, valid in high level."]
pub struct CAM_HSYNC_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> CAM_HSYNC_INV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `CAM_VSYNC_INV` reader - CAM_VSYNC invert enable signal, valid in high level."]
pub struct CAM_VSYNC_INV_R(crate::FieldReader<bool, bool>);
impl CAM_VSYNC_INV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CAM_VSYNC_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAM_VSYNC_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAM_VSYNC_INV` writer - CAM_VSYNC invert enable signal, valid in high level."]
pub struct CAM_VSYNC_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> CAM_VSYNC_INV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `CAM_VH_DE_MODE_EN` reader - 1: Input control signals are CAM_DE CAM_HSYNC and CAM_VSYNC is 1. 0: Input control signals are CAM_DE and CAM_VSYNC. CAM_HSYNC and CAM_DE are all 1 the the same time."]
pub struct CAM_VH_DE_MODE_EN_R(crate::FieldReader<bool, bool>);
impl CAM_VH_DE_MODE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CAM_VH_DE_MODE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAM_VH_DE_MODE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAM_VH_DE_MODE_EN` writer - 1: Input control signals are CAM_DE CAM_HSYNC and CAM_VSYNC is 1. 0: Input control signals are CAM_DE and CAM_VSYNC. CAM_HSYNC and CAM_DE are all 1 the the same time."]
pub struct CAM_VH_DE_MODE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CAM_VH_DE_MODE_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `CAM_START` reader - Camera module start signal."]
pub struct CAM_START_R(crate::FieldReader<bool, bool>);
impl CAM_START_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CAM_START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAM_START_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAM_START` writer - Camera module start signal."]
pub struct CAM_START_W<'a> {
    w: &'a mut W,
}
impl<'a> CAM_START_W<'a> {
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
#[doc = "Field `CAM_RESET` writer - Camera module reset signal."]
pub struct CAM_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> CAM_RESET_W<'a> {
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
#[doc = "Field `CAM_AFIFO_RESET` writer - Camera AFIFO reset signal."]
pub struct CAM_AFIFO_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> CAM_AFIFO_RESET_W<'a> {
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
    #[doc = "Bits 0:15 - Camera receive data byte length minus 1 to set DMA in_suc_eof_int."]
    #[inline(always)]
    pub fn cam_rec_data_bytelen(&self) -> CAM_REC_DATA_BYTELEN_R {
        CAM_REC_DATA_BYTELEN_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:21 - The line number minus 1 to generate cam_hs_int."]
    #[inline(always)]
    pub fn cam_line_int_num(&self) -> CAM_LINE_INT_NUM_R {
        CAM_LINE_INT_NUM_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - 1: Invert the input signal CAM_PCLK. 0: Not invert."]
    #[inline(always)]
    pub fn cam_clk_inv(&self) -> CAM_CLK_INV_R {
        CAM_CLK_INV_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - 1: Enable CAM_VSYNC filter function. 0: bypass."]
    #[inline(always)]
    pub fn cam_vsync_filter_en(&self) -> CAM_VSYNC_FILTER_EN_R {
        CAM_VSYNC_FILTER_EN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - 1: The bit number of input data is 9~16. 0: The bit number of input data is 0~8."]
    #[inline(always)]
    pub fn cam_2byte_en(&self) -> CAM_2BYTE_EN_R {
        CAM_2BYTE_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - CAM_DE invert enable signal, valid in high level."]
    #[inline(always)]
    pub fn cam_de_inv(&self) -> CAM_DE_INV_R {
        CAM_DE_INV_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - CAM_HSYNC invert enable signal, valid in high level."]
    #[inline(always)]
    pub fn cam_hsync_inv(&self) -> CAM_HSYNC_INV_R {
        CAM_HSYNC_INV_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - CAM_VSYNC invert enable signal, valid in high level."]
    #[inline(always)]
    pub fn cam_vsync_inv(&self) -> CAM_VSYNC_INV_R {
        CAM_VSYNC_INV_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - 1: Input control signals are CAM_DE CAM_HSYNC and CAM_VSYNC is 1. 0: Input control signals are CAM_DE and CAM_VSYNC. CAM_HSYNC and CAM_DE are all 1 the the same time."]
    #[inline(always)]
    pub fn cam_vh_de_mode_en(&self) -> CAM_VH_DE_MODE_EN_R {
        CAM_VH_DE_MODE_EN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Camera module start signal."]
    #[inline(always)]
    pub fn cam_start(&self) -> CAM_START_R {
        CAM_START_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Camera receive data byte length minus 1 to set DMA in_suc_eof_int."]
    #[inline(always)]
    pub fn cam_rec_data_bytelen(&mut self) -> CAM_REC_DATA_BYTELEN_W {
        CAM_REC_DATA_BYTELEN_W { w: self }
    }
    #[doc = "Bits 16:21 - The line number minus 1 to generate cam_hs_int."]
    #[inline(always)]
    pub fn cam_line_int_num(&mut self) -> CAM_LINE_INT_NUM_W {
        CAM_LINE_INT_NUM_W { w: self }
    }
    #[doc = "Bit 22 - 1: Invert the input signal CAM_PCLK. 0: Not invert."]
    #[inline(always)]
    pub fn cam_clk_inv(&mut self) -> CAM_CLK_INV_W {
        CAM_CLK_INV_W { w: self }
    }
    #[doc = "Bit 23 - 1: Enable CAM_VSYNC filter function. 0: bypass."]
    #[inline(always)]
    pub fn cam_vsync_filter_en(&mut self) -> CAM_VSYNC_FILTER_EN_W {
        CAM_VSYNC_FILTER_EN_W { w: self }
    }
    #[doc = "Bit 24 - 1: The bit number of input data is 9~16. 0: The bit number of input data is 0~8."]
    #[inline(always)]
    pub fn cam_2byte_en(&mut self) -> CAM_2BYTE_EN_W {
        CAM_2BYTE_EN_W { w: self }
    }
    #[doc = "Bit 25 - CAM_DE invert enable signal, valid in high level."]
    #[inline(always)]
    pub fn cam_de_inv(&mut self) -> CAM_DE_INV_W {
        CAM_DE_INV_W { w: self }
    }
    #[doc = "Bit 26 - CAM_HSYNC invert enable signal, valid in high level."]
    #[inline(always)]
    pub fn cam_hsync_inv(&mut self) -> CAM_HSYNC_INV_W {
        CAM_HSYNC_INV_W { w: self }
    }
    #[doc = "Bit 27 - CAM_VSYNC invert enable signal, valid in high level."]
    #[inline(always)]
    pub fn cam_vsync_inv(&mut self) -> CAM_VSYNC_INV_W {
        CAM_VSYNC_INV_W { w: self }
    }
    #[doc = "Bit 28 - 1: Input control signals are CAM_DE CAM_HSYNC and CAM_VSYNC is 1. 0: Input control signals are CAM_DE and CAM_VSYNC. CAM_HSYNC and CAM_DE are all 1 the the same time."]
    #[inline(always)]
    pub fn cam_vh_de_mode_en(&mut self) -> CAM_VH_DE_MODE_EN_W {
        CAM_VH_DE_MODE_EN_W { w: self }
    }
    #[doc = "Bit 29 - Camera module start signal."]
    #[inline(always)]
    pub fn cam_start(&mut self) -> CAM_START_W {
        CAM_START_W { w: self }
    }
    #[doc = "Bit 30 - Camera module reset signal."]
    #[inline(always)]
    pub fn cam_reset(&mut self) -> CAM_RESET_W {
        CAM_RESET_W { w: self }
    }
    #[doc = "Bit 31 - Camera AFIFO reset signal."]
    #[inline(always)]
    pub fn cam_afifo_reset(&mut self) -> CAM_AFIFO_RESET_W {
        CAM_AFIFO_RESET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Camera configuration register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cam_ctrl1]
(index.html) module"]
pub struct CAM_CTRL1_SPEC;
impl crate::RegisterSpec for CAM_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cam_ctrl1::R]
(R) reader structure"]
impl crate::Readable for CAM_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cam_ctrl1::W]
(W) writer structure"]
impl crate::Writable for CAM_CTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAM_CTRL1 to value 0"]
impl crate::Resettable for CAM_CTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
