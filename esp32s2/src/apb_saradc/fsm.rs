#[doc = "Register `FSM` reader"]
pub struct R(crate::R<FSM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSM` writer"]
pub struct W(crate::W<FSM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSM_SPEC>;
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
impl From<crate::W<FSM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAMPLE_NUM` reader - sample number"]
pub struct SAMPLE_NUM_R(crate::FieldReader<u8, u8>);
impl SAMPLE_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SAMPLE_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAMPLE_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAMPLE_NUM` writer - sample number"]
pub struct SAMPLE_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPLE_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `SAMPLE_CYCLE` reader - sample cycles"]
pub struct SAMPLE_CYCLE_R(crate::FieldReader<u8, u8>);
impl SAMPLE_CYCLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SAMPLE_CYCLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAMPLE_CYCLE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAMPLE_CYCLE` writer - sample cycles"]
pub struct SAMPLE_CYCLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPLE_CYCLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:23 - sample number"]
    #[inline(always)]
    pub fn sample_num(&self) -> SAMPLE_NUM_R {
        SAMPLE_NUM_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - sample cycles"]
    #[inline(always)]
    pub fn sample_cycle(&self) -> SAMPLE_CYCLE_R {
        SAMPLE_CYCLE_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - sample number"]
    #[inline(always)]
    pub fn sample_num(&mut self) -> SAMPLE_NUM_W {
        SAMPLE_NUM_W { w: self }
    }
    #[doc = "Bits 24:31 - sample cycles"]
    #[inline(always)]
    pub fn sample_cycle(&mut self) -> SAMPLE_CYCLE_W {
        SAMPLE_CYCLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "digital adc control register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm]
(index.html) module"]
pub struct FSM_SPEC;
impl crate::RegisterSpec for FSM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fsm::R]
(R) reader structure"]
impl crate::Readable for FSM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fsm::W]
(W) writer structure"]
impl crate::Writable for FSM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FSM to value 0x0200_0000"]
impl crate::Resettable for FSM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200_0000
    }
}
