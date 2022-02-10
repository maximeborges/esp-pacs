#[doc = "Register `INT_ST_RTC` reader"]
pub struct R(crate::R<INT_ST_RTC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ST_RTC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_ST_RTC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_ST_RTC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SLP_WAKEUP_INT_ST` reader - sleep wakeup interrupt state"]
pub struct SLP_WAKEUP_INT_ST_R(crate::FieldReader<bool, bool>);
impl SLP_WAKEUP_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLP_WAKEUP_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLP_WAKEUP_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLP_REJECT_INT_ST` reader - sleep reject interrupt state"]
pub struct SLP_REJECT_INT_ST_R(crate::FieldReader<bool, bool>);
impl SLP_REJECT_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLP_REJECT_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLP_REJECT_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIO_IDLE_INT_ST` reader - SDIO idle interrupt state"]
pub struct SDIO_IDLE_INT_ST_R(crate::FieldReader<bool, bool>);
impl SDIO_IDLE_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDIO_IDLE_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIO_IDLE_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_WDT_INT_ST` reader - RTC WDT interrupt state"]
pub struct RTC_WDT_INT_ST_R(crate::FieldReader<bool, bool>);
impl RTC_WDT_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_WDT_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_WDT_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_TOUCH_SCAN_DONE_INT_ST` reader - enable touch scan done interrupt raw"]
pub struct RTC_TOUCH_SCAN_DONE_INT_ST_R(crate::FieldReader<bool, bool>);
impl RTC_TOUCH_SCAN_DONE_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_TOUCH_SCAN_DONE_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_TOUCH_SCAN_DONE_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_ULP_CP_INT_ST` reader - ULP-coprocessor interrupt state"]
pub struct RTC_ULP_CP_INT_ST_R(crate::FieldReader<bool, bool>);
impl RTC_ULP_CP_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_ULP_CP_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_ULP_CP_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_TOUCH_DONE_INT_ST` reader - touch done interrupt state"]
pub struct RTC_TOUCH_DONE_INT_ST_R(crate::FieldReader<bool, bool>);
impl RTC_TOUCH_DONE_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_TOUCH_DONE_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_TOUCH_DONE_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_TOUCH_ACTIVE_INT_ST` reader - touch active interrupt state"]
pub struct RTC_TOUCH_ACTIVE_INT_ST_R(crate::FieldReader<bool, bool>);
impl RTC_TOUCH_ACTIVE_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_TOUCH_ACTIVE_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_TOUCH_ACTIVE_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_TOUCH_INACTIVE_INT_ST` reader - touch inactive interrupt state"]
pub struct RTC_TOUCH_INACTIVE_INT_ST_R(crate::FieldReader<bool, bool>);
impl RTC_TOUCH_INACTIVE_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_TOUCH_INACTIVE_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_TOUCH_INACTIVE_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_BROWN_OUT_INT_ST` reader - brown out interrupt state"]
pub struct RTC_BROWN_OUT_INT_ST_R(crate::FieldReader<bool, bool>);
impl RTC_BROWN_OUT_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_BROWN_OUT_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_BROWN_OUT_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_MAIN_TIMER_INT_ST` reader - RTC main timer interrupt state"]
pub struct RTC_MAIN_TIMER_INT_ST_R(crate::FieldReader<bool, bool>);
impl RTC_MAIN_TIMER_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_MAIN_TIMER_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_MAIN_TIMER_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_SARADC1_INT_ST` reader - saradc1 interrupt state"]
pub struct RTC_SARADC1_INT_ST_R(crate::FieldReader<bool, bool>);
impl RTC_SARADC1_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_SARADC1_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_SARADC1_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_TSENS_INT_ST` reader - tsens interrupt state"]
pub struct RTC_TSENS_INT_ST_R(crate::FieldReader<bool, bool>);
impl RTC_TSENS_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_TSENS_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_TSENS_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_COCPU_INT_ST` reader - riscV cocpu interrupt state"]
pub struct RTC_COCPU_INT_ST_R(crate::FieldReader<bool, bool>);
impl RTC_COCPU_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_COCPU_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_COCPU_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_SARADC2_INT_ST` reader - saradc2 interrupt state"]
pub struct RTC_SARADC2_INT_ST_R(crate::FieldReader<bool, bool>);
impl RTC_SARADC2_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_SARADC2_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_SARADC2_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_SWD_INT_ST` reader - super watch dog interrupt state"]
pub struct RTC_SWD_INT_ST_R(crate::FieldReader<bool, bool>);
impl RTC_SWD_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_SWD_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_SWD_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_XTAL32K_DEAD_INT_ST` reader - xtal32k dead detection interrupt state"]
pub struct RTC_XTAL32K_DEAD_INT_ST_R(crate::FieldReader<bool, bool>);
impl RTC_XTAL32K_DEAD_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_XTAL32K_DEAD_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_XTAL32K_DEAD_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_COCPU_TRAP_INT_ST` reader - cocpu trap interrupt state"]
pub struct RTC_COCPU_TRAP_INT_ST_R(crate::FieldReader<bool, bool>);
impl RTC_COCPU_TRAP_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_COCPU_TRAP_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_COCPU_TRAP_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_TOUCH_TIMEOUT_INT_ST` reader - Touch timeout interrupt state"]
pub struct RTC_TOUCH_TIMEOUT_INT_ST_R(crate::FieldReader<bool, bool>);
impl RTC_TOUCH_TIMEOUT_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_TOUCH_TIMEOUT_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_TOUCH_TIMEOUT_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_GLITCH_DET_INT_ST` reader - glitch_det_interrupt state"]
pub struct RTC_GLITCH_DET_INT_ST_R(crate::FieldReader<bool, bool>);
impl RTC_GLITCH_DET_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_GLITCH_DET_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_GLITCH_DET_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_TOUCH_APPROACH_LOOP_DONE_INT_ST` reader - touch approach mode loop interrupt state"]
pub struct RTC_TOUCH_APPROACH_LOOP_DONE_INT_ST_R(crate::FieldReader<bool, bool>);
impl RTC_TOUCH_APPROACH_LOOP_DONE_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_TOUCH_APPROACH_LOOP_DONE_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_TOUCH_APPROACH_LOOP_DONE_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - sleep wakeup interrupt state"]
    #[inline(always)]
    pub fn slp_wakeup_int_st(&self) -> SLP_WAKEUP_INT_ST_R {
        SLP_WAKEUP_INT_ST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - sleep reject interrupt state"]
    #[inline(always)]
    pub fn slp_reject_int_st(&self) -> SLP_REJECT_INT_ST_R {
        SLP_REJECT_INT_ST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SDIO idle interrupt state"]
    #[inline(always)]
    pub fn sdio_idle_int_st(&self) -> SDIO_IDLE_INT_ST_R {
        SDIO_IDLE_INT_ST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RTC WDT interrupt state"]
    #[inline(always)]
    pub fn rtc_wdt_int_st(&self) -> RTC_WDT_INT_ST_R {
        RTC_WDT_INT_ST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - enable touch scan done interrupt raw"]
    #[inline(always)]
    pub fn rtc_touch_scan_done_int_st(&self) -> RTC_TOUCH_SCAN_DONE_INT_ST_R {
        RTC_TOUCH_SCAN_DONE_INT_ST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ULP-coprocessor interrupt state"]
    #[inline(always)]
    pub fn rtc_ulp_cp_int_st(&self) -> RTC_ULP_CP_INT_ST_R {
        RTC_ULP_CP_INT_ST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - touch done interrupt state"]
    #[inline(always)]
    pub fn rtc_touch_done_int_st(&self) -> RTC_TOUCH_DONE_INT_ST_R {
        RTC_TOUCH_DONE_INT_ST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - touch active interrupt state"]
    #[inline(always)]
    pub fn rtc_touch_active_int_st(&self) -> RTC_TOUCH_ACTIVE_INT_ST_R {
        RTC_TOUCH_ACTIVE_INT_ST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - touch inactive interrupt state"]
    #[inline(always)]
    pub fn rtc_touch_inactive_int_st(&self) -> RTC_TOUCH_INACTIVE_INT_ST_R {
        RTC_TOUCH_INACTIVE_INT_ST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - brown out interrupt state"]
    #[inline(always)]
    pub fn rtc_brown_out_int_st(&self) -> RTC_BROWN_OUT_INT_ST_R {
        RTC_BROWN_OUT_INT_ST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - RTC main timer interrupt state"]
    #[inline(always)]
    pub fn rtc_main_timer_int_st(&self) -> RTC_MAIN_TIMER_INT_ST_R {
        RTC_MAIN_TIMER_INT_ST_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - saradc1 interrupt state"]
    #[inline(always)]
    pub fn rtc_saradc1_int_st(&self) -> RTC_SARADC1_INT_ST_R {
        RTC_SARADC1_INT_ST_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - tsens interrupt state"]
    #[inline(always)]
    pub fn rtc_tsens_int_st(&self) -> RTC_TSENS_INT_ST_R {
        RTC_TSENS_INT_ST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - riscV cocpu interrupt state"]
    #[inline(always)]
    pub fn rtc_cocpu_int_st(&self) -> RTC_COCPU_INT_ST_R {
        RTC_COCPU_INT_ST_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - saradc2 interrupt state"]
    #[inline(always)]
    pub fn rtc_saradc2_int_st(&self) -> RTC_SARADC2_INT_ST_R {
        RTC_SARADC2_INT_ST_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - super watch dog interrupt state"]
    #[inline(always)]
    pub fn rtc_swd_int_st(&self) -> RTC_SWD_INT_ST_R {
        RTC_SWD_INT_ST_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - xtal32k dead detection interrupt state"]
    #[inline(always)]
    pub fn rtc_xtal32k_dead_int_st(&self) -> RTC_XTAL32K_DEAD_INT_ST_R {
        RTC_XTAL32K_DEAD_INT_ST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - cocpu trap interrupt state"]
    #[inline(always)]
    pub fn rtc_cocpu_trap_int_st(&self) -> RTC_COCPU_TRAP_INT_ST_R {
        RTC_COCPU_TRAP_INT_ST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Touch timeout interrupt state"]
    #[inline(always)]
    pub fn rtc_touch_timeout_int_st(&self) -> RTC_TOUCH_TIMEOUT_INT_ST_R {
        RTC_TOUCH_TIMEOUT_INT_ST_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - glitch_det_interrupt state"]
    #[inline(always)]
    pub fn rtc_glitch_det_int_st(&self) -> RTC_GLITCH_DET_INT_ST_R {
        RTC_GLITCH_DET_INT_ST_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - touch approach mode loop interrupt state"]
    #[inline(always)]
    pub fn rtc_touch_approach_loop_done_int_st(&self) -> RTC_TOUCH_APPROACH_LOOP_DONE_INT_ST_R {
        RTC_TOUCH_APPROACH_LOOP_DONE_INT_ST_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
#[doc = "rtc interrupt register\n\nThis register you can [`read`]
(crate::generic::Reg::read). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_st_rtc]
(index.html) module"]
pub struct INT_ST_RTC_SPEC;
impl crate::RegisterSpec for INT_ST_RTC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_st_rtc::R]
(R) reader structure"]
impl crate::Readable for INT_ST_RTC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INT_ST_RTC to value 0"]
impl crate::Resettable for INT_ST_RTC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
