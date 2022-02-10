#[doc = "Register `SAR_COCPU_INT_RAW` reader"]
pub struct R(crate::R<SAR_COCPU_INT_RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_COCPU_INT_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_COCPU_INT_RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_COCPU_INT_RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COCPU_TOUCH_DONE_INT_RAW` reader - TOUCH_DONE_INT interrupt raw bit"]
pub struct COCPU_TOUCH_DONE_INT_RAW_R(crate::FieldReader<bool, bool>);
impl COCPU_TOUCH_DONE_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COCPU_TOUCH_DONE_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COCPU_TOUCH_DONE_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COCPU_TOUCH_INACTIVE_INT_RAW` reader - TOUCH_INACTIVE_INT interrupt raw bit"]
pub struct COCPU_TOUCH_INACTIVE_INT_RAW_R(crate::FieldReader<bool, bool>);
impl COCPU_TOUCH_INACTIVE_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COCPU_TOUCH_INACTIVE_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COCPU_TOUCH_INACTIVE_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COCPU_TOUCH_ACTIVE_INT_RAW` reader - TOUCH_ACTIVE_INT interrupt raw bit"]
pub struct COCPU_TOUCH_ACTIVE_INT_RAW_R(crate::FieldReader<bool, bool>);
impl COCPU_TOUCH_ACTIVE_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COCPU_TOUCH_ACTIVE_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COCPU_TOUCH_ACTIVE_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COCPU_SARADC1_INT_RAW` reader - SARADC1_DONE_INT interrupt raw bit"]
pub struct COCPU_SARADC1_INT_RAW_R(crate::FieldReader<bool, bool>);
impl COCPU_SARADC1_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COCPU_SARADC1_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COCPU_SARADC1_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COCPU_SARADC2_INT_RAW` reader - SARADC2_DONE_INT interrupt raw bit"]
pub struct COCPU_SARADC2_INT_RAW_R(crate::FieldReader<bool, bool>);
impl COCPU_SARADC2_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COCPU_SARADC2_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COCPU_SARADC2_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COCPU_TSENS_INT_RAW` reader - TSENS_DONE_INT interrupt raw bit"]
pub struct COCPU_TSENS_INT_RAW_R(crate::FieldReader<bool, bool>);
impl COCPU_TSENS_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COCPU_TSENS_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COCPU_TSENS_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COCPU_START_INT_RAW` reader - RISCV_START_INT interrupt raw bit"]
pub struct COCPU_START_INT_RAW_R(crate::FieldReader<bool, bool>);
impl COCPU_START_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COCPU_START_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COCPU_START_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COCPU_SW_INT_RAW` reader - SW_INT interrupt raw bit"]
pub struct COCPU_SW_INT_RAW_R(crate::FieldReader<bool, bool>);
impl COCPU_SW_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COCPU_SW_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COCPU_SW_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COCPU_SWD_INT_RAW` reader - SWD_INT interrupt raw bit"]
pub struct COCPU_SWD_INT_RAW_R(crate::FieldReader<bool, bool>);
impl COCPU_SWD_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COCPU_SWD_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COCPU_SWD_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - TOUCH_DONE_INT interrupt raw bit"]
    #[inline(always)]
    pub fn cocpu_touch_done_int_raw(&self) -> COCPU_TOUCH_DONE_INT_RAW_R {
        COCPU_TOUCH_DONE_INT_RAW_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TOUCH_INACTIVE_INT interrupt raw bit"]
    #[inline(always)]
    pub fn cocpu_touch_inactive_int_raw(&self) -> COCPU_TOUCH_INACTIVE_INT_RAW_R {
        COCPU_TOUCH_INACTIVE_INT_RAW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TOUCH_ACTIVE_INT interrupt raw bit"]
    #[inline(always)]
    pub fn cocpu_touch_active_int_raw(&self) -> COCPU_TOUCH_ACTIVE_INT_RAW_R {
        COCPU_TOUCH_ACTIVE_INT_RAW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SARADC1_DONE_INT interrupt raw bit"]
    #[inline(always)]
    pub fn cocpu_saradc1_int_raw(&self) -> COCPU_SARADC1_INT_RAW_R {
        COCPU_SARADC1_INT_RAW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SARADC2_DONE_INT interrupt raw bit"]
    #[inline(always)]
    pub fn cocpu_saradc2_int_raw(&self) -> COCPU_SARADC2_INT_RAW_R {
        COCPU_SARADC2_INT_RAW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TSENS_DONE_INT interrupt raw bit"]
    #[inline(always)]
    pub fn cocpu_tsens_int_raw(&self) -> COCPU_TSENS_INT_RAW_R {
        COCPU_TSENS_INT_RAW_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RISCV_START_INT interrupt raw bit"]
    #[inline(always)]
    pub fn cocpu_start_int_raw(&self) -> COCPU_START_INT_RAW_R {
        COCPU_START_INT_RAW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - SW_INT interrupt raw bit"]
    #[inline(always)]
    pub fn cocpu_sw_int_raw(&self) -> COCPU_SW_INT_RAW_R {
        COCPU_SW_INT_RAW_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SWD_INT interrupt raw bit"]
    #[inline(always)]
    pub fn cocpu_swd_int_raw(&self) -> COCPU_SWD_INT_RAW_R {
        COCPU_SWD_INT_RAW_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
#[doc = "Interrupt raw bit of ULP-RISCV\n\nThis register you can [`read`]
(crate::generic::Reg::read). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_cocpu_int_raw]
(index.html) module"]
pub struct SAR_COCPU_INT_RAW_SPEC;
impl crate::RegisterSpec for SAR_COCPU_INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_cocpu_int_raw::R]
(R) reader structure"]
impl crate::Readable for SAR_COCPU_INT_RAW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SAR_COCPU_INT_RAW to value 0"]
impl crate::Resettable for SAR_COCPU_INT_RAW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
