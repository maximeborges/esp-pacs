#[doc = "Register `CACHE_TAG_ACCESS_1` reader"]
pub struct R(crate::R<CACHE_TAG_ACCESS_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACHE_TAG_ACCESS_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACHE_TAG_ACCESS_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACHE_TAG_ACCESS_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CACHE_TAG_ACCESS_1` writer"]
pub struct W(crate::W<CACHE_TAG_ACCESS_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CACHE_TAG_ACCESS_1_SPEC>;
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
impl From<crate::W<CACHE_TAG_ACCESS_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CACHE_TAG_ACCESS_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRO_I_TAG_RD_ACS` reader - Setting to 1 permits read access to Icache tag memory."]
pub struct PRO_I_TAG_RD_ACS_R(crate::FieldReader<bool, bool>);
impl PRO_I_TAG_RD_ACS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_I_TAG_RD_ACS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_I_TAG_RD_ACS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_I_TAG_RD_ACS` writer - Setting to 1 permits read access to Icache tag memory."]
pub struct PRO_I_TAG_RD_ACS_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_I_TAG_RD_ACS_W<'a> {
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
#[doc = "Field `PRO_I_TAG_WR_ACS` reader - Setting to 1 permits write access to Icache tag memory."]
pub struct PRO_I_TAG_WR_ACS_R(crate::FieldReader<bool, bool>);
impl PRO_I_TAG_WR_ACS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_I_TAG_WR_ACS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_I_TAG_WR_ACS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_I_TAG_WR_ACS` writer - Setting to 1 permits write access to Icache tag memory."]
pub struct PRO_I_TAG_WR_ACS_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_I_TAG_WR_ACS_W<'a> {
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
#[doc = "Field `PRO_D_TAG_RD_ACS` reader - Setting to 1 permits read access to Dcache tag memory."]
pub struct PRO_D_TAG_RD_ACS_R(crate::FieldReader<bool, bool>);
impl PRO_D_TAG_RD_ACS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_D_TAG_RD_ACS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_D_TAG_RD_ACS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_D_TAG_RD_ACS` writer - Setting to 1 permits read access to Dcache tag memory."]
pub struct PRO_D_TAG_RD_ACS_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_D_TAG_RD_ACS_W<'a> {
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
#[doc = "Field `PRO_D_TAG_WR_ACS` reader - Setting to 1 permits write access to Dcache tag memory."]
pub struct PRO_D_TAG_WR_ACS_R(crate::FieldReader<bool, bool>);
impl PRO_D_TAG_WR_ACS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_D_TAG_WR_ACS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_D_TAG_WR_ACS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_D_TAG_WR_ACS` writer - Setting to 1 permits write access to Dcache tag memory."]
pub struct PRO_D_TAG_WR_ACS_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_D_TAG_WR_ACS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Setting to 1 permits read access to Icache tag memory."]
    #[inline(always)]
    pub fn pro_i_tag_rd_acs(&self) -> PRO_I_TAG_RD_ACS_R {
        PRO_I_TAG_RD_ACS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Setting to 1 permits write access to Icache tag memory."]
    #[inline(always)]
    pub fn pro_i_tag_wr_acs(&self) -> PRO_I_TAG_WR_ACS_R {
        PRO_I_TAG_WR_ACS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Setting to 1 permits read access to Dcache tag memory."]
    #[inline(always)]
    pub fn pro_d_tag_rd_acs(&self) -> PRO_D_TAG_RD_ACS_R {
        PRO_D_TAG_RD_ACS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Setting to 1 permits write access to Dcache tag memory."]
    #[inline(always)]
    pub fn pro_d_tag_wr_acs(&self) -> PRO_D_TAG_WR_ACS_R {
        PRO_D_TAG_WR_ACS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Setting to 1 permits read access to Icache tag memory."]
    #[inline(always)]
    pub fn pro_i_tag_rd_acs(&mut self) -> PRO_I_TAG_RD_ACS_W {
        PRO_I_TAG_RD_ACS_W { w: self }
    }
    #[doc = "Bit 1 - Setting to 1 permits write access to Icache tag memory."]
    #[inline(always)]
    pub fn pro_i_tag_wr_acs(&mut self) -> PRO_I_TAG_WR_ACS_W {
        PRO_I_TAG_WR_ACS_W { w: self }
    }
    #[doc = "Bit 2 - Setting to 1 permits read access to Dcache tag memory."]
    #[inline(always)]
    pub fn pro_d_tag_rd_acs(&mut self) -> PRO_D_TAG_RD_ACS_W {
        PRO_D_TAG_RD_ACS_W { w: self }
    }
    #[doc = "Bit 3 - Setting to 1 permits write access to Dcache tag memory."]
    #[inline(always)]
    pub fn pro_d_tag_wr_acs(&mut self) -> PRO_D_TAG_WR_ACS_W {
        PRO_D_TAG_WR_ACS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cache tag permission control register 1.\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cache_tag_access_1]
(index.html) module"]
pub struct CACHE_TAG_ACCESS_1_SPEC;
impl crate::RegisterSpec for CACHE_TAG_ACCESS_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cache_tag_access_1::R]
(R) reader structure"]
impl crate::Readable for CACHE_TAG_ACCESS_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cache_tag_access_1::W]
(W) writer structure"]
impl crate::Writable for CACHE_TAG_ACCESS_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CACHE_TAG_ACCESS_1 to value 0"]
impl crate::Resettable for CACHE_TAG_ACCESS_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
