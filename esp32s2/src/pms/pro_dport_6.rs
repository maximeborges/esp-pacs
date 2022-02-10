#[doc = "Register `PRO_DPORT_6` reader"]
pub struct R(crate::R<PRO_DPORT_6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_DPORT_6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_DPORT_6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_DPORT_6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRO_DPORT_6` writer"]
pub struct W(crate::W<PRO_DPORT_6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRO_DPORT_6_SPEC>;
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
impl From<crate::W<PRO_DPORT_6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRO_DPORT_6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRO_DPORT_ILG_CLR` reader - The clear signal for PeriBus1 access interrupt."]
pub struct PRO_DPORT_ILG_CLR_R(crate::FieldReader<bool, bool>);
impl PRO_DPORT_ILG_CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_DPORT_ILG_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_DPORT_ILG_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_DPORT_ILG_CLR` writer - The clear signal for PeriBus1 access interrupt."]
pub struct PRO_DPORT_ILG_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_DPORT_ILG_CLR_W<'a> {
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
#[doc = "Field `PRO_DPORT_ILG_EN` reader - The enable signal for PeriBus1 access interrupt."]
pub struct PRO_DPORT_ILG_EN_R(crate::FieldReader<bool, bool>);
impl PRO_DPORT_ILG_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_DPORT_ILG_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_DPORT_ILG_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_DPORT_ILG_EN` writer - The enable signal for PeriBus1 access interrupt."]
pub struct PRO_DPORT_ILG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_DPORT_ILG_EN_W<'a> {
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
#[doc = "Field `PRO_DPORT_ILG_INTR` reader - PeriBus1 access interrupt signal."]
pub struct PRO_DPORT_ILG_INTR_R(crate::FieldReader<bool, bool>);
impl PRO_DPORT_ILG_INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_DPORT_ILG_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_DPORT_ILG_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - The clear signal for PeriBus1 access interrupt."]
    #[inline(always)]
    pub fn pro_dport_ilg_clr(&self) -> PRO_DPORT_ILG_CLR_R {
        PRO_DPORT_ILG_CLR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - The enable signal for PeriBus1 access interrupt."]
    #[inline(always)]
    pub fn pro_dport_ilg_en(&self) -> PRO_DPORT_ILG_EN_R {
        PRO_DPORT_ILG_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PeriBus1 access interrupt signal."]
    #[inline(always)]
    pub fn pro_dport_ilg_intr(&self) -> PRO_DPORT_ILG_INTR_R {
        PRO_DPORT_ILG_INTR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The clear signal for PeriBus1 access interrupt."]
    #[inline(always)]
    pub fn pro_dport_ilg_clr(&mut self) -> PRO_DPORT_ILG_CLR_W {
        PRO_DPORT_ILG_CLR_W { w: self }
    }
    #[doc = "Bit 1 - The enable signal for PeriBus1 access interrupt."]
    #[inline(always)]
    pub fn pro_dport_ilg_en(&mut self) -> PRO_DPORT_ILG_EN_W {
        PRO_DPORT_ILG_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PeriBus1 permission control register 6.\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_dport_6]
(index.html) module"]
pub struct PRO_DPORT_6_SPEC;
impl crate::RegisterSpec for PRO_DPORT_6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_dport_6::R]
(R) reader structure"]
impl crate::Readable for PRO_DPORT_6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pro_dport_6::W]
(W) writer structure"]
impl crate::Writable for PRO_DPORT_6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRO_DPORT_6 to value 0"]
impl crate::Resettable for PRO_DPORT_6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
