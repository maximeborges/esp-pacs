#[doc = "Register `RTC_TIMER4` reader"]
pub struct R(crate::R<RTC_TIMER4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_TIMER4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_TIMER4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_TIMER4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_TIMER4` writer"]
pub struct W(crate::W<RTC_TIMER4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_TIMER4_SPEC>;
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
impl From<crate::W<RTC_TIMER4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_TIMER4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_WAIT_TIMER` reader - No public"]
pub struct RTC_WAIT_TIMER_R(crate::FieldReader<u16, u16>);
impl RTC_WAIT_TIMER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        RTC_WAIT_TIMER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_WAIT_TIMER_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_WAIT_TIMER` writer - No public"]
pub struct RTC_WAIT_TIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_WAIT_TIMER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | (value as u32 & 0x01ff);
        self.w
    }
}
#[doc = "Field `RTC_POWERUP_TIMER` reader - No public"]
pub struct RTC_POWERUP_TIMER_R(crate::FieldReader<u8, u8>);
impl RTC_POWERUP_TIMER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RTC_POWERUP_TIMER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_POWERUP_TIMER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_POWERUP_TIMER` writer - No public"]
pub struct RTC_POWERUP_TIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_POWERUP_TIMER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 9)) | ((value as u32 & 0x7f) << 9);
        self.w
    }
}
#[doc = "Field `DG_WRAP_WAIT_TIMER` reader - No public"]
pub struct DG_WRAP_WAIT_TIMER_R(crate::FieldReader<u16, u16>);
impl DG_WRAP_WAIT_TIMER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        DG_WRAP_WAIT_TIMER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DG_WRAP_WAIT_TIMER_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DG_WRAP_WAIT_TIMER` writer - No public"]
pub struct DG_WRAP_WAIT_TIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> DG_WRAP_WAIT_TIMER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 16)) | ((value as u32 & 0x01ff) << 16);
        self.w
    }
}
#[doc = "Field `DG_WRAP_POWERUP_TIMER` reader - No public"]
pub struct DG_WRAP_POWERUP_TIMER_R(crate::FieldReader<u8, u8>);
impl DG_WRAP_POWERUP_TIMER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DG_WRAP_POWERUP_TIMER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DG_WRAP_POWERUP_TIMER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DG_WRAP_POWERUP_TIMER` writer - No public"]
pub struct DG_WRAP_POWERUP_TIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> DG_WRAP_POWERUP_TIMER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 25)) | ((value as u32 & 0x7f) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - No public"]
    #[inline(always)]
    pub fn rtc_wait_timer(&self) -> RTC_WAIT_TIMER_R {
        RTC_WAIT_TIMER_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:15 - No public"]
    #[inline(always)]
    pub fn rtc_powerup_timer(&self) -> RTC_POWERUP_TIMER_R {
        RTC_POWERUP_TIMER_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 16:24 - No public"]
    #[inline(always)]
    pub fn dg_wrap_wait_timer(&self) -> DG_WRAP_WAIT_TIMER_R {
        DG_WRAP_WAIT_TIMER_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bits 25:31 - No public"]
    #[inline(always)]
    pub fn dg_wrap_powerup_timer(&self) -> DG_WRAP_POWERUP_TIMER_R {
        DG_WRAP_POWERUP_TIMER_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - No public"]
    #[inline(always)]
    pub fn rtc_wait_timer(&mut self) -> RTC_WAIT_TIMER_W {
        RTC_WAIT_TIMER_W { w: self }
    }
    #[doc = "Bits 9:15 - No public"]
    #[inline(always)]
    pub fn rtc_powerup_timer(&mut self) -> RTC_POWERUP_TIMER_W {
        RTC_POWERUP_TIMER_W { w: self }
    }
    #[doc = "Bits 16:24 - No public"]
    #[inline(always)]
    pub fn dg_wrap_wait_timer(&mut self) -> DG_WRAP_WAIT_TIMER_W {
        DG_WRAP_WAIT_TIMER_W { w: self }
    }
    #[doc = "Bits 25:31 - No public"]
    #[inline(always)]
    pub fn dg_wrap_powerup_timer(&mut self) -> DG_WRAP_POWERUP_TIMER_W {
        DG_WRAP_POWERUP_TIMER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No public\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_timer4]
(index.html) module"]
pub struct RTC_TIMER4_SPEC;
impl crate::RegisterSpec for RTC_TIMER4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_timer4::R]
(R) reader structure"]
impl crate::Readable for RTC_TIMER4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_timer4::W]
(W) writer structure"]
impl crate::Writable for RTC_TIMER4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_TIMER4 to value 0x1020_0a08"]
impl crate::Resettable for RTC_TIMER4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1020_0a08
    }
}
