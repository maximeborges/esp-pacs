#[doc = "Register `ULP_CP_TIMER` reader"]
pub struct R(crate::R<ULP_CP_TIMER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ULP_CP_TIMER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ULP_CP_TIMER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ULP_CP_TIMER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ULP_CP_TIMER` writer"]
pub struct W(crate::W<ULP_CP_TIMER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ULP_CP_TIMER_SPEC>;
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
impl From<crate::W<ULP_CP_TIMER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ULP_CP_TIMER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ULP_CP_PC_INIT` reader - ULP coprocessor PC initial address"]
pub type ULP_CP_PC_INIT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ULP_CP_PC_INIT` writer - ULP coprocessor PC initial address"]
pub type ULP_CP_PC_INIT_W<'a> = crate::FieldWriter<'a, u32, ULP_CP_TIMER_SPEC, u16, u16, 11, 0>;
#[doc = "Field `ULP_CP_GPIO_WAKEUP_ENA` reader - Enable the option of ULP coprocessor woken up by RTC GPIO"]
pub type ULP_CP_GPIO_WAKEUP_ENA_R = crate::BitReader<bool>;
#[doc = "Field `ULP_CP_GPIO_WAKEUP_ENA` writer - Enable the option of ULP coprocessor woken up by RTC GPIO"]
pub type ULP_CP_GPIO_WAKEUP_ENA_W<'a> = crate::BitWriter<'a, u32, ULP_CP_TIMER_SPEC, bool, 29>;
#[doc = "Field `ULP_CP_GPIO_WAKEUP_CLR` writer - Disable the option of ULP coprocessor woken up by RTC GPIO"]
pub type ULP_CP_GPIO_WAKEUP_CLR_W<'a> = crate::BitWriter<'a, u32, ULP_CP_TIMER_SPEC, bool, 30>;
#[doc = "Field `ULP_CP_SLP_TIMER_EN` reader - ULP coprocessor timer enable bit. 0: Disable hardware Timer. 1: Enable hardware timer"]
pub type ULP_CP_SLP_TIMER_EN_R = crate::BitReader<bool>;
#[doc = "Field `ULP_CP_SLP_TIMER_EN` writer - ULP coprocessor timer enable bit. 0: Disable hardware Timer. 1: Enable hardware timer"]
pub type ULP_CP_SLP_TIMER_EN_W<'a> = crate::BitWriter<'a, u32, ULP_CP_TIMER_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 0:10 - ULP coprocessor PC initial address"]
    #[inline(always)]
    pub fn ulp_cp_pc_init(&self) -> ULP_CP_PC_INIT_R {
        ULP_CP_PC_INIT_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 29 - Enable the option of ULP coprocessor woken up by RTC GPIO"]
    #[inline(always)]
    pub fn ulp_cp_gpio_wakeup_ena(&self) -> ULP_CP_GPIO_WAKEUP_ENA_R {
        ULP_CP_GPIO_WAKEUP_ENA_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - ULP coprocessor timer enable bit. 0: Disable hardware Timer. 1: Enable hardware timer"]
    #[inline(always)]
    pub fn ulp_cp_slp_timer_en(&self) -> ULP_CP_SLP_TIMER_EN_R {
        ULP_CP_SLP_TIMER_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - ULP coprocessor PC initial address"]
    #[inline(always)]
    pub fn ulp_cp_pc_init(&mut self) -> ULP_CP_PC_INIT_W {
        ULP_CP_PC_INIT_W::new(self)
    }
    #[doc = "Bit 29 - Enable the option of ULP coprocessor woken up by RTC GPIO"]
    #[inline(always)]
    pub fn ulp_cp_gpio_wakeup_ena(&mut self) -> ULP_CP_GPIO_WAKEUP_ENA_W {
        ULP_CP_GPIO_WAKEUP_ENA_W::new(self)
    }
    #[doc = "Bit 30 - Disable the option of ULP coprocessor woken up by RTC GPIO"]
    #[inline(always)]
    pub fn ulp_cp_gpio_wakeup_clr(&mut self) -> ULP_CP_GPIO_WAKEUP_CLR_W {
        ULP_CP_GPIO_WAKEUP_CLR_W::new(self)
    }
    #[doc = "Bit 31 - ULP coprocessor timer enable bit. 0: Disable hardware Timer. 1: Enable hardware timer"]
    #[inline(always)]
    pub fn ulp_cp_slp_timer_en(&mut self) -> ULP_CP_SLP_TIMER_EN_W {
        ULP_CP_SLP_TIMER_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configure coprocessor timer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ulp_cp_timer](index.html) module"]
pub struct ULP_CP_TIMER_SPEC;
impl crate::RegisterSpec for ULP_CP_TIMER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ulp_cp_timer::R](R) reader structure"]
impl crate::Readable for ULP_CP_TIMER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ulp_cp_timer::W](W) writer structure"]
impl crate::Writable for ULP_CP_TIMER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ULP_CP_TIMER to value 0"]
impl crate::Resettable for ULP_CP_TIMER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
