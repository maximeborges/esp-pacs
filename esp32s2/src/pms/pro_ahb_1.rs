#[doc = "Register `PRO_AHB_1` reader"]
pub struct R(crate::R<PRO_AHB_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_AHB_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_AHB_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_AHB_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRO_AHB_1` writer"]
pub struct W(crate::W<PRO_AHB_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRO_AHB_1_SPEC>;
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
impl From<crate::W<PRO_AHB_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRO_AHB_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRO_AHB_RTCSLOW_0_SPLTADDR` reader - Configure the split address of RTCSlow_0 for PeriBus2 access."]
pub struct PRO_AHB_RTCSLOW_0_SPLTADDR_R(crate::FieldReader<u16, u16>);
impl PRO_AHB_RTCSLOW_0_SPLTADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        PRO_AHB_RTCSLOW_0_SPLTADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_AHB_RTCSLOW_0_SPLTADDR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_AHB_RTCSLOW_0_SPLTADDR` writer - Configure the split address of RTCSlow_0 for PeriBus2 access."]
pub struct PRO_AHB_RTCSLOW_0_SPLTADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_AHB_RTCSLOW_0_SPLTADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | (value as u32 & 0x07ff);
        self.w
    }
}
#[doc = "Field `PRO_AHB_RTCSLOW_0_L_F` reader - Setting to 1 grants PeriBus2 permission to fetch RTCSlow_0 low address region."]
pub struct PRO_AHB_RTCSLOW_0_L_F_R(crate::FieldReader<bool, bool>);
impl PRO_AHB_RTCSLOW_0_L_F_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_AHB_RTCSLOW_0_L_F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_AHB_RTCSLOW_0_L_F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_AHB_RTCSLOW_0_L_F` writer - Setting to 1 grants PeriBus2 permission to fetch RTCSlow_0 low address region."]
pub struct PRO_AHB_RTCSLOW_0_L_F_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_AHB_RTCSLOW_0_L_F_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `PRO_AHB_RTCSLOW_0_L_R` reader - Setting to 1 grants PeriBus2 permission to read RTCSlow_0 low address region."]
pub struct PRO_AHB_RTCSLOW_0_L_R_R(crate::FieldReader<bool, bool>);
impl PRO_AHB_RTCSLOW_0_L_R_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_AHB_RTCSLOW_0_L_R_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_AHB_RTCSLOW_0_L_R_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_AHB_RTCSLOW_0_L_R` writer - Setting to 1 grants PeriBus2 permission to read RTCSlow_0 low address region."]
pub struct PRO_AHB_RTCSLOW_0_L_R_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_AHB_RTCSLOW_0_L_R_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `PRO_AHB_RTCSLOW_0_L_W` reader - Setting to 1 grants PeriBus2 permission to write RTCSlow_0 low address region."]
pub struct PRO_AHB_RTCSLOW_0_L_W_R(crate::FieldReader<bool, bool>);
impl PRO_AHB_RTCSLOW_0_L_W_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_AHB_RTCSLOW_0_L_W_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_AHB_RTCSLOW_0_L_W_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_AHB_RTCSLOW_0_L_W` writer - Setting to 1 grants PeriBus2 permission to write RTCSlow_0 low address region."]
pub struct PRO_AHB_RTCSLOW_0_L_W_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_AHB_RTCSLOW_0_L_W_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `PRO_AHB_RTCSLOW_0_H_F` reader - Setting to 1 grants PeriBus2 permission to fetch RTCSlow_0 high address region."]
pub struct PRO_AHB_RTCSLOW_0_H_F_R(crate::FieldReader<bool, bool>);
impl PRO_AHB_RTCSLOW_0_H_F_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_AHB_RTCSLOW_0_H_F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_AHB_RTCSLOW_0_H_F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_AHB_RTCSLOW_0_H_F` writer - Setting to 1 grants PeriBus2 permission to fetch RTCSlow_0 high address region."]
pub struct PRO_AHB_RTCSLOW_0_H_F_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_AHB_RTCSLOW_0_H_F_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `PRO_AHB_RTCSLOW_0_H_R` reader - Setting to 1 grants PeriBus2 permission to read RTCSlow_0 high address region."]
pub struct PRO_AHB_RTCSLOW_0_H_R_R(crate::FieldReader<bool, bool>);
impl PRO_AHB_RTCSLOW_0_H_R_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_AHB_RTCSLOW_0_H_R_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_AHB_RTCSLOW_0_H_R_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_AHB_RTCSLOW_0_H_R` writer - Setting to 1 grants PeriBus2 permission to read RTCSlow_0 high address region."]
pub struct PRO_AHB_RTCSLOW_0_H_R_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_AHB_RTCSLOW_0_H_R_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `PRO_AHB_RTCSLOW_0_H_W` reader - Setting to 1 grants PeriBus2 permission to write RTCSlow_0 high address region."]
pub struct PRO_AHB_RTCSLOW_0_H_W_R(crate::FieldReader<bool, bool>);
impl PRO_AHB_RTCSLOW_0_H_W_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_AHB_RTCSLOW_0_H_W_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_AHB_RTCSLOW_0_H_W_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_AHB_RTCSLOW_0_H_W` writer - Setting to 1 grants PeriBus2 permission to write RTCSlow_0 high address region."]
pub struct PRO_AHB_RTCSLOW_0_H_W_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_AHB_RTCSLOW_0_H_W_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:10 - Configure the split address of RTCSlow_0 for PeriBus2 access."]
    #[inline(always)]
    pub fn pro_ahb_rtcslow_0_spltaddr(&self) -> PRO_AHB_RTCSLOW_0_SPLTADDR_R {
        PRO_AHB_RTCSLOW_0_SPLTADDR_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 11 - Setting to 1 grants PeriBus2 permission to fetch RTCSlow_0 low address region."]
    #[inline(always)]
    pub fn pro_ahb_rtcslow_0_l_f(&self) -> PRO_AHB_RTCSLOW_0_L_F_R {
        PRO_AHB_RTCSLOW_0_L_F_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Setting to 1 grants PeriBus2 permission to read RTCSlow_0 low address region."]
    #[inline(always)]
    pub fn pro_ahb_rtcslow_0_l_r(&self) -> PRO_AHB_RTCSLOW_0_L_R_R {
        PRO_AHB_RTCSLOW_0_L_R_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Setting to 1 grants PeriBus2 permission to write RTCSlow_0 low address region."]
    #[inline(always)]
    pub fn pro_ahb_rtcslow_0_l_w(&self) -> PRO_AHB_RTCSLOW_0_L_W_R {
        PRO_AHB_RTCSLOW_0_L_W_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Setting to 1 grants PeriBus2 permission to fetch RTCSlow_0 high address region."]
    #[inline(always)]
    pub fn pro_ahb_rtcslow_0_h_f(&self) -> PRO_AHB_RTCSLOW_0_H_F_R {
        PRO_AHB_RTCSLOW_0_H_F_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Setting to 1 grants PeriBus2 permission to read RTCSlow_0 high address region."]
    #[inline(always)]
    pub fn pro_ahb_rtcslow_0_h_r(&self) -> PRO_AHB_RTCSLOW_0_H_R_R {
        PRO_AHB_RTCSLOW_0_H_R_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Setting to 1 grants PeriBus2 permission to write RTCSlow_0 high address region."]
    #[inline(always)]
    pub fn pro_ahb_rtcslow_0_h_w(&self) -> PRO_AHB_RTCSLOW_0_H_W_R {
        PRO_AHB_RTCSLOW_0_H_W_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - Configure the split address of RTCSlow_0 for PeriBus2 access."]
    #[inline(always)]
    pub fn pro_ahb_rtcslow_0_spltaddr(&mut self) -> PRO_AHB_RTCSLOW_0_SPLTADDR_W {
        PRO_AHB_RTCSLOW_0_SPLTADDR_W { w: self }
    }
    #[doc = "Bit 11 - Setting to 1 grants PeriBus2 permission to fetch RTCSlow_0 low address region."]
    #[inline(always)]
    pub fn pro_ahb_rtcslow_0_l_f(&mut self) -> PRO_AHB_RTCSLOW_0_L_F_W {
        PRO_AHB_RTCSLOW_0_L_F_W { w: self }
    }
    #[doc = "Bit 12 - Setting to 1 grants PeriBus2 permission to read RTCSlow_0 low address region."]
    #[inline(always)]
    pub fn pro_ahb_rtcslow_0_l_r(&mut self) -> PRO_AHB_RTCSLOW_0_L_R_W {
        PRO_AHB_RTCSLOW_0_L_R_W { w: self }
    }
    #[doc = "Bit 13 - Setting to 1 grants PeriBus2 permission to write RTCSlow_0 low address region."]
    #[inline(always)]
    pub fn pro_ahb_rtcslow_0_l_w(&mut self) -> PRO_AHB_RTCSLOW_0_L_W_W {
        PRO_AHB_RTCSLOW_0_L_W_W { w: self }
    }
    #[doc = "Bit 14 - Setting to 1 grants PeriBus2 permission to fetch RTCSlow_0 high address region."]
    #[inline(always)]
    pub fn pro_ahb_rtcslow_0_h_f(&mut self) -> PRO_AHB_RTCSLOW_0_H_F_W {
        PRO_AHB_RTCSLOW_0_H_F_W { w: self }
    }
    #[doc = "Bit 15 - Setting to 1 grants PeriBus2 permission to read RTCSlow_0 high address region."]
    #[inline(always)]
    pub fn pro_ahb_rtcslow_0_h_r(&mut self) -> PRO_AHB_RTCSLOW_0_H_R_W {
        PRO_AHB_RTCSLOW_0_H_R_W { w: self }
    }
    #[doc = "Bit 16 - Setting to 1 grants PeriBus2 permission to write RTCSlow_0 high address region."]
    #[inline(always)]
    pub fn pro_ahb_rtcslow_0_h_w(&mut self) -> PRO_AHB_RTCSLOW_0_H_W_W {
        PRO_AHB_RTCSLOW_0_H_W_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PeriBus2 permission control register 1.\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_ahb_1]
(index.html) module"]
pub struct PRO_AHB_1_SPEC;
impl crate::RegisterSpec for PRO_AHB_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_ahb_1::R]
(R) reader structure"]
impl crate::Readable for PRO_AHB_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pro_ahb_1::W]
(W) writer structure"]
impl crate::Writable for PRO_AHB_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRO_AHB_1 to value 0x0001_f800"]
impl crate::Resettable for PRO_AHB_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_f800
    }
}
