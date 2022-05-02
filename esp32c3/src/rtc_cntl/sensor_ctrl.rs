#[doc = "Register `SENSOR_CTRL` reader"]
pub struct R(crate::R<SENSOR_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SENSOR_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SENSOR_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SENSOR_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SENSOR_CTRL` writer"]
pub struct W(crate::W<SENSOR_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SENSOR_CTRL_SPEC>;
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
impl From<crate::W<SENSOR_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SENSOR_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAR2_PWDET_CCT` reader - reg_sar2_pwdet_cct"]
pub struct SAR2_PWDET_CCT_R(crate::FieldReader<u8>);
impl SAR2_PWDET_CCT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SAR2_PWDET_CCT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR2_PWDET_CCT_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR2_PWDET_CCT` writer - reg_sar2_pwdet_cct"]
pub struct SAR2_PWDET_CCT_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR2_PWDET_CCT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 27)) | ((value as u32 & 7) << 27);
        self.w
    }
}
#[doc = "Field `FORCE_XPD_SAR` reader - force power up SAR"]
pub struct FORCE_XPD_SAR_R(crate::FieldReader<u8>);
impl FORCE_XPD_SAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FORCE_XPD_SAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FORCE_XPD_SAR_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FORCE_XPD_SAR` writer - force power up SAR"]
pub struct FORCE_XPD_SAR_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_XPD_SAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 30)) | ((value as u32 & 3) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 27:29 - reg_sar2_pwdet_cct"]
    #[inline(always)]
    pub fn sar2_pwdet_cct(&self) -> SAR2_PWDET_CCT_R {
        SAR2_PWDET_CCT_R::new(((self.bits >> 27) & 7) as u8)
    }
    #[doc = "Bits 30:31 - force power up SAR"]
    #[inline(always)]
    pub fn force_xpd_sar(&self) -> FORCE_XPD_SAR_R {
        FORCE_XPD_SAR_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 27:29 - reg_sar2_pwdet_cct"]
    #[inline(always)]
    pub fn sar2_pwdet_cct(&mut self) -> SAR2_PWDET_CCT_W {
        SAR2_PWDET_CCT_W { w: self }
    }
    #[doc = "Bits 30:31 - force power up SAR"]
    #[inline(always)]
    pub fn force_xpd_sar(&mut self) -> FORCE_XPD_SAR_W {
        FORCE_XPD_SAR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rtc configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensor_ctrl](index.html) module"]
pub struct SENSOR_CTRL_SPEC;
impl crate::RegisterSpec for SENSOR_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sensor_ctrl::R](R) reader structure"]
impl crate::Readable for SENSOR_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sensor_ctrl::W](W) writer structure"]
impl crate::Writable for SENSOR_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SENSOR_CTRL to value 0"]
impl crate::Resettable for SENSOR_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
