#[doc = "Register `RTC_TIMER3` reader"]
pub struct R(crate::R<RTC_TIMER3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_TIMER3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_TIMER3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_TIMER3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_TIMER3` writer"]
pub struct W(crate::W<RTC_TIMER3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_TIMER3_SPEC>;
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
impl From<crate::W<RTC_TIMER3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_TIMER3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WIFI_WAIT_TIMER` reader - wifi power domain wakeup time"]
pub struct WIFI_WAIT_TIMER_R(crate::FieldReader<u16, u16>);
impl WIFI_WAIT_TIMER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        WIFI_WAIT_TIMER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WIFI_WAIT_TIMER_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WIFI_WAIT_TIMER` writer - wifi power domain wakeup time"]
pub struct WIFI_WAIT_TIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> WIFI_WAIT_TIMER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | (value as u32 & 0x01ff);
        self.w
    }
}
#[doc = "Field `WIFI_POWERUP_TIMER` reader - wifi power domain power on time"]
pub struct WIFI_POWERUP_TIMER_R(crate::FieldReader<u8, u8>);
impl WIFI_POWERUP_TIMER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WIFI_POWERUP_TIMER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WIFI_POWERUP_TIMER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WIFI_POWERUP_TIMER` writer - wifi power domain power on time"]
pub struct WIFI_POWERUP_TIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> WIFI_POWERUP_TIMER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 9)) | ((value as u32 & 0x7f) << 9);
        self.w
    }
}
#[doc = "Field `BT_WAIT_TIMER` reader - bt power domain wakeup time"]
pub struct BT_WAIT_TIMER_R(crate::FieldReader<u16, u16>);
impl BT_WAIT_TIMER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        BT_WAIT_TIMER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BT_WAIT_TIMER_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BT_WAIT_TIMER` writer - bt power domain wakeup time"]
pub struct BT_WAIT_TIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> BT_WAIT_TIMER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 16)) | ((value as u32 & 0x01ff) << 16);
        self.w
    }
}
#[doc = "Field `BT_POWERUP_TIMER` reader - bt power domain power on time"]
pub struct BT_POWERUP_TIMER_R(crate::FieldReader<u8, u8>);
impl BT_POWERUP_TIMER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BT_POWERUP_TIMER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BT_POWERUP_TIMER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BT_POWERUP_TIMER` writer - bt power domain power on time"]
pub struct BT_POWERUP_TIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> BT_POWERUP_TIMER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 25)) | ((value as u32 & 0x7f) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - wifi power domain wakeup time"]
    #[inline(always)]
    pub fn wifi_wait_timer(&self) -> WIFI_WAIT_TIMER_R {
        WIFI_WAIT_TIMER_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:15 - wifi power domain power on time"]
    #[inline(always)]
    pub fn wifi_powerup_timer(&self) -> WIFI_POWERUP_TIMER_R {
        WIFI_POWERUP_TIMER_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 16:24 - bt power domain wakeup time"]
    #[inline(always)]
    pub fn bt_wait_timer(&self) -> BT_WAIT_TIMER_R {
        BT_WAIT_TIMER_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bits 25:31 - bt power domain power on time"]
    #[inline(always)]
    pub fn bt_powerup_timer(&self) -> BT_POWERUP_TIMER_R {
        BT_POWERUP_TIMER_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - wifi power domain wakeup time"]
    #[inline(always)]
    pub fn wifi_wait_timer(&mut self) -> WIFI_WAIT_TIMER_W {
        WIFI_WAIT_TIMER_W { w: self }
    }
    #[doc = "Bits 9:15 - wifi power domain power on time"]
    #[inline(always)]
    pub fn wifi_powerup_timer(&mut self) -> WIFI_POWERUP_TIMER_W {
        WIFI_POWERUP_TIMER_W { w: self }
    }
    #[doc = "Bits 16:24 - bt power domain wakeup time"]
    #[inline(always)]
    pub fn bt_wait_timer(&mut self) -> BT_WAIT_TIMER_W {
        BT_WAIT_TIMER_W { w: self }
    }
    #[doc = "Bits 25:31 - bt power domain power on time"]
    #[inline(always)]
    pub fn bt_powerup_timer(&mut self) -> BT_POWERUP_TIMER_W {
        BT_POWERUP_TIMER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rtc configure register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_timer3]
(index.html) module"]
pub struct RTC_TIMER3_SPEC;
impl crate::RegisterSpec for RTC_TIMER3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_timer3::R]
(R) reader structure"]
impl crate::Readable for RTC_TIMER3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_timer3::W]
(W) writer structure"]
impl crate::Writable for RTC_TIMER3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_TIMER3 to value 0x0a08_0a08"]
impl crate::Resettable for RTC_TIMER3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0a08_0a08
    }
}
