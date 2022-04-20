#[doc = "Register `CMD13` reader"]
pub struct R(crate::R<CMD13_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD13_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD13_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD13_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMD13` writer"]
pub struct W(crate::W<CMD13_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD13_SPEC>;
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
impl From<crate::W<CMD13_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD13_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMMAND13` reader - command13"]
pub struct COMMAND13_R(crate::FieldReader<u16, u16>);
impl COMMAND13_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        COMMAND13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMMAND13_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMMAND13` writer - command13"]
pub struct COMMAND13_W<'a> {
    w: &'a mut W,
}
impl<'a> COMMAND13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | (value as u32 & 0x3fff);
        self.w
    }
}
#[doc = "Field `COMMAND13_DONE` reader - command13_done"]
pub struct COMMAND13_DONE_R(crate::FieldReader<bool, bool>);
impl COMMAND13_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMMAND13_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMMAND13_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:13 - command13"]
    #[inline(always)]
    pub fn command13(&self) -> COMMAND13_R {
        COMMAND13_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - command13_done"]
    #[inline(always)]
    pub fn command13_done(&self) -> COMMAND13_DONE_R {
        COMMAND13_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13 - command13"]
    #[inline(always)]
    pub fn command13(&mut self) -> COMMAND13_W {
        COMMAND13_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "i2c commond13 register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd13]
(index.html) module"]
pub struct CMD13_SPEC;
impl crate::RegisterSpec for CMD13_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmd13::R]
(R) reader structure"]
impl crate::Readable for CMD13_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmd13::W]
(W) writer structure"]
impl crate::Writable for CMD13_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMD13 to value 0x1901"]
impl crate::Resettable for CMD13_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1901
    }
}
