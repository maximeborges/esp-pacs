#[doc = "Register `BACKUP_BUS_PMS_CONSTRAIN_4` reader"]
pub struct R(crate::R<BACKUP_BUS_PMS_CONSTRAIN_4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BACKUP_BUS_PMS_CONSTRAIN_4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BACKUP_BUS_PMS_CONSTRAIN_4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BACKUP_BUS_PMS_CONSTRAIN_4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BACKUP_BUS_PMS_CONSTRAIN_4` writer"]
pub struct W(crate::W<BACKUP_BUS_PMS_CONSTRAIN_4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BACKUP_BUS_PMS_CONSTRAIN_4_SPEC>;
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
impl From<crate::W<BACKUP_BUS_PMS_CONSTRAIN_4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BACKUP_BUS_PMS_CONSTRAIN_4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_USB_DEVICE` reader - BackUp access usb_device permission."]
pub struct BACKUP_BUS_PMS_CONSTRAIN_USB_DEVICE_R(crate::FieldReader<u8, u8>);
impl BACKUP_BUS_PMS_CONSTRAIN_USB_DEVICE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BACKUP_BUS_PMS_CONSTRAIN_USB_DEVICE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BACKUP_BUS_PMS_CONSTRAIN_USB_DEVICE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_USB_DEVICE` writer - BackUp access usb_device permission."]
pub struct BACKUP_BUS_PMS_CONSTRAIN_USB_DEVICE_W<'a> {
    w: &'a mut W,
}
impl<'a> BACKUP_BUS_PMS_CONSTRAIN_USB_DEVICE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u32 & 3);
        self.w
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_USB_WRAP` reader - BackUp access usb_wrap permission."]
pub struct BACKUP_BUS_PMS_CONSTRAIN_USB_WRAP_R(crate::FieldReader<u8, u8>);
impl BACKUP_BUS_PMS_CONSTRAIN_USB_WRAP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BACKUP_BUS_PMS_CONSTRAIN_USB_WRAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BACKUP_BUS_PMS_CONSTRAIN_USB_WRAP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_USB_WRAP` writer - BackUp access usb_wrap permission."]
pub struct BACKUP_BUS_PMS_CONSTRAIN_USB_WRAP_W<'a> {
    w: &'a mut W,
}
impl<'a> BACKUP_BUS_PMS_CONSTRAIN_USB_WRAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 2)) | ((value as u32 & 3) << 2);
        self.w
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_PERI` reader - BackUp access crypto_peri permission."]
pub struct BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_PERI_R(crate::FieldReader<u8, u8>);
impl BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_PERI_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_PERI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_PERI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_PERI` writer - BackUp access crypto_peri permission."]
pub struct BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_PERI_W<'a> {
    w: &'a mut W,
}
impl<'a> BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_PERI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 4)) | ((value as u32 & 3) << 4);
        self.w
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_DMA` reader - BackUp access crypto_dma permission."]
pub struct BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_DMA_R(crate::FieldReader<u8, u8>);
impl BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_DMA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_DMA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_DMA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_DMA` writer - BackUp access crypto_dma permission."]
pub struct BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_DMA_W<'a> {
    w: &'a mut W,
}
impl<'a> BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_DMA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 6)) | ((value as u32 & 3) << 6);
        self.w
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_APB_ADC` reader - BackUp access apb_adc permission."]
pub struct BACKUP_BUS_PMS_CONSTRAIN_APB_ADC_R(crate::FieldReader<u8, u8>);
impl BACKUP_BUS_PMS_CONSTRAIN_APB_ADC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BACKUP_BUS_PMS_CONSTRAIN_APB_ADC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BACKUP_BUS_PMS_CONSTRAIN_APB_ADC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_APB_ADC` writer - BackUp access apb_adc permission."]
pub struct BACKUP_BUS_PMS_CONSTRAIN_APB_ADC_W<'a> {
    w: &'a mut W,
}
impl<'a> BACKUP_BUS_PMS_CONSTRAIN_APB_ADC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 8)) | ((value as u32 & 3) << 8);
        self.w
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_LCD_CAM` reader - BackUp access lcd_cam permission."]
pub struct BACKUP_BUS_PMS_CONSTRAIN_LCD_CAM_R(crate::FieldReader<u8, u8>);
impl BACKUP_BUS_PMS_CONSTRAIN_LCD_CAM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BACKUP_BUS_PMS_CONSTRAIN_LCD_CAM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BACKUP_BUS_PMS_CONSTRAIN_LCD_CAM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_LCD_CAM` writer - BackUp access lcd_cam permission."]
pub struct BACKUP_BUS_PMS_CONSTRAIN_LCD_CAM_W<'a> {
    w: &'a mut W,
}
impl<'a> BACKUP_BUS_PMS_CONSTRAIN_LCD_CAM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 10)) | ((value as u32 & 3) << 10);
        self.w
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_BT_PWR` reader - BackUp access bt_pwr permission."]
pub struct BACKUP_BUS_PMS_CONSTRAIN_BT_PWR_R(crate::FieldReader<u8, u8>);
impl BACKUP_BUS_PMS_CONSTRAIN_BT_PWR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BACKUP_BUS_PMS_CONSTRAIN_BT_PWR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BACKUP_BUS_PMS_CONSTRAIN_BT_PWR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_BT_PWR` writer - BackUp access bt_pwr permission."]
pub struct BACKUP_BUS_PMS_CONSTRAIN_BT_PWR_W<'a> {
    w: &'a mut W,
}
impl<'a> BACKUP_BUS_PMS_CONSTRAIN_BT_PWR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 12)) | ((value as u32 & 3) << 12);
        self.w
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_USB` reader - BackUp access usb permission."]
pub struct BACKUP_BUS_PMS_CONSTRAIN_USB_R(crate::FieldReader<u8, u8>);
impl BACKUP_BUS_PMS_CONSTRAIN_USB_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BACKUP_BUS_PMS_CONSTRAIN_USB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BACKUP_BUS_PMS_CONSTRAIN_USB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_USB` writer - BackUp access usb permission."]
pub struct BACKUP_BUS_PMS_CONSTRAIN_USB_W<'a> {
    w: &'a mut W,
}
impl<'a> BACKUP_BUS_PMS_CONSTRAIN_USB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 14)) | ((value as u32 & 3) << 14);
        self.w
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_SYSTEM` reader - BackUp access system permission."]
pub struct BACKUP_BUS_PMS_CONSTRAIN_SYSTEM_R(crate::FieldReader<u8, u8>);
impl BACKUP_BUS_PMS_CONSTRAIN_SYSTEM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BACKUP_BUS_PMS_CONSTRAIN_SYSTEM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BACKUP_BUS_PMS_CONSTRAIN_SYSTEM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_SYSTEM` writer - BackUp access system permission."]
pub struct BACKUP_BUS_PMS_CONSTRAIN_SYSTEM_W<'a> {
    w: &'a mut W,
}
impl<'a> BACKUP_BUS_PMS_CONSTRAIN_SYSTEM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 16)) | ((value as u32 & 3) << 16);
        self.w
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_SENSITIVE` reader - BackUp access sensitive permission."]
pub struct BACKUP_BUS_PMS_CONSTRAIN_SENSITIVE_R(crate::FieldReader<u8, u8>);
impl BACKUP_BUS_PMS_CONSTRAIN_SENSITIVE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BACKUP_BUS_PMS_CONSTRAIN_SENSITIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BACKUP_BUS_PMS_CONSTRAIN_SENSITIVE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_SENSITIVE` writer - BackUp access sensitive permission."]
pub struct BACKUP_BUS_PMS_CONSTRAIN_SENSITIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> BACKUP_BUS_PMS_CONSTRAIN_SENSITIVE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 18)) | ((value as u32 & 3) << 18);
        self.w
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_INTERRUPT` reader - BackUp access interrupt permission."]
pub struct BACKUP_BUS_PMS_CONSTRAIN_INTERRUPT_R(crate::FieldReader<u8, u8>);
impl BACKUP_BUS_PMS_CONSTRAIN_INTERRUPT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BACKUP_BUS_PMS_CONSTRAIN_INTERRUPT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BACKUP_BUS_PMS_CONSTRAIN_INTERRUPT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_INTERRUPT` writer - BackUp access interrupt permission."]
pub struct BACKUP_BUS_PMS_CONSTRAIN_INTERRUPT_W<'a> {
    w: &'a mut W,
}
impl<'a> BACKUP_BUS_PMS_CONSTRAIN_INTERRUPT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 20)) | ((value as u32 & 3) << 20);
        self.w
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_DMA_COPY` reader - BackUp access dma_copy permission."]
pub struct BACKUP_BUS_PMS_CONSTRAIN_DMA_COPY_R(crate::FieldReader<u8, u8>);
impl BACKUP_BUS_PMS_CONSTRAIN_DMA_COPY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BACKUP_BUS_PMS_CONSTRAIN_DMA_COPY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BACKUP_BUS_PMS_CONSTRAIN_DMA_COPY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_DMA_COPY` writer - BackUp access dma_copy permission."]
pub struct BACKUP_BUS_PMS_CONSTRAIN_DMA_COPY_W<'a> {
    w: &'a mut W,
}
impl<'a> BACKUP_BUS_PMS_CONSTRAIN_DMA_COPY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 22)) | ((value as u32 & 3) << 22);
        self.w
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_CACHE_CONFIG` reader - BackUp access cache_config permission."]
pub struct BACKUP_BUS_PMS_CONSTRAIN_CACHE_CONFIG_R(crate::FieldReader<u8, u8>);
impl BACKUP_BUS_PMS_CONSTRAIN_CACHE_CONFIG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BACKUP_BUS_PMS_CONSTRAIN_CACHE_CONFIG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BACKUP_BUS_PMS_CONSTRAIN_CACHE_CONFIG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_CACHE_CONFIG` writer - BackUp access cache_config permission."]
pub struct BACKUP_BUS_PMS_CONSTRAIN_CACHE_CONFIG_W<'a> {
    w: &'a mut W,
}
impl<'a> BACKUP_BUS_PMS_CONSTRAIN_CACHE_CONFIG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 24)) | ((value as u32 & 3) << 24);
        self.w
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_AD` reader - BackUp access ad permission."]
pub struct BACKUP_BUS_PMS_CONSTRAIN_AD_R(crate::FieldReader<u8, u8>);
impl BACKUP_BUS_PMS_CONSTRAIN_AD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BACKUP_BUS_PMS_CONSTRAIN_AD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BACKUP_BUS_PMS_CONSTRAIN_AD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_AD` writer - BackUp access ad permission."]
pub struct BACKUP_BUS_PMS_CONSTRAIN_AD_W<'a> {
    w: &'a mut W,
}
impl<'a> BACKUP_BUS_PMS_CONSTRAIN_AD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 26)) | ((value as u32 & 3) << 26);
        self.w
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_DIO` reader - BackUp access dio permission."]
pub struct BACKUP_BUS_PMS_CONSTRAIN_DIO_R(crate::FieldReader<u8, u8>);
impl BACKUP_BUS_PMS_CONSTRAIN_DIO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BACKUP_BUS_PMS_CONSTRAIN_DIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BACKUP_BUS_PMS_CONSTRAIN_DIO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_DIO` writer - BackUp access dio permission."]
pub struct BACKUP_BUS_PMS_CONSTRAIN_DIO_W<'a> {
    w: &'a mut W,
}
impl<'a> BACKUP_BUS_PMS_CONSTRAIN_DIO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 28)) | ((value as u32 & 3) << 28);
        self.w
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_WORLD_CONTROLLER` reader - BackUp access world_controller permission."]
pub struct BACKUP_BUS_PMS_CONSTRAIN_WORLD_CONTROLLER_R(crate::FieldReader<u8, u8>);
impl BACKUP_BUS_PMS_CONSTRAIN_WORLD_CONTROLLER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BACKUP_BUS_PMS_CONSTRAIN_WORLD_CONTROLLER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BACKUP_BUS_PMS_CONSTRAIN_WORLD_CONTROLLER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_WORLD_CONTROLLER` writer - BackUp access world_controller permission."]
pub struct BACKUP_BUS_PMS_CONSTRAIN_WORLD_CONTROLLER_W<'a> {
    w: &'a mut W,
}
impl<'a> BACKUP_BUS_PMS_CONSTRAIN_WORLD_CONTROLLER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 30)) | ((value as u32 & 3) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - BackUp access usb_device permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_usb_device(&self) -> BACKUP_BUS_PMS_CONSTRAIN_USB_DEVICE_R {
        BACKUP_BUS_PMS_CONSTRAIN_USB_DEVICE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - BackUp access usb_wrap permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_usb_wrap(&self) -> BACKUP_BUS_PMS_CONSTRAIN_USB_WRAP_R {
        BACKUP_BUS_PMS_CONSTRAIN_USB_WRAP_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - BackUp access crypto_peri permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_crypto_peri(&self) -> BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_PERI_R {
        BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_PERI_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - BackUp access crypto_dma permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_crypto_dma(&self) -> BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_DMA_R {
        BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_DMA_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - BackUp access apb_adc permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_apb_adc(&self) -> BACKUP_BUS_PMS_CONSTRAIN_APB_ADC_R {
        BACKUP_BUS_PMS_CONSTRAIN_APB_ADC_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - BackUp access lcd_cam permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_lcd_cam(&self) -> BACKUP_BUS_PMS_CONSTRAIN_LCD_CAM_R {
        BACKUP_BUS_PMS_CONSTRAIN_LCD_CAM_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - BackUp access bt_pwr permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_bt_pwr(&self) -> BACKUP_BUS_PMS_CONSTRAIN_BT_PWR_R {
        BACKUP_BUS_PMS_CONSTRAIN_BT_PWR_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - BackUp access usb permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_usb(&self) -> BACKUP_BUS_PMS_CONSTRAIN_USB_R {
        BACKUP_BUS_PMS_CONSTRAIN_USB_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - BackUp access system permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_system(&self) -> BACKUP_BUS_PMS_CONSTRAIN_SYSTEM_R {
        BACKUP_BUS_PMS_CONSTRAIN_SYSTEM_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - BackUp access sensitive permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_sensitive(&self) -> BACKUP_BUS_PMS_CONSTRAIN_SENSITIVE_R {
        BACKUP_BUS_PMS_CONSTRAIN_SENSITIVE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - BackUp access interrupt permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_interrupt(&self) -> BACKUP_BUS_PMS_CONSTRAIN_INTERRUPT_R {
        BACKUP_BUS_PMS_CONSTRAIN_INTERRUPT_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - BackUp access dma_copy permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_dma_copy(&self) -> BACKUP_BUS_PMS_CONSTRAIN_DMA_COPY_R {
        BACKUP_BUS_PMS_CONSTRAIN_DMA_COPY_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - BackUp access cache_config permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_cache_config(&self) -> BACKUP_BUS_PMS_CONSTRAIN_CACHE_CONFIG_R {
        BACKUP_BUS_PMS_CONSTRAIN_CACHE_CONFIG_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - BackUp access ad permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_ad(&self) -> BACKUP_BUS_PMS_CONSTRAIN_AD_R {
        BACKUP_BUS_PMS_CONSTRAIN_AD_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - BackUp access dio permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_dio(&self) -> BACKUP_BUS_PMS_CONSTRAIN_DIO_R {
        BACKUP_BUS_PMS_CONSTRAIN_DIO_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - BackUp access world_controller permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_world_controller(
        &self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_WORLD_CONTROLLER_R {
        BACKUP_BUS_PMS_CONSTRAIN_WORLD_CONTROLLER_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - BackUp access usb_device permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_usb_device(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_USB_DEVICE_W {
        BACKUP_BUS_PMS_CONSTRAIN_USB_DEVICE_W { w: self }
    }
    #[doc = "Bits 2:3 - BackUp access usb_wrap permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_usb_wrap(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_USB_WRAP_W {
        BACKUP_BUS_PMS_CONSTRAIN_USB_WRAP_W { w: self }
    }
    #[doc = "Bits 4:5 - BackUp access crypto_peri permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_crypto_peri(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_PERI_W {
        BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_PERI_W { w: self }
    }
    #[doc = "Bits 6:7 - BackUp access crypto_dma permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_crypto_dma(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_DMA_W {
        BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_DMA_W { w: self }
    }
    #[doc = "Bits 8:9 - BackUp access apb_adc permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_apb_adc(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_APB_ADC_W {
        BACKUP_BUS_PMS_CONSTRAIN_APB_ADC_W { w: self }
    }
    #[doc = "Bits 10:11 - BackUp access lcd_cam permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_lcd_cam(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_LCD_CAM_W {
        BACKUP_BUS_PMS_CONSTRAIN_LCD_CAM_W { w: self }
    }
    #[doc = "Bits 12:13 - BackUp access bt_pwr permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_bt_pwr(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_BT_PWR_W {
        BACKUP_BUS_PMS_CONSTRAIN_BT_PWR_W { w: self }
    }
    #[doc = "Bits 14:15 - BackUp access usb permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_usb(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_USB_W {
        BACKUP_BUS_PMS_CONSTRAIN_USB_W { w: self }
    }
    #[doc = "Bits 16:17 - BackUp access system permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_system(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_SYSTEM_W {
        BACKUP_BUS_PMS_CONSTRAIN_SYSTEM_W { w: self }
    }
    #[doc = "Bits 18:19 - BackUp access sensitive permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_sensitive(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_SENSITIVE_W {
        BACKUP_BUS_PMS_CONSTRAIN_SENSITIVE_W { w: self }
    }
    #[doc = "Bits 20:21 - BackUp access interrupt permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_interrupt(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_INTERRUPT_W {
        BACKUP_BUS_PMS_CONSTRAIN_INTERRUPT_W { w: self }
    }
    #[doc = "Bits 22:23 - BackUp access dma_copy permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_dma_copy(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_DMA_COPY_W {
        BACKUP_BUS_PMS_CONSTRAIN_DMA_COPY_W { w: self }
    }
    #[doc = "Bits 24:25 - BackUp access cache_config permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_cache_config(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_CACHE_CONFIG_W {
        BACKUP_BUS_PMS_CONSTRAIN_CACHE_CONFIG_W { w: self }
    }
    #[doc = "Bits 26:27 - BackUp access ad permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_ad(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_AD_W {
        BACKUP_BUS_PMS_CONSTRAIN_AD_W { w: self }
    }
    #[doc = "Bits 28:29 - BackUp access dio permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_dio(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_DIO_W {
        BACKUP_BUS_PMS_CONSTRAIN_DIO_W { w: self }
    }
    #[doc = "Bits 30:31 - BackUp access world_controller permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_world_controller(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_WORLD_CONTROLLER_W {
        BACKUP_BUS_PMS_CONSTRAIN_WORLD_CONTROLLER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BackUp access peripherals permission configuration register 4.\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [backup_bus_pms_constrain_4]
(index.html) module"]
pub struct BACKUP_BUS_PMS_CONSTRAIN_4_SPEC;
impl crate::RegisterSpec for BACKUP_BUS_PMS_CONSTRAIN_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [backup_bus_pms_constrain_4::R]
(R) reader structure"]
impl crate::Readable for BACKUP_BUS_PMS_CONSTRAIN_4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [backup_bus_pms_constrain_4::W]
(W) writer structure"]
impl crate::Writable for BACKUP_BUS_PMS_CONSTRAIN_4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BACKUP_BUS_PMS_CONSTRAIN_4 to value 0xffff_ffff"]
impl crate::Resettable for BACKUP_BUS_PMS_CONSTRAIN_4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
