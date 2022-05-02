#[doc = "Register `BIAS_CONF` reader"]
pub struct R(crate::R<BIAS_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BIAS_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BIAS_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BIAS_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BIAS_CONF` writer"]
pub struct W(crate::W<BIAS_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BIAS_CONF_SPEC>;
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
impl From<crate::W<BIAS_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BIAS_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BIAS_BUF_IDLE` reader - open bias buf when system in active"]
pub struct BIAS_BUF_IDLE_R(crate::FieldReader<bool>);
impl BIAS_BUF_IDLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BIAS_BUF_IDLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BIAS_BUF_IDLE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BIAS_BUF_IDLE` writer - open bias buf when system in active"]
pub struct BIAS_BUF_IDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_BUF_IDLE_W<'a> {
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
#[doc = "Field `BIAS_BUF_WAKE` reader - open bias buf when rtc in wakeup"]
pub struct BIAS_BUF_WAKE_R(crate::FieldReader<bool>);
impl BIAS_BUF_WAKE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BIAS_BUF_WAKE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BIAS_BUF_WAKE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BIAS_BUF_WAKE` writer - open bias buf when rtc in wakeup"]
pub struct BIAS_BUF_WAKE_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_BUF_WAKE_W<'a> {
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
#[doc = "Field `BIAS_BUF_DEEP_SLP` reader - open bias buf when rtc in deep sleep"]
pub struct BIAS_BUF_DEEP_SLP_R(crate::FieldReader<bool>);
impl BIAS_BUF_DEEP_SLP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BIAS_BUF_DEEP_SLP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BIAS_BUF_DEEP_SLP_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BIAS_BUF_DEEP_SLP` writer - open bias buf when rtc in deep sleep"]
pub struct BIAS_BUF_DEEP_SLP_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_BUF_DEEP_SLP_W<'a> {
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
#[doc = "Field `BIAS_BUF_MONITOR` reader - open bias buf when rtc in monitor state"]
pub struct BIAS_BUF_MONITOR_R(crate::FieldReader<bool>);
impl BIAS_BUF_MONITOR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BIAS_BUF_MONITOR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BIAS_BUF_MONITOR_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BIAS_BUF_MONITOR` writer - open bias buf when rtc in monitor state"]
pub struct BIAS_BUF_MONITOR_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_BUF_MONITOR_W<'a> {
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
#[doc = "Field `PD_CUR_DEEP_SLP` reader - xpd cur when rtc in sleep_state"]
pub struct PD_CUR_DEEP_SLP_R(crate::FieldReader<bool>);
impl PD_CUR_DEEP_SLP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PD_CUR_DEEP_SLP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD_CUR_DEEP_SLP_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD_CUR_DEEP_SLP` writer - xpd cur when rtc in sleep_state"]
pub struct PD_CUR_DEEP_SLP_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_CUR_DEEP_SLP_W<'a> {
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
#[doc = "Field `PD_CUR_MONITOR` reader - xpd cur when rtc in monitor state"]
pub struct PD_CUR_MONITOR_R(crate::FieldReader<bool>);
impl PD_CUR_MONITOR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PD_CUR_MONITOR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD_CUR_MONITOR_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD_CUR_MONITOR` writer - xpd cur when rtc in monitor state"]
pub struct PD_CUR_MONITOR_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_CUR_MONITOR_W<'a> {
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
#[doc = "Field `BIAS_SLEEP_DEEP_SLP` reader - bias_sleep when rtc in sleep_state"]
pub struct BIAS_SLEEP_DEEP_SLP_R(crate::FieldReader<bool>);
impl BIAS_SLEEP_DEEP_SLP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BIAS_SLEEP_DEEP_SLP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BIAS_SLEEP_DEEP_SLP_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BIAS_SLEEP_DEEP_SLP` writer - bias_sleep when rtc in sleep_state"]
pub struct BIAS_SLEEP_DEEP_SLP_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_SLEEP_DEEP_SLP_W<'a> {
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
#[doc = "Field `BIAS_SLEEP_MONITOR` reader - bias_sleep when rtc in monitor state"]
pub struct BIAS_SLEEP_MONITOR_R(crate::FieldReader<bool>);
impl BIAS_SLEEP_MONITOR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BIAS_SLEEP_MONITOR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BIAS_SLEEP_MONITOR_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BIAS_SLEEP_MONITOR` writer - bias_sleep when rtc in monitor state"]
pub struct BIAS_SLEEP_MONITOR_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_SLEEP_MONITOR_W<'a> {
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
#[doc = "Field `DBG_ATTEN_DEEP_SLP` reader - DBG_ATTEN when rtc in sleep state"]
pub struct DBG_ATTEN_DEEP_SLP_R(crate::FieldReader<u8>);
impl DBG_ATTEN_DEEP_SLP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DBG_ATTEN_DEEP_SLP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_ATTEN_DEEP_SLP_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_ATTEN_DEEP_SLP` writer - DBG_ATTEN when rtc in sleep state"]
pub struct DBG_ATTEN_DEEP_SLP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_ATTEN_DEEP_SLP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 18)) | ((value as u32 & 0x0f) << 18);
        self.w
    }
}
#[doc = "Field `DBG_ATTEN_MONITOR` reader - DBG_ATTEN when rtc in monitor state"]
pub struct DBG_ATTEN_MONITOR_R(crate::FieldReader<u8>);
impl DBG_ATTEN_MONITOR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DBG_ATTEN_MONITOR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_ATTEN_MONITOR_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_ATTEN_MONITOR` writer - DBG_ATTEN when rtc in monitor state"]
pub struct DBG_ATTEN_MONITOR_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_ATTEN_MONITOR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 22)) | ((value as u32 & 0x0f) << 22);
        self.w
    }
}
#[doc = "Field `ENB_SCK_XTAL` reader - ENB_SCK_XTAL"]
pub struct ENB_SCK_XTAL_R(crate::FieldReader<bool>);
impl ENB_SCK_XTAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENB_SCK_XTAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENB_SCK_XTAL_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENB_SCK_XTAL` writer - ENB_SCK_XTAL"]
pub struct ENB_SCK_XTAL_W<'a> {
    w: &'a mut W,
}
impl<'a> ENB_SCK_XTAL_W<'a> {
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
#[doc = "Field `INC_HEARTBEAT_REFRESH` reader - INC_HEARTBEAT_REFRESH"]
pub struct INC_HEARTBEAT_REFRESH_R(crate::FieldReader<bool>);
impl INC_HEARTBEAT_REFRESH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INC_HEARTBEAT_REFRESH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INC_HEARTBEAT_REFRESH_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INC_HEARTBEAT_REFRESH` writer - INC_HEARTBEAT_REFRESH"]
pub struct INC_HEARTBEAT_REFRESH_W<'a> {
    w: &'a mut W,
}
impl<'a> INC_HEARTBEAT_REFRESH_W<'a> {
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
#[doc = "Field `DEC_HEARTBEAT_PERIOD` reader - DEC_HEARTBEAT_PERIOD"]
pub struct DEC_HEARTBEAT_PERIOD_R(crate::FieldReader<bool>);
impl DEC_HEARTBEAT_PERIOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DEC_HEARTBEAT_PERIOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEC_HEARTBEAT_PERIOD_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEC_HEARTBEAT_PERIOD` writer - DEC_HEARTBEAT_PERIOD"]
pub struct DEC_HEARTBEAT_PERIOD_W<'a> {
    w: &'a mut W,
}
impl<'a> DEC_HEARTBEAT_PERIOD_W<'a> {
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
#[doc = "Field `INC_HEARTBEAT_PERIOD` reader - INC_HEARTBEAT_PERIOD"]
pub struct INC_HEARTBEAT_PERIOD_R(crate::FieldReader<bool>);
impl INC_HEARTBEAT_PERIOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INC_HEARTBEAT_PERIOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INC_HEARTBEAT_PERIOD_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INC_HEARTBEAT_PERIOD` writer - INC_HEARTBEAT_PERIOD"]
pub struct INC_HEARTBEAT_PERIOD_W<'a> {
    w: &'a mut W,
}
impl<'a> INC_HEARTBEAT_PERIOD_W<'a> {
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
#[doc = "Field `DEC_HEARTBEAT_WIDTH` reader - DEC_HEARTBEAT_WIDTH"]
pub struct DEC_HEARTBEAT_WIDTH_R(crate::FieldReader<bool>);
impl DEC_HEARTBEAT_WIDTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DEC_HEARTBEAT_WIDTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEC_HEARTBEAT_WIDTH_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEC_HEARTBEAT_WIDTH` writer - DEC_HEARTBEAT_WIDTH"]
pub struct DEC_HEARTBEAT_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> DEC_HEARTBEAT_WIDTH_W<'a> {
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
#[doc = "Field `RST_BIAS_I2C` reader - "]
pub struct RST_BIAS_I2C_R(crate::FieldReader<bool>);
impl RST_BIAS_I2C_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RST_BIAS_I2C_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RST_BIAS_I2C_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RST_BIAS_I2C` writer - "]
pub struct RST_BIAS_I2C_W<'a> {
    w: &'a mut W,
}
impl<'a> RST_BIAS_I2C_W<'a> {
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
    #[doc = "Bit 10 - open bias buf when system in active"]
    #[inline(always)]
    pub fn bias_buf_idle(&self) -> BIAS_BUF_IDLE_R {
        BIAS_BUF_IDLE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - open bias buf when rtc in wakeup"]
    #[inline(always)]
    pub fn bias_buf_wake(&self) -> BIAS_BUF_WAKE_R {
        BIAS_BUF_WAKE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - open bias buf when rtc in deep sleep"]
    #[inline(always)]
    pub fn bias_buf_deep_slp(&self) -> BIAS_BUF_DEEP_SLP_R {
        BIAS_BUF_DEEP_SLP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - open bias buf when rtc in monitor state"]
    #[inline(always)]
    pub fn bias_buf_monitor(&self) -> BIAS_BUF_MONITOR_R {
        BIAS_BUF_MONITOR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - xpd cur when rtc in sleep_state"]
    #[inline(always)]
    pub fn pd_cur_deep_slp(&self) -> PD_CUR_DEEP_SLP_R {
        PD_CUR_DEEP_SLP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - xpd cur when rtc in monitor state"]
    #[inline(always)]
    pub fn pd_cur_monitor(&self) -> PD_CUR_MONITOR_R {
        PD_CUR_MONITOR_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - bias_sleep when rtc in sleep_state"]
    #[inline(always)]
    pub fn bias_sleep_deep_slp(&self) -> BIAS_SLEEP_DEEP_SLP_R {
        BIAS_SLEEP_DEEP_SLP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - bias_sleep when rtc in monitor state"]
    #[inline(always)]
    pub fn bias_sleep_monitor(&self) -> BIAS_SLEEP_MONITOR_R {
        BIAS_SLEEP_MONITOR_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:21 - DBG_ATTEN when rtc in sleep state"]
    #[inline(always)]
    pub fn dbg_atten_deep_slp(&self) -> DBG_ATTEN_DEEP_SLP_R {
        DBG_ATTEN_DEEP_SLP_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 22:25 - DBG_ATTEN when rtc in monitor state"]
    #[inline(always)]
    pub fn dbg_atten_monitor(&self) -> DBG_ATTEN_MONITOR_R {
        DBG_ATTEN_MONITOR_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bit 26 - ENB_SCK_XTAL"]
    #[inline(always)]
    pub fn enb_sck_xtal(&self) -> ENB_SCK_XTAL_R {
        ENB_SCK_XTAL_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - INC_HEARTBEAT_REFRESH"]
    #[inline(always)]
    pub fn inc_heartbeat_refresh(&self) -> INC_HEARTBEAT_REFRESH_R {
        INC_HEARTBEAT_REFRESH_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - DEC_HEARTBEAT_PERIOD"]
    #[inline(always)]
    pub fn dec_heartbeat_period(&self) -> DEC_HEARTBEAT_PERIOD_R {
        DEC_HEARTBEAT_PERIOD_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - INC_HEARTBEAT_PERIOD"]
    #[inline(always)]
    pub fn inc_heartbeat_period(&self) -> INC_HEARTBEAT_PERIOD_R {
        INC_HEARTBEAT_PERIOD_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - DEC_HEARTBEAT_WIDTH"]
    #[inline(always)]
    pub fn dec_heartbeat_width(&self) -> DEC_HEARTBEAT_WIDTH_R {
        DEC_HEARTBEAT_WIDTH_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rst_bias_i2c(&self) -> RST_BIAS_I2C_R {
        RST_BIAS_I2C_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - open bias buf when system in active"]
    #[inline(always)]
    pub fn bias_buf_idle(&mut self) -> BIAS_BUF_IDLE_W {
        BIAS_BUF_IDLE_W { w: self }
    }
    #[doc = "Bit 11 - open bias buf when rtc in wakeup"]
    #[inline(always)]
    pub fn bias_buf_wake(&mut self) -> BIAS_BUF_WAKE_W {
        BIAS_BUF_WAKE_W { w: self }
    }
    #[doc = "Bit 12 - open bias buf when rtc in deep sleep"]
    #[inline(always)]
    pub fn bias_buf_deep_slp(&mut self) -> BIAS_BUF_DEEP_SLP_W {
        BIAS_BUF_DEEP_SLP_W { w: self }
    }
    #[doc = "Bit 13 - open bias buf when rtc in monitor state"]
    #[inline(always)]
    pub fn bias_buf_monitor(&mut self) -> BIAS_BUF_MONITOR_W {
        BIAS_BUF_MONITOR_W { w: self }
    }
    #[doc = "Bit 14 - xpd cur when rtc in sleep_state"]
    #[inline(always)]
    pub fn pd_cur_deep_slp(&mut self) -> PD_CUR_DEEP_SLP_W {
        PD_CUR_DEEP_SLP_W { w: self }
    }
    #[doc = "Bit 15 - xpd cur when rtc in monitor state"]
    #[inline(always)]
    pub fn pd_cur_monitor(&mut self) -> PD_CUR_MONITOR_W {
        PD_CUR_MONITOR_W { w: self }
    }
    #[doc = "Bit 16 - bias_sleep when rtc in sleep_state"]
    #[inline(always)]
    pub fn bias_sleep_deep_slp(&mut self) -> BIAS_SLEEP_DEEP_SLP_W {
        BIAS_SLEEP_DEEP_SLP_W { w: self }
    }
    #[doc = "Bit 17 - bias_sleep when rtc in monitor state"]
    #[inline(always)]
    pub fn bias_sleep_monitor(&mut self) -> BIAS_SLEEP_MONITOR_W {
        BIAS_SLEEP_MONITOR_W { w: self }
    }
    #[doc = "Bits 18:21 - DBG_ATTEN when rtc in sleep state"]
    #[inline(always)]
    pub fn dbg_atten_deep_slp(&mut self) -> DBG_ATTEN_DEEP_SLP_W {
        DBG_ATTEN_DEEP_SLP_W { w: self }
    }
    #[doc = "Bits 22:25 - DBG_ATTEN when rtc in monitor state"]
    #[inline(always)]
    pub fn dbg_atten_monitor(&mut self) -> DBG_ATTEN_MONITOR_W {
        DBG_ATTEN_MONITOR_W { w: self }
    }
    #[doc = "Bit 26 - ENB_SCK_XTAL"]
    #[inline(always)]
    pub fn enb_sck_xtal(&mut self) -> ENB_SCK_XTAL_W {
        ENB_SCK_XTAL_W { w: self }
    }
    #[doc = "Bit 27 - INC_HEARTBEAT_REFRESH"]
    #[inline(always)]
    pub fn inc_heartbeat_refresh(&mut self) -> INC_HEARTBEAT_REFRESH_W {
        INC_HEARTBEAT_REFRESH_W { w: self }
    }
    #[doc = "Bit 28 - DEC_HEARTBEAT_PERIOD"]
    #[inline(always)]
    pub fn dec_heartbeat_period(&mut self) -> DEC_HEARTBEAT_PERIOD_W {
        DEC_HEARTBEAT_PERIOD_W { w: self }
    }
    #[doc = "Bit 29 - INC_HEARTBEAT_PERIOD"]
    #[inline(always)]
    pub fn inc_heartbeat_period(&mut self) -> INC_HEARTBEAT_PERIOD_W {
        INC_HEARTBEAT_PERIOD_W { w: self }
    }
    #[doc = "Bit 30 - DEC_HEARTBEAT_WIDTH"]
    #[inline(always)]
    pub fn dec_heartbeat_width(&mut self) -> DEC_HEARTBEAT_WIDTH_W {
        DEC_HEARTBEAT_WIDTH_W { w: self }
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rst_bias_i2c(&mut self) -> RST_BIAS_I2C_W {
        RST_BIAS_I2C_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure power register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bias_conf](index.html) module"]
pub struct BIAS_CONF_SPEC;
impl crate::RegisterSpec for BIAS_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bias_conf::R](R) reader structure"]
impl crate::Readable for BIAS_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bias_conf::W](W) writer structure"]
impl crate::Writable for BIAS_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BIAS_CONF to value 0x0001_0800"]
impl crate::Resettable for BIAS_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_0800
    }
}
