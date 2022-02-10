#[doc = "Register `CALI` reader"]
pub struct R(crate::R<CALI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CALI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CALI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CALI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CALI` writer"]
pub struct W(crate::W<CALI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CALI_SPEC>;
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
impl From<crate::W<CALI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CALI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `APB_SARADC_CALI_CFG` reader - saradc cali factor"]
pub struct APB_SARADC_CALI_CFG_R(crate::FieldReader<u32, u32>);
impl APB_SARADC_CALI_CFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        APB_SARADC_CALI_CFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB_SARADC_CALI_CFG_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB_SARADC_CALI_CFG` writer - saradc cali factor"]
pub struct APB_SARADC_CALI_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_CALI_CFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0001_ffff) | (value as u32 & 0x0001_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:16 - saradc cali factor"]
    #[inline(always)]
    pub fn apb_saradc_cali_cfg(&self) -> APB_SARADC_CALI_CFG_R {
        APB_SARADC_CALI_CFG_R::new((self.bits & 0x0001_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:16 - saradc cali factor"]
    #[inline(always)]
    pub fn apb_saradc_cali_cfg(&mut self) -> APB_SARADC_CALI_CFG_W {
        APB_SARADC_CALI_CFG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "digital saradc configure register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cali]
(index.html) module"]
pub struct CALI_SPEC;
impl crate::RegisterSpec for CALI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cali::R]
(R) reader structure"]
impl crate::Readable for CALI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cali::W]
(W) writer structure"]
impl crate::Writable for CALI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CALI to value 0x8000"]
impl crate::Resettable for CALI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000
    }
}
