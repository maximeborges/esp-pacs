#[doc = "Register `MEM_TX_STATUS` reader"]
pub struct R(crate::R<MEM_TX_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEM_TX_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEM_TX_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEM_TX_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MEM_TX_STATUS` reader - "]
pub struct MEM_TX_STATUS_R(crate::FieldReader<u32>);
impl MEM_TX_STATUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        MEM_TX_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM_TX_STATUS_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn mem_tx_status(&self) -> MEM_TX_STATUS_R {
        MEM_TX_STATUS_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_tx_status](index.html) module"]
pub struct MEM_TX_STATUS_SPEC;
impl crate::RegisterSpec for MEM_TX_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mem_tx_status::R](R) reader structure"]
impl crate::Readable for MEM_TX_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MEM_TX_STATUS to value 0"]
impl crate::Resettable for MEM_TX_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
