#[doc = "Register `FSM_WAIT` reader"]
pub struct R(crate::R<FSM_WAIT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSM_WAIT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSM_WAIT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSM_WAIT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSM_WAIT` writer"]
pub struct W(crate::W<FSM_WAIT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSM_WAIT_SPEC>;
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
impl From<crate::W<FSM_WAIT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSM_WAIT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SARADC_XPD_WAIT` reader - the cycle which saradc controller in xpd state"]
pub struct SARADC_XPD_WAIT_R(crate::FieldReader<u8, u8>);
impl SARADC_XPD_WAIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SARADC_XPD_WAIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SARADC_XPD_WAIT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SARADC_XPD_WAIT` writer - the cycle which saradc controller in xpd state"]
pub struct SARADC_XPD_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> SARADC_XPD_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `SARADC_RSTB_WAIT` reader - the cycle which saradc controller in rst state"]
pub struct SARADC_RSTB_WAIT_R(crate::FieldReader<u8, u8>);
impl SARADC_RSTB_WAIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SARADC_RSTB_WAIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SARADC_RSTB_WAIT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SARADC_RSTB_WAIT` writer - the cycle which saradc controller in rst state"]
pub struct SARADC_RSTB_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> SARADC_RSTB_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `SARADC_STANDBY_WAIT` reader - the cycle which saradc controller in standby state"]
pub struct SARADC_STANDBY_WAIT_R(crate::FieldReader<u8, u8>);
impl SARADC_STANDBY_WAIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SARADC_STANDBY_WAIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SARADC_STANDBY_WAIT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SARADC_STANDBY_WAIT` writer - the cycle which saradc controller in standby state"]
pub struct SARADC_STANDBY_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> SARADC_STANDBY_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - the cycle which saradc controller in xpd state"]
    #[inline(always)]
    pub fn saradc_xpd_wait(&self) -> SARADC_XPD_WAIT_R {
        SARADC_XPD_WAIT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - the cycle which saradc controller in rst state"]
    #[inline(always)]
    pub fn saradc_rstb_wait(&self) -> SARADC_RSTB_WAIT_R {
        SARADC_RSTB_WAIT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - the cycle which saradc controller in standby state"]
    #[inline(always)]
    pub fn saradc_standby_wait(&self) -> SARADC_STANDBY_WAIT_R {
        SARADC_STANDBY_WAIT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - the cycle which saradc controller in xpd state"]
    #[inline(always)]
    pub fn saradc_xpd_wait(&mut self) -> SARADC_XPD_WAIT_W {
        SARADC_XPD_WAIT_W { w: self }
    }
    #[doc = "Bits 8:15 - the cycle which saradc controller in rst state"]
    #[inline(always)]
    pub fn saradc_rstb_wait(&mut self) -> SARADC_RSTB_WAIT_W {
        SARADC_RSTB_WAIT_W { w: self }
    }
    #[doc = "Bits 16:23 - the cycle which saradc controller in standby state"]
    #[inline(always)]
    pub fn saradc_standby_wait(&mut self) -> SARADC_STANDBY_WAIT_W {
        SARADC_STANDBY_WAIT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure apb saradc fsm\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_wait]
(index.html) module"]
pub struct FSM_WAIT_SPEC;
impl crate::RegisterSpec for FSM_WAIT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fsm_wait::R]
(R) reader structure"]
impl crate::Readable for FSM_WAIT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fsm_wait::W]
(W) writer structure"]
impl crate::Writable for FSM_WAIT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FSM_WAIT to value 0x00ff_0808"]
impl crate::Resettable for FSM_WAIT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x00ff_0808
    }
}
