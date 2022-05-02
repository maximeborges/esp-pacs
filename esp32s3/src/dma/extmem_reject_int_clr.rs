#[doc = "Register `EXTMEM_REJECT_INT_CLR` writer"]
pub struct W(crate::W<EXTMEM_REJECT_INT_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTMEM_REJECT_INT_CLR_SPEC>;
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
impl From<crate::W<EXTMEM_REJECT_INT_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTMEM_REJECT_INT_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTMEM_REJECT_INT_CLR` writer - Set this bit to clear the EXTMEM_REJECT_INT interrupt."]
pub struct EXTMEM_REJECT_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTMEM_REJECT_INT_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to clear the EXTMEM_REJECT_INT interrupt."]
    #[inline(always)]
    pub fn extmem_reject_int_clr(&mut self) -> EXTMEM_REJECT_INT_CLR_W {
        EXTMEM_REJECT_INT_CLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt clear bits of external RAM permission\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_reject_int_clr](index.html) module"]
pub struct EXTMEM_REJECT_INT_CLR_SPEC;
impl crate::RegisterSpec for EXTMEM_REJECT_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [extmem_reject_int_clr::W](W) writer structure"]
impl crate::Writable for EXTMEM_REJECT_INT_CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXTMEM_REJECT_INT_CLR to value 0"]
impl crate::Resettable for EXTMEM_REJECT_INT_CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
