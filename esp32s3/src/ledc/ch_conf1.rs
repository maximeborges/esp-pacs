#[doc = "Register `CH%s_CONF1` reader"]
pub struct R(crate::R<CH_CONF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH_CONF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH_CONF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH_CONF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH%s_CONF1` writer"]
pub struct W(crate::W<CH_CONF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH_CONF1_SPEC>;
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
impl From<crate::W<CH_CONF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH_CONF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DUTY_SCALE_CH0` reader - This register is used to configure the changing step scale of duty on channel %s."]
pub struct DUTY_SCALE_CH0_R(crate::FieldReader<u16, u16>);
impl DUTY_SCALE_CH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        DUTY_SCALE_CH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_SCALE_CH0_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY_SCALE_CH0` writer - This register is used to configure the changing step scale of duty on channel %s."]
pub struct DUTY_SCALE_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_SCALE_CH0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
#[doc = "Field `DUTY_CYCLE_CH0` reader - The duty will change every LEDC_DUTY_CYCLE_CH%s on channel %s."]
pub struct DUTY_CYCLE_CH0_R(crate::FieldReader<u16, u16>);
impl DUTY_CYCLE_CH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        DUTY_CYCLE_CH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_CYCLE_CH0_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY_CYCLE_CH0` writer - The duty will change every LEDC_DUTY_CYCLE_CH%s on channel %s."]
pub struct DUTY_CYCLE_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_CYCLE_CH0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 10)) | ((value as u32 & 0x03ff) << 10);
        self.w
    }
}
#[doc = "Field `DUTY_NUM_CH0` reader - This register is used to control the number of times the duty cycle will be changed."]
pub struct DUTY_NUM_CH0_R(crate::FieldReader<u16, u16>);
impl DUTY_NUM_CH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        DUTY_NUM_CH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_NUM_CH0_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY_NUM_CH0` writer - This register is used to control the number of times the duty cycle will be changed."]
pub struct DUTY_NUM_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_NUM_CH0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 20)) | ((value as u32 & 0x03ff) << 20);
        self.w
    }
}
#[doc = "Field `DUTY_INC_CH0` reader - This register is used to increase or decrease the duty of output signal on channel %s. 1: Increase; 0: Decrease."]
pub struct DUTY_INC_CH0_R(crate::FieldReader<bool, bool>);
impl DUTY_INC_CH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DUTY_INC_CH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_INC_CH0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY_INC_CH0` writer - This register is used to increase or decrease the duty of output signal on channel %s. 1: Increase; 0: Decrease."]
pub struct DUTY_INC_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_INC_CH0_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 30)) | ((value as u32 & 1) << 30);
        self.w
    }
}
#[doc = "Field `DUTY_START_CH0` reader - Other configured fields in LEDC_CH%s_CONF1_REG will start to take effect when this bit is set to 1."]
pub struct DUTY_START_CH0_R(crate::FieldReader<bool, bool>);
impl DUTY_START_CH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DUTY_START_CH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_START_CH0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY_START_CH0` writer - Other configured fields in LEDC_CH%s_CONF1_REG will start to take effect when this bit is set to 1."]
pub struct DUTY_START_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_START_CH0_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 31)) | ((value as u32 & 1) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - This register is used to configure the changing step scale of duty on channel %s."]
    #[inline(always)]
    pub fn duty_scale_ch0(&self) -> DUTY_SCALE_CH0_R {
        DUTY_SCALE_CH0_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - The duty will change every LEDC_DUTY_CYCLE_CH%s on channel %s."]
    #[inline(always)]
    pub fn duty_cycle_ch0(&self) -> DUTY_CYCLE_CH0_R {
        DUTY_CYCLE_CH0_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:29 - This register is used to control the number of times the duty cycle will be changed."]
    #[inline(always)]
    pub fn duty_num_ch0(&self) -> DUTY_NUM_CH0_R {
        DUTY_NUM_CH0_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
    #[doc = "Bit 30 - This register is used to increase or decrease the duty of output signal on channel %s. 1: Increase; 0: Decrease."]
    #[inline(always)]
    pub fn duty_inc_ch0(&self) -> DUTY_INC_CH0_R {
        DUTY_INC_CH0_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Other configured fields in LEDC_CH%s_CONF1_REG will start to take effect when this bit is set to 1."]
    #[inline(always)]
    pub fn duty_start_ch0(&self) -> DUTY_START_CH0_R {
        DUTY_START_CH0_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - This register is used to configure the changing step scale of duty on channel %s."]
    #[inline(always)]
    pub fn duty_scale_ch0(&mut self) -> DUTY_SCALE_CH0_W {
        DUTY_SCALE_CH0_W { w: self }
    }
    #[doc = "Bits 10:19 - The duty will change every LEDC_DUTY_CYCLE_CH%s on channel %s."]
    #[inline(always)]
    pub fn duty_cycle_ch0(&mut self) -> DUTY_CYCLE_CH0_W {
        DUTY_CYCLE_CH0_W { w: self }
    }
    #[doc = "Bits 20:29 - This register is used to control the number of times the duty cycle will be changed."]
    #[inline(always)]
    pub fn duty_num_ch0(&mut self) -> DUTY_NUM_CH0_W {
        DUTY_NUM_CH0_W { w: self }
    }
    #[doc = "Bit 30 - This register is used to increase or decrease the duty of output signal on channel %s. 1: Increase; 0: Decrease."]
    #[inline(always)]
    pub fn duty_inc_ch0(&mut self) -> DUTY_INC_CH0_W {
        DUTY_INC_CH0_W { w: self }
    }
    #[doc = "Bit 31 - Other configured fields in LEDC_CH%s_CONF1_REG will start to take effect when this bit is set to 1."]
    #[inline(always)]
    pub fn duty_start_ch0(&mut self) -> DUTY_START_CH0_W {
        DUTY_START_CH0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration register 1 for channel %s\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch_conf1]
(index.html) module"]
pub struct CH_CONF1_SPEC;
impl crate::RegisterSpec for CH_CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch_conf1::R]
(R) reader structure"]
impl crate::Readable for CH_CONF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch_conf1::W]
(W) writer structure"]
impl crate::Writable for CH_CONF1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CH%s_CONF1 to value 0x4000_0000"]
impl crate::Resettable for CH_CONF1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4000_0000
    }
}
