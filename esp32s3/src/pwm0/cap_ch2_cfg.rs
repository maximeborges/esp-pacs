#[doc = "Register `CAP_CH2_CFG` reader"]
pub struct R(crate::R<CAP_CH2_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAP_CH2_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAP_CH2_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAP_CH2_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAP_CH2_CFG` writer"]
pub struct W(crate::W<CAP_CH2_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAP_CH2_CFG_SPEC>;
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
impl From<crate::W<CAP_CH2_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAP_CH2_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAP2_EN` reader - When set, capture on channel 2 is enabled"]
pub struct CAP2_EN_R(crate::FieldReader<bool>);
impl CAP2_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CAP2_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAP2_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAP2_EN` writer - When set, capture on channel 2 is enabled"]
pub struct CAP2_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP2_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
#[doc = "Field `CAP2_MODE` reader - Edge of capture on channel 2 after prescaling. When bit0 is set to 1: enable capture on the negative edge, When bit1 is set to 1: enable capture on the positive edge."]
pub struct CAP2_MODE_R(crate::FieldReader<u8>);
impl CAP2_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CAP2_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAP2_MODE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAP2_MODE` writer - Edge of capture on channel 2 after prescaling. When bit0 is set to 1: enable capture on the negative edge, When bit1 is set to 1: enable capture on the positive edge."]
pub struct CAP2_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP2_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 1)) | ((value as u32 & 3) << 1);
        self.w
    }
}
#[doc = "Field `CAP2_PRESCALE` reader - Value of prescaling on possitive edge of CAP2. Prescale value = PWM_CAP2_PRESCALE + 1"]
pub struct CAP2_PRESCALE_R(crate::FieldReader<u8>);
impl CAP2_PRESCALE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CAP2_PRESCALE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAP2_PRESCALE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAP2_PRESCALE` writer - Value of prescaling on possitive edge of CAP2. Prescale value = PWM_CAP2_PRESCALE + 1"]
pub struct CAP2_PRESCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP2_PRESCALE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 3)) | ((value as u32 & 0xff) << 3);
        self.w
    }
}
#[doc = "Field `CAP2_IN_INVERT` reader - when set, CAP2 form GPIO matrix is inverted before prescale"]
pub struct CAP2_IN_INVERT_R(crate::FieldReader<bool>);
impl CAP2_IN_INVERT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CAP2_IN_INVERT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAP2_IN_INVERT_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAP2_IN_INVERT` writer - when set, CAP2 form GPIO matrix is inverted before prescale"]
pub struct CAP2_IN_INVERT_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP2_IN_INVERT_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 11)) | ((value as u32 & 1) << 11);
        self.w
    }
}
#[doc = "Field `CAP2_SW` writer - Write 1 will trigger a software forced capture on channel 2"]
pub struct CAP2_SW_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP2_SW_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 12)) | ((value as u32 & 1) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - When set, capture on channel 2 is enabled"]
    #[inline(always)]
    pub fn cap2_en(&self) -> CAP2_EN_R {
        CAP2_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Edge of capture on channel 2 after prescaling. When bit0 is set to 1: enable capture on the negative edge, When bit1 is set to 1: enable capture on the positive edge."]
    #[inline(always)]
    pub fn cap2_mode(&self) -> CAP2_MODE_R {
        CAP2_MODE_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:10 - Value of prescaling on possitive edge of CAP2. Prescale value = PWM_CAP2_PRESCALE + 1"]
    #[inline(always)]
    pub fn cap2_prescale(&self) -> CAP2_PRESCALE_R {
        CAP2_PRESCALE_R::new(((self.bits >> 3) & 0xff) as u8)
    }
    #[doc = "Bit 11 - when set, CAP2 form GPIO matrix is inverted before prescale"]
    #[inline(always)]
    pub fn cap2_in_invert(&self) -> CAP2_IN_INVERT_R {
        CAP2_IN_INVERT_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When set, capture on channel 2 is enabled"]
    #[inline(always)]
    pub fn cap2_en(&mut self) -> CAP2_EN_W {
        CAP2_EN_W { w: self }
    }
    #[doc = "Bits 1:2 - Edge of capture on channel 2 after prescaling. When bit0 is set to 1: enable capture on the negative edge, When bit1 is set to 1: enable capture on the positive edge."]
    #[inline(always)]
    pub fn cap2_mode(&mut self) -> CAP2_MODE_W {
        CAP2_MODE_W { w: self }
    }
    #[doc = "Bits 3:10 - Value of prescaling on possitive edge of CAP2. Prescale value = PWM_CAP2_PRESCALE + 1"]
    #[inline(always)]
    pub fn cap2_prescale(&mut self) -> CAP2_PRESCALE_W {
        CAP2_PRESCALE_W { w: self }
    }
    #[doc = "Bit 11 - when set, CAP2 form GPIO matrix is inverted before prescale"]
    #[inline(always)]
    pub fn cap2_in_invert(&mut self) -> CAP2_IN_INVERT_W {
        CAP2_IN_INVERT_W { w: self }
    }
    #[doc = "Bit 12 - Write 1 will trigger a software forced capture on channel 2"]
    #[inline(always)]
    pub fn cap2_sw(&mut self) -> CAP2_SW_W {
        CAP2_SW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Capture channel 2 configuration and enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cap_ch2_cfg](index.html) module"]
pub struct CAP_CH2_CFG_SPEC;
impl crate::RegisterSpec for CAP_CH2_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cap_ch2_cfg::R](R) reader structure"]
impl crate::Readable for CAP_CH2_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cap_ch2_cfg::W](W) writer structure"]
impl crate::Writable for CAP_CH2_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAP_CH2_CFG to value 0"]
impl crate::Resettable for CAP_CH2_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
