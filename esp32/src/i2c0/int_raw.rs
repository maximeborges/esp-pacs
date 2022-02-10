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
#[doc = "Field `RXFIFO_FULL_INT_RAW` reader - The raw interrupt status bit for rxfifo full when use apb fifo access."]
pub struct RXFIFO_FULL_INT_RAW_R(crate::FieldReader<bool, bool>);
impl RXFIFO_FULL_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXFIFO_FULL_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFIFO_FULL_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFIFO_EMPTY_INT_RAW` reader - The raw interrupt status bit for txfifo empty when use apb fifo access."]
pub struct TXFIFO_EMPTY_INT_RAW_R(crate::FieldReader<bool, bool>);
impl TXFIFO_EMPTY_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXFIFO_EMPTY_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFIFO_EMPTY_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFIFO_OVF_INT_RAW` reader - The raw interrupt status bit for receiving data overflow when use apb fifo access."]
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
#[doc = "Field `END_DETECT_INT_RAW` reader - The raw interrupt status bit for end_detect_int interrupt. when I2C deals with the END command it will produce end_detect_int interrupt."]
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
#[doc = "Field `SLAVE_TRAN_COMP_INT_RAW` reader - The raw interrupt status bit for slave_tran_comp_int interrupt. when I2C Slave detectsthe STOP bit it will produce slave_tran_comp_int interrupt."]
pub struct SLAVE_TRAN_COMP_INT_RAW_R(crate::FieldReader<bool, bool>);
impl SLAVE_TRAN_COMP_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLAVE_TRAN_COMP_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLAVE_TRAN_COMP_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARBITRATION_LOST_INT_RAW` reader - The raw interrupt status bit for arbitration_lost_int interrupt.when I2C lost the usage right of I2C BUS it will produce arbitration_lost_int interrupt."]
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
#[doc = "Field `MASTER_TRAN_COMP_INT_RAW` reader - The raw interrupt status bit for master_tra_comp_int interrupt. when I2C Master sends or receives a byte it will produce master_tran_comp_int interrupt."]
pub struct MASTER_TRAN_COMP_INT_RAW_R(crate::FieldReader<bool, bool>);
impl MASTER_TRAN_COMP_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MASTER_TRAN_COMP_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASTER_TRAN_COMP_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRANS_COMPLETE_INT_RAW` reader - The raw interrupt status bit for trans_complete_int interrupt. when I2C Master finished STOP command it will produce trans_complete_int interrupt."]
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
#[doc = "Field `TIME_OUT_INT_RAW` reader - The raw interrupt status bit for time_out_int interrupt. when I2C takes a lot of time to receive a data it will produce time_out_int interrupt."]
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
#[doc = "Field `TRANS_START_INT_RAW` reader - The raw interrupt status bit for trans_start_int interrupt. when I2C sends the START bit it will produce trans_start_int interrupt."]
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
#[doc = "Field `ACK_ERR_INT_RAW` reader - The raw interrupt status bit for ack_err_int interrupt. when I2C receives a wrong ACK bit it will produce ack_err_int interrupt.."]
pub struct ACK_ERR_INT_RAW_R(crate::FieldReader<bool, bool>);
impl ACK_ERR_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ACK_ERR_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACK_ERR_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_REC_FULL_INT_RAW` reader - The raw interrupt status bit for rx_rec_full_int interrupt. when I2C receives more data than nonfifo_rx_thres it will produce rx_rec_full_int interrupt."]
pub struct RX_REC_FULL_INT_RAW_R(crate::FieldReader<bool, bool>);
impl RX_REC_FULL_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_REC_FULL_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_REC_FULL_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_SEND_EMPTY_INT_RAW` reader - The raw interrupt status bit for tx_send_empty_int interrupt.when I2C sends more data than nonfifo_tx_thres it will produce tx_send_empty_int interrupt.."]
pub struct TX_SEND_EMPTY_INT_RAW_R(crate::FieldReader<bool, bool>);
impl TX_SEND_EMPTY_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_SEND_EMPTY_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_SEND_EMPTY_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - The raw interrupt status bit for rxfifo full when use apb fifo access."]
    #[inline(always)]
    pub fn rxfifo_full_int_raw(&self) -> RXFIFO_FULL_INT_RAW_R {
        RXFIFO_FULL_INT_RAW_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt status bit for txfifo empty when use apb fifo access."]
    #[inline(always)]
    pub fn txfifo_empty_int_raw(&self) -> TXFIFO_EMPTY_INT_RAW_R {
        TXFIFO_EMPTY_INT_RAW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt status bit for receiving data overflow when use apb fifo access."]
    #[inline(always)]
    pub fn rxfifo_ovf_int_raw(&self) -> RXFIFO_OVF_INT_RAW_R {
        RXFIFO_OVF_INT_RAW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - The raw interrupt status bit for end_detect_int interrupt. when I2C deals with the END command it will produce end_detect_int interrupt."]
    #[inline(always)]
    pub fn end_detect_int_raw(&self) -> END_DETECT_INT_RAW_R {
        END_DETECT_INT_RAW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - The raw interrupt status bit for slave_tran_comp_int interrupt. when I2C Slave detectsthe STOP bit it will produce slave_tran_comp_int interrupt."]
    #[inline(always)]
    pub fn slave_tran_comp_int_raw(&self) -> SLAVE_TRAN_COMP_INT_RAW_R {
        SLAVE_TRAN_COMP_INT_RAW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - The raw interrupt status bit for arbitration_lost_int interrupt.when I2C lost the usage right of I2C BUS it will produce arbitration_lost_int interrupt."]
    #[inline(always)]
    pub fn arbitration_lost_int_raw(&self) -> ARBITRATION_LOST_INT_RAW_R {
        ARBITRATION_LOST_INT_RAW_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - The raw interrupt status bit for master_tra_comp_int interrupt. when I2C Master sends or receives a byte it will produce master_tran_comp_int interrupt."]
    #[inline(always)]
    pub fn master_tran_comp_int_raw(&self) -> MASTER_TRAN_COMP_INT_RAW_R {
        MASTER_TRAN_COMP_INT_RAW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - The raw interrupt status bit for trans_complete_int interrupt. when I2C Master finished STOP command it will produce trans_complete_int interrupt."]
    #[inline(always)]
    pub fn trans_complete_int_raw(&self) -> TRANS_COMPLETE_INT_RAW_R {
        TRANS_COMPLETE_INT_RAW_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - The raw interrupt status bit for time_out_int interrupt. when I2C takes a lot of time to receive a data it will produce time_out_int interrupt."]
    #[inline(always)]
    pub fn time_out_int_raw(&self) -> TIME_OUT_INT_RAW_R {
        TIME_OUT_INT_RAW_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - The raw interrupt status bit for trans_start_int interrupt. when I2C sends the START bit it will produce trans_start_int interrupt."]
    #[inline(always)]
    pub fn trans_start_int_raw(&self) -> TRANS_START_INT_RAW_R {
        TRANS_START_INT_RAW_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - The raw interrupt status bit for ack_err_int interrupt. when I2C receives a wrong ACK bit it will produce ack_err_int interrupt.."]
    #[inline(always)]
    pub fn ack_err_int_raw(&self) -> ACK_ERR_INT_RAW_R {
        ACK_ERR_INT_RAW_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - The raw interrupt status bit for rx_rec_full_int interrupt. when I2C receives more data than nonfifo_rx_thres it will produce rx_rec_full_int interrupt."]
    #[inline(always)]
    pub fn rx_rec_full_int_raw(&self) -> RX_REC_FULL_INT_RAW_R {
        RX_REC_FULL_INT_RAW_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - The raw interrupt status bit for tx_send_empty_int interrupt.when I2C sends more data than nonfifo_tx_thres it will produce tx_send_empty_int interrupt.."]
    #[inline(always)]
    pub fn tx_send_empty_int_raw(&self) -> TX_SEND_EMPTY_INT_RAW_R {
        TX_SEND_EMPTY_INT_RAW_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
#[doc = "\n\nThis register you can [`read`]
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
