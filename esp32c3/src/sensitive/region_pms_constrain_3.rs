#[doc = "Register `REGION_PMS_CONSTRAIN_3` reader"]
pub struct R(crate::R<REGION_PMS_CONSTRAIN_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REGION_PMS_CONSTRAIN_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REGION_PMS_CONSTRAIN_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REGION_PMS_CONSTRAIN_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REGION_PMS_CONSTRAIN_3` writer"]
pub struct W(crate::W<REGION_PMS_CONSTRAIN_3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REGION_PMS_CONSTRAIN_3_SPEC>;
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
impl From<crate::W<REGION_PMS_CONSTRAIN_3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REGION_PMS_CONSTRAIN_3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REGION_PMS_CONSTRAIN_ADDR_0` reader - region_pms_constrain_addr_0"]
pub struct REGION_PMS_CONSTRAIN_ADDR_0_R(crate::FieldReader<u32>);
impl REGION_PMS_CONSTRAIN_ADDR_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        REGION_PMS_CONSTRAIN_ADDR_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REGION_PMS_CONSTRAIN_ADDR_0_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION_PMS_CONSTRAIN_ADDR_0` writer - region_pms_constrain_addr_0"]
pub struct REGION_PMS_CONSTRAIN_ADDR_0_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION_PMS_CONSTRAIN_ADDR_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff_ffff) | (value as u32 & 0x3fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:29 - region_pms_constrain_addr_0"]
    #[inline(always)]
    pub fn region_pms_constrain_addr_0(&self) -> REGION_PMS_CONSTRAIN_ADDR_0_R {
        REGION_PMS_CONSTRAIN_ADDR_0_R::new((self.bits & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:29 - region_pms_constrain_addr_0"]
    #[inline(always)]
    pub fn region_pms_constrain_addr_0(&mut self) -> REGION_PMS_CONSTRAIN_ADDR_0_W {
        REGION_PMS_CONSTRAIN_ADDR_0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SENSITIVE_REGION_PMS_CONSTRAIN_3_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [region_pms_constrain_3](index.html) module"]
pub struct REGION_PMS_CONSTRAIN_3_SPEC;
impl crate::RegisterSpec for REGION_PMS_CONSTRAIN_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [region_pms_constrain_3::R](R) reader structure"]
impl crate::Readable for REGION_PMS_CONSTRAIN_3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [region_pms_constrain_3::W](W) writer structure"]
impl crate::Writable for REGION_PMS_CONSTRAIN_3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REGION_PMS_CONSTRAIN_3 to value 0"]
impl crate::Resettable for REGION_PMS_CONSTRAIN_3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
