#[doc = "Register `CH%sSTATUS` reader"]
pub struct R(crate::R<CHSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MEM_WADDR_EX_CH0` reader - This register records the memory address offset when receiver of CHANNEL%s is using the RAM."]
pub struct MEM_WADDR_EX_CH0_R(crate::FieldReader<u16, u16>);
impl MEM_WADDR_EX_CH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        MEM_WADDR_EX_CH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM_WADDR_EX_CH0_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM_RADDR_EX_CH0` reader - This register records the memory address offset when transmitter of CHANNEL%s is using the RAM."]
pub struct MEM_RADDR_EX_CH0_R(crate::FieldReader<u16, u16>);
impl MEM_RADDR_EX_CH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        MEM_RADDR_EX_CH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM_RADDR_EX_CH0_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STATE_CH0` reader - This register records the FSM status of CHANNEL%s."]
pub struct STATE_CH0_R(crate::FieldReader<u8, u8>);
impl STATE_CH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        STATE_CH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STATE_CH0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM_OWNER_ERR_CH0` reader - This status bit will be set when the ownership of memory block is wrong."]
pub struct MEM_OWNER_ERR_CH0_R(crate::FieldReader<bool, bool>);
impl MEM_OWNER_ERR_CH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEM_OWNER_ERR_CH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM_OWNER_ERR_CH0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM_FULL_CH0` reader - This status bit will be set if the receiver receives more data than the memory size."]
pub struct MEM_FULL_CH0_R(crate::FieldReader<bool, bool>);
impl MEM_FULL_CH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEM_FULL_CH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM_FULL_CH0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM_EMPTY_CH0` reader - This status bit will be set when the data to be set is more than memory size and the wraparound mode is disabled."]
pub struct MEM_EMPTY_CH0_R(crate::FieldReader<bool, bool>);
impl MEM_EMPTY_CH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEM_EMPTY_CH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM_EMPTY_CH0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB_MEM_WR_ERR_CH0` reader - This status bit will be set if the offset address out of memory size when writes via APB bus."]
pub struct APB_MEM_WR_ERR_CH0_R(crate::FieldReader<bool, bool>);
impl APB_MEM_WR_ERR_CH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APB_MEM_WR_ERR_CH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB_MEM_WR_ERR_CH0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB_MEM_RD_ERR_CH0` reader - This status bit will be set if the offset address out of memory size when reads via APB bus."]
pub struct APB_MEM_RD_ERR_CH0_R(crate::FieldReader<bool, bool>);
impl APB_MEM_RD_ERR_CH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APB_MEM_RD_ERR_CH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB_MEM_RD_ERR_CH0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:8 - This register records the memory address offset when receiver of CHANNEL%s is using the RAM."]
    #[inline(always)]
    pub fn mem_waddr_ex_ch0(&self) -> MEM_WADDR_EX_CH0_R {
        MEM_WADDR_EX_CH0_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 10:18 - This register records the memory address offset when transmitter of CHANNEL%s is using the RAM."]
    #[inline(always)]
    pub fn mem_raddr_ex_ch0(&self) -> MEM_RADDR_EX_CH0_R {
        MEM_RADDR_EX_CH0_R::new(((self.bits >> 10) & 0x01ff) as u16)
    }
    #[doc = "Bits 20:22 - This register records the FSM status of CHANNEL%s."]
    #[inline(always)]
    pub fn state_ch0(&self) -> STATE_CH0_R {
        STATE_CH0_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bit 23 - This status bit will be set when the ownership of memory block is wrong."]
    #[inline(always)]
    pub fn mem_owner_err_ch0(&self) -> MEM_OWNER_ERR_CH0_R {
        MEM_OWNER_ERR_CH0_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - This status bit will be set if the receiver receives more data than the memory size."]
    #[inline(always)]
    pub fn mem_full_ch0(&self) -> MEM_FULL_CH0_R {
        MEM_FULL_CH0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - This status bit will be set when the data to be set is more than memory size and the wraparound mode is disabled."]
    #[inline(always)]
    pub fn mem_empty_ch0(&self) -> MEM_EMPTY_CH0_R {
        MEM_EMPTY_CH0_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - This status bit will be set if the offset address out of memory size when writes via APB bus."]
    #[inline(always)]
    pub fn apb_mem_wr_err_ch0(&self) -> APB_MEM_WR_ERR_CH0_R {
        APB_MEM_WR_ERR_CH0_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - This status bit will be set if the offset address out of memory size when reads via APB bus."]
    #[inline(always)]
    pub fn apb_mem_rd_err_ch0(&self) -> APB_MEM_RD_ERR_CH0_R {
        APB_MEM_RD_ERR_CH0_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
#[doc = "Channel %s status register\n\nThis register you can [`read`]
(crate::generic::Reg::read). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chstatus]
(index.html) module"]
pub struct CHSTATUS_SPEC;
impl crate::RegisterSpec for CHSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chstatus::R]
(R) reader structure"]
impl crate::Readable for CHSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CH%sSTATUS to value 0"]
impl crate::Resettable for CHSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
