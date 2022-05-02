#[doc = "Register `W8` reader"]
pub struct R(crate::R<W8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<W8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<W8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<W8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `W8` writer"]
pub struct W(crate::W<W8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<W8_SPEC>;
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
impl From<crate::W<W8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<W8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUF8` reader - data buffer"]
pub struct BUF8_R(crate::FieldReader<u32>);
impl BUF8_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        BUF8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUF8_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUF8` writer - data buffer"]
pub struct BUF8_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn buf8(&self) -> BUF8_R {
        BUF8_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn buf8(&mut self) -> BUF8_W {
        BUF8_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [w8](index.html) module"]
pub struct W8_SPEC;
impl crate::RegisterSpec for W8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [w8::R](R) reader structure"]
impl crate::Readable for W8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [w8::W](W) writer structure"]
impl crate::Writable for W8_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets W8 to value 0"]
impl crate::Resettable for W8_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
