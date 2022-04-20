#[doc = "Register `CORE_1_PIF_PMS_CONSTRAIN_4` reader"]
pub struct R(crate::R<CORE_1_PIF_PMS_CONSTRAIN_4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_1_PIF_PMS_CONSTRAIN_4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_1_PIF_PMS_CONSTRAIN_4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_1_PIF_PMS_CONSTRAIN_4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_1_PIF_PMS_CONSTRAIN_4` writer"]
pub struct W(crate::W<CORE_1_PIF_PMS_CONSTRAIN_4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_1_PIF_PMS_CONSTRAIN_4_SPEC>;
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
impl From<crate::W<CORE_1_PIF_PMS_CONSTRAIN_4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_1_PIF_PMS_CONSTRAIN_4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_USB_DEVICE` reader - Core1 access usb_device permission in world0."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_USB_DEVICE_R(crate::FieldReader<u8, u8>);
impl CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_USB_DEVICE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_USB_DEVICE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_USB_DEVICE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_USB_DEVICE` writer - Core1 access usb_device permission in world0."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_USB_DEVICE_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_USB_DEVICE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u32 & 3);
        self.w
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_USB_WRAP` reader - Core1 access usb_wrap permission in world0."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_USB_WRAP_R(crate::FieldReader<u8, u8>);
impl CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_USB_WRAP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_USB_WRAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_USB_WRAP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_USB_WRAP` writer - Core1 access usb_wrap permission in world0."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_USB_WRAP_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_USB_WRAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 2)) | ((value as u32 & 3) << 2);
        self.w
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_CRYPTO_PERI` reader - Core1 access crypto_peri permission in world0."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_CRYPTO_PERI_R(crate::FieldReader<u8, u8>);
impl CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_CRYPTO_PERI_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_CRYPTO_PERI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_CRYPTO_PERI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_CRYPTO_PERI` writer - Core1 access crypto_peri permission in world0."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_CRYPTO_PERI_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_CRYPTO_PERI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 4)) | ((value as u32 & 3) << 4);
        self.w
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_CRYPTO_DMA` reader - Core1 access crypto_dma permission in world0."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_CRYPTO_DMA_R(crate::FieldReader<u8, u8>);
impl CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_CRYPTO_DMA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_CRYPTO_DMA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_CRYPTO_DMA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_CRYPTO_DMA` writer - Core1 access crypto_dma permission in world0."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_CRYPTO_DMA_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_CRYPTO_DMA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 6)) | ((value as u32 & 3) << 6);
        self.w
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_APB_ADC` reader - Core1 access apb_adc permission in world0."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_APB_ADC_R(crate::FieldReader<u8, u8>);
impl CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_APB_ADC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_APB_ADC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_APB_ADC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_APB_ADC` writer - Core1 access apb_adc permission in world0."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_APB_ADC_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_APB_ADC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 8)) | ((value as u32 & 3) << 8);
        self.w
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_LCD_CAM` reader - Core1 access lcd_cam permission in world0."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_LCD_CAM_R(crate::FieldReader<u8, u8>);
impl CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_LCD_CAM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_LCD_CAM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_LCD_CAM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_LCD_CAM` writer - Core1 access lcd_cam permission in world0."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_LCD_CAM_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_LCD_CAM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 10)) | ((value as u32 & 3) << 10);
        self.w
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_BT_PWR` reader - Core1 access bt_pwr permission in world0."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_BT_PWR_R(crate::FieldReader<u8, u8>);
impl CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_BT_PWR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_BT_PWR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_BT_PWR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_BT_PWR` writer - Core1 access bt_pwr permission in world0."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_BT_PWR_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_BT_PWR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 12)) | ((value as u32 & 3) << 12);
        self.w
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_USB` reader - Core1 access usb permission in world0."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_USB_R(crate::FieldReader<u8, u8>);
impl CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_USB_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_USB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_USB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_USB` writer - Core1 access usb permission in world0."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_USB_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_USB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 14)) | ((value as u32 & 3) << 14);
        self.w
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SYSTEM` reader - Core1 access system permission in world0."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SYSTEM_R(crate::FieldReader<u8, u8>);
impl CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SYSTEM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SYSTEM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SYSTEM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SYSTEM` writer - Core1 access system permission in world0."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SYSTEM_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SYSTEM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 16)) | ((value as u32 & 3) << 16);
        self.w
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SENSITIVE` reader - Core1 access sensitive permission in world0."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SENSITIVE_R(crate::FieldReader<u8, u8>);
impl CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SENSITIVE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SENSITIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SENSITIVE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SENSITIVE` writer - Core1 access sensitive permission in world0."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SENSITIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SENSITIVE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 18)) | ((value as u32 & 3) << 18);
        self.w
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_INTERRUPT` reader - Core1 access interrupt permission in world0."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_INTERRUPT_R(crate::FieldReader<u8, u8>);
impl CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_INTERRUPT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_INTERRUPT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_INTERRUPT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_INTERRUPT` writer - Core1 access interrupt permission in world0."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_INTERRUPT_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_INTERRUPT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 20)) | ((value as u32 & 3) << 20);
        self.w
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_DMA_COPY` reader - Core1 access dma_copy permission in world0."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_DMA_COPY_R(crate::FieldReader<u8, u8>);
impl CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_DMA_COPY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_DMA_COPY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_DMA_COPY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_DMA_COPY` writer - Core1 access dma_copy permission in world0."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_DMA_COPY_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_DMA_COPY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 22)) | ((value as u32 & 3) << 22);
        self.w
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_CACHE_CONFIG` reader - Core1 access cache_config permission in world0."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_CACHE_CONFIG_R(crate::FieldReader<u8, u8>);
impl CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_CACHE_CONFIG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_CACHE_CONFIG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_CACHE_CONFIG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_CACHE_CONFIG` writer - Core1 access cache_config permission in world0."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_CACHE_CONFIG_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_CACHE_CONFIG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 24)) | ((value as u32 & 3) << 24);
        self.w
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_AD` reader - Core1 access ad permission in world0."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_AD_R(crate::FieldReader<u8, u8>);
impl CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_AD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_AD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_AD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_AD` writer - Core1 access ad permission in world0."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_AD_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_AD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 26)) | ((value as u32 & 3) << 26);
        self.w
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_DIO` reader - Core1 access dio permission in world0."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_DIO_R(crate::FieldReader<u8, u8>);
impl CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_DIO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_DIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_DIO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_DIO` writer - Core1 access dio permission in world0."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_DIO_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_DIO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 28)) | ((value as u32 & 3) << 28);
        self.w
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_WORLD_CONTROLLER` reader - Core1 access world_controller permission in world0."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_WORLD_CONTROLLER_R(crate::FieldReader<u8, u8>);
impl CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_WORLD_CONTROLLER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_WORLD_CONTROLLER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_WORLD_CONTROLLER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_WORLD_CONTROLLER` writer - Core1 access world_controller permission in world0."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_WORLD_CONTROLLER_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_WORLD_CONTROLLER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 30)) | ((value as u32 & 3) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Core1 access usb_device permission in world0."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_0_usb_device(
        &self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_USB_DEVICE_R {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_USB_DEVICE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Core1 access usb_wrap permission in world0."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_0_usb_wrap(
        &self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_USB_WRAP_R {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_USB_WRAP_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Core1 access crypto_peri permission in world0."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_0_crypto_peri(
        &self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_CRYPTO_PERI_R {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_CRYPTO_PERI_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Core1 access crypto_dma permission in world0."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_0_crypto_dma(
        &self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_CRYPTO_DMA_R {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_CRYPTO_DMA_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Core1 access apb_adc permission in world0."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_0_apb_adc(
        &self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_APB_ADC_R {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_APB_ADC_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Core1 access lcd_cam permission in world0."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_0_lcd_cam(
        &self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_LCD_CAM_R {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_LCD_CAM_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Core1 access bt_pwr permission in world0."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_0_bt_pwr(
        &self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_BT_PWR_R {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_BT_PWR_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Core1 access usb permission in world0."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_0_usb(&self) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_USB_R {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_USB_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Core1 access system permission in world0."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_0_system(
        &self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SYSTEM_R {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SYSTEM_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Core1 access sensitive permission in world0."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_0_sensitive(
        &self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SENSITIVE_R {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SENSITIVE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Core1 access interrupt permission in world0."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_0_interrupt(
        &self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_INTERRUPT_R {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_INTERRUPT_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Core1 access dma_copy permission in world0."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_0_dma_copy(
        &self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_DMA_COPY_R {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_DMA_COPY_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Core1 access cache_config permission in world0."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_0_cache_config(
        &self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_CACHE_CONFIG_R {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_CACHE_CONFIG_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Core1 access ad permission in world0."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_0_ad(&self) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_AD_R {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_AD_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Core1 access dio permission in world0."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_0_dio(&self) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_DIO_R {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_DIO_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Core1 access world_controller permission in world0."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_0_world_controller(
        &self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_WORLD_CONTROLLER_R {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_WORLD_CONTROLLER_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Core1 access usb_device permission in world0."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_0_usb_device(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_USB_DEVICE_W {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_USB_DEVICE_W { w: self }
    }
    #[doc = "Bits 2:3 - Core1 access usb_wrap permission in world0."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_0_usb_wrap(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_USB_WRAP_W {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_USB_WRAP_W { w: self }
    }
    #[doc = "Bits 4:5 - Core1 access crypto_peri permission in world0."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_0_crypto_peri(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_CRYPTO_PERI_W {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_CRYPTO_PERI_W { w: self }
    }
    #[doc = "Bits 6:7 - Core1 access crypto_dma permission in world0."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_0_crypto_dma(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_CRYPTO_DMA_W {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_CRYPTO_DMA_W { w: self }
    }
    #[doc = "Bits 8:9 - Core1 access apb_adc permission in world0."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_0_apb_adc(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_APB_ADC_W {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_APB_ADC_W { w: self }
    }
    #[doc = "Bits 10:11 - Core1 access lcd_cam permission in world0."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_0_lcd_cam(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_LCD_CAM_W {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_LCD_CAM_W { w: self }
    }
    #[doc = "Bits 12:13 - Core1 access bt_pwr permission in world0."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_0_bt_pwr(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_BT_PWR_W {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_BT_PWR_W { w: self }
    }
    #[doc = "Bits 14:15 - Core1 access usb permission in world0."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_0_usb(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_USB_W {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_USB_W { w: self }
    }
    #[doc = "Bits 16:17 - Core1 access system permission in world0."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_0_system(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SYSTEM_W {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SYSTEM_W { w: self }
    }
    #[doc = "Bits 18:19 - Core1 access sensitive permission in world0."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_0_sensitive(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SENSITIVE_W {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SENSITIVE_W { w: self }
    }
    #[doc = "Bits 20:21 - Core1 access interrupt permission in world0."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_0_interrupt(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_INTERRUPT_W {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_INTERRUPT_W { w: self }
    }
    #[doc = "Bits 22:23 - Core1 access dma_copy permission in world0."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_0_dma_copy(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_DMA_COPY_W {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_DMA_COPY_W { w: self }
    }
    #[doc = "Bits 24:25 - Core1 access cache_config permission in world0."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_0_cache_config(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_CACHE_CONFIG_W {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_CACHE_CONFIG_W { w: self }
    }
    #[doc = "Bits 26:27 - Core1 access ad permission in world0."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_0_ad(&mut self) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_AD_W {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_AD_W { w: self }
    }
    #[doc = "Bits 28:29 - Core1 access dio permission in world0."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_0_dio(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_DIO_W {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_DIO_W { w: self }
    }
    #[doc = "Bits 30:31 - Core1 access world_controller permission in world0."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_0_world_controller(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_WORLD_CONTROLLER_W {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_WORLD_CONTROLLER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Core1 access peripherals permission configuration register 4.\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_1_pif_pms_constrain_4]
(index.html) module"]
pub struct CORE_1_PIF_PMS_CONSTRAIN_4_SPEC;
impl crate::RegisterSpec for CORE_1_PIF_PMS_CONSTRAIN_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_1_pif_pms_constrain_4::R]
(R) reader structure"]
impl crate::Readable for CORE_1_PIF_PMS_CONSTRAIN_4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_1_pif_pms_constrain_4::W]
(W) writer structure"]
impl crate::Writable for CORE_1_PIF_PMS_CONSTRAIN_4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CORE_1_PIF_PMS_CONSTRAIN_4 to value 0xffff_ffff"]
impl crate::Resettable for CORE_1_PIF_PMS_CONSTRAIN_4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
