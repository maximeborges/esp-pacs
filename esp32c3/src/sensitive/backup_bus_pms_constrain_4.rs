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
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_USB_WRAP` reader - backup_bus_pms_constrain_usb_wrap"]
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
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_USB_WRAP` writer - backup_bus_pms_constrain_usb_wrap"]
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
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_PERI` reader - backup_bus_pms_constrain_crypto_peri"]
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
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_PERI` writer - backup_bus_pms_constrain_crypto_peri"]
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
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_DMA` reader - backup_bus_pms_constrain_crypto_dma"]
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
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_DMA` writer - backup_bus_pms_constrain_crypto_dma"]
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
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_APB_ADC` reader - backup_bus_pms_constrain_apb_adc"]
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
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_APB_ADC` writer - backup_bus_pms_constrain_apb_adc"]
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
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_BT_PWR` reader - backup_bus_pms_constrain_bt_pwr"]
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
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_BT_PWR` writer - backup_bus_pms_constrain_bt_pwr"]
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
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_USB_DEVICE` reader - backup_bus_pms_constrain_usb_device"]
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
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_USB_DEVICE` writer - backup_bus_pms_constrain_usb_device"]
pub struct BACKUP_BUS_PMS_CONSTRAIN_USB_DEVICE_W<'a> {
    w: &'a mut W,
}
impl<'a> BACKUP_BUS_PMS_CONSTRAIN_USB_DEVICE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 14)) | ((value as u32 & 3) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:3 - backup_bus_pms_constrain_usb_wrap"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_usb_wrap(&self) -> BACKUP_BUS_PMS_CONSTRAIN_USB_WRAP_R {
        BACKUP_BUS_PMS_CONSTRAIN_USB_WRAP_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - backup_bus_pms_constrain_crypto_peri"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_crypto_peri(&self) -> BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_PERI_R {
        BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_PERI_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - backup_bus_pms_constrain_crypto_dma"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_crypto_dma(&self) -> BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_DMA_R {
        BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_DMA_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - backup_bus_pms_constrain_apb_adc"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_apb_adc(&self) -> BACKUP_BUS_PMS_CONSTRAIN_APB_ADC_R {
        BACKUP_BUS_PMS_CONSTRAIN_APB_ADC_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - backup_bus_pms_constrain_bt_pwr"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_bt_pwr(&self) -> BACKUP_BUS_PMS_CONSTRAIN_BT_PWR_R {
        BACKUP_BUS_PMS_CONSTRAIN_BT_PWR_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - backup_bus_pms_constrain_usb_device"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_usb_device(&self) -> BACKUP_BUS_PMS_CONSTRAIN_USB_DEVICE_R {
        BACKUP_BUS_PMS_CONSTRAIN_USB_DEVICE_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 2:3 - backup_bus_pms_constrain_usb_wrap"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_usb_wrap(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_USB_WRAP_W {
        BACKUP_BUS_PMS_CONSTRAIN_USB_WRAP_W { w: self }
    }
    #[doc = "Bits 4:5 - backup_bus_pms_constrain_crypto_peri"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_crypto_peri(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_PERI_W {
        BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_PERI_W { w: self }
    }
    #[doc = "Bits 6:7 - backup_bus_pms_constrain_crypto_dma"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_crypto_dma(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_DMA_W {
        BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_DMA_W { w: self }
    }
    #[doc = "Bits 8:9 - backup_bus_pms_constrain_apb_adc"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_apb_adc(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_APB_ADC_W {
        BACKUP_BUS_PMS_CONSTRAIN_APB_ADC_W { w: self }
    }
    #[doc = "Bits 12:13 - backup_bus_pms_constrain_bt_pwr"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_bt_pwr(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_BT_PWR_W {
        BACKUP_BUS_PMS_CONSTRAIN_BT_PWR_W { w: self }
    }
    #[doc = "Bits 14:15 - backup_bus_pms_constrain_usb_device"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_usb_device(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_USB_DEVICE_W {
        BACKUP_BUS_PMS_CONSTRAIN_USB_DEVICE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_4_REG\n\nThis register you can [`read`]
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
#[doc = "`reset()` method sets BACKUP_BUS_PMS_CONSTRAIN_4 to value 0xf3fc"]
impl crate::Resettable for BACKUP_BUS_PMS_CONSTRAIN_4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xf3fc
    }
}
