#[doc = "Register `THRES_CTRL` reader"]
pub struct R(crate::R<THRES_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<THRES_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<THRES_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<THRES_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `THRES_CTRL` writer"]
pub struct W(crate::W<THRES_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<THRES_CTRL_SPEC>;
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
impl From<crate::W<THRES_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<THRES_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `THRES_ALL_EN` reader - enable thres0 to monitor all channel"]
pub struct THRES_ALL_EN_R(crate::FieldReader<bool, bool>);
impl THRES_ALL_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        THRES_ALL_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THRES_ALL_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THRES_ALL_EN` writer - enable thres0 to monitor all channel"]
pub struct THRES_ALL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> THRES_ALL_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `THRES3_EN` reader - no public"]
pub struct THRES3_EN_R(crate::FieldReader<bool, bool>);
impl THRES3_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        THRES3_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THRES3_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THRES3_EN` writer - no public"]
pub struct THRES3_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> THRES3_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `THRES2_EN` reader - no public"]
pub struct THRES2_EN_R(crate::FieldReader<bool, bool>);
impl THRES2_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        THRES2_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THRES2_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THRES2_EN` writer - no public"]
pub struct THRES2_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> THRES2_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `THRES1_EN` reader - enable thres1"]
pub struct THRES1_EN_R(crate::FieldReader<bool, bool>);
impl THRES1_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        THRES1_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THRES1_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THRES1_EN` writer - enable thres1"]
pub struct THRES1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> THRES1_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `THRES0_EN` reader - enable thres0"]
pub struct THRES0_EN_R(crate::FieldReader<bool, bool>);
impl THRES0_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        THRES0_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THRES0_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THRES0_EN` writer - enable thres0"]
pub struct THRES0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> THRES0_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 27 - enable thres0 to monitor all channel"]
    #[inline(always)]
    pub fn thres_all_en(&self) -> THRES_ALL_EN_R {
        THRES_ALL_EN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - no public"]
    #[inline(always)]
    pub fn thres3_en(&self) -> THRES3_EN_R {
        THRES3_EN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - no public"]
    #[inline(always)]
    pub fn thres2_en(&self) -> THRES2_EN_R {
        THRES2_EN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - enable thres1"]
    #[inline(always)]
    pub fn thres1_en(&self) -> THRES1_EN_R {
        THRES1_EN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - enable thres0"]
    #[inline(always)]
    pub fn thres0_en(&self) -> THRES0_EN_R {
        THRES0_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 27 - enable thres0 to monitor all channel"]
    #[inline(always)]
    pub fn thres_all_en(&mut self) -> THRES_ALL_EN_W {
        THRES_ALL_EN_W { w: self }
    }
    #[doc = "Bit 28 - no public"]
    #[inline(always)]
    pub fn thres3_en(&mut self) -> THRES3_EN_W {
        THRES3_EN_W { w: self }
    }
    #[doc = "Bit 29 - no public"]
    #[inline(always)]
    pub fn thres2_en(&mut self) -> THRES2_EN_W {
        THRES2_EN_W { w: self }
    }
    #[doc = "Bit 30 - enable thres1"]
    #[inline(always)]
    pub fn thres1_en(&mut self) -> THRES1_EN_W {
        THRES1_EN_W { w: self }
    }
    #[doc = "Bit 31 - enable thres0"]
    #[inline(always)]
    pub fn thres0_en(&mut self) -> THRES0_EN_W {
        THRES0_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure thres monitor enable\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [thres_ctrl]
(index.html) module"]
pub struct THRES_CTRL_SPEC;
impl crate::RegisterSpec for THRES_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [thres_ctrl::R]
(R) reader structure"]
impl crate::Readable for THRES_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [thres_ctrl::W]
(W) writer structure"]
impl crate::Writable for THRES_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets THRES_CTRL to value 0"]
impl crate::Resettable for THRES_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
