#[doc = "Register `RETENTION_CTRL4` reader"]
pub struct R(crate::R<RETENTION_CTRL4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RETENTION_CTRL4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RETENTION_CTRL4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RETENTION_CTRL4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RETENTION_CTRL4` writer"]
pub struct W(crate::W<RETENTION_CTRL4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RETENTION_CTRL4_SPEC>;
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
impl From<crate::W<RETENTION_CTRL4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RETENTION_CTRL4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RETENTION_INV_CFG` reader - ******* Description ***********"]
pub struct RETENTION_INV_CFG_R(crate::FieldReader<u32>);
impl RETENTION_INV_CFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        RETENTION_INV_CFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RETENTION_INV_CFG_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RETENTION_INV_CFG` writer - ******* Description ***********"]
pub struct RETENTION_INV_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> RETENTION_INV_CFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - ******* Description ***********"]
    #[inline(always)]
    pub fn retention_inv_cfg(&self) -> RETENTION_INV_CFG_R {
        RETENTION_INV_CFG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ******* Description ***********"]
    #[inline(always)]
    pub fn retention_inv_cfg(&mut self) -> RETENTION_INV_CFG_W {
        RETENTION_INV_CFG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "******* Description ***********\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [retention_ctrl4](index.html) module"]
pub struct RETENTION_CTRL4_SPEC;
impl crate::RegisterSpec for RETENTION_CTRL4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [retention_ctrl4::R](R) reader structure"]
impl crate::Readable for RETENTION_CTRL4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [retention_ctrl4::W](W) writer structure"]
impl crate::Writable for RETENTION_CTRL4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RETENTION_CTRL4 to value 0xffff_ffff"]
impl crate::Resettable for RETENTION_CTRL4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
