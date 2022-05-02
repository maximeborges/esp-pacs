#[doc = "Register `CH%s_TX_STATUS` reader"]
pub struct R(crate::R<CH_TX_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH_TX_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH_TX_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH_TX_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MEM_RADDR_EX` reader - This register records the memory address offset when transmitter of CHANNEL%s is using the RAM."]
pub struct MEM_RADDR_EX_R(crate::FieldReader<u16>);
impl MEM_RADDR_EX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        MEM_RADDR_EX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM_RADDR_EX_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB_MEM_WADDR` reader - This register records the memory address offset when writes RAM over APB bus."]
pub struct APB_MEM_WADDR_R(crate::FieldReader<u16>);
impl APB_MEM_WADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        APB_MEM_WADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB_MEM_WADDR_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STATE` reader - This register records the FSM status of CHANNEL%s."]
pub struct STATE_R(crate::FieldReader<u8>);
impl STATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        STATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STATE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM_EMPTY` reader - This status bit will be set when the data to be set is more than memory size and the wraparound mode is disabled."]
pub struct MEM_EMPTY_R(crate::FieldReader<bool>);
impl MEM_EMPTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEM_EMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM_EMPTY_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB_MEM_WR_ERR` reader - This status bit will be set if the offset address out of memory size when writes via APB bus."]
pub struct APB_MEM_WR_ERR_R(crate::FieldReader<bool>);
impl APB_MEM_WR_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APB_MEM_WR_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB_MEM_WR_ERR_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:9 - This register records the memory address offset when transmitter of CHANNEL%s is using the RAM."]
    #[inline(always)]
    pub fn mem_raddr_ex(&self) -> MEM_RADDR_EX_R {
        MEM_RADDR_EX_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 11:20 - This register records the memory address offset when writes RAM over APB bus."]
    #[inline(always)]
    pub fn apb_mem_waddr(&self) -> APB_MEM_WADDR_R {
        APB_MEM_WADDR_R::new(((self.bits >> 11) & 0x03ff) as u16)
    }
    #[doc = "Bits 22:24 - This register records the FSM status of CHANNEL%s."]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bit 25 - This status bit will be set when the data to be set is more than memory size and the wraparound mode is disabled."]
    #[inline(always)]
    pub fn mem_empty(&self) -> MEM_EMPTY_R {
        MEM_EMPTY_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - This status bit will be set if the offset address out of memory size when writes via APB bus."]
    #[inline(always)]
    pub fn apb_mem_wr_err(&self) -> APB_MEM_WR_ERR_R {
        APB_MEM_WR_ERR_R::new(((self.bits >> 26) & 1) != 0)
    }
}
#[doc = "Channel %s status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch_tx_status](index.html) module"]
pub struct CH_TX_STATUS_SPEC;
impl crate::RegisterSpec for CH_TX_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch_tx_status::R](R) reader structure"]
impl crate::Readable for CH_TX_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CH%s_TX_STATUS to value 0"]
impl crate::Resettable for CH_TX_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
