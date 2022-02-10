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
#[doc = "Field `SLP_WAKEUP_INT_CLR` writer - Clear sleep wakeup interrupt state"]
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `SLP_REJECT_INT_CLR` writer - Clear sleep reject interrupt state"]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `SDIO_IDLE_INT_CLR` writer - Clear SDIO idle interrupt state"]
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `RTC_WDT_INT_CLR` writer - Clear RTC WDT interrupt state"]
pub struct RTC_WDT_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_WDT_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `RTC_TOUCH_SCAN_DONE_INT_CLR` writer - clear touch scan done interrupt raw"]
pub struct RTC_TOUCH_SCAN_DONE_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_TOUCH_SCAN_DONE_INT_CLR_W<'a> {
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
#[doc = "Field `RTC_ULP_CP_INT_CLR` writer - Clear ULP-coprocessor interrupt state"]
pub struct RTC_ULP_CP_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_ULP_CP_INT_CLR_W<'a> {
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
#[doc = "Field `RTC_TOUCH_DONE_INT_CLR` writer - Clear touch done interrupt state"]
pub struct RTC_TOUCH_DONE_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_TOUCH_DONE_INT_CLR_W<'a> {
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
#[doc = "Field `RTC_TOUCH_ACTIVE_INT_CLR` writer - Clear touch active interrupt state"]
pub struct RTC_TOUCH_ACTIVE_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_TOUCH_ACTIVE_INT_CLR_W<'a> {
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
#[doc = "Field `RTC_TOUCH_INACTIVE_INT_CLR` writer - Clear touch inactive interrupt state"]
pub struct RTC_TOUCH_INACTIVE_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_TOUCH_INACTIVE_INT_CLR_W<'a> {
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
#[doc = "Field `RTC_BROWN_OUT_INT_CLR` writer - Clear brown out interrupt state"]
pub struct RTC_BROWN_OUT_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_BROWN_OUT_INT_CLR_W<'a> {
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
#[doc = "Field `RTC_MAIN_TIMER_INT_CLR` writer - Clear RTC main timer interrupt state"]
pub struct RTC_MAIN_TIMER_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_MAIN_TIMER_INT_CLR_W<'a> {
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
#[doc = "Field `RTC_SARADC1_INT_CLR` writer - Clear saradc1 interrupt state"]
pub struct RTC_SARADC1_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_SARADC1_INT_CLR_W<'a> {
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
#[doc = "Field `RTC_TSENS_INT_CLR` writer - Clear tsens interrupt state"]
pub struct RTC_TSENS_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_TSENS_INT_CLR_W<'a> {
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
#[doc = "Field `RTC_COCPU_INT_CLR` writer - Clear riscV cocpu interrupt state"]
pub struct RTC_COCPU_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_COCPU_INT_CLR_W<'a> {
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
#[doc = "Field `RTC_SARADC2_INT_CLR` writer - Clear saradc2 interrupt state"]
pub struct RTC_SARADC2_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_SARADC2_INT_CLR_W<'a> {
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
#[doc = "Field `RTC_SWD_INT_CLR` writer - Clear super watch dog interrupt state"]
pub struct RTC_SWD_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_SWD_INT_CLR_W<'a> {
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
#[doc = "Field `RTC_XTAL32K_DEAD_INT_CLR` writer - Clear RTC WDT interrupt state"]
pub struct RTC_XTAL32K_DEAD_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_XTAL32K_DEAD_INT_CLR_W<'a> {
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
#[doc = "Field `RTC_COCPU_TRAP_INT_CLR` writer - Clear cocpu trap interrupt state"]
pub struct RTC_COCPU_TRAP_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_COCPU_TRAP_INT_CLR_W<'a> {
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
#[doc = "Field `RTC_TOUCH_TIMEOUT_INT_CLR` writer - Clear touch timeout interrupt state"]
pub struct RTC_TOUCH_TIMEOUT_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_TOUCH_TIMEOUT_INT_CLR_W<'a> {
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
#[doc = "Field `RTC_GLITCH_DET_INT_CLR` writer - Clear glitch det interrupt state"]
pub struct RTC_GLITCH_DET_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_GLITCH_DET_INT_CLR_W<'a> {
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
#[doc = "Field `RTC_TOUCH_APPROACH_LOOP_DONE_INT_CLR` writer - cleartouch approach mode loop interrupt state"]
pub struct RTC_TOUCH_APPROACH_LOOP_DONE_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_TOUCH_APPROACH_LOOP_DONE_INT_CLR_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Clear sleep wakeup interrupt state"]
    #[inline(always)]
    pub fn slp_wakeup_int_clr(&mut self) -> SLP_WAKEUP_INT_CLR_W {
        SLP_WAKEUP_INT_CLR_W { w: self }
    }
    #[doc = "Bit 1 - Clear sleep reject interrupt state"]
    #[inline(always)]
    pub fn slp_reject_int_clr(&mut self) -> SLP_REJECT_INT_CLR_W {
        SLP_REJECT_INT_CLR_W { w: self }
    }
    #[doc = "Bit 2 - Clear SDIO idle interrupt state"]
    #[inline(always)]
    pub fn sdio_idle_int_clr(&mut self) -> SDIO_IDLE_INT_CLR_W {
        SDIO_IDLE_INT_CLR_W { w: self }
    }
    #[doc = "Bit 3 - Clear RTC WDT interrupt state"]
    #[inline(always)]
    pub fn rtc_wdt_int_clr(&mut self) -> RTC_WDT_INT_CLR_W {
        RTC_WDT_INT_CLR_W { w: self }
    }
    #[doc = "Bit 4 - clear touch scan done interrupt raw"]
    #[inline(always)]
    pub fn rtc_touch_scan_done_int_clr(&mut self) -> RTC_TOUCH_SCAN_DONE_INT_CLR_W {
        RTC_TOUCH_SCAN_DONE_INT_CLR_W { w: self }
    }
    #[doc = "Bit 5 - Clear ULP-coprocessor interrupt state"]
    #[inline(always)]
    pub fn rtc_ulp_cp_int_clr(&mut self) -> RTC_ULP_CP_INT_CLR_W {
        RTC_ULP_CP_INT_CLR_W { w: self }
    }
    #[doc = "Bit 6 - Clear touch done interrupt state"]
    #[inline(always)]
    pub fn rtc_touch_done_int_clr(&mut self) -> RTC_TOUCH_DONE_INT_CLR_W {
        RTC_TOUCH_DONE_INT_CLR_W { w: self }
    }
    #[doc = "Bit 7 - Clear touch active interrupt state"]
    #[inline(always)]
    pub fn rtc_touch_active_int_clr(&mut self) -> RTC_TOUCH_ACTIVE_INT_CLR_W {
        RTC_TOUCH_ACTIVE_INT_CLR_W { w: self }
    }
    #[doc = "Bit 8 - Clear touch inactive interrupt state"]
    #[inline(always)]
    pub fn rtc_touch_inactive_int_clr(&mut self) -> RTC_TOUCH_INACTIVE_INT_CLR_W {
        RTC_TOUCH_INACTIVE_INT_CLR_W { w: self }
    }
    #[doc = "Bit 9 - Clear brown out interrupt state"]
    #[inline(always)]
    pub fn rtc_brown_out_int_clr(&mut self) -> RTC_BROWN_OUT_INT_CLR_W {
        RTC_BROWN_OUT_INT_CLR_W { w: self }
    }
    #[doc = "Bit 10 - Clear RTC main timer interrupt state"]
    #[inline(always)]
    pub fn rtc_main_timer_int_clr(&mut self) -> RTC_MAIN_TIMER_INT_CLR_W {
        RTC_MAIN_TIMER_INT_CLR_W { w: self }
    }
    #[doc = "Bit 11 - Clear saradc1 interrupt state"]
    #[inline(always)]
    pub fn rtc_saradc1_int_clr(&mut self) -> RTC_SARADC1_INT_CLR_W {
        RTC_SARADC1_INT_CLR_W { w: self }
    }
    #[doc = "Bit 12 - Clear tsens interrupt state"]
    #[inline(always)]
    pub fn rtc_tsens_int_clr(&mut self) -> RTC_TSENS_INT_CLR_W {
        RTC_TSENS_INT_CLR_W { w: self }
    }
    #[doc = "Bit 13 - Clear riscV cocpu interrupt state"]
    #[inline(always)]
    pub fn rtc_cocpu_int_clr(&mut self) -> RTC_COCPU_INT_CLR_W {
        RTC_COCPU_INT_CLR_W { w: self }
    }
    #[doc = "Bit 14 - Clear saradc2 interrupt state"]
    #[inline(always)]
    pub fn rtc_saradc2_int_clr(&mut self) -> RTC_SARADC2_INT_CLR_W {
        RTC_SARADC2_INT_CLR_W { w: self }
    }
    #[doc = "Bit 15 - Clear super watch dog interrupt state"]
    #[inline(always)]
    pub fn rtc_swd_int_clr(&mut self) -> RTC_SWD_INT_CLR_W {
        RTC_SWD_INT_CLR_W { w: self }
    }
    #[doc = "Bit 16 - Clear RTC WDT interrupt state"]
    #[inline(always)]
    pub fn rtc_xtal32k_dead_int_clr(&mut self) -> RTC_XTAL32K_DEAD_INT_CLR_W {
        RTC_XTAL32K_DEAD_INT_CLR_W { w: self }
    }
    #[doc = "Bit 17 - Clear cocpu trap interrupt state"]
    #[inline(always)]
    pub fn rtc_cocpu_trap_int_clr(&mut self) -> RTC_COCPU_TRAP_INT_CLR_W {
        RTC_COCPU_TRAP_INT_CLR_W { w: self }
    }
    #[doc = "Bit 18 - Clear touch timeout interrupt state"]
    #[inline(always)]
    pub fn rtc_touch_timeout_int_clr(&mut self) -> RTC_TOUCH_TIMEOUT_INT_CLR_W {
        RTC_TOUCH_TIMEOUT_INT_CLR_W { w: self }
    }
    #[doc = "Bit 19 - Clear glitch det interrupt state"]
    #[inline(always)]
    pub fn rtc_glitch_det_int_clr(&mut self) -> RTC_GLITCH_DET_INT_CLR_W {
        RTC_GLITCH_DET_INT_CLR_W { w: self }
    }
    #[doc = "Bit 20 - cleartouch approach mode loop interrupt state"]
    #[inline(always)]
    pub fn rtc_touch_approach_loop_done_int_clr(
        &mut self,
    ) -> RTC_TOUCH_APPROACH_LOOP_DONE_INT_CLR_W {
        RTC_TOUCH_APPROACH_LOOP_DONE_INT_CLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rtc interrupt register\n\nThis register you can [`write_with_zero`]
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
