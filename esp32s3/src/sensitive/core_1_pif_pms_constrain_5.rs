#[doc = "Register `CORE_1_PIF_PMS_CONSTRAIN_5` reader"]
pub struct R(crate::R<CORE_1_PIF_PMS_CONSTRAIN_5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_1_PIF_PMS_CONSTRAIN_5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_1_PIF_PMS_CONSTRAIN_5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_1_PIF_PMS_CONSTRAIN_5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_1_PIF_PMS_CONSTRAIN_5` writer"]
pub struct W(crate::W<CORE_1_PIF_PMS_CONSTRAIN_5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_1_PIF_PMS_CONSTRAIN_5_SPEC>;
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
impl From<crate::W<CORE_1_PIF_PMS_CONSTRAIN_5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_1_PIF_PMS_CONSTRAIN_5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_UART` reader - Core1 access uart permission in world1."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_UART_R(crate::FieldReader<u8, u8>);
impl CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_UART_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_UART_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_UART_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_UART` writer - Core1 access uart permission in world1."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_UART_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_UART_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_G0SPI_1` reader - Core1 access g0spi_1 permission in world1."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_G0SPI_1_R(crate::FieldReader<u8, u8>);
impl CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_G0SPI_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_G0SPI_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_G0SPI_1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_G0SPI_1` writer - Core1 access g0spi_1 permission in world1."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_G0SPI_1_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_G0SPI_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_G0SPI_0` reader - Core1 access g0spi_0 permission in world1."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_G0SPI_0_R(crate::FieldReader<u8, u8>);
impl CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_G0SPI_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_G0SPI_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_G0SPI_0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_G0SPI_0` writer - Core1 access g0spi_0 permission in world1."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_G0SPI_0_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_G0SPI_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_GPIO` reader - Core1 access gpio permission in world1."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_GPIO_R(crate::FieldReader<u8, u8>);
impl CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_GPIO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_GPIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_GPIO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_GPIO` writer - Core1 access gpio permission in world1."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_GPIO_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_GPIO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_FE2` reader - Core1 access fe2 permission in world1."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_FE2_R(crate::FieldReader<u8, u8>);
impl CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_FE2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_FE2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_FE2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_FE2` writer - Core1 access fe2 permission in world1."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_FE2_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_FE2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_FE` reader - Core1 access fe permission in world1."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_FE_R(crate::FieldReader<u8, u8>);
impl CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_FE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_FE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_FE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_FE` writer - Core1 access fe permission in world1."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_FE_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_FE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_RTC` reader - Core1 access rtc permission in world1."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_RTC_R(crate::FieldReader<u8, u8>);
impl CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_RTC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_RTC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_RTC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_RTC` writer - Core1 access rtc permission in world1."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_RTC_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_RTC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_IO_MUX` reader - Core1 access io_mux permission in world1."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_IO_MUX_R(crate::FieldReader<u8, u8>);
impl CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_IO_MUX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_IO_MUX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_IO_MUX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_IO_MUX` writer - Core1 access io_mux permission in world1."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_IO_MUX_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_IO_MUX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_HINF` reader - Core1 access hinf permission in world1."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_HINF_R(crate::FieldReader<u8, u8>);
impl CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_HINF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_HINF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_HINF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_HINF` writer - Core1 access hinf permission in world1."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_HINF_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_HINF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_MISC` reader - Core1 access misc permission in world1."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_MISC_R(crate::FieldReader<u8, u8>);
impl CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_MISC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_MISC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_MISC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_MISC` writer - Core1 access misc permission in world1."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_MISC_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_MISC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_I2C` reader - Core1 access i2c permission in world1."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_I2C_R(crate::FieldReader<u8, u8>);
impl CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_I2C_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_I2C_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_I2C_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_I2C` writer - Core1 access i2c permission in world1."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_I2C_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_I2C_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | ((value as u32 & 0x03) << 26);
        self.w
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_I2S0` reader - Core1 access i2s0 permission in world1."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_I2S0_R(crate::FieldReader<u8, u8>);
impl CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_I2S0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_I2S0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_I2S0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_I2S0` writer - Core1 access i2s0 permission in world1."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_I2S0_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_I2S0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_UART1` reader - Core1 access uart1 permission in world1."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_UART1_R(crate::FieldReader<u8, u8>);
impl CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_UART1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_UART1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_UART1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_UART1` writer - Core1 access uart1 permission in world1."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_UART1_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_UART1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Core1 access uart permission in world1."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_1_uart(&self) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_UART_R {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_UART_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Core1 access g0spi_1 permission in world1."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_1_g0spi_1(
        &self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_G0SPI_1_R {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_G0SPI_1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Core1 access g0spi_0 permission in world1."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_1_g0spi_0(
        &self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_G0SPI_0_R {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_G0SPI_0_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Core1 access gpio permission in world1."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_1_gpio(&self) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_GPIO_R {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_GPIO_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Core1 access fe2 permission in world1."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_1_fe2(&self) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_FE2_R {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_FE2_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Core1 access fe permission in world1."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_1_fe(&self) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_FE_R {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_FE_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Core1 access rtc permission in world1."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_1_rtc(&self) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_RTC_R {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_RTC_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Core1 access io_mux permission in world1."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_1_io_mux(
        &self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_IO_MUX_R {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_IO_MUX_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Core1 access hinf permission in world1."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_1_hinf(&self) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_HINF_R {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_HINF_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - Core1 access misc permission in world1."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_1_misc(&self) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_MISC_R {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_MISC_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - Core1 access i2c permission in world1."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_1_i2c(&self) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_I2C_R {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_I2C_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - Core1 access i2s0 permission in world1."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_1_i2s0(&self) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_I2S0_R {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_I2S0_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - Core1 access uart1 permission in world1."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_1_uart1(
        &self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_UART1_R {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_UART1_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Core1 access uart permission in world1."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_1_uart(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_UART_W {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_UART_W { w: self }
    }
    #[doc = "Bits 2:3 - Core1 access g0spi_1 permission in world1."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_1_g0spi_1(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_G0SPI_1_W {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_G0SPI_1_W { w: self }
    }
    #[doc = "Bits 4:5 - Core1 access g0spi_0 permission in world1."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_1_g0spi_0(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_G0SPI_0_W {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_G0SPI_0_W { w: self }
    }
    #[doc = "Bits 6:7 - Core1 access gpio permission in world1."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_1_gpio(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_GPIO_W {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_GPIO_W { w: self }
    }
    #[doc = "Bits 8:9 - Core1 access fe2 permission in world1."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_1_fe2(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_FE2_W {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_FE2_W { w: self }
    }
    #[doc = "Bits 10:11 - Core1 access fe permission in world1."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_1_fe(&mut self) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_FE_W {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_FE_W { w: self }
    }
    #[doc = "Bits 14:15 - Core1 access rtc permission in world1."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_1_rtc(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_RTC_W {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_RTC_W { w: self }
    }
    #[doc = "Bits 16:17 - Core1 access io_mux permission in world1."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_1_io_mux(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_IO_MUX_W {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_IO_MUX_W { w: self }
    }
    #[doc = "Bits 20:21 - Core1 access hinf permission in world1."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_1_hinf(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_HINF_W {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_HINF_W { w: self }
    }
    #[doc = "Bits 24:25 - Core1 access misc permission in world1."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_1_misc(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_MISC_W {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_MISC_W { w: self }
    }
    #[doc = "Bits 26:27 - Core1 access i2c permission in world1."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_1_i2c(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_I2C_W {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_I2C_W { w: self }
    }
    #[doc = "Bits 28:29 - Core1 access i2s0 permission in world1."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_1_i2s0(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_I2S0_W {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_I2S0_W { w: self }
    }
    #[doc = "Bits 30:31 - Core1 access uart1 permission in world1."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_1_uart1(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_UART1_W {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_1_UART1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Core1 access peripherals permission configuration register 5.\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_1_pif_pms_constrain_5]
(index.html) module"]
pub struct CORE_1_PIF_PMS_CONSTRAIN_5_SPEC;
impl crate::RegisterSpec for CORE_1_PIF_PMS_CONSTRAIN_5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_1_pif_pms_constrain_5::R]
(R) reader structure"]
impl crate::Readable for CORE_1_PIF_PMS_CONSTRAIN_5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_1_pif_pms_constrain_5::W]
(W) writer structure"]
impl crate::Writable for CORE_1_PIF_PMS_CONSTRAIN_5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CORE_1_PIF_PMS_CONSTRAIN_5 to value 0xff33_cfff"]
impl crate::Resettable for CORE_1_PIF_PMS_CONSTRAIN_5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff33_cfff
    }
}
