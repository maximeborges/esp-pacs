#[doc = "Register `INT_ENA` reader"]
pub struct R(crate::R<INT_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_ENA` writer"]
pub struct W(crate::W<INT_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<INT_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER0_OVF_INT_ENA` reader - The interrupt enable bit for the LEDC_TIMER0_OVF_INT interrupt."]
pub struct TIMER0_OVF_INT_ENA_R(crate::FieldReader<bool, bool>);
impl TIMER0_OVF_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER0_OVF_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER0_OVF_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER0_OVF_INT_ENA` writer - The interrupt enable bit for the LEDC_TIMER0_OVF_INT interrupt."]
pub struct TIMER0_OVF_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER0_OVF_INT_ENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
#[doc = "Field `TIMER1_OVF_INT_ENA` reader - The interrupt enable bit for the LEDC_TIMER1_OVF_INT interrupt."]
pub struct TIMER1_OVF_INT_ENA_R(crate::FieldReader<bool, bool>);
impl TIMER1_OVF_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER1_OVF_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER1_OVF_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER1_OVF_INT_ENA` writer - The interrupt enable bit for the LEDC_TIMER1_OVF_INT interrupt."]
pub struct TIMER1_OVF_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER1_OVF_INT_ENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u32 & 1) << 1);
        self.w
    }
}
#[doc = "Field `TIMER2_OVF_INT_ENA` reader - The interrupt enable bit for the LEDC_TIMER2_OVF_INT interrupt."]
pub struct TIMER2_OVF_INT_ENA_R(crate::FieldReader<bool, bool>);
impl TIMER2_OVF_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER2_OVF_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER2_OVF_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER2_OVF_INT_ENA` writer - The interrupt enable bit for the LEDC_TIMER2_OVF_INT interrupt."]
pub struct TIMER2_OVF_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER2_OVF_INT_ENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u32 & 1) << 2);
        self.w
    }
}
#[doc = "Field `TIMER3_OVF_INT_ENA` reader - The interrupt enable bit for the LEDC_TIMER3_OVF_INT interrupt."]
pub struct TIMER3_OVF_INT_ENA_R(crate::FieldReader<bool, bool>);
impl TIMER3_OVF_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER3_OVF_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER3_OVF_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER3_OVF_INT_ENA` writer - The interrupt enable bit for the LEDC_TIMER3_OVF_INT interrupt."]
pub struct TIMER3_OVF_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER3_OVF_INT_ENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u32 & 1) << 3);
        self.w
    }
}
#[doc = "Field `DUTY_CHNG_END_CH0_INT_ENA` reader - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH0_INT interrupt."]
pub struct DUTY_CHNG_END_CH0_INT_ENA_R(crate::FieldReader<bool, bool>);
impl DUTY_CHNG_END_CH0_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DUTY_CHNG_END_CH0_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_CHNG_END_CH0_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY_CHNG_END_CH0_INT_ENA` writer - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH0_INT interrupt."]
pub struct DUTY_CHNG_END_CH0_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_CHNG_END_CH0_INT_ENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u32 & 1) << 4);
        self.w
    }
}
#[doc = "Field `DUTY_CHNG_END_CH1_INT_ENA` reader - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH1_INT interrupt."]
pub struct DUTY_CHNG_END_CH1_INT_ENA_R(crate::FieldReader<bool, bool>);
impl DUTY_CHNG_END_CH1_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DUTY_CHNG_END_CH1_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_CHNG_END_CH1_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY_CHNG_END_CH1_INT_ENA` writer - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH1_INT interrupt."]
pub struct DUTY_CHNG_END_CH1_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_CHNG_END_CH1_INT_ENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u32 & 1) << 5);
        self.w
    }
}
#[doc = "Field `DUTY_CHNG_END_CH2_INT_ENA` reader - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH2_INT interrupt."]
pub struct DUTY_CHNG_END_CH2_INT_ENA_R(crate::FieldReader<bool, bool>);
impl DUTY_CHNG_END_CH2_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DUTY_CHNG_END_CH2_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_CHNG_END_CH2_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY_CHNG_END_CH2_INT_ENA` writer - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH2_INT interrupt."]
pub struct DUTY_CHNG_END_CH2_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_CHNG_END_CH2_INT_ENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 6)) | ((value as u32 & 1) << 6);
        self.w
    }
}
#[doc = "Field `DUTY_CHNG_END_CH3_INT_ENA` reader - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH3_INT interrupt."]
pub struct DUTY_CHNG_END_CH3_INT_ENA_R(crate::FieldReader<bool, bool>);
impl DUTY_CHNG_END_CH3_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DUTY_CHNG_END_CH3_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_CHNG_END_CH3_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY_CHNG_END_CH3_INT_ENA` writer - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH3_INT interrupt."]
pub struct DUTY_CHNG_END_CH3_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_CHNG_END_CH3_INT_ENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 7)) | ((value as u32 & 1) << 7);
        self.w
    }
}
#[doc = "Field `DUTY_CHNG_END_CH4_INT_ENA` reader - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH4_INT interrupt."]
pub struct DUTY_CHNG_END_CH4_INT_ENA_R(crate::FieldReader<bool, bool>);
impl DUTY_CHNG_END_CH4_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DUTY_CHNG_END_CH4_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_CHNG_END_CH4_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY_CHNG_END_CH4_INT_ENA` writer - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH4_INT interrupt."]
pub struct DUTY_CHNG_END_CH4_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_CHNG_END_CH4_INT_ENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 8)) | ((value as u32 & 1) << 8);
        self.w
    }
}
#[doc = "Field `DUTY_CHNG_END_CH5_INT_ENA` reader - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH5_INT interrupt."]
pub struct DUTY_CHNG_END_CH5_INT_ENA_R(crate::FieldReader<bool, bool>);
impl DUTY_CHNG_END_CH5_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DUTY_CHNG_END_CH5_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_CHNG_END_CH5_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY_CHNG_END_CH5_INT_ENA` writer - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH5_INT interrupt."]
pub struct DUTY_CHNG_END_CH5_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_CHNG_END_CH5_INT_ENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 9)) | ((value as u32 & 1) << 9);
        self.w
    }
}
#[doc = "Field `DUTY_CHNG_END_CH6_INT_ENA` reader - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH6_INT interrupt."]
pub struct DUTY_CHNG_END_CH6_INT_ENA_R(crate::FieldReader<bool, bool>);
impl DUTY_CHNG_END_CH6_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DUTY_CHNG_END_CH6_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_CHNG_END_CH6_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY_CHNG_END_CH6_INT_ENA` writer - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH6_INT interrupt."]
pub struct DUTY_CHNG_END_CH6_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_CHNG_END_CH6_INT_ENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 10)) | ((value as u32 & 1) << 10);
        self.w
    }
}
#[doc = "Field `DUTY_CHNG_END_CH7_INT_ENA` reader - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH7_INT interrupt."]
pub struct DUTY_CHNG_END_CH7_INT_ENA_R(crate::FieldReader<bool, bool>);
impl DUTY_CHNG_END_CH7_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DUTY_CHNG_END_CH7_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_CHNG_END_CH7_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY_CHNG_END_CH7_INT_ENA` writer - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH7_INT interrupt."]
pub struct DUTY_CHNG_END_CH7_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_CHNG_END_CH7_INT_ENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 11)) | ((value as u32 & 1) << 11);
        self.w
    }
}
#[doc = "Field `OVF_CNT_CH0_INT_ENA` reader - The interrupt enable bit for the LEDC_OVF_CNT_CH0_INT interrupt."]
pub struct OVF_CNT_CH0_INT_ENA_R(crate::FieldReader<bool, bool>);
impl OVF_CNT_CH0_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVF_CNT_CH0_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVF_CNT_CH0_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVF_CNT_CH0_INT_ENA` writer - The interrupt enable bit for the LEDC_OVF_CNT_CH0_INT interrupt."]
pub struct OVF_CNT_CH0_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> OVF_CNT_CH0_INT_ENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 12)) | ((value as u32 & 1) << 12);
        self.w
    }
}
#[doc = "Field `OVF_CNT_CH1_INT_ENA` reader - The interrupt enable bit for the LEDC_OVF_CNT_CH1_INT interrupt."]
pub struct OVF_CNT_CH1_INT_ENA_R(crate::FieldReader<bool, bool>);
impl OVF_CNT_CH1_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVF_CNT_CH1_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVF_CNT_CH1_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVF_CNT_CH1_INT_ENA` writer - The interrupt enable bit for the LEDC_OVF_CNT_CH1_INT interrupt."]
pub struct OVF_CNT_CH1_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> OVF_CNT_CH1_INT_ENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 13)) | ((value as u32 & 1) << 13);
        self.w
    }
}
#[doc = "Field `OVF_CNT_CH2_INT_ENA` reader - The interrupt enable bit for the LEDC_OVF_CNT_CH2_INT interrupt."]
pub struct OVF_CNT_CH2_INT_ENA_R(crate::FieldReader<bool, bool>);
impl OVF_CNT_CH2_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVF_CNT_CH2_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVF_CNT_CH2_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVF_CNT_CH2_INT_ENA` writer - The interrupt enable bit for the LEDC_OVF_CNT_CH2_INT interrupt."]
pub struct OVF_CNT_CH2_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> OVF_CNT_CH2_INT_ENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 14)) | ((value as u32 & 1) << 14);
        self.w
    }
}
#[doc = "Field `OVF_CNT_CH3_INT_ENA` reader - The interrupt enable bit for the LEDC_OVF_CNT_CH3_INT interrupt."]
pub struct OVF_CNT_CH3_INT_ENA_R(crate::FieldReader<bool, bool>);
impl OVF_CNT_CH3_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVF_CNT_CH3_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVF_CNT_CH3_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVF_CNT_CH3_INT_ENA` writer - The interrupt enable bit for the LEDC_OVF_CNT_CH3_INT interrupt."]
pub struct OVF_CNT_CH3_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> OVF_CNT_CH3_INT_ENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 15)) | ((value as u32 & 1) << 15);
        self.w
    }
}
#[doc = "Field `OVF_CNT_CH4_INT_ENA` reader - The interrupt enable bit for the LEDC_OVF_CNT_CH4_INT interrupt."]
pub struct OVF_CNT_CH4_INT_ENA_R(crate::FieldReader<bool, bool>);
impl OVF_CNT_CH4_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVF_CNT_CH4_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVF_CNT_CH4_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVF_CNT_CH4_INT_ENA` writer - The interrupt enable bit for the LEDC_OVF_CNT_CH4_INT interrupt."]
pub struct OVF_CNT_CH4_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> OVF_CNT_CH4_INT_ENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 16)) | ((value as u32 & 1) << 16);
        self.w
    }
}
#[doc = "Field `OVF_CNT_CH5_INT_ENA` reader - The interrupt enable bit for the LEDC_OVF_CNT_CH5_INT interrupt."]
pub struct OVF_CNT_CH5_INT_ENA_R(crate::FieldReader<bool, bool>);
impl OVF_CNT_CH5_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVF_CNT_CH5_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVF_CNT_CH5_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVF_CNT_CH5_INT_ENA` writer - The interrupt enable bit for the LEDC_OVF_CNT_CH5_INT interrupt."]
pub struct OVF_CNT_CH5_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> OVF_CNT_CH5_INT_ENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 17)) | ((value as u32 & 1) << 17);
        self.w
    }
}
#[doc = "Field `OVF_CNT_CH6_INT_ENA` reader - The interrupt enable bit for the LEDC_OVF_CNT_CH6_INT interrupt."]
pub struct OVF_CNT_CH6_INT_ENA_R(crate::FieldReader<bool, bool>);
impl OVF_CNT_CH6_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVF_CNT_CH6_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVF_CNT_CH6_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVF_CNT_CH6_INT_ENA` writer - The interrupt enable bit for the LEDC_OVF_CNT_CH6_INT interrupt."]
pub struct OVF_CNT_CH6_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> OVF_CNT_CH6_INT_ENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 18)) | ((value as u32 & 1) << 18);
        self.w
    }
}
#[doc = "Field `OVF_CNT_CH7_INT_ENA` reader - The interrupt enable bit for the LEDC_OVF_CNT_CH7_INT interrupt."]
pub struct OVF_CNT_CH7_INT_ENA_R(crate::FieldReader<bool, bool>);
impl OVF_CNT_CH7_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVF_CNT_CH7_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVF_CNT_CH7_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVF_CNT_CH7_INT_ENA` writer - The interrupt enable bit for the LEDC_OVF_CNT_CH7_INT interrupt."]
pub struct OVF_CNT_CH7_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> OVF_CNT_CH7_INT_ENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 19)) | ((value as u32 & 1) << 19);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - The interrupt enable bit for the LEDC_TIMER0_OVF_INT interrupt."]
    #[inline(always)]
    pub fn timer0_ovf_int_ena(&self) -> TIMER0_OVF_INT_ENA_R {
        TIMER0_OVF_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for the LEDC_TIMER1_OVF_INT interrupt."]
    #[inline(always)]
    pub fn timer1_ovf_int_ena(&self) -> TIMER1_OVF_INT_ENA_R {
        TIMER1_OVF_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The interrupt enable bit for the LEDC_TIMER2_OVF_INT interrupt."]
    #[inline(always)]
    pub fn timer2_ovf_int_ena(&self) -> TIMER2_OVF_INT_ENA_R {
        TIMER2_OVF_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The interrupt enable bit for the LEDC_TIMER3_OVF_INT interrupt."]
    #[inline(always)]
    pub fn timer3_ovf_int_ena(&self) -> TIMER3_OVF_INT_ENA_R {
        TIMER3_OVF_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH0_INT interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_ch0_int_ena(&self) -> DUTY_CHNG_END_CH0_INT_ENA_R {
        DUTY_CHNG_END_CH0_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH1_INT interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_ch1_int_ena(&self) -> DUTY_CHNG_END_CH1_INT_ENA_R {
        DUTY_CHNG_END_CH1_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH2_INT interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_ch2_int_ena(&self) -> DUTY_CHNG_END_CH2_INT_ENA_R {
        DUTY_CHNG_END_CH2_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH3_INT interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_ch3_int_ena(&self) -> DUTY_CHNG_END_CH3_INT_ENA_R {
        DUTY_CHNG_END_CH3_INT_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH4_INT interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_ch4_int_ena(&self) -> DUTY_CHNG_END_CH4_INT_ENA_R {
        DUTY_CHNG_END_CH4_INT_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH5_INT interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_ch5_int_ena(&self) -> DUTY_CHNG_END_CH5_INT_ENA_R {
        DUTY_CHNG_END_CH5_INT_ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH6_INT interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_ch6_int_ena(&self) -> DUTY_CHNG_END_CH6_INT_ENA_R {
        DUTY_CHNG_END_CH6_INT_ENA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH7_INT interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_ch7_int_ena(&self) -> DUTY_CHNG_END_CH7_INT_ENA_R {
        DUTY_CHNG_END_CH7_INT_ENA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The interrupt enable bit for the LEDC_OVF_CNT_CH0_INT interrupt."]
    #[inline(always)]
    pub fn ovf_cnt_ch0_int_ena(&self) -> OVF_CNT_CH0_INT_ENA_R {
        OVF_CNT_CH0_INT_ENA_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The interrupt enable bit for the LEDC_OVF_CNT_CH1_INT interrupt."]
    #[inline(always)]
    pub fn ovf_cnt_ch1_int_ena(&self) -> OVF_CNT_CH1_INT_ENA_R {
        OVF_CNT_CH1_INT_ENA_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The interrupt enable bit for the LEDC_OVF_CNT_CH2_INT interrupt."]
    #[inline(always)]
    pub fn ovf_cnt_ch2_int_ena(&self) -> OVF_CNT_CH2_INT_ENA_R {
        OVF_CNT_CH2_INT_ENA_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The interrupt enable bit for the LEDC_OVF_CNT_CH3_INT interrupt."]
    #[inline(always)]
    pub fn ovf_cnt_ch3_int_ena(&self) -> OVF_CNT_CH3_INT_ENA_R {
        OVF_CNT_CH3_INT_ENA_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - The interrupt enable bit for the LEDC_OVF_CNT_CH4_INT interrupt."]
    #[inline(always)]
    pub fn ovf_cnt_ch4_int_ena(&self) -> OVF_CNT_CH4_INT_ENA_R {
        OVF_CNT_CH4_INT_ENA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The interrupt enable bit for the LEDC_OVF_CNT_CH5_INT interrupt."]
    #[inline(always)]
    pub fn ovf_cnt_ch5_int_ena(&self) -> OVF_CNT_CH5_INT_ENA_R {
        OVF_CNT_CH5_INT_ENA_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - The interrupt enable bit for the LEDC_OVF_CNT_CH6_INT interrupt."]
    #[inline(always)]
    pub fn ovf_cnt_ch6_int_ena(&self) -> OVF_CNT_CH6_INT_ENA_R {
        OVF_CNT_CH6_INT_ENA_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The interrupt enable bit for the LEDC_OVF_CNT_CH7_INT interrupt."]
    #[inline(always)]
    pub fn ovf_cnt_ch7_int_ena(&self) -> OVF_CNT_CH7_INT_ENA_R {
        OVF_CNT_CH7_INT_ENA_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The interrupt enable bit for the LEDC_TIMER0_OVF_INT interrupt."]
    #[inline(always)]
    pub fn timer0_ovf_int_ena(&mut self) -> TIMER0_OVF_INT_ENA_W {
        TIMER0_OVF_INT_ENA_W { w: self }
    }
    #[doc = "Bit 1 - The interrupt enable bit for the LEDC_TIMER1_OVF_INT interrupt."]
    #[inline(always)]
    pub fn timer1_ovf_int_ena(&mut self) -> TIMER1_OVF_INT_ENA_W {
        TIMER1_OVF_INT_ENA_W { w: self }
    }
    #[doc = "Bit 2 - The interrupt enable bit for the LEDC_TIMER2_OVF_INT interrupt."]
    #[inline(always)]
    pub fn timer2_ovf_int_ena(&mut self) -> TIMER2_OVF_INT_ENA_W {
        TIMER2_OVF_INT_ENA_W { w: self }
    }
    #[doc = "Bit 3 - The interrupt enable bit for the LEDC_TIMER3_OVF_INT interrupt."]
    #[inline(always)]
    pub fn timer3_ovf_int_ena(&mut self) -> TIMER3_OVF_INT_ENA_W {
        TIMER3_OVF_INT_ENA_W { w: self }
    }
    #[doc = "Bit 4 - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH0_INT interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_ch0_int_ena(&mut self) -> DUTY_CHNG_END_CH0_INT_ENA_W {
        DUTY_CHNG_END_CH0_INT_ENA_W { w: self }
    }
    #[doc = "Bit 5 - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH1_INT interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_ch1_int_ena(&mut self) -> DUTY_CHNG_END_CH1_INT_ENA_W {
        DUTY_CHNG_END_CH1_INT_ENA_W { w: self }
    }
    #[doc = "Bit 6 - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH2_INT interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_ch2_int_ena(&mut self) -> DUTY_CHNG_END_CH2_INT_ENA_W {
        DUTY_CHNG_END_CH2_INT_ENA_W { w: self }
    }
    #[doc = "Bit 7 - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH3_INT interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_ch3_int_ena(&mut self) -> DUTY_CHNG_END_CH3_INT_ENA_W {
        DUTY_CHNG_END_CH3_INT_ENA_W { w: self }
    }
    #[doc = "Bit 8 - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH4_INT interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_ch4_int_ena(&mut self) -> DUTY_CHNG_END_CH4_INT_ENA_W {
        DUTY_CHNG_END_CH4_INT_ENA_W { w: self }
    }
    #[doc = "Bit 9 - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH5_INT interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_ch5_int_ena(&mut self) -> DUTY_CHNG_END_CH5_INT_ENA_W {
        DUTY_CHNG_END_CH5_INT_ENA_W { w: self }
    }
    #[doc = "Bit 10 - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH6_INT interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_ch6_int_ena(&mut self) -> DUTY_CHNG_END_CH6_INT_ENA_W {
        DUTY_CHNG_END_CH6_INT_ENA_W { w: self }
    }
    #[doc = "Bit 11 - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH7_INT interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_ch7_int_ena(&mut self) -> DUTY_CHNG_END_CH7_INT_ENA_W {
        DUTY_CHNG_END_CH7_INT_ENA_W { w: self }
    }
    #[doc = "Bit 12 - The interrupt enable bit for the LEDC_OVF_CNT_CH0_INT interrupt."]
    #[inline(always)]
    pub fn ovf_cnt_ch0_int_ena(&mut self) -> OVF_CNT_CH0_INT_ENA_W {
        OVF_CNT_CH0_INT_ENA_W { w: self }
    }
    #[doc = "Bit 13 - The interrupt enable bit for the LEDC_OVF_CNT_CH1_INT interrupt."]
    #[inline(always)]
    pub fn ovf_cnt_ch1_int_ena(&mut self) -> OVF_CNT_CH1_INT_ENA_W {
        OVF_CNT_CH1_INT_ENA_W { w: self }
    }
    #[doc = "Bit 14 - The interrupt enable bit for the LEDC_OVF_CNT_CH2_INT interrupt."]
    #[inline(always)]
    pub fn ovf_cnt_ch2_int_ena(&mut self) -> OVF_CNT_CH2_INT_ENA_W {
        OVF_CNT_CH2_INT_ENA_W { w: self }
    }
    #[doc = "Bit 15 - The interrupt enable bit for the LEDC_OVF_CNT_CH3_INT interrupt."]
    #[inline(always)]
    pub fn ovf_cnt_ch3_int_ena(&mut self) -> OVF_CNT_CH3_INT_ENA_W {
        OVF_CNT_CH3_INT_ENA_W { w: self }
    }
    #[doc = "Bit 16 - The interrupt enable bit for the LEDC_OVF_CNT_CH4_INT interrupt."]
    #[inline(always)]
    pub fn ovf_cnt_ch4_int_ena(&mut self) -> OVF_CNT_CH4_INT_ENA_W {
        OVF_CNT_CH4_INT_ENA_W { w: self }
    }
    #[doc = "Bit 17 - The interrupt enable bit for the LEDC_OVF_CNT_CH5_INT interrupt."]
    #[inline(always)]
    pub fn ovf_cnt_ch5_int_ena(&mut self) -> OVF_CNT_CH5_INT_ENA_W {
        OVF_CNT_CH5_INT_ENA_W { w: self }
    }
    #[doc = "Bit 18 - The interrupt enable bit for the LEDC_OVF_CNT_CH6_INT interrupt."]
    #[inline(always)]
    pub fn ovf_cnt_ch6_int_ena(&mut self) -> OVF_CNT_CH6_INT_ENA_W {
        OVF_CNT_CH6_INT_ENA_W { w: self }
    }
    #[doc = "Bit 19 - The interrupt enable bit for the LEDC_OVF_CNT_CH7_INT interrupt."]
    #[inline(always)]
    pub fn ovf_cnt_ch7_int_ena(&mut self) -> OVF_CNT_CH7_INT_ENA_W {
        OVF_CNT_CH7_INT_ENA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt enable bits\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_ena]
(index.html) module"]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_ena::R]
(R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_ena::W]
(W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
