#[doc = "Register `CONF` reader"]
pub struct R(crate::R<CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONF` writer"]
pub struct W(crate::W<CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONF_SPEC>;
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
impl From<crate::W<CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_RESET` writer - Set this bit to reset transmitter."]
pub struct TX_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_RESET_W<'a> {
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
#[doc = "Field `RX_RESET` writer - Set this bit to reset receiver."]
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
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u32 & 1) << 1);
        self.w
    }
}
#[doc = "Field `TX_FIFO_RESET` writer - Set this bit to reset TX FIFO."]
pub struct TX_FIFO_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FIFO_RESET_W<'a> {
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
#[doc = "Field `RX_FIFO_RESET` writer - Set this bit to reset RX FIFO."]
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
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u32 & 1) << 3);
        self.w
    }
}
#[doc = "Field `TX_START` reader - Set this bit to start transmitting data."]
pub struct TX_START_R(crate::FieldReader<bool, bool>);
impl TX_START_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_START_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_START` writer - Set this bit to start transmitting data."]
pub struct TX_START_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_START_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u32 & 1) << 4);
        self.w
    }
}
#[doc = "Field `RX_START` reader - Set this bit to start receiving data."]
pub struct RX_START_R(crate::FieldReader<bool, bool>);
impl RX_START_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_START_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_START` writer - Set this bit to start receiving data."]
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
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u32 & 1) << 5);
        self.w
    }
}
#[doc = "Field `TX_SLAVE_MOD` reader - Set this bit to enable slave transmitter mode."]
pub struct TX_SLAVE_MOD_R(crate::FieldReader<bool, bool>);
impl TX_SLAVE_MOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_SLAVE_MOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_SLAVE_MOD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_SLAVE_MOD` writer - Set this bit to enable slave transmitter mode."]
pub struct TX_SLAVE_MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_SLAVE_MOD_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 6)) | ((value as u32 & 1) << 6);
        self.w
    }
}
#[doc = "Field `RX_SLAVE_MOD` reader - Set this bit to enable slave receiver mode."]
pub struct RX_SLAVE_MOD_R(crate::FieldReader<bool, bool>);
impl RX_SLAVE_MOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_SLAVE_MOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_SLAVE_MOD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_SLAVE_MOD` writer - Set this bit to enable slave receiver mode."]
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
        self.w.bits = (self.w.bits & !(1 << 7)) | ((value as u32 & 1) << 7);
        self.w
    }
}
#[doc = "Field `TX_RIGHT_FIRST` reader - Set this bit to transmit right channel data first."]
pub struct TX_RIGHT_FIRST_R(crate::FieldReader<bool, bool>);
impl TX_RIGHT_FIRST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_RIGHT_FIRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_RIGHT_FIRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_RIGHT_FIRST` writer - Set this bit to transmit right channel data first."]
pub struct TX_RIGHT_FIRST_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_RIGHT_FIRST_W<'a> {
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
#[doc = "Field `RX_RIGHT_FIRST` reader - Set this bit to receive right channel data first."]
pub struct RX_RIGHT_FIRST_R(crate::FieldReader<bool, bool>);
impl RX_RIGHT_FIRST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_RIGHT_FIRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_RIGHT_FIRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_RIGHT_FIRST` writer - Set this bit to receive right channel data first."]
pub struct RX_RIGHT_FIRST_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_RIGHT_FIRST_W<'a> {
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
#[doc = "Field `TX_MSB_SHIFT` reader - Set this bit to enable transmitter in Phillips standard mode."]
pub struct TX_MSB_SHIFT_R(crate::FieldReader<bool, bool>);
impl TX_MSB_SHIFT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_MSB_SHIFT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_MSB_SHIFT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_MSB_SHIFT` writer - Set this bit to enable transmitter in Phillips standard mode."]
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
        self.w.bits = (self.w.bits & !(1 << 10)) | ((value as u32 & 1) << 10);
        self.w
    }
}
#[doc = "Field `RX_MSB_SHIFT` reader - Set this bit to enable receiver in Phillips standard mode."]
pub struct RX_MSB_SHIFT_R(crate::FieldReader<bool, bool>);
impl RX_MSB_SHIFT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_MSB_SHIFT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_MSB_SHIFT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_MSB_SHIFT` writer - Set this bit to enable receiver in Phillips standard mode."]
pub struct RX_MSB_SHIFT_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_MSB_SHIFT_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 11)) | ((value as u32 & 1) << 11);
        self.w
    }
}
#[doc = "Field `TX_SHORT_SYNC` reader - Set this bit to enable transmitter in PCM standard mode."]
pub struct TX_SHORT_SYNC_R(crate::FieldReader<bool, bool>);
impl TX_SHORT_SYNC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_SHORT_SYNC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_SHORT_SYNC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_SHORT_SYNC` writer - Set this bit to enable transmitter in PCM standard mode."]
pub struct TX_SHORT_SYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_SHORT_SYNC_W<'a> {
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
#[doc = "Field `RX_SHORT_SYNC` reader - Set this bit to enable receiver in PCM standard mode."]
pub struct RX_SHORT_SYNC_R(crate::FieldReader<bool, bool>);
impl RX_SHORT_SYNC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_SHORT_SYNC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_SHORT_SYNC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_SHORT_SYNC` writer - Set this bit to enable receiver in PCM standard mode."]
pub struct RX_SHORT_SYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_SHORT_SYNC_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 13)) | ((value as u32 & 1) << 13);
        self.w
    }
}
#[doc = "Field `TX_MONO` reader - Set this bit to enable transmitter in mono mode."]
pub struct TX_MONO_R(crate::FieldReader<bool, bool>);
impl TX_MONO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_MONO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_MONO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_MONO` writer - Set this bit to enable transmitter in mono mode."]
pub struct TX_MONO_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_MONO_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 14)) | ((value as u32 & 1) << 14);
        self.w
    }
}
#[doc = "Field `RX_MONO` reader - Set this bit to enable receiver in mono mode."]
pub struct RX_MONO_R(crate::FieldReader<bool, bool>);
impl RX_MONO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_MONO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_MONO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_MONO` writer - Set this bit to enable receiver in mono mode."]
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
        self.w.bits = (self.w.bits & !(1 << 15)) | ((value as u32 & 1) << 15);
        self.w
    }
}
#[doc = "Field `TX_MSB_RIGHT` reader - Set this bit to place right channel data at the MSB in TX FIFO."]
pub struct TX_MSB_RIGHT_R(crate::FieldReader<bool, bool>);
impl TX_MSB_RIGHT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_MSB_RIGHT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_MSB_RIGHT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_MSB_RIGHT` writer - Set this bit to place right channel data at the MSB in TX FIFO."]
pub struct TX_MSB_RIGHT_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_MSB_RIGHT_W<'a> {
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
#[doc = "Field `RX_MSB_RIGHT` reader - Set this bit to place right channel data at the MSB in RX FIFO."]
pub struct RX_MSB_RIGHT_R(crate::FieldReader<bool, bool>);
impl RX_MSB_RIGHT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_MSB_RIGHT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_MSB_RIGHT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_MSB_RIGHT` writer - Set this bit to place right channel data at the MSB in RX FIFO."]
pub struct RX_MSB_RIGHT_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_MSB_RIGHT_W<'a> {
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
#[doc = "Field `TX_LSB_FIRST_DMA` reader - 1:the data in DMA/APB transform from low bits. 0:the data from DMA/APB transform from high bits."]
pub struct TX_LSB_FIRST_DMA_R(crate::FieldReader<bool, bool>);
impl TX_LSB_FIRST_DMA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_LSB_FIRST_DMA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_LSB_FIRST_DMA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_LSB_FIRST_DMA` writer - 1:the data in DMA/APB transform from low bits. 0:the data from DMA/APB transform from high bits."]
pub struct TX_LSB_FIRST_DMA_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_LSB_FIRST_DMA_W<'a> {
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
#[doc = "Field `RX_LSB_FIRST_DMA` reader - 1:the data in DMA/APB transform from low bits. 0:the data from DMA/APB transform from high bits."]
pub struct RX_LSB_FIRST_DMA_R(crate::FieldReader<bool, bool>);
impl RX_LSB_FIRST_DMA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_LSB_FIRST_DMA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_LSB_FIRST_DMA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_LSB_FIRST_DMA` writer - 1:the data in DMA/APB transform from low bits. 0:the data from DMA/APB transform from high bits."]
pub struct RX_LSB_FIRST_DMA_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_LSB_FIRST_DMA_W<'a> {
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
#[doc = "Field `SIG_LOOPBACK` reader - Enable signal loopback mode with transmitter module and receiver module sharing the same WS and BCK signals."]
pub struct SIG_LOOPBACK_R(crate::FieldReader<bool, bool>);
impl SIG_LOOPBACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SIG_LOOPBACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIG_LOOPBACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIG_LOOPBACK` writer - Enable signal loopback mode with transmitter module and receiver module sharing the same WS and BCK signals."]
pub struct SIG_LOOPBACK_W<'a> {
    w: &'a mut W,
}
impl<'a> SIG_LOOPBACK_W<'a> {
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
#[doc = "Field `TX_FIFO_RESET_ST` reader - I2S TX FIFO reset status. 1: I2S_TX_FIFO_RESET is not completed. 0: I2S_TX_FIFO_RESET is completed."]
pub struct TX_FIFO_RESET_ST_R(crate::FieldReader<bool, bool>);
impl TX_FIFO_RESET_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_FIFO_RESET_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_FIFO_RESET_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_FIFO_RESET_ST` reader - I2S RX FIFO reset status. 1: I2S_RX_FIFO_RESET is not completed. 0: I2S_RX_FIFO_RESET is completed."]
pub struct RX_FIFO_RESET_ST_R(crate::FieldReader<bool, bool>);
impl RX_FIFO_RESET_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_FIFO_RESET_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_FIFO_RESET_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_RESET_ST` reader - I2S TX reset status. 1: I2S_TX_RESET is not completed. 0: I2S_TX_RESET is completed."]
pub struct TX_RESET_ST_R(crate::FieldReader<bool, bool>);
impl TX_RESET_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_RESET_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_RESET_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_DMA_EQUAL` reader - 1: Data in left channel is equal to data in right channel. 0: Data in left channel is not equal to data in right channel."]
pub struct TX_DMA_EQUAL_R(crate::FieldReader<bool, bool>);
impl TX_DMA_EQUAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_DMA_EQUAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_DMA_EQUAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_DMA_EQUAL` writer - 1: Data in left channel is equal to data in right channel. 0: Data in left channel is not equal to data in right channel."]
pub struct TX_DMA_EQUAL_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DMA_EQUAL_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 24)) | ((value as u32 & 1) << 24);
        self.w
    }
}
#[doc = "Field `RX_DMA_EQUAL` reader - 1: Data in left channel is equal to data in right channel. 0: Data in left channel is not equal to data in right channel."]
pub struct RX_DMA_EQUAL_R(crate::FieldReader<bool, bool>);
impl RX_DMA_EQUAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_DMA_EQUAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_DMA_EQUAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_DMA_EQUAL` writer - 1: Data in left channel is equal to data in right channel. 0: Data in left channel is not equal to data in right channel."]
pub struct RX_DMA_EQUAL_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_DMA_EQUAL_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 25)) | ((value as u32 & 1) << 25);
        self.w
    }
}
#[doc = "Field `PRE_REQ_EN` reader - Set this bit to enable I2S to prepare data earlier."]
pub struct PRE_REQ_EN_R(crate::FieldReader<bool, bool>);
impl PRE_REQ_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRE_REQ_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRE_REQ_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRE_REQ_EN` writer - Set this bit to enable I2S to prepare data earlier."]
pub struct PRE_REQ_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PRE_REQ_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 26)) | ((value as u32 & 1) << 26);
        self.w
    }
}
#[doc = "Field `TX_BIG_ENDIAN` reader - I2S TX byte endianness."]
pub struct TX_BIG_ENDIAN_R(crate::FieldReader<bool, bool>);
impl TX_BIG_ENDIAN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_BIG_ENDIAN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_BIG_ENDIAN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_BIG_ENDIAN` writer - I2S TX byte endianness."]
pub struct TX_BIG_ENDIAN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_BIG_ENDIAN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 27)) | ((value as u32 & 1) << 27);
        self.w
    }
}
#[doc = "Field `RX_BIG_ENDIAN` reader - I2S RX byte endianness."]
pub struct RX_BIG_ENDIAN_R(crate::FieldReader<bool, bool>);
impl RX_BIG_ENDIAN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_BIG_ENDIAN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_BIG_ENDIAN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_BIG_ENDIAN` writer - I2S RX byte endianness."]
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
        self.w.bits = (self.w.bits & !(1 << 28)) | ((value as u32 & 1) << 28);
        self.w
    }
}
#[doc = "Field `RX_RESET_ST` reader - I2S RX reset status. 1: I2S_RX_RESET is not completed. 0: I2S_RX_RESET is completed."]
pub struct RX_RESET_ST_R(crate::FieldReader<bool, bool>);
impl RX_RESET_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_RESET_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_RESET_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 4 - Set this bit to start transmitting data."]
    #[inline(always)]
    pub fn tx_start(&self) -> TX_START_R {
        TX_START_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set this bit to start receiving data."]
    #[inline(always)]
    pub fn rx_start(&self) -> RX_START_R {
        RX_START_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set this bit to enable slave transmitter mode."]
    #[inline(always)]
    pub fn tx_slave_mod(&self) -> TX_SLAVE_MOD_R {
        TX_SLAVE_MOD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Set this bit to enable slave receiver mode."]
    #[inline(always)]
    pub fn rx_slave_mod(&self) -> RX_SLAVE_MOD_R {
        RX_SLAVE_MOD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Set this bit to transmit right channel data first."]
    #[inline(always)]
    pub fn tx_right_first(&self) -> TX_RIGHT_FIRST_R {
        TX_RIGHT_FIRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Set this bit to receive right channel data first."]
    #[inline(always)]
    pub fn rx_right_first(&self) -> RX_RIGHT_FIRST_R {
        RX_RIGHT_FIRST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Set this bit to enable transmitter in Phillips standard mode."]
    #[inline(always)]
    pub fn tx_msb_shift(&self) -> TX_MSB_SHIFT_R {
        TX_MSB_SHIFT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Set this bit to enable receiver in Phillips standard mode."]
    #[inline(always)]
    pub fn rx_msb_shift(&self) -> RX_MSB_SHIFT_R {
        RX_MSB_SHIFT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Set this bit to enable transmitter in PCM standard mode."]
    #[inline(always)]
    pub fn tx_short_sync(&self) -> TX_SHORT_SYNC_R {
        TX_SHORT_SYNC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Set this bit to enable receiver in PCM standard mode."]
    #[inline(always)]
    pub fn rx_short_sync(&self) -> RX_SHORT_SYNC_R {
        RX_SHORT_SYNC_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Set this bit to enable transmitter in mono mode."]
    #[inline(always)]
    pub fn tx_mono(&self) -> TX_MONO_R {
        TX_MONO_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Set this bit to enable receiver in mono mode."]
    #[inline(always)]
    pub fn rx_mono(&self) -> RX_MONO_R {
        RX_MONO_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Set this bit to place right channel data at the MSB in TX FIFO."]
    #[inline(always)]
    pub fn tx_msb_right(&self) -> TX_MSB_RIGHT_R {
        TX_MSB_RIGHT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Set this bit to place right channel data at the MSB in RX FIFO."]
    #[inline(always)]
    pub fn rx_msb_right(&self) -> RX_MSB_RIGHT_R {
        RX_MSB_RIGHT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 1:the data in DMA/APB transform from low bits. 0:the data from DMA/APB transform from high bits."]
    #[inline(always)]
    pub fn tx_lsb_first_dma(&self) -> TX_LSB_FIRST_DMA_R {
        TX_LSB_FIRST_DMA_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 1:the data in DMA/APB transform from low bits. 0:the data from DMA/APB transform from high bits."]
    #[inline(always)]
    pub fn rx_lsb_first_dma(&self) -> RX_LSB_FIRST_DMA_R {
        RX_LSB_FIRST_DMA_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable signal loopback mode with transmitter module and receiver module sharing the same WS and BCK signals."]
    #[inline(always)]
    pub fn sig_loopback(&self) -> SIG_LOOPBACK_R {
        SIG_LOOPBACK_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2S TX FIFO reset status. 1: I2S_TX_FIFO_RESET is not completed. 0: I2S_TX_FIFO_RESET is completed."]
    #[inline(always)]
    pub fn tx_fifo_reset_st(&self) -> TX_FIFO_RESET_ST_R {
        TX_FIFO_RESET_ST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2S RX FIFO reset status. 1: I2S_RX_FIFO_RESET is not completed. 0: I2S_RX_FIFO_RESET is completed."]
    #[inline(always)]
    pub fn rx_fifo_reset_st(&self) -> RX_FIFO_RESET_ST_R {
        RX_FIFO_RESET_ST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I2S TX reset status. 1: I2S_TX_RESET is not completed. 0: I2S_TX_RESET is completed."]
    #[inline(always)]
    pub fn tx_reset_st(&self) -> TX_RESET_ST_R {
        TX_RESET_ST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 1: Data in left channel is equal to data in right channel. 0: Data in left channel is not equal to data in right channel."]
    #[inline(always)]
    pub fn tx_dma_equal(&self) -> TX_DMA_EQUAL_R {
        TX_DMA_EQUAL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 1: Data in left channel is equal to data in right channel. 0: Data in left channel is not equal to data in right channel."]
    #[inline(always)]
    pub fn rx_dma_equal(&self) -> RX_DMA_EQUAL_R {
        RX_DMA_EQUAL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Set this bit to enable I2S to prepare data earlier."]
    #[inline(always)]
    pub fn pre_req_en(&self) -> PRE_REQ_EN_R {
        PRE_REQ_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - I2S TX byte endianness."]
    #[inline(always)]
    pub fn tx_big_endian(&self) -> TX_BIG_ENDIAN_R {
        TX_BIG_ENDIAN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - I2S RX byte endianness."]
    #[inline(always)]
    pub fn rx_big_endian(&self) -> RX_BIG_ENDIAN_R {
        RX_BIG_ENDIAN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - I2S RX reset status. 1: I2S_RX_RESET is not completed. 0: I2S_RX_RESET is completed."]
    #[inline(always)]
    pub fn rx_reset_st(&self) -> RX_RESET_ST_R {
        RX_RESET_ST_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to reset transmitter."]
    #[inline(always)]
    pub fn tx_reset(&mut self) -> TX_RESET_W {
        TX_RESET_W { w: self }
    }
    #[doc = "Bit 1 - Set this bit to reset receiver."]
    #[inline(always)]
    pub fn rx_reset(&mut self) -> RX_RESET_W {
        RX_RESET_W { w: self }
    }
    #[doc = "Bit 2 - Set this bit to reset TX FIFO."]
    #[inline(always)]
    pub fn tx_fifo_reset(&mut self) -> TX_FIFO_RESET_W {
        TX_FIFO_RESET_W { w: self }
    }
    #[doc = "Bit 3 - Set this bit to reset RX FIFO."]
    #[inline(always)]
    pub fn rx_fifo_reset(&mut self) -> RX_FIFO_RESET_W {
        RX_FIFO_RESET_W { w: self }
    }
    #[doc = "Bit 4 - Set this bit to start transmitting data."]
    #[inline(always)]
    pub fn tx_start(&mut self) -> TX_START_W {
        TX_START_W { w: self }
    }
    #[doc = "Bit 5 - Set this bit to start receiving data."]
    #[inline(always)]
    pub fn rx_start(&mut self) -> RX_START_W {
        RX_START_W { w: self }
    }
    #[doc = "Bit 6 - Set this bit to enable slave transmitter mode."]
    #[inline(always)]
    pub fn tx_slave_mod(&mut self) -> TX_SLAVE_MOD_W {
        TX_SLAVE_MOD_W { w: self }
    }
    #[doc = "Bit 7 - Set this bit to enable slave receiver mode."]
    #[inline(always)]
    pub fn rx_slave_mod(&mut self) -> RX_SLAVE_MOD_W {
        RX_SLAVE_MOD_W { w: self }
    }
    #[doc = "Bit 8 - Set this bit to transmit right channel data first."]
    #[inline(always)]
    pub fn tx_right_first(&mut self) -> TX_RIGHT_FIRST_W {
        TX_RIGHT_FIRST_W { w: self }
    }
    #[doc = "Bit 9 - Set this bit to receive right channel data first."]
    #[inline(always)]
    pub fn rx_right_first(&mut self) -> RX_RIGHT_FIRST_W {
        RX_RIGHT_FIRST_W { w: self }
    }
    #[doc = "Bit 10 - Set this bit to enable transmitter in Phillips standard mode."]
    #[inline(always)]
    pub fn tx_msb_shift(&mut self) -> TX_MSB_SHIFT_W {
        TX_MSB_SHIFT_W { w: self }
    }
    #[doc = "Bit 11 - Set this bit to enable receiver in Phillips standard mode."]
    #[inline(always)]
    pub fn rx_msb_shift(&mut self) -> RX_MSB_SHIFT_W {
        RX_MSB_SHIFT_W { w: self }
    }
    #[doc = "Bit 12 - Set this bit to enable transmitter in PCM standard mode."]
    #[inline(always)]
    pub fn tx_short_sync(&mut self) -> TX_SHORT_SYNC_W {
        TX_SHORT_SYNC_W { w: self }
    }
    #[doc = "Bit 13 - Set this bit to enable receiver in PCM standard mode."]
    #[inline(always)]
    pub fn rx_short_sync(&mut self) -> RX_SHORT_SYNC_W {
        RX_SHORT_SYNC_W { w: self }
    }
    #[doc = "Bit 14 - Set this bit to enable transmitter in mono mode."]
    #[inline(always)]
    pub fn tx_mono(&mut self) -> TX_MONO_W {
        TX_MONO_W { w: self }
    }
    #[doc = "Bit 15 - Set this bit to enable receiver in mono mode."]
    #[inline(always)]
    pub fn rx_mono(&mut self) -> RX_MONO_W {
        RX_MONO_W { w: self }
    }
    #[doc = "Bit 16 - Set this bit to place right channel data at the MSB in TX FIFO."]
    #[inline(always)]
    pub fn tx_msb_right(&mut self) -> TX_MSB_RIGHT_W {
        TX_MSB_RIGHT_W { w: self }
    }
    #[doc = "Bit 17 - Set this bit to place right channel data at the MSB in RX FIFO."]
    #[inline(always)]
    pub fn rx_msb_right(&mut self) -> RX_MSB_RIGHT_W {
        RX_MSB_RIGHT_W { w: self }
    }
    #[doc = "Bit 18 - 1:the data in DMA/APB transform from low bits. 0:the data from DMA/APB transform from high bits."]
    #[inline(always)]
    pub fn tx_lsb_first_dma(&mut self) -> TX_LSB_FIRST_DMA_W {
        TX_LSB_FIRST_DMA_W { w: self }
    }
    #[doc = "Bit 19 - 1:the data in DMA/APB transform from low bits. 0:the data from DMA/APB transform from high bits."]
    #[inline(always)]
    pub fn rx_lsb_first_dma(&mut self) -> RX_LSB_FIRST_DMA_W {
        RX_LSB_FIRST_DMA_W { w: self }
    }
    #[doc = "Bit 20 - Enable signal loopback mode with transmitter module and receiver module sharing the same WS and BCK signals."]
    #[inline(always)]
    pub fn sig_loopback(&mut self) -> SIG_LOOPBACK_W {
        SIG_LOOPBACK_W { w: self }
    }
    #[doc = "Bit 24 - 1: Data in left channel is equal to data in right channel. 0: Data in left channel is not equal to data in right channel."]
    #[inline(always)]
    pub fn tx_dma_equal(&mut self) -> TX_DMA_EQUAL_W {
        TX_DMA_EQUAL_W { w: self }
    }
    #[doc = "Bit 25 - 1: Data in left channel is equal to data in right channel. 0: Data in left channel is not equal to data in right channel."]
    #[inline(always)]
    pub fn rx_dma_equal(&mut self) -> RX_DMA_EQUAL_W {
        RX_DMA_EQUAL_W { w: self }
    }
    #[doc = "Bit 26 - Set this bit to enable I2S to prepare data earlier."]
    #[inline(always)]
    pub fn pre_req_en(&mut self) -> PRE_REQ_EN_W {
        PRE_REQ_EN_W { w: self }
    }
    #[doc = "Bit 27 - I2S TX byte endianness."]
    #[inline(always)]
    pub fn tx_big_endian(&mut self) -> TX_BIG_ENDIAN_W {
        TX_BIG_ENDIAN_W { w: self }
    }
    #[doc = "Bit 28 - I2S RX byte endianness."]
    #[inline(always)]
    pub fn rx_big_endian(&mut self) -> RX_BIG_ENDIAN_W {
        RX_BIG_ENDIAN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S configuration register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conf]
(index.html) module"]
pub struct CONF_SPEC;
impl crate::RegisterSpec for CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conf::R]
(R) reader structure"]
impl crate::Readable for CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conf::W]
(W) writer structure"]
impl crate::Writable for CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONF to value 0x000c_0300"]
impl crate::Resettable for CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x000c_0300
    }
}
