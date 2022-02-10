#[doc = "Register `LCD_RGB_YUV` reader"]
pub struct R(crate::R<LCD_RGB_YUV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCD_RGB_YUV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCD_RGB_YUV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCD_RGB_YUV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCD_RGB_YUV` writer"]
pub struct W(crate::W<LCD_RGB_YUV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCD_RGB_YUV_SPEC>;
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
impl From<crate::W<LCD_RGB_YUV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCD_RGB_YUV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCD_CONV_8BITS_DATA_INV` reader - 1:invert every two 8bits input data. 2. disabled."]
pub struct LCD_CONV_8BITS_DATA_INV_R(crate::FieldReader<bool, bool>);
impl LCD_CONV_8BITS_DATA_INV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LCD_CONV_8BITS_DATA_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCD_CONV_8BITS_DATA_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCD_CONV_8BITS_DATA_INV` writer - 1:invert every two 8bits input data. 2. disabled."]
pub struct LCD_CONV_8BITS_DATA_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> LCD_CONV_8BITS_DATA_INV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `LCD_CONV_TXTORX` reader - 0: txtorx mode off. 1: txtorx mode on."]
pub struct LCD_CONV_TXTORX_R(crate::FieldReader<bool, bool>);
impl LCD_CONV_TXTORX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LCD_CONV_TXTORX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCD_CONV_TXTORX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCD_CONV_TXTORX` writer - 0: txtorx mode off. 1: txtorx mode on."]
pub struct LCD_CONV_TXTORX_W<'a> {
    w: &'a mut W,
}
impl<'a> LCD_CONV_TXTORX_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `LCD_CONV_YUV2YUV_MODE` reader - 0: to yuv422. 1: to yuv420. 2: to yuv411. 3: disabled. To enable yuv2yuv mode, trans_mode must be set to 1."]
pub struct LCD_CONV_YUV2YUV_MODE_R(crate::FieldReader<u8, u8>);
impl LCD_CONV_YUV2YUV_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LCD_CONV_YUV2YUV_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCD_CONV_YUV2YUV_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCD_CONV_YUV2YUV_MODE` writer - 0: to yuv422. 1: to yuv420. 2: to yuv411. 3: disabled. To enable yuv2yuv mode, trans_mode must be set to 1."]
pub struct LCD_CONV_YUV2YUV_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LCD_CONV_YUV2YUV_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
#[doc = "Field `LCD_CONV_YUV_MODE` reader - 0: yuv422. 1: yuv420. 2: yuv411. When in yuv2yuv mode, yuv_mode decides the yuv mode of Data_in"]
pub struct LCD_CONV_YUV_MODE_R(crate::FieldReader<u8, u8>);
impl LCD_CONV_YUV_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LCD_CONV_YUV_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCD_CONV_YUV_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCD_CONV_YUV_MODE` writer - 0: yuv422. 1: yuv420. 2: yuv411. When in yuv2yuv mode, yuv_mode decides the yuv mode of Data_in"]
pub struct LCD_CONV_YUV_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LCD_CONV_YUV_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `LCD_CONV_PROTOCOL_MODE` reader - 0:BT601. 1:BT709."]
pub struct LCD_CONV_PROTOCOL_MODE_R(crate::FieldReader<bool, bool>);
impl LCD_CONV_PROTOCOL_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LCD_CONV_PROTOCOL_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCD_CONV_PROTOCOL_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCD_CONV_PROTOCOL_MODE` writer - 0:BT601. 1:BT709."]
pub struct LCD_CONV_PROTOCOL_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LCD_CONV_PROTOCOL_MODE_W<'a> {
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
#[doc = "Field `LCD_CONV_DATA_OUT_MODE` reader - LIMIT or FULL mode of Data out. 0: limit. 1: full"]
pub struct LCD_CONV_DATA_OUT_MODE_R(crate::FieldReader<bool, bool>);
impl LCD_CONV_DATA_OUT_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LCD_CONV_DATA_OUT_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCD_CONV_DATA_OUT_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCD_CONV_DATA_OUT_MODE` writer - LIMIT or FULL mode of Data out. 0: limit. 1: full"]
pub struct LCD_CONV_DATA_OUT_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LCD_CONV_DATA_OUT_MODE_W<'a> {
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
#[doc = "Field `LCD_CONV_DATA_IN_MODE` reader - LIMIT or FULL mode of Data in. 0: limit. 1: full"]
pub struct LCD_CONV_DATA_IN_MODE_R(crate::FieldReader<bool, bool>);
impl LCD_CONV_DATA_IN_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LCD_CONV_DATA_IN_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCD_CONV_DATA_IN_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCD_CONV_DATA_IN_MODE` writer - LIMIT or FULL mode of Data in. 0: limit. 1: full"]
pub struct LCD_CONV_DATA_IN_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LCD_CONV_DATA_IN_MODE_W<'a> {
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
#[doc = "Field `LCD_CONV_MODE_8BITS_ON` reader - 0: 16bits mode. 1: 8bits mode."]
pub struct LCD_CONV_MODE_8BITS_ON_R(crate::FieldReader<bool, bool>);
impl LCD_CONV_MODE_8BITS_ON_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LCD_CONV_MODE_8BITS_ON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCD_CONV_MODE_8BITS_ON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCD_CONV_MODE_8BITS_ON` writer - 0: 16bits mode. 1: 8bits mode."]
pub struct LCD_CONV_MODE_8BITS_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> LCD_CONV_MODE_8BITS_ON_W<'a> {
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
#[doc = "Field `LCD_CONV_TRANS_MODE` reader - 0: YUV to RGB. 1: RGB to YUV."]
pub struct LCD_CONV_TRANS_MODE_R(crate::FieldReader<bool, bool>);
impl LCD_CONV_TRANS_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LCD_CONV_TRANS_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCD_CONV_TRANS_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCD_CONV_TRANS_MODE` writer - 0: YUV to RGB. 1: RGB to YUV."]
pub struct LCD_CONV_TRANS_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LCD_CONV_TRANS_MODE_W<'a> {
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
#[doc = "Field `LCD_CONV_BYPASS` reader - 0: Bypass converter. 1: Enable converter."]
pub struct LCD_CONV_BYPASS_R(crate::FieldReader<bool, bool>);
impl LCD_CONV_BYPASS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LCD_CONV_BYPASS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCD_CONV_BYPASS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCD_CONV_BYPASS` writer - 0: Bypass converter. 1: Enable converter."]
pub struct LCD_CONV_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> LCD_CONV_BYPASS_W<'a> {
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
    #[doc = "Bit 20 - 1:invert every two 8bits input data. 2. disabled."]
    #[inline(always)]
    pub fn lcd_conv_8bits_data_inv(&self) -> LCD_CONV_8BITS_DATA_INV_R {
        LCD_CONV_8BITS_DATA_INV_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - 0: txtorx mode off. 1: txtorx mode on."]
    #[inline(always)]
    pub fn lcd_conv_txtorx(&self) -> LCD_CONV_TXTORX_R {
        LCD_CONV_TXTORX_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bits 22:23 - 0: to yuv422. 1: to yuv420. 2: to yuv411. 3: disabled. To enable yuv2yuv mode, trans_mode must be set to 1."]
    #[inline(always)]
    pub fn lcd_conv_yuv2yuv_mode(&self) -> LCD_CONV_YUV2YUV_MODE_R {
        LCD_CONV_YUV2YUV_MODE_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - 0: yuv422. 1: yuv420. 2: yuv411. When in yuv2yuv mode, yuv_mode decides the yuv mode of Data_in"]
    #[inline(always)]
    pub fn lcd_conv_yuv_mode(&self) -> LCD_CONV_YUV_MODE_R {
        LCD_CONV_YUV_MODE_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 26 - 0:BT601. 1:BT709."]
    #[inline(always)]
    pub fn lcd_conv_protocol_mode(&self) -> LCD_CONV_PROTOCOL_MODE_R {
        LCD_CONV_PROTOCOL_MODE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - LIMIT or FULL mode of Data out. 0: limit. 1: full"]
    #[inline(always)]
    pub fn lcd_conv_data_out_mode(&self) -> LCD_CONV_DATA_OUT_MODE_R {
        LCD_CONV_DATA_OUT_MODE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - LIMIT or FULL mode of Data in. 0: limit. 1: full"]
    #[inline(always)]
    pub fn lcd_conv_data_in_mode(&self) -> LCD_CONV_DATA_IN_MODE_R {
        LCD_CONV_DATA_IN_MODE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - 0: 16bits mode. 1: 8bits mode."]
    #[inline(always)]
    pub fn lcd_conv_mode_8bits_on(&self) -> LCD_CONV_MODE_8BITS_ON_R {
        LCD_CONV_MODE_8BITS_ON_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - 0: YUV to RGB. 1: RGB to YUV."]
    #[inline(always)]
    pub fn lcd_conv_trans_mode(&self) -> LCD_CONV_TRANS_MODE_R {
        LCD_CONV_TRANS_MODE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - 0: Bypass converter. 1: Enable converter."]
    #[inline(always)]
    pub fn lcd_conv_bypass(&self) -> LCD_CONV_BYPASS_R {
        LCD_CONV_BYPASS_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 20 - 1:invert every two 8bits input data. 2. disabled."]
    #[inline(always)]
    pub fn lcd_conv_8bits_data_inv(&mut self) -> LCD_CONV_8BITS_DATA_INV_W {
        LCD_CONV_8BITS_DATA_INV_W { w: self }
    }
    #[doc = "Bit 21 - 0: txtorx mode off. 1: txtorx mode on."]
    #[inline(always)]
    pub fn lcd_conv_txtorx(&mut self) -> LCD_CONV_TXTORX_W {
        LCD_CONV_TXTORX_W { w: self }
    }
    #[doc = "Bits 22:23 - 0: to yuv422. 1: to yuv420. 2: to yuv411. 3: disabled. To enable yuv2yuv mode, trans_mode must be set to 1."]
    #[inline(always)]
    pub fn lcd_conv_yuv2yuv_mode(&mut self) -> LCD_CONV_YUV2YUV_MODE_W {
        LCD_CONV_YUV2YUV_MODE_W { w: self }
    }
    #[doc = "Bits 24:25 - 0: yuv422. 1: yuv420. 2: yuv411. When in yuv2yuv mode, yuv_mode decides the yuv mode of Data_in"]
    #[inline(always)]
    pub fn lcd_conv_yuv_mode(&mut self) -> LCD_CONV_YUV_MODE_W {
        LCD_CONV_YUV_MODE_W { w: self }
    }
    #[doc = "Bit 26 - 0:BT601. 1:BT709."]
    #[inline(always)]
    pub fn lcd_conv_protocol_mode(&mut self) -> LCD_CONV_PROTOCOL_MODE_W {
        LCD_CONV_PROTOCOL_MODE_W { w: self }
    }
    #[doc = "Bit 27 - LIMIT or FULL mode of Data out. 0: limit. 1: full"]
    #[inline(always)]
    pub fn lcd_conv_data_out_mode(&mut self) -> LCD_CONV_DATA_OUT_MODE_W {
        LCD_CONV_DATA_OUT_MODE_W { w: self }
    }
    #[doc = "Bit 28 - LIMIT or FULL mode of Data in. 0: limit. 1: full"]
    #[inline(always)]
    pub fn lcd_conv_data_in_mode(&mut self) -> LCD_CONV_DATA_IN_MODE_W {
        LCD_CONV_DATA_IN_MODE_W { w: self }
    }
    #[doc = "Bit 29 - 0: 16bits mode. 1: 8bits mode."]
    #[inline(always)]
    pub fn lcd_conv_mode_8bits_on(&mut self) -> LCD_CONV_MODE_8BITS_ON_W {
        LCD_CONV_MODE_8BITS_ON_W { w: self }
    }
    #[doc = "Bit 30 - 0: YUV to RGB. 1: RGB to YUV."]
    #[inline(always)]
    pub fn lcd_conv_trans_mode(&mut self) -> LCD_CONV_TRANS_MODE_W {
        LCD_CONV_TRANS_MODE_W { w: self }
    }
    #[doc = "Bit 31 - 0: Bypass converter. 1: Enable converter."]
    #[inline(always)]
    pub fn lcd_conv_bypass(&mut self) -> LCD_CONV_BYPASS_W {
        LCD_CONV_BYPASS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD configuration register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcd_rgb_yuv]
(index.html) module"]
pub struct LCD_RGB_YUV_SPEC;
impl crate::RegisterSpec for LCD_RGB_YUV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcd_rgb_yuv::R]
(R) reader structure"]
impl crate::Readable for LCD_RGB_YUV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcd_rgb_yuv::W]
(W) writer structure"]
impl crate::Writable for LCD_RGB_YUV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LCD_RGB_YUV to value 0x00c0_0000"]
impl crate::Resettable for LCD_RGB_YUV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x00c0_0000
    }
}
