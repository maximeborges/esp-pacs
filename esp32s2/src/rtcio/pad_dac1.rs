#[doc = "Register `PAD_DAC1` reader"]
pub struct R(crate::R<PAD_DAC1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAD_DAC1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAD_DAC1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAD_DAC1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAD_DAC1` writer"]
pub struct W(crate::W<PAD_DAC1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAD_DAC1_SPEC>;
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
impl From<crate::W<PAD_DAC1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAD_DAC1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDAC1_DAC` reader - Configure DAC_1 output when RTCIO_PDAC1_DAC_XPD_FORCE is set to 1."]
pub type PDAC1_DAC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PDAC1_DAC` writer - Configure DAC_1 output when RTCIO_PDAC1_DAC_XPD_FORCE is set to 1."]
pub type PDAC1_DAC_W<'a> = crate::FieldWriter<'a, u32, PAD_DAC1_SPEC, u8, u8, 8, 3>;
#[doc = "Field `PDAC1_XPD_DAC` reader - When RTCIO_PDAC1_DAC_XPD_FORCE is set to 1, 1: enable DAC_1 output. 0: disable DAC_1 output."]
pub type PDAC1_XPD_DAC_R = crate::BitReader<bool>;
#[doc = "Field `PDAC1_XPD_DAC` writer - When RTCIO_PDAC1_DAC_XPD_FORCE is set to 1, 1: enable DAC_1 output. 0: disable DAC_1 output."]
pub type PDAC1_XPD_DAC_W<'a> = crate::BitWriter<'a, u32, PAD_DAC1_SPEC, bool, 11>;
#[doc = "Field `PDAC1_DAC_XPD_FORCE` reader - 1: use RTCIO_PDAC1_XPD_DAC to control DAC_1 output. 0: use SAR ADC FSM to control DAC_1 output."]
pub type PDAC1_DAC_XPD_FORCE_R = crate::BitReader<bool>;
#[doc = "Field `PDAC1_DAC_XPD_FORCE` writer - 1: use RTCIO_PDAC1_XPD_DAC to control DAC_1 output. 0: use SAR ADC FSM to control DAC_1 output."]
pub type PDAC1_DAC_XPD_FORCE_W<'a> = crate::BitWriter<'a, u32, PAD_DAC1_SPEC, bool, 12>;
#[doc = "Field `PDAC1_FUN_IE` reader - Input enable in normal execution."]
pub type PDAC1_FUN_IE_R = crate::BitReader<bool>;
#[doc = "Field `PDAC1_FUN_IE` writer - Input enable in normal execution."]
pub type PDAC1_FUN_IE_W<'a> = crate::BitWriter<'a, u32, PAD_DAC1_SPEC, bool, 13>;
#[doc = "Field `PDAC1_SLP_OE` reader - Output enable in sleep mode"]
pub type PDAC1_SLP_OE_R = crate::BitReader<bool>;
#[doc = "Field `PDAC1_SLP_OE` writer - Output enable in sleep mode"]
pub type PDAC1_SLP_OE_W<'a> = crate::BitWriter<'a, u32, PAD_DAC1_SPEC, bool, 14>;
#[doc = "Field `PDAC1_SLP_IE` reader - Input enable in sleep mode"]
pub type PDAC1_SLP_IE_R = crate::BitReader<bool>;
#[doc = "Field `PDAC1_SLP_IE` writer - Input enable in sleep mode"]
pub type PDAC1_SLP_IE_W<'a> = crate::BitWriter<'a, u32, PAD_DAC1_SPEC, bool, 15>;
#[doc = "Field `PDAC1_SLP_SEL` reader - 1: enable sleep mode. 0: no sleep mode"]
pub type PDAC1_SLP_SEL_R = crate::BitReader<bool>;
#[doc = "Field `PDAC1_SLP_SEL` writer - 1: enable sleep mode. 0: no sleep mode"]
pub type PDAC1_SLP_SEL_W<'a> = crate::BitWriter<'a, u32, PAD_DAC1_SPEC, bool, 16>;
#[doc = "Field `PDAC1_FUN_SEL` reader - DAC_1 function selection."]
pub type PDAC1_FUN_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PDAC1_FUN_SEL` writer - DAC_1 function selection."]
pub type PDAC1_FUN_SEL_W<'a> = crate::FieldWriter<'a, u32, PAD_DAC1_SPEC, u8, u8, 2, 17>;
#[doc = "Field `PDAC1_MUX_SEL` reader - 1: use RTC GPIO. 0: use digital GPIO"]
pub type PDAC1_MUX_SEL_R = crate::BitReader<bool>;
#[doc = "Field `PDAC1_MUX_SEL` writer - 1: use RTC GPIO. 0: use digital GPIO"]
pub type PDAC1_MUX_SEL_W<'a> = crate::BitWriter<'a, u32, PAD_DAC1_SPEC, bool, 19>;
#[doc = "Field `PDAC1_RUE` reader - Pull-down enable of the pad. 1: internal pull-down enabled. 0: internal pull-down disabled."]
pub type PDAC1_RUE_R = crate::BitReader<bool>;
#[doc = "Field `PDAC1_RUE` writer - Pull-down enable of the pad. 1: internal pull-down enabled. 0: internal pull-down disabled."]
pub type PDAC1_RUE_W<'a> = crate::BitWriter<'a, u32, PAD_DAC1_SPEC, bool, 27>;
#[doc = "Field `PDAC1_RDE` reader - Pull-up enable of the pad. 1: internal pull-up enabled. 0: internal pull-up disabled."]
pub type PDAC1_RDE_R = crate::BitReader<bool>;
#[doc = "Field `PDAC1_RDE` writer - Pull-up enable of the pad. 1: internal pull-up enabled. 0: internal pull-up disabled."]
pub type PDAC1_RDE_W<'a> = crate::BitWriter<'a, u32, PAD_DAC1_SPEC, bool, 28>;
#[doc = "Field `PDAC1_DRV` reader - Select the drive strength of the pad. 0: ~5 mA. 1: ~10 mA. 2: ~20 mA. 3: ~40 mA."]
pub type PDAC1_DRV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PDAC1_DRV` writer - Select the drive strength of the pad. 0: ~5 mA. 1: ~10 mA. 2: ~20 mA. 3: ~40 mA."]
pub type PDAC1_DRV_W<'a> = crate::FieldWriter<'a, u32, PAD_DAC1_SPEC, u8, u8, 2, 29>;
impl R {
    #[doc = "Bits 3:10 - Configure DAC_1 output when RTCIO_PDAC1_DAC_XPD_FORCE is set to 1."]
    #[inline(always)]
    pub fn pdac1_dac(&self) -> PDAC1_DAC_R {
        PDAC1_DAC_R::new(((self.bits >> 3) & 0xff) as u8)
    }
    #[doc = "Bit 11 - When RTCIO_PDAC1_DAC_XPD_FORCE is set to 1, 1: enable DAC_1 output. 0: disable DAC_1 output."]
    #[inline(always)]
    pub fn pdac1_xpd_dac(&self) -> PDAC1_XPD_DAC_R {
        PDAC1_XPD_DAC_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 1: use RTCIO_PDAC1_XPD_DAC to control DAC_1 output. 0: use SAR ADC FSM to control DAC_1 output."]
    #[inline(always)]
    pub fn pdac1_dac_xpd_force(&self) -> PDAC1_DAC_XPD_FORCE_R {
        PDAC1_DAC_XPD_FORCE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Input enable in normal execution."]
    #[inline(always)]
    pub fn pdac1_fun_ie(&self) -> PDAC1_FUN_IE_R {
        PDAC1_FUN_IE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Output enable in sleep mode"]
    #[inline(always)]
    pub fn pdac1_slp_oe(&self) -> PDAC1_SLP_OE_R {
        PDAC1_SLP_OE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Input enable in sleep mode"]
    #[inline(always)]
    pub fn pdac1_slp_ie(&self) -> PDAC1_SLP_IE_R {
        PDAC1_SLP_IE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 1: enable sleep mode. 0: no sleep mode"]
    #[inline(always)]
    pub fn pdac1_slp_sel(&self) -> PDAC1_SLP_SEL_R {
        PDAC1_SLP_SEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - DAC_1 function selection."]
    #[inline(always)]
    pub fn pdac1_fun_sel(&self) -> PDAC1_FUN_SEL_R {
        PDAC1_FUN_SEL_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - 1: use RTC GPIO. 0: use digital GPIO"]
    #[inline(always)]
    pub fn pdac1_mux_sel(&self) -> PDAC1_MUX_SEL_R {
        PDAC1_MUX_SEL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 27 - Pull-down enable of the pad. 1: internal pull-down enabled. 0: internal pull-down disabled."]
    #[inline(always)]
    pub fn pdac1_rue(&self) -> PDAC1_RUE_R {
        PDAC1_RUE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Pull-up enable of the pad. 1: internal pull-up enabled. 0: internal pull-up disabled."]
    #[inline(always)]
    pub fn pdac1_rde(&self) -> PDAC1_RDE_R {
        PDAC1_RDE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:30 - Select the drive strength of the pad. 0: ~5 mA. 1: ~10 mA. 2: ~20 mA. 3: ~40 mA."]
    #[inline(always)]
    pub fn pdac1_drv(&self) -> PDAC1_DRV_R {
        PDAC1_DRV_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 3:10 - Configure DAC_1 output when RTCIO_PDAC1_DAC_XPD_FORCE is set to 1."]
    #[inline(always)]
    pub fn pdac1_dac(&mut self) -> PDAC1_DAC_W {
        PDAC1_DAC_W::new(self)
    }
    #[doc = "Bit 11 - When RTCIO_PDAC1_DAC_XPD_FORCE is set to 1, 1: enable DAC_1 output. 0: disable DAC_1 output."]
    #[inline(always)]
    pub fn pdac1_xpd_dac(&mut self) -> PDAC1_XPD_DAC_W {
        PDAC1_XPD_DAC_W::new(self)
    }
    #[doc = "Bit 12 - 1: use RTCIO_PDAC1_XPD_DAC to control DAC_1 output. 0: use SAR ADC FSM to control DAC_1 output."]
    #[inline(always)]
    pub fn pdac1_dac_xpd_force(&mut self) -> PDAC1_DAC_XPD_FORCE_W {
        PDAC1_DAC_XPD_FORCE_W::new(self)
    }
    #[doc = "Bit 13 - Input enable in normal execution."]
    #[inline(always)]
    pub fn pdac1_fun_ie(&mut self) -> PDAC1_FUN_IE_W {
        PDAC1_FUN_IE_W::new(self)
    }
    #[doc = "Bit 14 - Output enable in sleep mode"]
    #[inline(always)]
    pub fn pdac1_slp_oe(&mut self) -> PDAC1_SLP_OE_W {
        PDAC1_SLP_OE_W::new(self)
    }
    #[doc = "Bit 15 - Input enable in sleep mode"]
    #[inline(always)]
    pub fn pdac1_slp_ie(&mut self) -> PDAC1_SLP_IE_W {
        PDAC1_SLP_IE_W::new(self)
    }
    #[doc = "Bit 16 - 1: enable sleep mode. 0: no sleep mode"]
    #[inline(always)]
    pub fn pdac1_slp_sel(&mut self) -> PDAC1_SLP_SEL_W {
        PDAC1_SLP_SEL_W::new(self)
    }
    #[doc = "Bits 17:18 - DAC_1 function selection."]
    #[inline(always)]
    pub fn pdac1_fun_sel(&mut self) -> PDAC1_FUN_SEL_W {
        PDAC1_FUN_SEL_W::new(self)
    }
    #[doc = "Bit 19 - 1: use RTC GPIO. 0: use digital GPIO"]
    #[inline(always)]
    pub fn pdac1_mux_sel(&mut self) -> PDAC1_MUX_SEL_W {
        PDAC1_MUX_SEL_W::new(self)
    }
    #[doc = "Bit 27 - Pull-down enable of the pad. 1: internal pull-down enabled. 0: internal pull-down disabled."]
    #[inline(always)]
    pub fn pdac1_rue(&mut self) -> PDAC1_RUE_W {
        PDAC1_RUE_W::new(self)
    }
    #[doc = "Bit 28 - Pull-up enable of the pad. 1: internal pull-up enabled. 0: internal pull-up disabled."]
    #[inline(always)]
    pub fn pdac1_rde(&mut self) -> PDAC1_RDE_W {
        PDAC1_RDE_W::new(self)
    }
    #[doc = "Bits 29:30 - Select the drive strength of the pad. 0: ~5 mA. 1: ~10 mA. 2: ~20 mA. 3: ~40 mA."]
    #[inline(always)]
    pub fn pdac1_drv(&mut self) -> PDAC1_DRV_W {
        PDAC1_DRV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC1 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pad_dac1](index.html) module"]
pub struct PAD_DAC1_SPEC;
impl crate::RegisterSpec for PAD_DAC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pad_dac1::R](R) reader structure"]
impl crate::Readable for PAD_DAC1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pad_dac1::W](W) writer structure"]
impl crate::Writable for PAD_DAC1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PAD_DAC1 to value 0x4000_0000"]
impl crate::Resettable for PAD_DAC1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4000_0000
    }
}
