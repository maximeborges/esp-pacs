#[doc = "Register `EXTMEM_REJECT_ST` reader"]
pub struct R(crate::R<EXTMEM_REJECT_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTMEM_REJECT_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTMEM_REJECT_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTMEM_REJECT_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EXTMEM_REJECT_ATRR` reader - The reject accessing. Bit 0: if this bit is 1, the rejected accessing is READ. Bit 1: if this bit is 1, the rejected accessing is WRITE."]
pub struct EXTMEM_REJECT_ATRR_R(crate::FieldReader<u8, u8>);
impl EXTMEM_REJECT_ATRR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EXTMEM_REJECT_ATRR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTMEM_REJECT_ATRR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTMEM_REJECT_CHANNEL_NUM` reader - The register indicate the reject accessing from which channel."]
pub struct EXTMEM_REJECT_CHANNEL_NUM_R(crate::FieldReader<u8, u8>);
impl EXTMEM_REJECT_CHANNEL_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EXTMEM_REJECT_CHANNEL_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTMEM_REJECT_CHANNEL_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTMEM_REJECT_PERI_NUM` reader - This register indicate reject accessing from which peripheral."]
pub struct EXTMEM_REJECT_PERI_NUM_R(crate::FieldReader<u8, u8>);
impl EXTMEM_REJECT_PERI_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EXTMEM_REJECT_PERI_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTMEM_REJECT_PERI_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:1 - The reject accessing. Bit 0: if this bit is 1, the rejected accessing is READ. Bit 1: if this bit is 1, the rejected accessing is WRITE."]
    #[inline(always)]
    pub fn extmem_reject_atrr(&self) -> EXTMEM_REJECT_ATRR_R {
        EXTMEM_REJECT_ATRR_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:5 - The register indicate the reject accessing from which channel."]
    #[inline(always)]
    pub fn extmem_reject_channel_num(&self) -> EXTMEM_REJECT_CHANNEL_NUM_R {
        EXTMEM_REJECT_CHANNEL_NUM_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:11 - This register indicate reject accessing from which peripheral."]
    #[inline(always)]
    pub fn extmem_reject_peri_num(&self) -> EXTMEM_REJECT_PERI_NUM_R {
        EXTMEM_REJECT_PERI_NUM_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
}
#[doc = "Reject status accessing external RAM\n\nThis register you can [`read`]
(crate::generic::Reg::read). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_reject_st]
(index.html) module"]
pub struct EXTMEM_REJECT_ST_SPEC;
impl crate::RegisterSpec for EXTMEM_REJECT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [extmem_reject_st::R]
(R) reader structure"]
impl crate::Readable for EXTMEM_REJECT_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EXTMEM_REJECT_ST to value 0"]
impl crate::Resettable for EXTMEM_REJECT_ST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
