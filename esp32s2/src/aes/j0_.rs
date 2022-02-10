#[doc = "Register `J0_%s` reader"]
pub struct R(crate::R<J0__SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<J0__SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<J0__SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<J0__SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `J0_%s` writer"]
pub struct W(crate::W<J0__SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<J0__SPEC>;
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
impl From<crate::W<J0__SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<J0__SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `J0_0` reader - This register stores the %sth 32-bit piece of 128-bit J0"]
pub struct J0_0_R(crate::FieldReader<u32, u32>);
impl J0_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        J0_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for J0_0_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `J0_0` writer - This register stores the %sth 32-bit piece of 128-bit J0"]
pub struct J0_0_W<'a> {
    w: &'a mut W,
}
impl<'a> J0_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - This register stores the %sth 32-bit piece of 128-bit J0"]
    #[inline(always)]
    pub fn j0_0(&self) -> J0_0_R {
        J0_0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This register stores the %sth 32-bit piece of 128-bit J0"]
    #[inline(always)]
    pub fn j0_0(&mut self) -> J0_0_W {
        J0_0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "J0\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [j0_]
(index.html) module"]
pub struct J0__SPEC;
impl crate::RegisterSpec for J0__SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [j0_::R]
(R) reader structure"]
impl crate::Readable for J0__SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [j0_::W]
(W) writer structure"]
impl crate::Writable for J0__SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets J0_%s to value 0"]
impl crate::Resettable for J0__SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
