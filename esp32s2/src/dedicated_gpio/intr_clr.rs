#[doc = "Register `INTR_CLR` writer"]
pub struct W(crate::W<INTR_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_CLR_SPEC>;
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
impl From<crate::W<INTR_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO0_INT_CLR` writer - Set this bit to clear the DEDIC_GPIO0_INT_RAW interrupt."]
pub struct GPIO0_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO0_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `GPIO1_INT_CLR` writer - Set this bit to clear the DEDIC_GPIO1_INT_RAW interrupt."]
pub struct GPIO1_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO1_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `GPIO2_INT_CLR` writer - Set this bit to clear the DEDIC_GPIO2_INT_RAW interrupt."]
pub struct GPIO2_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO2_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `GPIO3_INT_CLR` writer - Set this bit to clear the DEDIC_GPIO3_INT_RAW interrupt."]
pub struct GPIO3_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO3_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `GPIO4_INT_CLR` writer - Set this bit to clear the DEDIC_GPIO4_INT_RAW interrupt."]
pub struct GPIO4_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO4_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `GPIO5_INT_CLR` writer - Set this bit to clear the DEDIC_GPIO5_INT_RAW interrupt."]
pub struct GPIO5_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO5_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `GPIO6_INT_CLR` writer - Set this bit to clear the DEDIC_GPIO6_INT_RAW interrupt."]
pub struct GPIO6_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO6_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `GPIO7_INT_CLR` writer - Set this bit to clear the DEDIC_GPIO7_INT_RAW interrupt."]
pub struct GPIO7_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO7_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to clear the DEDIC_GPIO0_INT_RAW interrupt."]
    #[inline(always)]
    pub fn gpio0_int_clr(&mut self) -> GPIO0_INT_CLR_W {
        GPIO0_INT_CLR_W { w: self }
    }
    #[doc = "Bit 1 - Set this bit to clear the DEDIC_GPIO1_INT_RAW interrupt."]
    #[inline(always)]
    pub fn gpio1_int_clr(&mut self) -> GPIO1_INT_CLR_W {
        GPIO1_INT_CLR_W { w: self }
    }
    #[doc = "Bit 2 - Set this bit to clear the DEDIC_GPIO2_INT_RAW interrupt."]
    #[inline(always)]
    pub fn gpio2_int_clr(&mut self) -> GPIO2_INT_CLR_W {
        GPIO2_INT_CLR_W { w: self }
    }
    #[doc = "Bit 3 - Set this bit to clear the DEDIC_GPIO3_INT_RAW interrupt."]
    #[inline(always)]
    pub fn gpio3_int_clr(&mut self) -> GPIO3_INT_CLR_W {
        GPIO3_INT_CLR_W { w: self }
    }
    #[doc = "Bit 4 - Set this bit to clear the DEDIC_GPIO4_INT_RAW interrupt."]
    #[inline(always)]
    pub fn gpio4_int_clr(&mut self) -> GPIO4_INT_CLR_W {
        GPIO4_INT_CLR_W { w: self }
    }
    #[doc = "Bit 5 - Set this bit to clear the DEDIC_GPIO5_INT_RAW interrupt."]
    #[inline(always)]
    pub fn gpio5_int_clr(&mut self) -> GPIO5_INT_CLR_W {
        GPIO5_INT_CLR_W { w: self }
    }
    #[doc = "Bit 6 - Set this bit to clear the DEDIC_GPIO6_INT_RAW interrupt."]
    #[inline(always)]
    pub fn gpio6_int_clr(&mut self) -> GPIO6_INT_CLR_W {
        GPIO6_INT_CLR_W { w: self }
    }
    #[doc = "Bit 7 - Set this bit to clear the DEDIC_GPIO7_INT_RAW interrupt."]
    #[inline(always)]
    pub fn gpio7_int_clr(&mut self) -> GPIO7_INT_CLR_W {
        GPIO7_INT_CLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt clear bits\n\nThis register you can [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_clr]
(index.html) module"]
pub struct INTR_CLR_SPEC;
impl crate::RegisterSpec for INTR_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [intr_clr::W]
(W) writer structure"]
impl crate::Writable for INTR_CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTR_CLR to value 0"]
impl crate::Resettable for INTR_CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
