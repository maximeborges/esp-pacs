#[doc = "Register `INT_ENA_RTC_W1TS` writer"]
pub struct W(crate::W<INT_ENA_RTC_W1TS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_ENA_RTC_W1TS_SPEC>;
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
impl From<crate::W<INT_ENA_RTC_W1TS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_ENA_RTC_W1TS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLP_WAKEUP_INT_ENA_W1TS` writer - enable sleep wakeup interrupt"]
pub struct SLP_WAKEUP_INT_ENA_W1TS_W<'a> {
    w: &'a mut W,
}
impl<'a> SLP_WAKEUP_INT_ENA_W1TS_W<'a> {
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
#[doc = "Field `SLP_REJECT_INT_ENA_W1TS` writer - enable sleep reject interrupt"]
pub struct SLP_REJECT_INT_ENA_W1TS_W<'a> {
    w: &'a mut W,
}
impl<'a> SLP_REJECT_INT_ENA_W1TS_W<'a> {
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
#[doc = "Field `RTC_WDT_INT_ENA_W1TS` writer - enable RTC WDT interrupt"]
pub struct RTC_WDT_INT_ENA_W1TS_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_WDT_INT_ENA_W1TS_W<'a> {
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
#[doc = "Field `RTC_BROWN_OUT_INT_ENA_W1TS` writer - enable brown out interrupt"]
pub struct RTC_BROWN_OUT_INT_ENA_W1TS_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_BROWN_OUT_INT_ENA_W1TS_W<'a> {
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
#[doc = "Field `RTC_MAIN_TIMER_INT_ENA_W1TS` writer - enable RTC main timer interrupt"]
pub struct RTC_MAIN_TIMER_INT_ENA_W1TS_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_MAIN_TIMER_INT_ENA_W1TS_W<'a> {
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
#[doc = "Field `RTC_SWD_INT_ENA_W1TS` writer - enable super watch dog interrupt"]
pub struct RTC_SWD_INT_ENA_W1TS_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_SWD_INT_ENA_W1TS_W<'a> {
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
#[doc = "Field `RTC_XTAL32K_DEAD_INT_ENA_W1TS` writer - enable xtal32k_dead interrupt"]
pub struct RTC_XTAL32K_DEAD_INT_ENA_W1TS_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_XTAL32K_DEAD_INT_ENA_W1TS_W<'a> {
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
#[doc = "Field `RTC_GLITCH_DET_INT_ENA_W1TS` writer - enbale gitch det interrupt"]
pub struct RTC_GLITCH_DET_INT_ENA_W1TS_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_GLITCH_DET_INT_ENA_W1TS_W<'a> {
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
#[doc = "Field `RTC_BBPLL_CAL_INT_ENA_W1TS` writer - enbale bbpll cal interrupt"]
pub struct RTC_BBPLL_CAL_INT_ENA_W1TS_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_BBPLL_CAL_INT_ENA_W1TS_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 20)) | ((value as u32 & 1) << 20);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - enable sleep wakeup interrupt"]
    #[inline(always)]
    pub fn slp_wakeup_int_ena_w1ts(&mut self) -> SLP_WAKEUP_INT_ENA_W1TS_W {
        SLP_WAKEUP_INT_ENA_W1TS_W { w: self }
    }
    #[doc = "Bit 1 - enable sleep reject interrupt"]
    #[inline(always)]
    pub fn slp_reject_int_ena_w1ts(&mut self) -> SLP_REJECT_INT_ENA_W1TS_W {
        SLP_REJECT_INT_ENA_W1TS_W { w: self }
    }
    #[doc = "Bit 3 - enable RTC WDT interrupt"]
    #[inline(always)]
    pub fn rtc_wdt_int_ena_w1ts(&mut self) -> RTC_WDT_INT_ENA_W1TS_W {
        RTC_WDT_INT_ENA_W1TS_W { w: self }
    }
    #[doc = "Bit 9 - enable brown out interrupt"]
    #[inline(always)]
    pub fn rtc_brown_out_int_ena_w1ts(&mut self) -> RTC_BROWN_OUT_INT_ENA_W1TS_W {
        RTC_BROWN_OUT_INT_ENA_W1TS_W { w: self }
    }
    #[doc = "Bit 10 - enable RTC main timer interrupt"]
    #[inline(always)]
    pub fn rtc_main_timer_int_ena_w1ts(&mut self) -> RTC_MAIN_TIMER_INT_ENA_W1TS_W {
        RTC_MAIN_TIMER_INT_ENA_W1TS_W { w: self }
    }
    #[doc = "Bit 15 - enable super watch dog interrupt"]
    #[inline(always)]
    pub fn rtc_swd_int_ena_w1ts(&mut self) -> RTC_SWD_INT_ENA_W1TS_W {
        RTC_SWD_INT_ENA_W1TS_W { w: self }
    }
    #[doc = "Bit 16 - enable xtal32k_dead interrupt"]
    #[inline(always)]
    pub fn rtc_xtal32k_dead_int_ena_w1ts(&mut self) -> RTC_XTAL32K_DEAD_INT_ENA_W1TS_W {
        RTC_XTAL32K_DEAD_INT_ENA_W1TS_W { w: self }
    }
    #[doc = "Bit 19 - enbale gitch det interrupt"]
    #[inline(always)]
    pub fn rtc_glitch_det_int_ena_w1ts(&mut self) -> RTC_GLITCH_DET_INT_ENA_W1TS_W {
        RTC_GLITCH_DET_INT_ENA_W1TS_W { w: self }
    }
    #[doc = "Bit 20 - enbale bbpll cal interrupt"]
    #[inline(always)]
    pub fn rtc_bbpll_cal_int_ena_w1ts(&mut self) -> RTC_BBPLL_CAL_INT_ENA_W1TS_W {
        RTC_BBPLL_CAL_INT_ENA_W1TS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rtc configure register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_ena_rtc_w1ts](index.html) module"]
pub struct INT_ENA_RTC_W1TS_SPEC;
impl crate::RegisterSpec for INT_ENA_RTC_W1TS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [int_ena_rtc_w1ts::W](W) writer structure"]
impl crate::Writable for INT_ENA_RTC_W1TS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_ENA_RTC_W1TS to value 0"]
impl crate::Resettable for INT_ENA_RTC_W1TS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
