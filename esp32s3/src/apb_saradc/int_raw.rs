#[doc = "Register `INT_RAW` reader"]
pub struct R(crate::R<INT_RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `THRES1_LOW_INT_RAW` reader - interrupt of thres1 low"]
pub struct THRES1_LOW_INT_RAW_R(crate::FieldReader<bool, bool>);
impl THRES1_LOW_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        THRES1_LOW_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THRES1_LOW_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THRES0_LOW_INT_RAW` reader - interrupt of thres0 low"]
pub struct THRES0_LOW_INT_RAW_R(crate::FieldReader<bool, bool>);
impl THRES0_LOW_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        THRES0_LOW_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THRES0_LOW_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THRES1_HIGH_INT_RAW` reader - interrupt of thres1 high"]
pub struct THRES1_HIGH_INT_RAW_R(crate::FieldReader<bool, bool>);
impl THRES1_HIGH_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        THRES1_HIGH_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THRES1_HIGH_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THRES0_HIGH_INT_RAW` reader - interrupt of thres0 high"]
pub struct THRES0_HIGH_INT_RAW_R(crate::FieldReader<bool, bool>);
impl THRES0_HIGH_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        THRES0_HIGH_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THRES0_HIGH_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB_SARADC2_DONE_INT_RAW` reader - interrupt of sar2 done"]
pub struct APB_SARADC2_DONE_INT_RAW_R(crate::FieldReader<bool, bool>);
impl APB_SARADC2_DONE_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APB_SARADC2_DONE_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB_SARADC2_DONE_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB_SARADC1_DONE_INT_RAW` reader - interrupt of sar1 done"]
pub struct APB_SARADC1_DONE_INT_RAW_R(crate::FieldReader<bool, bool>);
impl APB_SARADC1_DONE_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APB_SARADC1_DONE_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB_SARADC1_DONE_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 26 - interrupt of thres1 low"]
    #[inline(always)]
    pub fn thres1_low_int_raw(&self) -> THRES1_LOW_INT_RAW_R {
        THRES1_LOW_INT_RAW_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - interrupt of thres0 low"]
    #[inline(always)]
    pub fn thres0_low_int_raw(&self) -> THRES0_LOW_INT_RAW_R {
        THRES0_LOW_INT_RAW_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - interrupt of thres1 high"]
    #[inline(always)]
    pub fn thres1_high_int_raw(&self) -> THRES1_HIGH_INT_RAW_R {
        THRES1_HIGH_INT_RAW_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - interrupt of thres0 high"]
    #[inline(always)]
    pub fn thres0_high_int_raw(&self) -> THRES0_HIGH_INT_RAW_R {
        THRES0_HIGH_INT_RAW_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - interrupt of sar2 done"]
    #[inline(always)]
    pub fn apb_saradc2_done_int_raw(&self) -> APB_SARADC2_DONE_INT_RAW_R {
        APB_SARADC2_DONE_INT_RAW_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - interrupt of sar1 done"]
    #[inline(always)]
    pub fn apb_saradc1_done_int_raw(&self) -> APB_SARADC1_DONE_INT_RAW_R {
        APB_SARADC1_DONE_INT_RAW_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "raw of interrupt\n\nThis register you can [`read`]
(crate::generic::Reg::read). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_raw]
(index.html) module"]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_raw::R]
(R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
