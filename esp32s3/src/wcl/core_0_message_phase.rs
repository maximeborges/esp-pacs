#[doc = "Register `Core_0_MESSAGE_PHASE` reader"]
pub struct R(crate::R<CORE_0_MESSAGE_PHASE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_MESSAGE_PHASE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_MESSAGE_PHASE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_MESSAGE_PHASE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CORE_0_MESSAGE_MATCH` reader - This bit indicates whether the check is successful"]
pub struct CORE_0_MESSAGE_MATCH_R(crate::FieldReader<bool, bool>);
impl CORE_0_MESSAGE_MATCH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CORE_0_MESSAGE_MATCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_MESSAGE_MATCH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_MESSAGE_EXPECT` reader - This field indicates the data to be written next time"]
pub struct CORE_0_MESSAGE_EXPECT_R(crate::FieldReader<u8, u8>);
impl CORE_0_MESSAGE_EXPECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_0_MESSAGE_EXPECT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_MESSAGE_EXPECT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_MESSAGE_DATAPHASE` reader - If this bit is 1, it means that is checking clear write_buffer operation,and is checking data"]
pub struct CORE_0_MESSAGE_DATAPHASE_R(crate::FieldReader<bool, bool>);
impl CORE_0_MESSAGE_DATAPHASE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CORE_0_MESSAGE_DATAPHASE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_MESSAGE_DATAPHASE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_MESSAGE_ADDRESSPHASE` reader - If this bit is 1, it means that is checking clear write_buffer operation,and is checking address."]
pub struct CORE_0_MESSAGE_ADDRESSPHASE_R(crate::FieldReader<bool, bool>);
impl CORE_0_MESSAGE_ADDRESSPHASE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CORE_0_MESSAGE_ADDRESSPHASE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_MESSAGE_ADDRESSPHASE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - This bit indicates whether the check is successful"]
    #[inline(always)]
    pub fn core_0_message_match(&self) -> CORE_0_MESSAGE_MATCH_R {
        CORE_0_MESSAGE_MATCH_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:4 - This field indicates the data to be written next time"]
    #[inline(always)]
    pub fn core_0_message_expect(&self) -> CORE_0_MESSAGE_EXPECT_R {
        CORE_0_MESSAGE_EXPECT_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 5 - If this bit is 1, it means that is checking clear write_buffer operation,and is checking data"]
    #[inline(always)]
    pub fn core_0_message_dataphase(&self) -> CORE_0_MESSAGE_DATAPHASE_R {
        CORE_0_MESSAGE_DATAPHASE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - If this bit is 1, it means that is checking clear write_buffer operation,and is checking address."]
    #[inline(always)]
    pub fn core_0_message_addressphase(&self) -> CORE_0_MESSAGE_ADDRESSPHASE_R {
        CORE_0_MESSAGE_ADDRESSPHASE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
#[doc = "Clear writer_buffer status register\n\nThis register you can [`read`]
(crate::generic::Reg::read). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_message_phase]
(index.html) module"]
pub struct CORE_0_MESSAGE_PHASE_SPEC;
impl crate::RegisterSpec for CORE_0_MESSAGE_PHASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_message_phase::R]
(R) reader structure"]
impl crate::Readable for CORE_0_MESSAGE_PHASE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets Core_0_MESSAGE_PHASE to value 0"]
impl crate::Resettable for CORE_0_MESSAGE_PHASE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
