#[doc = "Register `DMMU_TABLE11` reader"]
pub struct R(crate::R<DMMU_TABLE11_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMMU_TABLE11_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMMU_TABLE11_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMMU_TABLE11_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMMU_TABLE11` writer"]
pub struct W(crate::W<DMMU_TABLE11_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMMU_TABLE11_SPEC>;
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
impl From<crate::W<DMMU_TABLE11_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMMU_TABLE11_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMMU_TABLE11` reader - "]
pub struct DMMU_TABLE11_R(crate::FieldReader<u8>);
impl DMMU_TABLE11_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DMMU_TABLE11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMMU_TABLE11_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMMU_TABLE11` writer - "]
pub struct DMMU_TABLE11_W<'a> {
    w: &'a mut W,
}
impl<'a> DMMU_TABLE11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn dmmu_table11(&self) -> DMMU_TABLE11_R {
        DMMU_TABLE11_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn dmmu_table11(&mut self) -> DMMU_TABLE11_W {
        DMMU_TABLE11_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmmu_table11](index.html) module"]
pub struct DMMU_TABLE11_SPEC;
impl crate::RegisterSpec for DMMU_TABLE11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmmu_table11::R](R) reader structure"]
impl crate::Readable for DMMU_TABLE11_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmmu_table11::W](W) writer structure"]
impl crate::Writable for DMMU_TABLE11_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMMU_TABLE11 to value 0x0b"]
impl crate::Resettable for DMMU_TABLE11_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0b
    }
}
