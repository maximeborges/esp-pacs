#[doc = "Register `DAC_CONF` reader"]
pub struct R(crate::R<DAC_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAC_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAC_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAC_CONF` writer"]
pub struct W(crate::W<DAC_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAC_CONF_SPEC>;
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
impl From<crate::W<DAC_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAC_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAC_CLK_DIV` reader - Controls the division factor of the rising clock of the programming voltage."]
pub struct DAC_CLK_DIV_R(crate::FieldReader<u8, u8>);
impl DAC_CLK_DIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DAC_CLK_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAC_CLK_DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAC_CLK_DIV` writer - Controls the division factor of the rising clock of the programming voltage."]
pub struct DAC_CLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_CLK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `DAC_CLK_PAD_SEL` reader - Don't care."]
pub struct DAC_CLK_PAD_SEL_R(crate::FieldReader<bool, bool>);
impl DAC_CLK_PAD_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DAC_CLK_PAD_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAC_CLK_PAD_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAC_CLK_PAD_SEL` writer - Don't care."]
pub struct DAC_CLK_PAD_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_CLK_PAD_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 8)) | ((value as u32 & 1) << 8);
        self.w
    }
}
#[doc = "Field `DAC_NUM` reader - Controls the rising period of the programming voltage."]
pub struct DAC_NUM_R(crate::FieldReader<u8, u8>);
impl DAC_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DAC_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAC_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAC_NUM` writer - Controls the rising period of the programming voltage."]
pub struct DAC_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 9)) | ((value as u32 & 0xff) << 9);
        self.w
    }
}
#[doc = "Field `OE_CLR` reader - Reduces the power supply of the programming voltage."]
pub struct OE_CLR_R(crate::FieldReader<bool, bool>);
impl OE_CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OE_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OE_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OE_CLR` writer - Reduces the power supply of the programming voltage."]
pub struct OE_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> OE_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 17)) | ((value as u32 & 1) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Controls the division factor of the rising clock of the programming voltage."]
    #[inline(always)]
    pub fn dac_clk_div(&self) -> DAC_CLK_DIV_R {
        DAC_CLK_DIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Don't care."]
    #[inline(always)]
    pub fn dac_clk_pad_sel(&self) -> DAC_CLK_PAD_SEL_R {
        DAC_CLK_PAD_SEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:16 - Controls the rising period of the programming voltage."]
    #[inline(always)]
    pub fn dac_num(&self) -> DAC_NUM_R {
        DAC_NUM_R::new(((self.bits >> 9) & 0xff) as u8)
    }
    #[doc = "Bit 17 - Reduces the power supply of the programming voltage."]
    #[inline(always)]
    pub fn oe_clr(&self) -> OE_CLR_R {
        OE_CLR_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Controls the division factor of the rising clock of the programming voltage."]
    #[inline(always)]
    pub fn dac_clk_div(&mut self) -> DAC_CLK_DIV_W {
        DAC_CLK_DIV_W { w: self }
    }
    #[doc = "Bit 8 - Don't care."]
    #[inline(always)]
    pub fn dac_clk_pad_sel(&mut self) -> DAC_CLK_PAD_SEL_W {
        DAC_CLK_PAD_SEL_W { w: self }
    }
    #[doc = "Bits 9:16 - Controls the rising period of the programming voltage."]
    #[inline(always)]
    pub fn dac_num(&mut self) -> DAC_NUM_W {
        DAC_NUM_W { w: self }
    }
    #[doc = "Bit 17 - Reduces the power supply of the programming voltage."]
    #[inline(always)]
    pub fn oe_clr(&mut self) -> OE_CLR_W {
        OE_CLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Controls the eFuse programming voltage.\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_conf]
(index.html) module"]
pub struct DAC_CONF_SPEC;
impl crate::RegisterSpec for DAC_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dac_conf::R]
(R) reader structure"]
impl crate::Readable for DAC_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dac_conf::W]
(W) writer structure"]
impl crate::Writable for DAC_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DAC_CONF to value 0x0001_fe1c"]
impl crate::Resettable for DAC_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_fe1c
    }
}
