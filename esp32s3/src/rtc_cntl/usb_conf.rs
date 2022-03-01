#[doc = "Register `USB_CONF` reader"]
pub struct R(crate::R<USB_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USB_CONF` writer"]
pub struct W(crate::W<USB_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB_CONF_SPEC>;
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
impl From<crate::W<USB_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_VREFH` reader - reg_usb_vrefh"]
pub struct USB_VREFH_R(crate::FieldReader<u8, u8>);
impl USB_VREFH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        USB_VREFH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_VREFH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB_VREFH` writer - reg_usb_vrefh"]
pub struct USB_VREFH_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_VREFH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `USB_VREFL` reader - reg_usb_vrefl"]
pub struct USB_VREFL_R(crate::FieldReader<u8, u8>);
impl USB_VREFL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        USB_VREFL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_VREFL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB_VREFL` writer - reg_usb_vrefl"]
pub struct USB_VREFL_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_VREFL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `USB_VREF_OVERRIDE` reader - reg_usb_vref_override"]
pub struct USB_VREF_OVERRIDE_R(crate::FieldReader<bool, bool>);
impl USB_VREF_OVERRIDE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USB_VREF_OVERRIDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_VREF_OVERRIDE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB_VREF_OVERRIDE` writer - reg_usb_vref_override"]
pub struct USB_VREF_OVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_VREF_OVERRIDE_W<'a> {
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
#[doc = "Field `USB_PAD_PULL_OVERRIDE` reader - reg_usb_pad_pull_override"]
pub struct USB_PAD_PULL_OVERRIDE_R(crate::FieldReader<bool, bool>);
impl USB_PAD_PULL_OVERRIDE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USB_PAD_PULL_OVERRIDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_PAD_PULL_OVERRIDE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB_PAD_PULL_OVERRIDE` writer - reg_usb_pad_pull_override"]
pub struct USB_PAD_PULL_OVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_PAD_PULL_OVERRIDE_W<'a> {
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
#[doc = "Field `USB_DP_PULLUP` reader - reg_usb_dp_pullup"]
pub struct USB_DP_PULLUP_R(crate::FieldReader<bool, bool>);
impl USB_DP_PULLUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USB_DP_PULLUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_DP_PULLUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB_DP_PULLUP` writer - reg_usb_dp_pullup"]
pub struct USB_DP_PULLUP_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_DP_PULLUP_W<'a> {
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
#[doc = "Field `USB_DP_PULLDOWN` reader - reg_usb_dp_pulldown"]
pub struct USB_DP_PULLDOWN_R(crate::FieldReader<bool, bool>);
impl USB_DP_PULLDOWN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USB_DP_PULLDOWN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_DP_PULLDOWN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB_DP_PULLDOWN` writer - reg_usb_dp_pulldown"]
pub struct USB_DP_PULLDOWN_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_DP_PULLDOWN_W<'a> {
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
#[doc = "Field `USB_DM_PULLUP` reader - reg_usb_dm_pullup"]
pub struct USB_DM_PULLUP_R(crate::FieldReader<bool, bool>);
impl USB_DM_PULLUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USB_DM_PULLUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_DM_PULLUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB_DM_PULLUP` writer - reg_usb_dm_pullup"]
pub struct USB_DM_PULLUP_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_DM_PULLUP_W<'a> {
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
#[doc = "Field `USB_DM_PULLDOWN` reader - reg_usb_dm_pulldown"]
pub struct USB_DM_PULLDOWN_R(crate::FieldReader<bool, bool>);
impl USB_DM_PULLDOWN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USB_DM_PULLDOWN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_DM_PULLDOWN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB_DM_PULLDOWN` writer - reg_usb_dm_pulldown"]
pub struct USB_DM_PULLDOWN_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_DM_PULLDOWN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `USB_PULLUP_VALUE` reader - reg_usb_pullup_value"]
pub struct USB_PULLUP_VALUE_R(crate::FieldReader<bool, bool>);
impl USB_PULLUP_VALUE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USB_PULLUP_VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_PULLUP_VALUE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB_PULLUP_VALUE` writer - reg_usb_pullup_value"]
pub struct USB_PULLUP_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_PULLUP_VALUE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `USB_PAD_ENABLE_OVERRIDE` reader - reg_usb_pad_enable_override"]
pub struct USB_PAD_ENABLE_OVERRIDE_R(crate::FieldReader<bool, bool>);
impl USB_PAD_ENABLE_OVERRIDE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USB_PAD_ENABLE_OVERRIDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_PAD_ENABLE_OVERRIDE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB_PAD_ENABLE_OVERRIDE` writer - reg_usb_pad_enable_override"]
pub struct USB_PAD_ENABLE_OVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_PAD_ENABLE_OVERRIDE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `USB_PAD_ENABLE` reader - reg_usb_pad_enable"]
pub struct USB_PAD_ENABLE_R(crate::FieldReader<bool, bool>);
impl USB_PAD_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USB_PAD_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_PAD_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB_PAD_ENABLE` writer - reg_usb_pad_enable"]
pub struct USB_PAD_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_PAD_ENABLE_W<'a> {
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
#[doc = "Field `USB_TXM` reader - reg_usb_txm"]
pub struct USB_TXM_R(crate::FieldReader<bool, bool>);
impl USB_TXM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USB_TXM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_TXM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB_TXM` writer - reg_usb_txm"]
pub struct USB_TXM_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_TXM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `USB_TXP` reader - reg_usb_txp"]
pub struct USB_TXP_R(crate::FieldReader<bool, bool>);
impl USB_TXP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USB_TXP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_TXP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB_TXP` writer - reg_usb_txp"]
pub struct USB_TXP_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_TXP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `USB_TX_EN` reader - reg_usb_tx_en"]
pub struct USB_TX_EN_R(crate::FieldReader<bool, bool>);
impl USB_TX_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USB_TX_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_TX_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB_TX_EN` writer - reg_usb_tx_en"]
pub struct USB_TX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_TX_EN_W<'a> {
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
#[doc = "Field `USB_TX_EN_OVERRIDE` reader - reg_usb_tx_en_override"]
pub struct USB_TX_EN_OVERRIDE_R(crate::FieldReader<bool, bool>);
impl USB_TX_EN_OVERRIDE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USB_TX_EN_OVERRIDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_TX_EN_OVERRIDE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB_TX_EN_OVERRIDE` writer - reg_usb_tx_en_override"]
pub struct USB_TX_EN_OVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_TX_EN_OVERRIDE_W<'a> {
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
#[doc = "Field `USB_RESET_DISABLE` reader - reg_usb_reset_disable"]
pub struct USB_RESET_DISABLE_R(crate::FieldReader<bool, bool>);
impl USB_RESET_DISABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USB_RESET_DISABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_RESET_DISABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB_RESET_DISABLE` writer - reg_usb_reset_disable"]
pub struct USB_RESET_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_RESET_DISABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `IO_MUX_RESET_DISABLE` reader - reg_io_mux_reset_disable"]
pub struct IO_MUX_RESET_DISABLE_R(crate::FieldReader<bool, bool>);
impl IO_MUX_RESET_DISABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IO_MUX_RESET_DISABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IO_MUX_RESET_DISABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IO_MUX_RESET_DISABLE` writer - reg_io_mux_reset_disable"]
pub struct IO_MUX_RESET_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> IO_MUX_RESET_DISABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `SW_USB_PHY_SEL` reader - reg_sw_usb_phy_sel"]
pub struct SW_USB_PHY_SEL_R(crate::FieldReader<bool, bool>);
impl SW_USB_PHY_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SW_USB_PHY_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_USB_PHY_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_USB_PHY_SEL` writer - reg_sw_usb_phy_sel"]
pub struct SW_USB_PHY_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_USB_PHY_SEL_W<'a> {
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
#[doc = "Field `SW_HW_USB_PHY_SEL` reader - reg_sw_hw_usb_phy_sel"]
pub struct SW_HW_USB_PHY_SEL_R(crate::FieldReader<bool, bool>);
impl SW_HW_USB_PHY_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SW_HW_USB_PHY_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_HW_USB_PHY_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_HW_USB_PHY_SEL` writer - reg_sw_hw_usb_phy_sel"]
pub struct SW_HW_USB_PHY_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_HW_USB_PHY_SEL_W<'a> {
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
impl R {
    #[doc = "Bits 0:1 - reg_usb_vrefh"]
    #[inline(always)]
    pub fn usb_vrefh(&self) -> USB_VREFH_R {
        USB_VREFH_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - reg_usb_vrefl"]
    #[inline(always)]
    pub fn usb_vrefl(&self) -> USB_VREFL_R {
        USB_VREFL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - reg_usb_vref_override"]
    #[inline(always)]
    pub fn usb_vref_override(&self) -> USB_VREF_OVERRIDE_R {
        USB_VREF_OVERRIDE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - reg_usb_pad_pull_override"]
    #[inline(always)]
    pub fn usb_pad_pull_override(&self) -> USB_PAD_PULL_OVERRIDE_R {
        USB_PAD_PULL_OVERRIDE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - reg_usb_dp_pullup"]
    #[inline(always)]
    pub fn usb_dp_pullup(&self) -> USB_DP_PULLUP_R {
        USB_DP_PULLUP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - reg_usb_dp_pulldown"]
    #[inline(always)]
    pub fn usb_dp_pulldown(&self) -> USB_DP_PULLDOWN_R {
        USB_DP_PULLDOWN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - reg_usb_dm_pullup"]
    #[inline(always)]
    pub fn usb_dm_pullup(&self) -> USB_DM_PULLUP_R {
        USB_DM_PULLUP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - reg_usb_dm_pulldown"]
    #[inline(always)]
    pub fn usb_dm_pulldown(&self) -> USB_DM_PULLDOWN_R {
        USB_DM_PULLDOWN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - reg_usb_pullup_value"]
    #[inline(always)]
    pub fn usb_pullup_value(&self) -> USB_PULLUP_VALUE_R {
        USB_PULLUP_VALUE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - reg_usb_pad_enable_override"]
    #[inline(always)]
    pub fn usb_pad_enable_override(&self) -> USB_PAD_ENABLE_OVERRIDE_R {
        USB_PAD_ENABLE_OVERRIDE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - reg_usb_pad_enable"]
    #[inline(always)]
    pub fn usb_pad_enable(&self) -> USB_PAD_ENABLE_R {
        USB_PAD_ENABLE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - reg_usb_txm"]
    #[inline(always)]
    pub fn usb_txm(&self) -> USB_TXM_R {
        USB_TXM_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - reg_usb_txp"]
    #[inline(always)]
    pub fn usb_txp(&self) -> USB_TXP_R {
        USB_TXP_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - reg_usb_tx_en"]
    #[inline(always)]
    pub fn usb_tx_en(&self) -> USB_TX_EN_R {
        USB_TX_EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - reg_usb_tx_en_override"]
    #[inline(always)]
    pub fn usb_tx_en_override(&self) -> USB_TX_EN_OVERRIDE_R {
        USB_TX_EN_OVERRIDE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - reg_usb_reset_disable"]
    #[inline(always)]
    pub fn usb_reset_disable(&self) -> USB_RESET_DISABLE_R {
        USB_RESET_DISABLE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - reg_io_mux_reset_disable"]
    #[inline(always)]
    pub fn io_mux_reset_disable(&self) -> IO_MUX_RESET_DISABLE_R {
        IO_MUX_RESET_DISABLE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - reg_sw_usb_phy_sel"]
    #[inline(always)]
    pub fn sw_usb_phy_sel(&self) -> SW_USB_PHY_SEL_R {
        SW_USB_PHY_SEL_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - reg_sw_hw_usb_phy_sel"]
    #[inline(always)]
    pub fn sw_hw_usb_phy_sel(&self) -> SW_HW_USB_PHY_SEL_R {
        SW_HW_USB_PHY_SEL_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - reg_usb_vrefh"]
    #[inline(always)]
    pub fn usb_vrefh(&mut self) -> USB_VREFH_W {
        USB_VREFH_W { w: self }
    }
    #[doc = "Bits 2:3 - reg_usb_vrefl"]
    #[inline(always)]
    pub fn usb_vrefl(&mut self) -> USB_VREFL_W {
        USB_VREFL_W { w: self }
    }
    #[doc = "Bit 4 - reg_usb_vref_override"]
    #[inline(always)]
    pub fn usb_vref_override(&mut self) -> USB_VREF_OVERRIDE_W {
        USB_VREF_OVERRIDE_W { w: self }
    }
    #[doc = "Bit 5 - reg_usb_pad_pull_override"]
    #[inline(always)]
    pub fn usb_pad_pull_override(&mut self) -> USB_PAD_PULL_OVERRIDE_W {
        USB_PAD_PULL_OVERRIDE_W { w: self }
    }
    #[doc = "Bit 6 - reg_usb_dp_pullup"]
    #[inline(always)]
    pub fn usb_dp_pullup(&mut self) -> USB_DP_PULLUP_W {
        USB_DP_PULLUP_W { w: self }
    }
    #[doc = "Bit 7 - reg_usb_dp_pulldown"]
    #[inline(always)]
    pub fn usb_dp_pulldown(&mut self) -> USB_DP_PULLDOWN_W {
        USB_DP_PULLDOWN_W { w: self }
    }
    #[doc = "Bit 8 - reg_usb_dm_pullup"]
    #[inline(always)]
    pub fn usb_dm_pullup(&mut self) -> USB_DM_PULLUP_W {
        USB_DM_PULLUP_W { w: self }
    }
    #[doc = "Bit 9 - reg_usb_dm_pulldown"]
    #[inline(always)]
    pub fn usb_dm_pulldown(&mut self) -> USB_DM_PULLDOWN_W {
        USB_DM_PULLDOWN_W { w: self }
    }
    #[doc = "Bit 10 - reg_usb_pullup_value"]
    #[inline(always)]
    pub fn usb_pullup_value(&mut self) -> USB_PULLUP_VALUE_W {
        USB_PULLUP_VALUE_W { w: self }
    }
    #[doc = "Bit 11 - reg_usb_pad_enable_override"]
    #[inline(always)]
    pub fn usb_pad_enable_override(&mut self) -> USB_PAD_ENABLE_OVERRIDE_W {
        USB_PAD_ENABLE_OVERRIDE_W { w: self }
    }
    #[doc = "Bit 12 - reg_usb_pad_enable"]
    #[inline(always)]
    pub fn usb_pad_enable(&mut self) -> USB_PAD_ENABLE_W {
        USB_PAD_ENABLE_W { w: self }
    }
    #[doc = "Bit 13 - reg_usb_txm"]
    #[inline(always)]
    pub fn usb_txm(&mut self) -> USB_TXM_W {
        USB_TXM_W { w: self }
    }
    #[doc = "Bit 14 - reg_usb_txp"]
    #[inline(always)]
    pub fn usb_txp(&mut self) -> USB_TXP_W {
        USB_TXP_W { w: self }
    }
    #[doc = "Bit 15 - reg_usb_tx_en"]
    #[inline(always)]
    pub fn usb_tx_en(&mut self) -> USB_TX_EN_W {
        USB_TX_EN_W { w: self }
    }
    #[doc = "Bit 16 - reg_usb_tx_en_override"]
    #[inline(always)]
    pub fn usb_tx_en_override(&mut self) -> USB_TX_EN_OVERRIDE_W {
        USB_TX_EN_OVERRIDE_W { w: self }
    }
    #[doc = "Bit 17 - reg_usb_reset_disable"]
    #[inline(always)]
    pub fn usb_reset_disable(&mut self) -> USB_RESET_DISABLE_W {
        USB_RESET_DISABLE_W { w: self }
    }
    #[doc = "Bit 18 - reg_io_mux_reset_disable"]
    #[inline(always)]
    pub fn io_mux_reset_disable(&mut self) -> IO_MUX_RESET_DISABLE_W {
        IO_MUX_RESET_DISABLE_W { w: self }
    }
    #[doc = "Bit 19 - reg_sw_usb_phy_sel"]
    #[inline(always)]
    pub fn sw_usb_phy_sel(&mut self) -> SW_USB_PHY_SEL_W {
        SW_USB_PHY_SEL_W { w: self }
    }
    #[doc = "Bit 20 - reg_sw_hw_usb_phy_sel"]
    #[inline(always)]
    pub fn sw_hw_usb_phy_sel(&mut self) -> SW_HW_USB_PHY_SEL_W {
        SW_HW_USB_PHY_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "usb configure\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_conf]
(index.html) module"]
pub struct USB_CONF_SPEC;
impl crate::RegisterSpec for USB_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb_conf::R]
(R) reader structure"]
impl crate::Readable for USB_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb_conf::W]
(W) writer structure"]
impl crate::Writable for USB_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USB_CONF to value 0"]
impl crate::Resettable for USB_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
