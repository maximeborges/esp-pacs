#[doc = "Register `SCL_HIGH` reader"]
pub struct R(crate::R<SCL_HIGH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCL_HIGH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCL_HIGH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCL_HIGH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCL_HIGH` writer"]
pub struct W(crate::W<SCL_HIGH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCL_HIGH_SPEC>;
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
impl From<crate::W<SCL_HIGH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCL_HIGH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERIOD` reader - This register is used to configure how many cycles SCL remains high."]
pub struct PERIOD_R(crate::FieldReader<u32>);
impl PERIOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PERIOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERIOD_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PERIOD` writer - This register is used to configure how many cycles SCL remains high."]
pub struct PERIOD_W<'a> {
    w: &'a mut W,
}
impl<'a> PERIOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | (value as u32 & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19 - This register is used to configure how many cycles SCL remains high."]
    #[inline(always)]
    pub fn period(&self) -> PERIOD_R {
        PERIOD_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19 - This register is used to configure how many cycles SCL remains high."]
    #[inline(always)]
    pub fn period(&mut self) -> PERIOD_W {
        PERIOD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configure the high level width of SCL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scl_high](index.html) module"]
pub struct SCL_HIGH_SPEC;
impl crate::RegisterSpec for SCL_HIGH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scl_high::R](R) reader structure"]
impl crate::Readable for SCL_HIGH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scl_high::W](W) writer structure"]
impl crate::Writable for SCL_HIGH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCL_HIGH to value 0x0100"]
impl crate::Resettable for SCL_HIGH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0100
    }
}
