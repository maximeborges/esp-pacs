#[doc = "Register `SLP_TIMER1` reader"]
pub struct R(crate::R<SLP_TIMER1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLP_TIMER1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLP_TIMER1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLP_TIMER1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLP_TIMER1` writer"]
pub struct W(crate::W<SLP_TIMER1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLP_TIMER1_SPEC>;
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
impl From<crate::W<SLP_TIMER1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLP_TIMER1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLP_VAL_HI` reader - Sets the higher 16 bits of the trigger threshold for the RTC timer."]
pub type SLP_VAL_HI_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SLP_VAL_HI` writer - Sets the higher 16 bits of the trigger threshold for the RTC timer."]
pub type SLP_VAL_HI_W<'a> = crate::FieldWriter<'a, u32, SLP_TIMER1_SPEC, u16, u16, 16, 0>;
#[doc = "Field `MAIN_TIMER_ALARM_EN` writer - Sets this bit to enable the timer alarm."]
pub type MAIN_TIMER_ALARM_EN_W<'a> = crate::BitWriter<'a, u32, SLP_TIMER1_SPEC, bool, 16>;
impl R {
    #[doc = "Bits 0:15 - Sets the higher 16 bits of the trigger threshold for the RTC timer."]
    #[inline(always)]
    pub fn slp_val_hi(&self) -> SLP_VAL_HI_R {
        SLP_VAL_HI_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Sets the higher 16 bits of the trigger threshold for the RTC timer."]
    #[inline(always)]
    pub fn slp_val_hi(&mut self) -> SLP_VAL_HI_W {
        SLP_VAL_HI_W::new(self)
    }
    #[doc = "Bit 16 - Sets this bit to enable the timer alarm."]
    #[inline(always)]
    pub fn main_timer_alarm_en(&mut self) -> MAIN_TIMER_ALARM_EN_W {
        MAIN_TIMER_ALARM_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC timer threshold register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slp_timer1](index.html) module"]
pub struct SLP_TIMER1_SPEC;
impl crate::RegisterSpec for SLP_TIMER1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slp_timer1::R](R) reader structure"]
impl crate::Readable for SLP_TIMER1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slp_timer1::W](W) writer structure"]
impl crate::Writable for SLP_TIMER1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SLP_TIMER1 to value 0"]
impl crate::Resettable for SLP_TIMER1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
