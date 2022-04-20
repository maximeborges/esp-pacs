#[doc = "Register `FUNC%s_OUT_SEL_CFG` reader"]
pub struct R(crate::R<FUNC_OUT_SEL_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FUNC_OUT_SEL_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FUNC_OUT_SEL_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FUNC_OUT_SEL_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FUNC%s_OUT_SEL_CFG` writer"]
pub struct W(crate::W<FUNC_OUT_SEL_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FUNC_OUT_SEL_CFG_SPEC>;
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
impl From<crate::W<FUNC_OUT_SEL_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FUNC_OUT_SEL_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUT_SEL` reader - Selection control for GPIO output n. If a value s (0<=s<256) is written to this field, the peripheral output signal s will be connected to GPIO output n. If a value 256 is written to this field, bit n of GPIO_OUT_REG/GPIO_OUT1_REG and GPIO_ENABLE_REG/GPIO_ENABLE1_REG will be selected as the output value and output enable."]
pub struct OUT_SEL_R(crate::FieldReader<u16, u16>);
impl OUT_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        OUT_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_SEL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_SEL` writer - Selection control for GPIO output n. If a value s (0<=s<256) is written to this field, the peripheral output signal s will be connected to GPIO output n. If a value 256 is written to this field, bit n of GPIO_OUT_REG/GPIO_OUT1_REG and GPIO_ENABLE_REG/GPIO_ENABLE1_REG will be selected as the output value and output enable."]
pub struct OUT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | (value as u32 & 0x01ff);
        self.w
    }
}
#[doc = "Field `INV_SEL` reader - 0: Do not invert the output value; 1: Invert the output value."]
pub struct INV_SEL_R(crate::FieldReader<bool, bool>);
impl INV_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INV_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INV_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INV_SEL` writer - 0: Do not invert the output value; 1: Invert the output value."]
pub struct INV_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> INV_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 9)) | ((value as u32 & 1) << 9);
        self.w
    }
}
#[doc = "Field `OEN_SEL` reader - 0: Use output enable signal from peripheral; 1: Force the output enable signal to be sourced from bit n of GPIO_ENABLE_REG."]
pub struct OEN_SEL_R(crate::FieldReader<bool, bool>);
impl OEN_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OEN_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OEN_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OEN_SEL` writer - 0: Use output enable signal from peripheral; 1: Force the output enable signal to be sourced from bit n of GPIO_ENABLE_REG."]
pub struct OEN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OEN_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 10)) | ((value as u32 & 1) << 10);
        self.w
    }
}
#[doc = "Field `OEN_INV_SEL` reader - 0: Do not invert the output enable signal; 1: Invert the output enable signal."]
pub struct OEN_INV_SEL_R(crate::FieldReader<bool, bool>);
impl OEN_INV_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OEN_INV_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OEN_INV_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OEN_INV_SEL` writer - 0: Do not invert the output enable signal; 1: Invert the output enable signal."]
pub struct OEN_INV_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OEN_INV_SEL_W<'a> {
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
impl R {
    #[doc = "Bits 0:8 - Selection control for GPIO output n. If a value s (0<=s<256) is written to this field, the peripheral output signal s will be connected to GPIO output n. If a value 256 is written to this field, bit n of GPIO_OUT_REG/GPIO_OUT1_REG and GPIO_ENABLE_REG/GPIO_ENABLE1_REG will be selected as the output value and output enable."]
    #[inline(always)]
    pub fn out_sel(&self) -> OUT_SEL_R {
        OUT_SEL_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 9 - 0: Do not invert the output value; 1: Invert the output value."]
    #[inline(always)]
    pub fn inv_sel(&self) -> INV_SEL_R {
        INV_SEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 0: Use output enable signal from peripheral; 1: Force the output enable signal to be sourced from bit n of GPIO_ENABLE_REG."]
    #[inline(always)]
    pub fn oen_sel(&self) -> OEN_SEL_R {
        OEN_SEL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 0: Do not invert the output enable signal; 1: Invert the output enable signal."]
    #[inline(always)]
    pub fn oen_inv_sel(&self) -> OEN_INV_SEL_R {
        OEN_INV_SEL_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - Selection control for GPIO output n. If a value s (0<=s<256) is written to this field, the peripheral output signal s will be connected to GPIO output n. If a value 256 is written to this field, bit n of GPIO_OUT_REG/GPIO_OUT1_REG and GPIO_ENABLE_REG/GPIO_ENABLE1_REG will be selected as the output value and output enable."]
    #[inline(always)]
    pub fn out_sel(&mut self) -> OUT_SEL_W {
        OUT_SEL_W { w: self }
    }
    #[doc = "Bit 9 - 0: Do not invert the output value; 1: Invert the output value."]
    #[inline(always)]
    pub fn inv_sel(&mut self) -> INV_SEL_W {
        INV_SEL_W { w: self }
    }
    #[doc = "Bit 10 - 0: Use output enable signal from peripheral; 1: Force the output enable signal to be sourced from bit n of GPIO_ENABLE_REG."]
    #[inline(always)]
    pub fn oen_sel(&mut self) -> OEN_SEL_W {
        OEN_SEL_W { w: self }
    }
    #[doc = "Bit 11 - 0: Do not invert the output enable signal; 1: Invert the output enable signal."]
    #[inline(always)]
    pub fn oen_inv_sel(&mut self) -> OEN_INV_SEL_W {
        OEN_INV_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral output selection for GPIO %s\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [func_out_sel_cfg]
(index.html) module"]
pub struct FUNC_OUT_SEL_CFG_SPEC;
impl crate::RegisterSpec for FUNC_OUT_SEL_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [func_out_sel_cfg::R]
(R) reader structure"]
impl crate::Readable for FUNC_OUT_SEL_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [func_out_sel_cfg::W]
(W) writer structure"]
impl crate::Writable for FUNC_OUT_SEL_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FUNC%s_OUT_SEL_CFG to value 0x0100"]
impl crate::Resettable for FUNC_OUT_SEL_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0100
    }
}
