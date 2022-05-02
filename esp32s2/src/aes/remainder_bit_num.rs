#[doc = "Register `REMAINDER_BIT_NUM` reader"]
pub struct R(crate::R<REMAINDER_BIT_NUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REMAINDER_BIT_NUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REMAINDER_BIT_NUM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REMAINDER_BIT_NUM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REMAINDER_BIT_NUM` writer"]
pub struct W(crate::W<REMAINDER_BIT_NUM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REMAINDER_BIT_NUM_SPEC>;
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
impl From<crate::W<REMAINDER_BIT_NUM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REMAINDER_BIT_NUM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REMAINDER_BIT_NUM` reader - Stores the Remainder Bit Number for the GCM operation."]
pub struct REMAINDER_BIT_NUM_R(crate::FieldReader<u8>);
impl REMAINDER_BIT_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        REMAINDER_BIT_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REMAINDER_BIT_NUM_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REMAINDER_BIT_NUM` writer - Stores the Remainder Bit Number for the GCM operation."]
pub struct REMAINDER_BIT_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> REMAINDER_BIT_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Stores the Remainder Bit Number for the GCM operation."]
    #[inline(always)]
    pub fn remainder_bit_num(&self) -> REMAINDER_BIT_NUM_R {
        REMAINDER_BIT_NUM_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Stores the Remainder Bit Number for the GCM operation."]
    #[inline(always)]
    pub fn remainder_bit_num(&mut self) -> REMAINDER_BIT_NUM_W {
        REMAINDER_BIT_NUM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Remainder bit number of plaintext/ciphertext\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [remainder_bit_num](index.html) module"]
pub struct REMAINDER_BIT_NUM_SPEC;
impl crate::RegisterSpec for REMAINDER_BIT_NUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [remainder_bit_num::R](R) reader structure"]
impl crate::Readable for REMAINDER_BIT_NUM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [remainder_bit_num::W](W) writer structure"]
impl crate::Writable for REMAINDER_BIT_NUM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REMAINDER_BIT_NUM to value 0"]
impl crate::Resettable for REMAINDER_BIT_NUM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
