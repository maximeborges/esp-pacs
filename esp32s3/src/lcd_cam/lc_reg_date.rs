#[doc = "Register `LC_REG_DATE` reader"]
pub struct R(crate::R<LC_REG_DATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LC_REG_DATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LC_REG_DATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LC_REG_DATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LC_REG_DATE` writer"]
pub struct W(crate::W<LC_REG_DATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LC_REG_DATE_SPEC>;
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
impl From<crate::W<LC_REG_DATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LC_REG_DATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LC_DATE` reader - LCD_CAM version control register"]
pub struct LC_DATE_R(crate::FieldReader<u32>);
impl LC_DATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        LC_DATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LC_DATE_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LC_DATE` writer - LCD_CAM version control register"]
pub struct LC_DATE_W<'a> {
    w: &'a mut W,
}
impl<'a> LC_DATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff_ffff) | (value as u32 & 0x0fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:27 - LCD_CAM version control register"]
    #[inline(always)]
    pub fn lc_date(&self) -> LC_DATE_R {
        LC_DATE_R::new((self.bits & 0x0fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:27 - LCD_CAM version control register"]
    #[inline(always)]
    pub fn lc_date(&mut self) -> LC_DATE_W {
        LC_DATE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Version register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lc_reg_date](index.html) module"]
pub struct LC_REG_DATE_SPEC;
impl crate::RegisterSpec for LC_REG_DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lc_reg_date::R](R) reader structure"]
impl crate::Readable for LC_REG_DATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lc_reg_date::W](W) writer structure"]
impl crate::Writable for LC_REG_DATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LC_REG_DATE to value 0x0200_3020"]
impl crate::Resettable for LC_REG_DATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200_3020
    }
}
