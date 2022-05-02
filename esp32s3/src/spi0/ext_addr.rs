#[doc = "Register `EXT_ADDR` reader"]
pub struct R(crate::R<EXT_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXT_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXT_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXT_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXT_ADDR` writer"]
pub struct W(crate::W<EXT_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXT_ADDR_SPEC>;
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
impl From<crate::W<EXT_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXT_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXT_ADDR` reader - The register are the higher 32bits in the 64 bits address mode."]
pub struct EXT_ADDR_R(crate::FieldReader<u32>);
impl EXT_ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        EXT_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXT_ADDR_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXT_ADDR` writer - The register are the higher 32bits in the 64 bits address mode."]
pub struct EXT_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> EXT_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - The register are the higher 32bits in the 64 bits address mode."]
    #[inline(always)]
    pub fn ext_addr(&self) -> EXT_ADDR_R {
        EXT_ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The register are the higher 32bits in the 64 bits address mode."]
    #[inline(always)]
    pub fn ext_addr(&mut self) -> EXT_ADDR_W {
        EXT_ADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI0 extended address register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ext_addr](index.html) module"]
pub struct EXT_ADDR_SPEC;
impl crate::RegisterSpec for EXT_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ext_addr::R](R) reader structure"]
impl crate::Readable for EXT_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ext_addr::W](W) writer structure"]
impl crate::Writable for EXT_ADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXT_ADDR to value 0"]
impl crate::Resettable for EXT_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
