#[doc = "Register `STEP` reader"]
pub struct R(crate::R<STEP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STEP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STEP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STEP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STEP` writer"]
pub struct W(crate::W<STEP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STEP_SPEC>;
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
impl From<crate::W<STEP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STEP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER_XTAL_STEP` reader - Set system timer increment step when using XTAL_CLK."]
pub struct TIMER_XTAL_STEP_R(crate::FieldReader<u16>);
impl TIMER_XTAL_STEP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TIMER_XTAL_STEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER_XTAL_STEP_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER_XTAL_STEP` writer - Set system timer increment step when using XTAL_CLK."]
pub struct TIMER_XTAL_STEP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_XTAL_STEP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
#[doc = "Field `TIMER_PLL_STEP` reader - Set system timer increment step when using PLL_CLK"]
pub struct TIMER_PLL_STEP_R(crate::FieldReader<u16>);
impl TIMER_PLL_STEP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TIMER_PLL_STEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER_PLL_STEP_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER_PLL_STEP` writer - Set system timer increment step when using PLL_CLK"]
pub struct TIMER_PLL_STEP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_PLL_STEP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 10)) | ((value as u32 & 0x03ff) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Set system timer increment step when using XTAL_CLK."]
    #[inline(always)]
    pub fn timer_xtal_step(&self) -> TIMER_XTAL_STEP_R {
        TIMER_XTAL_STEP_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - Set system timer increment step when using PLL_CLK"]
    #[inline(always)]
    pub fn timer_pll_step(&self) -> TIMER_PLL_STEP_R {
        TIMER_PLL_STEP_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Set system timer increment step when using XTAL_CLK."]
    #[inline(always)]
    pub fn timer_xtal_step(&mut self) -> TIMER_XTAL_STEP_W {
        TIMER_XTAL_STEP_W { w: self }
    }
    #[doc = "Bits 10:19 - Set system timer increment step when using PLL_CLK"]
    #[inline(always)]
    pub fn timer_pll_step(&mut self) -> TIMER_PLL_STEP_W {
        TIMER_PLL_STEP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System timer accumulation step\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [step](index.html) module"]
pub struct STEP_SPEC;
impl crate::RegisterSpec for STEP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [step::R](R) reader structure"]
impl crate::Readable for STEP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [step::W](W) writer structure"]
impl crate::Writable for STEP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STEP to value 0x0450"]
impl crate::Resettable for STEP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0450
    }
}
