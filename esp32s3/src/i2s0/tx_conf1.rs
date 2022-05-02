#[doc = "Register `TX_CONF1` reader"]
pub struct R(crate::R<TX_CONF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_CONF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_CONF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_CONF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX_CONF1` writer"]
pub struct W(crate::W<TX_CONF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_CONF1_SPEC>;
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
impl From<crate::W<TX_CONF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_CONF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_TDM_WS_WIDTH` reader - The width of tx_ws_out in TDM mode is (I2S_TX_TDM_WS_WIDTH\\[6:0\\] +1) * T_bck"]
pub struct TX_TDM_WS_WIDTH_R(crate::FieldReader<u8>);
impl TX_TDM_WS_WIDTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TX_TDM_WS_WIDTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_TDM_WS_WIDTH_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_TDM_WS_WIDTH` writer - The width of tx_ws_out in TDM mode is (I2S_TX_TDM_WS_WIDTH\\[6:0\\] +1) * T_bck"]
pub struct TX_TDM_WS_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_TDM_WS_WIDTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
#[doc = "Field `TX_BCK_DIV_NUM` reader - Bit clock configuration bits in transmitter mode."]
pub struct TX_BCK_DIV_NUM_R(crate::FieldReader<u8>);
impl TX_BCK_DIV_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TX_BCK_DIV_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_BCK_DIV_NUM_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_BCK_DIV_NUM` writer - Bit clock configuration bits in transmitter mode."]
pub struct TX_BCK_DIV_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_BCK_DIV_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 7)) | ((value as u32 & 0x3f) << 7);
        self.w
    }
}
#[doc = "Field `TX_BITS_MOD` reader - Set the bits to configure the valid data bit length of I2S transmitter channel. 7: all the valid channel data is in 8-bit-mode. 15: all the valid channel data is in 16-bit-mode. 23: all the valid channel data is in 24-bit-mode. 31:all the valid channel data is in 32-bit-mode."]
pub struct TX_BITS_MOD_R(crate::FieldReader<u8>);
impl TX_BITS_MOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TX_BITS_MOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_BITS_MOD_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_BITS_MOD` writer - Set the bits to configure the valid data bit length of I2S transmitter channel. 7: all the valid channel data is in 8-bit-mode. 15: all the valid channel data is in 16-bit-mode. 23: all the valid channel data is in 24-bit-mode. 31:all the valid channel data is in 32-bit-mode."]
pub struct TX_BITS_MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_BITS_MOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 13)) | ((value as u32 & 0x1f) << 13);
        self.w
    }
}
#[doc = "Field `TX_HALF_SAMPLE_BITS` reader - I2S Tx half sample bits -1."]
pub struct TX_HALF_SAMPLE_BITS_R(crate::FieldReader<u8>);
impl TX_HALF_SAMPLE_BITS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TX_HALF_SAMPLE_BITS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_HALF_SAMPLE_BITS_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_HALF_SAMPLE_BITS` writer - I2S Tx half sample bits -1."]
pub struct TX_HALF_SAMPLE_BITS_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_HALF_SAMPLE_BITS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 18)) | ((value as u32 & 0x3f) << 18);
        self.w
    }
}
#[doc = "Field `TX_TDM_CHAN_BITS` reader - The Tx bit number for each channel minus 1in TDM mode."]
pub struct TX_TDM_CHAN_BITS_R(crate::FieldReader<u8>);
impl TX_TDM_CHAN_BITS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TX_TDM_CHAN_BITS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_TDM_CHAN_BITS_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_TDM_CHAN_BITS` writer - The Tx bit number for each channel minus 1in TDM mode."]
pub struct TX_TDM_CHAN_BITS_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_TDM_CHAN_BITS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | ((value as u32 & 0x1f) << 24);
        self.w
    }
}
#[doc = "Field `TX_MSB_SHIFT` reader - Set this bit to enable transmitter in Phillips standard mode"]
pub struct TX_MSB_SHIFT_R(crate::FieldReader<bool>);
impl TX_MSB_SHIFT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_MSB_SHIFT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_MSB_SHIFT_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_MSB_SHIFT` writer - Set this bit to enable transmitter in Phillips standard mode"]
pub struct TX_MSB_SHIFT_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_MSB_SHIFT_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 29)) | ((value as u32 & 1) << 29);
        self.w
    }
}
#[doc = "Field `TX_BCK_NO_DLY` reader - 1: BCK is not delayed to generate pos/neg edge in master mode. 0: BCK is delayed to generate pos/neg edge in master mode."]
pub struct TX_BCK_NO_DLY_R(crate::FieldReader<bool>);
impl TX_BCK_NO_DLY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_BCK_NO_DLY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_BCK_NO_DLY_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_BCK_NO_DLY` writer - 1: BCK is not delayed to generate pos/neg edge in master mode. 0: BCK is delayed to generate pos/neg edge in master mode."]
pub struct TX_BCK_NO_DLY_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_BCK_NO_DLY_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 30)) | ((value as u32 & 1) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - The width of tx_ws_out in TDM mode is (I2S_TX_TDM_WS_WIDTH\\[6:0\\] +1) * T_bck"]
    #[inline(always)]
    pub fn tx_tdm_ws_width(&self) -> TX_TDM_WS_WIDTH_R {
        TX_TDM_WS_WIDTH_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:12 - Bit clock configuration bits in transmitter mode."]
    #[inline(always)]
    pub fn tx_bck_div_num(&self) -> TX_BCK_DIV_NUM_R {
        TX_BCK_DIV_NUM_R::new(((self.bits >> 7) & 0x3f) as u8)
    }
    #[doc = "Bits 13:17 - Set the bits to configure the valid data bit length of I2S transmitter channel. 7: all the valid channel data is in 8-bit-mode. 15: all the valid channel data is in 16-bit-mode. 23: all the valid channel data is in 24-bit-mode. 31:all the valid channel data is in 32-bit-mode."]
    #[inline(always)]
    pub fn tx_bits_mod(&self) -> TX_BITS_MOD_R {
        TX_BITS_MOD_R::new(((self.bits >> 13) & 0x1f) as u8)
    }
    #[doc = "Bits 18:23 - I2S Tx half sample bits -1."]
    #[inline(always)]
    pub fn tx_half_sample_bits(&self) -> TX_HALF_SAMPLE_BITS_R {
        TX_HALF_SAMPLE_BITS_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 24:28 - The Tx bit number for each channel minus 1in TDM mode."]
    #[inline(always)]
    pub fn tx_tdm_chan_bits(&self) -> TX_TDM_CHAN_BITS_R {
        TX_TDM_CHAN_BITS_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 29 - Set this bit to enable transmitter in Phillips standard mode"]
    #[inline(always)]
    pub fn tx_msb_shift(&self) -> TX_MSB_SHIFT_R {
        TX_MSB_SHIFT_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 1: BCK is not delayed to generate pos/neg edge in master mode. 0: BCK is delayed to generate pos/neg edge in master mode."]
    #[inline(always)]
    pub fn tx_bck_no_dly(&self) -> TX_BCK_NO_DLY_R {
        TX_BCK_NO_DLY_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - The width of tx_ws_out in TDM mode is (I2S_TX_TDM_WS_WIDTH\\[6:0\\] +1) * T_bck"]
    #[inline(always)]
    pub fn tx_tdm_ws_width(&mut self) -> TX_TDM_WS_WIDTH_W {
        TX_TDM_WS_WIDTH_W { w: self }
    }
    #[doc = "Bits 7:12 - Bit clock configuration bits in transmitter mode."]
    #[inline(always)]
    pub fn tx_bck_div_num(&mut self) -> TX_BCK_DIV_NUM_W {
        TX_BCK_DIV_NUM_W { w: self }
    }
    #[doc = "Bits 13:17 - Set the bits to configure the valid data bit length of I2S transmitter channel. 7: all the valid channel data is in 8-bit-mode. 15: all the valid channel data is in 16-bit-mode. 23: all the valid channel data is in 24-bit-mode. 31:all the valid channel data is in 32-bit-mode."]
    #[inline(always)]
    pub fn tx_bits_mod(&mut self) -> TX_BITS_MOD_W {
        TX_BITS_MOD_W { w: self }
    }
    #[doc = "Bits 18:23 - I2S Tx half sample bits -1."]
    #[inline(always)]
    pub fn tx_half_sample_bits(&mut self) -> TX_HALF_SAMPLE_BITS_W {
        TX_HALF_SAMPLE_BITS_W { w: self }
    }
    #[doc = "Bits 24:28 - The Tx bit number for each channel minus 1in TDM mode."]
    #[inline(always)]
    pub fn tx_tdm_chan_bits(&mut self) -> TX_TDM_CHAN_BITS_W {
        TX_TDM_CHAN_BITS_W { w: self }
    }
    #[doc = "Bit 29 - Set this bit to enable transmitter in Phillips standard mode"]
    #[inline(always)]
    pub fn tx_msb_shift(&mut self) -> TX_MSB_SHIFT_W {
        TX_MSB_SHIFT_W { w: self }
    }
    #[doc = "Bit 30 - 1: BCK is not delayed to generate pos/neg edge in master mode. 0: BCK is delayed to generate pos/neg edge in master mode."]
    #[inline(always)]
    pub fn tx_bck_no_dly(&mut self) -> TX_BCK_NO_DLY_W {
        TX_BCK_NO_DLY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S TX configure register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_conf1](index.html) module"]
pub struct TX_CONF1_SPEC;
impl crate::RegisterSpec for TX_CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_conf1::R](R) reader structure"]
impl crate::Readable for TX_CONF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_conf1::W](W) writer structure"]
impl crate::Writable for TX_CONF1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TX_CONF1 to value 0x6f3d_e300"]
impl crate::Resettable for TX_CONF1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x6f3d_e300
    }
}
