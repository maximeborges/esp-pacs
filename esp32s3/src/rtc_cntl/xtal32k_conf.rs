#[doc = "Register `XTAL32K_CONF` reader"]
pub struct R(crate::R<XTAL32K_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XTAL32K_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XTAL32K_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XTAL32K_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XTAL32K_CONF` writer"]
pub struct W(crate::W<XTAL32K_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XTAL32K_CONF_SPEC>;
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
impl From<crate::W<XTAL32K_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XTAL32K_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XTAL32K_RETURN_WAIT` reader - cycles to wait to return noral xtal 32k"]
pub struct XTAL32K_RETURN_WAIT_R(crate::FieldReader<u8>);
impl XTAL32K_RETURN_WAIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        XTAL32K_RETURN_WAIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTAL32K_RETURN_WAIT_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTAL32K_RETURN_WAIT` writer - cycles to wait to return noral xtal 32k"]
pub struct XTAL32K_RETURN_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL32K_RETURN_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `XTAL32K_RESTART_WAIT` reader - cycles to wait to repower on xtal 32k"]
pub struct XTAL32K_RESTART_WAIT_R(crate::FieldReader<u16>);
impl XTAL32K_RESTART_WAIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        XTAL32K_RESTART_WAIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTAL32K_RESTART_WAIT_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTAL32K_RESTART_WAIT` writer - cycles to wait to repower on xtal 32k"]
pub struct XTAL32K_RESTART_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL32K_RESTART_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 4)) | ((value as u32 & 0xffff) << 4);
        self.w
    }
}
#[doc = "Field `XTAL32K_WDT_TIMEOUT` reader - If no clock detected for this amount of time 32k is regarded as dead"]
pub struct XTAL32K_WDT_TIMEOUT_R(crate::FieldReader<u8>);
impl XTAL32K_WDT_TIMEOUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        XTAL32K_WDT_TIMEOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTAL32K_WDT_TIMEOUT_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTAL32K_WDT_TIMEOUT` writer - If no clock detected for this amount of time 32k is regarded as dead"]
pub struct XTAL32K_WDT_TIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL32K_WDT_TIMEOUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 20)) | ((value as u32 & 0xff) << 20);
        self.w
    }
}
#[doc = "Field `XTAL32K_STABLE_THRES` reader - if restarted xtal32k period is smaller than this, it is regarded as stable"]
pub struct XTAL32K_STABLE_THRES_R(crate::FieldReader<u8>);
impl XTAL32K_STABLE_THRES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        XTAL32K_STABLE_THRES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTAL32K_STABLE_THRES_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTAL32K_STABLE_THRES` writer - if restarted xtal32k period is smaller than this, it is regarded as stable"]
pub struct XTAL32K_STABLE_THRES_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL32K_STABLE_THRES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - cycles to wait to return noral xtal 32k"]
    #[inline(always)]
    pub fn xtal32k_return_wait(&self) -> XTAL32K_RETURN_WAIT_R {
        XTAL32K_RETURN_WAIT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:19 - cycles to wait to repower on xtal 32k"]
    #[inline(always)]
    pub fn xtal32k_restart_wait(&self) -> XTAL32K_RESTART_WAIT_R {
        XTAL32K_RESTART_WAIT_R::new(((self.bits >> 4) & 0xffff) as u16)
    }
    #[doc = "Bits 20:27 - If no clock detected for this amount of time 32k is regarded as dead"]
    #[inline(always)]
    pub fn xtal32k_wdt_timeout(&self) -> XTAL32K_WDT_TIMEOUT_R {
        XTAL32K_WDT_TIMEOUT_R::new(((self.bits >> 20) & 0xff) as u8)
    }
    #[doc = "Bits 28:31 - if restarted xtal32k period is smaller than this, it is regarded as stable"]
    #[inline(always)]
    pub fn xtal32k_stable_thres(&self) -> XTAL32K_STABLE_THRES_R {
        XTAL32K_STABLE_THRES_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - cycles to wait to return noral xtal 32k"]
    #[inline(always)]
    pub fn xtal32k_return_wait(&mut self) -> XTAL32K_RETURN_WAIT_W {
        XTAL32K_RETURN_WAIT_W { w: self }
    }
    #[doc = "Bits 4:19 - cycles to wait to repower on xtal 32k"]
    #[inline(always)]
    pub fn xtal32k_restart_wait(&mut self) -> XTAL32K_RESTART_WAIT_W {
        XTAL32K_RESTART_WAIT_W { w: self }
    }
    #[doc = "Bits 20:27 - If no clock detected for this amount of time 32k is regarded as dead"]
    #[inline(always)]
    pub fn xtal32k_wdt_timeout(&mut self) -> XTAL32K_WDT_TIMEOUT_W {
        XTAL32K_WDT_TIMEOUT_W { w: self }
    }
    #[doc = "Bits 28:31 - if restarted xtal32k period is smaller than this, it is regarded as stable"]
    #[inline(always)]
    pub fn xtal32k_stable_thres(&mut self) -> XTAL32K_STABLE_THRES_W {
        XTAL32K_STABLE_THRES_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure xtal32k\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xtal32k_conf](index.html) module"]
pub struct XTAL32K_CONF_SPEC;
impl crate::RegisterSpec for XTAL32K_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xtal32k_conf::R](R) reader structure"]
impl crate::Readable for XTAL32K_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xtal32k_conf::W](W) writer structure"]
impl crate::Writable for XTAL32K_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets XTAL32K_CONF to value 0x0ff0_0000"]
impl crate::Resettable for XTAL32K_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0ff0_0000
    }
}
