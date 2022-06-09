#[doc = "Register `APP_INT_SET` writer"]
pub struct W(crate::W<APP_INT_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APP_INT_SET_SPEC>;
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
impl From<crate::W<APP_INT_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APP_INT_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `APP_CTRL0_INT_SET` writer - This bit is software interrupt trigger source of UHCI_APP_CTRL0_INT."]
pub type APP_CTRL0_INT_SET_W<'a> = crate::BitWriter<'a, u32, APP_INT_SET_SPEC, bool, 0>;
#[doc = "Field `APP_CTRL1_INT_SET` writer - This bit is software interrupt trigger source of UHCI_APP_CTRL1_INT."]
pub type APP_CTRL1_INT_SET_W<'a> = crate::BitWriter<'a, u32, APP_INT_SET_SPEC, bool, 1>;
impl W {
    #[doc = "Bit 0 - This bit is software interrupt trigger source of UHCI_APP_CTRL0_INT."]
    #[inline(always)]
    pub fn app_ctrl0_int_set(&mut self) -> APP_CTRL0_INT_SET_W {
        APP_CTRL0_INT_SET_W::new(self)
    }
    #[doc = "Bit 1 - This bit is software interrupt trigger source of UHCI_APP_CTRL1_INT."]
    #[inline(always)]
    pub fn app_ctrl1_int_set(&mut self) -> APP_CTRL1_INT_SET_W {
        APP_CTRL1_INT_SET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software interrupt trigger source\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [app_int_set](index.html) module"]
pub struct APP_INT_SET_SPEC;
impl crate::RegisterSpec for APP_INT_SET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [app_int_set::W](W) writer structure"]
impl crate::Writable for APP_INT_SET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APP_INT_SET to value 0"]
impl crate::Resettable for APP_INT_SET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
