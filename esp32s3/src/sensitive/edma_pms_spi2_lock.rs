#[doc = "Register `EDMA_PMS_SPI2_LOCK` reader"]
pub struct R(crate::R<EDMA_PMS_SPI2_LOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EDMA_PMS_SPI2_LOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EDMA_PMS_SPI2_LOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EDMA_PMS_SPI2_LOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EDMA_PMS_SPI2_LOCK` writer"]
pub struct W(crate::W<EDMA_PMS_SPI2_LOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EDMA_PMS_SPI2_LOCK_SPEC>;
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
impl From<crate::W<EDMA_PMS_SPI2_LOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EDMA_PMS_SPI2_LOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EDMA_PMS_SPI2_LOCK` reader - Set 1 to lock EDMA-SPI2 permission control registers."]
pub struct EDMA_PMS_SPI2_LOCK_R(crate::FieldReader<bool>);
impl EDMA_PMS_SPI2_LOCK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EDMA_PMS_SPI2_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EDMA_PMS_SPI2_LOCK_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EDMA_PMS_SPI2_LOCK` writer - Set 1 to lock EDMA-SPI2 permission control registers."]
pub struct EDMA_PMS_SPI2_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> EDMA_PMS_SPI2_LOCK_W<'a> {
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
    #[doc = "Bit 0 - Set 1 to lock EDMA-SPI2 permission control registers."]
    #[inline(always)]
    pub fn edma_pms_spi2_lock(&self) -> EDMA_PMS_SPI2_LOCK_R {
        EDMA_PMS_SPI2_LOCK_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to lock EDMA-SPI2 permission control registers."]
    #[inline(always)]
    pub fn edma_pms_spi2_lock(&mut self) -> EDMA_PMS_SPI2_LOCK_W {
        EDMA_PMS_SPI2_LOCK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EDMA-SPI2 permission lock register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [edma_pms_spi2_lock](index.html) module"]
pub struct EDMA_PMS_SPI2_LOCK_SPEC;
impl crate::RegisterSpec for EDMA_PMS_SPI2_LOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [edma_pms_spi2_lock::R](R) reader structure"]
impl crate::Readable for EDMA_PMS_SPI2_LOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [edma_pms_spi2_lock::W](W) writer structure"]
impl crate::Writable for EDMA_PMS_SPI2_LOCK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EDMA_PMS_SPI2_LOCK to value 0"]
impl crate::Resettable for EDMA_PMS_SPI2_LOCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
