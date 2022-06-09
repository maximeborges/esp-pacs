#[doc = "Register `M_%s` reader"]
pub struct R(crate::R<M__SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M__SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M__SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M__SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `M_%s` writer"]
pub struct W(crate::W<M__SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<M__SPEC>;
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
impl From<crate::W<M__SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<M__SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `M_0` reader - Stores the %sth 32-bit piece of the message."]
pub type M_0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `M_0` writer - Stores the %sth 32-bit piece of the message."]
pub type M_0_W<'a> = crate::FieldWriter<'a, u32, M__SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Stores the %sth 32-bit piece of the message."]
    #[inline(always)]
    pub fn m_0(&self) -> M_0_R {
        M_0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Stores the %sth 32-bit piece of the message."]
    #[inline(always)]
    pub fn m_0(&mut self) -> M_0_W {
        M_0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Message\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m_](index.html) module"]
pub struct M__SPEC;
impl crate::RegisterSpec for M__SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [m_::R](R) reader structure"]
impl crate::Readable for M__SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [m_::W](W) writer structure"]
impl crate::Writable for M__SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets M_%s to value 0"]
impl crate::Resettable for M__SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
