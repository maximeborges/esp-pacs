#[doc = "Register `ARB_CTRL` reader"]
pub struct R(crate::R<ARB_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ARB_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ARB_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ARB_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ARB_CTRL` writer"]
pub struct W(crate::W<ARB_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ARB_CTRL_SPEC>;
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
impl From<crate::W<ARB_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ARB_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC_ARB_APB_FORCE` reader - ADC2 arbiter forces to enable DIG ADC2 CTRL."]
pub type ADC_ARB_APB_FORCE_R = crate::BitReader<bool>;
#[doc = "Field `ADC_ARB_APB_FORCE` writer - ADC2 arbiter forces to enable DIG ADC2 CTRL."]
pub type ADC_ARB_APB_FORCE_W<'a> = crate::BitWriter<'a, u32, ARB_CTRL_SPEC, bool, 2>;
#[doc = "Field `ADC_ARB_RTC_FORCE` reader - ADC2 arbiter forces to enable RTC ADC2 CTRL."]
pub type ADC_ARB_RTC_FORCE_R = crate::BitReader<bool>;
#[doc = "Field `ADC_ARB_RTC_FORCE` writer - ADC2 arbiter forces to enable RTC ADC2 CTRL."]
pub type ADC_ARB_RTC_FORCE_W<'a> = crate::BitWriter<'a, u32, ARB_CTRL_SPEC, bool, 3>;
#[doc = "Field `ADC_ARB_WIFI_FORCE` reader - ADC2 arbiter forces to enable PWDET/PKDET CTRL."]
pub type ADC_ARB_WIFI_FORCE_R = crate::BitReader<bool>;
#[doc = "Field `ADC_ARB_WIFI_FORCE` writer - ADC2 arbiter forces to enable PWDET/PKDET CTRL."]
pub type ADC_ARB_WIFI_FORCE_W<'a> = crate::BitWriter<'a, u32, ARB_CTRL_SPEC, bool, 4>;
#[doc = "Field `ADC_ARB_GRANT_FORCE` reader - ADC2 arbiter force grant."]
pub type ADC_ARB_GRANT_FORCE_R = crate::BitReader<bool>;
#[doc = "Field `ADC_ARB_GRANT_FORCE` writer - ADC2 arbiter force grant."]
pub type ADC_ARB_GRANT_FORCE_W<'a> = crate::BitWriter<'a, u32, ARB_CTRL_SPEC, bool, 5>;
#[doc = "Field `ADC_ARB_APB_PRIORITY` reader - Set DIG ADC2 CTRL priority."]
pub type ADC_ARB_APB_PRIORITY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADC_ARB_APB_PRIORITY` writer - Set DIG ADC2 CTRL priority."]
pub type ADC_ARB_APB_PRIORITY_W<'a> = crate::FieldWriter<'a, u32, ARB_CTRL_SPEC, u8, u8, 2, 6>;
#[doc = "Field `ADC_ARB_RTC_PRIORITY` reader - Set RTC ADC2 CTRL priority."]
pub type ADC_ARB_RTC_PRIORITY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADC_ARB_RTC_PRIORITY` writer - Set RTC ADC2 CTRL priority."]
pub type ADC_ARB_RTC_PRIORITY_W<'a> = crate::FieldWriter<'a, u32, ARB_CTRL_SPEC, u8, u8, 2, 8>;
#[doc = "Field `ADC_ARB_WIFI_PRIORITY` reader - Set PWDET/PKDET CTRL priority."]
pub type ADC_ARB_WIFI_PRIORITY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADC_ARB_WIFI_PRIORITY` writer - Set PWDET/PKDET CTRL priority."]
pub type ADC_ARB_WIFI_PRIORITY_W<'a> = crate::FieldWriter<'a, u32, ARB_CTRL_SPEC, u8, u8, 2, 10>;
#[doc = "Field `ADC_ARB_FIX_PRIORITY` reader - ADC2 arbiter uses fixed priority."]
pub type ADC_ARB_FIX_PRIORITY_R = crate::BitReader<bool>;
#[doc = "Field `ADC_ARB_FIX_PRIORITY` writer - ADC2 arbiter uses fixed priority."]
pub type ADC_ARB_FIX_PRIORITY_W<'a> = crate::BitWriter<'a, u32, ARB_CTRL_SPEC, bool, 12>;
impl R {
    #[doc = "Bit 2 - ADC2 arbiter forces to enable DIG ADC2 CTRL."]
    #[inline(always)]
    pub fn adc_arb_apb_force(&self) -> ADC_ARB_APB_FORCE_R {
        ADC_ARB_APB_FORCE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC2 arbiter forces to enable RTC ADC2 CTRL."]
    #[inline(always)]
    pub fn adc_arb_rtc_force(&self) -> ADC_ARB_RTC_FORCE_R {
        ADC_ARB_RTC_FORCE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC2 arbiter forces to enable PWDET/PKDET CTRL."]
    #[inline(always)]
    pub fn adc_arb_wifi_force(&self) -> ADC_ARB_WIFI_FORCE_R {
        ADC_ARB_WIFI_FORCE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC2 arbiter force grant."]
    #[inline(always)]
    pub fn adc_arb_grant_force(&self) -> ADC_ARB_GRANT_FORCE_R {
        ADC_ARB_GRANT_FORCE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Set DIG ADC2 CTRL priority."]
    #[inline(always)]
    pub fn adc_arb_apb_priority(&self) -> ADC_ARB_APB_PRIORITY_R {
        ADC_ARB_APB_PRIORITY_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Set RTC ADC2 CTRL priority."]
    #[inline(always)]
    pub fn adc_arb_rtc_priority(&self) -> ADC_ARB_RTC_PRIORITY_R {
        ADC_ARB_RTC_PRIORITY_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Set PWDET/PKDET CTRL priority."]
    #[inline(always)]
    pub fn adc_arb_wifi_priority(&self) -> ADC_ARB_WIFI_PRIORITY_R {
        ADC_ARB_WIFI_PRIORITY_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - ADC2 arbiter uses fixed priority."]
    #[inline(always)]
    pub fn adc_arb_fix_priority(&self) -> ADC_ARB_FIX_PRIORITY_R {
        ADC_ARB_FIX_PRIORITY_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - ADC2 arbiter forces to enable DIG ADC2 CTRL."]
    #[inline(always)]
    pub fn adc_arb_apb_force(&mut self) -> ADC_ARB_APB_FORCE_W {
        ADC_ARB_APB_FORCE_W::new(self)
    }
    #[doc = "Bit 3 - ADC2 arbiter forces to enable RTC ADC2 CTRL."]
    #[inline(always)]
    pub fn adc_arb_rtc_force(&mut self) -> ADC_ARB_RTC_FORCE_W {
        ADC_ARB_RTC_FORCE_W::new(self)
    }
    #[doc = "Bit 4 - ADC2 arbiter forces to enable PWDET/PKDET CTRL."]
    #[inline(always)]
    pub fn adc_arb_wifi_force(&mut self) -> ADC_ARB_WIFI_FORCE_W {
        ADC_ARB_WIFI_FORCE_W::new(self)
    }
    #[doc = "Bit 5 - ADC2 arbiter force grant."]
    #[inline(always)]
    pub fn adc_arb_grant_force(&mut self) -> ADC_ARB_GRANT_FORCE_W {
        ADC_ARB_GRANT_FORCE_W::new(self)
    }
    #[doc = "Bits 6:7 - Set DIG ADC2 CTRL priority."]
    #[inline(always)]
    pub fn adc_arb_apb_priority(&mut self) -> ADC_ARB_APB_PRIORITY_W {
        ADC_ARB_APB_PRIORITY_W::new(self)
    }
    #[doc = "Bits 8:9 - Set RTC ADC2 CTRL priority."]
    #[inline(always)]
    pub fn adc_arb_rtc_priority(&mut self) -> ADC_ARB_RTC_PRIORITY_W {
        ADC_ARB_RTC_PRIORITY_W::new(self)
    }
    #[doc = "Bits 10:11 - Set PWDET/PKDET CTRL priority."]
    #[inline(always)]
    pub fn adc_arb_wifi_priority(&mut self) -> ADC_ARB_WIFI_PRIORITY_W {
        ADC_ARB_WIFI_PRIORITY_W::new(self)
    }
    #[doc = "Bit 12 - ADC2 arbiter uses fixed priority."]
    #[inline(always)]
    pub fn adc_arb_fix_priority(&mut self) -> ADC_ARB_FIX_PRIORITY_W {
        ADC_ARB_FIX_PRIORITY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configure the settings of DIG ADC2 arbiter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_ctrl](index.html) module"]
pub struct ARB_CTRL_SPEC;
impl crate::RegisterSpec for ARB_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [arb_ctrl::R](R) reader structure"]
impl crate::Readable for ARB_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [arb_ctrl::W](W) writer structure"]
impl crate::Writable for ARB_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ARB_CTRL to value 0x0900"]
impl crate::Resettable for ARB_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0900
    }
}
