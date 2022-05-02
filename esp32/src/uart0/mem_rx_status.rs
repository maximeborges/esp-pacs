#[doc = "Register `MEM_RX_STATUS` reader"]
pub struct R(crate::R<MEM_RX_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEM_RX_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEM_RX_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEM_RX_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MEM_RX_STATUS` reader - This register stores the current uart rx mem read address and rx mem write address"]
pub struct MEM_RX_STATUS_R(crate::FieldReader<u32>);
impl MEM_RX_STATUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        MEM_RX_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM_RX_STATUS_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM_RX_RD_ADDR` reader - This register stores the rx mem read address"]
pub struct MEM_RX_RD_ADDR_R(crate::FieldReader<u16>);
impl MEM_RX_RD_ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        MEM_RX_RD_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM_RX_RD_ADDR_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM_RX_WR_ADDR` reader - This register stores the rx mem write address"]
pub struct MEM_RX_WR_ADDR_R(crate::FieldReader<u16>);
impl MEM_RX_WR_ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        MEM_RX_WR_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM_RX_WR_ADDR_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:23 - This register stores the current uart rx mem read address and rx mem write address"]
    #[inline(always)]
    pub fn mem_rx_status(&self) -> MEM_RX_STATUS_R {
        MEM_RX_STATUS_R::new((self.bits & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 2:12 - This register stores the rx mem read address"]
    #[inline(always)]
    pub fn mem_rx_rd_addr(&self) -> MEM_RX_RD_ADDR_R {
        MEM_RX_RD_ADDR_R::new(((self.bits >> 2) & 0x07ff) as u16)
    }
    #[doc = "Bits 13:23 - This register stores the rx mem write address"]
    #[inline(always)]
    pub fn mem_rx_wr_addr(&self) -> MEM_RX_WR_ADDR_R {
        MEM_RX_WR_ADDR_R::new(((self.bits >> 13) & 0x07ff) as u16)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_rx_status](index.html) module"]
pub struct MEM_RX_STATUS_SPEC;
impl crate::RegisterSpec for MEM_RX_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mem_rx_status::R](R) reader structure"]
impl crate::Readable for MEM_RX_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MEM_RX_STATUS to value 0"]
impl crate::Resettable for MEM_RX_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
