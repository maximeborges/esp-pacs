#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RESP_REC` reader - The received ACK value in master mode or slave mode. 0: ACK. 1: NACK."]
pub struct RESP_REC_R(crate::FieldReader<bool>);
impl RESP_REC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RESP_REC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESP_REC_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLAVE_RW` reader - When in slave mode, 1: master reads from slave. 0: master writes to slave."]
pub struct SLAVE_RW_R(crate::FieldReader<bool>);
impl SLAVE_RW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLAVE_RW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLAVE_RW_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIME_OUT` reader - When the I2C controller takes more than I2C_TIME_OUT clocks to receive a data bit, this field changes to 1."]
pub struct TIME_OUT_R(crate::FieldReader<bool>);
impl TIME_OUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIME_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIME_OUT_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARB_LOST` reader - When the I2C controller loses control of SCL line, this register changes to 1."]
pub struct ARB_LOST_R(crate::FieldReader<bool>);
impl ARB_LOST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ARB_LOST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARB_LOST_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUS_BUSY` reader - 1: the I2C bus is busy transferring data. 0: the I2C bus is in idle state."]
pub struct BUS_BUSY_R(crate::FieldReader<bool>);
impl BUS_BUSY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUS_BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUS_BUSY_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLAVE_ADDRESSED` reader - When configured as an I2C Slave, and the address sent by the master is equal to the address of the slave, then this bit will be of high level."]
pub struct SLAVE_ADDRESSED_R(crate::FieldReader<bool>);
impl SLAVE_ADDRESSED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLAVE_ADDRESSED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLAVE_ADDRESSED_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BYTE_TRANS` reader - This field changes to 1 when one byte is transferred."]
pub struct BYTE_TRANS_R(crate::FieldReader<bool>);
impl BYTE_TRANS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BYTE_TRANS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTE_TRANS_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFIFO_CNT` reader - This field represents the amount of data needed to be sent."]
pub struct RXFIFO_CNT_R(crate::FieldReader<u8>);
impl RXFIFO_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RXFIFO_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFIFO_CNT_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STRETCH_CAUSE` reader - The cause of stretching SCL low in slave mode. 0: stretching SCL low at the beginning of I2C read data state. 1: stretching SCL low when I2C TX FIFO is empty in slave mode. 2: stretching SCL low when I2C RX FIFO is full in slave mode."]
pub struct STRETCH_CAUSE_R(crate::FieldReader<u8>);
impl STRETCH_CAUSE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        STRETCH_CAUSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STRETCH_CAUSE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFIFO_CNT` reader - This field stores the amount of received data in RAM."]
pub struct TXFIFO_CNT_R(crate::FieldReader<u8>);
impl TXFIFO_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TXFIFO_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFIFO_CNT_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCL_MAIN_STATE_LAST` reader - This field indicates the states of the I2C module state machine. 0: Idle. 1: Address shift. 2: ACK address. 3: RX data. 4: TX data. 5: Send ACK. 6: Wait ACK"]
pub struct SCL_MAIN_STATE_LAST_R(crate::FieldReader<u8>);
impl SCL_MAIN_STATE_LAST_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SCL_MAIN_STATE_LAST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCL_MAIN_STATE_LAST_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCL_STATE_LAST` reader - This field indicates the states of the state machine used to produce SCL. 0: Idle. 1: Start. 2: Negative edge. 3: Low. 4: Positive edge. 5: High. 6: Stop"]
pub struct SCL_STATE_LAST_R(crate::FieldReader<u8>);
impl SCL_STATE_LAST_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SCL_STATE_LAST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCL_STATE_LAST_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - The received ACK value in master mode or slave mode. 0: ACK. 1: NACK."]
    #[inline(always)]
    pub fn resp_rec(&self) -> RESP_REC_R {
        RESP_REC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When in slave mode, 1: master reads from slave. 0: master writes to slave."]
    #[inline(always)]
    pub fn slave_rw(&self) -> SLAVE_RW_R {
        SLAVE_RW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When the I2C controller takes more than I2C_TIME_OUT clocks to receive a data bit, this field changes to 1."]
    #[inline(always)]
    pub fn time_out(&self) -> TIME_OUT_R {
        TIME_OUT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When the I2C controller loses control of SCL line, this register changes to 1."]
    #[inline(always)]
    pub fn arb_lost(&self) -> ARB_LOST_R {
        ARB_LOST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1: the I2C bus is busy transferring data. 0: the I2C bus is in idle state."]
    #[inline(always)]
    pub fn bus_busy(&self) -> BUS_BUSY_R {
        BUS_BUSY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - When configured as an I2C Slave, and the address sent by the master is equal to the address of the slave, then this bit will be of high level."]
    #[inline(always)]
    pub fn slave_addressed(&self) -> SLAVE_ADDRESSED_R {
        SLAVE_ADDRESSED_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This field changes to 1 when one byte is transferred."]
    #[inline(always)]
    pub fn byte_trans(&self) -> BYTE_TRANS_R {
        BYTE_TRANS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:13 - This field represents the amount of data needed to be sent."]
    #[inline(always)]
    pub fn rxfifo_cnt(&self) -> RXFIFO_CNT_R {
        RXFIFO_CNT_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:15 - The cause of stretching SCL low in slave mode. 0: stretching SCL low at the beginning of I2C read data state. 1: stretching SCL low when I2C TX FIFO is empty in slave mode. 2: stretching SCL low when I2C RX FIFO is full in slave mode."]
    #[inline(always)]
    pub fn stretch_cause(&self) -> STRETCH_CAUSE_R {
        STRETCH_CAUSE_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 18:23 - This field stores the amount of received data in RAM."]
    #[inline(always)]
    pub fn txfifo_cnt(&self) -> TXFIFO_CNT_R {
        TXFIFO_CNT_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 24:26 - This field indicates the states of the I2C module state machine. 0: Idle. 1: Address shift. 2: ACK address. 3: RX data. 4: TX data. 5: Send ACK. 6: Wait ACK"]
    #[inline(always)]
    pub fn scl_main_state_last(&self) -> SCL_MAIN_STATE_LAST_R {
        SCL_MAIN_STATE_LAST_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - This field indicates the states of the state machine used to produce SCL. 0: Idle. 1: Start. 2: Negative edge. 3: Low. 4: Positive edge. 5: High. 6: Stop"]
    #[inline(always)]
    pub fn scl_state_last(&self) -> SCL_STATE_LAST_R {
        SCL_STATE_LAST_R::new(((self.bits >> 28) & 7) as u8)
    }
}
#[doc = "Describe I2C work status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
