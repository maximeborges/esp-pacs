#[doc = "Register `TIMER%s_CONF` reader"]
pub struct R(crate::R<TIMER_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER%s_CONF` writer"]
pub struct W(crate::W<TIMER_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER_CONF_SPEC>;
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
impl From<crate::W<TIMER_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER0_DUTY_RES` reader - This register is used to control the range of the counter in timer %s."]
pub struct TIMER0_DUTY_RES_R(crate::FieldReader<u8>);
impl TIMER0_DUTY_RES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TIMER0_DUTY_RES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER0_DUTY_RES_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER0_DUTY_RES` writer - This register is used to control the range of the counter in timer %s."]
pub struct TIMER0_DUTY_RES_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER0_DUTY_RES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `CLK_DIV_TIMER0` reader - This register is used to configure the divisor for the divider in timer %s. The least significant eight bits represent the fractional part."]
pub struct CLK_DIV_TIMER0_R(crate::FieldReader<u32>);
impl CLK_DIV_TIMER0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CLK_DIV_TIMER0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_DIV_TIMER0_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK_DIV_TIMER0` writer - This register is used to configure the divisor for the divider in timer %s. The least significant eight bits represent the fractional part."]
pub struct CLK_DIV_TIMER0_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_DIV_TIMER0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0003_ffff << 4)) | ((value as u32 & 0x0003_ffff) << 4);
        self.w
    }
}
#[doc = "Field `TIMER0_PAUSE` reader - This bit is used to suspend the counter in timer %s."]
pub struct TIMER0_PAUSE_R(crate::FieldReader<bool>);
impl TIMER0_PAUSE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER0_PAUSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER0_PAUSE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER0_PAUSE` writer - This bit is used to suspend the counter in timer %s."]
pub struct TIMER0_PAUSE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER0_PAUSE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 22)) | ((value as u32 & 1) << 22);
        self.w
    }
}
#[doc = "Field `TIMER0_RST` reader - This bit is used to reset timer %s. The counter will show 0 after reset."]
pub struct TIMER0_RST_R(crate::FieldReader<bool>);
impl TIMER0_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER0_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER0_RST_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER0_RST` writer - This bit is used to reset timer %s. The counter will show 0 after reset."]
pub struct TIMER0_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER0_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 23)) | ((value as u32 & 1) << 23);
        self.w
    }
}
#[doc = "Field `TICK_SEL_TIMER0` reader - This bit is used to select clock for timer %s. When this bit is set to 1 LEDC_APB_CLK_SEL\\[1:0\\] should be 1, otherwise the timer clock may be not accurate. 0: LEDC_PWM_CLK. 1: REF_TICK."]
pub struct TICK_SEL_TIMER0_R(crate::FieldReader<bool>);
impl TICK_SEL_TIMER0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TICK_SEL_TIMER0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TICK_SEL_TIMER0_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TICK_SEL_TIMER0` writer - This bit is used to select clock for timer %s. When this bit is set to 1 LEDC_APB_CLK_SEL\\[1:0\\] should be 1, otherwise the timer clock may be not accurate. 0: LEDC_PWM_CLK. 1: REF_TICK."]
pub struct TICK_SEL_TIMER0_W<'a> {
    w: &'a mut W,
}
impl<'a> TICK_SEL_TIMER0_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 24)) | ((value as u32 & 1) << 24);
        self.w
    }
}
#[doc = "Field `TIMER0_PARA_UP` writer - Set this bit to update LEDC_CLK_DIV_TIMER%s and LEDC_TIMER%s_DUTY_RES."]
pub struct TIMER0_PARA_UP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER0_PARA_UP_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 25)) | ((value as u32 & 1) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - This register is used to control the range of the counter in timer %s."]
    #[inline(always)]
    pub fn timer0_duty_res(&self) -> TIMER0_DUTY_RES_R {
        TIMER0_DUTY_RES_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:21 - This register is used to configure the divisor for the divider in timer %s. The least significant eight bits represent the fractional part."]
    #[inline(always)]
    pub fn clk_div_timer0(&self) -> CLK_DIV_TIMER0_R {
        CLK_DIV_TIMER0_R::new(((self.bits >> 4) & 0x0003_ffff) as u32)
    }
    #[doc = "Bit 22 - This bit is used to suspend the counter in timer %s."]
    #[inline(always)]
    pub fn timer0_pause(&self) -> TIMER0_PAUSE_R {
        TIMER0_PAUSE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - This bit is used to reset timer %s. The counter will show 0 after reset."]
    #[inline(always)]
    pub fn timer0_rst(&self) -> TIMER0_RST_R {
        TIMER0_RST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - This bit is used to select clock for timer %s. When this bit is set to 1 LEDC_APB_CLK_SEL\\[1:0\\] should be 1, otherwise the timer clock may be not accurate. 0: LEDC_PWM_CLK. 1: REF_TICK."]
    #[inline(always)]
    pub fn tick_sel_timer0(&self) -> TICK_SEL_TIMER0_R {
        TICK_SEL_TIMER0_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - This register is used to control the range of the counter in timer %s."]
    #[inline(always)]
    pub fn timer0_duty_res(&mut self) -> TIMER0_DUTY_RES_W {
        TIMER0_DUTY_RES_W { w: self }
    }
    #[doc = "Bits 4:21 - This register is used to configure the divisor for the divider in timer %s. The least significant eight bits represent the fractional part."]
    #[inline(always)]
    pub fn clk_div_timer0(&mut self) -> CLK_DIV_TIMER0_W {
        CLK_DIV_TIMER0_W { w: self }
    }
    #[doc = "Bit 22 - This bit is used to suspend the counter in timer %s."]
    #[inline(always)]
    pub fn timer0_pause(&mut self) -> TIMER0_PAUSE_W {
        TIMER0_PAUSE_W { w: self }
    }
    #[doc = "Bit 23 - This bit is used to reset timer %s. The counter will show 0 after reset."]
    #[inline(always)]
    pub fn timer0_rst(&mut self) -> TIMER0_RST_W {
        TIMER0_RST_W { w: self }
    }
    #[doc = "Bit 24 - This bit is used to select clock for timer %s. When this bit is set to 1 LEDC_APB_CLK_SEL\\[1:0\\] should be 1, otherwise the timer clock may be not accurate. 0: LEDC_PWM_CLK. 1: REF_TICK."]
    #[inline(always)]
    pub fn tick_sel_timer0(&mut self) -> TICK_SEL_TIMER0_W {
        TICK_SEL_TIMER0_W { w: self }
    }
    #[doc = "Bit 25 - Set this bit to update LEDC_CLK_DIV_TIMER%s and LEDC_TIMER%s_DUTY_RES."]
    #[inline(always)]
    pub fn timer0_para_up(&mut self) -> TIMER0_PARA_UP_W {
        TIMER0_PARA_UP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer %s configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer_conf](index.html) module"]
pub struct TIMER_CONF_SPEC;
impl crate::RegisterSpec for TIMER_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer_conf::R](R) reader structure"]
impl crate::Readable for TIMER_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer_conf::W](W) writer structure"]
impl crate::Writable for TIMER_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMER%s_CONF to value 0x0080_0000"]
impl crate::Resettable for TIMER_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0080_0000
    }
}
