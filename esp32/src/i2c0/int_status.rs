#[doc = "Register `INT_STATUS` reader"]
pub struct R(crate::R<INT_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXFIFO_FULL_INT_ST` reader - The masked interrupt status for rxfifo_full_int interrupt."]
pub struct RXFIFO_FULL_INT_ST_R(crate::FieldReader<bool, bool>);
impl RXFIFO_FULL_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXFIFO_FULL_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFIFO_FULL_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFIFO_EMPTY_INT_ST` reader - The masked interrupt status for txfifo_empty_int interrupt."]
pub struct TXFIFO_EMPTY_INT_ST_R(crate::FieldReader<bool, bool>);
impl TXFIFO_EMPTY_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXFIFO_EMPTY_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFIFO_EMPTY_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFIFO_OVF_INT_ST` reader - The masked interrupt status for rxfifo_ovf_int interrupt."]
pub struct RXFIFO_OVF_INT_ST_R(crate::FieldReader<bool, bool>);
impl RXFIFO_OVF_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXFIFO_OVF_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFIFO_OVF_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `END_DETECT_INT_ST` reader - The masked interrupt status for end_detect_int interrupt."]
pub struct END_DETECT_INT_ST_R(crate::FieldReader<bool, bool>);
impl END_DETECT_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        END_DETECT_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for END_DETECT_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLAVE_TRAN_COMP_INT_ST` reader - The masked interrupt status for slave_tran_comp_int interrupt."]
pub struct SLAVE_TRAN_COMP_INT_ST_R(crate::FieldReader<bool, bool>);
impl SLAVE_TRAN_COMP_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLAVE_TRAN_COMP_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLAVE_TRAN_COMP_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARBITRATION_LOST_INT_ST` reader - The masked interrupt status for arbitration_lost_int interrupt."]
pub struct ARBITRATION_LOST_INT_ST_R(crate::FieldReader<bool, bool>);
impl ARBITRATION_LOST_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ARBITRATION_LOST_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARBITRATION_LOST_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASTER_TRAN_COMP_INT_ST` reader - The masked interrupt status for master_tran_comp_int interrupt."]
pub struct MASTER_TRAN_COMP_INT_ST_R(crate::FieldReader<bool, bool>);
impl MASTER_TRAN_COMP_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MASTER_TRAN_COMP_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASTER_TRAN_COMP_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRANS_COMPLETE_INT_ST` reader - The masked interrupt status for trans_complete_int interrupt."]
pub struct TRANS_COMPLETE_INT_ST_R(crate::FieldReader<bool, bool>);
impl TRANS_COMPLETE_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRANS_COMPLETE_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRANS_COMPLETE_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIME_OUT_INT_ST` reader - The masked interrupt status for time_out_int interrupt."]
pub struct TIME_OUT_INT_ST_R(crate::FieldReader<bool, bool>);
impl TIME_OUT_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIME_OUT_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIME_OUT_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRANS_START_INT_ST` reader - The masked interrupt status for trans_start_int interrupt."]
pub struct TRANS_START_INT_ST_R(crate::FieldReader<bool, bool>);
impl TRANS_START_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRANS_START_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRANS_START_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACK_ERR_INT_ST` reader - The masked interrupt status for ack_err_int interrupt."]
pub struct ACK_ERR_INT_ST_R(crate::FieldReader<bool, bool>);
impl ACK_ERR_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ACK_ERR_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACK_ERR_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_REC_FULL_INT_ST` reader - The masked interrupt status for rx_rec_full_int interrupt."]
pub struct RX_REC_FULL_INT_ST_R(crate::FieldReader<bool, bool>);
impl RX_REC_FULL_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_REC_FULL_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_REC_FULL_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_SEND_EMPTY_INT_ST` reader - The masked interrupt status for tx_send_empty_int interrupt."]
pub struct TX_SEND_EMPTY_INT_ST_R(crate::FieldReader<bool, bool>);
impl TX_SEND_EMPTY_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_SEND_EMPTY_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_SEND_EMPTY_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - The masked interrupt status for rxfifo_full_int interrupt."]
    #[inline(always)]
    pub fn rxfifo_full_int_st(&self) -> RXFIFO_FULL_INT_ST_R {
        RXFIFO_FULL_INT_ST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - The masked interrupt status for txfifo_empty_int interrupt."]
    #[inline(always)]
    pub fn txfifo_empty_int_st(&self) -> TXFIFO_EMPTY_INT_ST_R {
        TXFIFO_EMPTY_INT_ST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - The masked interrupt status for rxfifo_ovf_int interrupt."]
    #[inline(always)]
    pub fn rxfifo_ovf_int_st(&self) -> RXFIFO_OVF_INT_ST_R {
        RXFIFO_OVF_INT_ST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - The masked interrupt status for end_detect_int interrupt."]
    #[inline(always)]
    pub fn end_detect_int_st(&self) -> END_DETECT_INT_ST_R {
        END_DETECT_INT_ST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - The masked interrupt status for slave_tran_comp_int interrupt."]
    #[inline(always)]
    pub fn slave_tran_comp_int_st(&self) -> SLAVE_TRAN_COMP_INT_ST_R {
        SLAVE_TRAN_COMP_INT_ST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - The masked interrupt status for arbitration_lost_int interrupt."]
    #[inline(always)]
    pub fn arbitration_lost_int_st(&self) -> ARBITRATION_LOST_INT_ST_R {
        ARBITRATION_LOST_INT_ST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - The masked interrupt status for master_tran_comp_int interrupt."]
    #[inline(always)]
    pub fn master_tran_comp_int_st(&self) -> MASTER_TRAN_COMP_INT_ST_R {
        MASTER_TRAN_COMP_INT_ST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - The masked interrupt status for trans_complete_int interrupt."]
    #[inline(always)]
    pub fn trans_complete_int_st(&self) -> TRANS_COMPLETE_INT_ST_R {
        TRANS_COMPLETE_INT_ST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - The masked interrupt status for time_out_int interrupt."]
    #[inline(always)]
    pub fn time_out_int_st(&self) -> TIME_OUT_INT_ST_R {
        TIME_OUT_INT_ST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - The masked interrupt status for trans_start_int interrupt."]
    #[inline(always)]
    pub fn trans_start_int_st(&self) -> TRANS_START_INT_ST_R {
        TRANS_START_INT_ST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - The masked interrupt status for ack_err_int interrupt."]
    #[inline(always)]
    pub fn ack_err_int_st(&self) -> ACK_ERR_INT_ST_R {
        ACK_ERR_INT_ST_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - The masked interrupt status for rx_rec_full_int interrupt."]
    #[inline(always)]
    pub fn rx_rec_full_int_st(&self) -> RX_REC_FULL_INT_ST_R {
        RX_REC_FULL_INT_ST_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - The masked interrupt status for tx_send_empty_int interrupt."]
    #[inline(always)]
    pub fn tx_send_empty_int_st(&self) -> TX_SEND_EMPTY_INT_ST_R {
        TX_SEND_EMPTY_INT_ST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
#[doc = "\n\nThis register you can [`read`]
(crate::generic::Reg::read). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_status]
(index.html) module"]
pub struct INT_STATUS_SPEC;
impl crate::RegisterSpec for INT_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_status::R]
(R) reader structure"]
impl crate::Readable for INT_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INT_STATUS to value 0"]
impl crate::Resettable for INT_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
