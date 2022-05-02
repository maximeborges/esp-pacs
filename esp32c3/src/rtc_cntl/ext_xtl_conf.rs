#[doc = "Register `EXT_XTL_CONF` reader"]
pub struct R(crate::R<EXT_XTL_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXT_XTL_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXT_XTL_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXT_XTL_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXT_XTL_CONF` writer"]
pub struct W(crate::W<EXT_XTL_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXT_XTL_CONF_SPEC>;
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
impl From<crate::W<EXT_XTL_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXT_XTL_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XTAL32K_WDT_EN` reader - xtal 32k watch dog enable"]
pub struct XTAL32K_WDT_EN_R(crate::FieldReader<bool>);
impl XTAL32K_WDT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XTAL32K_WDT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTAL32K_WDT_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTAL32K_WDT_EN` writer - xtal 32k watch dog enable"]
pub struct XTAL32K_WDT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL32K_WDT_EN_W<'a> {
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
#[doc = "Field `XTAL32K_WDT_CLK_FO` reader - xtal 32k watch dog clock force on"]
pub struct XTAL32K_WDT_CLK_FO_R(crate::FieldReader<bool>);
impl XTAL32K_WDT_CLK_FO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XTAL32K_WDT_CLK_FO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTAL32K_WDT_CLK_FO_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTAL32K_WDT_CLK_FO` writer - xtal 32k watch dog clock force on"]
pub struct XTAL32K_WDT_CLK_FO_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL32K_WDT_CLK_FO_W<'a> {
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
#[doc = "Field `XTAL32K_WDT_RESET` reader - xtal 32k watch dog sw reset"]
pub struct XTAL32K_WDT_RESET_R(crate::FieldReader<bool>);
impl XTAL32K_WDT_RESET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XTAL32K_WDT_RESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTAL32K_WDT_RESET_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTAL32K_WDT_RESET` writer - xtal 32k watch dog sw reset"]
pub struct XTAL32K_WDT_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL32K_WDT_RESET_W<'a> {
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
#[doc = "Field `XTAL32K_EXT_CLK_FO` reader - xtal 32k external xtal clock force on"]
pub struct XTAL32K_EXT_CLK_FO_R(crate::FieldReader<bool>);
impl XTAL32K_EXT_CLK_FO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XTAL32K_EXT_CLK_FO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTAL32K_EXT_CLK_FO_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTAL32K_EXT_CLK_FO` writer - xtal 32k external xtal clock force on"]
pub struct XTAL32K_EXT_CLK_FO_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL32K_EXT_CLK_FO_W<'a> {
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
#[doc = "Field `XTAL32K_AUTO_BACKUP` reader - xtal 32k switch to back up clock when xtal is dead"]
pub struct XTAL32K_AUTO_BACKUP_R(crate::FieldReader<bool>);
impl XTAL32K_AUTO_BACKUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XTAL32K_AUTO_BACKUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTAL32K_AUTO_BACKUP_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTAL32K_AUTO_BACKUP` writer - xtal 32k switch to back up clock when xtal is dead"]
pub struct XTAL32K_AUTO_BACKUP_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL32K_AUTO_BACKUP_W<'a> {
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
#[doc = "Field `XTAL32K_AUTO_RESTART` reader - xtal 32k restart xtal when xtal is dead"]
pub struct XTAL32K_AUTO_RESTART_R(crate::FieldReader<bool>);
impl XTAL32K_AUTO_RESTART_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XTAL32K_AUTO_RESTART_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTAL32K_AUTO_RESTART_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTAL32K_AUTO_RESTART` writer - xtal 32k restart xtal when xtal is dead"]
pub struct XTAL32K_AUTO_RESTART_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL32K_AUTO_RESTART_W<'a> {
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
#[doc = "Field `XTAL32K_AUTO_RETURN` reader - xtal 32k switch back xtal when xtal is restarted"]
pub struct XTAL32K_AUTO_RETURN_R(crate::FieldReader<bool>);
impl XTAL32K_AUTO_RETURN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XTAL32K_AUTO_RETURN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTAL32K_AUTO_RETURN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTAL32K_AUTO_RETURN` writer - xtal 32k switch back xtal when xtal is restarted"]
pub struct XTAL32K_AUTO_RETURN_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL32K_AUTO_RETURN_W<'a> {
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
#[doc = "Field `XTAL32K_XPD_FORCE` reader - Xtal 32k xpd control by sw or fsm"]
pub struct XTAL32K_XPD_FORCE_R(crate::FieldReader<bool>);
impl XTAL32K_XPD_FORCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XTAL32K_XPD_FORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTAL32K_XPD_FORCE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTAL32K_XPD_FORCE` writer - Xtal 32k xpd control by sw or fsm"]
pub struct XTAL32K_XPD_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL32K_XPD_FORCE_W<'a> {
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
#[doc = "Field `ENCKINIT_XTAL_32K` reader - apply an internal clock to help xtal 32k to start"]
pub struct ENCKINIT_XTAL_32K_R(crate::FieldReader<bool>);
impl ENCKINIT_XTAL_32K_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENCKINIT_XTAL_32K_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENCKINIT_XTAL_32K_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENCKINIT_XTAL_32K` writer - apply an internal clock to help xtal 32k to start"]
pub struct ENCKINIT_XTAL_32K_W<'a> {
    w: &'a mut W,
}
impl<'a> ENCKINIT_XTAL_32K_W<'a> {
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
#[doc = "Field `DBUF_XTAL_32K` reader - 0: single-end buffer 1: differential buffer"]
pub struct DBUF_XTAL_32K_R(crate::FieldReader<bool>);
impl DBUF_XTAL_32K_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DBUF_XTAL_32K_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBUF_XTAL_32K_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBUF_XTAL_32K` writer - 0: single-end buffer 1: differential buffer"]
pub struct DBUF_XTAL_32K_W<'a> {
    w: &'a mut W,
}
impl<'a> DBUF_XTAL_32K_W<'a> {
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
#[doc = "Field `DGM_XTAL_32K` reader - xtal_32k gm control"]
pub struct DGM_XTAL_32K_R(crate::FieldReader<u8>);
impl DGM_XTAL_32K_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DGM_XTAL_32K_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DGM_XTAL_32K_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DGM_XTAL_32K` writer - xtal_32k gm control"]
pub struct DGM_XTAL_32K_W<'a> {
    w: &'a mut W,
}
impl<'a> DGM_XTAL_32K_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 10)) | ((value as u32 & 7) << 10);
        self.w
    }
}
#[doc = "Field `DRES_XTAL_32K` reader - DRES_XTAL_32K"]
pub struct DRES_XTAL_32K_R(crate::FieldReader<u8>);
impl DRES_XTAL_32K_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DRES_XTAL_32K_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DRES_XTAL_32K_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DRES_XTAL_32K` writer - DRES_XTAL_32K"]
pub struct DRES_XTAL_32K_W<'a> {
    w: &'a mut W,
}
impl<'a> DRES_XTAL_32K_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 13)) | ((value as u32 & 7) << 13);
        self.w
    }
}
#[doc = "Field `XPD_XTAL_32K` reader - XPD_XTAL_32K"]
pub struct XPD_XTAL_32K_R(crate::FieldReader<bool>);
impl XPD_XTAL_32K_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XPD_XTAL_32K_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XPD_XTAL_32K_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XPD_XTAL_32K` writer - XPD_XTAL_32K"]
pub struct XPD_XTAL_32K_W<'a> {
    w: &'a mut W,
}
impl<'a> XPD_XTAL_32K_W<'a> {
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
#[doc = "Field `DAC_XTAL_32K` reader - DAC_XTAL_32K"]
pub struct DAC_XTAL_32K_R(crate::FieldReader<u8>);
impl DAC_XTAL_32K_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DAC_XTAL_32K_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAC_XTAL_32K_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAC_XTAL_32K` writer - DAC_XTAL_32K"]
pub struct DAC_XTAL_32K_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_XTAL_32K_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 17)) | ((value as u32 & 7) << 17);
        self.w
    }
}
#[doc = "Field `RTC_WDT_STATE` reader - state of 32k_wdt"]
pub struct RTC_WDT_STATE_R(crate::FieldReader<u8>);
impl RTC_WDT_STATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RTC_WDT_STATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_WDT_STATE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_XTAL32K_GPIO_SEL` reader - XTAL_32K sel. 0: external XTAL_32K"]
pub struct RTC_XTAL32K_GPIO_SEL_R(crate::FieldReader<bool>);
impl RTC_XTAL32K_GPIO_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_XTAL32K_GPIO_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_XTAL32K_GPIO_SEL_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_XTAL32K_GPIO_SEL` writer - XTAL_32K sel. 0: external XTAL_32K"]
pub struct RTC_XTAL32K_GPIO_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_XTAL32K_GPIO_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 23)) | ((value as u32 & 1) << 23);
        self.w
    }
}
#[doc = "Field `XTL_EXT_CTR_LV` reader - 0: power down XTAL at high level"]
pub struct XTL_EXT_CTR_LV_R(crate::FieldReader<bool>);
impl XTL_EXT_CTR_LV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XTL_EXT_CTR_LV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTL_EXT_CTR_LV_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTL_EXT_CTR_LV` writer - 0: power down XTAL at high level"]
pub struct XTL_EXT_CTR_LV_W<'a> {
    w: &'a mut W,
}
impl<'a> XTL_EXT_CTR_LV_W<'a> {
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
#[doc = "Field `XTL_EXT_CTR_EN` reader - enable gpio configure xtal power on"]
pub struct XTL_EXT_CTR_EN_R(crate::FieldReader<bool>);
impl XTL_EXT_CTR_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XTL_EXT_CTR_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTL_EXT_CTR_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTL_EXT_CTR_EN` writer - enable gpio configure xtal power on"]
pub struct XTL_EXT_CTR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> XTL_EXT_CTR_EN_W<'a> {
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
    #[doc = "Bit 0 - xtal 32k watch dog enable"]
    #[inline(always)]
    pub fn xtal32k_wdt_en(&self) -> XTAL32K_WDT_EN_R {
        XTAL32K_WDT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - xtal 32k watch dog clock force on"]
    #[inline(always)]
    pub fn xtal32k_wdt_clk_fo(&self) -> XTAL32K_WDT_CLK_FO_R {
        XTAL32K_WDT_CLK_FO_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - xtal 32k watch dog sw reset"]
    #[inline(always)]
    pub fn xtal32k_wdt_reset(&self) -> XTAL32K_WDT_RESET_R {
        XTAL32K_WDT_RESET_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - xtal 32k external xtal clock force on"]
    #[inline(always)]
    pub fn xtal32k_ext_clk_fo(&self) -> XTAL32K_EXT_CLK_FO_R {
        XTAL32K_EXT_CLK_FO_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - xtal 32k switch to back up clock when xtal is dead"]
    #[inline(always)]
    pub fn xtal32k_auto_backup(&self) -> XTAL32K_AUTO_BACKUP_R {
        XTAL32K_AUTO_BACKUP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - xtal 32k restart xtal when xtal is dead"]
    #[inline(always)]
    pub fn xtal32k_auto_restart(&self) -> XTAL32K_AUTO_RESTART_R {
        XTAL32K_AUTO_RESTART_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - xtal 32k switch back xtal when xtal is restarted"]
    #[inline(always)]
    pub fn xtal32k_auto_return(&self) -> XTAL32K_AUTO_RETURN_R {
        XTAL32K_AUTO_RETURN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Xtal 32k xpd control by sw or fsm"]
    #[inline(always)]
    pub fn xtal32k_xpd_force(&self) -> XTAL32K_XPD_FORCE_R {
        XTAL32K_XPD_FORCE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - apply an internal clock to help xtal 32k to start"]
    #[inline(always)]
    pub fn enckinit_xtal_32k(&self) -> ENCKINIT_XTAL_32K_R {
        ENCKINIT_XTAL_32K_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 0: single-end buffer 1: differential buffer"]
    #[inline(always)]
    pub fn dbuf_xtal_32k(&self) -> DBUF_XTAL_32K_R {
        DBUF_XTAL_32K_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:12 - xtal_32k gm control"]
    #[inline(always)]
    pub fn dgm_xtal_32k(&self) -> DGM_XTAL_32K_R {
        DGM_XTAL_32K_R::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bits 13:15 - DRES_XTAL_32K"]
    #[inline(always)]
    pub fn dres_xtal_32k(&self) -> DRES_XTAL_32K_R {
        DRES_XTAL_32K_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bit 16 - XPD_XTAL_32K"]
    #[inline(always)]
    pub fn xpd_xtal_32k(&self) -> XPD_XTAL_32K_R {
        XPD_XTAL_32K_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - DAC_XTAL_32K"]
    #[inline(always)]
    pub fn dac_xtal_32k(&self) -> DAC_XTAL_32K_R {
        DAC_XTAL_32K_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:22 - state of 32k_wdt"]
    #[inline(always)]
    pub fn rtc_wdt_state(&self) -> RTC_WDT_STATE_R {
        RTC_WDT_STATE_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - XTAL_32K sel. 0: external XTAL_32K"]
    #[inline(always)]
    pub fn rtc_xtal32k_gpio_sel(&self) -> RTC_XTAL32K_GPIO_SEL_R {
        RTC_XTAL32K_GPIO_SEL_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 30 - 0: power down XTAL at high level"]
    #[inline(always)]
    pub fn xtl_ext_ctr_lv(&self) -> XTL_EXT_CTR_LV_R {
        XTL_EXT_CTR_LV_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - enable gpio configure xtal power on"]
    #[inline(always)]
    pub fn xtl_ext_ctr_en(&self) -> XTL_EXT_CTR_EN_R {
        XTL_EXT_CTR_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - xtal 32k watch dog enable"]
    #[inline(always)]
    pub fn xtal32k_wdt_en(&mut self) -> XTAL32K_WDT_EN_W {
        XTAL32K_WDT_EN_W { w: self }
    }
    #[doc = "Bit 1 - xtal 32k watch dog clock force on"]
    #[inline(always)]
    pub fn xtal32k_wdt_clk_fo(&mut self) -> XTAL32K_WDT_CLK_FO_W {
        XTAL32K_WDT_CLK_FO_W { w: self }
    }
    #[doc = "Bit 2 - xtal 32k watch dog sw reset"]
    #[inline(always)]
    pub fn xtal32k_wdt_reset(&mut self) -> XTAL32K_WDT_RESET_W {
        XTAL32K_WDT_RESET_W { w: self }
    }
    #[doc = "Bit 3 - xtal 32k external xtal clock force on"]
    #[inline(always)]
    pub fn xtal32k_ext_clk_fo(&mut self) -> XTAL32K_EXT_CLK_FO_W {
        XTAL32K_EXT_CLK_FO_W { w: self }
    }
    #[doc = "Bit 4 - xtal 32k switch to back up clock when xtal is dead"]
    #[inline(always)]
    pub fn xtal32k_auto_backup(&mut self) -> XTAL32K_AUTO_BACKUP_W {
        XTAL32K_AUTO_BACKUP_W { w: self }
    }
    #[doc = "Bit 5 - xtal 32k restart xtal when xtal is dead"]
    #[inline(always)]
    pub fn xtal32k_auto_restart(&mut self) -> XTAL32K_AUTO_RESTART_W {
        XTAL32K_AUTO_RESTART_W { w: self }
    }
    #[doc = "Bit 6 - xtal 32k switch back xtal when xtal is restarted"]
    #[inline(always)]
    pub fn xtal32k_auto_return(&mut self) -> XTAL32K_AUTO_RETURN_W {
        XTAL32K_AUTO_RETURN_W { w: self }
    }
    #[doc = "Bit 7 - Xtal 32k xpd control by sw or fsm"]
    #[inline(always)]
    pub fn xtal32k_xpd_force(&mut self) -> XTAL32K_XPD_FORCE_W {
        XTAL32K_XPD_FORCE_W { w: self }
    }
    #[doc = "Bit 8 - apply an internal clock to help xtal 32k to start"]
    #[inline(always)]
    pub fn enckinit_xtal_32k(&mut self) -> ENCKINIT_XTAL_32K_W {
        ENCKINIT_XTAL_32K_W { w: self }
    }
    #[doc = "Bit 9 - 0: single-end buffer 1: differential buffer"]
    #[inline(always)]
    pub fn dbuf_xtal_32k(&mut self) -> DBUF_XTAL_32K_W {
        DBUF_XTAL_32K_W { w: self }
    }
    #[doc = "Bits 10:12 - xtal_32k gm control"]
    #[inline(always)]
    pub fn dgm_xtal_32k(&mut self) -> DGM_XTAL_32K_W {
        DGM_XTAL_32K_W { w: self }
    }
    #[doc = "Bits 13:15 - DRES_XTAL_32K"]
    #[inline(always)]
    pub fn dres_xtal_32k(&mut self) -> DRES_XTAL_32K_W {
        DRES_XTAL_32K_W { w: self }
    }
    #[doc = "Bit 16 - XPD_XTAL_32K"]
    #[inline(always)]
    pub fn xpd_xtal_32k(&mut self) -> XPD_XTAL_32K_W {
        XPD_XTAL_32K_W { w: self }
    }
    #[doc = "Bits 17:19 - DAC_XTAL_32K"]
    #[inline(always)]
    pub fn dac_xtal_32k(&mut self) -> DAC_XTAL_32K_W {
        DAC_XTAL_32K_W { w: self }
    }
    #[doc = "Bit 23 - XTAL_32K sel. 0: external XTAL_32K"]
    #[inline(always)]
    pub fn rtc_xtal32k_gpio_sel(&mut self) -> RTC_XTAL32K_GPIO_SEL_W {
        RTC_XTAL32K_GPIO_SEL_W { w: self }
    }
    #[doc = "Bit 30 - 0: power down XTAL at high level"]
    #[inline(always)]
    pub fn xtl_ext_ctr_lv(&mut self) -> XTL_EXT_CTR_LV_W {
        XTL_EXT_CTR_LV_W { w: self }
    }
    #[doc = "Bit 31 - enable gpio configure xtal power on"]
    #[inline(always)]
    pub fn xtl_ext_ctr_en(&mut self) -> XTL_EXT_CTR_EN_W {
        XTL_EXT_CTR_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rtc configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ext_xtl_conf](index.html) module"]
pub struct EXT_XTL_CONF_SPEC;
impl crate::RegisterSpec for EXT_XTL_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ext_xtl_conf::R](R) reader structure"]
impl crate::Readable for EXT_XTL_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ext_xtl_conf::W](W) writer structure"]
impl crate::Writable for EXT_XTL_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXT_XTL_CONF to value 0x0006_6c80"]
impl crate::Resettable for EXT_XTL_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0006_6c80
    }
}
