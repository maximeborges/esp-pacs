#[doc = "Register `LSTIMER1_CONF` reader"]
pub struct R(crate::R<LSTIMER1_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LSTIMER1_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LSTIMER1_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LSTIMER1_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LSTIMER1_CONF` writer"]
pub struct W(crate::W<LSTIMER1_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LSTIMER1_CONF_SPEC>;
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
impl From<crate::W<LSTIMER1_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LSTIMER1_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LSTIMER1_DUTY_RES` reader - reg_lstimer1_duty_res."]
pub struct LSTIMER1_DUTY_RES_R(crate::FieldReader<u8, u8>);
impl LSTIMER1_DUTY_RES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LSTIMER1_DUTY_RES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSTIMER1_DUTY_RES_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSTIMER1_DUTY_RES` writer - reg_lstimer1_duty_res."]
pub struct LSTIMER1_DUTY_RES_W<'a> {
    w: &'a mut W,
}
impl<'a> LSTIMER1_DUTY_RES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `CLK_DIV_LSTIMER1` reader - reg_clk_div_lstimer1."]
pub struct CLK_DIV_LSTIMER1_R(crate::FieldReader<u32, u32>);
impl CLK_DIV_LSTIMER1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CLK_DIV_LSTIMER1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_DIV_LSTIMER1_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK_DIV_LSTIMER1` writer - reg_clk_div_lstimer1."]
pub struct CLK_DIV_LSTIMER1_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_DIV_LSTIMER1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0003_ffff << 4)) | ((value as u32 & 0x0003_ffff) << 4);
        self.w
    }
}
#[doc = "Field `LSTIMER1_PAUSE` reader - reg_lstimer1_pause."]
pub struct LSTIMER1_PAUSE_R(crate::FieldReader<bool, bool>);
impl LSTIMER1_PAUSE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LSTIMER1_PAUSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSTIMER1_PAUSE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSTIMER1_PAUSE` writer - reg_lstimer1_pause."]
pub struct LSTIMER1_PAUSE_W<'a> {
    w: &'a mut W,
}
impl<'a> LSTIMER1_PAUSE_W<'a> {
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
#[doc = "Field `LSTIMER1_RST` reader - reg_lstimer1_rst."]
pub struct LSTIMER1_RST_R(crate::FieldReader<bool, bool>);
impl LSTIMER1_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LSTIMER1_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSTIMER1_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSTIMER1_RST` writer - reg_lstimer1_rst."]
pub struct LSTIMER1_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> LSTIMER1_RST_W<'a> {
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
#[doc = "Field `TICK_SEL_LSTIMER1` reader - reg_tick_sel_lstimer1."]
pub struct TICK_SEL_LSTIMER1_R(crate::FieldReader<bool, bool>);
impl TICK_SEL_LSTIMER1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TICK_SEL_LSTIMER1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TICK_SEL_LSTIMER1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TICK_SEL_LSTIMER1` writer - reg_tick_sel_lstimer1."]
pub struct TICK_SEL_LSTIMER1_W<'a> {
    w: &'a mut W,
}
impl<'a> TICK_SEL_LSTIMER1_W<'a> {
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
#[doc = "Field `LSTIMER1_PARA_UP` writer - reg_lstimer1_para_up."]
pub struct LSTIMER1_PARA_UP_W<'a> {
    w: &'a mut W,
}
impl<'a> LSTIMER1_PARA_UP_W<'a> {
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
    #[doc = "Bits 0:3 - reg_lstimer1_duty_res."]
    #[inline(always)]
    pub fn lstimer1_duty_res(&self) -> LSTIMER1_DUTY_RES_R {
        LSTIMER1_DUTY_RES_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:21 - reg_clk_div_lstimer1."]
    #[inline(always)]
    pub fn clk_div_lstimer1(&self) -> CLK_DIV_LSTIMER1_R {
        CLK_DIV_LSTIMER1_R::new(((self.bits >> 4) & 0x0003_ffff) as u32)
    }
    #[doc = "Bit 22 - reg_lstimer1_pause."]
    #[inline(always)]
    pub fn lstimer1_pause(&self) -> LSTIMER1_PAUSE_R {
        LSTIMER1_PAUSE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - reg_lstimer1_rst."]
    #[inline(always)]
    pub fn lstimer1_rst(&self) -> LSTIMER1_RST_R {
        LSTIMER1_RST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - reg_tick_sel_lstimer1."]
    #[inline(always)]
    pub fn tick_sel_lstimer1(&self) -> TICK_SEL_LSTIMER1_R {
        TICK_SEL_LSTIMER1_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - reg_lstimer1_duty_res."]
    #[inline(always)]
    pub fn lstimer1_duty_res(&mut self) -> LSTIMER1_DUTY_RES_W {
        LSTIMER1_DUTY_RES_W { w: self }
    }
    #[doc = "Bits 4:21 - reg_clk_div_lstimer1."]
    #[inline(always)]
    pub fn clk_div_lstimer1(&mut self) -> CLK_DIV_LSTIMER1_W {
        CLK_DIV_LSTIMER1_W { w: self }
    }
    #[doc = "Bit 22 - reg_lstimer1_pause."]
    #[inline(always)]
    pub fn lstimer1_pause(&mut self) -> LSTIMER1_PAUSE_W {
        LSTIMER1_PAUSE_W { w: self }
    }
    #[doc = "Bit 23 - reg_lstimer1_rst."]
    #[inline(always)]
    pub fn lstimer1_rst(&mut self) -> LSTIMER1_RST_W {
        LSTIMER1_RST_W { w: self }
    }
    #[doc = "Bit 24 - reg_tick_sel_lstimer1."]
    #[inline(always)]
    pub fn tick_sel_lstimer1(&mut self) -> TICK_SEL_LSTIMER1_W {
        TICK_SEL_LSTIMER1_W { w: self }
    }
    #[doc = "Bit 25 - reg_lstimer1_para_up."]
    #[inline(always)]
    pub fn lstimer1_para_up(&mut self) -> LSTIMER1_PARA_UP_W {
        LSTIMER1_PARA_UP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LEDC_LSTIMER1_CONF.\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lstimer1_conf]
(index.html) module"]
pub struct LSTIMER1_CONF_SPEC;
impl crate::RegisterSpec for LSTIMER1_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lstimer1_conf::R]
(R) reader structure"]
impl crate::Readable for LSTIMER1_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lstimer1_conf::W]
(W) writer structure"]
impl crate::Writable for LSTIMER1_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LSTIMER1_CONF to value 0x0080_0000"]
impl crate::Resettable for LSTIMER1_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0080_0000
    }
}
