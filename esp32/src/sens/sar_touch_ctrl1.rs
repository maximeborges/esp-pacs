#[doc = "Register `SAR_TOUCH_CTRL1` reader"]
pub struct R(crate::R<SAR_TOUCH_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_TOUCH_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_TOUCH_CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_TOUCH_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_TOUCH_CTRL1` writer"]
pub struct W(crate::W<SAR_TOUCH_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_TOUCH_CTRL1_SPEC>;
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
impl From<crate::W<SAR_TOUCH_CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_TOUCH_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOUCH_MEAS_DELAY` reader - the meas length (in 8MHz)"]
pub type TOUCH_MEAS_DELAY_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TOUCH_MEAS_DELAY` writer - the meas length (in 8MHz)"]
pub type TOUCH_MEAS_DELAY_W<'a> =
    crate::FieldWriter<'a, u32, SAR_TOUCH_CTRL1_SPEC, u16, u16, 16, 0>;
#[doc = "Field `TOUCH_XPD_WAIT` reader - the waiting cycles (in 8MHz) between TOUCH_START and TOUCH_XPD"]
pub type TOUCH_XPD_WAIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TOUCH_XPD_WAIT` writer - the waiting cycles (in 8MHz) between TOUCH_START and TOUCH_XPD"]
pub type TOUCH_XPD_WAIT_W<'a> = crate::FieldWriter<'a, u32, SAR_TOUCH_CTRL1_SPEC, u8, u8, 8, 16>;
#[doc = "Field `TOUCH_OUT_SEL` reader - 1: when the counter is greater then the threshold the touch pad is considered as \"touched\" 0: when the counter is less than the threshold the touch pad is considered as \"touched\""]
pub type TOUCH_OUT_SEL_R = crate::BitReader<bool>;
#[doc = "Field `TOUCH_OUT_SEL` writer - 1: when the counter is greater then the threshold the touch pad is considered as \"touched\" 0: when the counter is less than the threshold the touch pad is considered as \"touched\""]
pub type TOUCH_OUT_SEL_W<'a> = crate::BitWriter<'a, u32, SAR_TOUCH_CTRL1_SPEC, bool, 24>;
#[doc = "Field `TOUCH_OUT_1EN` reader - 1: wakeup interrupt is generated if SET1 is \"touched\" 0: wakeup interrupt is generated only if SET1 & SET2 is both \"touched\""]
pub type TOUCH_OUT_1EN_R = crate::BitReader<bool>;
#[doc = "Field `TOUCH_OUT_1EN` writer - 1: wakeup interrupt is generated if SET1 is \"touched\" 0: wakeup interrupt is generated only if SET1 & SET2 is both \"touched\""]
pub type TOUCH_OUT_1EN_W<'a> = crate::BitWriter<'a, u32, SAR_TOUCH_CTRL1_SPEC, bool, 25>;
#[doc = "Field `XPD_HALL_FORCE` reader - 1: XPD HALL is controlled by SW. 0: XPD HALL is controlled by FSM in ULP-coprocessor"]
pub type XPD_HALL_FORCE_R = crate::BitReader<bool>;
#[doc = "Field `XPD_HALL_FORCE` writer - 1: XPD HALL is controlled by SW. 0: XPD HALL is controlled by FSM in ULP-coprocessor"]
pub type XPD_HALL_FORCE_W<'a> = crate::BitWriter<'a, u32, SAR_TOUCH_CTRL1_SPEC, bool, 26>;
#[doc = "Field `HALL_PHASE_FORCE` reader - 1: HALL PHASE is controlled by SW 0: HALL PHASE is controlled by FSM in ULP-coprocessor"]
pub type HALL_PHASE_FORCE_R = crate::BitReader<bool>;
#[doc = "Field `HALL_PHASE_FORCE` writer - 1: HALL PHASE is controlled by SW 0: HALL PHASE is controlled by FSM in ULP-coprocessor"]
pub type HALL_PHASE_FORCE_W<'a> = crate::BitWriter<'a, u32, SAR_TOUCH_CTRL1_SPEC, bool, 27>;
impl R {
    #[doc = "Bits 0:15 - the meas length (in 8MHz)"]
    #[inline(always)]
    pub fn touch_meas_delay(&self) -> TOUCH_MEAS_DELAY_R {
        TOUCH_MEAS_DELAY_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - the waiting cycles (in 8MHz) between TOUCH_START and TOUCH_XPD"]
    #[inline(always)]
    pub fn touch_xpd_wait(&self) -> TOUCH_XPD_WAIT_R {
        TOUCH_XPD_WAIT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - 1: when the counter is greater then the threshold the touch pad is considered as \"touched\" 0: when the counter is less than the threshold the touch pad is considered as \"touched\""]
    #[inline(always)]
    pub fn touch_out_sel(&self) -> TOUCH_OUT_SEL_R {
        TOUCH_OUT_SEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 1: wakeup interrupt is generated if SET1 is \"touched\" 0: wakeup interrupt is generated only if SET1 & SET2 is both \"touched\""]
    #[inline(always)]
    pub fn touch_out_1en(&self) -> TOUCH_OUT_1EN_R {
        TOUCH_OUT_1EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 1: XPD HALL is controlled by SW. 0: XPD HALL is controlled by FSM in ULP-coprocessor"]
    #[inline(always)]
    pub fn xpd_hall_force(&self) -> XPD_HALL_FORCE_R {
        XPD_HALL_FORCE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 1: HALL PHASE is controlled by SW 0: HALL PHASE is controlled by FSM in ULP-coprocessor"]
    #[inline(always)]
    pub fn hall_phase_force(&self) -> HALL_PHASE_FORCE_R {
        HALL_PHASE_FORCE_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - the meas length (in 8MHz)"]
    #[inline(always)]
    pub fn touch_meas_delay(&mut self) -> TOUCH_MEAS_DELAY_W {
        TOUCH_MEAS_DELAY_W::new(self)
    }
    #[doc = "Bits 16:23 - the waiting cycles (in 8MHz) between TOUCH_START and TOUCH_XPD"]
    #[inline(always)]
    pub fn touch_xpd_wait(&mut self) -> TOUCH_XPD_WAIT_W {
        TOUCH_XPD_WAIT_W::new(self)
    }
    #[doc = "Bit 24 - 1: when the counter is greater then the threshold the touch pad is considered as \"touched\" 0: when the counter is less than the threshold the touch pad is considered as \"touched\""]
    #[inline(always)]
    pub fn touch_out_sel(&mut self) -> TOUCH_OUT_SEL_W {
        TOUCH_OUT_SEL_W::new(self)
    }
    #[doc = "Bit 25 - 1: wakeup interrupt is generated if SET1 is \"touched\" 0: wakeup interrupt is generated only if SET1 & SET2 is both \"touched\""]
    #[inline(always)]
    pub fn touch_out_1en(&mut self) -> TOUCH_OUT_1EN_W {
        TOUCH_OUT_1EN_W::new(self)
    }
    #[doc = "Bit 26 - 1: XPD HALL is controlled by SW. 0: XPD HALL is controlled by FSM in ULP-coprocessor"]
    #[inline(always)]
    pub fn xpd_hall_force(&mut self) -> XPD_HALL_FORCE_W {
        XPD_HALL_FORCE_W::new(self)
    }
    #[doc = "Bit 27 - 1: HALL PHASE is controlled by SW 0: HALL PHASE is controlled by FSM in ULP-coprocessor"]
    #[inline(always)]
    pub fn hall_phase_force(&mut self) -> HALL_PHASE_FORCE_W {
        HALL_PHASE_FORCE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_touch_ctrl1](index.html) module"]
pub struct SAR_TOUCH_CTRL1_SPEC;
impl crate::RegisterSpec for SAR_TOUCH_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_touch_ctrl1::R](R) reader structure"]
impl crate::Readable for SAR_TOUCH_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_touch_ctrl1::W](W) writer structure"]
impl crate::Writable for SAR_TOUCH_CTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAR_TOUCH_CTRL1 to value 0x0204_1000"]
impl crate::Resettable for SAR_TOUCH_CTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0204_1000
    }
}
