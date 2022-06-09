#[doc = "Register `LOAD_HI` reader"]
pub struct R(crate::R<LOAD_HI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOAD_HI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOAD_HI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOAD_HI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LOAD_HI` writer"]
pub struct W(crate::W<LOAD_HI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LOAD_HI_SPEC>;
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
impl From<crate::W<LOAD_HI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LOAD_HI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER_LOAD_HI` reader - The value to be loaded into system timer, high 32 bits."]
pub type TIMER_LOAD_HI_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TIMER_LOAD_HI` writer - The value to be loaded into system timer, high 32 bits."]
pub type TIMER_LOAD_HI_W<'a> = crate::FieldWriter<'a, u32, LOAD_HI_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - The value to be loaded into system timer, high 32 bits."]
    #[inline(always)]
    pub fn timer_load_hi(&self) -> TIMER_LOAD_HI_R {
        TIMER_LOAD_HI_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The value to be loaded into system timer, high 32 bits."]
    #[inline(always)]
    pub fn timer_load_hi(&mut self) -> TIMER_LOAD_HI_W {
        TIMER_LOAD_HI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "High 32 bits to be loaded to system timer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [load_hi](index.html) module"]
pub struct LOAD_HI_SPEC;
impl crate::RegisterSpec for LOAD_HI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [load_hi::R](R) reader structure"]
impl crate::Readable for LOAD_HI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [load_hi::W](W) writer structure"]
impl crate::Writable for LOAD_HI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LOAD_HI to value 0"]
impl crate::Resettable for LOAD_HI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
