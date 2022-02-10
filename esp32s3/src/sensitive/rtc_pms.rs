#[doc = "Register `RTC_PMS` reader"]
pub struct R(crate::R<RTC_PMS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_PMS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_PMS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_PMS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_PMS` writer"]
pub struct W(crate::W<RTC_PMS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_PMS_SPEC>;
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
impl From<crate::W<RTC_PMS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_PMS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIS_RTC_CPU` reader - Set 1 to disable rtc coprocessor."]
pub struct DIS_RTC_CPU_R(crate::FieldReader<bool, bool>);
impl DIS_RTC_CPU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIS_RTC_CPU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIS_RTC_CPU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIS_RTC_CPU` writer - Set 1 to disable rtc coprocessor."]
pub struct DIS_RTC_CPU_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_RTC_CPU_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Set 1 to disable rtc coprocessor."]
    #[inline(always)]
    pub fn dis_rtc_cpu(&self) -> DIS_RTC_CPU_R {
        DIS_RTC_CPU_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to disable rtc coprocessor."]
    #[inline(always)]
    pub fn dis_rtc_cpu(&mut self) -> DIS_RTC_CPU_W {
        DIS_RTC_CPU_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC coprocessor permission register.\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_pms]
(index.html) module"]
pub struct RTC_PMS_SPEC;
impl crate::RegisterSpec for RTC_PMS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_pms::R]
(R) reader structure"]
impl crate::Readable for RTC_PMS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_pms::W]
(W) writer structure"]
impl crate::Writable for RTC_PMS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_PMS to value 0"]
impl crate::Resettable for RTC_PMS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
