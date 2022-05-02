#[doc = "Register `CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE` reader"]
pub struct R(crate::R<CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE` writer"]
pub struct W(crate::W<CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE_SPEC>;
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
impl From<crate::W<CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RECORD_DISABLE_DB_ENCRYPT` reader - Reserved."]
pub struct RECORD_DISABLE_DB_ENCRYPT_R(crate::FieldReader<bool>);
impl RECORD_DISABLE_DB_ENCRYPT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RECORD_DISABLE_DB_ENCRYPT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RECORD_DISABLE_DB_ENCRYPT_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RECORD_DISABLE_DB_ENCRYPT` writer - Reserved."]
pub struct RECORD_DISABLE_DB_ENCRYPT_W<'a> {
    w: &'a mut W,
}
impl<'a> RECORD_DISABLE_DB_ENCRYPT_W<'a> {
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
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
#[doc = "Field `RECORD_DISABLE_G0CB_DECRYPT` reader - Reserved."]
pub struct RECORD_DISABLE_G0CB_DECRYPT_R(crate::FieldReader<bool>);
impl RECORD_DISABLE_G0CB_DECRYPT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RECORD_DISABLE_G0CB_DECRYPT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RECORD_DISABLE_G0CB_DECRYPT_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RECORD_DISABLE_G0CB_DECRYPT` writer - Reserved."]
pub struct RECORD_DISABLE_G0CB_DECRYPT_W<'a> {
    w: &'a mut W,
}
impl<'a> RECORD_DISABLE_G0CB_DECRYPT_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u32 & 1) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Reserved."]
    #[inline(always)]
    pub fn record_disable_db_encrypt(&self) -> RECORD_DISABLE_DB_ENCRYPT_R {
        RECORD_DISABLE_DB_ENCRYPT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved."]
    #[inline(always)]
    pub fn record_disable_g0cb_decrypt(&self) -> RECORD_DISABLE_G0CB_DECRYPT_R {
        RECORD_DISABLE_G0CB_DECRYPT_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reserved."]
    #[inline(always)]
    pub fn record_disable_db_encrypt(&mut self) -> RECORD_DISABLE_DB_ENCRYPT_W {
        RECORD_DISABLE_DB_ENCRYPT_W { w: self }
    }
    #[doc = "Bit 1 - Reserved."]
    #[inline(always)]
    pub fn record_disable_g0cb_decrypt(&mut self) -> RECORD_DISABLE_G0CB_DECRYPT_W {
        RECORD_DISABLE_G0CB_DECRYPT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This description will be updated in the near future.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cache_encrypt_decrypt_record_disable](index.html) module"]
pub struct CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE_SPEC;
impl crate::RegisterSpec for CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cache_encrypt_decrypt_record_disable::R](R) reader structure"]
impl crate::Readable for CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cache_encrypt_decrypt_record_disable::W](W) writer structure"]
impl crate::Writable for CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE to value 0"]
impl crate::Resettable for CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
