#[doc = "Register `SET_START_MODMULT` writer"]
pub struct W(crate::W<SET_START_MODMULT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SET_START_MODMULT_SPEC>;
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
impl From<crate::W<SET_START_MODMULT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SET_START_MODMULT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SET_START_MODMULT` writer - start modular multiplication"]
pub type SET_START_MODMULT_W<'a> = crate::BitWriter<'a, u32, SET_START_MODMULT_SPEC, bool, 0>;
impl W {
    #[doc = "Bit 0 - start modular multiplication"]
    #[inline(always)]
    pub fn set_start_modmult(&mut self) -> SET_START_MODMULT_W {
        SET_START_MODMULT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RSA modular multiplication trigger register.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [set_start_modmult](index.html) module"]
pub struct SET_START_MODMULT_SPEC;
impl crate::RegisterSpec for SET_START_MODMULT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [set_start_modmult::W](W) writer structure"]
impl crate::Writable for SET_START_MODMULT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SET_START_MODMULT to value 0"]
impl crate::Resettable for SET_START_MODMULT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
