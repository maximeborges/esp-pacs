#[doc = "Register `INT_CLR_RTC` writer"]
pub struct W(crate::W<INT_CLR_RTC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_CLR_RTC_SPEC>;
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
impl From<crate::W<INT_CLR_RTC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_CLR_RTC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLP_WAKEUP_INT_CLR` writer - Clears the interrupt triggered when the chip wakes up from sleep."]
pub struct SLP_WAKEUP_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLP_WAKEUP_INT_CLR_W<'a> {
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
#[doc = "Field `SLP_REJECT_INT_CLR` writer - Clears the interrupt triggered when the chip rejects to go to sleep."]
pub struct SLP_REJECT_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLP_REJECT_INT_CLR_W<'a> {
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
#[doc = "Field `SDIO_IDLE_INT_CLR` writer - Clears the interrupt triggered when the SDIO idles."]
pub struct SDIO_IDLE_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_IDLE_INT_CLR_W<'a> {
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
#[doc = "Field `WDT_INT_CLR` writer - Enables the RTC watchdog interrupt."]
pub struct WDT_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_INT_CLR_W<'a> {
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
#[doc = "Field `TOUCH_SCAN_DONE_INT_CLR` writer - Clears the interrupt triggered upon the completion of a touch scanning."]
pub struct TOUCH_SCAN_DONE_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_SCAN_DONE_INT_CLR_W<'a> {
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
#[doc = "Field `ULP_CP_INT_CLR` writer - Enables the ULP co-processor interrupt."]
pub struct ULP_CP_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> ULP_CP_INT_CLR_W<'a> {
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
#[doc = "Field `TOUCH_DONE_INT_CLR` writer - Clears the interrupt triggered upon the completion of a single touch."]
pub struct TOUCH_DONE_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_DONE_INT_CLR_W<'a> {
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
#[doc = "Field `TOUCH_ACTIVE_INT_CLR` writer - Clears the interrupt triggered when a touch is detected."]
pub struct TOUCH_ACTIVE_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_ACTIVE_INT_CLR_W<'a> {
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
#[doc = "Field `TOUCH_INACTIVE_INT_CLR` writer - Clears the interrupt triggered when a touch is released."]
pub struct TOUCH_INACTIVE_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_INACTIVE_INT_CLR_W<'a> {
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
#[doc = "Field `BROWN_OUT_INT_CLR` writer - Clears the brown out interrupt."]
pub struct BROWN_OUT_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> BROWN_OUT_INT_CLR_W<'a> {
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
#[doc = "Field `MAIN_TIMER_INT_CLR` writer - Clears the RTC main timer interrupt."]
pub struct MAIN_TIMER_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> MAIN_TIMER_INT_CLR_W<'a> {
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
#[doc = "Field `SARADC1_INT_CLR` writer - Clears the SAR ADC 1 interrupt."]
pub struct SARADC1_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SARADC1_INT_CLR_W<'a> {
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
#[doc = "Field `TSENS_INT_CLR` writer - Clears the touch sensor interrupt."]
pub struct TSENS_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TSENS_INT_CLR_W<'a> {
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
#[doc = "Field `COCPU_INT_CLR` writer - Clears the ULP-RISCV interrupt."]
pub struct COCPU_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> COCPU_INT_CLR_W<'a> {
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
#[doc = "Field `SARADC2_INT_CLR` writer - Clears the SAR ADC 2 interrupt."]
pub struct SARADC2_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SARADC2_INT_CLR_W<'a> {
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
#[doc = "Field `SWD_INT_CLR` writer - Clears the super watchdog interrupt."]
pub struct SWD_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SWD_INT_CLR_W<'a> {
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
#[doc = "Field `XTAL32K_DEAD_INT_CLR` writer - Clears the interrupt triggered when the 32 kHz crystal is dead."]
pub struct XTAL32K_DEAD_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL32K_DEAD_INT_CLR_W<'a> {
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
#[doc = "Field `COCPU_TRAP_INT_CLR` writer - Clears the interrupt triggered when the ULP-RISCV is trapped."]
pub struct COCPU_TRAP_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> COCPU_TRAP_INT_CLR_W<'a> {
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
#[doc = "Field `TOUCH_TIMEOUT_INT_CLR` writer - Clears the interrupt triggered when touch sensor times out."]
pub struct TOUCH_TIMEOUT_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_TIMEOUT_INT_CLR_W<'a> {
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
#[doc = "Field `GLITCH_DET_INT_CLR` writer - Clears the interrupt triggered when a glitch is detected."]
pub struct GLITCH_DET_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> GLITCH_DET_INT_CLR_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Clears the interrupt triggered when the chip wakes up from sleep."]
    #[inline(always)]
    pub fn slp_wakeup_int_clr(&mut self) -> SLP_WAKEUP_INT_CLR_W {
        SLP_WAKEUP_INT_CLR_W { w: self }
    }
    #[doc = "Bit 1 - Clears the interrupt triggered when the chip rejects to go to sleep."]
    #[inline(always)]
    pub fn slp_reject_int_clr(&mut self) -> SLP_REJECT_INT_CLR_W {
        SLP_REJECT_INT_CLR_W { w: self }
    }
    #[doc = "Bit 2 - Clears the interrupt triggered when the SDIO idles."]
    #[inline(always)]
    pub fn sdio_idle_int_clr(&mut self) -> SDIO_IDLE_INT_CLR_W {
        SDIO_IDLE_INT_CLR_W { w: self }
    }
    #[doc = "Bit 3 - Enables the RTC watchdog interrupt."]
    #[inline(always)]
    pub fn wdt_int_clr(&mut self) -> WDT_INT_CLR_W {
        WDT_INT_CLR_W { w: self }
    }
    #[doc = "Bit 4 - Clears the interrupt triggered upon the completion of a touch scanning."]
    #[inline(always)]
    pub fn touch_scan_done_int_clr(&mut self) -> TOUCH_SCAN_DONE_INT_CLR_W {
        TOUCH_SCAN_DONE_INT_CLR_W { w: self }
    }
    #[doc = "Bit 5 - Enables the ULP co-processor interrupt."]
    #[inline(always)]
    pub fn ulp_cp_int_clr(&mut self) -> ULP_CP_INT_CLR_W {
        ULP_CP_INT_CLR_W { w: self }
    }
    #[doc = "Bit 6 - Clears the interrupt triggered upon the completion of a single touch."]
    #[inline(always)]
    pub fn touch_done_int_clr(&mut self) -> TOUCH_DONE_INT_CLR_W {
        TOUCH_DONE_INT_CLR_W { w: self }
    }
    #[doc = "Bit 7 - Clears the interrupt triggered when a touch is detected."]
    #[inline(always)]
    pub fn touch_active_int_clr(&mut self) -> TOUCH_ACTIVE_INT_CLR_W {
        TOUCH_ACTIVE_INT_CLR_W { w: self }
    }
    #[doc = "Bit 8 - Clears the interrupt triggered when a touch is released."]
    #[inline(always)]
    pub fn touch_inactive_int_clr(&mut self) -> TOUCH_INACTIVE_INT_CLR_W {
        TOUCH_INACTIVE_INT_CLR_W { w: self }
    }
    #[doc = "Bit 9 - Clears the brown out interrupt."]
    #[inline(always)]
    pub fn brown_out_int_clr(&mut self) -> BROWN_OUT_INT_CLR_W {
        BROWN_OUT_INT_CLR_W { w: self }
    }
    #[doc = "Bit 10 - Clears the RTC main timer interrupt."]
    #[inline(always)]
    pub fn main_timer_int_clr(&mut self) -> MAIN_TIMER_INT_CLR_W {
        MAIN_TIMER_INT_CLR_W { w: self }
    }
    #[doc = "Bit 11 - Clears the SAR ADC 1 interrupt."]
    #[inline(always)]
    pub fn saradc1_int_clr(&mut self) -> SARADC1_INT_CLR_W {
        SARADC1_INT_CLR_W { w: self }
    }
    #[doc = "Bit 12 - Clears the touch sensor interrupt."]
    #[inline(always)]
    pub fn tsens_int_clr(&mut self) -> TSENS_INT_CLR_W {
        TSENS_INT_CLR_W { w: self }
    }
    #[doc = "Bit 13 - Clears the ULP-RISCV interrupt."]
    #[inline(always)]
    pub fn cocpu_int_clr(&mut self) -> COCPU_INT_CLR_W {
        COCPU_INT_CLR_W { w: self }
    }
    #[doc = "Bit 14 - Clears the SAR ADC 2 interrupt."]
    #[inline(always)]
    pub fn saradc2_int_clr(&mut self) -> SARADC2_INT_CLR_W {
        SARADC2_INT_CLR_W { w: self }
    }
    #[doc = "Bit 15 - Clears the super watchdog interrupt."]
    #[inline(always)]
    pub fn swd_int_clr(&mut self) -> SWD_INT_CLR_W {
        SWD_INT_CLR_W { w: self }
    }
    #[doc = "Bit 16 - Clears the interrupt triggered when the 32 kHz crystal is dead."]
    #[inline(always)]
    pub fn xtal32k_dead_int_clr(&mut self) -> XTAL32K_DEAD_INT_CLR_W {
        XTAL32K_DEAD_INT_CLR_W { w: self }
    }
    #[doc = "Bit 17 - Clears the interrupt triggered when the ULP-RISCV is trapped."]
    #[inline(always)]
    pub fn cocpu_trap_int_clr(&mut self) -> COCPU_TRAP_INT_CLR_W {
        COCPU_TRAP_INT_CLR_W { w: self }
    }
    #[doc = "Bit 18 - Clears the interrupt triggered when touch sensor times out."]
    #[inline(always)]
    pub fn touch_timeout_int_clr(&mut self) -> TOUCH_TIMEOUT_INT_CLR_W {
        TOUCH_TIMEOUT_INT_CLR_W { w: self }
    }
    #[doc = "Bit 19 - Clears the interrupt triggered when a glitch is detected."]
    #[inline(always)]
    pub fn glitch_det_int_clr(&mut self) -> GLITCH_DET_INT_CLR_W {
        GLITCH_DET_INT_CLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC interrupt clear register\n\nThis register you can [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_clr_rtc]
(index.html) module"]
pub struct INT_CLR_RTC_SPEC;
impl crate::RegisterSpec for INT_CLR_RTC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [int_clr_rtc::W]
(W) writer structure"]
impl crate::Writable for INT_CLR_RTC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_CLR_RTC to value 0"]
impl crate::Resettable for INT_CLR_RTC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
