#[doc = "Register `BUSTOEXTMEM_ENA` reader"]
pub struct R(crate::R<BUSTOEXTMEM_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUSTOEXTMEM_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUSTOEXTMEM_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUSTOEXTMEM_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUSTOEXTMEM_ENA` writer"]
pub struct W(crate::W<BUSTOEXTMEM_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUSTOEXTMEM_ENA_SPEC>;
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
impl From<crate::W<BUSTOEXTMEM_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUSTOEXTMEM_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUSTOEXTMEM_ENA` reader - Set this bit to enable bus to EDMA."]
pub struct BUSTOEXTMEM_ENA_R(crate::FieldReader<bool, bool>);
impl BUSTOEXTMEM_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUSTOEXTMEM_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSTOEXTMEM_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUSTOEXTMEM_ENA` writer - Set this bit to enable bus to EDMA."]
pub struct BUSTOEXTMEM_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSTOEXTMEM_ENA_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Set this bit to enable bus to EDMA."]
    #[inline(always)]
    pub fn bustoextmem_ena(&self) -> BUSTOEXTMEM_ENA_R {
        BUSTOEXTMEM_ENA_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to enable bus to EDMA."]
    #[inline(always)]
    pub fn bustoextmem_ena(&mut self) -> BUSTOEXTMEM_ENA_W {
        BUSTOEXTMEM_ENA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EDMA enable register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bustoextmem_ena]
(index.html) module"]
pub struct BUSTOEXTMEM_ENA_SPEC;
impl crate::RegisterSpec for BUSTOEXTMEM_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bustoextmem_ena::R]
(R) reader structure"]
impl crate::Readable for BUSTOEXTMEM_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bustoextmem_ena::W]
(W) writer structure"]
impl crate::Writable for BUSTOEXTMEM_ENA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BUSTOEXTMEM_ENA to value 0x01"]
impl crate::Resettable for BUSTOEXTMEM_ENA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
