#[doc = "Register `SLP_TIMER0` reader"]
pub struct R(crate::R<SLP_TIMER0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLP_TIMER0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLP_TIMER0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLP_TIMER0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLP_TIMER0` writer"]
pub struct W(crate::W<SLP_TIMER0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLP_TIMER0_SPEC>;
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
impl From<crate::W<SLP_TIMER0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLP_TIMER0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLP_VAL_LO` reader - Sets the lower 32 bits of the trigger threshold for the RTC timer."]
pub struct SLP_VAL_LO_R(crate::FieldReader<u32>);
impl SLP_VAL_LO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        SLP_VAL_LO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLP_VAL_LO_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLP_VAL_LO` writer - Sets the lower 32 bits of the trigger threshold for the RTC timer."]
pub struct SLP_VAL_LO_W<'a> {
    w: &'a mut W,
}
impl<'a> SLP_VAL_LO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Sets the lower 32 bits of the trigger threshold for the RTC timer."]
    #[inline(always)]
    pub fn slp_val_lo(&self) -> SLP_VAL_LO_R {
        SLP_VAL_LO_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Sets the lower 32 bits of the trigger threshold for the RTC timer."]
    #[inline(always)]
    pub fn slp_val_lo(&mut self) -> SLP_VAL_LO_W {
        SLP_VAL_LO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC timer threshold register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slp_timer0](index.html) module"]
pub struct SLP_TIMER0_SPEC;
impl crate::RegisterSpec for SLP_TIMER0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slp_timer0::R](R) reader structure"]
impl crate::Readable for SLP_TIMER0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slp_timer0::W](W) writer structure"]
impl crate::Writable for SLP_TIMER0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SLP_TIMER0 to value 0"]
impl crate::Resettable for SLP_TIMER0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
