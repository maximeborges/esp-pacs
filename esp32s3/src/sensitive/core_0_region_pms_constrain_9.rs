#[doc = "Register `CORE_0_REGION_PMS_CONSTRAIN_9` reader"]
pub struct R(crate::R<CORE_0_REGION_PMS_CONSTRAIN_9_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_REGION_PMS_CONSTRAIN_9_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_REGION_PMS_CONSTRAIN_9_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_REGION_PMS_CONSTRAIN_9_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_0_REGION_PMS_CONSTRAIN_9` writer"]
pub struct W(crate::W<CORE_0_REGION_PMS_CONSTRAIN_9_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_0_REGION_PMS_CONSTRAIN_9_SPEC>;
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
impl From<crate::W<CORE_0_REGION_PMS_CONSTRAIN_9_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_0_REGION_PMS_CONSTRAIN_9_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_0_REGION_PMS_CONSTRAIN_ADDR_6` reader - Region 5 end address and Region 6 start address for core0."]
pub type CORE_0_REGION_PMS_CONSTRAIN_ADDR_6_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CORE_0_REGION_PMS_CONSTRAIN_ADDR_6` writer - Region 5 end address and Region 6 start address for core0."]
pub type CORE_0_REGION_PMS_CONSTRAIN_ADDR_6_W<'a> =
    crate::FieldWriter<'a, u32, CORE_0_REGION_PMS_CONSTRAIN_9_SPEC, u32, u32, 30, 0>;
impl R {
    #[doc = "Bits 0:29 - Region 5 end address and Region 6 start address for core0."]
    #[inline(always)]
    pub fn core_0_region_pms_constrain_addr_6(&self) -> CORE_0_REGION_PMS_CONSTRAIN_ADDR_6_R {
        CORE_0_REGION_PMS_CONSTRAIN_ADDR_6_R::new((self.bits & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:29 - Region 5 end address and Region 6 start address for core0."]
    #[inline(always)]
    pub fn core_0_region_pms_constrain_addr_6(&mut self) -> CORE_0_REGION_PMS_CONSTRAIN_ADDR_6_W {
        CORE_0_REGION_PMS_CONSTRAIN_ADDR_6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Core0 region permission register 9.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_region_pms_constrain_9](index.html) module"]
pub struct CORE_0_REGION_PMS_CONSTRAIN_9_SPEC;
impl crate::RegisterSpec for CORE_0_REGION_PMS_CONSTRAIN_9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_region_pms_constrain_9::R](R) reader structure"]
impl crate::Readable for CORE_0_REGION_PMS_CONSTRAIN_9_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_0_region_pms_constrain_9::W](W) writer structure"]
impl crate::Writable for CORE_0_REGION_PMS_CONSTRAIN_9_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CORE_0_REGION_PMS_CONSTRAIN_9 to value 0"]
impl crate::Resettable for CORE_0_REGION_PMS_CONSTRAIN_9_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
