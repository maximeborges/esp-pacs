#[doc = "Register `REGULATOR_DRV_CTRL` reader"]
pub struct R(crate::R<REGULATOR_DRV_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REGULATOR_DRV_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REGULATOR_DRV_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REGULATOR_DRV_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REGULATOR_DRV_CTRL` writer"]
pub struct W(crate::W<REGULATOR_DRV_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REGULATOR_DRV_CTRL_SPEC>;
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
impl From<crate::W<REGULATOR_DRV_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REGULATOR_DRV_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_REGULATOR_DRV_B_MONITOR` reader - No public"]
pub struct RTC_REGULATOR_DRV_B_MONITOR_R(crate::FieldReader<u8, u8>);
impl RTC_REGULATOR_DRV_B_MONITOR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RTC_REGULATOR_DRV_B_MONITOR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_REGULATOR_DRV_B_MONITOR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_REGULATOR_DRV_B_MONITOR` writer - No public"]
pub struct RTC_REGULATOR_DRV_B_MONITOR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_REGULATOR_DRV_B_MONITOR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Field `RTC_REGULATOR_DRV_B_SLP` reader - No public"]
pub struct RTC_REGULATOR_DRV_B_SLP_R(crate::FieldReader<u8, u8>);
impl RTC_REGULATOR_DRV_B_SLP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RTC_REGULATOR_DRV_B_SLP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_REGULATOR_DRV_B_SLP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_REGULATOR_DRV_B_SLP` writer - No public"]
pub struct RTC_REGULATOR_DRV_B_SLP_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_REGULATOR_DRV_B_SLP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 6)) | ((value as u32 & 0x3f) << 6);
        self.w
    }
}
#[doc = "Field `DG_VDD_DRV_B_SLP` reader - No public"]
pub struct DG_VDD_DRV_B_SLP_R(crate::FieldReader<u8, u8>);
impl DG_VDD_DRV_B_SLP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DG_VDD_DRV_B_SLP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DG_VDD_DRV_B_SLP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DG_VDD_DRV_B_SLP` writer - No public"]
pub struct DG_VDD_DRV_B_SLP_W<'a> {
    w: &'a mut W,
}
impl<'a> DG_VDD_DRV_B_SLP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 12)) | ((value as u32 & 0xff) << 12);
        self.w
    }
}
#[doc = "Field `DG_VDD_DRV_B_MONITOR` reader - No public"]
pub struct DG_VDD_DRV_B_MONITOR_R(crate::FieldReader<u8, u8>);
impl DG_VDD_DRV_B_MONITOR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DG_VDD_DRV_B_MONITOR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DG_VDD_DRV_B_MONITOR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DG_VDD_DRV_B_MONITOR` writer - No public"]
pub struct DG_VDD_DRV_B_MONITOR_W<'a> {
    w: &'a mut W,
}
impl<'a> DG_VDD_DRV_B_MONITOR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 20)) | ((value as u32 & 0xff) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - No public"]
    #[inline(always)]
    pub fn rtc_regulator_drv_b_monitor(&self) -> RTC_REGULATOR_DRV_B_MONITOR_R {
        RTC_REGULATOR_DRV_B_MONITOR_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - No public"]
    #[inline(always)]
    pub fn rtc_regulator_drv_b_slp(&self) -> RTC_REGULATOR_DRV_B_SLP_R {
        RTC_REGULATOR_DRV_B_SLP_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:19 - No public"]
    #[inline(always)]
    pub fn dg_vdd_drv_b_slp(&self) -> DG_VDD_DRV_B_SLP_R {
        DG_VDD_DRV_B_SLP_R::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bits 20:27 - No public"]
    #[inline(always)]
    pub fn dg_vdd_drv_b_monitor(&self) -> DG_VDD_DRV_B_MONITOR_R {
        DG_VDD_DRV_B_MONITOR_R::new(((self.bits >> 20) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - No public"]
    #[inline(always)]
    pub fn rtc_regulator_drv_b_monitor(&mut self) -> RTC_REGULATOR_DRV_B_MONITOR_W {
        RTC_REGULATOR_DRV_B_MONITOR_W { w: self }
    }
    #[doc = "Bits 6:11 - No public"]
    #[inline(always)]
    pub fn rtc_regulator_drv_b_slp(&mut self) -> RTC_REGULATOR_DRV_B_SLP_W {
        RTC_REGULATOR_DRV_B_SLP_W { w: self }
    }
    #[doc = "Bits 12:19 - No public"]
    #[inline(always)]
    pub fn dg_vdd_drv_b_slp(&mut self) -> DG_VDD_DRV_B_SLP_W {
        DG_VDD_DRV_B_SLP_W { w: self }
    }
    #[doc = "Bits 20:27 - No public"]
    #[inline(always)]
    pub fn dg_vdd_drv_b_monitor(&mut self) -> DG_VDD_DRV_B_MONITOR_W {
        DG_VDD_DRV_B_MONITOR_W { w: self }
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
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [regulator_drv_ctrl]
(index.html) module"]
pub struct REGULATOR_DRV_CTRL_SPEC;
impl crate::RegisterSpec for REGULATOR_DRV_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [regulator_drv_ctrl::R]
(R) reader structure"]
impl crate::Readable for REGULATOR_DRV_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [regulator_drv_ctrl::W]
(W) writer structure"]
impl crate::Writable for REGULATOR_DRV_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REGULATOR_DRV_CTRL to value 0"]
impl crate::Resettable for REGULATOR_DRV_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
