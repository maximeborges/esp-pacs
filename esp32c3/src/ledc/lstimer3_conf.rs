#[doc = "Register `LSTIMER3_CONF` reader"]
pub struct R(crate::R<LSTIMER3_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LSTIMER3_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LSTIMER3_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LSTIMER3_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LSTIMER3_CONF` writer"]
pub struct W(crate::W<LSTIMER3_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LSTIMER3_CONF_SPEC>;
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
impl From<crate::W<LSTIMER3_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LSTIMER3_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LSTIMER3_DUTY_RES` reader - reg_lstimer3_duty_res."]
pub struct LSTIMER3_DUTY_RES_R(crate::FieldReader<u8, u8>);
impl LSTIMER3_DUTY_RES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LSTIMER3_DUTY_RES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSTIMER3_DUTY_RES_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSTIMER3_DUTY_RES` writer - reg_lstimer3_duty_res."]
pub struct LSTIMER3_DUTY_RES_W<'a> {
    w: &'a mut W,
}
impl<'a> LSTIMER3_DUTY_RES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `CLK_DIV_LSTIMER3` reader - reg_clk_div_lstimer3."]
pub struct CLK_DIV_LSTIMER3_R(crate::FieldReader<u32, u32>);
impl CLK_DIV_LSTIMER3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CLK_DIV_LSTIMER3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_DIV_LSTIMER3_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK_DIV_LSTIMER3` writer - reg_clk_div_lstimer3."]
pub struct CLK_DIV_LSTIMER3_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_DIV_LSTIMER3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0003_ffff << 4)) | ((value as u32 & 0x0003_ffff) << 4);
        self.w
    }
}
#[doc = "Field `LSTIMER3_PAUSE` reader - reg_lstimer3_pause."]
pub struct LSTIMER3_PAUSE_R(crate::FieldReader<bool, bool>);
impl LSTIMER3_PAUSE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LSTIMER3_PAUSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSTIMER3_PAUSE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSTIMER3_PAUSE` writer - reg_lstimer3_pause."]
pub struct LSTIMER3_PAUSE_W<'a> {
    w: &'a mut W,
}
impl<'a> LSTIMER3_PAUSE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `LSTIMER3_RST` reader - reg_lstimer3_rst."]
pub struct LSTIMER3_RST_R(crate::FieldReader<bool, bool>);
impl LSTIMER3_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LSTIMER3_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSTIMER3_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSTIMER3_RST` writer - reg_lstimer3_rst."]
pub struct LSTIMER3_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> LSTIMER3_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `TICK_SEL_LSTIMER3` reader - reg_tick_sel_lstimer3."]
pub struct TICK_SEL_LSTIMER3_R(crate::FieldReader<bool, bool>);
impl TICK_SEL_LSTIMER3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TICK_SEL_LSTIMER3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TICK_SEL_LSTIMER3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TICK_SEL_LSTIMER3` writer - reg_tick_sel_lstimer3."]
pub struct TICK_SEL_LSTIMER3_W<'a> {
    w: &'a mut W,
}
impl<'a> TICK_SEL_LSTIMER3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `LSTIMER3_PARA_UP` writer - reg_lstimer3_para_up."]
pub struct LSTIMER3_PARA_UP_W<'a> {
    w: &'a mut W,
}
impl<'a> LSTIMER3_PARA_UP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - reg_lstimer3_duty_res."]
    #[inline(always)]
    pub fn lstimer3_duty_res(&self) -> LSTIMER3_DUTY_RES_R {
        LSTIMER3_DUTY_RES_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:21 - reg_clk_div_lstimer3."]
    #[inline(always)]
    pub fn clk_div_lstimer3(&self) -> CLK_DIV_LSTIMER3_R {
        CLK_DIV_LSTIMER3_R::new(((self.bits >> 4) & 0x0003_ffff) as u32)
    }
    #[doc = "Bit 22 - reg_lstimer3_pause."]
    #[inline(always)]
    pub fn lstimer3_pause(&self) -> LSTIMER3_PAUSE_R {
        LSTIMER3_PAUSE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - reg_lstimer3_rst."]
    #[inline(always)]
    pub fn lstimer3_rst(&self) -> LSTIMER3_RST_R {
        LSTIMER3_RST_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - reg_tick_sel_lstimer3."]
    #[inline(always)]
    pub fn tick_sel_lstimer3(&self) -> TICK_SEL_LSTIMER3_R {
        TICK_SEL_LSTIMER3_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - reg_lstimer3_duty_res."]
    #[inline(always)]
    pub fn lstimer3_duty_res(&mut self) -> LSTIMER3_DUTY_RES_W {
        LSTIMER3_DUTY_RES_W { w: self }
    }
    #[doc = "Bits 4:21 - reg_clk_div_lstimer3."]
    #[inline(always)]
    pub fn clk_div_lstimer3(&mut self) -> CLK_DIV_LSTIMER3_W {
        CLK_DIV_LSTIMER3_W { w: self }
    }
    #[doc = "Bit 22 - reg_lstimer3_pause."]
    #[inline(always)]
    pub fn lstimer3_pause(&mut self) -> LSTIMER3_PAUSE_W {
        LSTIMER3_PAUSE_W { w: self }
    }
    #[doc = "Bit 23 - reg_lstimer3_rst."]
    #[inline(always)]
    pub fn lstimer3_rst(&mut self) -> LSTIMER3_RST_W {
        LSTIMER3_RST_W { w: self }
    }
    #[doc = "Bit 24 - reg_tick_sel_lstimer3."]
    #[inline(always)]
    pub fn tick_sel_lstimer3(&mut self) -> TICK_SEL_LSTIMER3_W {
        TICK_SEL_LSTIMER3_W { w: self }
    }
    #[doc = "Bit 25 - reg_lstimer3_para_up."]
    #[inline(always)]
    pub fn lstimer3_para_up(&mut self) -> LSTIMER3_PARA_UP_W {
        LSTIMER3_PARA_UP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LEDC_LSTIMER3_CONF.\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lstimer3_conf]
(index.html) module"]
pub struct LSTIMER3_CONF_SPEC;
impl crate::RegisterSpec for LSTIMER3_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lstimer3_conf::R]
(R) reader structure"]
impl crate::Readable for LSTIMER3_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lstimer3_conf::W]
(W) writer structure"]
impl crate::Writable for LSTIMER3_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LSTIMER3_CONF to value 0x0080_0000"]
impl crate::Resettable for LSTIMER3_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0080_0000
    }
}
