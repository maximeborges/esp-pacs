#[doc = "Register `SAR_MEAS1_MUX` reader"]
pub struct R(crate::R<SAR_MEAS1_MUX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_MEAS1_MUX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_MEAS1_MUX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_MEAS1_MUX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_MEAS1_MUX` writer"]
pub struct W(crate::W<SAR_MEAS1_MUX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_MEAS1_MUX_SPEC>;
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
impl From<crate::W<SAR_MEAS1_MUX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_MEAS1_MUX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAR1_DIG_FORCE` reader - 1: SAR ADC1 controlled by DIG ADC1 CTRL"]
pub struct SAR1_DIG_FORCE_R(crate::FieldReader<bool>);
impl SAR1_DIG_FORCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SAR1_DIG_FORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR1_DIG_FORCE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR1_DIG_FORCE` writer - 1: SAR ADC1 controlled by DIG ADC1 CTRL"]
pub struct SAR1_DIG_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR1_DIG_FORCE_W<'a> {
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
    #[doc = "Bit 31 - 1: SAR ADC1 controlled by DIG ADC1 CTRL"]
    #[inline(always)]
    pub fn sar1_dig_force(&self) -> SAR1_DIG_FORCE_R {
        SAR1_DIG_FORCE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - 1: SAR ADC1 controlled by DIG ADC1 CTRL"]
    #[inline(always)]
    pub fn sar1_dig_force(&mut self) -> SAR1_DIG_FORCE_W {
        SAR1_DIG_FORCE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Select the controller for SAR ADC1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_meas1_mux](index.html) module"]
pub struct SAR_MEAS1_MUX_SPEC;
impl crate::RegisterSpec for SAR_MEAS1_MUX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_meas1_mux::R](R) reader structure"]
impl crate::Readable for SAR_MEAS1_MUX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_meas1_mux::W](W) writer structure"]
impl crate::Writable for SAR_MEAS1_MUX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAR_MEAS1_MUX to value 0"]
impl crate::Resettable for SAR_MEAS1_MUX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
