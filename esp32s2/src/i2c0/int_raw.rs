#[doc = "Register `INT_RAW` reader"]
pub struct R(crate::R<INT_RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXFIFO_WM_INT_RAW` reader - The raw interrupt bit for I2C_RXFIFO_WM_INT interrupt."]
pub struct RXFIFO_WM_INT_RAW_R(crate::FieldReader<bool, bool>);
impl RXFIFO_WM_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXFIFO_WM_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFIFO_WM_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFIFO_WM_INT_RAW` reader - The raw interrupt bit for I2C_TXFIFO_WM_INT interrupt."]
pub struct TXFIFO_WM_INT_RAW_R(crate::FieldReader<bool, bool>);
impl TXFIFO_WM_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXFIFO_WM_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFIFO_WM_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFIFO_OVF_INT_RAW` reader - The raw interrupt bit for I2C_RXFIFO_OVF_INT interrupt."]
pub struct RXFIFO_OVF_INT_RAW_R(crate::FieldReader<bool, bool>);
impl RXFIFO_OVF_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXFIFO_OVF_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFIFO_OVF_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `END_DETECT_INT_RAW` reader - The raw interrupt bit for the I2C_END_DETECT_INT interrupt."]
pub struct END_DETECT_INT_RAW_R(crate::FieldReader<bool, bool>);
impl END_DETECT_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        END_DETECT_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for END_DETECT_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BYTE_TRANS_DONE_INT_RAW` reader - The raw interrupt bit for the I2C_END_DETECT_INT interrupt."]
pub struct BYTE_TRANS_DONE_INT_RAW_R(crate::FieldReader<bool, bool>);
impl BYTE_TRANS_DONE_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BYTE_TRANS_DONE_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTE_TRANS_DONE_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARBITRATION_LOST_INT_RAW` reader - The raw interrupt bit for the I2C_ARBITRATION_LOST_INT interrupt."]
pub struct ARBITRATION_LOST_INT_RAW_R(crate::FieldReader<bool, bool>);
impl ARBITRATION_LOST_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ARBITRATION_LOST_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARBITRATION_LOST_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MST_TXFIFO_UDF_INT_RAW` reader - The raw interrupt bit for I2C_TRANS_COMPLETE_INT interrupt."]
pub struct MST_TXFIFO_UDF_INT_RAW_R(crate::FieldReader<bool, bool>);
impl MST_TXFIFO_UDF_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MST_TXFIFO_UDF_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MST_TXFIFO_UDF_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRANS_COMPLETE_INT_RAW` reader - The raw interrupt bit for the I2C_TRANS_COMPLETE_INT interrupt."]
pub struct TRANS_COMPLETE_INT_RAW_R(crate::FieldReader<bool, bool>);
impl TRANS_COMPLETE_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRANS_COMPLETE_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRANS_COMPLETE_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIME_OUT_INT_RAW` reader - The raw interrupt bit for the I2C_TIME_OUT_INT interrupt."]
pub struct TIME_OUT_INT_RAW_R(crate::FieldReader<bool, bool>);
impl TIME_OUT_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIME_OUT_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIME_OUT_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRANS_START_INT_RAW` reader - The raw interrupt bit for the I2C_TRANS_START_INT interrupt."]
pub struct TRANS_START_INT_RAW_R(crate::FieldReader<bool, bool>);
impl TRANS_START_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRANS_START_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRANS_START_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NACK_INT_RAW` reader - The raw interrupt bit for I2C_SLAVE_STRETCH_INT interrupt."]
pub struct NACK_INT_RAW_R(crate::FieldReader<bool, bool>);
impl NACK_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NACK_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NACK_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFIFO_OVF_INT_RAW` reader - The raw interrupt bit for I2C_TXFIFO_OVF_INT interrupt."]
pub struct TXFIFO_OVF_INT_RAW_R(crate::FieldReader<bool, bool>);
impl TXFIFO_OVF_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXFIFO_OVF_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFIFO_OVF_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFIFO_UDF_INT_RAW` reader - The raw interrupt bit for I2C_RXFIFO_UDF_INT interrupt."]
pub struct RXFIFO_UDF_INT_RAW_R(crate::FieldReader<bool, bool>);
impl RXFIFO_UDF_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXFIFO_UDF_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFIFO_UDF_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCL_ST_TO_INT_RAW` reader - The raw interrupt bit for I2C_SCL_ST_TO_INT interrupt."]
pub struct SCL_ST_TO_INT_RAW_R(crate::FieldReader<bool, bool>);
impl SCL_ST_TO_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCL_ST_TO_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCL_ST_TO_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCL_MAIN_ST_TO_INT_RAW` reader - The raw interrupt bit for I2C_SCL_MAIN_ST_TO_INT interrupt."]
pub struct SCL_MAIN_ST_TO_INT_RAW_R(crate::FieldReader<bool, bool>);
impl SCL_MAIN_ST_TO_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCL_MAIN_ST_TO_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCL_MAIN_ST_TO_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DET_START_INT_RAW` reader - The raw interrupt bit for I2C_DET_START_INT interrupt."]
pub struct DET_START_INT_RAW_R(crate::FieldReader<bool, bool>);
impl DET_START_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DET_START_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DET_START_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLAVE_STRETCH_INT_RAW` reader - The raw interrupt bit for I2C_SLAVE_STRETCH_INT interrupt."]
pub struct SLAVE_STRETCH_INT_RAW_R(crate::FieldReader<bool, bool>);
impl SLAVE_STRETCH_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLAVE_STRETCH_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLAVE_STRETCH_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - The raw interrupt bit for I2C_RXFIFO_WM_INT interrupt."]
    #[inline(always)]
    pub fn rxfifo_wm_int_raw(&self) -> RXFIFO_WM_INT_RAW_R {
        RXFIFO_WM_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt bit for I2C_TXFIFO_WM_INT interrupt."]
    #[inline(always)]
    pub fn txfifo_wm_int_raw(&self) -> TXFIFO_WM_INT_RAW_R {
        TXFIFO_WM_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt bit for I2C_RXFIFO_OVF_INT interrupt."]
    #[inline(always)]
    pub fn rxfifo_ovf_int_raw(&self) -> RXFIFO_OVF_INT_RAW_R {
        RXFIFO_OVF_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw interrupt bit for the I2C_END_DETECT_INT interrupt."]
    #[inline(always)]
    pub fn end_detect_int_raw(&self) -> END_DETECT_INT_RAW_R {
        END_DETECT_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The raw interrupt bit for the I2C_END_DETECT_INT interrupt."]
    #[inline(always)]
    pub fn byte_trans_done_int_raw(&self) -> BYTE_TRANS_DONE_INT_RAW_R {
        BYTE_TRANS_DONE_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The raw interrupt bit for the I2C_ARBITRATION_LOST_INT interrupt."]
    #[inline(always)]
    pub fn arbitration_lost_int_raw(&self) -> ARBITRATION_LOST_INT_RAW_R {
        ARBITRATION_LOST_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The raw interrupt bit for I2C_TRANS_COMPLETE_INT interrupt."]
    #[inline(always)]
    pub fn mst_txfifo_udf_int_raw(&self) -> MST_TXFIFO_UDF_INT_RAW_R {
        MST_TXFIFO_UDF_INT_RAW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The raw interrupt bit for the I2C_TRANS_COMPLETE_INT interrupt."]
    #[inline(always)]
    pub fn trans_complete_int_raw(&self) -> TRANS_COMPLETE_INT_RAW_R {
        TRANS_COMPLETE_INT_RAW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The raw interrupt bit for the I2C_TIME_OUT_INT interrupt."]
    #[inline(always)]
    pub fn time_out_int_raw(&self) -> TIME_OUT_INT_RAW_R {
        TIME_OUT_INT_RAW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The raw interrupt bit for the I2C_TRANS_START_INT interrupt."]
    #[inline(always)]
    pub fn trans_start_int_raw(&self) -> TRANS_START_INT_RAW_R {
        TRANS_START_INT_RAW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The raw interrupt bit for I2C_SLAVE_STRETCH_INT interrupt."]
    #[inline(always)]
    pub fn nack_int_raw(&self) -> NACK_INT_RAW_R {
        NACK_INT_RAW_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The raw interrupt bit for I2C_TXFIFO_OVF_INT interrupt."]
    #[inline(always)]
    pub fn txfifo_ovf_int_raw(&self) -> TXFIFO_OVF_INT_RAW_R {
        TXFIFO_OVF_INT_RAW_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The raw interrupt bit for I2C_RXFIFO_UDF_INT interrupt."]
    #[inline(always)]
    pub fn rxfifo_udf_int_raw(&self) -> RXFIFO_UDF_INT_RAW_R {
        RXFIFO_UDF_INT_RAW_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The raw interrupt bit for I2C_SCL_ST_TO_INT interrupt."]
    #[inline(always)]
    pub fn scl_st_to_int_raw(&self) -> SCL_ST_TO_INT_RAW_R {
        SCL_ST_TO_INT_RAW_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The raw interrupt bit for I2C_SCL_MAIN_ST_TO_INT interrupt."]
    #[inline(always)]
    pub fn scl_main_st_to_int_raw(&self) -> SCL_MAIN_ST_TO_INT_RAW_R {
        SCL_MAIN_ST_TO_INT_RAW_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The raw interrupt bit for I2C_DET_START_INT interrupt."]
    #[inline(always)]
    pub fn det_start_int_raw(&self) -> DET_START_INT_RAW_R {
        DET_START_INT_RAW_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - The raw interrupt bit for I2C_SLAVE_STRETCH_INT interrupt."]
    #[inline(always)]
    pub fn slave_stretch_int_raw(&self) -> SLAVE_STRETCH_INT_RAW_R {
        SLAVE_STRETCH_INT_RAW_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "Raw interrupt status\n\nThis register you can [`read`]
(crate::generic::Reg::read). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_raw]
(index.html) module"]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_raw::R]
(R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
