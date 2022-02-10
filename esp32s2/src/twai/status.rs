#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RX_BUF_ST` reader - 1: The data in the RX buffer is not empty, with at least one received data packet."]
pub struct RX_BUF_ST_R(crate::FieldReader<bool, bool>);
impl RX_BUF_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_BUF_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_BUF_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVERRUN_ST` reader - 1: The RX FIFO is full and data overrun has occurred."]
pub struct OVERRUN_ST_R(crate::FieldReader<bool, bool>);
impl OVERRUN_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVERRUN_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVERRUN_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_BUF_ST` reader - 1: The TX buffer is empty, the CPU may write a message into it."]
pub struct TX_BUF_ST_R(crate::FieldReader<bool, bool>);
impl TX_BUF_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_BUF_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_BUF_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_COMPLETE` reader - 1: The TWAI controller has successfully received a packet from the bus."]
pub struct TX_COMPLETE_R(crate::FieldReader<bool, bool>);
impl TX_COMPLETE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_COMPLETE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_COMPLETE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_ST` reader - 1: The TWAI Controller is receiving a message from the bus."]
pub struct RX_ST_R(crate::FieldReader<bool, bool>);
impl RX_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_ST` reader - 1: The TWAI Controller is transmitting a message to the bus."]
pub struct TX_ST_R(crate::FieldReader<bool, bool>);
impl TX_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERR_ST` reader - 1: At least one of the RX/TX error counter has reached or exceeded the value set in register TWAI_ERR_WARNING_LIMIT_REG."]
pub struct ERR_ST_R(crate::FieldReader<bool, bool>);
impl ERR_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERR_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERR_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUS_OFF_ST` reader - 1: In bus-off status, the TWAI Controller is no longer involved in bus activities."]
pub struct BUS_OFF_ST_R(crate::FieldReader<bool, bool>);
impl BUS_OFF_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUS_OFF_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUS_OFF_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MISS_ST` reader - This bit reflects whether the data packet in the RX FIFO is complete. 1: The current packet is missing; 0: The current packet is complete"]
pub struct MISS_ST_R(crate::FieldReader<bool, bool>);
impl MISS_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MISS_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MISS_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - 1: The data in the RX buffer is not empty, with at least one received data packet."]
    #[inline(always)]
    pub fn rx_buf_st(&self) -> RX_BUF_ST_R {
        RX_BUF_ST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1: The RX FIFO is full and data overrun has occurred."]
    #[inline(always)]
    pub fn overrun_st(&self) -> OVERRUN_ST_R {
        OVERRUN_ST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 1: The TX buffer is empty, the CPU may write a message into it."]
    #[inline(always)]
    pub fn tx_buf_st(&self) -> TX_BUF_ST_R {
        TX_BUF_ST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 1: The TWAI controller has successfully received a packet from the bus."]
    #[inline(always)]
    pub fn tx_complete(&self) -> TX_COMPLETE_R {
        TX_COMPLETE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 1: The TWAI Controller is receiving a message from the bus."]
    #[inline(always)]
    pub fn rx_st(&self) -> RX_ST_R {
        RX_ST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 1: The TWAI Controller is transmitting a message to the bus."]
    #[inline(always)]
    pub fn tx_st(&self) -> TX_ST_R {
        TX_ST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 1: At least one of the RX/TX error counter has reached or exceeded the value set in register TWAI_ERR_WARNING_LIMIT_REG."]
    #[inline(always)]
    pub fn err_st(&self) -> ERR_ST_R {
        ERR_ST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 1: In bus-off status, the TWAI Controller is no longer involved in bus activities."]
    #[inline(always)]
    pub fn bus_off_st(&self) -> BUS_OFF_ST_R {
        BUS_OFF_ST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - This bit reflects whether the data packet in the RX FIFO is complete. 1: The current packet is missing; 0: The current packet is complete"]
    #[inline(always)]
    pub fn miss_st(&self) -> MISS_ST_R {
        MISS_ST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
#[doc = "Status register\n\nThis register you can [`read`]
(crate::generic::Reg::read). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status]
(index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R]
(R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
