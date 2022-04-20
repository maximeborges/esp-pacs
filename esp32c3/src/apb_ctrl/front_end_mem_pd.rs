#[doc = "Register `FRONT_END_MEM_PD` reader"]
pub struct R(crate::R<FRONT_END_MEM_PD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRONT_END_MEM_PD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRONT_END_MEM_PD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRONT_END_MEM_PD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRONT_END_MEM_PD` writer"]
pub struct W(crate::W<FRONT_END_MEM_PD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRONT_END_MEM_PD_SPEC>;
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
impl From<crate::W<FRONT_END_MEM_PD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRONT_END_MEM_PD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AGC_MEM_FORCE_PU` reader - reg_agc_mem_force_pu"]
pub struct AGC_MEM_FORCE_PU_R(crate::FieldReader<bool, bool>);
impl AGC_MEM_FORCE_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AGC_MEM_FORCE_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AGC_MEM_FORCE_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AGC_MEM_FORCE_PU` writer - reg_agc_mem_force_pu"]
pub struct AGC_MEM_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> AGC_MEM_FORCE_PU_W<'a> {
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
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
#[doc = "Field `AGC_MEM_FORCE_PD` reader - reg_agc_mem_force_pd"]
pub struct AGC_MEM_FORCE_PD_R(crate::FieldReader<bool, bool>);
impl AGC_MEM_FORCE_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AGC_MEM_FORCE_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AGC_MEM_FORCE_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AGC_MEM_FORCE_PD` writer - reg_agc_mem_force_pd"]
pub struct AGC_MEM_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> AGC_MEM_FORCE_PD_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u32 & 1) << 1);
        self.w
    }
}
#[doc = "Field `PBUS_MEM_FORCE_PU` reader - reg_pbus_mem_force_pu"]
pub struct PBUS_MEM_FORCE_PU_R(crate::FieldReader<bool, bool>);
impl PBUS_MEM_FORCE_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PBUS_MEM_FORCE_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PBUS_MEM_FORCE_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PBUS_MEM_FORCE_PU` writer - reg_pbus_mem_force_pu"]
pub struct PBUS_MEM_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> PBUS_MEM_FORCE_PU_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u32 & 1) << 2);
        self.w
    }
}
#[doc = "Field `PBUS_MEM_FORCE_PD` reader - reg_pbus_mem_force_pd"]
pub struct PBUS_MEM_FORCE_PD_R(crate::FieldReader<bool, bool>);
impl PBUS_MEM_FORCE_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PBUS_MEM_FORCE_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PBUS_MEM_FORCE_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PBUS_MEM_FORCE_PD` writer - reg_pbus_mem_force_pd"]
pub struct PBUS_MEM_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> PBUS_MEM_FORCE_PD_W<'a> {
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
#[doc = "Field `DC_MEM_FORCE_PU` reader - reg_dc_mem_force_pu"]
pub struct DC_MEM_FORCE_PU_R(crate::FieldReader<bool, bool>);
impl DC_MEM_FORCE_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DC_MEM_FORCE_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DC_MEM_FORCE_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DC_MEM_FORCE_PU` writer - reg_dc_mem_force_pu"]
pub struct DC_MEM_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> DC_MEM_FORCE_PU_W<'a> {
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
#[doc = "Field `DC_MEM_FORCE_PD` reader - reg_dc_mem_force_pd"]
pub struct DC_MEM_FORCE_PD_R(crate::FieldReader<bool, bool>);
impl DC_MEM_FORCE_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DC_MEM_FORCE_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DC_MEM_FORCE_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DC_MEM_FORCE_PD` writer - reg_dc_mem_force_pd"]
pub struct DC_MEM_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> DC_MEM_FORCE_PD_W<'a> {
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
impl R {
    #[doc = "Bit 0 - reg_agc_mem_force_pu"]
    #[inline(always)]
    pub fn agc_mem_force_pu(&self) -> AGC_MEM_FORCE_PU_R {
        AGC_MEM_FORCE_PU_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reg_agc_mem_force_pd"]
    #[inline(always)]
    pub fn agc_mem_force_pd(&self) -> AGC_MEM_FORCE_PD_R {
        AGC_MEM_FORCE_PD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - reg_pbus_mem_force_pu"]
    #[inline(always)]
    pub fn pbus_mem_force_pu(&self) -> PBUS_MEM_FORCE_PU_R {
        PBUS_MEM_FORCE_PU_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reg_pbus_mem_force_pd"]
    #[inline(always)]
    pub fn pbus_mem_force_pd(&self) -> PBUS_MEM_FORCE_PD_R {
        PBUS_MEM_FORCE_PD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - reg_dc_mem_force_pu"]
    #[inline(always)]
    pub fn dc_mem_force_pu(&self) -> DC_MEM_FORCE_PU_R {
        DC_MEM_FORCE_PU_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - reg_dc_mem_force_pd"]
    #[inline(always)]
    pub fn dc_mem_force_pd(&self) -> DC_MEM_FORCE_PD_R {
        DC_MEM_FORCE_PD_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - reg_agc_mem_force_pu"]
    #[inline(always)]
    pub fn agc_mem_force_pu(&mut self) -> AGC_MEM_FORCE_PU_W {
        AGC_MEM_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 1 - reg_agc_mem_force_pd"]
    #[inline(always)]
    pub fn agc_mem_force_pd(&mut self) -> AGC_MEM_FORCE_PD_W {
        AGC_MEM_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 2 - reg_pbus_mem_force_pu"]
    #[inline(always)]
    pub fn pbus_mem_force_pu(&mut self) -> PBUS_MEM_FORCE_PU_W {
        PBUS_MEM_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 3 - reg_pbus_mem_force_pd"]
    #[inline(always)]
    pub fn pbus_mem_force_pd(&mut self) -> PBUS_MEM_FORCE_PD_W {
        PBUS_MEM_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 4 - reg_dc_mem_force_pu"]
    #[inline(always)]
    pub fn dc_mem_force_pu(&mut self) -> DC_MEM_FORCE_PU_W {
        DC_MEM_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 5 - reg_dc_mem_force_pd"]
    #[inline(always)]
    pub fn dc_mem_force_pd(&mut self) -> DC_MEM_FORCE_PD_W {
        DC_MEM_FORCE_PD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB_CTRL_FRONT_END_MEM_PD_REG\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [front_end_mem_pd]
(index.html) module"]
pub struct FRONT_END_MEM_PD_SPEC;
impl crate::RegisterSpec for FRONT_END_MEM_PD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [front_end_mem_pd::R]
(R) reader structure"]
impl crate::Readable for FRONT_END_MEM_PD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [front_end_mem_pd::W]
(W) writer structure"]
impl crate::Writable for FRONT_END_MEM_PD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FRONT_END_MEM_PD to value 0x15"]
impl crate::Resettable for FRONT_END_MEM_PD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x15
    }
}
