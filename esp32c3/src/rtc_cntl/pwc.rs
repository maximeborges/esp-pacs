#[doc = "Register `PWC` reader"]
pub struct R(crate::R<PWC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWC` writer"]
pub struct W(crate::W<PWC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWC_SPEC>;
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
impl From<crate::W<PWC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_PAD_FORCE_HOLD` reader - rtc pad force hold"]
pub struct RTC_PAD_FORCE_HOLD_R(crate::FieldReader<bool>);
impl RTC_PAD_FORCE_HOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_PAD_FORCE_HOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_PAD_FORCE_HOLD_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_PAD_FORCE_HOLD` writer - rtc pad force hold"]
pub struct RTC_PAD_FORCE_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_PAD_FORCE_HOLD_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 21)) | ((value as u32 & 1) << 21);
        self.w
    }
}
impl R {
    #[doc = "Bit 21 - rtc pad force hold"]
    #[inline(always)]
    pub fn rtc_pad_force_hold(&self) -> RTC_PAD_FORCE_HOLD_R {
        RTC_PAD_FORCE_HOLD_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 21 - rtc pad force hold"]
    #[inline(always)]
    pub fn rtc_pad_force_hold(&mut self) -> RTC_PAD_FORCE_HOLD_W {
        RTC_PAD_FORCE_HOLD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rtc configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwc](index.html) module"]
pub struct PWC_SPEC;
impl crate::RegisterSpec for PWC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwc::R](R) reader structure"]
impl crate::Readable for PWC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwc::W](W) writer structure"]
impl crate::Writable for PWC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWC to value 0"]
impl crate::Resettable for PWC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
