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
#[doc = "Field `XPD_WAIT` reader - xpd wait"]
pub struct XPD_WAIT_R(crate::FieldReader<u8, u8>);
impl XPD_WAIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        XPD_WAIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XPD_WAIT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XPD_WAIT` writer - xpd wait"]
pub struct XPD_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> XPD_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `RSTB_WAIT` reader - reset time"]
pub struct RSTB_WAIT_R(crate::FieldReader<u8, u8>);
impl RSTB_WAIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RSTB_WAIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTB_WAIT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTB_WAIT` writer - reset time"]
pub struct RSTB_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTB_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `STANDBY_WAIT` reader - standby wait"]
pub struct STANDBY_WAIT_R(crate::FieldReader<u8, u8>);
impl STANDBY_WAIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        STANDBY_WAIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STANDBY_WAIT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STANDBY_WAIT` writer - standby wait"]
pub struct STANDBY_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> STANDBY_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - xpd wait"]
    #[inline(always)]
    pub fn xpd_wait(&self) -> XPD_WAIT_R {
        XPD_WAIT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - reset time"]
    #[inline(always)]
    pub fn rstb_wait(&self) -> RSTB_WAIT_R {
        RSTB_WAIT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - standby wait"]
    #[inline(always)]
    pub fn standby_wait(&self) -> STANDBY_WAIT_R {
        STANDBY_WAIT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - xpd wait"]
    #[inline(always)]
    pub fn xpd_wait(&mut self) -> XPD_WAIT_W {
        XPD_WAIT_W { w: self }
    }
    #[doc = "Bits 8:15 - reset time"]
    #[inline(always)]
    pub fn rstb_wait(&mut self) -> RSTB_WAIT_W {
        RSTB_WAIT_W { w: self }
    }
    #[doc = "Bits 16:23 - standby wait"]
    #[inline(always)]
    pub fn standby_wait(&mut self) -> STANDBY_WAIT_W {
        STANDBY_WAIT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure saradc fsm internal parameter base on test\n\nThis register you can [`read`]
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
