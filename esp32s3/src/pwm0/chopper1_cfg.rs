#[doc = "Register `CHOPPER1_CFG` reader"]
pub struct R(crate::R<CHOPPER1_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHOPPER1_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHOPPER1_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHOPPER1_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHOPPER1_CFG` writer"]
pub struct W(crate::W<CHOPPER1_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHOPPER1_CFG_SPEC>;
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
impl From<crate::W<CHOPPER1_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHOPPER1_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHOPPER1_EN` reader - When set, carrier0 function is enabled. When cleared, carrier0 is bypassed"]
pub struct CHOPPER1_EN_R(crate::FieldReader<bool, bool>);
impl CHOPPER1_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHOPPER1_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHOPPER1_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHOPPER1_EN` writer - When set, carrier0 function is enabled. When cleared, carrier0 is bypassed"]
pub struct CHOPPER1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CHOPPER1_EN_W<'a> {
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
#[doc = "Field `CHOPPER1_PRESCALE` reader - PWM carrier0 clock (PC_clk) prescale value. Period of PC_clk = period of PWM_clk * (PWM_CARRIER0_PRESCALE + 1)"]
pub struct CHOPPER1_PRESCALE_R(crate::FieldReader<u8, u8>);
impl CHOPPER1_PRESCALE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CHOPPER1_PRESCALE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHOPPER1_PRESCALE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHOPPER1_PRESCALE` writer - PWM carrier0 clock (PC_clk) prescale value. Period of PC_clk = period of PWM_clk * (PWM_CARRIER0_PRESCALE + 1)"]
pub struct CHOPPER1_PRESCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> CHOPPER1_PRESCALE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 1)) | ((value as u32 & 0x0f) << 1);
        self.w
    }
}
#[doc = "Field `CHOPPER1_DUTY` reader - carrier duty selection. Duty = PWM_CARRIER0_DUTY / 8"]
pub struct CHOPPER1_DUTY_R(crate::FieldReader<u8, u8>);
impl CHOPPER1_DUTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CHOPPER1_DUTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHOPPER1_DUTY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHOPPER1_DUTY` writer - carrier duty selection. Duty = PWM_CARRIER0_DUTY / 8"]
pub struct CHOPPER1_DUTY_W<'a> {
    w: &'a mut W,
}
impl<'a> CHOPPER1_DUTY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | ((value as u32 & 0x07) << 5);
        self.w
    }
}
#[doc = "Field `CHOPPER1_OSHTWTH` reader - width of the fist pulse in number of periods of the carrier"]
pub struct CHOPPER1_OSHTWTH_R(crate::FieldReader<u8, u8>);
impl CHOPPER1_OSHTWTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CHOPPER1_OSHTWTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHOPPER1_OSHTWTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHOPPER1_OSHTWTH` writer - width of the fist pulse in number of periods of the carrier"]
pub struct CHOPPER1_OSHTWTH_W<'a> {
    w: &'a mut W,
}
impl<'a> CHOPPER1_OSHTWTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `CHOPPER1_OUT_INVERT` reader - when set, invert the output of PWM1A and PWM1B for this submodule"]
pub struct CHOPPER1_OUT_INVERT_R(crate::FieldReader<bool, bool>);
impl CHOPPER1_OUT_INVERT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHOPPER1_OUT_INVERT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHOPPER1_OUT_INVERT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHOPPER1_OUT_INVERT` writer - when set, invert the output of PWM1A and PWM1B for this submodule"]
pub struct CHOPPER1_OUT_INVERT_W<'a> {
    w: &'a mut W,
}
impl<'a> CHOPPER1_OUT_INVERT_W<'a> {
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
#[doc = "Field `CHOPPER1_IN_INVERT` reader - when set, invert the input of PWM1A and PWM1B for this submodule"]
pub struct CHOPPER1_IN_INVERT_R(crate::FieldReader<bool, bool>);
impl CHOPPER1_IN_INVERT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHOPPER1_IN_INVERT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHOPPER1_IN_INVERT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHOPPER1_IN_INVERT` writer - when set, invert the input of PWM1A and PWM1B for this submodule"]
pub struct CHOPPER1_IN_INVERT_W<'a> {
    w: &'a mut W,
}
impl<'a> CHOPPER1_IN_INVERT_W<'a> {
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
    pub fn chopper1_en(&self) -> CHOPPER1_EN_R {
        CHOPPER1_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:4 - PWM carrier0 clock (PC_clk) prescale value. Period of PC_clk = period of PWM_clk * (PWM_CARRIER0_PRESCALE + 1)"]
    #[inline(always)]
    pub fn chopper1_prescale(&self) -> CHOPPER1_PRESCALE_R {
        CHOPPER1_PRESCALE_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bits 5:7 - carrier duty selection. Duty = PWM_CARRIER0_DUTY / 8"]
    #[inline(always)]
    pub fn chopper1_duty(&self) -> CHOPPER1_DUTY_R {
        CHOPPER1_DUTY_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bits 8:11 - width of the fist pulse in number of periods of the carrier"]
    #[inline(always)]
    pub fn chopper1_oshtwth(&self) -> CHOPPER1_OSHTWTH_R {
        CHOPPER1_OSHTWTH_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - when set, invert the output of PWM1A and PWM1B for this submodule"]
    #[inline(always)]
    pub fn chopper1_out_invert(&self) -> CHOPPER1_OUT_INVERT_R {
        CHOPPER1_OUT_INVERT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - when set, invert the input of PWM1A and PWM1B for this submodule"]
    #[inline(always)]
    pub fn chopper1_in_invert(&self) -> CHOPPER1_IN_INVERT_R {
        CHOPPER1_IN_INVERT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When set, carrier0 function is enabled. When cleared, carrier0 is bypassed"]
    #[inline(always)]
    pub fn chopper1_en(&mut self) -> CHOPPER1_EN_W {
        CHOPPER1_EN_W { w: self }
    }
    #[doc = "Bits 1:4 - PWM carrier0 clock (PC_clk) prescale value. Period of PC_clk = period of PWM_clk * (PWM_CARRIER0_PRESCALE + 1)"]
    #[inline(always)]
    pub fn chopper1_prescale(&mut self) -> CHOPPER1_PRESCALE_W {
        CHOPPER1_PRESCALE_W { w: self }
    }
    #[doc = "Bits 5:7 - carrier duty selection. Duty = PWM_CARRIER0_DUTY / 8"]
    #[inline(always)]
    pub fn chopper1_duty(&mut self) -> CHOPPER1_DUTY_W {
        CHOPPER1_DUTY_W { w: self }
    }
    #[doc = "Bits 8:11 - width of the fist pulse in number of periods of the carrier"]
    #[inline(always)]
    pub fn chopper1_oshtwth(&mut self) -> CHOPPER1_OSHTWTH_W {
        CHOPPER1_OSHTWTH_W { w: self }
    }
    #[doc = "Bit 12 - when set, invert the output of PWM1A and PWM1B for this submodule"]
    #[inline(always)]
    pub fn chopper1_out_invert(&mut self) -> CHOPPER1_OUT_INVERT_W {
        CHOPPER1_OUT_INVERT_W { w: self }
    }
    #[doc = "Bit 13 - when set, invert the input of PWM1A and PWM1B for this submodule"]
    #[inline(always)]
    pub fn chopper1_in_invert(&mut self) -> CHOPPER1_IN_INVERT_W {
        CHOPPER1_IN_INVERT_W { w: self }
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
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chopper1_cfg]
(index.html) module"]
pub struct CHOPPER1_CFG_SPEC;
impl crate::RegisterSpec for CHOPPER1_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chopper1_cfg::R]
(R) reader structure"]
impl crate::Readable for CHOPPER1_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chopper1_cfg::W]
(W) writer structure"]
impl crate::Writable for CHOPPER1_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHOPPER1_CFG to value 0"]
impl crate::Resettable for CHOPPER1_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
