#[doc = "Register `INT_ENA_RTC` reader"]
pub struct R(crate::R<INT_ENA_RTC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ENA_RTC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_ENA_RTC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_ENA_RTC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_ENA_RTC` writer"]
pub struct W(crate::W<INT_ENA_RTC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_ENA_RTC_SPEC>;
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
impl From<crate::W<INT_ENA_RTC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_ENA_RTC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLP_WAKEUP_INT_ENA` reader - Enables interruption when the chip wakes up from sleep."]
pub struct SLP_WAKEUP_INT_ENA_R(crate::FieldReader<bool>);
impl SLP_WAKEUP_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLP_WAKEUP_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLP_WAKEUP_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLP_WAKEUP_INT_ENA` writer - Enables interruption when the chip wakes up from sleep."]
pub struct SLP_WAKEUP_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SLP_WAKEUP_INT_ENA_W<'a> {
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
#[doc = "Field `SLP_REJECT_INT_ENA` reader - Enables interruption when the chip rejects to go to sleep."]
pub struct SLP_REJECT_INT_ENA_R(crate::FieldReader<bool>);
impl SLP_REJECT_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLP_REJECT_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLP_REJECT_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLP_REJECT_INT_ENA` writer - Enables interruption when the chip rejects to go to sleep."]
pub struct SLP_REJECT_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SLP_REJECT_INT_ENA_W<'a> {
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
#[doc = "Field `SDIO_IDLE_INT_ENA` reader - Enables interruption when the SDIO idles."]
pub struct SDIO_IDLE_INT_ENA_R(crate::FieldReader<bool>);
impl SDIO_IDLE_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDIO_IDLE_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIO_IDLE_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIO_IDLE_INT_ENA` writer - Enables interruption when the SDIO idles."]
pub struct SDIO_IDLE_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_IDLE_INT_ENA_W<'a> {
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
#[doc = "Field `WDT_INT_ENA` reader - Enables the RTC watchdog interrupt."]
pub struct WDT_INT_ENA_R(crate::FieldReader<bool>);
impl WDT_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WDT_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDT_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDT_INT_ENA` writer - Enables the RTC watchdog interrupt."]
pub struct WDT_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_INT_ENA_W<'a> {
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
#[doc = "Field `TOUCH_SCAN_DONE_INT_ENA` reader - Enables interruption upon the completion of a touch scanning."]
pub struct TOUCH_SCAN_DONE_INT_ENA_R(crate::FieldReader<bool>);
impl TOUCH_SCAN_DONE_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOUCH_SCAN_DONE_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_SCAN_DONE_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_SCAN_DONE_INT_ENA` writer - Enables interruption upon the completion of a touch scanning."]
pub struct TOUCH_SCAN_DONE_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_SCAN_DONE_INT_ENA_W<'a> {
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
#[doc = "Field `ULP_CP_INT_ENA` reader - Enables the ULP co-processor interrupt."]
pub struct ULP_CP_INT_ENA_R(crate::FieldReader<bool>);
impl ULP_CP_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ULP_CP_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ULP_CP_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ULP_CP_INT_ENA` writer - Enables the ULP co-processor interrupt."]
pub struct ULP_CP_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> ULP_CP_INT_ENA_W<'a> {
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
#[doc = "Field `TOUCH_DONE_INT_ENA` reader - Enables interruption upon the completion of a single touch."]
pub struct TOUCH_DONE_INT_ENA_R(crate::FieldReader<bool>);
impl TOUCH_DONE_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOUCH_DONE_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_DONE_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_DONE_INT_ENA` writer - Enables interruption upon the completion of a single touch."]
pub struct TOUCH_DONE_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_DONE_INT_ENA_W<'a> {
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
#[doc = "Field `TOUCH_ACTIVE_INT_ENA` reader - Enables interruption when a touch is detected."]
pub struct TOUCH_ACTIVE_INT_ENA_R(crate::FieldReader<bool>);
impl TOUCH_ACTIVE_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOUCH_ACTIVE_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_ACTIVE_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_ACTIVE_INT_ENA` writer - Enables interruption when a touch is detected."]
pub struct TOUCH_ACTIVE_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_ACTIVE_INT_ENA_W<'a> {
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
#[doc = "Field `TOUCH_INACTIVE_INT_ENA` reader - Enables interruption when a touch is released."]
pub struct TOUCH_INACTIVE_INT_ENA_R(crate::FieldReader<bool>);
impl TOUCH_INACTIVE_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOUCH_INACTIVE_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_INACTIVE_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_INACTIVE_INT_ENA` writer - Enables interruption when a touch is released."]
pub struct TOUCH_INACTIVE_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_INACTIVE_INT_ENA_W<'a> {
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
#[doc = "Field `BROWN_OUT_INT_ENA` reader - Enables the brown out interrupt."]
pub struct BROWN_OUT_INT_ENA_R(crate::FieldReader<bool>);
impl BROWN_OUT_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BROWN_OUT_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BROWN_OUT_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BROWN_OUT_INT_ENA` writer - Enables the brown out interrupt."]
pub struct BROWN_OUT_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> BROWN_OUT_INT_ENA_W<'a> {
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
#[doc = "Field `MAIN_TIMER_INT_ENA` reader - Enables the RTC main timer interrupt."]
pub struct MAIN_TIMER_INT_ENA_R(crate::FieldReader<bool>);
impl MAIN_TIMER_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MAIN_TIMER_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAIN_TIMER_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAIN_TIMER_INT_ENA` writer - Enables the RTC main timer interrupt."]
pub struct MAIN_TIMER_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> MAIN_TIMER_INT_ENA_W<'a> {
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
#[doc = "Field `SARADC1_INT_ENA` reader - Enables the SAR ADC 1 interrupt."]
pub struct SARADC1_INT_ENA_R(crate::FieldReader<bool>);
impl SARADC1_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SARADC1_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SARADC1_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SARADC1_INT_ENA` writer - Enables the SAR ADC 1 interrupt."]
pub struct SARADC1_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SARADC1_INT_ENA_W<'a> {
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
#[doc = "Field `TSENS_INT_ENA` reader - Enables the touch sensor interrupt."]
pub struct TSENS_INT_ENA_R(crate::FieldReader<bool>);
impl TSENS_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TSENS_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSENS_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSENS_INT_ENA` writer - Enables the touch sensor interrupt."]
pub struct TSENS_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TSENS_INT_ENA_W<'a> {
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
#[doc = "Field `COCPU_INT_ENA` reader - Enables the ULP-RISCV interrupt."]
pub struct COCPU_INT_ENA_R(crate::FieldReader<bool>);
impl COCPU_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COCPU_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COCPU_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COCPU_INT_ENA` writer - Enables the ULP-RISCV interrupt."]
pub struct COCPU_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> COCPU_INT_ENA_W<'a> {
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
#[doc = "Field `SARADC2_INT_ENA` reader - Enables the SAR ADC 2 interrupt."]
pub struct SARADC2_INT_ENA_R(crate::FieldReader<bool>);
impl SARADC2_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SARADC2_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SARADC2_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SARADC2_INT_ENA` writer - Enables the SAR ADC 2 interrupt."]
pub struct SARADC2_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SARADC2_INT_ENA_W<'a> {
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
#[doc = "Field `SWD_INT_ENA` reader - Enables the super watchdog interrupt."]
pub struct SWD_INT_ENA_R(crate::FieldReader<bool>);
impl SWD_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWD_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWD_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWD_INT_ENA` writer - Enables the super watchdog interrupt."]
pub struct SWD_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SWD_INT_ENA_W<'a> {
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
#[doc = "Field `XTAL32K_DEAD_INT_ENA` reader - Enables interruption when the 32 kHz crystal is dead."]
pub struct XTAL32K_DEAD_INT_ENA_R(crate::FieldReader<bool>);
impl XTAL32K_DEAD_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XTAL32K_DEAD_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTAL32K_DEAD_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTAL32K_DEAD_INT_ENA` writer - Enables interruption when the 32 kHz crystal is dead."]
pub struct XTAL32K_DEAD_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL32K_DEAD_INT_ENA_W<'a> {
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
#[doc = "Field `COCPU_TRAP_INT_ENA` reader - Enables interruption when the ULP-RISCV is trapped."]
pub struct COCPU_TRAP_INT_ENA_R(crate::FieldReader<bool>);
impl COCPU_TRAP_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COCPU_TRAP_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COCPU_TRAP_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COCPU_TRAP_INT_ENA` writer - Enables interruption when the ULP-RISCV is trapped."]
pub struct COCPU_TRAP_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> COCPU_TRAP_INT_ENA_W<'a> {
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
#[doc = "Field `TOUCH_TIMEOUT_INT_ENA` reader - Enables interruption when touch sensor times out."]
pub struct TOUCH_TIMEOUT_INT_ENA_R(crate::FieldReader<bool>);
impl TOUCH_TIMEOUT_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOUCH_TIMEOUT_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_TIMEOUT_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_TIMEOUT_INT_ENA` writer - Enables interruption when touch sensor times out."]
pub struct TOUCH_TIMEOUT_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_TIMEOUT_INT_ENA_W<'a> {
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
#[doc = "Field `GLITCH_DET_INT_ENA` reader - Enables interruption when a glitch is detected."]
pub struct GLITCH_DET_INT_ENA_R(crate::FieldReader<bool>);
impl GLITCH_DET_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GLITCH_DET_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GLITCH_DET_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GLITCH_DET_INT_ENA` writer - Enables interruption when a glitch is detected."]
pub struct GLITCH_DET_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> GLITCH_DET_INT_ENA_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Enables interruption when the chip wakes up from sleep."]
    #[inline(always)]
    pub fn slp_wakeup_int_ena(&self) -> SLP_WAKEUP_INT_ENA_R {
        SLP_WAKEUP_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enables interruption when the chip rejects to go to sleep."]
    #[inline(always)]
    pub fn slp_reject_int_ena(&self) -> SLP_REJECT_INT_ENA_R {
        SLP_REJECT_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enables interruption when the SDIO idles."]
    #[inline(always)]
    pub fn sdio_idle_int_ena(&self) -> SDIO_IDLE_INT_ENA_R {
        SDIO_IDLE_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enables the RTC watchdog interrupt."]
    #[inline(always)]
    pub fn wdt_int_ena(&self) -> WDT_INT_ENA_R {
        WDT_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enables interruption upon the completion of a touch scanning."]
    #[inline(always)]
    pub fn touch_scan_done_int_ena(&self) -> TOUCH_SCAN_DONE_INT_ENA_R {
        TOUCH_SCAN_DONE_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enables the ULP co-processor interrupt."]
    #[inline(always)]
    pub fn ulp_cp_int_ena(&self) -> ULP_CP_INT_ENA_R {
        ULP_CP_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enables interruption upon the completion of a single touch."]
    #[inline(always)]
    pub fn touch_done_int_ena(&self) -> TOUCH_DONE_INT_ENA_R {
        TOUCH_DONE_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enables interruption when a touch is detected."]
    #[inline(always)]
    pub fn touch_active_int_ena(&self) -> TOUCH_ACTIVE_INT_ENA_R {
        TOUCH_ACTIVE_INT_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enables interruption when a touch is released."]
    #[inline(always)]
    pub fn touch_inactive_int_ena(&self) -> TOUCH_INACTIVE_INT_ENA_R {
        TOUCH_INACTIVE_INT_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enables the brown out interrupt."]
    #[inline(always)]
    pub fn brown_out_int_ena(&self) -> BROWN_OUT_INT_ENA_R {
        BROWN_OUT_INT_ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enables the RTC main timer interrupt."]
    #[inline(always)]
    pub fn main_timer_int_ena(&self) -> MAIN_TIMER_INT_ENA_R {
        MAIN_TIMER_INT_ENA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enables the SAR ADC 1 interrupt."]
    #[inline(always)]
    pub fn saradc1_int_ena(&self) -> SARADC1_INT_ENA_R {
        SARADC1_INT_ENA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enables the touch sensor interrupt."]
    #[inline(always)]
    pub fn tsens_int_ena(&self) -> TSENS_INT_ENA_R {
        TSENS_INT_ENA_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enables the ULP-RISCV interrupt."]
    #[inline(always)]
    pub fn cocpu_int_ena(&self) -> COCPU_INT_ENA_R {
        COCPU_INT_ENA_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enables the SAR ADC 2 interrupt."]
    #[inline(always)]
    pub fn saradc2_int_ena(&self) -> SARADC2_INT_ENA_R {
        SARADC2_INT_ENA_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enables the super watchdog interrupt."]
    #[inline(always)]
    pub fn swd_int_ena(&self) -> SWD_INT_ENA_R {
        SWD_INT_ENA_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enables interruption when the 32 kHz crystal is dead."]
    #[inline(always)]
    pub fn xtal32k_dead_int_ena(&self) -> XTAL32K_DEAD_INT_ENA_R {
        XTAL32K_DEAD_INT_ENA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enables interruption when the ULP-RISCV is trapped."]
    #[inline(always)]
    pub fn cocpu_trap_int_ena(&self) -> COCPU_TRAP_INT_ENA_R {
        COCPU_TRAP_INT_ENA_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enables interruption when touch sensor times out."]
    #[inline(always)]
    pub fn touch_timeout_int_ena(&self) -> TOUCH_TIMEOUT_INT_ENA_R {
        TOUCH_TIMEOUT_INT_ENA_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enables interruption when a glitch is detected."]
    #[inline(always)]
    pub fn glitch_det_int_ena(&self) -> GLITCH_DET_INT_ENA_R {
        GLITCH_DET_INT_ENA_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables interruption when the chip wakes up from sleep."]
    #[inline(always)]
    pub fn slp_wakeup_int_ena(&mut self) -> SLP_WAKEUP_INT_ENA_W {
        SLP_WAKEUP_INT_ENA_W { w: self }
    }
    #[doc = "Bit 1 - Enables interruption when the chip rejects to go to sleep."]
    #[inline(always)]
    pub fn slp_reject_int_ena(&mut self) -> SLP_REJECT_INT_ENA_W {
        SLP_REJECT_INT_ENA_W { w: self }
    }
    #[doc = "Bit 2 - Enables interruption when the SDIO idles."]
    #[inline(always)]
    pub fn sdio_idle_int_ena(&mut self) -> SDIO_IDLE_INT_ENA_W {
        SDIO_IDLE_INT_ENA_W { w: self }
    }
    #[doc = "Bit 3 - Enables the RTC watchdog interrupt."]
    #[inline(always)]
    pub fn wdt_int_ena(&mut self) -> WDT_INT_ENA_W {
        WDT_INT_ENA_W { w: self }
    }
    #[doc = "Bit 4 - Enables interruption upon the completion of a touch scanning."]
    #[inline(always)]
    pub fn touch_scan_done_int_ena(&mut self) -> TOUCH_SCAN_DONE_INT_ENA_W {
        TOUCH_SCAN_DONE_INT_ENA_W { w: self }
    }
    #[doc = "Bit 5 - Enables the ULP co-processor interrupt."]
    #[inline(always)]
    pub fn ulp_cp_int_ena(&mut self) -> ULP_CP_INT_ENA_W {
        ULP_CP_INT_ENA_W { w: self }
    }
    #[doc = "Bit 6 - Enables interruption upon the completion of a single touch."]
    #[inline(always)]
    pub fn touch_done_int_ena(&mut self) -> TOUCH_DONE_INT_ENA_W {
        TOUCH_DONE_INT_ENA_W { w: self }
    }
    #[doc = "Bit 7 - Enables interruption when a touch is detected."]
    #[inline(always)]
    pub fn touch_active_int_ena(&mut self) -> TOUCH_ACTIVE_INT_ENA_W {
        TOUCH_ACTIVE_INT_ENA_W { w: self }
    }
    #[doc = "Bit 8 - Enables interruption when a touch is released."]
    #[inline(always)]
    pub fn touch_inactive_int_ena(&mut self) -> TOUCH_INACTIVE_INT_ENA_W {
        TOUCH_INACTIVE_INT_ENA_W { w: self }
    }
    #[doc = "Bit 9 - Enables the brown out interrupt."]
    #[inline(always)]
    pub fn brown_out_int_ena(&mut self) -> BROWN_OUT_INT_ENA_W {
        BROWN_OUT_INT_ENA_W { w: self }
    }
    #[doc = "Bit 10 - Enables the RTC main timer interrupt."]
    #[inline(always)]
    pub fn main_timer_int_ena(&mut self) -> MAIN_TIMER_INT_ENA_W {
        MAIN_TIMER_INT_ENA_W { w: self }
    }
    #[doc = "Bit 11 - Enables the SAR ADC 1 interrupt."]
    #[inline(always)]
    pub fn saradc1_int_ena(&mut self) -> SARADC1_INT_ENA_W {
        SARADC1_INT_ENA_W { w: self }
    }
    #[doc = "Bit 12 - Enables the touch sensor interrupt."]
    #[inline(always)]
    pub fn tsens_int_ena(&mut self) -> TSENS_INT_ENA_W {
        TSENS_INT_ENA_W { w: self }
    }
    #[doc = "Bit 13 - Enables the ULP-RISCV interrupt."]
    #[inline(always)]
    pub fn cocpu_int_ena(&mut self) -> COCPU_INT_ENA_W {
        COCPU_INT_ENA_W { w: self }
    }
    #[doc = "Bit 14 - Enables the SAR ADC 2 interrupt."]
    #[inline(always)]
    pub fn saradc2_int_ena(&mut self) -> SARADC2_INT_ENA_W {
        SARADC2_INT_ENA_W { w: self }
    }
    #[doc = "Bit 15 - Enables the super watchdog interrupt."]
    #[inline(always)]
    pub fn swd_int_ena(&mut self) -> SWD_INT_ENA_W {
        SWD_INT_ENA_W { w: self }
    }
    #[doc = "Bit 16 - Enables interruption when the 32 kHz crystal is dead."]
    #[inline(always)]
    pub fn xtal32k_dead_int_ena(&mut self) -> XTAL32K_DEAD_INT_ENA_W {
        XTAL32K_DEAD_INT_ENA_W { w: self }
    }
    #[doc = "Bit 17 - Enables interruption when the ULP-RISCV is trapped."]
    #[inline(always)]
    pub fn cocpu_trap_int_ena(&mut self) -> COCPU_TRAP_INT_ENA_W {
        COCPU_TRAP_INT_ENA_W { w: self }
    }
    #[doc = "Bit 18 - Enables interruption when touch sensor times out."]
    #[inline(always)]
    pub fn touch_timeout_int_ena(&mut self) -> TOUCH_TIMEOUT_INT_ENA_W {
        TOUCH_TIMEOUT_INT_ENA_W { w: self }
    }
    #[doc = "Bit 19 - Enables interruption when a glitch is detected."]
    #[inline(always)]
    pub fn glitch_det_int_ena(&mut self) -> GLITCH_DET_INT_ENA_W {
        GLITCH_DET_INT_ENA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC interrupt enabling register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_ena_rtc](index.html) module"]
pub struct INT_ENA_RTC_SPEC;
impl crate::RegisterSpec for INT_ENA_RTC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_ena_rtc::R](R) reader structure"]
impl crate::Readable for INT_ENA_RTC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_ena_rtc::W](W) writer structure"]
impl crate::Writable for INT_ENA_RTC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_ENA_RTC to value 0"]
impl crate::Resettable for INT_ENA_RTC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
