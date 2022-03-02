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
#[doc = "Field `RESP_REC` reader - reg_resp_rec"]
pub struct RESP_REC_R(crate::FieldReader<bool, bool>);
impl RESP_REC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RESP_REC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESP_REC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLAVE_RW` reader - reg_slave_rw"]
pub struct SLAVE_RW_R(crate::FieldReader<bool, bool>);
impl SLAVE_RW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLAVE_RW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLAVE_RW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARB_LOST` reader - reg_arb_lost"]
pub struct ARB_LOST_R(crate::FieldReader<bool, bool>);
impl ARB_LOST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ARB_LOST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARB_LOST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUS_BUSY` reader - reg_bus_busy"]
pub struct BUS_BUSY_R(crate::FieldReader<bool, bool>);
impl BUS_BUSY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUS_BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUS_BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLAVE_ADDRESSED` reader - reg_slave_addressed"]
pub struct SLAVE_ADDRESSED_R(crate::FieldReader<bool, bool>);
impl SLAVE_ADDRESSED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLAVE_ADDRESSED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLAVE_ADDRESSED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFIFO_CNT` reader - reg_rxfifo_cnt"]
pub struct RXFIFO_CNT_R(crate::FieldReader<u8, u8>);
impl RXFIFO_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RXFIFO_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFIFO_CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STRETCH_CAUSE` reader - reg_stretch_cause"]
pub struct STRETCH_CAUSE_R(crate::FieldReader<u8, u8>);
impl STRETCH_CAUSE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        STRETCH_CAUSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STRETCH_CAUSE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFIFO_CNT` reader - reg_txfifo_cnt"]
pub struct TXFIFO_CNT_R(crate::FieldReader<u8, u8>);
impl TXFIFO_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TXFIFO_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFIFO_CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCL_MAIN_STATE_LAST` reader - reg_scl_main_state_last"]
pub struct SCL_MAIN_STATE_LAST_R(crate::FieldReader<u8, u8>);
impl SCL_MAIN_STATE_LAST_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SCL_MAIN_STATE_LAST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCL_MAIN_STATE_LAST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCL_STATE_LAST` reader - reg_scl_state_last"]
pub struct SCL_STATE_LAST_R(crate::FieldReader<u8, u8>);
impl SCL_STATE_LAST_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SCL_STATE_LAST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCL_STATE_LAST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - reg_resp_rec"]
    #[inline(always)]
    pub fn resp_rec(&self) -> RESP_REC_R {
        RESP_REC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - reg_slave_rw"]
    #[inline(always)]
    pub fn slave_rw(&self) -> SLAVE_RW_R {
        SLAVE_RW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - reg_arb_lost"]
    #[inline(always)]
    pub fn arb_lost(&self) -> ARB_LOST_R {
        ARB_LOST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - reg_bus_busy"]
    #[inline(always)]
    pub fn bus_busy(&self) -> BUS_BUSY_R {
        BUS_BUSY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - reg_slave_addressed"]
    #[inline(always)]
    pub fn slave_addressed(&self) -> SLAVE_ADDRESSED_R {
        SLAVE_ADDRESSED_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 8:13 - reg_rxfifo_cnt"]
    #[inline(always)]
    pub fn rxfifo_cnt(&self) -> RXFIFO_CNT_R {
        RXFIFO_CNT_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:15 - reg_stretch_cause"]
    #[inline(always)]
    pub fn stretch_cause(&self) -> STRETCH_CAUSE_R {
        STRETCH_CAUSE_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 18:23 - reg_txfifo_cnt"]
    #[inline(always)]
    pub fn txfifo_cnt(&self) -> TXFIFO_CNT_R {
        TXFIFO_CNT_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 24:26 - reg_scl_main_state_last"]
    #[inline(always)]
    pub fn scl_main_state_last(&self) -> SCL_MAIN_STATE_LAST_R {
        SCL_MAIN_STATE_LAST_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 28:30 - reg_scl_state_last"]
    #[inline(always)]
    pub fn scl_state_last(&self) -> SCL_STATE_LAST_R {
        SCL_STATE_LAST_R::new(((self.bits >> 28) & 0x07) as u8)
    }
}
#[doc = "I2C_SR_REG\n\nThis register you can [`read`]
(crate::generic::Reg::read). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr]
(index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R]
(R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR to value 0xc000"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xc000
    }
}
