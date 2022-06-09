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
pub type SLP_WAKEUP_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_RTC_SPEC, bool, 0>;
#[doc = "Field `SLP_REJECT_INT_CLR` writer - Clears the interrupt triggered when the chip rejects to go to sleep."]
pub type SLP_REJECT_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_RTC_SPEC, bool, 1>;
#[doc = "Field `SDIO_IDLE_INT_CLR` writer - Clears the interrupt triggered when the SDIO idles."]
pub type SDIO_IDLE_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_RTC_SPEC, bool, 2>;
#[doc = "Field `WDT_INT_CLR` writer - Enables the RTC watchdog interrupt."]
pub type WDT_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_RTC_SPEC, bool, 3>;
#[doc = "Field `TOUCH_SCAN_DONE_INT_CLR` writer - Clears the interrupt triggered upon the completion of a touch scanning."]
pub type TOUCH_SCAN_DONE_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_RTC_SPEC, bool, 4>;
#[doc = "Field `ULP_CP_INT_CLR` writer - Enables the ULP co-processor interrupt."]
pub type ULP_CP_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_RTC_SPEC, bool, 5>;
#[doc = "Field `TOUCH_DONE_INT_CLR` writer - Clears the interrupt triggered upon the completion of a single touch."]
pub type TOUCH_DONE_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_RTC_SPEC, bool, 6>;
#[doc = "Field `TOUCH_ACTIVE_INT_CLR` writer - Clears the interrupt triggered when a touch is detected."]
pub type TOUCH_ACTIVE_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_RTC_SPEC, bool, 7>;
#[doc = "Field `TOUCH_INACTIVE_INT_CLR` writer - Clears the interrupt triggered when a touch is released."]
pub type TOUCH_INACTIVE_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_RTC_SPEC, bool, 8>;
#[doc = "Field `BROWN_OUT_INT_CLR` writer - Clears the brown out interrupt."]
pub type BROWN_OUT_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_RTC_SPEC, bool, 9>;
#[doc = "Field `MAIN_TIMER_INT_CLR` writer - Clears the RTC main timer interrupt."]
pub type MAIN_TIMER_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_RTC_SPEC, bool, 10>;
#[doc = "Field `SARADC1_INT_CLR` writer - Clears the SAR ADC 1 interrupt."]
pub type SARADC1_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_RTC_SPEC, bool, 11>;
#[doc = "Field `TSENS_INT_CLR` writer - Clears the touch sensor interrupt."]
pub type TSENS_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_RTC_SPEC, bool, 12>;
#[doc = "Field `COCPU_INT_CLR` writer - Clears the ULP-RISCV interrupt."]
pub type COCPU_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_RTC_SPEC, bool, 13>;
#[doc = "Field `SARADC2_INT_CLR` writer - Clears the SAR ADC 2 interrupt."]
pub type SARADC2_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_RTC_SPEC, bool, 14>;
#[doc = "Field `SWD_INT_CLR` writer - Clears the super watchdog interrupt."]
pub type SWD_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_RTC_SPEC, bool, 15>;
#[doc = "Field `XTAL32K_DEAD_INT_CLR` writer - Clears the interrupt triggered when the 32 kHz crystal is dead."]
pub type XTAL32K_DEAD_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_RTC_SPEC, bool, 16>;
#[doc = "Field `COCPU_TRAP_INT_CLR` writer - Clears the interrupt triggered when the ULP-RISCV is trapped."]
pub type COCPU_TRAP_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_RTC_SPEC, bool, 17>;
#[doc = "Field `TOUCH_TIMEOUT_INT_CLR` writer - Clears the interrupt triggered when touch sensor times out."]
pub type TOUCH_TIMEOUT_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_RTC_SPEC, bool, 18>;
#[doc = "Field `GLITCH_DET_INT_CLR` writer - Clears the interrupt triggered when a glitch is detected."]
pub type GLITCH_DET_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_RTC_SPEC, bool, 19>;
impl W {
    #[doc = "Bit 0 - Clears the interrupt triggered when the chip wakes up from sleep."]
    #[inline(always)]
    pub fn slp_wakeup_int_clr(&mut self) -> SLP_WAKEUP_INT_CLR_W {
        SLP_WAKEUP_INT_CLR_W::new(self)
    }
    #[doc = "Bit 1 - Clears the interrupt triggered when the chip rejects to go to sleep."]
    #[inline(always)]
    pub fn slp_reject_int_clr(&mut self) -> SLP_REJECT_INT_CLR_W {
        SLP_REJECT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 2 - Clears the interrupt triggered when the SDIO idles."]
    #[inline(always)]
    pub fn sdio_idle_int_clr(&mut self) -> SDIO_IDLE_INT_CLR_W {
        SDIO_IDLE_INT_CLR_W::new(self)
    }
    #[doc = "Bit 3 - Enables the RTC watchdog interrupt."]
    #[inline(always)]
    pub fn wdt_int_clr(&mut self) -> WDT_INT_CLR_W {
        WDT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 4 - Clears the interrupt triggered upon the completion of a touch scanning."]
    #[inline(always)]
    pub fn touch_scan_done_int_clr(&mut self) -> TOUCH_SCAN_DONE_INT_CLR_W {
        TOUCH_SCAN_DONE_INT_CLR_W::new(self)
    }
    #[doc = "Bit 5 - Enables the ULP co-processor interrupt."]
    #[inline(always)]
    pub fn ulp_cp_int_clr(&mut self) -> ULP_CP_INT_CLR_W {
        ULP_CP_INT_CLR_W::new(self)
    }
    #[doc = "Bit 6 - Clears the interrupt triggered upon the completion of a single touch."]
    #[inline(always)]
    pub fn touch_done_int_clr(&mut self) -> TOUCH_DONE_INT_CLR_W {
        TOUCH_DONE_INT_CLR_W::new(self)
    }
    #[doc = "Bit 7 - Clears the interrupt triggered when a touch is detected."]
    #[inline(always)]
    pub fn touch_active_int_clr(&mut self) -> TOUCH_ACTIVE_INT_CLR_W {
        TOUCH_ACTIVE_INT_CLR_W::new(self)
    }
    #[doc = "Bit 8 - Clears the interrupt triggered when a touch is released."]
    #[inline(always)]
    pub fn touch_inactive_int_clr(&mut self) -> TOUCH_INACTIVE_INT_CLR_W {
        TOUCH_INACTIVE_INT_CLR_W::new(self)
    }
    #[doc = "Bit 9 - Clears the brown out interrupt."]
    #[inline(always)]
    pub fn brown_out_int_clr(&mut self) -> BROWN_OUT_INT_CLR_W {
        BROWN_OUT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 10 - Clears the RTC main timer interrupt."]
    #[inline(always)]
    pub fn main_timer_int_clr(&mut self) -> MAIN_TIMER_INT_CLR_W {
        MAIN_TIMER_INT_CLR_W::new(self)
    }
    #[doc = "Bit 11 - Clears the SAR ADC 1 interrupt."]
    #[inline(always)]
    pub fn saradc1_int_clr(&mut self) -> SARADC1_INT_CLR_W {
        SARADC1_INT_CLR_W::new(self)
    }
    #[doc = "Bit 12 - Clears the touch sensor interrupt."]
    #[inline(always)]
    pub fn tsens_int_clr(&mut self) -> TSENS_INT_CLR_W {
        TSENS_INT_CLR_W::new(self)
    }
    #[doc = "Bit 13 - Clears the ULP-RISCV interrupt."]
    #[inline(always)]
    pub fn cocpu_int_clr(&mut self) -> COCPU_INT_CLR_W {
        COCPU_INT_CLR_W::new(self)
    }
    #[doc = "Bit 14 - Clears the SAR ADC 2 interrupt."]
    #[inline(always)]
    pub fn saradc2_int_clr(&mut self) -> SARADC2_INT_CLR_W {
        SARADC2_INT_CLR_W::new(self)
    }
    #[doc = "Bit 15 - Clears the super watchdog interrupt."]
    #[inline(always)]
    pub fn swd_int_clr(&mut self) -> SWD_INT_CLR_W {
        SWD_INT_CLR_W::new(self)
    }
    #[doc = "Bit 16 - Clears the interrupt triggered when the 32 kHz crystal is dead."]
    #[inline(always)]
    pub fn xtal32k_dead_int_clr(&mut self) -> XTAL32K_DEAD_INT_CLR_W {
        XTAL32K_DEAD_INT_CLR_W::new(self)
    }
    #[doc = "Bit 17 - Clears the interrupt triggered when the ULP-RISCV is trapped."]
    #[inline(always)]
    pub fn cocpu_trap_int_clr(&mut self) -> COCPU_TRAP_INT_CLR_W {
        COCPU_TRAP_INT_CLR_W::new(self)
    }
    #[doc = "Bit 18 - Clears the interrupt triggered when touch sensor times out."]
    #[inline(always)]
    pub fn touch_timeout_int_clr(&mut self) -> TOUCH_TIMEOUT_INT_CLR_W {
        TOUCH_TIMEOUT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 19 - Clears the interrupt triggered when a glitch is detected."]
    #[inline(always)]
    pub fn glitch_det_int_clr(&mut self) -> GLITCH_DET_INT_CLR_W {
        GLITCH_DET_INT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC interrupt clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_clr_rtc](index.html) module"]
pub struct INT_CLR_RTC_SPEC;
impl crate::RegisterSpec for INT_CLR_RTC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [int_clr_rtc::W](W) writer structure"]
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
