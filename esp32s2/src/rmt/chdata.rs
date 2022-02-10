#[doc = "Register `CH%sDATA` reader"]
pub struct R(crate::R<CHDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CH_DATA` reader - The read and write data register for CHANNEL%s by apb fifo access."]
pub struct CH_DATA_R(crate::FieldReader<u32, u32>);
impl CH_DATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CH_DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH_DATA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - The read and write data register for CHANNEL%s by apb fifo access."]
    #[inline(always)]
    pub fn ch_data(&self) -> CH_DATA_R {
        CH_DATA_R::new(self.bits)
    }
}
#[doc = "The read and write data register for CHANNEL%s by apb fifo access.\n\nThis register you can [`read`]
(crate::generic::Reg::read). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chdata]
(index.html) module"]
pub struct CHDATA_SPEC;
impl crate::RegisterSpec for CHDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chdata::R]
(R) reader structure"]
impl crate::Readable for CHDATA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CH%sDATA to value 0"]
impl crate::Resettable for CHDATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
