#[doc = "Register `ESC_CONF1` reader"]
pub struct R(crate::R<ESC_CONF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ESC_CONF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ESC_CONF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ESC_CONF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ESC_CONF1` writer"]
pub struct W(crate::W<ESC_CONF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ESC_CONF1_SPEC>;
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
impl From<crate::W<ESC_CONF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ESC_CONF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ESC_SEQ0` reader - a"]
pub struct ESC_SEQ0_R(crate::FieldReader<u8>);
impl ESC_SEQ0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ESC_SEQ0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ESC_SEQ0_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ESC_SEQ0` writer - a"]
pub struct ESC_SEQ0_W<'a> {
    w: &'a mut W,
}
impl<'a> ESC_SEQ0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `ESC_SEQ0_CHAR0` reader - a"]
pub struct ESC_SEQ0_CHAR0_R(crate::FieldReader<u8>);
impl ESC_SEQ0_CHAR0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ESC_SEQ0_CHAR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ESC_SEQ0_CHAR0_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ESC_SEQ0_CHAR0` writer - a"]
pub struct ESC_SEQ0_CHAR0_W<'a> {
    w: &'a mut W,
}
impl<'a> ESC_SEQ0_CHAR0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `ESC_SEQ0_CHAR1` reader - a"]
pub struct ESC_SEQ0_CHAR1_R(crate::FieldReader<u8>);
impl ESC_SEQ0_CHAR1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ESC_SEQ0_CHAR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ESC_SEQ0_CHAR1_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ESC_SEQ0_CHAR1` writer - a"]
pub struct ESC_SEQ0_CHAR1_W<'a> {
    w: &'a mut W,
}
impl<'a> ESC_SEQ0_CHAR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - a"]
    #[inline(always)]
    pub fn esc_seq0(&self) -> ESC_SEQ0_R {
        ESC_SEQ0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - a"]
    #[inline(always)]
    pub fn esc_seq0_char0(&self) -> ESC_SEQ0_CHAR0_R {
        ESC_SEQ0_CHAR0_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - a"]
    #[inline(always)]
    pub fn esc_seq0_char1(&self) -> ESC_SEQ0_CHAR1_R {
        ESC_SEQ0_CHAR1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - a"]
    #[inline(always)]
    pub fn esc_seq0(&mut self) -> ESC_SEQ0_W {
        ESC_SEQ0_W { w: self }
    }
    #[doc = "Bits 8:15 - a"]
    #[inline(always)]
    pub fn esc_seq0_char0(&mut self) -> ESC_SEQ0_CHAR0_W {
        ESC_SEQ0_CHAR0_W { w: self }
    }
    #[doc = "Bits 16:23 - a"]
    #[inline(always)]
    pub fn esc_seq0_char1(&mut self) -> ESC_SEQ0_CHAR1_W {
        ESC_SEQ0_CHAR1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "a\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [esc_conf1](index.html) module"]
pub struct ESC_CONF1_SPEC;
impl crate::RegisterSpec for ESC_CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [esc_conf1::R](R) reader structure"]
impl crate::Readable for ESC_CONF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [esc_conf1::W](W) writer structure"]
impl crate::Writable for ESC_CONF1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ESC_CONF1 to value 0x00dd_dbdb"]
impl crate::Resettable for ESC_CONF1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x00dd_dbdb
    }
}
