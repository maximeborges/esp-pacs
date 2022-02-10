#[doc = "Register `FUNC%s_IN_SEL_CFG` reader"]
pub struct R(crate::R<FUNC_IN_SEL_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FUNC_IN_SEL_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FUNC_IN_SEL_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FUNC_IN_SEL_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FUNC%s_IN_SEL_CFG` writer"]
pub struct W(crate::W<FUNC_IN_SEL_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FUNC_IN_SEL_CFG_SPEC>;
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
impl From<crate::W<FUNC_IN_SEL_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FUNC_IN_SEL_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IN_SEL` reader - set this value: s=-53: connect GPIO\\[s\\]
 to this port. s=x38: set this port always high level. s=x3C: set this port always low level."]
pub struct IN_SEL_R(crate::FieldReader<u8, u8>);
impl IN_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IN_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN_SEL` writer - set this value: s=-53: connect GPIO\\[s\\]
 to this port. s=x38: set this port always high level. s=x3C: set this port always low level."]
pub struct IN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Field `IN_INV_SEL` reader - set this bit to invert input signal. 1:invert. :not invert."]
pub struct IN_INV_SEL_R(crate::FieldReader<bool, bool>);
impl IN_INV_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IN_INV_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN_INV_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN_INV_SEL` writer - set this bit to invert input signal. 1:invert. :not invert."]
pub struct IN_INV_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_INV_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `SEL` reader - set this bit to bypass GPIO. 1:do not bypass GPIO. :bypass GPIO."]
pub struct SEL_R(crate::FieldReader<bool, bool>);
impl SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEL` writer - set this bit to bypass GPIO. 1:do not bypass GPIO. :bypass GPIO."]
pub struct SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - set this value: s=-53: connect GPIO\\[s\\]
 to this port. s=x38: set this port always high level. s=x3C: set this port always low level."]
    #[inline(always)]
    pub fn in_sel(&self) -> IN_SEL_R {
        IN_SEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - set this bit to invert input signal. 1:invert. :not invert."]
    #[inline(always)]
    pub fn in_inv_sel(&self) -> IN_INV_SEL_R {
        IN_INV_SEL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - set this bit to bypass GPIO. 1:do not bypass GPIO. :bypass GPIO."]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - set this value: s=-53: connect GPIO\\[s\\]
 to this port. s=x38: set this port always high level. s=x3C: set this port always low level."]
    #[inline(always)]
    pub fn in_sel(&mut self) -> IN_SEL_W {
        IN_SEL_W { w: self }
    }
    #[doc = "Bit 5 - set this bit to invert input signal. 1:invert. :not invert."]
    #[inline(always)]
    pub fn in_inv_sel(&mut self) -> IN_INV_SEL_W {
        IN_INV_SEL_W { w: self }
    }
    #[doc = "Bit 6 - set this bit to bypass GPIO. 1:do not bypass GPIO. :bypass GPIO."]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W {
        SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO input function configuration register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [func_in_sel_cfg]
(index.html) module"]
pub struct FUNC_IN_SEL_CFG_SPEC;
impl crate::RegisterSpec for FUNC_IN_SEL_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [func_in_sel_cfg::R]
(R) reader structure"]
impl crate::Readable for FUNC_IN_SEL_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [func_in_sel_cfg::W]
(W) writer structure"]
impl crate::Writable for FUNC_IN_SEL_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FUNC%s_IN_SEL_CFG to value 0"]
impl crate::Resettable for FUNC_IN_SEL_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
