#[doc = "Register `OUT1_W1TC` writer"]
pub struct W(crate::W<OUT1_W1TC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUT1_W1TC_SPEC>;
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
impl From<crate::W<OUT1_W1TC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUT1_W1TC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUT1_W1TC` writer - GPIO32 ~ 53 output value clear register. If the value 1 is written to a bit here, the corresponding bit in GPIO_OUT1_REG will be cleared. Recommended operation: use this register to clear GPIO_OUT1_REG."]
pub struct OUT1_W1TC_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT1_W1TC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x003f_ffff) | (value as u32 & 0x003f_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:21 - GPIO32 ~ 53 output value clear register. If the value 1 is written to a bit here, the corresponding bit in GPIO_OUT1_REG will be cleared. Recommended operation: use this register to clear GPIO_OUT1_REG."]
    #[inline(always)]
    pub fn out1_w1tc(&mut self) -> OUT1_W1TC_W {
        OUT1_W1TC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO32 ~ 53 output bit clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out1_w1tc](index.html) module"]
pub struct OUT1_W1TC_SPEC;
impl crate::RegisterSpec for OUT1_W1TC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [out1_w1tc::W](W) writer structure"]
impl crate::Writable for OUT1_W1TC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OUT1_W1TC to value 0"]
impl crate::Resettable for OUT1_W1TC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
