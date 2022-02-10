#[doc = "Register `CORE_1_VECBASE_OVERRIDE_1` reader"]
pub struct R(crate::R<CORE_1_VECBASE_OVERRIDE_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_1_VECBASE_OVERRIDE_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_1_VECBASE_OVERRIDE_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_1_VECBASE_OVERRIDE_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_1_VECBASE_OVERRIDE_1` writer"]
pub struct W(crate::W<CORE_1_VECBASE_OVERRIDE_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_1_VECBASE_OVERRIDE_1_SPEC>;
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
impl From<crate::W<CORE_1_VECBASE_OVERRIDE_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_1_VECBASE_OVERRIDE_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_1_VECBASE_OVERRIDE_WORLD0_VALUE` reader - world0 vecbase_override register, when core1 in world0 use this register to override vecbase register."]
pub struct CORE_1_VECBASE_OVERRIDE_WORLD0_VALUE_R(crate::FieldReader<u32, u32>);
impl CORE_1_VECBASE_OVERRIDE_WORLD0_VALUE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CORE_1_VECBASE_OVERRIDE_WORLD0_VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_1_VECBASE_OVERRIDE_WORLD0_VALUE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_1_VECBASE_OVERRIDE_WORLD0_VALUE` writer - world0 vecbase_override register, when core1 in world0 use this register to override vecbase register."]
pub struct CORE_1_VECBASE_OVERRIDE_WORLD0_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_1_VECBASE_OVERRIDE_WORLD0_VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x003f_ffff) | (value as u32 & 0x003f_ffff);
        self.w
    }
}
#[doc = "Field `CORE_1_VECBASE_OVERRIDE_SEL` reader - Set 0x3 to sel vecbase_override to override vecbase register."]
pub struct CORE_1_VECBASE_OVERRIDE_SEL_R(crate::FieldReader<u8, u8>);
impl CORE_1_VECBASE_OVERRIDE_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_1_VECBASE_OVERRIDE_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_1_VECBASE_OVERRIDE_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_1_VECBASE_OVERRIDE_SEL` writer - Set 0x3 to sel vecbase_override to override vecbase register."]
pub struct CORE_1_VECBASE_OVERRIDE_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_1_VECBASE_OVERRIDE_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:21 - world0 vecbase_override register, when core1 in world0 use this register to override vecbase register."]
    #[inline(always)]
    pub fn core_1_vecbase_override_world0_value(&self) -> CORE_1_VECBASE_OVERRIDE_WORLD0_VALUE_R {
        CORE_1_VECBASE_OVERRIDE_WORLD0_VALUE_R::new((self.bits & 0x003f_ffff) as u32)
    }
    #[doc = "Bits 22:23 - Set 0x3 to sel vecbase_override to override vecbase register."]
    #[inline(always)]
    pub fn core_1_vecbase_override_sel(&self) -> CORE_1_VECBASE_OVERRIDE_SEL_R {
        CORE_1_VECBASE_OVERRIDE_SEL_R::new(((self.bits >> 22) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:21 - world0 vecbase_override register, when core1 in world0 use this register to override vecbase register."]
    #[inline(always)]
    pub fn core_1_vecbase_override_world0_value(
        &mut self,
    ) -> CORE_1_VECBASE_OVERRIDE_WORLD0_VALUE_W {
        CORE_1_VECBASE_OVERRIDE_WORLD0_VALUE_W { w: self }
    }
    #[doc = "Bits 22:23 - Set 0x3 to sel vecbase_override to override vecbase register."]
    #[inline(always)]
    pub fn core_1_vecbase_override_sel(&mut self) -> CORE_1_VECBASE_OVERRIDE_SEL_W {
        CORE_1_VECBASE_OVERRIDE_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "core1 vecbase override configuration register 1\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_1_vecbase_override_1]
(index.html) module"]
pub struct CORE_1_VECBASE_OVERRIDE_1_SPEC;
impl crate::RegisterSpec for CORE_1_VECBASE_OVERRIDE_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_1_vecbase_override_1::R]
(R) reader structure"]
impl crate::Readable for CORE_1_VECBASE_OVERRIDE_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_1_vecbase_override_1::W]
(W) writer structure"]
impl crate::Writable for CORE_1_VECBASE_OVERRIDE_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CORE_1_VECBASE_OVERRIDE_1 to value 0"]
impl crate::Resettable for CORE_1_VECBASE_OVERRIDE_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
