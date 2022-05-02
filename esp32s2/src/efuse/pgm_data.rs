#[doc = "Register `PGM_DATA%s` reader"]
pub struct R(crate::R<PGM_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PGM_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PGM_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PGM_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PGM_DATA%s` writer"]
pub struct W(crate::W<PGM_DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PGM_DATA_SPEC>;
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
impl From<crate::W<PGM_DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PGM_DATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PGM_DATA_0` reader - The content of the %sth 32-bit data to be programmed."]
pub struct PGM_DATA_0_R(crate::FieldReader<u32>);
impl PGM_DATA_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PGM_DATA_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PGM_DATA_0_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PGM_DATA_0` writer - The content of the %sth 32-bit data to be programmed."]
pub struct PGM_DATA_0_W<'a> {
    w: &'a mut W,
}
impl<'a> PGM_DATA_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - The content of the %sth 32-bit data to be programmed."]
    #[inline(always)]
    pub fn pgm_data_0(&self) -> PGM_DATA_0_R {
        PGM_DATA_0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The content of the %sth 32-bit data to be programmed."]
    #[inline(always)]
    pub fn pgm_data_0(&mut self) -> PGM_DATA_0_W {
        PGM_DATA_0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register %s that stores data to be programmed.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pgm_data](index.html) module"]
pub struct PGM_DATA_SPEC;
impl crate::RegisterSpec for PGM_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pgm_data::R](R) reader structure"]
impl crate::Readable for PGM_DATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pgm_data::W](W) writer structure"]
impl crate::Writable for PGM_DATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PGM_DATA%s to value 0"]
impl crate::Resettable for PGM_DATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
