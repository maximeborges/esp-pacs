#[doc = "Register `RX_CONF` reader"]
pub struct R(crate::R<RX_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RX_CONF` writer"]
pub struct W(crate::W<RX_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_CONF_SPEC>;
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
impl From<crate::W<RX_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_RESET` writer - Set this bit to reset receiver"]
pub struct RX_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_RESET_W<'a> {
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
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
#[doc = "Field `RX_FIFO_RESET` writer - Set this bit to reset Rx AFIFO"]
pub struct RX_FIFO_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FIFO_RESET_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u32 & 1) << 1);
        self.w
    }
}
#[doc = "Field `RX_START` reader - Set this bit to start receiving data"]
pub struct RX_START_R(crate::FieldReader<bool>);
impl RX_START_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_START_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_START` writer - Set this bit to start receiving data"]
pub struct RX_START_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_START_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u32 & 1) << 2);
        self.w
    }
}
#[doc = "Field `RX_SLAVE_MOD` reader - Set this bit to enable slave receiver mode"]
pub struct RX_SLAVE_MOD_R(crate::FieldReader<bool>);
impl RX_SLAVE_MOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_SLAVE_MOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_SLAVE_MOD_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_SLAVE_MOD` writer - Set this bit to enable slave receiver mode"]
pub struct RX_SLAVE_MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_SLAVE_MOD_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u32 & 1) << 3);
        self.w
    }
}
#[doc = "Field `RX_MONO` reader - Set this bit to enable receiver in mono mode"]
pub struct RX_MONO_R(crate::FieldReader<bool>);
impl RX_MONO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_MONO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_MONO_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_MONO` writer - Set this bit to enable receiver in mono mode"]
pub struct RX_MONO_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_MONO_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u32 & 1) << 5);
        self.w
    }
}
#[doc = "Field `RX_BIG_ENDIAN` reader - I2S Rx byte endian, 1: low addr value to high addr. 0: low addr with low addr value."]
pub struct RX_BIG_ENDIAN_R(crate::FieldReader<bool>);
impl RX_BIG_ENDIAN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_BIG_ENDIAN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_BIG_ENDIAN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_BIG_ENDIAN` writer - I2S Rx byte endian, 1: low addr value to high addr. 0: low addr with low addr value."]
pub struct RX_BIG_ENDIAN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_BIG_ENDIAN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 7)) | ((value as u32 & 1) << 7);
        self.w
    }
}
#[doc = "Field `RX_UPDATE` reader - Set 1 to update I2S RX registers from APB clock domain to I2S RX clock domain. This bit will be cleared by hardware after update register done."]
pub struct RX_UPDATE_R(crate::FieldReader<bool>);
impl RX_UPDATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_UPDATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_UPDATE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_UPDATE` writer - Set 1 to update I2S RX registers from APB clock domain to I2S RX clock domain. This bit will be cleared by hardware after update register done."]
pub struct RX_UPDATE_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_UPDATE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 8)) | ((value as u32 & 1) << 8);
        self.w
    }
}
#[doc = "Field `RX_MONO_FST_VLD` reader - 1: The first channel data value is valid in I2S RX mono mode. 0: The second channel data value is valid in I2S RX mono mode."]
pub struct RX_MONO_FST_VLD_R(crate::FieldReader<bool>);
impl RX_MONO_FST_VLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_MONO_FST_VLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_MONO_FST_VLD_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_MONO_FST_VLD` writer - 1: The first channel data value is valid in I2S RX mono mode. 0: The second channel data value is valid in I2S RX mono mode."]
pub struct RX_MONO_FST_VLD_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_MONO_FST_VLD_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 9)) | ((value as u32 & 1) << 9);
        self.w
    }
}
#[doc = "Field `RX_PCM_CONF` reader - I2S RX compress/decompress configuration bit. & 0 (atol): A-Law decompress, 1 (ltoa) : A-Law compress, 2 (utol) : u-Law decompress, 3 (ltou) : u-Law compress. &"]
pub struct RX_PCM_CONF_R(crate::FieldReader<u8>);
impl RX_PCM_CONF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RX_PCM_CONF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_PCM_CONF_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_PCM_CONF` writer - I2S RX compress/decompress configuration bit. & 0 (atol): A-Law decompress, 1 (ltoa) : A-Law compress, 2 (utol) : u-Law decompress, 3 (ltou) : u-Law compress. &"]
pub struct RX_PCM_CONF_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_PCM_CONF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 10)) | ((value as u32 & 3) << 10);
        self.w
    }
}
#[doc = "Field `RX_PCM_BYPASS` reader - Set this bit to bypass Compress/Decompress module for received data."]
pub struct RX_PCM_BYPASS_R(crate::FieldReader<bool>);
impl RX_PCM_BYPASS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_PCM_BYPASS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_PCM_BYPASS_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_PCM_BYPASS` writer - Set this bit to bypass Compress/Decompress module for received data."]
pub struct RX_PCM_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_PCM_BYPASS_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 12)) | ((value as u32 & 1) << 12);
        self.w
    }
}
#[doc = "Field `RX_STOP_MODE` reader - 0 : I2S Rx only stop when reg_rx_start is cleared. 1: Stop when reg_rx_start is 0 or in_suc_eof is 1. 2: Stop I2S RX when reg_rx_start is 0 or RX FIFO is full."]
pub struct RX_STOP_MODE_R(crate::FieldReader<u8>);
impl RX_STOP_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RX_STOP_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_STOP_MODE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_STOP_MODE` writer - 0 : I2S Rx only stop when reg_rx_start is cleared. 1: Stop when reg_rx_start is 0 or in_suc_eof is 1. 2: Stop I2S RX when reg_rx_start is 0 or RX FIFO is full."]
pub struct RX_STOP_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_STOP_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 13)) | ((value as u32 & 3) << 13);
        self.w
    }
}
#[doc = "Field `RX_LEFT_ALIGN` reader - 1: I2S RX left alignment mode. 0: I2S RX right alignment mode."]
pub struct RX_LEFT_ALIGN_R(crate::FieldReader<bool>);
impl RX_LEFT_ALIGN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_LEFT_ALIGN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_LEFT_ALIGN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_LEFT_ALIGN` writer - 1: I2S RX left alignment mode. 0: I2S RX right alignment mode."]
pub struct RX_LEFT_ALIGN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_LEFT_ALIGN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 15)) | ((value as u32 & 1) << 15);
        self.w
    }
}
#[doc = "Field `RX_24_FILL_EN` reader - 1: store 24 channel bits to 32 bits. 0:store 24 channel bits to 24 bits."]
pub struct RX_24_FILL_EN_R(crate::FieldReader<bool>);
impl RX_24_FILL_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_24_FILL_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_24_FILL_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_24_FILL_EN` writer - 1: store 24 channel bits to 32 bits. 0:store 24 channel bits to 24 bits."]
pub struct RX_24_FILL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_24_FILL_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 16)) | ((value as u32 & 1) << 16);
        self.w
    }
}
#[doc = "Field `RX_WS_IDLE_POL` reader - 0: WS should be 0 when receiving left channel data, and WS is 1in right channel. 1: WS should be 1 when receiving left channel data, and WS is 0in right channel."]
pub struct RX_WS_IDLE_POL_R(crate::FieldReader<bool>);
impl RX_WS_IDLE_POL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_WS_IDLE_POL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_WS_IDLE_POL_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_WS_IDLE_POL` writer - 0: WS should be 0 when receiving left channel data, and WS is 1in right channel. 1: WS should be 1 when receiving left channel data, and WS is 0in right channel."]
pub struct RX_WS_IDLE_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_WS_IDLE_POL_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 17)) | ((value as u32 & 1) << 17);
        self.w
    }
}
#[doc = "Field `RX_BIT_ORDER` reader - I2S Rx bit endian. 1:small endian, the LSB is received first. 0:big endian, the MSB is received first."]
pub struct RX_BIT_ORDER_R(crate::FieldReader<bool>);
impl RX_BIT_ORDER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_BIT_ORDER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_BIT_ORDER_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_BIT_ORDER` writer - I2S Rx bit endian. 1:small endian, the LSB is received first. 0:big endian, the MSB is received first."]
pub struct RX_BIT_ORDER_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_BIT_ORDER_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 18)) | ((value as u32 & 1) << 18);
        self.w
    }
}
#[doc = "Field `RX_TDM_EN` reader - 1: Enable I2S TDM Rx mode . 0: Disable."]
pub struct RX_TDM_EN_R(crate::FieldReader<bool>);
impl RX_TDM_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_TDM_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_TDM_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_TDM_EN` writer - 1: Enable I2S TDM Rx mode . 0: Disable."]
pub struct RX_TDM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_TDM_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 19)) | ((value as u32 & 1) << 19);
        self.w
    }
}
#[doc = "Field `RX_PDM_EN` reader - 1: Enable I2S PDM Rx mode . 0: Disable."]
pub struct RX_PDM_EN_R(crate::FieldReader<bool>);
impl RX_PDM_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_PDM_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_PDM_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_PDM_EN` writer - 1: Enable I2S PDM Rx mode . 0: Disable."]
pub struct RX_PDM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_PDM_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 20)) | ((value as u32 & 1) << 20);
        self.w
    }
}
#[doc = "Field `RX_PDM2PCM_EN` reader - 1: Enable PDM2PCM RX mode. 0: DIsable."]
pub struct RX_PDM2PCM_EN_R(crate::FieldReader<bool>);
impl RX_PDM2PCM_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_PDM2PCM_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_PDM2PCM_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_PDM2PCM_EN` writer - 1: Enable PDM2PCM RX mode. 0: DIsable."]
pub struct RX_PDM2PCM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_PDM2PCM_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 21)) | ((value as u32 & 1) << 21);
        self.w
    }
}
#[doc = "Field `RX_PDM_SINC_DSR_16_EN` reader - Configure the down sampling rate of PDM RX filter group1 module. 1: The down sampling rate is 128. 0: down sampling rate is 64."]
pub struct RX_PDM_SINC_DSR_16_EN_R(crate::FieldReader<bool>);
impl RX_PDM_SINC_DSR_16_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_PDM_SINC_DSR_16_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_PDM_SINC_DSR_16_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_PDM_SINC_DSR_16_EN` writer - Configure the down sampling rate of PDM RX filter group1 module. 1: The down sampling rate is 128. 0: down sampling rate is 64."]
pub struct RX_PDM_SINC_DSR_16_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_PDM_SINC_DSR_16_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 22)) | ((value as u32 & 1) << 22);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - Set this bit to start receiving data"]
    #[inline(always)]
    pub fn rx_start(&self) -> RX_START_R {
        RX_START_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to enable slave receiver mode"]
    #[inline(always)]
    pub fn rx_slave_mod(&self) -> RX_SLAVE_MOD_R {
        RX_SLAVE_MOD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Set this bit to enable receiver in mono mode"]
    #[inline(always)]
    pub fn rx_mono(&self) -> RX_MONO_R {
        RX_MONO_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - I2S Rx byte endian, 1: low addr value to high addr. 0: low addr with low addr value."]
    #[inline(always)]
    pub fn rx_big_endian(&self) -> RX_BIG_ENDIAN_R {
        RX_BIG_ENDIAN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Set 1 to update I2S RX registers from APB clock domain to I2S RX clock domain. This bit will be cleared by hardware after update register done."]
    #[inline(always)]
    pub fn rx_update(&self) -> RX_UPDATE_R {
        RX_UPDATE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 1: The first channel data value is valid in I2S RX mono mode. 0: The second channel data value is valid in I2S RX mono mode."]
    #[inline(always)]
    pub fn rx_mono_fst_vld(&self) -> RX_MONO_FST_VLD_R {
        RX_MONO_FST_VLD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - I2S RX compress/decompress configuration bit. & 0 (atol): A-Law decompress, 1 (ltoa) : A-Law compress, 2 (utol) : u-Law decompress, 3 (ltou) : u-Law compress. &"]
    #[inline(always)]
    pub fn rx_pcm_conf(&self) -> RX_PCM_CONF_R {
        RX_PCM_CONF_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Set this bit to bypass Compress/Decompress module for received data."]
    #[inline(always)]
    pub fn rx_pcm_bypass(&self) -> RX_PCM_BYPASS_R {
        RX_PCM_BYPASS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - 0 : I2S Rx only stop when reg_rx_start is cleared. 1: Stop when reg_rx_start is 0 or in_suc_eof is 1. 2: Stop I2S RX when reg_rx_start is 0 or RX FIFO is full."]
    #[inline(always)]
    pub fn rx_stop_mode(&self) -> RX_STOP_MODE_R {
        RX_STOP_MODE_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - 1: I2S RX left alignment mode. 0: I2S RX right alignment mode."]
    #[inline(always)]
    pub fn rx_left_align(&self) -> RX_LEFT_ALIGN_R {
        RX_LEFT_ALIGN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 1: store 24 channel bits to 32 bits. 0:store 24 channel bits to 24 bits."]
    #[inline(always)]
    pub fn rx_24_fill_en(&self) -> RX_24_FILL_EN_R {
        RX_24_FILL_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 0: WS should be 0 when receiving left channel data, and WS is 1in right channel. 1: WS should be 1 when receiving left channel data, and WS is 0in right channel."]
    #[inline(always)]
    pub fn rx_ws_idle_pol(&self) -> RX_WS_IDLE_POL_R {
        RX_WS_IDLE_POL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - I2S Rx bit endian. 1:small endian, the LSB is received first. 0:big endian, the MSB is received first."]
    #[inline(always)]
    pub fn rx_bit_order(&self) -> RX_BIT_ORDER_R {
        RX_BIT_ORDER_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 1: Enable I2S TDM Rx mode . 0: Disable."]
    #[inline(always)]
    pub fn rx_tdm_en(&self) -> RX_TDM_EN_R {
        RX_TDM_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 1: Enable I2S PDM Rx mode . 0: Disable."]
    #[inline(always)]
    pub fn rx_pdm_en(&self) -> RX_PDM_EN_R {
        RX_PDM_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 1: Enable PDM2PCM RX mode. 0: DIsable."]
    #[inline(always)]
    pub fn rx_pdm2pcm_en(&self) -> RX_PDM2PCM_EN_R {
        RX_PDM2PCM_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Configure the down sampling rate of PDM RX filter group1 module. 1: The down sampling rate is 128. 0: down sampling rate is 64."]
    #[inline(always)]
    pub fn rx_pdm_sinc_dsr_16_en(&self) -> RX_PDM_SINC_DSR_16_EN_R {
        RX_PDM_SINC_DSR_16_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to reset receiver"]
    #[inline(always)]
    pub fn rx_reset(&mut self) -> RX_RESET_W {
        RX_RESET_W { w: self }
    }
    #[doc = "Bit 1 - Set this bit to reset Rx AFIFO"]
    #[inline(always)]
    pub fn rx_fifo_reset(&mut self) -> RX_FIFO_RESET_W {
        RX_FIFO_RESET_W { w: self }
    }
    #[doc = "Bit 2 - Set this bit to start receiving data"]
    #[inline(always)]
    pub fn rx_start(&mut self) -> RX_START_W {
        RX_START_W { w: self }
    }
    #[doc = "Bit 3 - Set this bit to enable slave receiver mode"]
    #[inline(always)]
    pub fn rx_slave_mod(&mut self) -> RX_SLAVE_MOD_W {
        RX_SLAVE_MOD_W { w: self }
    }
    #[doc = "Bit 5 - Set this bit to enable receiver in mono mode"]
    #[inline(always)]
    pub fn rx_mono(&mut self) -> RX_MONO_W {
        RX_MONO_W { w: self }
    }
    #[doc = "Bit 7 - I2S Rx byte endian, 1: low addr value to high addr. 0: low addr with low addr value."]
    #[inline(always)]
    pub fn rx_big_endian(&mut self) -> RX_BIG_ENDIAN_W {
        RX_BIG_ENDIAN_W { w: self }
    }
    #[doc = "Bit 8 - Set 1 to update I2S RX registers from APB clock domain to I2S RX clock domain. This bit will be cleared by hardware after update register done."]
    #[inline(always)]
    pub fn rx_update(&mut self) -> RX_UPDATE_W {
        RX_UPDATE_W { w: self }
    }
    #[doc = "Bit 9 - 1: The first channel data value is valid in I2S RX mono mode. 0: The second channel data value is valid in I2S RX mono mode."]
    #[inline(always)]
    pub fn rx_mono_fst_vld(&mut self) -> RX_MONO_FST_VLD_W {
        RX_MONO_FST_VLD_W { w: self }
    }
    #[doc = "Bits 10:11 - I2S RX compress/decompress configuration bit. & 0 (atol): A-Law decompress, 1 (ltoa) : A-Law compress, 2 (utol) : u-Law decompress, 3 (ltou) : u-Law compress. &"]
    #[inline(always)]
    pub fn rx_pcm_conf(&mut self) -> RX_PCM_CONF_W {
        RX_PCM_CONF_W { w: self }
    }
    #[doc = "Bit 12 - Set this bit to bypass Compress/Decompress module for received data."]
    #[inline(always)]
    pub fn rx_pcm_bypass(&mut self) -> RX_PCM_BYPASS_W {
        RX_PCM_BYPASS_W { w: self }
    }
    #[doc = "Bits 13:14 - 0 : I2S Rx only stop when reg_rx_start is cleared. 1: Stop when reg_rx_start is 0 or in_suc_eof is 1. 2: Stop I2S RX when reg_rx_start is 0 or RX FIFO is full."]
    #[inline(always)]
    pub fn rx_stop_mode(&mut self) -> RX_STOP_MODE_W {
        RX_STOP_MODE_W { w: self }
    }
    #[doc = "Bit 15 - 1: I2S RX left alignment mode. 0: I2S RX right alignment mode."]
    #[inline(always)]
    pub fn rx_left_align(&mut self) -> RX_LEFT_ALIGN_W {
        RX_LEFT_ALIGN_W { w: self }
    }
    #[doc = "Bit 16 - 1: store 24 channel bits to 32 bits. 0:store 24 channel bits to 24 bits."]
    #[inline(always)]
    pub fn rx_24_fill_en(&mut self) -> RX_24_FILL_EN_W {
        RX_24_FILL_EN_W { w: self }
    }
    #[doc = "Bit 17 - 0: WS should be 0 when receiving left channel data, and WS is 1in right channel. 1: WS should be 1 when receiving left channel data, and WS is 0in right channel."]
    #[inline(always)]
    pub fn rx_ws_idle_pol(&mut self) -> RX_WS_IDLE_POL_W {
        RX_WS_IDLE_POL_W { w: self }
    }
    #[doc = "Bit 18 - I2S Rx bit endian. 1:small endian, the LSB is received first. 0:big endian, the MSB is received first."]
    #[inline(always)]
    pub fn rx_bit_order(&mut self) -> RX_BIT_ORDER_W {
        RX_BIT_ORDER_W { w: self }
    }
    #[doc = "Bit 19 - 1: Enable I2S TDM Rx mode . 0: Disable."]
    #[inline(always)]
    pub fn rx_tdm_en(&mut self) -> RX_TDM_EN_W {
        RX_TDM_EN_W { w: self }
    }
    #[doc = "Bit 20 - 1: Enable I2S PDM Rx mode . 0: Disable."]
    #[inline(always)]
    pub fn rx_pdm_en(&mut self) -> RX_PDM_EN_W {
        RX_PDM_EN_W { w: self }
    }
    #[doc = "Bit 21 - 1: Enable PDM2PCM RX mode. 0: DIsable."]
    #[inline(always)]
    pub fn rx_pdm2pcm_en(&mut self) -> RX_PDM2PCM_EN_W {
        RX_PDM2PCM_EN_W { w: self }
    }
    #[doc = "Bit 22 - Configure the down sampling rate of PDM RX filter group1 module. 1: The down sampling rate is 128. 0: down sampling rate is 64."]
    #[inline(always)]
    pub fn rx_pdm_sinc_dsr_16_en(&mut self) -> RX_PDM_SINC_DSR_16_EN_W {
        RX_PDM_SINC_DSR_16_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S RX configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_conf](index.html) module"]
pub struct RX_CONF_SPEC;
impl crate::RegisterSpec for RX_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_conf::R](R) reader structure"]
impl crate::Readable for RX_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx_conf::W](W) writer structure"]
impl crate::Writable for RX_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RX_CONF to value 0x9600"]
impl crate::Resettable for RX_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x9600
    }
}
