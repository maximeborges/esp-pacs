#[doc = "Register `INT_ENA` reader"]
pub struct R(crate::R<INT_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_ENA` writer"]
pub struct W(crate::W<INT_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_ENA_SPEC>;
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
impl From<crate::W<INT_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TARGET0_INT_ENA` reader - interupt0 enable"]
pub struct TARGET0_INT_ENA_R(crate::FieldReader<bool>);
impl TARGET0_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TARGET0_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TARGET0_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TARGET0_INT_ENA` writer - interupt0 enable"]
pub struct TARGET0_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TARGET0_INT_ENA_W<'a> {
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
#[doc = "Field `TARGET1_INT_ENA` reader - interupt1 enable"]
pub struct TARGET1_INT_ENA_R(crate::FieldReader<bool>);
impl TARGET1_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TARGET1_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TARGET1_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TARGET1_INT_ENA` writer - interupt1 enable"]
pub struct TARGET1_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TARGET1_INT_ENA_W<'a> {
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
#[doc = "Field `TARGET2_INT_ENA` reader - interupt2 enable"]
pub struct TARGET2_INT_ENA_R(crate::FieldReader<bool>);
impl TARGET2_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TARGET2_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TARGET2_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TARGET2_INT_ENA` writer - interupt2 enable"]
pub struct TARGET2_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TARGET2_INT_ENA_W<'a> {
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
impl R {
    #[doc = "Bit 0 - interupt0 enable"]
    #[inline(always)]
    pub fn target0_int_ena(&self) -> TARGET0_INT_ENA_R {
        TARGET0_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - interupt1 enable"]
    #[inline(always)]
    pub fn target1_int_ena(&self) -> TARGET1_INT_ENA_R {
        TARGET1_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - interupt2 enable"]
    #[inline(always)]
    pub fn target2_int_ena(&self) -> TARGET2_INT_ENA_R {
        TARGET2_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - interupt0 enable"]
    #[inline(always)]
    pub fn target0_int_ena(&mut self) -> TARGET0_INT_ENA_W {
        TARGET0_INT_ENA_W { w: self }
    }
    #[doc = "Bit 1 - interupt1 enable"]
    #[inline(always)]
    pub fn target1_int_ena(&mut self) -> TARGET1_INT_ENA_W {
        TARGET1_INT_ENA_W { w: self }
    }
    #[doc = "Bit 2 - interupt2 enable"]
    #[inline(always)]
    pub fn target2_int_ena(&mut self) -> TARGET2_INT_ENA_W {
        TARGET2_INT_ENA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYSTIMER_INT_ENA.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_ena](index.html) module"]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_ena::R](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_ena::W](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
