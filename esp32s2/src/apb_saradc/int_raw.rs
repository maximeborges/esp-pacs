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
#[doc = "Field `ADC2_THRES_INT_RAW` reader - Raw bit of APB_SARADC_ADC2_THRES_INT interrupt."]
pub struct ADC2_THRES_INT_RAW_R(crate::FieldReader<bool>);
impl ADC2_THRES_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC2_THRES_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC2_THRES_INT_RAW_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC1_THRES_INT_RAW` reader - Raw bit of APB_SARADC_ADC1_THRES_INT interrupt."]
pub struct ADC1_THRES_INT_RAW_R(crate::FieldReader<bool>);
impl ADC1_THRES_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC1_THRES_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC1_THRES_INT_RAW_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC2_DONE_INT_RAW` reader - Raw bit of APB_SARADC_ADC2_DONE_INT interrupt."]
pub struct ADC2_DONE_INT_RAW_R(crate::FieldReader<bool>);
impl ADC2_DONE_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC2_DONE_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC2_DONE_INT_RAW_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC1_DONE_INT_RAW` reader - Raw bit of APB_SARADC_ADC1_DONE_INT interrupt."]
pub struct ADC1_DONE_INT_RAW_R(crate::FieldReader<bool>);
impl ADC1_DONE_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC1_DONE_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC1_DONE_INT_RAW_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 28 - Raw bit of APB_SARADC_ADC2_THRES_INT interrupt."]
    #[inline(always)]
    pub fn adc2_thres_int_raw(&self) -> ADC2_THRES_INT_RAW_R {
        ADC2_THRES_INT_RAW_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Raw bit of APB_SARADC_ADC1_THRES_INT interrupt."]
    #[inline(always)]
    pub fn adc1_thres_int_raw(&self) -> ADC1_THRES_INT_RAW_R {
        ADC1_THRES_INT_RAW_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Raw bit of APB_SARADC_ADC2_DONE_INT interrupt."]
    #[inline(always)]
    pub fn adc2_done_int_raw(&self) -> ADC2_DONE_INT_RAW_R {
        ADC2_DONE_INT_RAW_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Raw bit of APB_SARADC_ADC1_DONE_INT interrupt."]
    #[inline(always)]
    pub fn adc1_done_int_raw(&self) -> ADC1_DONE_INT_RAW_R {
        ADC1_DONE_INT_RAW_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "DIG ADC interrupt raw bits\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_raw](index.html) module"]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_raw::R](R) reader structure"]
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
