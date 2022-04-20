#[doc = "Register `CONF0` reader"]
pub struct R(crate::R<CONF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONF0` writer"]
pub struct W(crate::W<CONF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONF0_SPEC>;
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
impl From<crate::W<CONF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PHY_SEL` reader - Select internal/external PHY"]
pub struct PHY_SEL_R(crate::FieldReader<bool, bool>);
impl PHY_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PHY_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PHY_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PHY_SEL` writer - Select internal/external PHY"]
pub struct PHY_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PHY_SEL_W<'a> {
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
#[doc = "Field `EXCHG_PINS_OVERRIDE` reader - Enable software control USB D+ D- exchange"]
pub struct EXCHG_PINS_OVERRIDE_R(crate::FieldReader<bool, bool>);
impl EXCHG_PINS_OVERRIDE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXCHG_PINS_OVERRIDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXCHG_PINS_OVERRIDE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXCHG_PINS_OVERRIDE` writer - Enable software control USB D+ D- exchange"]
pub struct EXCHG_PINS_OVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> EXCHG_PINS_OVERRIDE_W<'a> {
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
#[doc = "Field `EXCHG_PINS` reader - USB D+ D- exchange"]
pub struct EXCHG_PINS_R(crate::FieldReader<bool, bool>);
impl EXCHG_PINS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXCHG_PINS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXCHG_PINS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXCHG_PINS` writer - USB D+ D- exchange"]
pub struct EXCHG_PINS_W<'a> {
    w: &'a mut W,
}
impl<'a> EXCHG_PINS_W<'a> {
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
#[doc = "Field `VREFH` reader - Control single-end input high threshold,1.76V to 2V, step 80mV"]
pub struct VREFH_R(crate::FieldReader<u8, u8>);
impl VREFH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        VREFH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VREFH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VREFH` writer - Control single-end input high threshold,1.76V to 2V, step 80mV"]
pub struct VREFH_W<'a> {
    w: &'a mut W,
}
impl<'a> VREFH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 3)) | ((value as u32 & 3) << 3);
        self.w
    }
}
#[doc = "Field `VREFL` reader - Control single-end input low threshold,0.8V to 1.04V, step 80mV"]
pub struct VREFL_R(crate::FieldReader<u8, u8>);
impl VREFL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        VREFL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VREFL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VREFL` writer - Control single-end input low threshold,0.8V to 1.04V, step 80mV"]
pub struct VREFL_W<'a> {
    w: &'a mut W,
}
impl<'a> VREFL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 5)) | ((value as u32 & 3) << 5);
        self.w
    }
}
#[doc = "Field `VREF_OVERRIDE` reader - Enable software control input threshold"]
pub struct VREF_OVERRIDE_R(crate::FieldReader<bool, bool>);
impl VREF_OVERRIDE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VREF_OVERRIDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VREF_OVERRIDE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VREF_OVERRIDE` writer - Enable software control input threshold"]
pub struct VREF_OVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> VREF_OVERRIDE_W<'a> {
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
#[doc = "Field `PAD_PULL_OVERRIDE` reader - Enable software control USB D+ D- pullup pulldown"]
pub struct PAD_PULL_OVERRIDE_R(crate::FieldReader<bool, bool>);
impl PAD_PULL_OVERRIDE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PAD_PULL_OVERRIDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_PULL_OVERRIDE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD_PULL_OVERRIDE` writer - Enable software control USB D+ D- pullup pulldown"]
pub struct PAD_PULL_OVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_PULL_OVERRIDE_W<'a> {
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
#[doc = "Field `DP_PULLUP` reader - Control USB D+ pull up."]
pub struct DP_PULLUP_R(crate::FieldReader<bool, bool>);
impl DP_PULLUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DP_PULLUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DP_PULLUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DP_PULLUP` writer - Control USB D+ pull up."]
pub struct DP_PULLUP_W<'a> {
    w: &'a mut W,
}
impl<'a> DP_PULLUP_W<'a> {
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
#[doc = "Field `DP_PULLDOWN` reader - Control USB D+ pull down."]
pub struct DP_PULLDOWN_R(crate::FieldReader<bool, bool>);
impl DP_PULLDOWN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DP_PULLDOWN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DP_PULLDOWN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DP_PULLDOWN` writer - Control USB D+ pull down."]
pub struct DP_PULLDOWN_W<'a> {
    w: &'a mut W,
}
impl<'a> DP_PULLDOWN_W<'a> {
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
#[doc = "Field `DM_PULLUP` reader - Control USB D- pull up."]
pub struct DM_PULLUP_R(crate::FieldReader<bool, bool>);
impl DM_PULLUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DM_PULLUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DM_PULLUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DM_PULLUP` writer - Control USB D- pull up."]
pub struct DM_PULLUP_W<'a> {
    w: &'a mut W,
}
impl<'a> DM_PULLUP_W<'a> {
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
#[doc = "Field `DM_PULLDOWN` reader - Control USB D- pull down."]
pub struct DM_PULLDOWN_R(crate::FieldReader<bool, bool>);
impl DM_PULLDOWN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DM_PULLDOWN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DM_PULLDOWN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DM_PULLDOWN` writer - Control USB D- pull down."]
pub struct DM_PULLDOWN_W<'a> {
    w: &'a mut W,
}
impl<'a> DM_PULLDOWN_W<'a> {
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
#[doc = "Field `PULLUP_VALUE` reader - Control pull up value."]
pub struct PULLUP_VALUE_R(crate::FieldReader<bool, bool>);
impl PULLUP_VALUE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PULLUP_VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PULLUP_VALUE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PULLUP_VALUE` writer - Control pull up value."]
pub struct PULLUP_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> PULLUP_VALUE_W<'a> {
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
#[doc = "Field `USB_PAD_ENABLE` reader - Enable USB pad function."]
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
#[doc = "Field `USB_PAD_ENABLE` writer - Enable USB pad function."]
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
        self.w.bits = (self.w.bits & !(1 << 14)) | ((value as u32 & 1) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Select internal/external PHY"]
    #[inline(always)]
    pub fn phy_sel(&self) -> PHY_SEL_R {
        PHY_SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable software control USB D+ D- exchange"]
    #[inline(always)]
    pub fn exchg_pins_override(&self) -> EXCHG_PINS_OVERRIDE_R {
        EXCHG_PINS_OVERRIDE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USB D+ D- exchange"]
    #[inline(always)]
    pub fn exchg_pins(&self) -> EXCHG_PINS_R {
        EXCHG_PINS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Control single-end input high threshold,1.76V to 2V, step 80mV"]
    #[inline(always)]
    pub fn vrefh(&self) -> VREFH_R {
        VREFH_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - Control single-end input low threshold,0.8V to 1.04V, step 80mV"]
    #[inline(always)]
    pub fn vrefl(&self) -> VREFL_R {
        VREFL_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Enable software control input threshold"]
    #[inline(always)]
    pub fn vref_override(&self) -> VREF_OVERRIDE_R {
        VREF_OVERRIDE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable software control USB D+ D- pullup pulldown"]
    #[inline(always)]
    pub fn pad_pull_override(&self) -> PAD_PULL_OVERRIDE_R {
        PAD_PULL_OVERRIDE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Control USB D+ pull up."]
    #[inline(always)]
    pub fn dp_pullup(&self) -> DP_PULLUP_R {
        DP_PULLUP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Control USB D+ pull down."]
    #[inline(always)]
    pub fn dp_pulldown(&self) -> DP_PULLDOWN_R {
        DP_PULLDOWN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Control USB D- pull up."]
    #[inline(always)]
    pub fn dm_pullup(&self) -> DM_PULLUP_R {
        DM_PULLUP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Control USB D- pull down."]
    #[inline(always)]
    pub fn dm_pulldown(&self) -> DM_PULLDOWN_R {
        DM_PULLDOWN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Control pull up value."]
    #[inline(always)]
    pub fn pullup_value(&self) -> PULLUP_VALUE_R {
        PULLUP_VALUE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable USB pad function."]
    #[inline(always)]
    pub fn usb_pad_enable(&self) -> USB_PAD_ENABLE_R {
        USB_PAD_ENABLE_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select internal/external PHY"]
    #[inline(always)]
    pub fn phy_sel(&mut self) -> PHY_SEL_W {
        PHY_SEL_W { w: self }
    }
    #[doc = "Bit 1 - Enable software control USB D+ D- exchange"]
    #[inline(always)]
    pub fn exchg_pins_override(&mut self) -> EXCHG_PINS_OVERRIDE_W {
        EXCHG_PINS_OVERRIDE_W { w: self }
    }
    #[doc = "Bit 2 - USB D+ D- exchange"]
    #[inline(always)]
    pub fn exchg_pins(&mut self) -> EXCHG_PINS_W {
        EXCHG_PINS_W { w: self }
    }
    #[doc = "Bits 3:4 - Control single-end input high threshold,1.76V to 2V, step 80mV"]
    #[inline(always)]
    pub fn vrefh(&mut self) -> VREFH_W {
        VREFH_W { w: self }
    }
    #[doc = "Bits 5:6 - Control single-end input low threshold,0.8V to 1.04V, step 80mV"]
    #[inline(always)]
    pub fn vrefl(&mut self) -> VREFL_W {
        VREFL_W { w: self }
    }
    #[doc = "Bit 7 - Enable software control input threshold"]
    #[inline(always)]
    pub fn vref_override(&mut self) -> VREF_OVERRIDE_W {
        VREF_OVERRIDE_W { w: self }
    }
    #[doc = "Bit 8 - Enable software control USB D+ D- pullup pulldown"]
    #[inline(always)]
    pub fn pad_pull_override(&mut self) -> PAD_PULL_OVERRIDE_W {
        PAD_PULL_OVERRIDE_W { w: self }
    }
    #[doc = "Bit 9 - Control USB D+ pull up."]
    #[inline(always)]
    pub fn dp_pullup(&mut self) -> DP_PULLUP_W {
        DP_PULLUP_W { w: self }
    }
    #[doc = "Bit 10 - Control USB D+ pull down."]
    #[inline(always)]
    pub fn dp_pulldown(&mut self) -> DP_PULLDOWN_W {
        DP_PULLDOWN_W { w: self }
    }
    #[doc = "Bit 11 - Control USB D- pull up."]
    #[inline(always)]
    pub fn dm_pullup(&mut self) -> DM_PULLUP_W {
        DM_PULLUP_W { w: self }
    }
    #[doc = "Bit 12 - Control USB D- pull down."]
    #[inline(always)]
    pub fn dm_pulldown(&mut self) -> DM_PULLDOWN_W {
        DM_PULLDOWN_W { w: self }
    }
    #[doc = "Bit 13 - Control pull up value."]
    #[inline(always)]
    pub fn pullup_value(&mut self) -> PULLUP_VALUE_W {
        PULLUP_VALUE_W { w: self }
    }
    #[doc = "Bit 14 - Enable USB pad function."]
    #[inline(always)]
    pub fn usb_pad_enable(&mut self) -> USB_PAD_ENABLE_W {
        USB_PAD_ENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB_DEVICE_CONF0_REG.\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conf0]
(index.html) module"]
pub struct CONF0_SPEC;
impl crate::RegisterSpec for CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conf0::R]
(R) reader structure"]
impl crate::Readable for CONF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conf0::W]
(W) writer structure"]
impl crate::Writable for CONF0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONF0 to value 0x4200"]
impl crate::Resettable for CONF0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4200
    }
}
