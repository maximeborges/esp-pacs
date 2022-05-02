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
#[doc = "Field `ACK_REC` reader - ack response"]
pub struct ACK_REC_R(crate::FieldReader<bool>);
impl ACK_REC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ACK_REC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACK_REC_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLAVE_RW` reader - slave read or write"]
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
#[doc = "Field `ARB_LOST` reader - arbitration is lost"]
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
#[doc = "Field `BUS_BUSY` reader - bus is busy"]
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
#[doc = "Field `SLAVE_ADDRESSED` reader - slave reg sub address"]
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
#[doc = "Field `BYTE_TRANS` reader - One byte transit done"]
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
#[doc = "Field `OP_CNT` reader - which operation is working"]
pub struct OP_CNT_R(crate::FieldReader<u8>);
impl OP_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OP_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OP_CNT_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SHIFT` reader - shifter content"]
pub struct SHIFT_R(crate::FieldReader<u8>);
impl SHIFT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SHIFT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SHIFT_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCL_MAIN_STATE_LAST` reader - i2c last main status"]
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
#[doc = "Field `SCL_STATE_LAST` reader - scl last status"]
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
    #[doc = "Bit 0 - ack response"]
    #[inline(always)]
    pub fn ack_rec(&self) -> ACK_REC_R {
        ACK_REC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - slave read or write"]
    #[inline(always)]
    pub fn slave_rw(&self) -> SLAVE_RW_R {
        SLAVE_RW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - arbitration is lost"]
    #[inline(always)]
    pub fn arb_lost(&self) -> ARB_LOST_R {
        ARB_LOST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - bus is busy"]
    #[inline(always)]
    pub fn bus_busy(&self) -> BUS_BUSY_R {
        BUS_BUSY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - slave reg sub address"]
    #[inline(always)]
    pub fn slave_addressed(&self) -> SLAVE_ADDRESSED_R {
        SLAVE_ADDRESSED_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - One byte transit done"]
    #[inline(always)]
    pub fn byte_trans(&self) -> BYTE_TRANS_R {
        BYTE_TRANS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - which operation is working"]
    #[inline(always)]
    pub fn op_cnt(&self) -> OP_CNT_R {
        OP_CNT_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 16:23 - shifter content"]
    #[inline(always)]
    pub fn shift(&self) -> SHIFT_R {
        SHIFT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:26 - i2c last main status"]
    #[inline(always)]
    pub fn scl_main_state_last(&self) -> SCL_MAIN_STATE_LAST_R {
        SCL_MAIN_STATE_LAST_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - scl last status"]
    #[inline(always)]
    pub fn scl_state_last(&self) -> SCL_STATE_LAST_R {
        SCL_STATE_LAST_R::new(((self.bits >> 28) & 7) as u8)
    }
}
#[doc = "get i2c status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
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
