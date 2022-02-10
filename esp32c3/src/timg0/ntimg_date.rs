#[doc = "Register `NTIMG_DATE` reader"]
pub struct R(crate::R<NTIMG_DATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NTIMG_DATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NTIMG_DATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NTIMG_DATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NTIMG_DATE` writer"]
pub struct W(crate::W<NTIMG_DATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NTIMG_DATE_SPEC>;
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
impl From<crate::W<NTIMG_DATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NTIMG_DATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NTIMGS_DATE` reader - reg_ntimers_date."]
pub struct NTIMGS_DATE_R(crate::FieldReader<u32, u32>);
impl NTIMGS_DATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        NTIMGS_DATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NTIMGS_DATE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NTIMGS_DATE` writer - reg_ntimers_date."]
pub struct NTIMGS_DATE_W<'a> {
    w: &'a mut W,
}
impl<'a> NTIMGS_DATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff_ffff) | (value as u32 & 0x0fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:27 - reg_ntimers_date."]
    #[inline(always)]
    pub fn ntimgs_date(&self) -> NTIMGS_DATE_R {
        NTIMGS_DATE_R::new((self.bits & 0x0fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:27 - reg_ntimers_date."]
    #[inline(always)]
    pub fn ntimgs_date(&mut self) -> NTIMGS_DATE_W {
        NTIMGS_DATE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIMG_NTIMG_DATE_REG.\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ntimg_date]
(index.html) module"]
pub struct NTIMG_DATE_SPEC;
impl crate::RegisterSpec for NTIMG_DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ntimg_date::R]
(R) reader structure"]
impl crate::Readable for NTIMG_DATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ntimg_date::W]
(W) writer structure"]
impl crate::Writable for NTIMG_DATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NTIMG_DATE to value 0x0200_6191"]
impl crate::Resettable for NTIMG_DATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200_6191
    }
}
