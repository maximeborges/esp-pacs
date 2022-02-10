#[doc = "Register `RD_RS_ERR0` reader"]
pub struct R(crate::R<RD_RS_ERR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_RS_ERR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_RS_ERR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_RS_ERR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MAC_SPI_8M_ERR_NUM` reader - The value of this signal means the number of error bytes."]
pub struct MAC_SPI_8M_ERR_NUM_R(crate::FieldReader<u8, u8>);
impl MAC_SPI_8M_ERR_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MAC_SPI_8M_ERR_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAC_SPI_8M_ERR_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAC_SPI_8M_FAIL` reader - 0: Means no failure and that the data of MAC_SPI_8M is reliable 1: Means that programming user data failed and the number of error bytes is over 6."]
pub struct MAC_SPI_8M_FAIL_R(crate::FieldReader<bool, bool>);
impl MAC_SPI_8M_FAIL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MAC_SPI_8M_FAIL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAC_SPI_8M_FAIL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYS_PART1_NUM` reader - The value of this signal means the number of error bytes."]
pub struct SYS_PART1_NUM_R(crate::FieldReader<u8, u8>);
impl SYS_PART1_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SYS_PART1_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYS_PART1_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYS_PART1_FAIL` reader - 0: Means no failure and that the data of system part1 is reliable 1: Means that programming user data failed and the number of error bytes is over 6."]
pub struct SYS_PART1_FAIL_R(crate::FieldReader<bool, bool>);
impl SYS_PART1_FAIL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SYS_PART1_FAIL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYS_PART1_FAIL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USR_DATA_ERR_NUM` reader - The value of this signal means the number of error bytes."]
pub struct USR_DATA_ERR_NUM_R(crate::FieldReader<u8, u8>);
impl USR_DATA_ERR_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        USR_DATA_ERR_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USR_DATA_ERR_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USR_DATA_FAIL` reader - 0: Means no failure and that the user data is reliable 1: Means that programming user data failed and the number of error bytes is over 6."]
pub struct USR_DATA_FAIL_R(crate::FieldReader<bool, bool>);
impl USR_DATA_FAIL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USR_DATA_FAIL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USR_DATA_FAIL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY0_ERR_NUM` reader - The value of this signal means the number of error bytes."]
pub struct KEY0_ERR_NUM_R(crate::FieldReader<u8, u8>);
impl KEY0_ERR_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        KEY0_ERR_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY0_ERR_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY0_FAIL` reader - 0: Means no failure and that the data of key0 is reliable 1: Means that programming key0 failed and the number of error bytes is over 6."]
pub struct KEY0_FAIL_R(crate::FieldReader<bool, bool>);
impl KEY0_FAIL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        KEY0_FAIL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY0_FAIL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY1_ERR_NUM` reader - The value of this signal means the number of error bytes."]
pub struct KEY1_ERR_NUM_R(crate::FieldReader<u8, u8>);
impl KEY1_ERR_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        KEY1_ERR_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY1_ERR_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY1_FAIL` reader - 0: Means no failure and that the data of key1 is reliable 1: Means that programming key1 failed and the number of error bytes is over 6."]
pub struct KEY1_FAIL_R(crate::FieldReader<bool, bool>);
impl KEY1_FAIL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        KEY1_FAIL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY1_FAIL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY2_ERR_NUM` reader - The value of this signal means the number of error bytes."]
pub struct KEY2_ERR_NUM_R(crate::FieldReader<u8, u8>);
impl KEY2_ERR_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        KEY2_ERR_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY2_ERR_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY2_FAIL` reader - 0: Means no failure and that the data of key2 is reliable 1: Means that programming key2 failed and the number of error bytes is over 6."]
pub struct KEY2_FAIL_R(crate::FieldReader<bool, bool>);
impl KEY2_FAIL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        KEY2_FAIL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY2_FAIL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY3_ERR_NUM` reader - The value of this signal means the number of error bytes."]
pub struct KEY3_ERR_NUM_R(crate::FieldReader<u8, u8>);
impl KEY3_ERR_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        KEY3_ERR_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY3_ERR_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY3_FAIL` reader - 0: Means no failure and that the data of key3 is reliable 1: Means that programming key3 failed and the number of error bytes is over 6."]
pub struct KEY3_FAIL_R(crate::FieldReader<bool, bool>);
impl KEY3_FAIL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        KEY3_FAIL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY3_FAIL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY4_ERR_NUM` reader - The value of this signal means the number of error bytes."]
pub struct KEY4_ERR_NUM_R(crate::FieldReader<u8, u8>);
impl KEY4_ERR_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        KEY4_ERR_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY4_ERR_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY4_FAIL` reader - 0: Means no failure and that the data of key4 is reliable 1: Means that programming key4 failed and the number of error bytes is over 6."]
pub struct KEY4_FAIL_R(crate::FieldReader<bool, bool>);
impl KEY4_FAIL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        KEY4_FAIL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY4_FAIL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:2 - The value of this signal means the number of error bytes."]
    #[inline(always)]
    pub fn mac_spi_8m_err_num(&self) -> MAC_SPI_8M_ERR_NUM_R {
        MAC_SPI_8M_ERR_NUM_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - 0: Means no failure and that the data of MAC_SPI_8M is reliable 1: Means that programming user data failed and the number of error bytes is over 6."]
    #[inline(always)]
    pub fn mac_spi_8m_fail(&self) -> MAC_SPI_8M_FAIL_R {
        MAC_SPI_8M_FAIL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - The value of this signal means the number of error bytes."]
    #[inline(always)]
    pub fn sys_part1_num(&self) -> SYS_PART1_NUM_R {
        SYS_PART1_NUM_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - 0: Means no failure and that the data of system part1 is reliable 1: Means that programming user data failed and the number of error bytes is over 6."]
    #[inline(always)]
    pub fn sys_part1_fail(&self) -> SYS_PART1_FAIL_R {
        SYS_PART1_FAIL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - The value of this signal means the number of error bytes."]
    #[inline(always)]
    pub fn usr_data_err_num(&self) -> USR_DATA_ERR_NUM_R {
        USR_DATA_ERR_NUM_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 11 - 0: Means no failure and that the user data is reliable 1: Means that programming user data failed and the number of error bytes is over 6."]
    #[inline(always)]
    pub fn usr_data_fail(&self) -> USR_DATA_FAIL_R {
        USR_DATA_FAIL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:14 - The value of this signal means the number of error bytes."]
    #[inline(always)]
    pub fn key0_err_num(&self) -> KEY0_ERR_NUM_R {
        KEY0_ERR_NUM_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 15 - 0: Means no failure and that the data of key0 is reliable 1: Means that programming key0 failed and the number of error bytes is over 6."]
    #[inline(always)]
    pub fn key0_fail(&self) -> KEY0_FAIL_R {
        KEY0_FAIL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - The value of this signal means the number of error bytes."]
    #[inline(always)]
    pub fn key1_err_num(&self) -> KEY1_ERR_NUM_R {
        KEY1_ERR_NUM_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 19 - 0: Means no failure and that the data of key1 is reliable 1: Means that programming key1 failed and the number of error bytes is over 6."]
    #[inline(always)]
    pub fn key1_fail(&self) -> KEY1_FAIL_R {
        KEY1_FAIL_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 20:22 - The value of this signal means the number of error bytes."]
    #[inline(always)]
    pub fn key2_err_num(&self) -> KEY2_ERR_NUM_R {
        KEY2_ERR_NUM_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bit 23 - 0: Means no failure and that the data of key2 is reliable 1: Means that programming key2 failed and the number of error bytes is over 6."]
    #[inline(always)]
    pub fn key2_fail(&self) -> KEY2_FAIL_R {
        KEY2_FAIL_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:26 - The value of this signal means the number of error bytes."]
    #[inline(always)]
    pub fn key3_err_num(&self) -> KEY3_ERR_NUM_R {
        KEY3_ERR_NUM_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bit 27 - 0: Means no failure and that the data of key3 is reliable 1: Means that programming key3 failed and the number of error bytes is over 6."]
    #[inline(always)]
    pub fn key3_fail(&self) -> KEY3_FAIL_R {
        KEY3_FAIL_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 28:30 - The value of this signal means the number of error bytes."]
    #[inline(always)]
    pub fn key4_err_num(&self) -> KEY4_ERR_NUM_R {
        KEY4_ERR_NUM_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bit 31 - 0: Means no failure and that the data of key4 is reliable 1: Means that programming key4 failed and the number of error bytes is over 6."]
    #[inline(always)]
    pub fn key4_fail(&self) -> KEY4_FAIL_R {
        KEY4_FAIL_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "Programming error record register 0 of BLOCK1-10.\n\nThis register you can [`read`]
(crate::generic::Reg::read). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_rs_err0]
(index.html) module"]
pub struct RD_RS_ERR0_SPEC;
impl crate::RegisterSpec for RD_RS_ERR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_rs_err0::R]
(R) reader structure"]
impl crate::Readable for RD_RS_ERR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_RS_ERR0 to value 0"]
impl crate::Resettable for RD_RS_ERR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
