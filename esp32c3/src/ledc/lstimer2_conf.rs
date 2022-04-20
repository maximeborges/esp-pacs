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
#[doc = "Field `LSTIMER2_DUTY_RES` reader - reg_lstimer2_duty_res."]
pub struct LSTIMER2_DUTY_RES_R(crate::FieldReader<u8, u8>);
impl LSTIMER2_DUTY_RES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LSTIMER2_DUTY_RES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSTIMER2_DUTY_RES_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSTIMER2_DUTY_RES` writer - reg_lstimer2_duty_res."]
pub struct LSTIMER2_DUTY_RES_W<'a> {
    w: &'a mut W,
}
impl<'a> LSTIMER2_DUTY_RES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `CLK_DIV_LSTIMER2` reader - reg_clk_div_lstimer2."]
pub struct CLK_DIV_LSTIMER2_R(crate::FieldReader<u32, u32>);
impl CLK_DIV_LSTIMER2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CLK_DIV_LSTIMER2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_DIV_LSTIMER2_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK_DIV_LSTIMER2` writer - reg_clk_div_lstimer2."]
pub struct CLK_DIV_LSTIMER2_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_DIV_LSTIMER2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0003_ffff << 4)) | ((value as u32 & 0x0003_ffff) << 4);
        self.w
    }
}
#[doc = "Field `LSTIMER2_PAUSE` reader - reg_lstimer2_pause."]
pub struct LSTIMER2_PAUSE_R(crate::FieldReader<bool, bool>);
impl LSTIMER2_PAUSE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LSTIMER2_PAUSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSTIMER2_PAUSE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSTIMER2_PAUSE` writer - reg_lstimer2_pause."]
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
        self.w.bits = (self.w.bits & !(1 << 22)) | ((value as u32 & 1) << 22);
        self.w
    }
}
#[doc = "Field `LSTIMER2_RST` reader - reg_lstimer2_rst."]
pub struct LSTIMER2_RST_R(crate::FieldReader<bool, bool>);
impl LSTIMER2_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LSTIMER2_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSTIMER2_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSTIMER2_RST` writer - reg_lstimer2_rst."]
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
        self.w.bits = (self.w.bits & !(1 << 23)) | ((value as u32 & 1) << 23);
        self.w
    }
}
#[doc = "Field `TICK_SEL_LSTIMER2` reader - reg_tick_sel_lstimer2."]
pub struct TICK_SEL_LSTIMER2_R(crate::FieldReader<bool, bool>);
impl TICK_SEL_LSTIMER2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TICK_SEL_LSTIMER2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TICK_SEL_LSTIMER2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TICK_SEL_LSTIMER2` writer - reg_tick_sel_lstimer2."]
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
        self.w.bits = (self.w.bits & !(1 << 24)) | ((value as u32 & 1) << 24);
        self.w
    }
}
#[doc = "Field `LSTIMER2_PARA_UP` writer - reg_lstimer2_para_up."]
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
        self.w.bits = (self.w.bits & !(1 << 25)) | ((value as u32 & 1) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - reg_lstimer2_duty_res."]
    #[inline(always)]
    pub fn lstimer2_duty_res(&self) -> LSTIMER2_DUTY_RES_R {
        LSTIMER2_DUTY_RES_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:21 - reg_clk_div_lstimer2."]
    #[inline(always)]
    pub fn clk_div_lstimer2(&self) -> CLK_DIV_LSTIMER2_R {
        CLK_DIV_LSTIMER2_R::new(((self.bits >> 4) & 0x0003_ffff) as u32)
    }
    #[doc = "Bit 22 - reg_lstimer2_pause."]
    #[inline(always)]
    pub fn lstimer2_pause(&self) -> LSTIMER2_PAUSE_R {
        LSTIMER2_PAUSE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - reg_lstimer2_rst."]
    #[inline(always)]
    pub fn lstimer2_rst(&self) -> LSTIMER2_RST_R {
        LSTIMER2_RST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - reg_tick_sel_lstimer2."]
    #[inline(always)]
    pub fn tick_sel_lstimer2(&self) -> TICK_SEL_LSTIMER2_R {
        TICK_SEL_LSTIMER2_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - reg_lstimer2_duty_res."]
    #[inline(always)]
    pub fn lstimer2_duty_res(&mut self) -> LSTIMER2_DUTY_RES_W {
        LSTIMER2_DUTY_RES_W { w: self }
    }
    #[doc = "Bits 4:21 - reg_clk_div_lstimer2."]
    #[inline(always)]
    pub fn clk_div_lstimer2(&mut self) -> CLK_DIV_LSTIMER2_W {
        CLK_DIV_LSTIMER2_W { w: self }
    }
    #[doc = "Bit 22 - reg_lstimer2_pause."]
    #[inline(always)]
    pub fn lstimer2_pause(&mut self) -> LSTIMER2_PAUSE_W {
        LSTIMER2_PAUSE_W { w: self }
    }
    #[doc = "Bit 23 - reg_lstimer2_rst."]
    #[inline(always)]
    pub fn lstimer2_rst(&mut self) -> LSTIMER2_RST_W {
        LSTIMER2_RST_W { w: self }
    }
    #[doc = "Bit 24 - reg_tick_sel_lstimer2."]
    #[inline(always)]
    pub fn tick_sel_lstimer2(&mut self) -> TICK_SEL_LSTIMER2_W {
        TICK_SEL_LSTIMER2_W { w: self }
    }
    #[doc = "Bit 25 - reg_lstimer2_para_up."]
    #[inline(always)]
    pub fn lstimer2_para_up(&mut self) -> LSTIMER2_PARA_UP_W {
        LSTIMER2_PARA_UP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LEDC_LSTIMER2_CONF.\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lstimer2_conf]
(index.html) module"]
pub struct LSTIMER2_CONF_SPEC;
impl crate::RegisterSpec for LSTIMER2_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lstimer2_conf::R]
(R) reader structure"]
impl crate::Readable for LSTIMER2_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lstimer2_conf::W]
(W) writer structure"]
impl crate::Writable for LSTIMER2_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LSTIMER2_CONF to value 0x0080_0000"]
impl crate::Resettable for LSTIMER2_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0080_0000
    }
}
