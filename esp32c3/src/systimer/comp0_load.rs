#[doc = "Register `COMP0_LOAD` writer"]
pub struct W(crate::W<COMP0_LOAD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP0_LOAD_SPEC>;
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
impl From<crate::W<COMP0_LOAD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMP0_LOAD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER_COMP0_LOAD` writer - timer comp0 load value"]
pub type TIMER_COMP0_LOAD_W<'a> = crate::BitWriter<'a, u32, COMP0_LOAD_SPEC, bool, 0>;
impl W {
    #[doc = "Bit 0 - timer comp0 load value"]
    #[inline(always)]
    pub fn timer_comp0_load(&mut self) -> TIMER_COMP0_LOAD_W {
        TIMER_COMP0_LOAD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYSTIMER_COMP0_LOAD.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp0_load](index.html) module"]
pub struct COMP0_LOAD_SPEC;
impl crate::RegisterSpec for COMP0_LOAD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [comp0_load::W](W) writer structure"]
impl crate::Writable for COMP0_LOAD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COMP0_LOAD to value 0"]
impl crate::Resettable for COMP0_LOAD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
