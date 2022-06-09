#[doc = "Register `KEY_2` reader"]
pub struct R(crate::R<KEY_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEY_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KEY_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KEY_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KEY_2` writer"]
pub struct W(crate::W<KEY_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEY_2_SPEC>;
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
impl From<crate::W<KEY_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEY_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEY_2` reader - This bits stores key_2 that is a part of key material."]
pub type KEY_2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `KEY_2` writer - This bits stores key_2 that is a part of key material."]
pub type KEY_2_W<'a> = crate::FieldWriter<'a, u32, KEY_2_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - This bits stores key_2 that is a part of key material."]
    #[inline(always)]
    pub fn key_2(&self) -> KEY_2_R {
        KEY_2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This bits stores key_2 that is a part of key material."]
    #[inline(always)]
    pub fn key_2(&mut self) -> KEY_2_W {
        KEY_2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Key material key_2 configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key_2](index.html) module"]
pub struct KEY_2_SPEC;
impl crate::RegisterSpec for KEY_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [key_2::R](R) reader structure"]
impl crate::Readable for KEY_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [key_2::W](W) writer structure"]
impl crate::Writable for KEY_2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets KEY_2 to value 0"]
impl crate::Resettable for KEY_2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
