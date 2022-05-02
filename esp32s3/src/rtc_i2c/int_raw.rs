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
#[doc = "Field `SLAVE_TRAN_COMP_INT_RAW` reader - slave transit complete interrupt raw"]
pub struct SLAVE_TRAN_COMP_INT_RAW_R(crate::FieldReader<bool>);
impl SLAVE_TRAN_COMP_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLAVE_TRAN_COMP_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLAVE_TRAN_COMP_INT_RAW_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARBITRATION_LOST_INT_RAW` reader - arbitration lost interrupt raw"]
pub struct ARBITRATION_LOST_INT_RAW_R(crate::FieldReader<bool>);
impl ARBITRATION_LOST_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ARBITRATION_LOST_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARBITRATION_LOST_INT_RAW_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASTER_TRAN_COMP_INT_RAW` reader - master transit complete interrupt raw"]
pub struct MASTER_TRAN_COMP_INT_RAW_R(crate::FieldReader<bool>);
impl MASTER_TRAN_COMP_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MASTER_TRAN_COMP_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASTER_TRAN_COMP_INT_RAW_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRANS_COMPLETE_INT_RAW` reader - transit complete interrupt raw"]
pub struct TRANS_COMPLETE_INT_RAW_R(crate::FieldReader<bool>);
impl TRANS_COMPLETE_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRANS_COMPLETE_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRANS_COMPLETE_INT_RAW_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIME_OUT_INT_RAW` reader - time out interrupt raw"]
pub struct TIME_OUT_INT_RAW_R(crate::FieldReader<bool>);
impl TIME_OUT_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIME_OUT_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIME_OUT_INT_RAW_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACK_ERR_INT_RAW` reader - ack error interrupt raw"]
pub struct ACK_ERR_INT_RAW_R(crate::FieldReader<bool>);
impl ACK_ERR_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ACK_ERR_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACK_ERR_INT_RAW_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_DATA_INT_RAW` reader - receive data interrupt raw"]
pub struct RX_DATA_INT_RAW_R(crate::FieldReader<bool>);
impl RX_DATA_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_DATA_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_DATA_INT_RAW_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_DATA_INT_RAW` reader - transit data interrupt raw"]
pub struct TX_DATA_INT_RAW_R(crate::FieldReader<bool>);
impl TX_DATA_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_DATA_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_DATA_INT_RAW_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DETECT_START_INT_RAW` reader - detect start interrupt raw"]
pub struct DETECT_START_INT_RAW_R(crate::FieldReader<bool>);
impl DETECT_START_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DETECT_START_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DETECT_START_INT_RAW_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - slave transit complete interrupt raw"]
    #[inline(always)]
    pub fn slave_tran_comp_int_raw(&self) -> SLAVE_TRAN_COMP_INT_RAW_R {
        SLAVE_TRAN_COMP_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - arbitration lost interrupt raw"]
    #[inline(always)]
    pub fn arbitration_lost_int_raw(&self) -> ARBITRATION_LOST_INT_RAW_R {
        ARBITRATION_LOST_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - master transit complete interrupt raw"]
    #[inline(always)]
    pub fn master_tran_comp_int_raw(&self) -> MASTER_TRAN_COMP_INT_RAW_R {
        MASTER_TRAN_COMP_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - transit complete interrupt raw"]
    #[inline(always)]
    pub fn trans_complete_int_raw(&self) -> TRANS_COMPLETE_INT_RAW_R {
        TRANS_COMPLETE_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - time out interrupt raw"]
    #[inline(always)]
    pub fn time_out_int_raw(&self) -> TIME_OUT_INT_RAW_R {
        TIME_OUT_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ack error interrupt raw"]
    #[inline(always)]
    pub fn ack_err_int_raw(&self) -> ACK_ERR_INT_RAW_R {
        ACK_ERR_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - receive data interrupt raw"]
    #[inline(always)]
    pub fn rx_data_int_raw(&self) -> RX_DATA_INT_RAW_R {
        RX_DATA_INT_RAW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - transit data interrupt raw"]
    #[inline(always)]
    pub fn tx_data_int_raw(&self) -> TX_DATA_INT_RAW_R {
        TX_DATA_INT_RAW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - detect start interrupt raw"]
    #[inline(always)]
    pub fn detect_start_int_raw(&self) -> DETECT_START_INT_RAW_R {
        DETECT_START_INT_RAW_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "interrupt raw register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_raw](index.html) module"]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_raw::R](R) reader structure"]
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
