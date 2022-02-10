#[doc = "Register `EDMA_BOUNDARY_0` reader"]
pub struct R(crate::R<EDMA_BOUNDARY_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EDMA_BOUNDARY_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EDMA_BOUNDARY_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EDMA_BOUNDARY_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EDMA_BOUNDARY_0` writer"]
pub struct W(crate::W<EDMA_BOUNDARY_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EDMA_BOUNDARY_0_SPEC>;
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
impl From<crate::W<EDMA_BOUNDARY_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EDMA_BOUNDARY_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EDMA_BOUNDARY_0` reader - This field is used to configure the boundary 0 of external RAM. The unit is 4K. For example, set this field to 0x80, then the address boundary 0 would be 0x3C080000 (0x3C000000 + 0x80 * 4K)."]
pub struct EDMA_BOUNDARY_0_R(crate::FieldReader<u16, u16>);
impl EDMA_BOUNDARY_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        EDMA_BOUNDARY_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EDMA_BOUNDARY_0_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EDMA_BOUNDARY_0` writer - This field is used to configure the boundary 0 of external RAM. The unit is 4K. For example, set this field to 0x80, then the address boundary 0 would be 0x3C080000 (0x3C000000 + 0x80 * 4K)."]
pub struct EDMA_BOUNDARY_0_W<'a> {
    w: &'a mut W,
}
impl<'a> EDMA_BOUNDARY_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | (value as u32 & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13 - This field is used to configure the boundary 0 of external RAM. The unit is 4K. For example, set this field to 0x80, then the address boundary 0 would be 0x3C080000 (0x3C000000 + 0x80 * 4K)."]
    #[inline(always)]
    pub fn edma_boundary_0(&self) -> EDMA_BOUNDARY_0_R {
        EDMA_BOUNDARY_0_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - This field is used to configure the boundary 0 of external RAM. The unit is 4K. For example, set this field to 0x80, then the address boundary 0 would be 0x3C080000 (0x3C000000 + 0x80 * 4K)."]
    #[inline(always)]
    pub fn edma_boundary_0(&mut self) -> EDMA_BOUNDARY_0_W {
        EDMA_BOUNDARY_0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EDMA boundary 0 configuration\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [edma_boundary_0]
(index.html) module"]
pub struct EDMA_BOUNDARY_0_SPEC;
impl crate::RegisterSpec for EDMA_BOUNDARY_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [edma_boundary_0::R]
(R) reader structure"]
impl crate::Readable for EDMA_BOUNDARY_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [edma_boundary_0::W]
(W) writer structure"]
impl crate::Writable for EDMA_BOUNDARY_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EDMA_BOUNDARY_0 to value 0"]
impl crate::Resettable for EDMA_BOUNDARY_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
