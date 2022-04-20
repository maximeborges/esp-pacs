#[doc = "Register `OTG_CONF` reader"]
pub struct R(crate::R<OTG_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_CONF` writer"]
pub struct W(crate::W<OTG_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_CONF_SPEC>;
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
impl From<crate::W<OTG_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRP_SESSEND_OVERRIDE` reader - This bit is used to enable the software over-ride of srp session end signal. 1'b0: the signal is controlled by the chip input. 1'b1: the signal is controlled by the software."]
pub struct SRP_SESSEND_OVERRIDE_R(crate::FieldReader<bool, bool>);
impl SRP_SESSEND_OVERRIDE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRP_SESSEND_OVERRIDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRP_SESSEND_OVERRIDE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRP_SESSEND_OVERRIDE` writer - This bit is used to enable the software over-ride of srp session end signal. 1'b0: the signal is controlled by the chip input. 1'b1: the signal is controlled by the software."]
pub struct SRP_SESSEND_OVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> SRP_SESSEND_OVERRIDE_W<'a> {
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
#[doc = "Field `SRP_SESSEND_VALUE` reader - Software over-ride value of srp session end signal."]
pub struct SRP_SESSEND_VALUE_R(crate::FieldReader<bool, bool>);
impl SRP_SESSEND_VALUE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRP_SESSEND_VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRP_SESSEND_VALUE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRP_SESSEND_VALUE` writer - Software over-ride value of srp session end signal."]
pub struct SRP_SESSEND_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> SRP_SESSEND_VALUE_W<'a> {
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
#[doc = "Field `PHY_SEL` reader - Select internal external PHY. 1'b0: Select internal PHY. 1'b1: Select external PHY."]
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
#[doc = "Field `PHY_SEL` writer - Select internal external PHY. 1'b0: Select internal PHY. 1'b1: Select external PHY."]
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
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u32 & 1) << 2);
        self.w
    }
}
#[doc = "Field `DFIFO_FORCE_PD` reader - Force the dfifo to go into low power mode. The data in dfifo will not lost."]
pub struct DFIFO_FORCE_PD_R(crate::FieldReader<bool, bool>);
impl DFIFO_FORCE_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DFIFO_FORCE_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFIFO_FORCE_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFIFO_FORCE_PD` writer - Force the dfifo to go into low power mode. The data in dfifo will not lost."]
pub struct DFIFO_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> DFIFO_FORCE_PD_W<'a> {
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
#[doc = "Field `DBNCE_FLTR_BYPASS` reader - Bypass Debounce filters for avalid,bvalid,vbusvalid,session end, id signals"]
pub struct DBNCE_FLTR_BYPASS_R(crate::FieldReader<bool, bool>);
impl DBNCE_FLTR_BYPASS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DBNCE_FLTR_BYPASS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBNCE_FLTR_BYPASS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBNCE_FLTR_BYPASS` writer - Bypass Debounce filters for avalid,bvalid,vbusvalid,session end, id signals"]
pub struct DBNCE_FLTR_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> DBNCE_FLTR_BYPASS_W<'a> {
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
#[doc = "Field `EXCHG_PINS_OVERRIDE` reader - Enable software controlle USB D+ D- exchange"]
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
#[doc = "Field `EXCHG_PINS_OVERRIDE` writer - Enable software controlle USB D+ D- exchange"]
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
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u32 & 1) << 5);
        self.w
    }
}
#[doc = "Field `EXCHG_PINS` reader - USB D+ D- exchange. 1'b0: don't change. 1'b1: exchange D+ D-"]
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
#[doc = "Field `EXCHG_PINS` writer - USB D+ D- exchange. 1'b0: don't change. 1'b1: exchange D+ D-"]
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
        self.w.bits = (self.w.bits & !(1 << 6)) | ((value as u32 & 1) << 6);
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
        self.w.bits = (self.w.bits & !(3 << 7)) | ((value as u32 & 3) << 7);
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
        self.w.bits = (self.w.bits & !(3 << 9)) | ((value as u32 & 3) << 9);
        self.w
    }
}
#[doc = "Field `VREF_OVERRIDE` reader - Enable software controlle input threshold"]
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
#[doc = "Field `VREF_OVERRIDE` writer - Enable software controlle input threshold"]
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
        self.w.bits = (self.w.bits & !(1 << 11)) | ((value as u32 & 1) << 11);
        self.w
    }
}
#[doc = "Field `PAD_PULL_OVERRIDE` reader - Enable software controlle USB D+ D- pullup pulldown"]
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
#[doc = "Field `PAD_PULL_OVERRIDE` writer - Enable software controlle USB D+ D- pullup pulldown"]
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
        self.w.bits = (self.w.bits & !(1 << 12)) | ((value as u32 & 1) << 12);
        self.w
    }
}
#[doc = "Field `DP_PULLUP` reader - Controlle USB D+ pullup"]
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
#[doc = "Field `DP_PULLUP` writer - Controlle USB D+ pullup"]
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
        self.w.bits = (self.w.bits & !(1 << 13)) | ((value as u32 & 1) << 13);
        self.w
    }
}
#[doc = "Field `DP_PULLDOWN` reader - Controlle USB D+ pulldown"]
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
#[doc = "Field `DP_PULLDOWN` writer - Controlle USB D+ pulldown"]
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
        self.w.bits = (self.w.bits & !(1 << 14)) | ((value as u32 & 1) << 14);
        self.w
    }
}
#[doc = "Field `DM_PULLUP` reader - Controlle USB D+ pullup"]
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
#[doc = "Field `DM_PULLUP` writer - Controlle USB D+ pullup"]
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
        self.w.bits = (self.w.bits & !(1 << 15)) | ((value as u32 & 1) << 15);
        self.w
    }
}
#[doc = "Field `DM_PULLDOWN` reader - Controlle USB D+ pulldown"]
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
#[doc = "Field `DM_PULLDOWN` writer - Controlle USB D+ pulldown"]
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
        self.w.bits = (self.w.bits & !(1 << 16)) | ((value as u32 & 1) << 16);
        self.w
    }
}
#[doc = "Field `PULLUP_VALUE` reader - Controlle pullup value. 1'b0: typical value is 2.4K. 1'b1: typical value is 1.2K."]
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
#[doc = "Field `PULLUP_VALUE` writer - Controlle pullup value. 1'b0: typical value is 2.4K. 1'b1: typical value is 1.2K."]
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
        self.w.bits = (self.w.bits & !(1 << 17)) | ((value as u32 & 1) << 17);
        self.w
    }
}
#[doc = "Field `USB_PAD_ENABLE` reader - Enable USB pad function"]
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
#[doc = "Field `USB_PAD_ENABLE` writer - Enable USB pad function"]
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
        self.w.bits = (self.w.bits & !(1 << 18)) | ((value as u32 & 1) << 18);
        self.w
    }
}
#[doc = "Field `AHB_CLK_FORCE_ON` reader - Force ahb clock always on"]
pub struct AHB_CLK_FORCE_ON_R(crate::FieldReader<bool, bool>);
impl AHB_CLK_FORCE_ON_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AHB_CLK_FORCE_ON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AHB_CLK_FORCE_ON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AHB_CLK_FORCE_ON` writer - Force ahb clock always on"]
pub struct AHB_CLK_FORCE_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> AHB_CLK_FORCE_ON_W<'a> {
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
#[doc = "Field `PHY_CLK_FORCE_ON` reader - Force phy clock always on"]
pub struct PHY_CLK_FORCE_ON_R(crate::FieldReader<bool, bool>);
impl PHY_CLK_FORCE_ON_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PHY_CLK_FORCE_ON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PHY_CLK_FORCE_ON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PHY_CLK_FORCE_ON` writer - Force phy clock always on"]
pub struct PHY_CLK_FORCE_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> PHY_CLK_FORCE_ON_W<'a> {
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
#[doc = "Field `PHY_TX_EDGE_SEL` reader - Select phy tx signal output clock edge. 1'b0: negedge. 1'b1: posedge."]
pub struct PHY_TX_EDGE_SEL_R(crate::FieldReader<bool, bool>);
impl PHY_TX_EDGE_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PHY_TX_EDGE_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PHY_TX_EDGE_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PHY_TX_EDGE_SEL` writer - Select phy tx signal output clock edge. 1'b0: negedge. 1'b1: posedge."]
pub struct PHY_TX_EDGE_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PHY_TX_EDGE_SEL_W<'a> {
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
#[doc = "Field `DFIFO_FORCE_PU` reader - Disable the dfifo to go into low power mode. The data in dfifo will not lost."]
pub struct DFIFO_FORCE_PU_R(crate::FieldReader<bool, bool>);
impl DFIFO_FORCE_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DFIFO_FORCE_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFIFO_FORCE_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFIFO_FORCE_PU` writer - Disable the dfifo to go into low power mode. The data in dfifo will not lost."]
pub struct DFIFO_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> DFIFO_FORCE_PU_W<'a> {
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
#[doc = "Field `CLK_EN` reader - Disable auto clock gating of CSR registers"]
pub struct CLK_EN_R(crate::FieldReader<bool, bool>);
impl CLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK_EN` writer - Disable auto clock gating of CSR registers"]
pub struct CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 31)) | ((value as u32 & 1) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - This bit is used to enable the software over-ride of srp session end signal. 1'b0: the signal is controlled by the chip input. 1'b1: the signal is controlled by the software."]
    #[inline(always)]
    pub fn srp_sessend_override(&self) -> SRP_SESSEND_OVERRIDE_R {
        SRP_SESSEND_OVERRIDE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Software over-ride value of srp session end signal."]
    #[inline(always)]
    pub fn srp_sessend_value(&self) -> SRP_SESSEND_VALUE_R {
        SRP_SESSEND_VALUE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Select internal external PHY. 1'b0: Select internal PHY. 1'b1: Select external PHY."]
    #[inline(always)]
    pub fn phy_sel(&self) -> PHY_SEL_R {
        PHY_SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Force the dfifo to go into low power mode. The data in dfifo will not lost."]
    #[inline(always)]
    pub fn dfifo_force_pd(&self) -> DFIFO_FORCE_PD_R {
        DFIFO_FORCE_PD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Bypass Debounce filters for avalid,bvalid,vbusvalid,session end, id signals"]
    #[inline(always)]
    pub fn dbnce_fltr_bypass(&self) -> DBNCE_FLTR_BYPASS_R {
        DBNCE_FLTR_BYPASS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable software controlle USB D+ D- exchange"]
    #[inline(always)]
    pub fn exchg_pins_override(&self) -> EXCHG_PINS_OVERRIDE_R {
        EXCHG_PINS_OVERRIDE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - USB D+ D- exchange. 1'b0: don't change. 1'b1: exchange D+ D-"]
    #[inline(always)]
    pub fn exchg_pins(&self) -> EXCHG_PINS_R {
        EXCHG_PINS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8 - Control single-end input high threshold,1.76V to 2V, step 80mV"]
    #[inline(always)]
    pub fn vrefh(&self) -> VREFH_R {
        VREFH_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 9:10 - Control single-end input low threshold,0.8V to 1.04V, step 80mV"]
    #[inline(always)]
    pub fn vrefl(&self) -> VREFL_R {
        VREFL_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - Enable software controlle input threshold"]
    #[inline(always)]
    pub fn vref_override(&self) -> VREF_OVERRIDE_R {
        VREF_OVERRIDE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable software controlle USB D+ D- pullup pulldown"]
    #[inline(always)]
    pub fn pad_pull_override(&self) -> PAD_PULL_OVERRIDE_R {
        PAD_PULL_OVERRIDE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Controlle USB D+ pullup"]
    #[inline(always)]
    pub fn dp_pullup(&self) -> DP_PULLUP_R {
        DP_PULLUP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Controlle USB D+ pulldown"]
    #[inline(always)]
    pub fn dp_pulldown(&self) -> DP_PULLDOWN_R {
        DP_PULLDOWN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Controlle USB D+ pullup"]
    #[inline(always)]
    pub fn dm_pullup(&self) -> DM_PULLUP_R {
        DM_PULLUP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Controlle USB D+ pulldown"]
    #[inline(always)]
    pub fn dm_pulldown(&self) -> DM_PULLDOWN_R {
        DM_PULLDOWN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Controlle pullup value. 1'b0: typical value is 2.4K. 1'b1: typical value is 1.2K."]
    #[inline(always)]
    pub fn pullup_value(&self) -> PULLUP_VALUE_R {
        PULLUP_VALUE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable USB pad function"]
    #[inline(always)]
    pub fn usb_pad_enable(&self) -> USB_PAD_ENABLE_R {
        USB_PAD_ENABLE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Force ahb clock always on"]
    #[inline(always)]
    pub fn ahb_clk_force_on(&self) -> AHB_CLK_FORCE_ON_R {
        AHB_CLK_FORCE_ON_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Force phy clock always on"]
    #[inline(always)]
    pub fn phy_clk_force_on(&self) -> PHY_CLK_FORCE_ON_R {
        PHY_CLK_FORCE_ON_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Select phy tx signal output clock edge. 1'b0: negedge. 1'b1: posedge."]
    #[inline(always)]
    pub fn phy_tx_edge_sel(&self) -> PHY_TX_EDGE_SEL_R {
        PHY_TX_EDGE_SEL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Disable the dfifo to go into low power mode. The data in dfifo will not lost."]
    #[inline(always)]
    pub fn dfifo_force_pu(&self) -> DFIFO_FORCE_PU_R {
        DFIFO_FORCE_PU_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 31 - Disable auto clock gating of CSR registers"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is used to enable the software over-ride of srp session end signal. 1'b0: the signal is controlled by the chip input. 1'b1: the signal is controlled by the software."]
    #[inline(always)]
    pub fn srp_sessend_override(&mut self) -> SRP_SESSEND_OVERRIDE_W {
        SRP_SESSEND_OVERRIDE_W { w: self }
    }
    #[doc = "Bit 1 - Software over-ride value of srp session end signal."]
    #[inline(always)]
    pub fn srp_sessend_value(&mut self) -> SRP_SESSEND_VALUE_W {
        SRP_SESSEND_VALUE_W { w: self }
    }
    #[doc = "Bit 2 - Select internal external PHY. 1'b0: Select internal PHY. 1'b1: Select external PHY."]
    #[inline(always)]
    pub fn phy_sel(&mut self) -> PHY_SEL_W {
        PHY_SEL_W { w: self }
    }
    #[doc = "Bit 3 - Force the dfifo to go into low power mode. The data in dfifo will not lost."]
    #[inline(always)]
    pub fn dfifo_force_pd(&mut self) -> DFIFO_FORCE_PD_W {
        DFIFO_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 4 - Bypass Debounce filters for avalid,bvalid,vbusvalid,session end, id signals"]
    #[inline(always)]
    pub fn dbnce_fltr_bypass(&mut self) -> DBNCE_FLTR_BYPASS_W {
        DBNCE_FLTR_BYPASS_W { w: self }
    }
    #[doc = "Bit 5 - Enable software controlle USB D+ D- exchange"]
    #[inline(always)]
    pub fn exchg_pins_override(&mut self) -> EXCHG_PINS_OVERRIDE_W {
        EXCHG_PINS_OVERRIDE_W { w: self }
    }
    #[doc = "Bit 6 - USB D+ D- exchange. 1'b0: don't change. 1'b1: exchange D+ D-"]
    #[inline(always)]
    pub fn exchg_pins(&mut self) -> EXCHG_PINS_W {
        EXCHG_PINS_W { w: self }
    }
    #[doc = "Bits 7:8 - Control single-end input high threshold,1.76V to 2V, step 80mV"]
    #[inline(always)]
    pub fn vrefh(&mut self) -> VREFH_W {
        VREFH_W { w: self }
    }
    #[doc = "Bits 9:10 - Control single-end input low threshold,0.8V to 1.04V, step 80mV"]
    #[inline(always)]
    pub fn vrefl(&mut self) -> VREFL_W {
        VREFL_W { w: self }
    }
    #[doc = "Bit 11 - Enable software controlle input threshold"]
    #[inline(always)]
    pub fn vref_override(&mut self) -> VREF_OVERRIDE_W {
        VREF_OVERRIDE_W { w: self }
    }
    #[doc = "Bit 12 - Enable software controlle USB D+ D- pullup pulldown"]
    #[inline(always)]
    pub fn pad_pull_override(&mut self) -> PAD_PULL_OVERRIDE_W {
        PAD_PULL_OVERRIDE_W { w: self }
    }
    #[doc = "Bit 13 - Controlle USB D+ pullup"]
    #[inline(always)]
    pub fn dp_pullup(&mut self) -> DP_PULLUP_W {
        DP_PULLUP_W { w: self }
    }
    #[doc = "Bit 14 - Controlle USB D+ pulldown"]
    #[inline(always)]
    pub fn dp_pulldown(&mut self) -> DP_PULLDOWN_W {
        DP_PULLDOWN_W { w: self }
    }
    #[doc = "Bit 15 - Controlle USB D+ pullup"]
    #[inline(always)]
    pub fn dm_pullup(&mut self) -> DM_PULLUP_W {
        DM_PULLUP_W { w: self }
    }
    #[doc = "Bit 16 - Controlle USB D+ pulldown"]
    #[inline(always)]
    pub fn dm_pulldown(&mut self) -> DM_PULLDOWN_W {
        DM_PULLDOWN_W { w: self }
    }
    #[doc = "Bit 17 - Controlle pullup value. 1'b0: typical value is 2.4K. 1'b1: typical value is 1.2K."]
    #[inline(always)]
    pub fn pullup_value(&mut self) -> PULLUP_VALUE_W {
        PULLUP_VALUE_W { w: self }
    }
    #[doc = "Bit 18 - Enable USB pad function"]
    #[inline(always)]
    pub fn usb_pad_enable(&mut self) -> USB_PAD_ENABLE_W {
        USB_PAD_ENABLE_W { w: self }
    }
    #[doc = "Bit 19 - Force ahb clock always on"]
    #[inline(always)]
    pub fn ahb_clk_force_on(&mut self) -> AHB_CLK_FORCE_ON_W {
        AHB_CLK_FORCE_ON_W { w: self }
    }
    #[doc = "Bit 20 - Force phy clock always on"]
    #[inline(always)]
    pub fn phy_clk_force_on(&mut self) -> PHY_CLK_FORCE_ON_W {
        PHY_CLK_FORCE_ON_W { w: self }
    }
    #[doc = "Bit 21 - Select phy tx signal output clock edge. 1'b0: negedge. 1'b1: posedge."]
    #[inline(always)]
    pub fn phy_tx_edge_sel(&mut self) -> PHY_TX_EDGE_SEL_W {
        PHY_TX_EDGE_SEL_W { w: self }
    }
    #[doc = "Bit 22 - Disable the dfifo to go into low power mode. The data in dfifo will not lost."]
    #[inline(always)]
    pub fn dfifo_force_pu(&mut self) -> DFIFO_FORCE_PU_W {
        DFIFO_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 31 - Disable auto clock gating of CSR registers"]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W {
        CLK_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB OTG Wrapper Configure Register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_conf]
(index.html) module"]
pub struct OTG_CONF_SPEC;
impl crate::RegisterSpec for OTG_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_conf::R]
(R) reader structure"]
impl crate::Readable for OTG_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_conf::W]
(W) writer structure"]
impl crate::Writable for OTG_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTG_CONF to value 0x001c_0000"]
impl crate::Resettable for OTG_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x001c_0000
    }
}
