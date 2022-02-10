#[doc = "Register `LCD_CMD_VAL` reader"]
pub struct R(crate::R<LCD_CMD_VAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCD_CMD_VAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCD_CMD_VAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCD_CMD_VAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCD_CMD_VAL` writer"]
pub struct W(crate::W<LCD_CMD_VAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCD_CMD_VAL_SPEC>;
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
impl From<crate::W<LCD_CMD_VAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCD_CMD_VAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCD_CMD_VALUE` reader - The LCD write command value."]
pub struct LCD_CMD_VALUE_R(crate::FieldReader<u32, u32>);
impl LCD_CMD_VALUE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        LCD_CMD_VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCD_CMD_VALUE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCD_CMD_VALUE` writer - The LCD write command value."]
pub struct LCD_CMD_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> LCD_CMD_VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - The LCD write command value."]
    #[inline(always)]
    pub fn lcd_cmd_value(&self) -> LCD_CMD_VALUE_R {
        LCD_CMD_VALUE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The LCD write command value."]
    #[inline(always)]
    pub fn lcd_cmd_value(&mut self) -> LCD_CMD_VALUE_W {
        LCD_CMD_VALUE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD configuration register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcd_cmd_val]
(index.html) module"]
pub struct LCD_CMD_VAL_SPEC;
impl crate::RegisterSpec for LCD_CMD_VAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcd_cmd_val::R]
(R) reader structure"]
impl crate::Readable for LCD_CMD_VAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcd_cmd_val::W]
(W) writer structure"]
impl crate::Writable for LCD_CMD_VAL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LCD_CMD_VAL to value 0"]
impl crate::Resettable for LCD_CMD_VAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
