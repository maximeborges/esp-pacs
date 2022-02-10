#[doc = "Register `ACK_NUM` reader"]
pub struct R(crate::R<ACK_NUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACK_NUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACK_NUM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACK_NUM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACK_NUM` writer"]
pub struct W(crate::W<ACK_NUM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACK_NUM_SPEC>;
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
impl From<crate::W<ACK_NUM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACK_NUM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACK_NUM` reader - a"]
pub struct ACK_NUM_R(crate::FieldReader<u8, u8>);
impl ACK_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ACK_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACK_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACK_NUM` writer - a"]
pub struct ACK_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> ACK_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `LOAD` writer - a"]
pub struct LOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> LOAD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - a"]
    #[inline(always)]
    pub fn ack_num(&self) -> ACK_NUM_R {
        ACK_NUM_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - a"]
    #[inline(always)]
    pub fn ack_num(&mut self) -> ACK_NUM_W {
        ACK_NUM_W { w: self }
    }
    #[doc = "Bit 3 - a"]
    #[inline(always)]
    pub fn load(&mut self) -> LOAD_W {
        LOAD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "a\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ack_num]
(index.html) module"]
pub struct ACK_NUM_SPEC;
impl crate::RegisterSpec for ACK_NUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ack_num::R]
(R) reader structure"]
impl crate::Readable for ACK_NUM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ack_num::W]
(W) writer structure"]
impl crate::Writable for ACK_NUM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ACK_NUM to value 0x08"]
impl crate::Resettable for ACK_NUM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x08
    }
}
