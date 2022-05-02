#[doc = "Register `PERIP_RST_EN1` reader"]
pub struct R(crate::R<PERIP_RST_EN1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERIP_RST_EN1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERIP_RST_EN1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERIP_RST_EN1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERIP_RST_EN1` writer"]
pub struct W(crate::W<PERIP_RST_EN1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERIP_RST_EN1_SPEC>;
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
impl From<crate::W<PERIP_RST_EN1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERIP_RST_EN1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERI_BACKUP_RST` reader - Set 1 to let BACKUP reset"]
pub struct PERI_BACKUP_RST_R(crate::FieldReader<bool>);
impl PERI_BACKUP_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PERI_BACKUP_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERI_BACKUP_RST_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PERI_BACKUP_RST` writer - Set 1 to let BACKUP reset"]
pub struct PERI_BACKUP_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> PERI_BACKUP_RST_W<'a> {
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
#[doc = "Field `CRYPTO_AES_RST` reader - Set 1 to let CRYPTO_AES reset"]
pub struct CRYPTO_AES_RST_R(crate::FieldReader<bool>);
impl CRYPTO_AES_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CRYPTO_AES_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRYPTO_AES_RST_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRYPTO_AES_RST` writer - Set 1 to let CRYPTO_AES reset"]
pub struct CRYPTO_AES_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYPTO_AES_RST_W<'a> {
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
#[doc = "Field `CRYPTO_SHA_RST` reader - Set 1 to let CRYPTO_SHA reset"]
pub struct CRYPTO_SHA_RST_R(crate::FieldReader<bool>);
impl CRYPTO_SHA_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CRYPTO_SHA_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRYPTO_SHA_RST_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRYPTO_SHA_RST` writer - Set 1 to let CRYPTO_SHA reset"]
pub struct CRYPTO_SHA_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYPTO_SHA_RST_W<'a> {
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
#[doc = "Field `CRYPTO_RSA_RST` reader - Set 1 to let CRYPTO_RSA reset"]
pub struct CRYPTO_RSA_RST_R(crate::FieldReader<bool>);
impl CRYPTO_RSA_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CRYPTO_RSA_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRYPTO_RSA_RST_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRYPTO_RSA_RST` writer - Set 1 to let CRYPTO_RSA reset"]
pub struct CRYPTO_RSA_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYPTO_RSA_RST_W<'a> {
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
#[doc = "Field `CRYPTO_DS_RST` reader - Set 1 to let CRYPTO_DS reset"]
pub struct CRYPTO_DS_RST_R(crate::FieldReader<bool>);
impl CRYPTO_DS_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CRYPTO_DS_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRYPTO_DS_RST_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRYPTO_DS_RST` writer - Set 1 to let CRYPTO_DS reset"]
pub struct CRYPTO_DS_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYPTO_DS_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u32 & 1) << 4);
        self.w
    }
}
#[doc = "Field `CRYPTO_HMAC_RST` reader - Set 1 to let CRYPTO_HMAC reset"]
pub struct CRYPTO_HMAC_RST_R(crate::FieldReader<bool>);
impl CRYPTO_HMAC_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CRYPTO_HMAC_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRYPTO_HMAC_RST_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRYPTO_HMAC_RST` writer - Set 1 to let CRYPTO_HMAC reset"]
pub struct CRYPTO_HMAC_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYPTO_HMAC_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u32 & 1) << 5);
        self.w
    }
}
#[doc = "Field `DMA_RST` reader - Set 1 to let DMA reset"]
pub struct DMA_RST_R(crate::FieldReader<bool>);
impl DMA_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_RST_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_RST` writer - Set 1 to let DMA reset"]
pub struct DMA_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 6)) | ((value as u32 & 1) << 6);
        self.w
    }
}
#[doc = "Field `SDIO_HOST_RST` reader - Set 1 to let SDIO_HOST reset"]
pub struct SDIO_HOST_RST_R(crate::FieldReader<bool>);
impl SDIO_HOST_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDIO_HOST_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIO_HOST_RST_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIO_HOST_RST` writer - Set 1 to let SDIO_HOST reset"]
pub struct SDIO_HOST_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_HOST_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 7)) | ((value as u32 & 1) << 7);
        self.w
    }
}
#[doc = "Field `LCD_CAM_RST` reader - Set 1 to let LCD_CAM reset"]
pub struct LCD_CAM_RST_R(crate::FieldReader<bool>);
impl LCD_CAM_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LCD_CAM_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCD_CAM_RST_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCD_CAM_RST` writer - Set 1 to let LCD_CAM reset"]
pub struct LCD_CAM_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> LCD_CAM_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 8)) | ((value as u32 & 1) << 8);
        self.w
    }
}
#[doc = "Field `UART2_RST` reader - Set 1 to let UART2 reset"]
pub struct UART2_RST_R(crate::FieldReader<bool>);
impl UART2_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UART2_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART2_RST_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART2_RST` writer - Set 1 to let UART2 reset"]
pub struct UART2_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> UART2_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 9)) | ((value as u32 & 1) << 9);
        self.w
    }
}
#[doc = "Field `USB_DEVICE_RST` reader - Set 1 to let USB_DEVICE reset"]
pub struct USB_DEVICE_RST_R(crate::FieldReader<bool>);
impl USB_DEVICE_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USB_DEVICE_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_DEVICE_RST_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB_DEVICE_RST` writer - Set 1 to let USB_DEVICE reset"]
pub struct USB_DEVICE_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_DEVICE_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 10)) | ((value as u32 & 1) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Set 1 to let BACKUP reset"]
    #[inline(always)]
    pub fn peri_backup_rst(&self) -> PERI_BACKUP_RST_R {
        PERI_BACKUP_RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 1 to let CRYPTO_AES reset"]
    #[inline(always)]
    pub fn crypto_aes_rst(&self) -> CRYPTO_AES_RST_R {
        CRYPTO_AES_RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set 1 to let CRYPTO_SHA reset"]
    #[inline(always)]
    pub fn crypto_sha_rst(&self) -> CRYPTO_SHA_RST_R {
        CRYPTO_SHA_RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set 1 to let CRYPTO_RSA reset"]
    #[inline(always)]
    pub fn crypto_rsa_rst(&self) -> CRYPTO_RSA_RST_R {
        CRYPTO_RSA_RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set 1 to let CRYPTO_DS reset"]
    #[inline(always)]
    pub fn crypto_ds_rst(&self) -> CRYPTO_DS_RST_R {
        CRYPTO_DS_RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set 1 to let CRYPTO_HMAC reset"]
    #[inline(always)]
    pub fn crypto_hmac_rst(&self) -> CRYPTO_HMAC_RST_R {
        CRYPTO_HMAC_RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set 1 to let DMA reset"]
    #[inline(always)]
    pub fn dma_rst(&self) -> DMA_RST_R {
        DMA_RST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Set 1 to let SDIO_HOST reset"]
    #[inline(always)]
    pub fn sdio_host_rst(&self) -> SDIO_HOST_RST_R {
        SDIO_HOST_RST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Set 1 to let LCD_CAM reset"]
    #[inline(always)]
    pub fn lcd_cam_rst(&self) -> LCD_CAM_RST_R {
        LCD_CAM_RST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Set 1 to let UART2 reset"]
    #[inline(always)]
    pub fn uart2_rst(&self) -> UART2_RST_R {
        UART2_RST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Set 1 to let USB_DEVICE reset"]
    #[inline(always)]
    pub fn usb_device_rst(&self) -> USB_DEVICE_RST_R {
        USB_DEVICE_RST_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to let BACKUP reset"]
    #[inline(always)]
    pub fn peri_backup_rst(&mut self) -> PERI_BACKUP_RST_W {
        PERI_BACKUP_RST_W { w: self }
    }
    #[doc = "Bit 1 - Set 1 to let CRYPTO_AES reset"]
    #[inline(always)]
    pub fn crypto_aes_rst(&mut self) -> CRYPTO_AES_RST_W {
        CRYPTO_AES_RST_W { w: self }
    }
    #[doc = "Bit 2 - Set 1 to let CRYPTO_SHA reset"]
    #[inline(always)]
    pub fn crypto_sha_rst(&mut self) -> CRYPTO_SHA_RST_W {
        CRYPTO_SHA_RST_W { w: self }
    }
    #[doc = "Bit 3 - Set 1 to let CRYPTO_RSA reset"]
    #[inline(always)]
    pub fn crypto_rsa_rst(&mut self) -> CRYPTO_RSA_RST_W {
        CRYPTO_RSA_RST_W { w: self }
    }
    #[doc = "Bit 4 - Set 1 to let CRYPTO_DS reset"]
    #[inline(always)]
    pub fn crypto_ds_rst(&mut self) -> CRYPTO_DS_RST_W {
        CRYPTO_DS_RST_W { w: self }
    }
    #[doc = "Bit 5 - Set 1 to let CRYPTO_HMAC reset"]
    #[inline(always)]
    pub fn crypto_hmac_rst(&mut self) -> CRYPTO_HMAC_RST_W {
        CRYPTO_HMAC_RST_W { w: self }
    }
    #[doc = "Bit 6 - Set 1 to let DMA reset"]
    #[inline(always)]
    pub fn dma_rst(&mut self) -> DMA_RST_W {
        DMA_RST_W { w: self }
    }
    #[doc = "Bit 7 - Set 1 to let SDIO_HOST reset"]
    #[inline(always)]
    pub fn sdio_host_rst(&mut self) -> SDIO_HOST_RST_W {
        SDIO_HOST_RST_W { w: self }
    }
    #[doc = "Bit 8 - Set 1 to let LCD_CAM reset"]
    #[inline(always)]
    pub fn lcd_cam_rst(&mut self) -> LCD_CAM_RST_W {
        LCD_CAM_RST_W { w: self }
    }
    #[doc = "Bit 9 - Set 1 to let UART2 reset"]
    #[inline(always)]
    pub fn uart2_rst(&mut self) -> UART2_RST_W {
        UART2_RST_W { w: self }
    }
    #[doc = "Bit 10 - Set 1 to let USB_DEVICE reset"]
    #[inline(always)]
    pub fn usb_device_rst(&mut self) -> USB_DEVICE_RST_W {
        USB_DEVICE_RST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "peripheral reset configuration regsiter 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perip_rst_en1](index.html) module"]
pub struct PERIP_RST_EN1_SPEC;
impl crate::RegisterSpec for PERIP_RST_EN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [perip_rst_en1::R](R) reader structure"]
impl crate::Readable for PERIP_RST_EN1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [perip_rst_en1::W](W) writer structure"]
impl crate::Writable for PERIP_RST_EN1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PERIP_RST_EN1 to value 0x01fe"]
impl crate::Resettable for PERIP_RST_EN1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01fe
    }
}
