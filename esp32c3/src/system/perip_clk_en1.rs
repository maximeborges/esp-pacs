#[doc = "Register `PERIP_CLK_EN1` reader"]
pub struct R(crate::R<PERIP_CLK_EN1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERIP_CLK_EN1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERIP_CLK_EN1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERIP_CLK_EN1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERIP_CLK_EN1` writer"]
pub struct W(crate::W<PERIP_CLK_EN1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERIP_CLK_EN1_SPEC>;
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
impl From<crate::W<PERIP_CLK_EN1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERIP_CLK_EN1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRYPTO_AES_CLK_EN` reader - reg_crypto_aes_clk_en"]
pub struct CRYPTO_AES_CLK_EN_R(crate::FieldReader<bool, bool>);
impl CRYPTO_AES_CLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CRYPTO_AES_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRYPTO_AES_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRYPTO_AES_CLK_EN` writer - reg_crypto_aes_clk_en"]
pub struct CRYPTO_AES_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYPTO_AES_CLK_EN_W<'a> {
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
#[doc = "Field `CRYPTO_SHA_CLK_EN` reader - reg_crypto_sha_clk_en"]
pub struct CRYPTO_SHA_CLK_EN_R(crate::FieldReader<bool, bool>);
impl CRYPTO_SHA_CLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CRYPTO_SHA_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRYPTO_SHA_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRYPTO_SHA_CLK_EN` writer - reg_crypto_sha_clk_en"]
pub struct CRYPTO_SHA_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYPTO_SHA_CLK_EN_W<'a> {
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
#[doc = "Field `CRYPTO_RSA_CLK_EN` reader - reg_crypto_rsa_clk_en"]
pub struct CRYPTO_RSA_CLK_EN_R(crate::FieldReader<bool, bool>);
impl CRYPTO_RSA_CLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CRYPTO_RSA_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRYPTO_RSA_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRYPTO_RSA_CLK_EN` writer - reg_crypto_rsa_clk_en"]
pub struct CRYPTO_RSA_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYPTO_RSA_CLK_EN_W<'a> {
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
#[doc = "Field `CRYPTO_DS_CLK_EN` reader - reg_crypto_ds_clk_en"]
pub struct CRYPTO_DS_CLK_EN_R(crate::FieldReader<bool, bool>);
impl CRYPTO_DS_CLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CRYPTO_DS_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRYPTO_DS_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRYPTO_DS_CLK_EN` writer - reg_crypto_ds_clk_en"]
pub struct CRYPTO_DS_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYPTO_DS_CLK_EN_W<'a> {
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
#[doc = "Field `CRYPTO_HMAC_CLK_EN` reader - reg_crypto_hmac_clk_en"]
pub struct CRYPTO_HMAC_CLK_EN_R(crate::FieldReader<bool, bool>);
impl CRYPTO_HMAC_CLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CRYPTO_HMAC_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRYPTO_HMAC_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRYPTO_HMAC_CLK_EN` writer - reg_crypto_hmac_clk_en"]
pub struct CRYPTO_HMAC_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYPTO_HMAC_CLK_EN_W<'a> {
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
#[doc = "Field `DMA_CLK_EN` reader - reg_dma_clk_en"]
pub struct DMA_CLK_EN_R(crate::FieldReader<bool, bool>);
impl DMA_CLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_CLK_EN` writer - reg_dma_clk_en"]
pub struct DMA_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_CLK_EN_W<'a> {
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
#[doc = "Field `SDIO_HOST_CLK_EN` reader - reg_sdio_host_clk_en"]
pub struct SDIO_HOST_CLK_EN_R(crate::FieldReader<bool, bool>);
impl SDIO_HOST_CLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDIO_HOST_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIO_HOST_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIO_HOST_CLK_EN` writer - reg_sdio_host_clk_en"]
pub struct SDIO_HOST_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_HOST_CLK_EN_W<'a> {
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
#[doc = "Field `LCD_CAM_CLK_EN` reader - reg_lcd_cam_clk_en"]
pub struct LCD_CAM_CLK_EN_R(crate::FieldReader<bool, bool>);
impl LCD_CAM_CLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LCD_CAM_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCD_CAM_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCD_CAM_CLK_EN` writer - reg_lcd_cam_clk_en"]
pub struct LCD_CAM_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LCD_CAM_CLK_EN_W<'a> {
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
#[doc = "Field `UART2_CLK_EN` reader - reg_uart2_clk_en"]
pub struct UART2_CLK_EN_R(crate::FieldReader<bool, bool>);
impl UART2_CLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UART2_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART2_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART2_CLK_EN` writer - reg_uart2_clk_en"]
pub struct UART2_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART2_CLK_EN_W<'a> {
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
#[doc = "Field `TSENS_CLK_EN` reader - reg_tsens_clk_en"]
pub struct TSENS_CLK_EN_R(crate::FieldReader<bool, bool>);
impl TSENS_CLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TSENS_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSENS_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSENS_CLK_EN` writer - reg_tsens_clk_en"]
pub struct TSENS_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TSENS_CLK_EN_W<'a> {
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
    #[doc = "Bit 1 - reg_crypto_aes_clk_en"]
    #[inline(always)]
    pub fn crypto_aes_clk_en(&self) -> CRYPTO_AES_CLK_EN_R {
        CRYPTO_AES_CLK_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - reg_crypto_sha_clk_en"]
    #[inline(always)]
    pub fn crypto_sha_clk_en(&self) -> CRYPTO_SHA_CLK_EN_R {
        CRYPTO_SHA_CLK_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reg_crypto_rsa_clk_en"]
    #[inline(always)]
    pub fn crypto_rsa_clk_en(&self) -> CRYPTO_RSA_CLK_EN_R {
        CRYPTO_RSA_CLK_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - reg_crypto_ds_clk_en"]
    #[inline(always)]
    pub fn crypto_ds_clk_en(&self) -> CRYPTO_DS_CLK_EN_R {
        CRYPTO_DS_CLK_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - reg_crypto_hmac_clk_en"]
    #[inline(always)]
    pub fn crypto_hmac_clk_en(&self) -> CRYPTO_HMAC_CLK_EN_R {
        CRYPTO_HMAC_CLK_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - reg_dma_clk_en"]
    #[inline(always)]
    pub fn dma_clk_en(&self) -> DMA_CLK_EN_R {
        DMA_CLK_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - reg_sdio_host_clk_en"]
    #[inline(always)]
    pub fn sdio_host_clk_en(&self) -> SDIO_HOST_CLK_EN_R {
        SDIO_HOST_CLK_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - reg_lcd_cam_clk_en"]
    #[inline(always)]
    pub fn lcd_cam_clk_en(&self) -> LCD_CAM_CLK_EN_R {
        LCD_CAM_CLK_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - reg_uart2_clk_en"]
    #[inline(always)]
    pub fn uart2_clk_en(&self) -> UART2_CLK_EN_R {
        UART2_CLK_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - reg_tsens_clk_en"]
    #[inline(always)]
    pub fn tsens_clk_en(&self) -> TSENS_CLK_EN_R {
        TSENS_CLK_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - reg_crypto_aes_clk_en"]
    #[inline(always)]
    pub fn crypto_aes_clk_en(&mut self) -> CRYPTO_AES_CLK_EN_W {
        CRYPTO_AES_CLK_EN_W { w: self }
    }
    #[doc = "Bit 2 - reg_crypto_sha_clk_en"]
    #[inline(always)]
    pub fn crypto_sha_clk_en(&mut self) -> CRYPTO_SHA_CLK_EN_W {
        CRYPTO_SHA_CLK_EN_W { w: self }
    }
    #[doc = "Bit 3 - reg_crypto_rsa_clk_en"]
    #[inline(always)]
    pub fn crypto_rsa_clk_en(&mut self) -> CRYPTO_RSA_CLK_EN_W {
        CRYPTO_RSA_CLK_EN_W { w: self }
    }
    #[doc = "Bit 4 - reg_crypto_ds_clk_en"]
    #[inline(always)]
    pub fn crypto_ds_clk_en(&mut self) -> CRYPTO_DS_CLK_EN_W {
        CRYPTO_DS_CLK_EN_W { w: self }
    }
    #[doc = "Bit 5 - reg_crypto_hmac_clk_en"]
    #[inline(always)]
    pub fn crypto_hmac_clk_en(&mut self) -> CRYPTO_HMAC_CLK_EN_W {
        CRYPTO_HMAC_CLK_EN_W { w: self }
    }
    #[doc = "Bit 6 - reg_dma_clk_en"]
    #[inline(always)]
    pub fn dma_clk_en(&mut self) -> DMA_CLK_EN_W {
        DMA_CLK_EN_W { w: self }
    }
    #[doc = "Bit 7 - reg_sdio_host_clk_en"]
    #[inline(always)]
    pub fn sdio_host_clk_en(&mut self) -> SDIO_HOST_CLK_EN_W {
        SDIO_HOST_CLK_EN_W { w: self }
    }
    #[doc = "Bit 8 - reg_lcd_cam_clk_en"]
    #[inline(always)]
    pub fn lcd_cam_clk_en(&mut self) -> LCD_CAM_CLK_EN_W {
        LCD_CAM_CLK_EN_W { w: self }
    }
    #[doc = "Bit 9 - reg_uart2_clk_en"]
    #[inline(always)]
    pub fn uart2_clk_en(&mut self) -> UART2_CLK_EN_W {
        UART2_CLK_EN_W { w: self }
    }
    #[doc = "Bit 10 - reg_tsens_clk_en"]
    #[inline(always)]
    pub fn tsens_clk_en(&mut self) -> TSENS_CLK_EN_W {
        TSENS_CLK_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "peripheral clock gating register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perip_clk_en1]
(index.html) module"]
pub struct PERIP_CLK_EN1_SPEC;
impl crate::RegisterSpec for PERIP_CLK_EN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [perip_clk_en1::R]
(R) reader structure"]
impl crate::Readable for PERIP_CLK_EN1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [perip_clk_en1::W]
(W) writer structure"]
impl crate::Writable for PERIP_CLK_EN1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PERIP_CLK_EN1 to value 0x0200"]
impl crate::Resettable for PERIP_CLK_EN1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200
    }
}
