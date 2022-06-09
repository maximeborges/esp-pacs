#[doc = "Register `BUFFER_%s` writer"]
pub struct W(crate::W<BUFFER__SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUFFER__SPEC>;
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
impl From<crate::W<BUFFER__SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUFFER__SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUFFER` writer - Data buffers for encryption."]
pub type BUFFER_W<'a> = crate::FieldWriter<'a, u32, BUFFER__SPEC, u8, u8, 8, 0>;
impl W {
    #[doc = "Bits 0:7 - Data buffers for encryption."]
    #[inline(always)]
    pub fn buffer(&mut self) -> BUFFER_W {
        BUFFER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buffer_](index.html) module"]
pub struct BUFFER__SPEC;
impl crate::RegisterSpec for BUFFER__SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [buffer_::W](W) writer structure"]
impl crate::Writable for BUFFER__SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BUFFER_%s to value 0"]
impl crate::Resettable for BUFFER__SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
