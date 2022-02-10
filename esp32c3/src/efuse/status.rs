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
#[doc = "Field `STATE` reader - Indicates the state of the eFuse state machine."]
pub struct STATE_R(crate::FieldReader<u8, u8>);
impl STATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        STATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STATE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OTP_LOAD_SW` reader - The value of OTP_LOAD_SW."]
pub struct OTP_LOAD_SW_R(crate::FieldReader<bool, bool>);
impl OTP_LOAD_SW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OTP_LOAD_SW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OTP_LOAD_SW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OTP_VDDQ_C_SYNC2` reader - The value of OTP_VDDQ_C_SYNC2."]
pub struct OTP_VDDQ_C_SYNC2_R(crate::FieldReader<bool, bool>);
impl OTP_VDDQ_C_SYNC2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OTP_VDDQ_C_SYNC2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OTP_VDDQ_C_SYNC2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OTP_STROBE_SW` reader - The value of OTP_STROBE_SW."]
pub struct OTP_STROBE_SW_R(crate::FieldReader<bool, bool>);
impl OTP_STROBE_SW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OTP_STROBE_SW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OTP_STROBE_SW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OTP_CSB_SW` reader - The value of OTP_CSB_SW."]
pub struct OTP_CSB_SW_R(crate::FieldReader<bool, bool>);
impl OTP_CSB_SW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OTP_CSB_SW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OTP_CSB_SW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OTP_PGENB_SW` reader - The value of OTP_PGENB_SW."]
pub struct OTP_PGENB_SW_R(crate::FieldReader<bool, bool>);
impl OTP_PGENB_SW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OTP_PGENB_SW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OTP_PGENB_SW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OTP_VDDQ_IS_SW` reader - The value of OTP_VDDQ_IS_SW."]
pub struct OTP_VDDQ_IS_SW_R(crate::FieldReader<bool, bool>);
impl OTP_VDDQ_IS_SW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OTP_VDDQ_IS_SW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OTP_VDDQ_IS_SW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REPEAT_ERR_CNT` reader - Indicates the number of error bits during programming BLOCK0."]
pub struct REPEAT_ERR_CNT_R(crate::FieldReader<u8, u8>);
impl REPEAT_ERR_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        REPEAT_ERR_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REPEAT_ERR_CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - Indicates the state of the eFuse state machine."]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - The value of OTP_LOAD_SW."]
    #[inline(always)]
    pub fn otp_load_sw(&self) -> OTP_LOAD_SW_R {
        OTP_LOAD_SW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - The value of OTP_VDDQ_C_SYNC2."]
    #[inline(always)]
    pub fn otp_vddq_c_sync2(&self) -> OTP_VDDQ_C_SYNC2_R {
        OTP_VDDQ_C_SYNC2_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - The value of OTP_STROBE_SW."]
    #[inline(always)]
    pub fn otp_strobe_sw(&self) -> OTP_STROBE_SW_R {
        OTP_STROBE_SW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - The value of OTP_CSB_SW."]
    #[inline(always)]
    pub fn otp_csb_sw(&self) -> OTP_CSB_SW_R {
        OTP_CSB_SW_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - The value of OTP_PGENB_SW."]
    #[inline(always)]
    pub fn otp_pgenb_sw(&self) -> OTP_PGENB_SW_R {
        OTP_PGENB_SW_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - The value of OTP_VDDQ_IS_SW."]
    #[inline(always)]
    pub fn otp_vddq_is_sw(&self) -> OTP_VDDQ_IS_SW_R {
        OTP_VDDQ_IS_SW_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 10:17 - Indicates the number of error bits during programming BLOCK0."]
    #[inline(always)]
    pub fn repeat_err_cnt(&self) -> REPEAT_ERR_CNT_R {
        REPEAT_ERR_CNT_R::new(((self.bits >> 10) & 0xff) as u8)
    }
}
#[doc = "eFuse status register.\n\nThis register you can [`read`]
(crate::generic::Reg::read). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status]
(index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R]
(R) reader structure"]
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
