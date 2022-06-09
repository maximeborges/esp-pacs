#[doc = "Register `INT_ENA_RTC_W1TC` writer"]
pub struct W(crate::W<INT_ENA_RTC_W1TC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_ENA_RTC_W1TC_SPEC>;
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
impl From<crate::W<INT_ENA_RTC_W1TC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_ENA_RTC_W1TC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLP_WAKEUP_INT_ENA_W1TC` writer - clear sleep wakeup interrupt enable"]
pub type SLP_WAKEUP_INT_ENA_W1TC_W<'a> = crate::BitWriter<'a, u32, INT_ENA_RTC_W1TC_SPEC, bool, 0>;
#[doc = "Field `SLP_REJECT_INT_ENA_W1TC` writer - clear sleep reject interrupt enable"]
pub type SLP_REJECT_INT_ENA_W1TC_W<'a> = crate::BitWriter<'a, u32, INT_ENA_RTC_W1TC_SPEC, bool, 1>;
#[doc = "Field `RTC_WDT_INT_ENA_W1TC` writer - clear RTC WDT interrupt enable"]
pub type RTC_WDT_INT_ENA_W1TC_W<'a> = crate::BitWriter<'a, u32, INT_ENA_RTC_W1TC_SPEC, bool, 3>;
#[doc = "Field `RTC_BROWN_OUT_INT_ENA_W1TC` writer - clear brown out interrupt enable"]
pub type RTC_BROWN_OUT_INT_ENA_W1TC_W<'a> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_W1TC_SPEC, bool, 9>;
#[doc = "Field `RTC_MAIN_TIMER_INT_ENA_W1TC` writer - Clear RTC main timer interrupt enable"]
pub type RTC_MAIN_TIMER_INT_ENA_W1TC_W<'a> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_W1TC_SPEC, bool, 10>;
#[doc = "Field `RTC_SWD_INT_ENA_W1TC` writer - clear super watch dog interrupt enable"]
pub type RTC_SWD_INT_ENA_W1TC_W<'a> = crate::BitWriter<'a, u32, INT_ENA_RTC_W1TC_SPEC, bool, 15>;
#[doc = "Field `RTC_XTAL32K_DEAD_INT_ENA_W1TC` writer - clear xtal32k_dead interrupt enable"]
pub type RTC_XTAL32K_DEAD_INT_ENA_W1TC_W<'a> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_W1TC_SPEC, bool, 16>;
#[doc = "Field `RTC_GLITCH_DET_INT_ENA_W1TC` writer - clear gitch det interrupt enable"]
pub type RTC_GLITCH_DET_INT_ENA_W1TC_W<'a> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_W1TC_SPEC, bool, 19>;
#[doc = "Field `RTC_BBPLL_CAL_INT_ENA_W1TC` writer - clear bbpll cal interrupt enable"]
pub type RTC_BBPLL_CAL_INT_ENA_W1TC_W<'a> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_W1TC_SPEC, bool, 20>;
impl W {
    #[doc = "Bit 0 - clear sleep wakeup interrupt enable"]
    #[inline(always)]
    pub fn slp_wakeup_int_ena_w1tc(&mut self) -> SLP_WAKEUP_INT_ENA_W1TC_W {
        SLP_WAKEUP_INT_ENA_W1TC_W::new(self)
    }
    #[doc = "Bit 1 - clear sleep reject interrupt enable"]
    #[inline(always)]
    pub fn slp_reject_int_ena_w1tc(&mut self) -> SLP_REJECT_INT_ENA_W1TC_W {
        SLP_REJECT_INT_ENA_W1TC_W::new(self)
    }
    #[doc = "Bit 3 - clear RTC WDT interrupt enable"]
    #[inline(always)]
    pub fn rtc_wdt_int_ena_w1tc(&mut self) -> RTC_WDT_INT_ENA_W1TC_W {
        RTC_WDT_INT_ENA_W1TC_W::new(self)
    }
    #[doc = "Bit 9 - clear brown out interrupt enable"]
    #[inline(always)]
    pub fn rtc_brown_out_int_ena_w1tc(&mut self) -> RTC_BROWN_OUT_INT_ENA_W1TC_W {
        RTC_BROWN_OUT_INT_ENA_W1TC_W::new(self)
    }
    #[doc = "Bit 10 - Clear RTC main timer interrupt enable"]
    #[inline(always)]
    pub fn rtc_main_timer_int_ena_w1tc(&mut self) -> RTC_MAIN_TIMER_INT_ENA_W1TC_W {
        RTC_MAIN_TIMER_INT_ENA_W1TC_W::new(self)
    }
    #[doc = "Bit 15 - clear super watch dog interrupt enable"]
    #[inline(always)]
    pub fn rtc_swd_int_ena_w1tc(&mut self) -> RTC_SWD_INT_ENA_W1TC_W {
        RTC_SWD_INT_ENA_W1TC_W::new(self)
    }
    #[doc = "Bit 16 - clear xtal32k_dead interrupt enable"]
    #[inline(always)]
    pub fn rtc_xtal32k_dead_int_ena_w1tc(&mut self) -> RTC_XTAL32K_DEAD_INT_ENA_W1TC_W {
        RTC_XTAL32K_DEAD_INT_ENA_W1TC_W::new(self)
    }
    #[doc = "Bit 19 - clear gitch det interrupt enable"]
    #[inline(always)]
    pub fn rtc_glitch_det_int_ena_w1tc(&mut self) -> RTC_GLITCH_DET_INT_ENA_W1TC_W {
        RTC_GLITCH_DET_INT_ENA_W1TC_W::new(self)
    }
    #[doc = "Bit 20 - clear bbpll cal interrupt enable"]
    #[inline(always)]
    pub fn rtc_bbpll_cal_int_ena_w1tc(&mut self) -> RTC_BBPLL_CAL_INT_ENA_W1TC_W {
        RTC_BBPLL_CAL_INT_ENA_W1TC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rtc configure register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_ena_rtc_w1tc](index.html) module"]
pub struct INT_ENA_RTC_W1TC_SPEC;
impl crate::RegisterSpec for INT_ENA_RTC_W1TC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [int_ena_rtc_w1tc::W](W) writer structure"]
impl crate::Writable for INT_ENA_RTC_W1TC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_ENA_RTC_W1TC to value 0"]
impl crate::Resettable for INT_ENA_RTC_W1TC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
