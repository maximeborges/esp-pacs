#[doc = "Register `INT_RAW_RTC` reader"]
pub struct R(crate::R<INT_RAW_RTC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_RAW_RTC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_RAW_RTC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_RAW_RTC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SLP_WAKEUP_INT_RAW` reader - Stores the raw interrupt triggered when the chip wakes up from sleep."]
pub struct SLP_WAKEUP_INT_RAW_R(crate::FieldReader<bool, bool>);
impl SLP_WAKEUP_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLP_WAKEUP_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLP_WAKEUP_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLP_REJECT_INT_RAW` reader - Stores the raw interrupt triggered when the chip rejects to go to sleep."]
pub struct SLP_REJECT_INT_RAW_R(crate::FieldReader<bool, bool>);
impl SLP_REJECT_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLP_REJECT_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLP_REJECT_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIO_IDLE_INT_RAW` reader - Stores the raw interrupt triggered when the SDIO idles."]
pub struct SDIO_IDLE_INT_RAW_R(crate::FieldReader<bool, bool>);
impl SDIO_IDLE_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDIO_IDLE_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIO_IDLE_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDT_INT_RAW` reader - Stores the raw RTC watchdog interrupt."]
pub struct WDT_INT_RAW_R(crate::FieldReader<bool, bool>);
impl WDT_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WDT_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDT_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_SCAN_DONE_INT_RAW` reader - Stores the raw interrupt triggered upon the completion of a touch scanning."]
pub struct TOUCH_SCAN_DONE_INT_RAW_R(crate::FieldReader<bool, bool>);
impl TOUCH_SCAN_DONE_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOUCH_SCAN_DONE_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_SCAN_DONE_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ULP_CP_INT_RAW` reader - Stores the raw ULP co-processor interrupt."]
pub struct ULP_CP_INT_RAW_R(crate::FieldReader<bool, bool>);
impl ULP_CP_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ULP_CP_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ULP_CP_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_DONE_INT_RAW` reader - Stores the raw interrupt triggered upon the completion of a single touch."]
pub struct TOUCH_DONE_INT_RAW_R(crate::FieldReader<bool, bool>);
impl TOUCH_DONE_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOUCH_DONE_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_DONE_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_ACTIVE_INT_RAW` reader - Stores the raw interrupt triggered when a touch is detected."]
pub struct TOUCH_ACTIVE_INT_RAW_R(crate::FieldReader<bool, bool>);
impl TOUCH_ACTIVE_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOUCH_ACTIVE_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_ACTIVE_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_INACTIVE_INT_RAW` reader - Stores the raw interrupt triggered when a touch is released."]
pub struct TOUCH_INACTIVE_INT_RAW_R(crate::FieldReader<bool, bool>);
impl TOUCH_INACTIVE_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOUCH_INACTIVE_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_INACTIVE_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BROWN_OUT_INT_RAW` reader - Stores the raw brown out interrupt."]
pub struct BROWN_OUT_INT_RAW_R(crate::FieldReader<bool, bool>);
impl BROWN_OUT_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BROWN_OUT_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BROWN_OUT_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAIN_TIMER_INT_RAW` reader - Stores the raw RTC main timer interrupt."]
pub struct MAIN_TIMER_INT_RAW_R(crate::FieldReader<bool, bool>);
impl MAIN_TIMER_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MAIN_TIMER_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAIN_TIMER_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SARADC1_INT_RAW` reader - Stores the raw SAR ADC 1 interrupt."]
pub struct SARADC1_INT_RAW_R(crate::FieldReader<bool, bool>);
impl SARADC1_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SARADC1_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SARADC1_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSENS_INT_RAW` reader - Stores the raw temperature sensor interrupt."]
pub struct TSENS_INT_RAW_R(crate::FieldReader<bool, bool>);
impl TSENS_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TSENS_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSENS_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COCPU_INT_RAW` reader - Stores the raw ULP-RISCV interrupt."]
pub struct COCPU_INT_RAW_R(crate::FieldReader<bool, bool>);
impl COCPU_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COCPU_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COCPU_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SARADC2_INT_RAW` reader - Stores the raw SAR ADC 2 interrupt."]
pub struct SARADC2_INT_RAW_R(crate::FieldReader<bool, bool>);
impl SARADC2_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SARADC2_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SARADC2_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWD_INT_RAW` reader - Stores the raw super watchdog interrupt."]
pub struct SWD_INT_RAW_R(crate::FieldReader<bool, bool>);
impl SWD_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWD_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWD_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTAL32K_DEAD_INT_RAW` reader - Stores the raw interrupt triggered when the 32 kHz crystal is dead."]
pub struct XTAL32K_DEAD_INT_RAW_R(crate::FieldReader<bool, bool>);
impl XTAL32K_DEAD_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XTAL32K_DEAD_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTAL32K_DEAD_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COCPU_TRAP_INT_RAW` reader - Stores the raw interrupt triggered when the ULP-RISCV is trapped."]
pub struct COCPU_TRAP_INT_RAW_R(crate::FieldReader<bool, bool>);
impl COCPU_TRAP_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COCPU_TRAP_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COCPU_TRAP_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_TIMEOUT_INT_RAW` reader - Stores the raw interrupt triggered when touch sensor times out."]
pub struct TOUCH_TIMEOUT_INT_RAW_R(crate::FieldReader<bool, bool>);
impl TOUCH_TIMEOUT_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOUCH_TIMEOUT_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_TIMEOUT_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GLITCH_DET_INT_RAW` reader - Stores the raw interrupt triggered when a glitch is detected."]
pub struct GLITCH_DET_INT_RAW_R(crate::FieldReader<bool, bool>);
impl GLITCH_DET_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GLITCH_DET_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GLITCH_DET_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Stores the raw interrupt triggered when the chip wakes up from sleep."]
    #[inline(always)]
    pub fn slp_wakeup_int_raw(&self) -> SLP_WAKEUP_INT_RAW_R {
        SLP_WAKEUP_INT_RAW_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Stores the raw interrupt triggered when the chip rejects to go to sleep."]
    #[inline(always)]
    pub fn slp_reject_int_raw(&self) -> SLP_REJECT_INT_RAW_R {
        SLP_REJECT_INT_RAW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Stores the raw interrupt triggered when the SDIO idles."]
    #[inline(always)]
    pub fn sdio_idle_int_raw(&self) -> SDIO_IDLE_INT_RAW_R {
        SDIO_IDLE_INT_RAW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Stores the raw RTC watchdog interrupt."]
    #[inline(always)]
    pub fn wdt_int_raw(&self) -> WDT_INT_RAW_R {
        WDT_INT_RAW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Stores the raw interrupt triggered upon the completion of a touch scanning."]
    #[inline(always)]
    pub fn touch_scan_done_int_raw(&self) -> TOUCH_SCAN_DONE_INT_RAW_R {
        TOUCH_SCAN_DONE_INT_RAW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Stores the raw ULP co-processor interrupt."]
    #[inline(always)]
    pub fn ulp_cp_int_raw(&self) -> ULP_CP_INT_RAW_R {
        ULP_CP_INT_RAW_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Stores the raw interrupt triggered upon the completion of a single touch."]
    #[inline(always)]
    pub fn touch_done_int_raw(&self) -> TOUCH_DONE_INT_RAW_R {
        TOUCH_DONE_INT_RAW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Stores the raw interrupt triggered when a touch is detected."]
    #[inline(always)]
    pub fn touch_active_int_raw(&self) -> TOUCH_ACTIVE_INT_RAW_R {
        TOUCH_ACTIVE_INT_RAW_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Stores the raw interrupt triggered when a touch is released."]
    #[inline(always)]
    pub fn touch_inactive_int_raw(&self) -> TOUCH_INACTIVE_INT_RAW_R {
        TOUCH_INACTIVE_INT_RAW_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Stores the raw brown out interrupt."]
    #[inline(always)]
    pub fn brown_out_int_raw(&self) -> BROWN_OUT_INT_RAW_R {
        BROWN_OUT_INT_RAW_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Stores the raw RTC main timer interrupt."]
    #[inline(always)]
    pub fn main_timer_int_raw(&self) -> MAIN_TIMER_INT_RAW_R {
        MAIN_TIMER_INT_RAW_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Stores the raw SAR ADC 1 interrupt."]
    #[inline(always)]
    pub fn saradc1_int_raw(&self) -> SARADC1_INT_RAW_R {
        SARADC1_INT_RAW_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Stores the raw temperature sensor interrupt."]
    #[inline(always)]
    pub fn tsens_int_raw(&self) -> TSENS_INT_RAW_R {
        TSENS_INT_RAW_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Stores the raw ULP-RISCV interrupt."]
    #[inline(always)]
    pub fn cocpu_int_raw(&self) -> COCPU_INT_RAW_R {
        COCPU_INT_RAW_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Stores the raw SAR ADC 2 interrupt."]
    #[inline(always)]
    pub fn saradc2_int_raw(&self) -> SARADC2_INT_RAW_R {
        SARADC2_INT_RAW_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Stores the raw super watchdog interrupt."]
    #[inline(always)]
    pub fn swd_int_raw(&self) -> SWD_INT_RAW_R {
        SWD_INT_RAW_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Stores the raw interrupt triggered when the 32 kHz crystal is dead."]
    #[inline(always)]
    pub fn xtal32k_dead_int_raw(&self) -> XTAL32K_DEAD_INT_RAW_R {
        XTAL32K_DEAD_INT_RAW_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Stores the raw interrupt triggered when the ULP-RISCV is trapped."]
    #[inline(always)]
    pub fn cocpu_trap_int_raw(&self) -> COCPU_TRAP_INT_RAW_R {
        COCPU_TRAP_INT_RAW_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Stores the raw interrupt triggered when touch sensor times out."]
    #[inline(always)]
    pub fn touch_timeout_int_raw(&self) -> TOUCH_TIMEOUT_INT_RAW_R {
        TOUCH_TIMEOUT_INT_RAW_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Stores the raw interrupt triggered when a glitch is detected."]
    #[inline(always)]
    pub fn glitch_det_int_raw(&self) -> GLITCH_DET_INT_RAW_R {
        GLITCH_DET_INT_RAW_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
#[doc = "RTC interrupt raw register\n\nThis register you can [`read`]
(crate::generic::Reg::read). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_raw_rtc]
(index.html) module"]
pub struct INT_RAW_RTC_SPEC;
impl crate::RegisterSpec for INT_RAW_RTC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_raw_rtc::R]
(R) reader structure"]
impl crate::Readable for INT_RAW_RTC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INT_RAW_RTC to value 0"]
impl crate::Resettable for INT_RAW_RTC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
