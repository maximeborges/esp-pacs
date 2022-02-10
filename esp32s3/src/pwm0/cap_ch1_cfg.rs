#[doc = "Register `CAP_CH1_CFG` reader"]
pub struct R(crate::R<CAP_CH1_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAP_CH1_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAP_CH1_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAP_CH1_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAP_CH1_CFG` writer"]
pub struct W(crate::W<CAP_CH1_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAP_CH1_CFG_SPEC>;
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
impl From<crate::W<CAP_CH1_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAP_CH1_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAP1_EN` reader - When set, capture on channel 2 is enabled"]
pub struct CAP1_EN_R(crate::FieldReader<bool, bool>);
impl CAP1_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CAP1_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAP1_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAP1_EN` writer - When set, capture on channel 2 is enabled"]
pub struct CAP1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP1_EN_W<'a> {
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
#[doc = "Field `CAP1_MODE` reader - Edge of capture on channel 1 after prescaling. When bit0 is set to 1: enable capture on the negative edge, When bit1 is set to 1: enable capture on the positive edge."]
pub struct CAP1_MODE_R(crate::FieldReader<u8, u8>);
impl CAP1_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CAP1_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAP1_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAP1_MODE` writer - Edge of capture on channel 1 after prescaling. When bit0 is set to 1: enable capture on the negative edge, When bit1 is set to 1: enable capture on the positive edge."]
pub struct CAP1_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP1_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | ((value as u32 & 0x03) << 1);
        self.w
    }
}
#[doc = "Field `CAP1_PRESCALE` reader - Value of prescaling on possitive edge of CAP1. Prescale value = PWM_CAP1_PRESCALE + 1"]
pub struct CAP1_PRESCALE_R(crate::FieldReader<u8, u8>);
impl CAP1_PRESCALE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CAP1_PRESCALE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAP1_PRESCALE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAP1_PRESCALE` writer - Value of prescaling on possitive edge of CAP1. Prescale value = PWM_CAP1_PRESCALE + 1"]
pub struct CAP1_PRESCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP1_PRESCALE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 3)) | ((value as u32 & 0xff) << 3);
        self.w
    }
}
#[doc = "Field `CAP1_IN_INVERT` reader - when set, CAP1 form GPIO matrix is inverted before prescale"]
pub struct CAP1_IN_INVERT_R(crate::FieldReader<bool, bool>);
impl CAP1_IN_INVERT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CAP1_IN_INVERT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAP1_IN_INVERT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAP1_IN_INVERT` writer - when set, CAP1 form GPIO matrix is inverted before prescale"]
pub struct CAP1_IN_INVERT_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP1_IN_INVERT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `CAP1_SW` writer - Write 1 will trigger a software forced capture on channel 1"]
pub struct CAP1_SW_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP1_SW_W<'a> {
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
impl R {
    #[doc = "Bit 0 - When set, capture on channel 2 is enabled"]
    #[inline(always)]
    pub fn cap1_en(&self) -> CAP1_EN_R {
        CAP1_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - Edge of capture on channel 1 after prescaling. When bit0 is set to 1: enable capture on the negative edge, When bit1 is set to 1: enable capture on the positive edge."]
    #[inline(always)]
    pub fn cap1_mode(&self) -> CAP1_MODE_R {
        CAP1_MODE_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bits 3:10 - Value of prescaling on possitive edge of CAP1. Prescale value = PWM_CAP1_PRESCALE + 1"]
    #[inline(always)]
    pub fn cap1_prescale(&self) -> CAP1_PRESCALE_R {
        CAP1_PRESCALE_R::new(((self.bits >> 3) & 0xff) as u8)
    }
    #[doc = "Bit 11 - when set, CAP1 form GPIO matrix is inverted before prescale"]
    #[inline(always)]
    pub fn cap1_in_invert(&self) -> CAP1_IN_INVERT_R {
        CAP1_IN_INVERT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When set, capture on channel 2 is enabled"]
    #[inline(always)]
    pub fn cap1_en(&mut self) -> CAP1_EN_W {
        CAP1_EN_W { w: self }
    }
    #[doc = "Bits 1:2 - Edge of capture on channel 1 after prescaling. When bit0 is set to 1: enable capture on the negative edge, When bit1 is set to 1: enable capture on the positive edge."]
    #[inline(always)]
    pub fn cap1_mode(&mut self) -> CAP1_MODE_W {
        CAP1_MODE_W { w: self }
    }
    #[doc = "Bits 3:10 - Value of prescaling on possitive edge of CAP1. Prescale value = PWM_CAP1_PRESCALE + 1"]
    #[inline(always)]
    pub fn cap1_prescale(&mut self) -> CAP1_PRESCALE_W {
        CAP1_PRESCALE_W { w: self }
    }
    #[doc = "Bit 11 - when set, CAP1 form GPIO matrix is inverted before prescale"]
    #[inline(always)]
    pub fn cap1_in_invert(&mut self) -> CAP1_IN_INVERT_W {
        CAP1_IN_INVERT_W { w: self }
    }
    #[doc = "Bit 12 - Write 1 will trigger a software forced capture on channel 1"]
    #[inline(always)]
    pub fn cap1_sw(&mut self) -> CAP1_SW_W {
        CAP1_SW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Capture channel 1 configuration and enable\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cap_ch1_cfg]
(index.html) module"]
pub struct CAP_CH1_CFG_SPEC;
impl crate::RegisterSpec for CAP_CH1_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cap_ch1_cfg::R]
(R) reader structure"]
impl crate::Readable for CAP_CH1_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cap_ch1_cfg::W]
(W) writer structure"]
impl crate::Writable for CAP_CH1_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAP_CH1_CFG to value 0"]
impl crate::Resettable for CAP_CH1_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
