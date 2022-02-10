#[doc = "Register `CAM_CTRL` reader"]
pub struct R(crate::R<CAM_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAM_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAM_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAM_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAM_CTRL` writer"]
pub struct W(crate::W<CAM_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAM_CTRL_SPEC>;
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
impl From<crate::W<CAM_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAM_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAM_STOP_EN` reader - Camera stop enable signal, 1: camera stops when DMA Rx FIFO is full. 0: Not stop."]
pub struct CAM_STOP_EN_R(crate::FieldReader<bool, bool>);
impl CAM_STOP_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CAM_STOP_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAM_STOP_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAM_STOP_EN` writer - Camera stop enable signal, 1: camera stops when DMA Rx FIFO is full. 0: Not stop."]
pub struct CAM_STOP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CAM_STOP_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `CAM_VSYNC_FILTER_THRES` reader - Filter threshold value for CAM_VSYNC signal."]
pub struct CAM_VSYNC_FILTER_THRES_R(crate::FieldReader<u8, u8>);
impl CAM_VSYNC_FILTER_THRES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CAM_VSYNC_FILTER_THRES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAM_VSYNC_FILTER_THRES_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAM_VSYNC_FILTER_THRES` writer - Filter threshold value for CAM_VSYNC signal."]
pub struct CAM_VSYNC_FILTER_THRES_W<'a> {
    w: &'a mut W,
}
impl<'a> CAM_VSYNC_FILTER_THRES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | ((value as u32 & 0x07) << 1);
        self.w
    }
}
#[doc = "Field `CAM_UPDATE` reader - 1: Update Camera registers, will be cleared by hardware. 0 : Not care."]
pub struct CAM_UPDATE_R(crate::FieldReader<bool, bool>);
impl CAM_UPDATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CAM_UPDATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAM_UPDATE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAM_UPDATE` writer - 1: Update Camera registers, will be cleared by hardware. 0 : Not care."]
pub struct CAM_UPDATE_W<'a> {
    w: &'a mut W,
}
impl<'a> CAM_UPDATE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `CAM_BYTE_ORDER` reader - 1: Change data bit order, change CAM_DATA_in\\[7:0\\]
 to CAM_DATA_in\\[0:7\\]
 in one byte mode, and bits\\[15:0\\]
 to bits\\[0:15\\]
 in two byte mode. 0: Not change."]
pub struct CAM_BYTE_ORDER_R(crate::FieldReader<bool, bool>);
impl CAM_BYTE_ORDER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CAM_BYTE_ORDER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAM_BYTE_ORDER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAM_BYTE_ORDER` writer - 1: Change data bit order, change CAM_DATA_in\\[7:0\\]
 to CAM_DATA_in\\[0:7\\]
 in one byte mode, and bits\\[15:0\\]
 to bits\\[0:15\\]
 in two byte mode. 0: Not change."]
pub struct CAM_BYTE_ORDER_W<'a> {
    w: &'a mut W,
}
impl<'a> CAM_BYTE_ORDER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `CAM_BIT_ORDER` reader - 1: invert data byte order, only valid in 2 byte mode. 0: Not change."]
pub struct CAM_BIT_ORDER_R(crate::FieldReader<bool, bool>);
impl CAM_BIT_ORDER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CAM_BIT_ORDER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAM_BIT_ORDER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAM_BIT_ORDER` writer - 1: invert data byte order, only valid in 2 byte mode. 0: Not change."]
pub struct CAM_BIT_ORDER_W<'a> {
    w: &'a mut W,
}
impl<'a> CAM_BIT_ORDER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `CAM_LINE_INT_EN` reader - 1: Enable to generate CAM_HS_INT. 0: Disable."]
pub struct CAM_LINE_INT_EN_R(crate::FieldReader<bool, bool>);
impl CAM_LINE_INT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CAM_LINE_INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAM_LINE_INT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAM_LINE_INT_EN` writer - 1: Enable to generate CAM_HS_INT. 0: Disable."]
pub struct CAM_LINE_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CAM_LINE_INT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `CAM_VS_EOF_EN` reader - 1: CAM_VSYNC to generate in_suc_eof. 0: in_suc_eof is controlled by reg_cam_rec_data_cyclelen."]
pub struct CAM_VS_EOF_EN_R(crate::FieldReader<bool, bool>);
impl CAM_VS_EOF_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CAM_VS_EOF_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAM_VS_EOF_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAM_VS_EOF_EN` writer - 1: CAM_VSYNC to generate in_suc_eof. 0: in_suc_eof is controlled by reg_cam_rec_data_cyclelen."]
pub struct CAM_VS_EOF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CAM_VS_EOF_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `CAM_CLKM_DIV_NUM` reader - Integral Camera clock divider value"]
pub struct CAM_CLKM_DIV_NUM_R(crate::FieldReader<u8, u8>);
impl CAM_CLKM_DIV_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CAM_CLKM_DIV_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAM_CLKM_DIV_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAM_CLKM_DIV_NUM` writer - Integral Camera clock divider value"]
pub struct CAM_CLKM_DIV_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> CAM_CLKM_DIV_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 9)) | ((value as u32 & 0xff) << 9);
        self.w
    }
}
#[doc = "Field `CAM_CLKM_DIV_B` reader - Fractional clock divider numerator value"]
pub struct CAM_CLKM_DIV_B_R(crate::FieldReader<u8, u8>);
impl CAM_CLKM_DIV_B_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CAM_CLKM_DIV_B_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAM_CLKM_DIV_B_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAM_CLKM_DIV_B` writer - Fractional clock divider numerator value"]
pub struct CAM_CLKM_DIV_B_W<'a> {
    w: &'a mut W,
}
impl<'a> CAM_CLKM_DIV_B_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 17)) | ((value as u32 & 0x3f) << 17);
        self.w
    }
}
#[doc = "Field `CAM_CLKM_DIV_A` reader - Fractional clock divider denominator value"]
pub struct CAM_CLKM_DIV_A_R(crate::FieldReader<u8, u8>);
impl CAM_CLKM_DIV_A_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CAM_CLKM_DIV_A_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAM_CLKM_DIV_A_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAM_CLKM_DIV_A` writer - Fractional clock divider denominator value"]
pub struct CAM_CLKM_DIV_A_W<'a> {
    w: &'a mut W,
}
impl<'a> CAM_CLKM_DIV_A_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 23)) | ((value as u32 & 0x3f) << 23);
        self.w
    }
}
#[doc = "Field `CAM_CLK_SEL` reader - Select Camera module source clock. 0: no clock. 1: APLL. 2: CLK160. 3: no clock."]
pub struct CAM_CLK_SEL_R(crate::FieldReader<u8, u8>);
impl CAM_CLK_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CAM_CLK_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAM_CLK_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAM_CLK_SEL` writer - Select Camera module source clock. 0: no clock. 1: APLL. 2: CLK160. 3: no clock."]
pub struct CAM_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CAM_CLK_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 29)) | ((value as u32 & 0x03) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Camera stop enable signal, 1: camera stops when DMA Rx FIFO is full. 0: Not stop."]
    #[inline(always)]
    pub fn cam_stop_en(&self) -> CAM_STOP_EN_R {
        CAM_STOP_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - Filter threshold value for CAM_VSYNC signal."]
    #[inline(always)]
    pub fn cam_vsync_filter_thres(&self) -> CAM_VSYNC_FILTER_THRES_R {
        CAM_VSYNC_FILTER_THRES_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bit 4 - 1: Update Camera registers, will be cleared by hardware. 0 : Not care."]
    #[inline(always)]
    pub fn cam_update(&self) -> CAM_UPDATE_R {
        CAM_UPDATE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 1: Change data bit order, change CAM_DATA_in\\[7:0\\]
 to CAM_DATA_in\\[0:7\\]
 in one byte mode, and bits\\[15:0\\]
 to bits\\[0:15\\]
 in two byte mode. 0: Not change."]
    #[inline(always)]
    pub fn cam_byte_order(&self) -> CAM_BYTE_ORDER_R {
        CAM_BYTE_ORDER_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 1: invert data byte order, only valid in 2 byte mode. 0: Not change."]
    #[inline(always)]
    pub fn cam_bit_order(&self) -> CAM_BIT_ORDER_R {
        CAM_BIT_ORDER_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 1: Enable to generate CAM_HS_INT. 0: Disable."]
    #[inline(always)]
    pub fn cam_line_int_en(&self) -> CAM_LINE_INT_EN_R {
        CAM_LINE_INT_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 1: CAM_VSYNC to generate in_suc_eof. 0: in_suc_eof is controlled by reg_cam_rec_data_cyclelen."]
    #[inline(always)]
    pub fn cam_vs_eof_en(&self) -> CAM_VS_EOF_EN_R {
        CAM_VS_EOF_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 9:16 - Integral Camera clock divider value"]
    #[inline(always)]
    pub fn cam_clkm_div_num(&self) -> CAM_CLKM_DIV_NUM_R {
        CAM_CLKM_DIV_NUM_R::new(((self.bits >> 9) & 0xff) as u8)
    }
    #[doc = "Bits 17:22 - Fractional clock divider numerator value"]
    #[inline(always)]
    pub fn cam_clkm_div_b(&self) -> CAM_CLKM_DIV_B_R {
        CAM_CLKM_DIV_B_R::new(((self.bits >> 17) & 0x3f) as u8)
    }
    #[doc = "Bits 23:28 - Fractional clock divider denominator value"]
    #[inline(always)]
    pub fn cam_clkm_div_a(&self) -> CAM_CLKM_DIV_A_R {
        CAM_CLKM_DIV_A_R::new(((self.bits >> 23) & 0x3f) as u8)
    }
    #[doc = "Bits 29:30 - Select Camera module source clock. 0: no clock. 1: APLL. 2: CLK160. 3: no clock."]
    #[inline(always)]
    pub fn cam_clk_sel(&self) -> CAM_CLK_SEL_R {
        CAM_CLK_SEL_R::new(((self.bits >> 29) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Camera stop enable signal, 1: camera stops when DMA Rx FIFO is full. 0: Not stop."]
    #[inline(always)]
    pub fn cam_stop_en(&mut self) -> CAM_STOP_EN_W {
        CAM_STOP_EN_W { w: self }
    }
    #[doc = "Bits 1:3 - Filter threshold value for CAM_VSYNC signal."]
    #[inline(always)]
    pub fn cam_vsync_filter_thres(&mut self) -> CAM_VSYNC_FILTER_THRES_W {
        CAM_VSYNC_FILTER_THRES_W { w: self }
    }
    #[doc = "Bit 4 - 1: Update Camera registers, will be cleared by hardware. 0 : Not care."]
    #[inline(always)]
    pub fn cam_update(&mut self) -> CAM_UPDATE_W {
        CAM_UPDATE_W { w: self }
    }
    #[doc = "Bit 5 - 1: Change data bit order, change CAM_DATA_in\\[7:0\\]
 to CAM_DATA_in\\[0:7\\]
 in one byte mode, and bits\\[15:0\\]
 to bits\\[0:15\\]
 in two byte mode. 0: Not change."]
    #[inline(always)]
    pub fn cam_byte_order(&mut self) -> CAM_BYTE_ORDER_W {
        CAM_BYTE_ORDER_W { w: self }
    }
    #[doc = "Bit 6 - 1: invert data byte order, only valid in 2 byte mode. 0: Not change."]
    #[inline(always)]
    pub fn cam_bit_order(&mut self) -> CAM_BIT_ORDER_W {
        CAM_BIT_ORDER_W { w: self }
    }
    #[doc = "Bit 7 - 1: Enable to generate CAM_HS_INT. 0: Disable."]
    #[inline(always)]
    pub fn cam_line_int_en(&mut self) -> CAM_LINE_INT_EN_W {
        CAM_LINE_INT_EN_W { w: self }
    }
    #[doc = "Bit 8 - 1: CAM_VSYNC to generate in_suc_eof. 0: in_suc_eof is controlled by reg_cam_rec_data_cyclelen."]
    #[inline(always)]
    pub fn cam_vs_eof_en(&mut self) -> CAM_VS_EOF_EN_W {
        CAM_VS_EOF_EN_W { w: self }
    }
    #[doc = "Bits 9:16 - Integral Camera clock divider value"]
    #[inline(always)]
    pub fn cam_clkm_div_num(&mut self) -> CAM_CLKM_DIV_NUM_W {
        CAM_CLKM_DIV_NUM_W { w: self }
    }
    #[doc = "Bits 17:22 - Fractional clock divider numerator value"]
    #[inline(always)]
    pub fn cam_clkm_div_b(&mut self) -> CAM_CLKM_DIV_B_W {
        CAM_CLKM_DIV_B_W { w: self }
    }
    #[doc = "Bits 23:28 - Fractional clock divider denominator value"]
    #[inline(always)]
    pub fn cam_clkm_div_a(&mut self) -> CAM_CLKM_DIV_A_W {
        CAM_CLKM_DIV_A_W { w: self }
    }
    #[doc = "Bits 29:30 - Select Camera module source clock. 0: no clock. 1: APLL. 2: CLK160. 3: no clock."]
    #[inline(always)]
    pub fn cam_clk_sel(&mut self) -> CAM_CLK_SEL_W {
        CAM_CLK_SEL_W { w: self }
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
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cam_ctrl]
(index.html) module"]
pub struct CAM_CTRL_SPEC;
impl crate::RegisterSpec for CAM_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cam_ctrl::R]
(R) reader structure"]
impl crate::Readable for CAM_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cam_ctrl::W]
(W) writer structure"]
impl crate::Writable for CAM_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAM_CTRL to value 0x0800"]
impl crate::Resettable for CAM_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0800
    }
}
