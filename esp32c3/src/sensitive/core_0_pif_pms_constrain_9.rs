#[doc = "Register `CORE_0_PIF_PMS_CONSTRAIN_9` reader"]
pub struct R(crate::R<CORE_0_PIF_PMS_CONSTRAIN_9_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_PIF_PMS_CONSTRAIN_9_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_PIF_PMS_CONSTRAIN_9_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_PIF_PMS_CONSTRAIN_9_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_0_PIF_PMS_CONSTRAIN_9` writer"]
pub struct W(crate::W<CORE_0_PIF_PMS_CONSTRAIN_9_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_0_PIF_PMS_CONSTRAIN_9_SPEC>;
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
impl From<crate::W<CORE_0_PIF_PMS_CONSTRAIN_9_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_0_PIF_PMS_CONSTRAIN_9_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_0` reader - core_0_pif_pms_constrain_rtcfast_spltaddr_world_0"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_0_R(crate::FieldReader<u16>);
impl CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_0_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_0` writer - core_0_pif_pms_constrain_rtcfast_spltaddr_world_0"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_0_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | (value as u32 & 0x07ff);
        self.w
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_1` reader - core_0_pif_pms_constrain_rtcfast_spltaddr_world_1"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_1_R(crate::FieldReader<u16>);
impl CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_1_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_1` writer - core_0_pif_pms_constrain_rtcfast_spltaddr_world_1"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_1_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 11)) | ((value as u32 & 0x07ff) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:10 - core_0_pif_pms_constrain_rtcfast_spltaddr_world_0"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_rtcfast_spltaddr_world_0(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_0_R {
        CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_0_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:21 - core_0_pif_pms_constrain_rtcfast_spltaddr_world_1"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_rtcfast_spltaddr_world_1(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_1_R {
        CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_1_R::new(
            ((self.bits >> 11) & 0x07ff) as u16,
        )
    }
}
impl W {
    #[doc = "Bits 0:10 - core_0_pif_pms_constrain_rtcfast_spltaddr_world_0"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_rtcfast_spltaddr_world_0(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_0_W {
        CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_0_W { w: self }
    }
    #[doc = "Bits 11:21 - core_0_pif_pms_constrain_rtcfast_spltaddr_world_1"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_rtcfast_spltaddr_world_1(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_1_W {
        CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_9_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_pif_pms_constrain_9](index.html) module"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_9_SPEC;
impl crate::RegisterSpec for CORE_0_PIF_PMS_CONSTRAIN_9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_pif_pms_constrain_9::R](R) reader structure"]
impl crate::Readable for CORE_0_PIF_PMS_CONSTRAIN_9_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_0_pif_pms_constrain_9::W](W) writer structure"]
impl crate::Writable for CORE_0_PIF_PMS_CONSTRAIN_9_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CORE_0_PIF_PMS_CONSTRAIN_9 to value 0x003f_ffff"]
impl crate::Resettable for CORE_0_PIF_PMS_CONSTRAIN_9_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x003f_ffff
    }
}
