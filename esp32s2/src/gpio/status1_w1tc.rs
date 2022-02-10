#[doc = "Register `STATUS1_W1TC` writer"]
pub struct W(crate::W<STATUS1_W1TC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATUS1_W1TC_SPEC>;
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
impl From<crate::W<STATUS1_W1TC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATUS1_W1TC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STATUS1_W1TC` writer - GPIO32 ~ 53 interrupt status clear register. If the value 1 is written to a bit here, the corresponding bit in GPIO_STATUS1_REG will be cleared. Recommended operation: use this register to clear GPIO_STATUS1_REG."]
pub struct STATUS1_W1TC_W<'a> {
    w: &'a mut W,
}
impl<'a> STATUS1_W1TC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x003f_ffff) | (value as u32 & 0x003f_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:21 - GPIO32 ~ 53 interrupt status clear register. If the value 1 is written to a bit here, the corresponding bit in GPIO_STATUS1_REG will be cleared. Recommended operation: use this register to clear GPIO_STATUS1_REG."]
    #[inline(always)]
    pub fn status1_w1tc(&mut self) -> STATUS1_W1TC_W {
        STATUS1_W1TC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO32 ~ 53 interrupt status bit clear register\n\nThis register you can [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status1_w1tc]
(index.html) module"]
pub struct STATUS1_W1TC_SPEC;
impl crate::RegisterSpec for STATUS1_W1TC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [status1_w1tc::W]
(W) writer structure"]
impl crate::Writable for STATUS1_W1TC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STATUS1_W1TC to value 0"]
impl crate::Resettable for STATUS1_W1TC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
