#[doc = "Register `STORE4` reader"]
pub struct R(crate::R<STORE4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STORE4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STORE4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STORE4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STORE4` writer"]
pub struct W(crate::W<STORE4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STORE4_SPEC>;
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
impl From<crate::W<STORE4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STORE4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_SCRATCH4` reader - reserved register"]
pub struct RTC_SCRATCH4_R(crate::FieldReader<u32>);
impl RTC_SCRATCH4_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        RTC_SCRATCH4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_SCRATCH4_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_SCRATCH4` writer - reserved register"]
pub struct RTC_SCRATCH4_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_SCRATCH4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - reserved register"]
    #[inline(always)]
    pub fn rtc_scratch4(&self) -> RTC_SCRATCH4_R {
        RTC_SCRATCH4_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - reserved register"]
    #[inline(always)]
    pub fn rtc_scratch4(&mut self) -> RTC_SCRATCH4_W {
        RTC_SCRATCH4_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rtc configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [store4](index.html) module"]
pub struct STORE4_SPEC;
impl crate::RegisterSpec for STORE4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [store4::R](R) reader structure"]
impl crate::Readable for STORE4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [store4::W](W) writer structure"]
impl crate::Writable for STORE4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STORE4 to value 0"]
impl crate::Resettable for STORE4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
