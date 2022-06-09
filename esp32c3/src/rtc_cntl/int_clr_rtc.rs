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
pub type SLP_WAKEUP_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_RTC_SPEC, bool, 0>;
#[doc = "Field `SLP_REJECT_INT_CLR` writer - Clear sleep reject interrupt state"]
pub type SLP_REJECT_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_RTC_SPEC, bool, 1>;
#[doc = "Field `RTC_WDT_INT_CLR` writer - Clear RTC WDT interrupt state"]
pub type RTC_WDT_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_RTC_SPEC, bool, 3>;
#[doc = "Field `RTC_BROWN_OUT_INT_CLR` writer - Clear brown out interrupt state"]
pub type RTC_BROWN_OUT_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_RTC_SPEC, bool, 9>;
#[doc = "Field `RTC_MAIN_TIMER_INT_CLR` writer - Clear RTC main timer interrupt state"]
pub type RTC_MAIN_TIMER_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_RTC_SPEC, bool, 10>;
#[doc = "Field `RTC_SWD_INT_CLR` writer - Clear super watch dog interrupt state"]
pub type RTC_SWD_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_RTC_SPEC, bool, 15>;
#[doc = "Field `RTC_XTAL32K_DEAD_INT_CLR` writer - Clear RTC WDT interrupt state"]
pub type RTC_XTAL32K_DEAD_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_RTC_SPEC, bool, 16>;
#[doc = "Field `RTC_GLITCH_DET_INT_CLR` writer - Clear glitch det interrupt state"]
pub type RTC_GLITCH_DET_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_RTC_SPEC, bool, 19>;
#[doc = "Field `RTC_BBPLL_CAL_INT_CLR` writer - clear bbpll cal end interrupt state"]
pub type RTC_BBPLL_CAL_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_RTC_SPEC, bool, 20>;
impl W {
    #[doc = "Bit 0 - Clear sleep wakeup interrupt state"]
    #[inline(always)]
    pub fn slp_wakeup_int_clr(&mut self) -> SLP_WAKEUP_INT_CLR_W {
        SLP_WAKEUP_INT_CLR_W::new(self)
    }
    #[doc = "Bit 1 - Clear sleep reject interrupt state"]
    #[inline(always)]
    pub fn slp_reject_int_clr(&mut self) -> SLP_REJECT_INT_CLR_W {
        SLP_REJECT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 3 - Clear RTC WDT interrupt state"]
    #[inline(always)]
    pub fn rtc_wdt_int_clr(&mut self) -> RTC_WDT_INT_CLR_W {
        RTC_WDT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 9 - Clear brown out interrupt state"]
    #[inline(always)]
    pub fn rtc_brown_out_int_clr(&mut self) -> RTC_BROWN_OUT_INT_CLR_W {
        RTC_BROWN_OUT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 10 - Clear RTC main timer interrupt state"]
    #[inline(always)]
    pub fn rtc_main_timer_int_clr(&mut self) -> RTC_MAIN_TIMER_INT_CLR_W {
        RTC_MAIN_TIMER_INT_CLR_W::new(self)
    }
    #[doc = "Bit 15 - Clear super watch dog interrupt state"]
    #[inline(always)]
    pub fn rtc_swd_int_clr(&mut self) -> RTC_SWD_INT_CLR_W {
        RTC_SWD_INT_CLR_W::new(self)
    }
    #[doc = "Bit 16 - Clear RTC WDT interrupt state"]
    #[inline(always)]
    pub fn rtc_xtal32k_dead_int_clr(&mut self) -> RTC_XTAL32K_DEAD_INT_CLR_W {
        RTC_XTAL32K_DEAD_INT_CLR_W::new(self)
    }
    #[doc = "Bit 19 - Clear glitch det interrupt state"]
    #[inline(always)]
    pub fn rtc_glitch_det_int_clr(&mut self) -> RTC_GLITCH_DET_INT_CLR_W {
        RTC_GLITCH_DET_INT_CLR_W::new(self)
    }
    #[doc = "Bit 20 - clear bbpll cal end interrupt state"]
    #[inline(always)]
    pub fn rtc_bbpll_cal_int_clr(&mut self) -> RTC_BBPLL_CAL_INT_CLR_W {
        RTC_BBPLL_CAL_INT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rtc configure register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_clr_rtc](index.html) module"]
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
