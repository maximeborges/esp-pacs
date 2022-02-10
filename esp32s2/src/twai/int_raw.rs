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
#[doc = "Field `RX_INT_ST` reader - Receive interrupt. If this bit is set to 1, it indicates there are messages to be handled in the RX FIFO."]
pub struct RX_INT_ST_R(crate::FieldReader<bool, bool>);
impl RX_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_INT_ST` reader - Transmit interrupt. If this bit is set to 1, it indicates the message transmitting mis- sion is finished and a new transmission is able to execute."]
pub struct TX_INT_ST_R(crate::FieldReader<bool, bool>);
impl TX_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERR_WARN_INT_ST` reader - Error warning interrupt. If this bit is set to 1, it indicates the error status signal and the bus-off status signal of Status register have changed (e.g., switched from 0 to 1 or from 1 to 0)."]
pub struct ERR_WARN_INT_ST_R(crate::FieldReader<bool, bool>);
impl ERR_WARN_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERR_WARN_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERR_WARN_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVERRUN_INT_ST` reader - Data overrun interrupt. If this bit is set to 1, it indicates a data overrun interrupt is generated in the RX FIFO."]
pub struct OVERRUN_INT_ST_R(crate::FieldReader<bool, bool>);
impl OVERRUN_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVERRUN_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVERRUN_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERR_PASSIVE_INT_ST` reader - Error passive interrupt. If this bit is set to 1, it indicates the TWAI Controller is switched between error active status and error passive status due to the change of error counters."]
pub struct ERR_PASSIVE_INT_ST_R(crate::FieldReader<bool, bool>);
impl ERR_PASSIVE_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERR_PASSIVE_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERR_PASSIVE_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARB_LOST_INT_ST` reader - Arbitration lost interrupt. If this bit is set to 1, it indicates an arbitration lost interrupt is generated."]
pub struct ARB_LOST_INT_ST_R(crate::FieldReader<bool, bool>);
impl ARB_LOST_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ARB_LOST_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARB_LOST_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUS_ERR_INT_ST` reader - Error interrupt. If this bit is set to 1, it indicates an error is detected on the bus."]
pub struct BUS_ERR_INT_ST_R(crate::FieldReader<bool, bool>);
impl BUS_ERR_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUS_ERR_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUS_ERR_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Receive interrupt. If this bit is set to 1, it indicates there are messages to be handled in the RX FIFO."]
    #[inline(always)]
    pub fn rx_int_st(&self) -> RX_INT_ST_R {
        RX_INT_ST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit interrupt. If this bit is set to 1, it indicates the message transmitting mis- sion is finished and a new transmission is able to execute."]
    #[inline(always)]
    pub fn tx_int_st(&self) -> TX_INT_ST_R {
        TX_INT_ST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Error warning interrupt. If this bit is set to 1, it indicates the error status signal and the bus-off status signal of Status register have changed (e.g., switched from 0 to 1 or from 1 to 0)."]
    #[inline(always)]
    pub fn err_warn_int_st(&self) -> ERR_WARN_INT_ST_R {
        ERR_WARN_INT_ST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Data overrun interrupt. If this bit is set to 1, it indicates a data overrun interrupt is generated in the RX FIFO."]
    #[inline(always)]
    pub fn overrun_int_st(&self) -> OVERRUN_INT_ST_R {
        OVERRUN_INT_ST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Error passive interrupt. If this bit is set to 1, it indicates the TWAI Controller is switched between error active status and error passive status due to the change of error counters."]
    #[inline(always)]
    pub fn err_passive_int_st(&self) -> ERR_PASSIVE_INT_ST_R {
        ERR_PASSIVE_INT_ST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Arbitration lost interrupt. If this bit is set to 1, it indicates an arbitration lost interrupt is generated."]
    #[inline(always)]
    pub fn arb_lost_int_st(&self) -> ARB_LOST_INT_ST_R {
        ARB_LOST_INT_ST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Error interrupt. If this bit is set to 1, it indicates an error is detected on the bus."]
    #[inline(always)]
    pub fn bus_err_int_st(&self) -> BUS_ERR_INT_ST_R {
        BUS_ERR_INT_ST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
#[doc = "Interrupt Register\n\nThis register you can [`read`]
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
