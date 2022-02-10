#[doc = "Register `CHOPPER2_CFG` reader"]
pub struct R(crate::R<CHOPPER2_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHOPPER2_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHOPPER2_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHOPPER2_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHOPPER2_CFG` writer"]
pub struct W(crate::W<CHOPPER2_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHOPPER2_CFG_SPEC>;
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
impl From<crate::W<CHOPPER2_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHOPPER2_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHOPPER2_EN` reader - When set, carrier0 function is enabled. When cleared, carrier0 is bypassed"]
pub struct CHOPPER2_EN_R(crate::FieldReader<bool, bool>);
impl CHOPPER2_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHOPPER2_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHOPPER2_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHOPPER2_EN` writer - When set, carrier0 function is enabled. When cleared, carrier0 is bypassed"]
pub struct CHOPPER2_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CHOPPER2_EN_W<'a> {
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
#[doc = "Field `CHOPPER2_PRESCALE` reader - PWM carrier0 clock (PC_clk) prescale value. Period of PC_clk = period of PWM_clk * (PWM_CARRIER0_PRESCALE + 1)"]
pub struct CHOPPER2_PRESCALE_R(crate::FieldReader<u8, u8>);
impl CHOPPER2_PRESCALE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CHOPPER2_PRESCALE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHOPPER2_PRESCALE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHOPPER2_PRESCALE` writer - PWM carrier0 clock (PC_clk) prescale value. Period of PC_clk = period of PWM_clk * (PWM_CARRIER0_PRESCALE + 1)"]
pub struct CHOPPER2_PRESCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> CHOPPER2_PRESCALE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 1)) | ((value as u32 & 0x0f) << 1);
        self.w
    }
}
#[doc = "Field `CHOPPER2_DUTY` reader - carrier duty selection. Duty = PWM_CARRIER0_DUTY / 8"]
pub struct CHOPPER2_DUTY_R(crate::FieldReader<u8, u8>);
impl CHOPPER2_DUTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CHOPPER2_DUTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHOPPER2_DUTY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHOPPER2_DUTY` writer - carrier duty selection. Duty = PWM_CARRIER0_DUTY / 8"]
pub struct CHOPPER2_DUTY_W<'a> {
    w: &'a mut W,
}
impl<'a> CHOPPER2_DUTY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | ((value as u32 & 0x07) << 5);
        self.w
    }
}
#[doc = "Field `CHOPPER2_OSHTWTH` reader - width of the fist pulse in number of periods of the carrier"]
pub struct CHOPPER2_OSHTWTH_R(crate::FieldReader<u8, u8>);
impl CHOPPER2_OSHTWTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CHOPPER2_OSHTWTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHOPPER2_OSHTWTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHOPPER2_OSHTWTH` writer - width of the fist pulse in number of periods of the carrier"]
pub struct CHOPPER2_OSHTWTH_W<'a> {
    w: &'a mut W,
}
impl<'a> CHOPPER2_OSHTWTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `CHOPPER2_OUT_INVERT` reader - when set, invert the output of PWM2A and PWM2B for this submodule"]
pub struct CHOPPER2_OUT_INVERT_R(crate::FieldReader<bool, bool>);
impl CHOPPER2_OUT_INVERT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHOPPER2_OUT_INVERT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHOPPER2_OUT_INVERT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHOPPER2_OUT_INVERT` writer - when set, invert the output of PWM2A and PWM2B for this submodule"]
pub struct CHOPPER2_OUT_INVERT_W<'a> {
    w: &'a mut W,
}
impl<'a> CHOPPER2_OUT_INVERT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `CHOPPER2_IN_INVERT` reader - when set, invert the input of PWM2A and PWM2B for this submodule"]
pub struct CHOPPER2_IN_INVERT_R(crate::FieldReader<bool, bool>);
impl CHOPPER2_IN_INVERT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHOPPER2_IN_INVERT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHOPPER2_IN_INVERT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHOPPER2_IN_INVERT` writer - when set, invert the input of PWM2A and PWM2B for this submodule"]
pub struct CHOPPER2_IN_INVERT_W<'a> {
    w: &'a mut W,
}
impl<'a> CHOPPER2_IN_INVERT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - When set, carrier0 function is enabled. When cleared, carrier0 is bypassed"]
    #[inline(always)]
    pub fn chopper2_en(&self) -> CHOPPER2_EN_R {
        CHOPPER2_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:4 - PWM carrier0 clock (PC_clk) prescale value. Period of PC_clk = period of PWM_clk * (PWM_CARRIER0_PRESCALE + 1)"]
    #[inline(always)]
    pub fn chopper2_prescale(&self) -> CHOPPER2_PRESCALE_R {
        CHOPPER2_PRESCALE_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bits 5:7 - carrier duty selection. Duty = PWM_CARRIER0_DUTY / 8"]
    #[inline(always)]
    pub fn chopper2_duty(&self) -> CHOPPER2_DUTY_R {
        CHOPPER2_DUTY_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bits 8:11 - width of the fist pulse in number of periods of the carrier"]
    #[inline(always)]
    pub fn chopper2_oshtwth(&self) -> CHOPPER2_OSHTWTH_R {
        CHOPPER2_OSHTWTH_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - when set, invert the output of PWM2A and PWM2B for this submodule"]
    #[inline(always)]
    pub fn chopper2_out_invert(&self) -> CHOPPER2_OUT_INVERT_R {
        CHOPPER2_OUT_INVERT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - when set, invert the input of PWM2A and PWM2B for this submodule"]
    #[inline(always)]
    pub fn chopper2_in_invert(&self) -> CHOPPER2_IN_INVERT_R {
        CHOPPER2_IN_INVERT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When set, carrier0 function is enabled. When cleared, carrier0 is bypassed"]
    #[inline(always)]
    pub fn chopper2_en(&mut self) -> CHOPPER2_EN_W {
        CHOPPER2_EN_W { w: self }
    }
    #[doc = "Bits 1:4 - PWM carrier0 clock (PC_clk) prescale value. Period of PC_clk = period of PWM_clk * (PWM_CARRIER0_PRESCALE + 1)"]
    #[inline(always)]
    pub fn chopper2_prescale(&mut self) -> CHOPPER2_PRESCALE_W {
        CHOPPER2_PRESCALE_W { w: self }
    }
    #[doc = "Bits 5:7 - carrier duty selection. Duty = PWM_CARRIER0_DUTY / 8"]
    #[inline(always)]
    pub fn chopper2_duty(&mut self) -> CHOPPER2_DUTY_W {
        CHOPPER2_DUTY_W { w: self }
    }
    #[doc = "Bits 8:11 - width of the fist pulse in number of periods of the carrier"]
    #[inline(always)]
    pub fn chopper2_oshtwth(&mut self) -> CHOPPER2_OSHTWTH_W {
        CHOPPER2_OSHTWTH_W { w: self }
    }
    #[doc = "Bit 12 - when set, invert the output of PWM2A and PWM2B for this submodule"]
    #[inline(always)]
    pub fn chopper2_out_invert(&mut self) -> CHOPPER2_OUT_INVERT_W {
        CHOPPER2_OUT_INVERT_W { w: self }
    }
    #[doc = "Bit 13 - when set, invert the input of PWM2A and PWM2B for this submodule"]
    #[inline(always)]
    pub fn chopper2_in_invert(&mut self) -> CHOPPER2_IN_INVERT_W {
        CHOPPER2_IN_INVERT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Carrier enable and configuratoin\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chopper2_cfg]
(index.html) module"]
pub struct CHOPPER2_CFG_SPEC;
impl crate::RegisterSpec for CHOPPER2_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chopper2_cfg::R]
(R) reader structure"]
impl crate::Readable for CHOPPER2_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chopper2_cfg::W]
(W) writer structure"]
impl crate::Writable for CHOPPER2_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHOPPER2_CFG to value 0"]
impl crate::Resettable for CHOPPER2_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
