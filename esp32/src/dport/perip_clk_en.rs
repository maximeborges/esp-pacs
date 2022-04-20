#[doc = "Register `PERIP_CLK_EN` reader"]
pub struct R(crate::R<PERIP_CLK_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERIP_CLK_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERIP_CLK_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERIP_CLK_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERIP_CLK_EN` writer"]
pub struct W(crate::W<PERIP_CLK_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERIP_CLK_EN_SPEC>;
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
impl From<crate::W<PERIP_CLK_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERIP_CLK_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI01_CLK_EN` reader - SPI0 and SPI1 module."]
pub struct SPI01_CLK_EN_R(crate::FieldReader<bool, bool>);
impl SPI01_CLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPI01_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI01_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI01_CLK_EN` writer - SPI0 and SPI1 module."]
pub struct SPI01_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI01_CLK_EN_W<'a> {
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
#[doc = "Field `UART_CLK_EN` reader - UART0 module."]
pub struct UART_CLK_EN_R(crate::FieldReader<bool, bool>);
impl UART_CLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UART_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART_CLK_EN` writer - UART0 module."]
pub struct UART_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_CLK_EN_W<'a> {
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
#[doc = "Field `I2S0_CLK_EN` reader - I2S0 module."]
pub struct I2S0_CLK_EN_R(crate::FieldReader<bool, bool>);
impl I2S0_CLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2S0_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2S0_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2S0_CLK_EN` writer - I2S0 module."]
pub struct I2S0_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S0_CLK_EN_W<'a> {
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
#[doc = "Field `UART1_CLK_EN` reader - UART1 module."]
pub struct UART1_CLK_EN_R(crate::FieldReader<bool, bool>);
impl UART1_CLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UART1_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART1_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART1_CLK_EN` writer - UART1 module."]
pub struct UART1_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART1_CLK_EN_W<'a> {
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
#[doc = "Field `SPI2_CLK_EN` reader - SPI2 module."]
pub struct SPI2_CLK_EN_R(crate::FieldReader<bool, bool>);
impl SPI2_CLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPI2_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI2_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI2_CLK_EN` writer - SPI2 module."]
pub struct SPI2_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI2_CLK_EN_W<'a> {
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
#[doc = "Field `I2C0_EXT0_CLK_EN` reader - I2C0 module."]
pub struct I2C0_EXT0_CLK_EN_R(crate::FieldReader<bool, bool>);
impl I2C0_EXT0_CLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C0_EXT0_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C0_EXT0_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C0_EXT0_CLK_EN` writer - I2C0 module."]
pub struct I2C0_EXT0_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C0_EXT0_CLK_EN_W<'a> {
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
#[doc = "Field `UHCI0_CLK_EN` reader - UDMA0 module."]
pub struct UHCI0_CLK_EN_R(crate::FieldReader<bool, bool>);
impl UHCI0_CLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UHCI0_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UHCI0_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UHCI0_CLK_EN` writer - UDMA0 module."]
pub struct UHCI0_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI0_CLK_EN_W<'a> {
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
#[doc = "Field `RMT_CLK_EN` reader - RMT module."]
pub struct RMT_CLK_EN_R(crate::FieldReader<bool, bool>);
impl RMT_CLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RMT_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RMT_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RMT_CLK_EN` writer - RMT module."]
pub struct RMT_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CLK_EN_W<'a> {
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
#[doc = "Field `PCNT_CLK_EN` reader - PCNT module."]
pub struct PCNT_CLK_EN_R(crate::FieldReader<bool, bool>);
impl PCNT_CLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PCNT_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCNT_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCNT_CLK_EN` writer - PCNT module."]
pub struct PCNT_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PCNT_CLK_EN_W<'a> {
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
#[doc = "Field `LEDC_CLK_EN` reader - LEDC module."]
pub struct LEDC_CLK_EN_R(crate::FieldReader<bool, bool>);
impl LEDC_CLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LEDC_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LEDC_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LEDC_CLK_EN` writer - LEDC module."]
pub struct LEDC_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 11)) | ((value as u32 & 1) << 11);
        self.w
    }
}
#[doc = "Field `UHCI1_CLK_EN` reader - UDMA1 module."]
pub struct UHCI1_CLK_EN_R(crate::FieldReader<bool, bool>);
impl UHCI1_CLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UHCI1_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UHCI1_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UHCI1_CLK_EN` writer - UDMA1 module."]
pub struct UHCI1_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI1_CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 12)) | ((value as u32 & 1) << 12);
        self.w
    }
}
#[doc = "Field `TIMERGROUP_CLK_EN` reader - TIMG0 module."]
pub struct TIMERGROUP_CLK_EN_R(crate::FieldReader<bool, bool>);
impl TIMERGROUP_CLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMERGROUP_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMERGROUP_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMERGROUP_CLK_EN` writer - TIMG0 module."]
pub struct TIMERGROUP_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMERGROUP_CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 13)) | ((value as u32 & 1) << 13);
        self.w
    }
}
#[doc = "Field `EFUSE_CLK_EN` reader - eFuse module."]
pub struct EFUSE_CLK_EN_R(crate::FieldReader<bool, bool>);
impl EFUSE_CLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EFUSE_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EFUSE_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EFUSE_CLK_EN` writer - eFuse module."]
pub struct EFUSE_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 14)) | ((value as u32 & 1) << 14);
        self.w
    }
}
#[doc = "Field `TIMERGROUP1_CLK_EN` reader - TIMG1 module."]
pub struct TIMERGROUP1_CLK_EN_R(crate::FieldReader<bool, bool>);
impl TIMERGROUP1_CLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMERGROUP1_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMERGROUP1_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMERGROUP1_CLK_EN` writer - TIMG1 module."]
pub struct TIMERGROUP1_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMERGROUP1_CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 15)) | ((value as u32 & 1) << 15);
        self.w
    }
}
#[doc = "Field `SPI3_CLK_EN` reader - SPI3 module."]
pub struct SPI3_CLK_EN_R(crate::FieldReader<bool, bool>);
impl SPI3_CLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPI3_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI3_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI3_CLK_EN` writer - SPI3 module."]
pub struct SPI3_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI3_CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 16)) | ((value as u32 & 1) << 16);
        self.w
    }
}
#[doc = "Field `PWM0_CLK_EN` reader - PWM0 module."]
pub struct PWM0_CLK_EN_R(crate::FieldReader<bool, bool>);
impl PWM0_CLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWM0_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWM0_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWM0_CLK_EN` writer - PWM0 module."]
pub struct PWM0_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM0_CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 17)) | ((value as u32 & 1) << 17);
        self.w
    }
}
#[doc = "Field `I2C_EXT1_CLK_EN` reader - I2C1 module."]
pub struct I2C_EXT1_CLK_EN_R(crate::FieldReader<bool, bool>);
impl I2C_EXT1_CLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C_EXT1_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_EXT1_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C_EXT1_CLK_EN` writer - I2C1 module."]
pub struct I2C_EXT1_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_EXT1_CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 18)) | ((value as u32 & 1) << 18);
        self.w
    }
}
#[doc = "Field `TWAI_CLK_EN` reader - TWAI module."]
pub struct TWAI_CLK_EN_R(crate::FieldReader<bool, bool>);
impl TWAI_CLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TWAI_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TWAI_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TWAI_CLK_EN` writer - TWAI module."]
pub struct TWAI_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TWAI_CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 19)) | ((value as u32 & 1) << 19);
        self.w
    }
}
#[doc = "Field `PWM1_CLK_EN` reader - PWM1 module."]
pub struct PWM1_CLK_EN_R(crate::FieldReader<bool, bool>);
impl PWM1_CLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWM1_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWM1_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWM1_CLK_EN` writer - PWM1 module."]
pub struct PWM1_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM1_CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 20)) | ((value as u32 & 1) << 20);
        self.w
    }
}
#[doc = "Field `I2S1_CLK_EN` reader - I2S1 module."]
pub struct I2S1_CLK_EN_R(crate::FieldReader<bool, bool>);
impl I2S1_CLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2S1_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2S1_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2S1_CLK_EN` writer - I2S1 module."]
pub struct I2S1_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S1_CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 21)) | ((value as u32 & 1) << 21);
        self.w
    }
}
#[doc = "Field `SPI_DMA_CLK_EN` reader - SPI_DMA module."]
pub struct SPI_DMA_CLK_EN_R(crate::FieldReader<bool, bool>);
impl SPI_DMA_CLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPI_DMA_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_DMA_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_DMA_CLK_EN` writer - SPI_DMA module."]
pub struct SPI_DMA_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_DMA_CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 22)) | ((value as u32 & 1) << 22);
        self.w
    }
}
#[doc = "Field `UART2_CLK_EN` reader - UART2 module."]
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
#[doc = "Field `UART2_CLK_EN` writer - UART2 module."]
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
        self.w.bits = (self.w.bits & !(1 << 23)) | ((value as u32 & 1) << 23);
        self.w
    }
}
#[doc = "Field `UART_MEM_CLK_EN` reader - Shared memory of UART0 ~ 2."]
pub struct UART_MEM_CLK_EN_R(crate::FieldReader<bool, bool>);
impl UART_MEM_CLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UART_MEM_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_MEM_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART_MEM_CLK_EN` writer - Shared memory of UART0 ~ 2."]
pub struct UART_MEM_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_MEM_CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 24)) | ((value as u32 & 1) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - SPI0 and SPI1 module."]
    #[inline(always)]
    pub fn spi01_clk_en(&self) -> SPI01_CLK_EN_R {
        SPI01_CLK_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UART0 module."]
    #[inline(always)]
    pub fn uart_clk_en(&self) -> UART_CLK_EN_R {
        UART_CLK_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - I2S0 module."]
    #[inline(always)]
    pub fn i2s0_clk_en(&self) -> I2S0_CLK_EN_R {
        I2S0_CLK_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UART1 module."]
    #[inline(always)]
    pub fn uart1_clk_en(&self) -> UART1_CLK_EN_R {
        UART1_CLK_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SPI2 module."]
    #[inline(always)]
    pub fn spi2_clk_en(&self) -> SPI2_CLK_EN_R {
        SPI2_CLK_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C0 module."]
    #[inline(always)]
    pub fn i2c0_ext0_clk_en(&self) -> I2C0_EXT0_CLK_EN_R {
        I2C0_EXT0_CLK_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - UDMA0 module."]
    #[inline(always)]
    pub fn uhci0_clk_en(&self) -> UHCI0_CLK_EN_R {
        UHCI0_CLK_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RMT module."]
    #[inline(always)]
    pub fn rmt_clk_en(&self) -> RMT_CLK_EN_R {
        RMT_CLK_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PCNT module."]
    #[inline(always)]
    pub fn pcnt_clk_en(&self) -> PCNT_CLK_EN_R {
        PCNT_CLK_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - LEDC module."]
    #[inline(always)]
    pub fn ledc_clk_en(&self) -> LEDC_CLK_EN_R {
        LEDC_CLK_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - UDMA1 module."]
    #[inline(always)]
    pub fn uhci1_clk_en(&self) -> UHCI1_CLK_EN_R {
        UHCI1_CLK_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TIMG0 module."]
    #[inline(always)]
    pub fn timergroup_clk_en(&self) -> TIMERGROUP_CLK_EN_R {
        TIMERGROUP_CLK_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - eFuse module."]
    #[inline(always)]
    pub fn efuse_clk_en(&self) -> EFUSE_CLK_EN_R {
        EFUSE_CLK_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TIMG1 module."]
    #[inline(always)]
    pub fn timergroup1_clk_en(&self) -> TIMERGROUP1_CLK_EN_R {
        TIMERGROUP1_CLK_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SPI3 module."]
    #[inline(always)]
    pub fn spi3_clk_en(&self) -> SPI3_CLK_EN_R {
        SPI3_CLK_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PWM0 module."]
    #[inline(always)]
    pub fn pwm0_clk_en(&self) -> PWM0_CLK_EN_R {
        PWM0_CLK_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - I2C1 module."]
    #[inline(always)]
    pub fn i2c_ext1_clk_en(&self) -> I2C_EXT1_CLK_EN_R {
        I2C_EXT1_CLK_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - TWAI module."]
    #[inline(always)]
    pub fn twai_clk_en(&self) -> TWAI_CLK_EN_R {
        TWAI_CLK_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - PWM1 module."]
    #[inline(always)]
    pub fn pwm1_clk_en(&self) -> PWM1_CLK_EN_R {
        PWM1_CLK_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2S1 module."]
    #[inline(always)]
    pub fn i2s1_clk_en(&self) -> I2S1_CLK_EN_R {
        I2S1_CLK_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SPI_DMA module."]
    #[inline(always)]
    pub fn spi_dma_clk_en(&self) -> SPI_DMA_CLK_EN_R {
        SPI_DMA_CLK_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - UART2 module."]
    #[inline(always)]
    pub fn uart2_clk_en(&self) -> UART2_CLK_EN_R {
        UART2_CLK_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Shared memory of UART0 ~ 2."]
    #[inline(always)]
    pub fn uart_mem_clk_en(&self) -> UART_MEM_CLK_EN_R {
        UART_MEM_CLK_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - SPI0 and SPI1 module."]
    #[inline(always)]
    pub fn spi01_clk_en(&mut self) -> SPI01_CLK_EN_W {
        SPI01_CLK_EN_W { w: self }
    }
    #[doc = "Bit 2 - UART0 module."]
    #[inline(always)]
    pub fn uart_clk_en(&mut self) -> UART_CLK_EN_W {
        UART_CLK_EN_W { w: self }
    }
    #[doc = "Bit 4 - I2S0 module."]
    #[inline(always)]
    pub fn i2s0_clk_en(&mut self) -> I2S0_CLK_EN_W {
        I2S0_CLK_EN_W { w: self }
    }
    #[doc = "Bit 5 - UART1 module."]
    #[inline(always)]
    pub fn uart1_clk_en(&mut self) -> UART1_CLK_EN_W {
        UART1_CLK_EN_W { w: self }
    }
    #[doc = "Bit 6 - SPI2 module."]
    #[inline(always)]
    pub fn spi2_clk_en(&mut self) -> SPI2_CLK_EN_W {
        SPI2_CLK_EN_W { w: self }
    }
    #[doc = "Bit 7 - I2C0 module."]
    #[inline(always)]
    pub fn i2c0_ext0_clk_en(&mut self) -> I2C0_EXT0_CLK_EN_W {
        I2C0_EXT0_CLK_EN_W { w: self }
    }
    #[doc = "Bit 8 - UDMA0 module."]
    #[inline(always)]
    pub fn uhci0_clk_en(&mut self) -> UHCI0_CLK_EN_W {
        UHCI0_CLK_EN_W { w: self }
    }
    #[doc = "Bit 9 - RMT module."]
    #[inline(always)]
    pub fn rmt_clk_en(&mut self) -> RMT_CLK_EN_W {
        RMT_CLK_EN_W { w: self }
    }
    #[doc = "Bit 10 - PCNT module."]
    #[inline(always)]
    pub fn pcnt_clk_en(&mut self) -> PCNT_CLK_EN_W {
        PCNT_CLK_EN_W { w: self }
    }
    #[doc = "Bit 11 - LEDC module."]
    #[inline(always)]
    pub fn ledc_clk_en(&mut self) -> LEDC_CLK_EN_W {
        LEDC_CLK_EN_W { w: self }
    }
    #[doc = "Bit 12 - UDMA1 module."]
    #[inline(always)]
    pub fn uhci1_clk_en(&mut self) -> UHCI1_CLK_EN_W {
        UHCI1_CLK_EN_W { w: self }
    }
    #[doc = "Bit 13 - TIMG0 module."]
    #[inline(always)]
    pub fn timergroup_clk_en(&mut self) -> TIMERGROUP_CLK_EN_W {
        TIMERGROUP_CLK_EN_W { w: self }
    }
    #[doc = "Bit 14 - eFuse module."]
    #[inline(always)]
    pub fn efuse_clk_en(&mut self) -> EFUSE_CLK_EN_W {
        EFUSE_CLK_EN_W { w: self }
    }
    #[doc = "Bit 15 - TIMG1 module."]
    #[inline(always)]
    pub fn timergroup1_clk_en(&mut self) -> TIMERGROUP1_CLK_EN_W {
        TIMERGROUP1_CLK_EN_W { w: self }
    }
    #[doc = "Bit 16 - SPI3 module."]
    #[inline(always)]
    pub fn spi3_clk_en(&mut self) -> SPI3_CLK_EN_W {
        SPI3_CLK_EN_W { w: self }
    }
    #[doc = "Bit 17 - PWM0 module."]
    #[inline(always)]
    pub fn pwm0_clk_en(&mut self) -> PWM0_CLK_EN_W {
        PWM0_CLK_EN_W { w: self }
    }
    #[doc = "Bit 18 - I2C1 module."]
    #[inline(always)]
    pub fn i2c_ext1_clk_en(&mut self) -> I2C_EXT1_CLK_EN_W {
        I2C_EXT1_CLK_EN_W { w: self }
    }
    #[doc = "Bit 19 - TWAI module."]
    #[inline(always)]
    pub fn twai_clk_en(&mut self) -> TWAI_CLK_EN_W {
        TWAI_CLK_EN_W { w: self }
    }
    #[doc = "Bit 20 - PWM1 module."]
    #[inline(always)]
    pub fn pwm1_clk_en(&mut self) -> PWM1_CLK_EN_W {
        PWM1_CLK_EN_W { w: self }
    }
    #[doc = "Bit 21 - I2S1 module."]
    #[inline(always)]
    pub fn i2s1_clk_en(&mut self) -> I2S1_CLK_EN_W {
        I2S1_CLK_EN_W { w: self }
    }
    #[doc = "Bit 22 - SPI_DMA module."]
    #[inline(always)]
    pub fn spi_dma_clk_en(&mut self) -> SPI_DMA_CLK_EN_W {
        SPI_DMA_CLK_EN_W { w: self }
    }
    #[doc = "Bit 23 - UART2 module."]
    #[inline(always)]
    pub fn uart2_clk_en(&mut self) -> UART2_CLK_EN_W {
        UART2_CLK_EN_W { w: self }
    }
    #[doc = "Bit 24 - Shared memory of UART0 ~ 2."]
    #[inline(always)]
    pub fn uart_mem_clk_en(&mut self) -> UART_MEM_CLK_EN_W {
        UART_MEM_CLK_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perip_clk_en]
(index.html) module"]
pub struct PERIP_CLK_EN_SPEC;
impl crate::RegisterSpec for PERIP_CLK_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [perip_clk_en::R]
(R) reader structure"]
impl crate::Readable for PERIP_CLK_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [perip_clk_en::W]
(W) writer structure"]
impl crate::Writable for PERIP_CLK_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PERIP_CLK_EN to value 0"]
impl crate::Resettable for PERIP_CLK_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
