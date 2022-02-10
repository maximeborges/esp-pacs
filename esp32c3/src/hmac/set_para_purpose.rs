#[doc = "Register `SET_PARA_PURPOSE` writer"]
pub struct W(crate::W<SET_PARA_PURPOSE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SET_PARA_PURPOSE_SPEC>;
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
impl From<crate::W<SET_PARA_PURPOSE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SET_PARA_PURPOSE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PURPOSE_SET` writer - Set hmac parameter purpose."]
pub struct PURPOSE_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> PURPOSE_SET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:3 - Set hmac parameter purpose."]
    #[inline(always)]
    pub fn purpose_set(&mut self) -> PURPOSE_SET_W {
        PURPOSE_SET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configure purpose.\n\nThis register you can [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [set_para_purpose]
(index.html) module"]
pub struct SET_PARA_PURPOSE_SPEC;
impl crate::RegisterSpec for SET_PARA_PURPOSE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [set_para_purpose::W]
(W) writer structure"]
impl crate::Writable for SET_PARA_PURPOSE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SET_PARA_PURPOSE to value 0"]
impl crate::Resettable for SET_PARA_PURPOSE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
