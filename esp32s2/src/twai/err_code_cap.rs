#[doc = "Register `ERR_CODE_CAP` reader"]
pub struct R(crate::R<ERR_CODE_CAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERR_CODE_CAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERR_CODE_CAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERR_CODE_CAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ECC_SEGMENT` reader - This register contains information about the location of errors, see Table 181 for details."]
pub struct ECC_SEGMENT_R(crate::FieldReader<u8, u8>);
impl ECC_SEGMENT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ECC_SEGMENT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECC_SEGMENT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECC_DIRECTION` reader - This register contains information about transmission direction of the node when error occurs. 1: Error occurs when receiving a message; 0: Error occurs when transmitting a message"]
pub struct ECC_DIRECTION_R(crate::FieldReader<bool, bool>);
impl ECC_DIRECTION_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ECC_DIRECTION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECC_DIRECTION_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECC_TYPE` reader - This register contains information about error types: 00: bit error; 01: form error; 10: stuff error; 11: other type of error"]
pub struct ECC_TYPE_R(crate::FieldReader<u8, u8>);
impl ECC_TYPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ECC_TYPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECC_TYPE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:4 - This register contains information about the location of errors, see Table 181 for details."]
    #[inline(always)]
    pub fn ecc_segment(&self) -> ECC_SEGMENT_R {
        ECC_SEGMENT_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - This register contains information about transmission direction of the node when error occurs. 1: Error occurs when receiving a message; 0: Error occurs when transmitting a message"]
    #[inline(always)]
    pub fn ecc_direction(&self) -> ECC_DIRECTION_R {
        ECC_DIRECTION_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - This register contains information about error types: 00: bit error; 01: form error; 10: stuff error; 11: other type of error"]
    #[inline(always)]
    pub fn ecc_type(&self) -> ECC_TYPE_R {
        ECC_TYPE_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
#[doc = "Error Code Capture Register\n\nThis register you can [`read`]
(crate::generic::Reg::read). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [err_code_cap]
(index.html) module"]
pub struct ERR_CODE_CAP_SPEC;
impl crate::RegisterSpec for ERR_CODE_CAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [err_code_cap::R]
(R) reader structure"]
impl crate::Readable for ERR_CODE_CAP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ERR_CODE_CAP to value 0"]
impl crate::Resettable for ERR_CODE_CAP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
