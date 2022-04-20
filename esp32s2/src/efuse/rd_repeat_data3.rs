#[doc = "Register `RD_REPEAT_DATA3` reader"]
pub struct R(crate::R<RD_REPEAT_DATA3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_REPEAT_DATA3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_REPEAT_DATA3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_REPEAT_DATA3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DIS_DOWNLOAD_MODE` reader - Set this bit to disable all download boot modes."]
pub struct DIS_DOWNLOAD_MODE_R(crate::FieldReader<bool, bool>);
impl DIS_DOWNLOAD_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIS_DOWNLOAD_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIS_DOWNLOAD_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIS_LEGACY_SPI_BOOT` reader - Set this bit to disable Legacy SPI boot mode."]
pub struct DIS_LEGACY_SPI_BOOT_R(crate::FieldReader<bool, bool>);
impl DIS_LEGACY_SPI_BOOT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIS_LEGACY_SPI_BOOT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIS_LEGACY_SPI_BOOT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART_PRINT_CHANNEL` reader - Selects the default UART for printing boot messages. 0: UART0. 1: UART1."]
pub struct UART_PRINT_CHANNEL_R(crate::FieldReader<bool, bool>);
impl UART_PRINT_CHANNEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UART_PRINT_CHANNEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_PRINT_CHANNEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPT4_RESERVED3` reader - Reserved (used for four backups method)."]
pub struct RPT4_RESERVED3_R(crate::FieldReader<bool, bool>);
impl RPT4_RESERVED3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RPT4_RESERVED3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPT4_RESERVED3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIS_USB_DOWNLOAD_MODE` reader - Set this bit to disable use of USB OTG in UART download boot mode."]
pub struct DIS_USB_DOWNLOAD_MODE_R(crate::FieldReader<bool, bool>);
impl DIS_USB_DOWNLOAD_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIS_USB_DOWNLOAD_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIS_USB_DOWNLOAD_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE_SECURITY_DOWNLOAD` reader - Set this bit to enable secure UART download mode (read/write flash only)."]
pub struct ENABLE_SECURITY_DOWNLOAD_R(crate::FieldReader<bool, bool>);
impl ENABLE_SECURITY_DOWNLOAD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_SECURITY_DOWNLOAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLE_SECURITY_DOWNLOAD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART_PRINT_CONTROL` reader - Set the default UART boot message output mode. 00: Enabled. 01: Enable when GPIO46 is low at reset. 10: Enable when GPIO46 is high at reset. 11: Disabled."]
pub struct UART_PRINT_CONTROL_R(crate::FieldReader<u8, u8>);
impl UART_PRINT_CONTROL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        UART_PRINT_CONTROL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_PRINT_CONTROL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN_POWER_SELECTION` reader - Set default power supply for GPIO33-GPIO37, set when SPI flash is initialized. 0: VDD3P3_CPU. 1: VDD_SPI."]
pub struct PIN_POWER_SELECTION_R(crate::FieldReader<bool, bool>);
impl PIN_POWER_SELECTION_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIN_POWER_SELECTION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIN_POWER_SELECTION_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH_TYPE` reader - SPI flash type. 0: maximum four data lines, 1: eight data lines."]
pub struct FLASH_TYPE_R(crate::FieldReader<bool, bool>);
impl FLASH_TYPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FLASH_TYPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH_TYPE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FORCE_SEND_RESUME` reader - If set, forces ROM code to send an SPI flash resume command during SPI boot."]
pub struct FORCE_SEND_RESUME_R(crate::FieldReader<bool, bool>);
impl FORCE_SEND_RESUME_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FORCE_SEND_RESUME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FORCE_SEND_RESUME_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SECURE_VERSION` reader - Secure version (used by ESP-IDF anti-rollback feature)."]
pub struct SECURE_VERSION_R(crate::FieldReader<u16, u16>);
impl SECURE_VERSION_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        SECURE_VERSION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SECURE_VERSION_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPT4_RESERVED2` reader - Reserved (used for four backups method)."]
pub struct RPT4_RESERVED2_R(crate::FieldReader<u8, u8>);
impl RPT4_RESERVED2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RPT4_RESERVED2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPT4_RESERVED2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Set this bit to disable all download boot modes."]
    #[inline(always)]
    pub fn dis_download_mode(&self) -> DIS_DOWNLOAD_MODE_R {
        DIS_DOWNLOAD_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to disable Legacy SPI boot mode."]
    #[inline(always)]
    pub fn dis_legacy_spi_boot(&self) -> DIS_LEGACY_SPI_BOOT_R {
        DIS_LEGACY_SPI_BOOT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Selects the default UART for printing boot messages. 0: UART0. 1: UART1."]
    #[inline(always)]
    pub fn uart_print_channel(&self) -> UART_PRINT_CHANNEL_R {
        UART_PRINT_CHANNEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved (used for four backups method)."]
    #[inline(always)]
    pub fn rpt4_reserved3(&self) -> RPT4_RESERVED3_R {
        RPT4_RESERVED3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit to disable use of USB OTG in UART download boot mode."]
    #[inline(always)]
    pub fn dis_usb_download_mode(&self) -> DIS_USB_DOWNLOAD_MODE_R {
        DIS_USB_DOWNLOAD_MODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set this bit to enable secure UART download mode (read/write flash only)."]
    #[inline(always)]
    pub fn enable_security_download(&self) -> ENABLE_SECURITY_DOWNLOAD_R {
        ENABLE_SECURITY_DOWNLOAD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Set the default UART boot message output mode. 00: Enabled. 01: Enable when GPIO46 is low at reset. 10: Enable when GPIO46 is high at reset. 11: Disabled."]
    #[inline(always)]
    pub fn uart_print_control(&self) -> UART_PRINT_CONTROL_R {
        UART_PRINT_CONTROL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Set default power supply for GPIO33-GPIO37, set when SPI flash is initialized. 0: VDD3P3_CPU. 1: VDD_SPI."]
    #[inline(always)]
    pub fn pin_power_selection(&self) -> PIN_POWER_SELECTION_R {
        PIN_POWER_SELECTION_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SPI flash type. 0: maximum four data lines, 1: eight data lines."]
    #[inline(always)]
    pub fn flash_type(&self) -> FLASH_TYPE_R {
        FLASH_TYPE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - If set, forces ROM code to send an SPI flash resume command during SPI boot."]
    #[inline(always)]
    pub fn force_send_resume(&self) -> FORCE_SEND_RESUME_R {
        FORCE_SEND_RESUME_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:26 - Secure version (used by ESP-IDF anti-rollback feature)."]
    #[inline(always)]
    pub fn secure_version(&self) -> SECURE_VERSION_R {
        SECURE_VERSION_R::new(((self.bits >> 11) & 0xffff) as u16)
    }
    #[doc = "Bits 27:31 - Reserved (used for four backups method)."]
    #[inline(always)]
    pub fn rpt4_reserved2(&self) -> RPT4_RESERVED2_R {
        RPT4_RESERVED2_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
#[doc = "Register 4 of BLOCK0.\n\nThis register you can [`read`]
(crate::generic::Reg::read). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_repeat_data3]
(index.html) module"]
pub struct RD_REPEAT_DATA3_SPEC;
impl crate::RegisterSpec for RD_REPEAT_DATA3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_repeat_data3::R]
(R) reader structure"]
impl crate::Readable for RD_REPEAT_DATA3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_REPEAT_DATA3 to value 0"]
impl crate::Resettable for RD_REPEAT_DATA3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
