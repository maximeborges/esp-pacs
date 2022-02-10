#[doc = "Register `CACHE_MMU_POWER_CTRL` reader"]
pub struct R(crate::R<CACHE_MMU_POWER_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACHE_MMU_POWER_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACHE_MMU_POWER_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACHE_MMU_POWER_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CACHE_MMU_POWER_CTRL` writer"]
pub struct W(crate::W<CACHE_MMU_POWER_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CACHE_MMU_POWER_CTRL_SPEC>;
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
impl From<crate::W<CACHE_MMU_POWER_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CACHE_MMU_POWER_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CACHE_MMU_MEM_FORCE_ON` reader - The bit is used to enable clock gating to save power when access mmu memory, 0: enable, 1: disable"]
pub struct CACHE_MMU_MEM_FORCE_ON_R(crate::FieldReader<bool, bool>);
impl CACHE_MMU_MEM_FORCE_ON_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CACHE_MMU_MEM_FORCE_ON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CACHE_MMU_MEM_FORCE_ON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CACHE_MMU_MEM_FORCE_ON` writer - The bit is used to enable clock gating to save power when access mmu memory, 0: enable, 1: disable"]
pub struct CACHE_MMU_MEM_FORCE_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHE_MMU_MEM_FORCE_ON_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `CACHE_MMU_MEM_FORCE_PD` reader - The bit is used to power mmu memory down, 0: follow_rtc_lslp_pd, 1: power down"]
pub struct CACHE_MMU_MEM_FORCE_PD_R(crate::FieldReader<bool, bool>);
impl CACHE_MMU_MEM_FORCE_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CACHE_MMU_MEM_FORCE_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CACHE_MMU_MEM_FORCE_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CACHE_MMU_MEM_FORCE_PD` writer - The bit is used to power mmu memory down, 0: follow_rtc_lslp_pd, 1: power down"]
pub struct CACHE_MMU_MEM_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHE_MMU_MEM_FORCE_PD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `CACHE_MMU_MEM_FORCE_PU` reader - The bit is used to power mmu memory down, 0: follow_rtc_lslp_pd, 1: power up"]
pub struct CACHE_MMU_MEM_FORCE_PU_R(crate::FieldReader<bool, bool>);
impl CACHE_MMU_MEM_FORCE_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CACHE_MMU_MEM_FORCE_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CACHE_MMU_MEM_FORCE_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CACHE_MMU_MEM_FORCE_PU` writer - The bit is used to power mmu memory down, 0: follow_rtc_lslp_pd, 1: power up"]
pub struct CACHE_MMU_MEM_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHE_MMU_MEM_FORCE_PU_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - The bit is used to enable clock gating to save power when access mmu memory, 0: enable, 1: disable"]
    #[inline(always)]
    pub fn cache_mmu_mem_force_on(&self) -> CACHE_MMU_MEM_FORCE_ON_R {
        CACHE_MMU_MEM_FORCE_ON_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - The bit is used to power mmu memory down, 0: follow_rtc_lslp_pd, 1: power down"]
    #[inline(always)]
    pub fn cache_mmu_mem_force_pd(&self) -> CACHE_MMU_MEM_FORCE_PD_R {
        CACHE_MMU_MEM_FORCE_PD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - The bit is used to power mmu memory down, 0: follow_rtc_lslp_pd, 1: power up"]
    #[inline(always)]
    pub fn cache_mmu_mem_force_pu(&self) -> CACHE_MMU_MEM_FORCE_PU_R {
        CACHE_MMU_MEM_FORCE_PU_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to enable clock gating to save power when access mmu memory, 0: enable, 1: disable"]
    #[inline(always)]
    pub fn cache_mmu_mem_force_on(&mut self) -> CACHE_MMU_MEM_FORCE_ON_W {
        CACHE_MMU_MEM_FORCE_ON_W { w: self }
    }
    #[doc = "Bit 1 - The bit is used to power mmu memory down, 0: follow_rtc_lslp_pd, 1: power down"]
    #[inline(always)]
    pub fn cache_mmu_mem_force_pd(&mut self) -> CACHE_MMU_MEM_FORCE_PD_W {
        CACHE_MMU_MEM_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 2 - The bit is used to power mmu memory down, 0: follow_rtc_lslp_pd, 1: power up"]
    #[inline(always)]
    pub fn cache_mmu_mem_force_pu(&mut self) -> CACHE_MMU_MEM_FORCE_PU_W {
        CACHE_MMU_MEM_FORCE_PU_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This description will be updated in the near future.\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cache_mmu_power_ctrl]
(index.html) module"]
pub struct CACHE_MMU_POWER_CTRL_SPEC;
impl crate::RegisterSpec for CACHE_MMU_POWER_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cache_mmu_power_ctrl::R]
(R) reader structure"]
impl crate::Readable for CACHE_MMU_POWER_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cache_mmu_power_ctrl::W]
(W) writer structure"]
impl crate::Writable for CACHE_MMU_POWER_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CACHE_MMU_POWER_CTRL to value 0x05"]
impl crate::Resettable for CACHE_MMU_POWER_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x05
    }
}
