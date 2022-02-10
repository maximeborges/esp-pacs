#[doc = "Register `FIFO_CONF` reader"]
pub struct R(crate::R<FIFO_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFO_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFO_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFO_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFO_CONF` writer"]
pub struct W(crate::W<FIFO_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFO_CONF_SPEC>;
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
impl From<crate::W<FIFO_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFO_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_DATA_NUM` reader - I2S_RX_TAKE_DATA_INT is triggered when the left and right channel data number in RX FIFO is larger than the value of I2S_RX_DATA_NUM\\[5:0\\]
. (RX FIFO is almost full threshold.)"]
pub struct RX_DATA_NUM_R(crate::FieldReader<u8, u8>);
impl RX_DATA_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RX_DATA_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_DATA_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_DATA_NUM` writer - I2S_RX_TAKE_DATA_INT is triggered when the left and right channel data number in RX FIFO is larger than the value of I2S_RX_DATA_NUM\\[5:0\\]
. (RX FIFO is almost full threshold.)"]
pub struct RX_DATA_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_DATA_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Field `TX_DATA_NUM` reader - I2S_TX_PUT_DATA_INT is triggered when the left and right channel data number in TX FIFO is smaller than the value of I2S_TX_DATA_NUM\\[5:0\\]
. (TX FIFO is almost empty threshold.)"]
pub struct TX_DATA_NUM_R(crate::FieldReader<u8, u8>);
impl TX_DATA_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TX_DATA_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_DATA_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_DATA_NUM` writer - I2S_TX_PUT_DATA_INT is triggered when the left and right channel data number in TX FIFO is smaller than the value of I2S_TX_DATA_NUM\\[5:0\\]
. (TX FIFO is almost empty threshold.)"]
pub struct TX_DATA_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DATA_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 6)) | ((value as u32 & 0x3f) << 6);
        self.w
    }
}
#[doc = "Field `DSCR_EN` reader - Set this bit to enable I2S DMA mode."]
pub struct DSCR_EN_R(crate::FieldReader<bool, bool>);
impl DSCR_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DSCR_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSCR_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSCR_EN` writer - Set this bit to enable I2S DMA mode."]
pub struct DSCR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DSCR_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `TX_FIFO_MOD` reader - Transmitter FIFO mode configuration bits"]
pub struct TX_FIFO_MOD_R(crate::FieldReader<u8, u8>);
impl TX_FIFO_MOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TX_FIFO_MOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_FIFO_MOD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_FIFO_MOD` writer - Transmitter FIFO mode configuration bits"]
pub struct TX_FIFO_MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FIFO_MOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | ((value as u32 & 0x07) << 13);
        self.w
    }
}
#[doc = "Field `RX_FIFO_MOD` reader - Receiver FIFO mode configuration bits"]
pub struct RX_FIFO_MOD_R(crate::FieldReader<u8, u8>);
impl RX_FIFO_MOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RX_FIFO_MOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_FIFO_MOD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_FIFO_MOD` writer - Receiver FIFO mode configuration bits"]
pub struct RX_FIFO_MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FIFO_MOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "Field `TX_FIFO_MOD_FORCE_EN` reader - The bit should always be set to 1"]
pub struct TX_FIFO_MOD_FORCE_EN_R(crate::FieldReader<bool, bool>);
impl TX_FIFO_MOD_FORCE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_FIFO_MOD_FORCE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_FIFO_MOD_FORCE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_FIFO_MOD_FORCE_EN` writer - The bit should always be set to 1"]
pub struct TX_FIFO_MOD_FORCE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FIFO_MOD_FORCE_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `RX_FIFO_MOD_FORCE_EN` reader - The bit should always be set to 1"]
pub struct RX_FIFO_MOD_FORCE_EN_R(crate::FieldReader<bool, bool>);
impl RX_FIFO_MOD_FORCE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_FIFO_MOD_FORCE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_FIFO_MOD_FORCE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_FIFO_MOD_FORCE_EN` writer - The bit should always be set to 1"]
pub struct RX_FIFO_MOD_FORCE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FIFO_MOD_FORCE_EN_W<'a> {
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
#[doc = "Field `RX_FIFO_SYNC` reader - force write back rx data to memory"]
pub struct RX_FIFO_SYNC_R(crate::FieldReader<bool, bool>);
impl RX_FIFO_SYNC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_FIFO_SYNC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_FIFO_SYNC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_FIFO_SYNC` writer - force write back rx data to memory"]
pub struct RX_FIFO_SYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FIFO_SYNC_W<'a> {
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
#[doc = "Field `RX_24MSB_EN` reader - Only useful in rx 24bit mode. 1: the high 24 bits are effective in i2s fifo 0: the low 24 bits are effective in i2s fifo"]
pub struct RX_24MSB_EN_R(crate::FieldReader<bool, bool>);
impl RX_24MSB_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_24MSB_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_24MSB_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_24MSB_EN` writer - Only useful in rx 24bit mode. 1: the high 24 bits are effective in i2s fifo 0: the low 24 bits are effective in i2s fifo"]
pub struct RX_24MSB_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_24MSB_EN_W<'a> {
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
#[doc = "Field `TX_24MSB_EN` reader - Only useful in tx 24bit mode. 1: the high 24 bits are effective in i2s fifo 0: the low 24 bits are effective in i2s fifo"]
pub struct TX_24MSB_EN_R(crate::FieldReader<bool, bool>);
impl TX_24MSB_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_24MSB_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_24MSB_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_24MSB_EN` writer - Only useful in tx 24bit mode. 1: the high 24 bits are effective in i2s fifo 0: the low 24 bits are effective in i2s fifo"]
pub struct TX_24MSB_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_24MSB_EN_W<'a> {
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
impl R {
    #[doc = "Bits 0:5 - I2S_RX_TAKE_DATA_INT is triggered when the left and right channel data number in RX FIFO is larger than the value of I2S_RX_DATA_NUM\\[5:0\\]
. (RX FIFO is almost full threshold.)"]
    #[inline(always)]
    pub fn rx_data_num(&self) -> RX_DATA_NUM_R {
        RX_DATA_NUM_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - I2S_TX_PUT_DATA_INT is triggered when the left and right channel data number in TX FIFO is smaller than the value of I2S_TX_DATA_NUM\\[5:0\\]
. (TX FIFO is almost empty threshold.)"]
    #[inline(always)]
    pub fn tx_data_num(&self) -> TX_DATA_NUM_R {
        TX_DATA_NUM_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bit 12 - Set this bit to enable I2S DMA mode."]
    #[inline(always)]
    pub fn dscr_en(&self) -> DSCR_EN_R {
        DSCR_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 13:15 - Transmitter FIFO mode configuration bits"]
    #[inline(always)]
    pub fn tx_fifo_mod(&self) -> TX_FIFO_MOD_R {
        TX_FIFO_MOD_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    #[doc = "Bits 16:18 - Receiver FIFO mode configuration bits"]
    #[inline(always)]
    pub fn rx_fifo_mod(&self) -> RX_FIFO_MOD_R {
        RX_FIFO_MOD_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 19 - The bit should always be set to 1"]
    #[inline(always)]
    pub fn tx_fifo_mod_force_en(&self) -> TX_FIFO_MOD_FORCE_EN_R {
        TX_FIFO_MOD_FORCE_EN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - The bit should always be set to 1"]
    #[inline(always)]
    pub fn rx_fifo_mod_force_en(&self) -> RX_FIFO_MOD_FORCE_EN_R {
        RX_FIFO_MOD_FORCE_EN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - force write back rx data to memory"]
    #[inline(always)]
    pub fn rx_fifo_sync(&self) -> RX_FIFO_SYNC_R {
        RX_FIFO_SYNC_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Only useful in rx 24bit mode. 1: the high 24 bits are effective in i2s fifo 0: the low 24 bits are effective in i2s fifo"]
    #[inline(always)]
    pub fn rx_24msb_en(&self) -> RX_24MSB_EN_R {
        RX_24MSB_EN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Only useful in tx 24bit mode. 1: the high 24 bits are effective in i2s fifo 0: the low 24 bits are effective in i2s fifo"]
    #[inline(always)]
    pub fn tx_24msb_en(&self) -> TX_24MSB_EN_R {
        TX_24MSB_EN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - I2S_RX_TAKE_DATA_INT is triggered when the left and right channel data number in RX FIFO is larger than the value of I2S_RX_DATA_NUM\\[5:0\\]
. (RX FIFO is almost full threshold.)"]
    #[inline(always)]
    pub fn rx_data_num(&mut self) -> RX_DATA_NUM_W {
        RX_DATA_NUM_W { w: self }
    }
    #[doc = "Bits 6:11 - I2S_TX_PUT_DATA_INT is triggered when the left and right channel data number in TX FIFO is smaller than the value of I2S_TX_DATA_NUM\\[5:0\\]
. (TX FIFO is almost empty threshold.)"]
    #[inline(always)]
    pub fn tx_data_num(&mut self) -> TX_DATA_NUM_W {
        TX_DATA_NUM_W { w: self }
    }
    #[doc = "Bit 12 - Set this bit to enable I2S DMA mode."]
    #[inline(always)]
    pub fn dscr_en(&mut self) -> DSCR_EN_W {
        DSCR_EN_W { w: self }
    }
    #[doc = "Bits 13:15 - Transmitter FIFO mode configuration bits"]
    #[inline(always)]
    pub fn tx_fifo_mod(&mut self) -> TX_FIFO_MOD_W {
        TX_FIFO_MOD_W { w: self }
    }
    #[doc = "Bits 16:18 - Receiver FIFO mode configuration bits"]
    #[inline(always)]
    pub fn rx_fifo_mod(&mut self) -> RX_FIFO_MOD_W {
        RX_FIFO_MOD_W { w: self }
    }
    #[doc = "Bit 19 - The bit should always be set to 1"]
    #[inline(always)]
    pub fn tx_fifo_mod_force_en(&mut self) -> TX_FIFO_MOD_FORCE_EN_W {
        TX_FIFO_MOD_FORCE_EN_W { w: self }
    }
    #[doc = "Bit 20 - The bit should always be set to 1"]
    #[inline(always)]
    pub fn rx_fifo_mod_force_en(&mut self) -> RX_FIFO_MOD_FORCE_EN_W {
        RX_FIFO_MOD_FORCE_EN_W { w: self }
    }
    #[doc = "Bit 21 - force write back rx data to memory"]
    #[inline(always)]
    pub fn rx_fifo_sync(&mut self) -> RX_FIFO_SYNC_W {
        RX_FIFO_SYNC_W { w: self }
    }
    #[doc = "Bit 22 - Only useful in rx 24bit mode. 1: the high 24 bits are effective in i2s fifo 0: the low 24 bits are effective in i2s fifo"]
    #[inline(always)]
    pub fn rx_24msb_en(&mut self) -> RX_24MSB_EN_W {
        RX_24MSB_EN_W { w: self }
    }
    #[doc = "Bit 23 - Only useful in tx 24bit mode. 1: the high 24 bits are effective in i2s fifo 0: the low 24 bits are effective in i2s fifo"]
    #[inline(always)]
    pub fn tx_24msb_en(&mut self) -> TX_24MSB_EN_W {
        TX_24MSB_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S FIFO configuration register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo_conf]
(index.html) module"]
pub struct FIFO_CONF_SPEC;
impl crate::RegisterSpec for FIFO_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifo_conf::R]
(R) reader structure"]
impl crate::Readable for FIFO_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifo_conf::W]
(W) writer structure"]
impl crate::Writable for FIFO_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FIFO_CONF to value 0x1820"]
impl crate::Resettable for FIFO_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1820
    }
}
