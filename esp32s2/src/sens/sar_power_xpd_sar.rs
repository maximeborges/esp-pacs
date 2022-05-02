#[doc = "Register `SAR_POWER_XPD_SAR` reader"]
pub struct R(crate::R<SAR_POWER_XPD_SAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_POWER_XPD_SAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_POWER_XPD_SAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_POWER_XPD_SAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_POWER_XPD_SAR` writer"]
pub struct W(crate::W<SAR_POWER_XPD_SAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_POWER_XPD_SAR_SPEC>;
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
impl From<crate::W<SAR_POWER_XPD_SAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_POWER_XPD_SAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FORCE_XPD_SAR` reader - "]
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
#[doc = "Field `FORCE_XPD_SAR` writer - "]
pub struct FORCE_XPD_SAR_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_XPD_SAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 29)) | ((value as u32 & 3) << 29);
        self.w
    }
}
#[doc = "Field `SARCLK_EN` reader - "]
pub struct SARCLK_EN_R(crate::FieldReader<bool>);
impl SARCLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SARCLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SARCLK_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SARCLK_EN` writer - "]
pub struct SARCLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SARCLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 31)) | ((value as u32 & 1) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 29:30"]
    #[inline(always)]
    pub fn force_xpd_sar(&self) -> FORCE_XPD_SAR_R {
        FORCE_XPD_SAR_R::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sarclk_en(&self) -> SARCLK_EN_R {
        SARCLK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 29:30"]
    #[inline(always)]
    pub fn force_xpd_sar(&mut self) -> FORCE_XPD_SAR_W {
        FORCE_XPD_SAR_W { w: self }
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sarclk_en(&mut self) -> SARCLK_EN_W {
        SARCLK_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure saradc’s power by sw\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_power_xpd_sar](index.html) module"]
pub struct SAR_POWER_XPD_SAR_SPEC;
impl crate::RegisterSpec for SAR_POWER_XPD_SAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_power_xpd_sar::R](R) reader structure"]
impl crate::Readable for SAR_POWER_XPD_SAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_power_xpd_sar::W](W) writer structure"]
impl crate::Writable for SAR_POWER_XPD_SAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAR_POWER_XPD_SAR to value 0"]
impl crate::Resettable for SAR_POWER_XPD_SAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
