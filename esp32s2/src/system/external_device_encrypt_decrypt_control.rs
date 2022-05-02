#[doc = "Register `EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL` reader"]
pub struct R(crate::R<EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL` writer"]
pub struct W(crate::W<EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_SPEC>;
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
impl From<crate::W<EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE_SPI_MANUAL_ENCRYPT` reader - Set this bit to enable Manual Encryption under SPI Boot mode."]
pub struct ENABLE_SPI_MANUAL_ENCRYPT_R(crate::FieldReader<bool>);
impl ENABLE_SPI_MANUAL_ENCRYPT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_SPI_MANUAL_ENCRYPT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLE_SPI_MANUAL_ENCRYPT_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE_SPI_MANUAL_ENCRYPT` writer - Set this bit to enable Manual Encryption under SPI Boot mode."]
pub struct ENABLE_SPI_MANUAL_ENCRYPT_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_SPI_MANUAL_ENCRYPT_W<'a> {
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
#[doc = "Field `ENABLE_DOWNLOAD_DB_ENCRYPT` reader - Set this bit to enable Auto Encryption under Download Boot mode."]
pub struct ENABLE_DOWNLOAD_DB_ENCRYPT_R(crate::FieldReader<bool>);
impl ENABLE_DOWNLOAD_DB_ENCRYPT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_DOWNLOAD_DB_ENCRYPT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLE_DOWNLOAD_DB_ENCRYPT_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE_DOWNLOAD_DB_ENCRYPT` writer - Set this bit to enable Auto Encryption under Download Boot mode."]
pub struct ENABLE_DOWNLOAD_DB_ENCRYPT_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_DOWNLOAD_DB_ENCRYPT_W<'a> {
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
#[doc = "Field `ENABLE_DOWNLOAD_G0CB_DECRYPT` reader - Set this bit to enable Auto Decryption under Download Boot mode."]
pub struct ENABLE_DOWNLOAD_G0CB_DECRYPT_R(crate::FieldReader<bool>);
impl ENABLE_DOWNLOAD_G0CB_DECRYPT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_DOWNLOAD_G0CB_DECRYPT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLE_DOWNLOAD_G0CB_DECRYPT_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE_DOWNLOAD_G0CB_DECRYPT` writer - Set this bit to enable Auto Decryption under Download Boot mode."]
pub struct ENABLE_DOWNLOAD_G0CB_DECRYPT_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_DOWNLOAD_G0CB_DECRYPT_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u32 & 1) << 2);
        self.w
    }
}
#[doc = "Field `ENABLE_DOWNLOAD_MANUAL_ENCRYPT` reader - Set this bit to enable Manual Encryption under Download Boot mode."]
pub struct ENABLE_DOWNLOAD_MANUAL_ENCRYPT_R(crate::FieldReader<bool>);
impl ENABLE_DOWNLOAD_MANUAL_ENCRYPT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_DOWNLOAD_MANUAL_ENCRYPT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLE_DOWNLOAD_MANUAL_ENCRYPT_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE_DOWNLOAD_MANUAL_ENCRYPT` writer - Set this bit to enable Manual Encryption under Download Boot mode."]
pub struct ENABLE_DOWNLOAD_MANUAL_ENCRYPT_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_DOWNLOAD_MANUAL_ENCRYPT_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u32 & 1) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Set this bit to enable Manual Encryption under SPI Boot mode."]
    #[inline(always)]
    pub fn enable_spi_manual_encrypt(&self) -> ENABLE_SPI_MANUAL_ENCRYPT_R {
        ENABLE_SPI_MANUAL_ENCRYPT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to enable Auto Encryption under Download Boot mode."]
    #[inline(always)]
    pub fn enable_download_db_encrypt(&self) -> ENABLE_DOWNLOAD_DB_ENCRYPT_R {
        ENABLE_DOWNLOAD_DB_ENCRYPT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to enable Auto Decryption under Download Boot mode."]
    #[inline(always)]
    pub fn enable_download_g0cb_decrypt(&self) -> ENABLE_DOWNLOAD_G0CB_DECRYPT_R {
        ENABLE_DOWNLOAD_G0CB_DECRYPT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to enable Manual Encryption under Download Boot mode."]
    #[inline(always)]
    pub fn enable_download_manual_encrypt(&self) -> ENABLE_DOWNLOAD_MANUAL_ENCRYPT_R {
        ENABLE_DOWNLOAD_MANUAL_ENCRYPT_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to enable Manual Encryption under SPI Boot mode."]
    #[inline(always)]
    pub fn enable_spi_manual_encrypt(&mut self) -> ENABLE_SPI_MANUAL_ENCRYPT_W {
        ENABLE_SPI_MANUAL_ENCRYPT_W { w: self }
    }
    #[doc = "Bit 1 - Set this bit to enable Auto Encryption under Download Boot mode."]
    #[inline(always)]
    pub fn enable_download_db_encrypt(&mut self) -> ENABLE_DOWNLOAD_DB_ENCRYPT_W {
        ENABLE_DOWNLOAD_DB_ENCRYPT_W { w: self }
    }
    #[doc = "Bit 2 - Set this bit to enable Auto Decryption under Download Boot mode."]
    #[inline(always)]
    pub fn enable_download_g0cb_decrypt(&mut self) -> ENABLE_DOWNLOAD_G0CB_DECRYPT_W {
        ENABLE_DOWNLOAD_G0CB_DECRYPT_W { w: self }
    }
    #[doc = "Bit 3 - Set this bit to enable Manual Encryption under Download Boot mode."]
    #[inline(always)]
    pub fn enable_download_manual_encrypt(&mut self) -> ENABLE_DOWNLOAD_MANUAL_ENCRYPT_W {
        ENABLE_DOWNLOAD_MANUAL_ENCRYPT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "External memory encrypt and decrypt controlling register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [external_device_encrypt_decrypt_control](index.html) module"]
pub struct EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_SPEC;
impl crate::RegisterSpec for EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [external_device_encrypt_decrypt_control::R](R) reader structure"]
impl crate::Readable for EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [external_device_encrypt_decrypt_control::W](W) writer structure"]
impl crate::Writable for EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL to value 0"]
impl crate::Resettable for EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
