#[doc = "Register `CACHE_SOURCE_1` reader"]
pub struct R(crate::R<CACHE_SOURCE_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACHE_SOURCE_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACHE_SOURCE_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACHE_SOURCE_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CACHE_SOURCE_1` writer"]
pub struct W(crate::W<CACHE_SOURCE_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CACHE_SOURCE_1_SPEC>;
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
impl From<crate::W<CACHE_SOURCE_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CACHE_SOURCE_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRO_CACHE_I_SOURCE_PRO_IRAM1` reader - xx"]
pub struct PRO_CACHE_I_SOURCE_PRO_IRAM1_R(crate::FieldReader<bool, bool>);
impl PRO_CACHE_I_SOURCE_PRO_IRAM1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_CACHE_I_SOURCE_PRO_IRAM1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_CACHE_I_SOURCE_PRO_IRAM1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_CACHE_I_SOURCE_PRO_IRAM1` writer - xx"]
pub struct PRO_CACHE_I_SOURCE_PRO_IRAM1_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_CACHE_I_SOURCE_PRO_IRAM1_W<'a> {
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
#[doc = "Field `PRO_CACHE_I_SOURCE_PRO_IROM0` reader - xx"]
pub struct PRO_CACHE_I_SOURCE_PRO_IROM0_R(crate::FieldReader<bool, bool>);
impl PRO_CACHE_I_SOURCE_PRO_IROM0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_CACHE_I_SOURCE_PRO_IROM0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_CACHE_I_SOURCE_PRO_IROM0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_CACHE_I_SOURCE_PRO_IROM0` writer - xx"]
pub struct PRO_CACHE_I_SOURCE_PRO_IROM0_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_CACHE_I_SOURCE_PRO_IROM0_W<'a> {
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
#[doc = "Field `PRO_CACHE_I_SOURCE_PRO_DROM0` reader - xx"]
pub struct PRO_CACHE_I_SOURCE_PRO_DROM0_R(crate::FieldReader<bool, bool>);
impl PRO_CACHE_I_SOURCE_PRO_DROM0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_CACHE_I_SOURCE_PRO_DROM0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_CACHE_I_SOURCE_PRO_DROM0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_CACHE_I_SOURCE_PRO_DROM0` writer - xx"]
pub struct PRO_CACHE_I_SOURCE_PRO_DROM0_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_CACHE_I_SOURCE_PRO_DROM0_W<'a> {
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
#[doc = "Field `PRO_CACHE_D_SOURCE_PRO_DRAM0` reader - xx"]
pub struct PRO_CACHE_D_SOURCE_PRO_DRAM0_R(crate::FieldReader<bool, bool>);
impl PRO_CACHE_D_SOURCE_PRO_DRAM0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_CACHE_D_SOURCE_PRO_DRAM0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_CACHE_D_SOURCE_PRO_DRAM0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_CACHE_D_SOURCE_PRO_DRAM0` writer - xx"]
pub struct PRO_CACHE_D_SOURCE_PRO_DRAM0_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_CACHE_D_SOURCE_PRO_DRAM0_W<'a> {
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
#[doc = "Field `PRO_CACHE_D_SOURCE_PRO_DPORT` reader - xx"]
pub struct PRO_CACHE_D_SOURCE_PRO_DPORT_R(crate::FieldReader<bool, bool>);
impl PRO_CACHE_D_SOURCE_PRO_DPORT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_CACHE_D_SOURCE_PRO_DPORT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_CACHE_D_SOURCE_PRO_DPORT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_CACHE_D_SOURCE_PRO_DPORT` writer - xx"]
pub struct PRO_CACHE_D_SOURCE_PRO_DPORT_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_CACHE_D_SOURCE_PRO_DPORT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `PRO_CACHE_D_SOURCE_PRO_DROM0` reader - xx"]
pub struct PRO_CACHE_D_SOURCE_PRO_DROM0_R(crate::FieldReader<bool, bool>);
impl PRO_CACHE_D_SOURCE_PRO_DROM0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_CACHE_D_SOURCE_PRO_DROM0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_CACHE_D_SOURCE_PRO_DROM0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_CACHE_D_SOURCE_PRO_DROM0` writer - xx"]
pub struct PRO_CACHE_D_SOURCE_PRO_DROM0_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_CACHE_D_SOURCE_PRO_DROM0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - xx"]
    #[inline(always)]
    pub fn pro_cache_i_source_pro_iram1(&self) -> PRO_CACHE_I_SOURCE_PRO_IRAM1_R {
        PRO_CACHE_I_SOURCE_PRO_IRAM1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - xx"]
    #[inline(always)]
    pub fn pro_cache_i_source_pro_irom0(&self) -> PRO_CACHE_I_SOURCE_PRO_IROM0_R {
        PRO_CACHE_I_SOURCE_PRO_IROM0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - xx"]
    #[inline(always)]
    pub fn pro_cache_i_source_pro_drom0(&self) -> PRO_CACHE_I_SOURCE_PRO_DROM0_R {
        PRO_CACHE_I_SOURCE_PRO_DROM0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - xx"]
    #[inline(always)]
    pub fn pro_cache_d_source_pro_dram0(&self) -> PRO_CACHE_D_SOURCE_PRO_DRAM0_R {
        PRO_CACHE_D_SOURCE_PRO_DRAM0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - xx"]
    #[inline(always)]
    pub fn pro_cache_d_source_pro_dport(&self) -> PRO_CACHE_D_SOURCE_PRO_DPORT_R {
        PRO_CACHE_D_SOURCE_PRO_DPORT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - xx"]
    #[inline(always)]
    pub fn pro_cache_d_source_pro_drom0(&self) -> PRO_CACHE_D_SOURCE_PRO_DROM0_R {
        PRO_CACHE_D_SOURCE_PRO_DROM0_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - xx"]
    #[inline(always)]
    pub fn pro_cache_i_source_pro_iram1(&mut self) -> PRO_CACHE_I_SOURCE_PRO_IRAM1_W {
        PRO_CACHE_I_SOURCE_PRO_IRAM1_W { w: self }
    }
    #[doc = "Bit 1 - xx"]
    #[inline(always)]
    pub fn pro_cache_i_source_pro_irom0(&mut self) -> PRO_CACHE_I_SOURCE_PRO_IROM0_W {
        PRO_CACHE_I_SOURCE_PRO_IROM0_W { w: self }
    }
    #[doc = "Bit 2 - xx"]
    #[inline(always)]
    pub fn pro_cache_i_source_pro_drom0(&mut self) -> PRO_CACHE_I_SOURCE_PRO_DROM0_W {
        PRO_CACHE_I_SOURCE_PRO_DROM0_W { w: self }
    }
    #[doc = "Bit 3 - xx"]
    #[inline(always)]
    pub fn pro_cache_d_source_pro_dram0(&mut self) -> PRO_CACHE_D_SOURCE_PRO_DRAM0_W {
        PRO_CACHE_D_SOURCE_PRO_DRAM0_W { w: self }
    }
    #[doc = "Bit 4 - xx"]
    #[inline(always)]
    pub fn pro_cache_d_source_pro_dport(&mut self) -> PRO_CACHE_D_SOURCE_PRO_DPORT_W {
        PRO_CACHE_D_SOURCE_PRO_DPORT_W { w: self }
    }
    #[doc = "Bit 5 - xx"]
    #[inline(always)]
    pub fn pro_cache_d_source_pro_drom0(&mut self) -> PRO_CACHE_D_SOURCE_PRO_DROM0_W {
        PRO_CACHE_D_SOURCE_PRO_DROM0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cache access permission control register 1.\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cache_source_1]
(index.html) module"]
pub struct CACHE_SOURCE_1_SPEC;
impl crate::RegisterSpec for CACHE_SOURCE_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cache_source_1::R]
(R) reader structure"]
impl crate::Readable for CACHE_SOURCE_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cache_source_1::W]
(W) writer structure"]
impl crate::Writable for CACHE_SOURCE_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CACHE_SOURCE_1 to value 0"]
impl crate::Resettable for CACHE_SOURCE_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
