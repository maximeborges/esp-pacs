#[doc = "Register `INT_ST_TIMERS` reader"]
pub struct R(crate::R<INT_ST_TIMERS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ST_TIMERS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_ST_TIMERS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_ST_TIMERS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `T0_INT_ST` reader - The masked interrupt status bit for the TIMG_T0_INT interrupt."]
pub struct T0_INT_ST_R(crate::FieldReader<bool, bool>);
impl T0_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        T0_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T0_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T1_INT_ST` reader - The masked interrupt status bit for the TIMG_T1_INT interrupt."]
pub struct T1_INT_ST_R(crate::FieldReader<bool, bool>);
impl T1_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        T1_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T1_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDT_INT_ST` reader - The masked interrupt status bit for the TIMG_WDT_INT interrupt."]
pub struct WDT_INT_ST_R(crate::FieldReader<bool, bool>);
impl WDT_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WDT_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDT_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - The masked interrupt status bit for the TIMG_T0_INT interrupt."]
    #[inline(always)]
    pub fn t0_int_st(&self) -> T0_INT_ST_R {
        T0_INT_ST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - The masked interrupt status bit for the TIMG_T1_INT interrupt."]
    #[inline(always)]
    pub fn t1_int_st(&self) -> T1_INT_ST_R {
        T1_INT_ST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - The masked interrupt status bit for the TIMG_WDT_INT interrupt."]
    #[inline(always)]
    pub fn wdt_int_st(&self) -> WDT_INT_ST_R {
        WDT_INT_ST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
#[doc = "Masked interrupt status\n\nThis register you can [`read`]
(crate::generic::Reg::read). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_st_timers]
(index.html) module"]
pub struct INT_ST_TIMERS_SPEC;
impl crate::RegisterSpec for INT_ST_TIMERS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_st_timers::R]
(R) reader structure"]
impl crate::Readable for INT_ST_TIMERS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INT_ST_TIMERS to value 0"]
impl crate::Resettable for INT_ST_TIMERS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
