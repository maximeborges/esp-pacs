#[doc = "Register `APB_CTRL_DATE` reader"]
pub struct R(crate::R<APB_CTRL_DATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB_CTRL_DATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB_CTRL_DATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB_CTRL_DATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB_CTRL_DATE` writer"]
pub struct W(crate::W<APB_CTRL_DATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB_CTRL_DATE_SPEC>;
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
impl From<crate::W<APB_CTRL_DATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB_CTRL_DATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `APB_CTRL_DATE` reader - version"]
pub struct APB_CTRL_DATE_R(crate::FieldReader<u32, u32>);
impl APB_CTRL_DATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        APB_CTRL_DATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB_CTRL_DATE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB_CTRL_DATE` writer - version"]
pub struct APB_CTRL_DATE_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_CTRL_DATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - version"]
    #[inline(always)]
    pub fn apb_ctrl_date(&self) -> APB_CTRL_DATE_R {
        APB_CTRL_DATE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - version"]
    #[inline(always)]
    pub fn apb_ctrl_date(&mut self) -> APB_CTRL_DATE_W {
        APB_CTRL_DATE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "version\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_ctrl_date]
(index.html) module"]
pub struct APB_CTRL_DATE_SPEC;
impl crate::RegisterSpec for APB_CTRL_DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb_ctrl_date::R]
(R) reader structure"]
impl crate::Readable for APB_CTRL_DATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb_ctrl_date::W]
(W) writer structure"]
impl crate::Writable for APB_CTRL_DATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB_CTRL_DATE to value 0x0210_1180"]
impl crate::Resettable for APB_CTRL_DATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0210_1180
    }
}
