#[doc = "Register `CORE_1_PIF_PMS_CONSTRAIN_3` reader"]
pub struct R(crate::R<CORE_1_PIF_PMS_CONSTRAIN_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_1_PIF_PMS_CONSTRAIN_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_1_PIF_PMS_CONSTRAIN_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_1_PIF_PMS_CONSTRAIN_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_1_PIF_PMS_CONSTRAIN_3` writer"]
pub struct W(crate::W<CORE_1_PIF_PMS_CONSTRAIN_3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_1_PIF_PMS_CONSTRAIN_3_SPEC>;
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
impl From<crate::W<CORE_1_PIF_PMS_CONSTRAIN_3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_1_PIF_PMS_CONSTRAIN_3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SPI_2` reader - Core1 access spi_2 permission in world0."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SPI_2_R(crate::FieldReader<u8, u8>);
impl CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SPI_2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SPI_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SPI_2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SPI_2` writer - Core1 access spi_2 permission in world0."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SPI_2_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SPI_2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SPI_3` reader - Core1 access spi_3 permission in world0."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SPI_3_R(crate::FieldReader<u8, u8>);
impl CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SPI_3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SPI_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SPI_3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SPI_3` writer - Core1 access spi_3 permission in world0."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SPI_3_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SPI_3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_APB_CTRL` reader - Core1 access apb_ctrl permission in world0."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_APB_CTRL_R(crate::FieldReader<u8, u8>);
impl CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_APB_CTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_APB_CTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_APB_CTRL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_APB_CTRL` writer - Core1 access apb_ctrl permission in world0."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_APB_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_APB_CTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_I2C_EXT1` reader - Core1 access i2c_ext1 permission in world0."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_I2C_EXT1_R(crate::FieldReader<u8, u8>);
impl CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_I2C_EXT1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_I2C_EXT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_I2C_EXT1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_I2C_EXT1` writer - Core1 access i2c_ext1 permission in world0."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_I2C_EXT1_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_I2C_EXT1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SDIO_HOST` reader - Core1 access sdio_host permission in world0."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SDIO_HOST_R(crate::FieldReader<u8, u8>);
impl CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SDIO_HOST_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SDIO_HOST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SDIO_HOST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SDIO_HOST` writer - Core1 access sdio_host permission in world0."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SDIO_HOST_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SDIO_HOST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_CAN` reader - Core1 access can permission in world0."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_CAN_R(crate::FieldReader<u8, u8>);
impl CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_CAN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_CAN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_CAN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_CAN` writer - Core1 access can permission in world0."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_CAN_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_CAN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_PWM1` reader - Core1 access pwm1 permission in world0."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_PWM1_R(crate::FieldReader<u8, u8>);
impl CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_PWM1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_PWM1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_PWM1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_PWM1` writer - Core1 access pwm1 permission in world0."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_PWM1_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_PWM1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_I2S1` reader - Core1 access i2s1 permission in world0."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_I2S1_R(crate::FieldReader<u8, u8>);
impl CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_I2S1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_I2S1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_I2S1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_I2S1` writer - Core1 access i2s1 permission in world0."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_I2S1_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_I2S1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_UART2` reader - Core1 access uart2 permission in world0."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_UART2_R(crate::FieldReader<u8, u8>);
impl CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_UART2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_UART2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_UART2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_UART2` writer - Core1 access uart2 permission in world0."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_UART2_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_UART2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_RWBT` reader - Core1 access rwbt permission in world0."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_RWBT_R(crate::FieldReader<u8, u8>);
impl CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_RWBT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_RWBT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_RWBT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_RWBT` writer - Core1 access rwbt permission in world0."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_RWBT_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_RWBT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_WIFIMAC` reader - Core1 access wifimac permission in world0."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_WIFIMAC_R(crate::FieldReader<u8, u8>);
impl CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_WIFIMAC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_WIFIMAC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_WIFIMAC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_WIFIMAC` writer - Core1 access wifimac permission in world0."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_WIFIMAC_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_WIFIMAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | ((value as u32 & 0x03) << 26);
        self.w
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_PWR` reader - Core1 access pwr permission in world0."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_PWR_R(crate::FieldReader<u8, u8>);
impl CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_PWR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_PWR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_PWR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_PWR` writer - Core1 access pwr permission in world0."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_PWR_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_PWR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Core1 access spi_2 permission in world0."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_0_spi_2(
        &self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SPI_2_R {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SPI_2_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Core1 access spi_3 permission in world0."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_0_spi_3(
        &self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SPI_3_R {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SPI_3_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Core1 access apb_ctrl permission in world0."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_0_apb_ctrl(
        &self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_APB_CTRL_R {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_APB_CTRL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Core1 access i2c_ext1 permission in world0."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_0_i2c_ext1(
        &self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_I2C_EXT1_R {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_I2C_EXT1_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Core1 access sdio_host permission in world0."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_0_sdio_host(
        &self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SDIO_HOST_R {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SDIO_HOST_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Core1 access can permission in world0."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_0_can(&self) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_CAN_R {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_CAN_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Core1 access pwm1 permission in world0."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_0_pwm1(&self) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_PWM1_R {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_PWM1_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Core1 access i2s1 permission in world0."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_0_i2s1(&self) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_I2S1_R {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_I2S1_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Core1 access uart2 permission in world0."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_0_uart2(
        &self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_UART2_R {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_UART2_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Core1 access rwbt permission in world0."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_0_rwbt(&self) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_RWBT_R {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_RWBT_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - Core1 access wifimac permission in world0."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_0_wifimac(
        &self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_WIFIMAC_R {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_WIFIMAC_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - Core1 access pwr permission in world0."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_0_pwr(&self) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_PWR_R {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_PWR_R::new(((self.bits >> 28) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Core1 access spi_2 permission in world0."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_0_spi_2(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SPI_2_W {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SPI_2_W { w: self }
    }
    #[doc = "Bits 2:3 - Core1 access spi_3 permission in world0."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_0_spi_3(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SPI_3_W {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SPI_3_W { w: self }
    }
    #[doc = "Bits 4:5 - Core1 access apb_ctrl permission in world0."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_0_apb_ctrl(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_APB_CTRL_W {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_APB_CTRL_W { w: self }
    }
    #[doc = "Bits 6:7 - Core1 access i2c_ext1 permission in world0."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_0_i2c_ext1(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_I2C_EXT1_W {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_I2C_EXT1_W { w: self }
    }
    #[doc = "Bits 8:9 - Core1 access sdio_host permission in world0."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_0_sdio_host(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SDIO_HOST_W {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SDIO_HOST_W { w: self }
    }
    #[doc = "Bits 10:11 - Core1 access can permission in world0."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_0_can(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_CAN_W {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_CAN_W { w: self }
    }
    #[doc = "Bits 12:13 - Core1 access pwm1 permission in world0."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_0_pwm1(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_PWM1_W {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_PWM1_W { w: self }
    }
    #[doc = "Bits 14:15 - Core1 access i2s1 permission in world0."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_0_i2s1(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_I2S1_W {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_I2S1_W { w: self }
    }
    #[doc = "Bits 16:17 - Core1 access uart2 permission in world0."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_0_uart2(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_UART2_W {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_UART2_W { w: self }
    }
    #[doc = "Bits 22:23 - Core1 access rwbt permission in world0."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_0_rwbt(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_RWBT_W {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_RWBT_W { w: self }
    }
    #[doc = "Bits 26:27 - Core1 access wifimac permission in world0."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_0_wifimac(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_WIFIMAC_W {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_WIFIMAC_W { w: self }
    }
    #[doc = "Bits 28:29 - Core1 access pwr permission in world0."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_0_pwr(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_PWR_W {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_PWR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Core1 access peripherals permission configuration register 3.\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_1_pif_pms_constrain_3]
(index.html) module"]
pub struct CORE_1_PIF_PMS_CONSTRAIN_3_SPEC;
impl crate::RegisterSpec for CORE_1_PIF_PMS_CONSTRAIN_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_1_pif_pms_constrain_3::R]
(R) reader structure"]
impl crate::Readable for CORE_1_PIF_PMS_CONSTRAIN_3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_1_pif_pms_constrain_3::W]
(W) writer structure"]
impl crate::Writable for CORE_1_PIF_PMS_CONSTRAIN_3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CORE_1_PIF_PMS_CONSTRAIN_3 to value 0x3cc3_ffff"]
impl crate::Resettable for CORE_1_PIF_PMS_CONSTRAIN_3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3cc3_ffff
    }
}
