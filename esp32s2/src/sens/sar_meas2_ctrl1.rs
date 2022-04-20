#[doc = "Register `SAR_MEAS2_CTRL1` reader"]
pub struct R(crate::R<SAR_MEAS2_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_MEAS2_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_MEAS2_CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_MEAS2_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_MEAS2_CTRL1` writer"]
pub struct W(crate::W<SAR_MEAS2_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_MEAS2_CTRL1_SPEC>;
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
impl From<crate::W<SAR_MEAS2_CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_MEAS2_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAR2_CNTL_STATE` reader - saradc2_cntl_fsm"]
pub struct SAR2_CNTL_STATE_R(crate::FieldReader<u8, u8>);
impl SAR2_CNTL_STATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SAR2_CNTL_STATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR2_CNTL_STATE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR2_PWDET_CAL_EN` reader - rtc control pwdet enable"]
pub struct SAR2_PWDET_CAL_EN_R(crate::FieldReader<bool, bool>);
impl SAR2_PWDET_CAL_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SAR2_PWDET_CAL_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR2_PWDET_CAL_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR2_PWDET_CAL_EN` writer - rtc control pwdet enable"]
pub struct SAR2_PWDET_CAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR2_PWDET_CAL_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u32 & 1) << 3);
        self.w
    }
}
#[doc = "Field `SAR2_PKDET_CAL_EN` reader - rtc control pkdet enable"]
pub struct SAR2_PKDET_CAL_EN_R(crate::FieldReader<bool, bool>);
impl SAR2_PKDET_CAL_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SAR2_PKDET_CAL_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR2_PKDET_CAL_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR2_PKDET_CAL_EN` writer - rtc control pkdet enable"]
pub struct SAR2_PKDET_CAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR2_PKDET_CAL_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u32 & 1) << 4);
        self.w
    }
}
#[doc = "Field `SAR2_EN_TEST` reader - SAR2_EN_TEST"]
pub struct SAR2_EN_TEST_R(crate::FieldReader<bool, bool>);
impl SAR2_EN_TEST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SAR2_EN_TEST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR2_EN_TEST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR2_EN_TEST` writer - SAR2_EN_TEST"]
pub struct SAR2_EN_TEST_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR2_EN_TEST_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u32 & 1) << 5);
        self.w
    }
}
#[doc = "Field `SAR2_RSTB_FORCE` reader - "]
pub struct SAR2_RSTB_FORCE_R(crate::FieldReader<u8, u8>);
impl SAR2_RSTB_FORCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SAR2_RSTB_FORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR2_RSTB_FORCE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR2_RSTB_FORCE` writer - "]
pub struct SAR2_RSTB_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR2_RSTB_FORCE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 6)) | ((value as u32 & 3) << 6);
        self.w
    }
}
#[doc = "Field `SAR2_STANDBY_WAIT` reader - "]
pub struct SAR2_STANDBY_WAIT_R(crate::FieldReader<u8, u8>);
impl SAR2_STANDBY_WAIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SAR2_STANDBY_WAIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR2_STANDBY_WAIT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR2_STANDBY_WAIT` writer - "]
pub struct SAR2_STANDBY_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR2_STANDBY_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `SAR2_RSTB_WAIT` reader - "]
pub struct SAR2_RSTB_WAIT_R(crate::FieldReader<u8, u8>);
impl SAR2_RSTB_WAIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SAR2_RSTB_WAIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR2_RSTB_WAIT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR2_RSTB_WAIT` writer - "]
pub struct SAR2_RSTB_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR2_RSTB_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `SAR2_XPD_WAIT` reader - "]
pub struct SAR2_XPD_WAIT_R(crate::FieldReader<u8, u8>);
impl SAR2_XPD_WAIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SAR2_XPD_WAIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR2_XPD_WAIT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR2_XPD_WAIT` writer - "]
pub struct SAR2_XPD_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR2_XPD_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - saradc2_cntl_fsm"]
    #[inline(always)]
    pub fn sar2_cntl_state(&self) -> SAR2_CNTL_STATE_R {
        SAR2_CNTL_STATE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - rtc control pwdet enable"]
    #[inline(always)]
    pub fn sar2_pwdet_cal_en(&self) -> SAR2_PWDET_CAL_EN_R {
        SAR2_PWDET_CAL_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - rtc control pkdet enable"]
    #[inline(always)]
    pub fn sar2_pkdet_cal_en(&self) -> SAR2_PKDET_CAL_EN_R {
        SAR2_PKDET_CAL_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SAR2_EN_TEST"]
    #[inline(always)]
    pub fn sar2_en_test(&self) -> SAR2_EN_TEST_R {
        SAR2_EN_TEST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn sar2_rstb_force(&self) -> SAR2_RSTB_FORCE_R {
        SAR2_RSTB_FORCE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn sar2_standby_wait(&self) -> SAR2_STANDBY_WAIT_R {
        SAR2_STANDBY_WAIT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn sar2_rstb_wait(&self) -> SAR2_RSTB_WAIT_R {
        SAR2_RSTB_WAIT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn sar2_xpd_wait(&self) -> SAR2_XPD_WAIT_R {
        SAR2_XPD_WAIT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 3 - rtc control pwdet enable"]
    #[inline(always)]
    pub fn sar2_pwdet_cal_en(&mut self) -> SAR2_PWDET_CAL_EN_W {
        SAR2_PWDET_CAL_EN_W { w: self }
    }
    #[doc = "Bit 4 - rtc control pkdet enable"]
    #[inline(always)]
    pub fn sar2_pkdet_cal_en(&mut self) -> SAR2_PKDET_CAL_EN_W {
        SAR2_PKDET_CAL_EN_W { w: self }
    }
    #[doc = "Bit 5 - SAR2_EN_TEST"]
    #[inline(always)]
    pub fn sar2_en_test(&mut self) -> SAR2_EN_TEST_W {
        SAR2_EN_TEST_W { w: self }
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn sar2_rstb_force(&mut self) -> SAR2_RSTB_FORCE_W {
        SAR2_RSTB_FORCE_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn sar2_standby_wait(&mut self) -> SAR2_STANDBY_WAIT_W {
        SAR2_STANDBY_WAIT_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn sar2_rstb_wait(&mut self) -> SAR2_RSTB_WAIT_W {
        SAR2_RSTB_WAIT_W { w: self }
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn sar2_xpd_wait(&mut self) -> SAR2_XPD_WAIT_W {
        SAR2_XPD_WAIT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure rtc saradc2\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_meas2_ctrl1]
(index.html) module"]
pub struct SAR_MEAS2_CTRL1_SPEC;
impl crate::RegisterSpec for SAR_MEAS2_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_meas2_ctrl1::R]
(R) reader structure"]
impl crate::Readable for SAR_MEAS2_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_meas2_ctrl1::W]
(W) writer structure"]
impl crate::Writable for SAR_MEAS2_CTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAR_MEAS2_CTRL1 to value 0x0702_0200"]
impl crate::Resettable for SAR_MEAS2_CTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0702_0200
    }
}
