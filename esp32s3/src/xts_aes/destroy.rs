#[doc = "Register `DESTROY` writer"]
pub struct W(crate::W<DESTROY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DESTROY_SPEC>;
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
impl From<crate::W<DESTROY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DESTROY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DESTROY` writer - Write 1 to destroy encrypted result."]
pub type DESTROY_W<'a> = crate::BitWriter<'a, u32, DESTROY_SPEC, bool, 0>;
impl W {
    #[doc = "Bit 0 - Write 1 to destroy encrypted result."]
    #[inline(always)]
    pub fn destroy(&mut self) -> DESTROY_W {
        DESTROY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "XTS-AES destroy control register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [destroy](index.html) module"]
pub struct DESTROY_SPEC;
impl crate::RegisterSpec for DESTROY_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [destroy::W](W) writer structure"]
impl crate::Writable for DESTROY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DESTROY to value 0"]
impl crate::Resettable for DESTROY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
