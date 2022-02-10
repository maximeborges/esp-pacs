#[doc = "Register `APPCPU_CTRL_A` reader"]
pub struct R(crate::R<APPCPU_CTRL_A_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APPCPU_CTRL_A_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APPCPU_CTRL_A_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APPCPU_CTRL_A_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APPCPU_CTRL_A` writer"]
pub struct W(crate::W<APPCPU_CTRL_A_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APPCPU_CTRL_A_SPEC>;
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
impl From<crate::W<APPCPU_CTRL_A_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APPCPU_CTRL_A_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `APPCPU_RESETTING` reader - "]
pub struct APPCPU_RESETTING_R(crate::FieldReader<bool, bool>);
impl APPCPU_RESETTING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APPCPU_RESETTING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APPCPU_RESETTING_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APPCPU_RESETTING` writer - "]
pub struct APPCPU_RESETTING_W<'a> {
    w: &'a mut W,
}
impl<'a> APPCPU_RESETTING_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn appcpu_resetting(&self) -> APPCPU_RESETTING_R {
        APPCPU_RESETTING_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn appcpu_resetting(&mut self) -> APPCPU_RESETTING_W {
        APPCPU_RESETTING_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [appcpu_ctrl_a]
(index.html) module"]
pub struct APPCPU_CTRL_A_SPEC;
impl crate::RegisterSpec for APPCPU_CTRL_A_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [appcpu_ctrl_a::R]
(R) reader structure"]
impl crate::Readable for APPCPU_CTRL_A_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [appcpu_ctrl_a::W]
(W) writer structure"]
impl crate::Writable for APPCPU_CTRL_A_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APPCPU_CTRL_A to value 0x01"]
impl crate::Resettable for APPCPU_CTRL_A_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
