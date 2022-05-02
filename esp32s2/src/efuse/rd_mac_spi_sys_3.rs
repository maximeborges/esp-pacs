#[doc = "Register `RD_MAC_SPI_SYS_3` reader"]
pub struct R(crate::R<RD_MAC_SPI_SYS_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_MAC_SPI_SYS_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_MAC_SPI_SYS_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_MAC_SPI_SYS_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SPI_PAD_CONF_2` reader - Stores the second part of SPI_PAD_CONF."]
pub struct SPI_PAD_CONF_2_R(crate::FieldReader<u32>);
impl SPI_PAD_CONF_2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        SPI_PAD_CONF_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_PAD_CONF_2_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYS_DATA_PART0_0` reader - Stores the zeroth part of the zeroth part of system data."]
pub struct SYS_DATA_PART0_0_R(crate::FieldReader<u16>);
impl SYS_DATA_PART0_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        SYS_DATA_PART0_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYS_DATA_PART0_0_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:17 - Stores the second part of SPI_PAD_CONF."]
    #[inline(always)]
    pub fn spi_pad_conf_2(&self) -> SPI_PAD_CONF_2_R {
        SPI_PAD_CONF_2_R::new((self.bits & 0x0003_ffff) as u32)
    }
    #[doc = "Bits 18:31 - Stores the zeroth part of the zeroth part of system data."]
    #[inline(always)]
    pub fn sys_data_part0_0(&self) -> SYS_DATA_PART0_0_R {
        SYS_DATA_PART0_0_R::new(((self.bits >> 18) & 0x3fff) as u16)
    }
}
#[doc = "Register 3 of BLOCK1.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_mac_spi_sys_3](index.html) module"]
pub struct RD_MAC_SPI_SYS_3_SPEC;
impl crate::RegisterSpec for RD_MAC_SPI_SYS_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_mac_spi_sys_3::R](R) reader structure"]
impl crate::Readable for RD_MAC_SPI_SYS_3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_MAC_SPI_SYS_3 to value 0"]
impl crate::Resettable for RD_MAC_SPI_SYS_3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
