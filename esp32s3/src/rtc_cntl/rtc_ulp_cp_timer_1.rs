#[doc = "Register `RTC_ULP_CP_TIMER_1` reader"]
pub struct R(crate::R<RTC_ULP_CP_TIMER_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_ULP_CP_TIMER_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_ULP_CP_TIMER_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_ULP_CP_TIMER_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_ULP_CP_TIMER_1` writer"]
pub struct W(crate::W<RTC_ULP_CP_TIMER_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_ULP_CP_TIMER_1_SPEC>;
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
impl From<crate::W<RTC_ULP_CP_TIMER_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_ULP_CP_TIMER_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ULP_CP_TIMER_SLP_CYCLE` reader - sleep cycles for ULP-coprocessor timer"]
pub struct ULP_CP_TIMER_SLP_CYCLE_R(crate::FieldReader<u32, u32>);
impl ULP_CP_TIMER_SLP_CYCLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        ULP_CP_TIMER_SLP_CYCLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ULP_CP_TIMER_SLP_CYCLE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ULP_CP_TIMER_SLP_CYCLE` writer - sleep cycles for ULP-coprocessor timer"]
pub struct ULP_CP_TIMER_SLP_CYCLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ULP_CP_TIMER_SLP_CYCLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | ((value as u32 & 0x00ff_ffff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:31 - sleep cycles for ULP-coprocessor timer"]
    #[inline(always)]
    pub fn ulp_cp_timer_slp_cycle(&self) -> ULP_CP_TIMER_SLP_CYCLE_R {
        ULP_CP_TIMER_SLP_CYCLE_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 8:31 - sleep cycles for ULP-coprocessor timer"]
    #[inline(always)]
    pub fn ulp_cp_timer_slp_cycle(&mut self) -> ULP_CP_TIMER_SLP_CYCLE_W {
        ULP_CP_TIMER_SLP_CYCLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure ulp sleep time\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_ulp_cp_timer_1]
(index.html) module"]
pub struct RTC_ULP_CP_TIMER_1_SPEC;
impl crate::RegisterSpec for RTC_ULP_CP_TIMER_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_ulp_cp_timer_1::R]
(R) reader structure"]
impl crate::Readable for RTC_ULP_CP_TIMER_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_ulp_cp_timer_1::W]
(W) writer structure"]
impl crate::Writable for RTC_ULP_CP_TIMER_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_ULP_CP_TIMER_1 to value 0xc800"]
impl crate::Resettable for RTC_ULP_CP_TIMER_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xc800
    }
}
