#[doc = "Register `CMD8` reader"]
pub struct R(crate::R<CMD8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMD8` writer"]
pub struct W(crate::W<CMD8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD8_SPEC>;
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
impl From<crate::W<CMD8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMMAND8` reader - command8"]
pub struct COMMAND8_R(crate::FieldReader<u16, u16>);
impl COMMAND8_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        COMMAND8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMMAND8_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMMAND8` writer - command8"]
pub struct COMMAND8_W<'a> {
    w: &'a mut W,
}
impl<'a> COMMAND8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | (value as u32 & 0x3fff);
        self.w
    }
}
#[doc = "Field `COMMAND8_DONE` reader - command8_done"]
pub struct COMMAND8_DONE_R(crate::FieldReader<bool, bool>);
impl COMMAND8_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMMAND8_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMMAND8_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:13 - command8"]
    #[inline(always)]
    pub fn command8(&self) -> COMMAND8_R {
        COMMAND8_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - command8_done"]
    #[inline(always)]
    pub fn command8_done(&self) -> COMMAND8_DONE_R {
        COMMAND8_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13 - command8"]
    #[inline(always)]
    pub fn command8(&mut self) -> COMMAND8_W {
        COMMAND8_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "i2c commond8 register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd8]
(index.html) module"]
pub struct CMD8_SPEC;
impl crate::RegisterSpec for CMD8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmd8::R]
(R) reader structure"]
impl crate::Readable for CMD8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmd8::W]
(W) writer structure"]
impl crate::Writable for CMD8_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMD8 to value 0x1901"]
impl crate::Resettable for CMD8_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1901
    }
}
