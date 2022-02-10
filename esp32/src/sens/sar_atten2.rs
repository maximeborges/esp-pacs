#[doc = "Register `SAR_ATTEN2` reader"]
pub struct R(crate::R<SAR_ATTEN2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_ATTEN2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_ATTEN2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_ATTEN2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_ATTEN2` writer"]
pub struct W(crate::W<SAR_ATTEN2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_ATTEN2_SPEC>;
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
impl From<crate::W<SAR_ATTEN2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_ATTEN2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAR2_ATTEN` reader - 2-bit attenuation for each pad 11:1dB 10:6dB 01:3dB 00:0dB"]
pub struct SAR2_ATTEN_R(crate::FieldReader<u32, u32>);
impl SAR2_ATTEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        SAR2_ATTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR2_ATTEN_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR2_ATTEN` writer - 2-bit attenuation for each pad 11:1dB 10:6dB 01:3dB 00:0dB"]
pub struct SAR2_ATTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR2_ATTEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 2-bit attenuation for each pad 11:1dB 10:6dB 01:3dB 00:0dB"]
    #[inline(always)]
    pub fn sar2_atten(&self) -> SAR2_ATTEN_R {
        SAR2_ATTEN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 2-bit attenuation for each pad 11:1dB 10:6dB 01:3dB 00:0dB"]
    #[inline(always)]
    pub fn sar2_atten(&mut self) -> SAR2_ATTEN_W {
        SAR2_ATTEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_atten2]
(index.html) module"]
pub struct SAR_ATTEN2_SPEC;
impl crate::RegisterSpec for SAR_ATTEN2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_atten2::R]
(R) reader structure"]
impl crate::Readable for SAR_ATTEN2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_atten2::W]
(W) writer structure"]
impl crate::Writable for SAR_ATTEN2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAR_ATTEN2 to value 0xffff_ffff"]
impl crate::Resettable for SAR_ATTEN2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
