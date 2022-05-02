#[doc = "Register `LSTIMER2_CONF` reader"]
pub struct R(crate::R<LSTIMER2_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LSTIMER2_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LSTIMER2_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LSTIMER2_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LSTIMER2_CONF` writer"]
pub struct W(crate::W<LSTIMER2_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LSTIMER2_CONF_SPEC>;
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
impl From<crate::W<LSTIMER2_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LSTIMER2_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LSTIMER2_DUTY_RES` reader - This register controls the range of the counter in low speed timer2. the counter range is \\[0 2**reg_lstimer2_lim\\] the max bit width for counter is 20."]
pub struct LSTIMER2_DUTY_RES_R(crate::FieldReader<u8>);
impl LSTIMER2_DUTY_RES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LSTIMER2_DUTY_RES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSTIMER2_DUTY_RES_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSTIMER2_DUTY_RES` writer - This register controls the range of the counter in low speed timer2. the counter range is \\[0 2**reg_lstimer2_lim\\] the max bit width for counter is 20."]
pub struct LSTIMER2_DUTY_RES_W<'a> {
    w: &'a mut W,
}
impl<'a> LSTIMER2_DUTY_RES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Field `DIV_NUM_LSTIMER2` reader - This register is used to configure parameter for divider in low speed timer2 the least significant eight bits represent the decimal part."]
pub struct DIV_NUM_LSTIMER2_R(crate::FieldReader<u32>);
impl DIV_NUM_LSTIMER2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        DIV_NUM_LSTIMER2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIV_NUM_LSTIMER2_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIV_NUM_LSTIMER2` writer - This register is used to configure parameter for divider in low speed timer2 the least significant eight bits represent the decimal part."]
pub struct DIV_NUM_LSTIMER2_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_NUM_LSTIMER2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0003_ffff << 5)) | ((value as u32 & 0x0003_ffff) << 5);
        self.w
    }
}
#[doc = "Field `LSTIMER2_PAUSE` reader - This bit is used to pause the counter in low speed timer2."]
pub struct LSTIMER2_PAUSE_R(crate::FieldReader<bool>);
impl LSTIMER2_PAUSE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LSTIMER2_PAUSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSTIMER2_PAUSE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSTIMER2_PAUSE` writer - This bit is used to pause the counter in low speed timer2."]
pub struct LSTIMER2_PAUSE_W<'a> {
    w: &'a mut W,
}
impl<'a> LSTIMER2_PAUSE_W<'a> {
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
#[doc = "Field `LSTIMER2_RST` reader - This bit is used to reset low speed timer2 the counter will be 0 after reset."]
pub struct LSTIMER2_RST_R(crate::FieldReader<bool>);
impl LSTIMER2_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LSTIMER2_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSTIMER2_RST_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSTIMER2_RST` writer - This bit is used to reset low speed timer2 the counter will be 0 after reset."]
pub struct LSTIMER2_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> LSTIMER2_RST_W<'a> {
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
#[doc = "Field `TICK_SEL_LSTIMER2` reader - This bit is used to choose slow_clk or ref_tick for low speed timer2. 1'b1:slow_clk 0:ref_tick"]
pub struct TICK_SEL_LSTIMER2_R(crate::FieldReader<bool>);
impl TICK_SEL_LSTIMER2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TICK_SEL_LSTIMER2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TICK_SEL_LSTIMER2_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TICK_SEL_LSTIMER2` writer - This bit is used to choose slow_clk or ref_tick for low speed timer2. 1'b1:slow_clk 0:ref_tick"]
pub struct TICK_SEL_LSTIMER2_W<'a> {
    w: &'a mut W,
}
impl<'a> TICK_SEL_LSTIMER2_W<'a> {
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
#[doc = "Field `LSTIMER2_PARA_UP` reader - Set this bit to update reg_div_num_lstime2 and reg_lstimer2_lim."]
pub struct LSTIMER2_PARA_UP_R(crate::FieldReader<bool>);
impl LSTIMER2_PARA_UP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LSTIMER2_PARA_UP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSTIMER2_PARA_UP_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSTIMER2_PARA_UP` writer - Set this bit to update reg_div_num_lstime2 and reg_lstimer2_lim."]
pub struct LSTIMER2_PARA_UP_W<'a> {
    w: &'a mut W,
}
impl<'a> LSTIMER2_PARA_UP_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 26)) | ((value as u32 & 1) << 26);
        self.w
    }
}
#[doc = "Field `LSTIMER2_LIM` reader - "]
pub struct LSTIMER2_LIM_R(crate::FieldReader<u8>);
impl LSTIMER2_LIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LSTIMER2_LIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSTIMER2_LIM_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSTIMER2_LIM` writer - "]
pub struct LSTIMER2_LIM_W<'a> {
    w: &'a mut W,
}
impl<'a> LSTIMER2_LIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 31)) | ((value as u32 & 0x1f) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - This register controls the range of the counter in low speed timer2. the counter range is \\[0 2**reg_lstimer2_lim\\] the max bit width for counter is 20."]
    #[inline(always)]
    pub fn lstimer2_duty_res(&self) -> LSTIMER2_DUTY_RES_R {
        LSTIMER2_DUTY_RES_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:22 - This register is used to configure parameter for divider in low speed timer2 the least significant eight bits represent the decimal part."]
    #[inline(always)]
    pub fn div_num_lstimer2(&self) -> DIV_NUM_LSTIMER2_R {
        DIV_NUM_LSTIMER2_R::new(((self.bits >> 5) & 0x0003_ffff) as u32)
    }
    #[doc = "Bit 23 - This bit is used to pause the counter in low speed timer2."]
    #[inline(always)]
    pub fn lstimer2_pause(&self) -> LSTIMER2_PAUSE_R {
        LSTIMER2_PAUSE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - This bit is used to reset low speed timer2 the counter will be 0 after reset."]
    #[inline(always)]
    pub fn lstimer2_rst(&self) -> LSTIMER2_RST_R {
        LSTIMER2_RST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - This bit is used to choose slow_clk or ref_tick for low speed timer2. 1'b1:slow_clk 0:ref_tick"]
    #[inline(always)]
    pub fn tick_sel_lstimer2(&self) -> TICK_SEL_LSTIMER2_R {
        TICK_SEL_LSTIMER2_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Set this bit to update reg_div_num_lstime2 and reg_lstimer2_lim."]
    #[inline(always)]
    pub fn lstimer2_para_up(&self) -> LSTIMER2_PARA_UP_R {
        LSTIMER2_PARA_UP_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 31:35"]
    #[inline(always)]
    pub fn lstimer2_lim(&self) -> LSTIMER2_LIM_R {
        LSTIMER2_LIM_R::new(((self.bits >> 31) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - This register controls the range of the counter in low speed timer2. the counter range is \\[0 2**reg_lstimer2_lim\\] the max bit width for counter is 20."]
    #[inline(always)]
    pub fn lstimer2_duty_res(&mut self) -> LSTIMER2_DUTY_RES_W {
        LSTIMER2_DUTY_RES_W { w: self }
    }
    #[doc = "Bits 5:22 - This register is used to configure parameter for divider in low speed timer2 the least significant eight bits represent the decimal part."]
    #[inline(always)]
    pub fn div_num_lstimer2(&mut self) -> DIV_NUM_LSTIMER2_W {
        DIV_NUM_LSTIMER2_W { w: self }
    }
    #[doc = "Bit 23 - This bit is used to pause the counter in low speed timer2."]
    #[inline(always)]
    pub fn lstimer2_pause(&mut self) -> LSTIMER2_PAUSE_W {
        LSTIMER2_PAUSE_W { w: self }
    }
    #[doc = "Bit 24 - This bit is used to reset low speed timer2 the counter will be 0 after reset."]
    #[inline(always)]
    pub fn lstimer2_rst(&mut self) -> LSTIMER2_RST_W {
        LSTIMER2_RST_W { w: self }
    }
    #[doc = "Bit 25 - This bit is used to choose slow_clk or ref_tick for low speed timer2. 1'b1:slow_clk 0:ref_tick"]
    #[inline(always)]
    pub fn tick_sel_lstimer2(&mut self) -> TICK_SEL_LSTIMER2_W {
        TICK_SEL_LSTIMER2_W { w: self }
    }
    #[doc = "Bit 26 - Set this bit to update reg_div_num_lstime2 and reg_lstimer2_lim."]
    #[inline(always)]
    pub fn lstimer2_para_up(&mut self) -> LSTIMER2_PARA_UP_W {
        LSTIMER2_PARA_UP_W { w: self }
    }
    #[doc = "Bits 31:35"]
    #[inline(always)]
    pub fn lstimer2_lim(&mut self) -> LSTIMER2_LIM_W {
        LSTIMER2_LIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lstimer2_conf](index.html) module"]
pub struct LSTIMER2_CONF_SPEC;
impl crate::RegisterSpec for LSTIMER2_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lstimer2_conf::R](R) reader structure"]
impl crate::Readable for LSTIMER2_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lstimer2_conf::W](W) writer structure"]
impl crate::Writable for LSTIMER2_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LSTIMER2_CONF to value 0x0100_0000"]
impl crate::Resettable for LSTIMER2_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0100_0000
    }
}
