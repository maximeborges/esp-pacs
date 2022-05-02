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
#[doc = "Field `RXFIFO_WM_INT_ENA` reader - The interrupt enable bit for I2C_RXFIFO_WM_INT interrupt."]
pub struct RXFIFO_WM_INT_ENA_R(crate::FieldReader<bool>);
impl RXFIFO_WM_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXFIFO_WM_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFIFO_WM_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFIFO_WM_INT_ENA` writer - The interrupt enable bit for I2C_RXFIFO_WM_INT interrupt."]
pub struct RXFIFO_WM_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFO_WM_INT_ENA_W<'a> {
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
#[doc = "Field `TXFIFO_WM_INT_ENA` reader - The interrupt enable bit for I2C_TXFIFO_WM_INT interrupt."]
pub struct TXFIFO_WM_INT_ENA_R(crate::FieldReader<bool>);
impl TXFIFO_WM_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXFIFO_WM_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFIFO_WM_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFIFO_WM_INT_ENA` writer - The interrupt enable bit for I2C_TXFIFO_WM_INT interrupt."]
pub struct TXFIFO_WM_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFIFO_WM_INT_ENA_W<'a> {
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
#[doc = "Field `RXFIFO_OVF_INT_ENA` reader - The interrupt enable bit for I2C_RXFIFO_OVF_INT interrupt."]
pub struct RXFIFO_OVF_INT_ENA_R(crate::FieldReader<bool>);
impl RXFIFO_OVF_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXFIFO_OVF_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFIFO_OVF_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFIFO_OVF_INT_ENA` writer - The interrupt enable bit for I2C_RXFIFO_OVF_INT interrupt."]
pub struct RXFIFO_OVF_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFO_OVF_INT_ENA_W<'a> {
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
#[doc = "Field `END_DETECT_INT_ENA` reader - The interrupt enable bit for the I2C_END_DETECT_INT interrupt."]
pub struct END_DETECT_INT_ENA_R(crate::FieldReader<bool>);
impl END_DETECT_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        END_DETECT_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for END_DETECT_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `END_DETECT_INT_ENA` writer - The interrupt enable bit for the I2C_END_DETECT_INT interrupt."]
pub struct END_DETECT_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> END_DETECT_INT_ENA_W<'a> {
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
#[doc = "Field `BYTE_TRANS_DONE_INT_ENA` reader - The interrupt enable bit for the I2C_END_DETECT_INT interrupt."]
pub struct BYTE_TRANS_DONE_INT_ENA_R(crate::FieldReader<bool>);
impl BYTE_TRANS_DONE_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BYTE_TRANS_DONE_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTE_TRANS_DONE_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BYTE_TRANS_DONE_INT_ENA` writer - The interrupt enable bit for the I2C_END_DETECT_INT interrupt."]
pub struct BYTE_TRANS_DONE_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE_TRANS_DONE_INT_ENA_W<'a> {
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
#[doc = "Field `ARBITRATION_LOST_INT_ENA` reader - The interrupt enable bit for the I2C_ARBITRATION_LOST_INT interrupt."]
pub struct ARBITRATION_LOST_INT_ENA_R(crate::FieldReader<bool>);
impl ARBITRATION_LOST_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ARBITRATION_LOST_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARBITRATION_LOST_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARBITRATION_LOST_INT_ENA` writer - The interrupt enable bit for the I2C_ARBITRATION_LOST_INT interrupt."]
pub struct ARBITRATION_LOST_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> ARBITRATION_LOST_INT_ENA_W<'a> {
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
#[doc = "Field `MST_TXFIFO_UDF_INT_ENA` reader - The interrupt enable bit for I2C_TRANS_COMPLETE_INT interrupt."]
pub struct MST_TXFIFO_UDF_INT_ENA_R(crate::FieldReader<bool>);
impl MST_TXFIFO_UDF_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MST_TXFIFO_UDF_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MST_TXFIFO_UDF_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MST_TXFIFO_UDF_INT_ENA` writer - The interrupt enable bit for I2C_TRANS_COMPLETE_INT interrupt."]
pub struct MST_TXFIFO_UDF_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> MST_TXFIFO_UDF_INT_ENA_W<'a> {
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
#[doc = "Field `TRANS_COMPLETE_INT_ENA` reader - The interrupt enable bit for the I2C_TRANS_COMPLETE_INT interrupt."]
pub struct TRANS_COMPLETE_INT_ENA_R(crate::FieldReader<bool>);
impl TRANS_COMPLETE_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRANS_COMPLETE_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRANS_COMPLETE_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRANS_COMPLETE_INT_ENA` writer - The interrupt enable bit for the I2C_TRANS_COMPLETE_INT interrupt."]
pub struct TRANS_COMPLETE_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TRANS_COMPLETE_INT_ENA_W<'a> {
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
#[doc = "Field `TIME_OUT_INT_ENA` reader - The interrupt enable bit for the I2C_TIME_OUT_INT interrupt."]
pub struct TIME_OUT_INT_ENA_R(crate::FieldReader<bool>);
impl TIME_OUT_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIME_OUT_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIME_OUT_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIME_OUT_INT_ENA` writer - The interrupt enable bit for the I2C_TIME_OUT_INT interrupt."]
pub struct TIME_OUT_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TIME_OUT_INT_ENA_W<'a> {
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
#[doc = "Field `TRANS_START_INT_ENA` reader - The interrupt enable bit for the I2C_TRANS_START_INT interrupt."]
pub struct TRANS_START_INT_ENA_R(crate::FieldReader<bool>);
impl TRANS_START_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRANS_START_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRANS_START_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRANS_START_INT_ENA` writer - The interrupt enable bit for the I2C_TRANS_START_INT interrupt."]
pub struct TRANS_START_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TRANS_START_INT_ENA_W<'a> {
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
#[doc = "Field `NACK_INT_ENA` reader - The interrupt enable bit for I2C_SLAVE_STRETCH_INT interrupt."]
pub struct NACK_INT_ENA_R(crate::FieldReader<bool>);
impl NACK_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NACK_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NACK_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NACK_INT_ENA` writer - The interrupt enable bit for I2C_SLAVE_STRETCH_INT interrupt."]
pub struct NACK_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> NACK_INT_ENA_W<'a> {
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
#[doc = "Field `TXFIFO_OVF_INT_ENA` reader - The interrupt enable bit for I2C_TXFIFO_OVF_INT interrupt."]
pub struct TXFIFO_OVF_INT_ENA_R(crate::FieldReader<bool>);
impl TXFIFO_OVF_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXFIFO_OVF_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFIFO_OVF_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFIFO_OVF_INT_ENA` writer - The interrupt enable bit for I2C_TXFIFO_OVF_INT interrupt."]
pub struct TXFIFO_OVF_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFIFO_OVF_INT_ENA_W<'a> {
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
#[doc = "Field `RXFIFO_UDF_INT_ENA` reader - The interrupt enable bit for I2C_RXFIFO_UDF_INT interrupt."]
pub struct RXFIFO_UDF_INT_ENA_R(crate::FieldReader<bool>);
impl RXFIFO_UDF_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXFIFO_UDF_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFIFO_UDF_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFIFO_UDF_INT_ENA` writer - The interrupt enable bit for I2C_RXFIFO_UDF_INT interrupt."]
pub struct RXFIFO_UDF_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFO_UDF_INT_ENA_W<'a> {
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
#[doc = "Field `SCL_ST_TO_INT_ENA` reader - The interrupt enable bit for I2C_SCL_ST_TO_INT interrupt."]
pub struct SCL_ST_TO_INT_ENA_R(crate::FieldReader<bool>);
impl SCL_ST_TO_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCL_ST_TO_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCL_ST_TO_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCL_ST_TO_INT_ENA` writer - The interrupt enable bit for I2C_SCL_ST_TO_INT interrupt."]
pub struct SCL_ST_TO_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SCL_ST_TO_INT_ENA_W<'a> {
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
#[doc = "Field `SCL_MAIN_ST_TO_INT_ENA` reader - The interrupt enable bit for I2C_SCL_MAIN_ST_TO_INT interrupt."]
pub struct SCL_MAIN_ST_TO_INT_ENA_R(crate::FieldReader<bool>);
impl SCL_MAIN_ST_TO_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCL_MAIN_ST_TO_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCL_MAIN_ST_TO_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCL_MAIN_ST_TO_INT_ENA` writer - The interrupt enable bit for I2C_SCL_MAIN_ST_TO_INT interrupt."]
pub struct SCL_MAIN_ST_TO_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SCL_MAIN_ST_TO_INT_ENA_W<'a> {
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
#[doc = "Field `DET_START_INT_ENA` reader - The interrupt enable bit for I2C_DET_START_INT interrupt."]
pub struct DET_START_INT_ENA_R(crate::FieldReader<bool>);
impl DET_START_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DET_START_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DET_START_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DET_START_INT_ENA` writer - The interrupt enable bit for I2C_DET_START_INT interrupt."]
pub struct DET_START_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DET_START_INT_ENA_W<'a> {
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
#[doc = "Field `SLAVE_STRETCH_INT_ENA` reader - The interrupt enable bit for I2C_SLAVE_STRETCH_INT interrupt."]
pub struct SLAVE_STRETCH_INT_ENA_R(crate::FieldReader<bool>);
impl SLAVE_STRETCH_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLAVE_STRETCH_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLAVE_STRETCH_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLAVE_STRETCH_INT_ENA` writer - The interrupt enable bit for I2C_SLAVE_STRETCH_INT interrupt."]
pub struct SLAVE_STRETCH_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_STRETCH_INT_ENA_W<'a> {
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
#[doc = "Field `GENERAL_CALL_INT_ENA` reader - The interrupt enable bit for I2C_GENARAL_CALL_INT interrupt."]
pub struct GENERAL_CALL_INT_ENA_R(crate::FieldReader<bool>);
impl GENERAL_CALL_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GENERAL_CALL_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GENERAL_CALL_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GENERAL_CALL_INT_ENA` writer - The interrupt enable bit for I2C_GENARAL_CALL_INT interrupt."]
pub struct GENERAL_CALL_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> GENERAL_CALL_INT_ENA_W<'a> {
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
impl R {
    #[doc = "Bit 0 - The interrupt enable bit for I2C_RXFIFO_WM_INT interrupt."]
    #[inline(always)]
    pub fn rxfifo_wm_int_ena(&self) -> RXFIFO_WM_INT_ENA_R {
        RXFIFO_WM_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for I2C_TXFIFO_WM_INT interrupt."]
    #[inline(always)]
    pub fn txfifo_wm_int_ena(&self) -> TXFIFO_WM_INT_ENA_R {
        TXFIFO_WM_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The interrupt enable bit for I2C_RXFIFO_OVF_INT interrupt."]
    #[inline(always)]
    pub fn rxfifo_ovf_int_ena(&self) -> RXFIFO_OVF_INT_ENA_R {
        RXFIFO_OVF_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The interrupt enable bit for the I2C_END_DETECT_INT interrupt."]
    #[inline(always)]
    pub fn end_detect_int_ena(&self) -> END_DETECT_INT_ENA_R {
        END_DETECT_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The interrupt enable bit for the I2C_END_DETECT_INT interrupt."]
    #[inline(always)]
    pub fn byte_trans_done_int_ena(&self) -> BYTE_TRANS_DONE_INT_ENA_R {
        BYTE_TRANS_DONE_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The interrupt enable bit for the I2C_ARBITRATION_LOST_INT interrupt."]
    #[inline(always)]
    pub fn arbitration_lost_int_ena(&self) -> ARBITRATION_LOST_INT_ENA_R {
        ARBITRATION_LOST_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The interrupt enable bit for I2C_TRANS_COMPLETE_INT interrupt."]
    #[inline(always)]
    pub fn mst_txfifo_udf_int_ena(&self) -> MST_TXFIFO_UDF_INT_ENA_R {
        MST_TXFIFO_UDF_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The interrupt enable bit for the I2C_TRANS_COMPLETE_INT interrupt."]
    #[inline(always)]
    pub fn trans_complete_int_ena(&self) -> TRANS_COMPLETE_INT_ENA_R {
        TRANS_COMPLETE_INT_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The interrupt enable bit for the I2C_TIME_OUT_INT interrupt."]
    #[inline(always)]
    pub fn time_out_int_ena(&self) -> TIME_OUT_INT_ENA_R {
        TIME_OUT_INT_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The interrupt enable bit for the I2C_TRANS_START_INT interrupt."]
    #[inline(always)]
    pub fn trans_start_int_ena(&self) -> TRANS_START_INT_ENA_R {
        TRANS_START_INT_ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The interrupt enable bit for I2C_SLAVE_STRETCH_INT interrupt."]
    #[inline(always)]
    pub fn nack_int_ena(&self) -> NACK_INT_ENA_R {
        NACK_INT_ENA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The interrupt enable bit for I2C_TXFIFO_OVF_INT interrupt."]
    #[inline(always)]
    pub fn txfifo_ovf_int_ena(&self) -> TXFIFO_OVF_INT_ENA_R {
        TXFIFO_OVF_INT_ENA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The interrupt enable bit for I2C_RXFIFO_UDF_INT interrupt."]
    #[inline(always)]
    pub fn rxfifo_udf_int_ena(&self) -> RXFIFO_UDF_INT_ENA_R {
        RXFIFO_UDF_INT_ENA_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The interrupt enable bit for I2C_SCL_ST_TO_INT interrupt."]
    #[inline(always)]
    pub fn scl_st_to_int_ena(&self) -> SCL_ST_TO_INT_ENA_R {
        SCL_ST_TO_INT_ENA_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The interrupt enable bit for I2C_SCL_MAIN_ST_TO_INT interrupt."]
    #[inline(always)]
    pub fn scl_main_st_to_int_ena(&self) -> SCL_MAIN_ST_TO_INT_ENA_R {
        SCL_MAIN_ST_TO_INT_ENA_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The interrupt enable bit for I2C_DET_START_INT interrupt."]
    #[inline(always)]
    pub fn det_start_int_ena(&self) -> DET_START_INT_ENA_R {
        DET_START_INT_ENA_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - The interrupt enable bit for I2C_SLAVE_STRETCH_INT interrupt."]
    #[inline(always)]
    pub fn slave_stretch_int_ena(&self) -> SLAVE_STRETCH_INT_ENA_R {
        SLAVE_STRETCH_INT_ENA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The interrupt enable bit for I2C_GENARAL_CALL_INT interrupt."]
    #[inline(always)]
    pub fn general_call_int_ena(&self) -> GENERAL_CALL_INT_ENA_R {
        GENERAL_CALL_INT_ENA_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The interrupt enable bit for I2C_RXFIFO_WM_INT interrupt."]
    #[inline(always)]
    pub fn rxfifo_wm_int_ena(&mut self) -> RXFIFO_WM_INT_ENA_W {
        RXFIFO_WM_INT_ENA_W { w: self }
    }
    #[doc = "Bit 1 - The interrupt enable bit for I2C_TXFIFO_WM_INT interrupt."]
    #[inline(always)]
    pub fn txfifo_wm_int_ena(&mut self) -> TXFIFO_WM_INT_ENA_W {
        TXFIFO_WM_INT_ENA_W { w: self }
    }
    #[doc = "Bit 2 - The interrupt enable bit for I2C_RXFIFO_OVF_INT interrupt."]
    #[inline(always)]
    pub fn rxfifo_ovf_int_ena(&mut self) -> RXFIFO_OVF_INT_ENA_W {
        RXFIFO_OVF_INT_ENA_W { w: self }
    }
    #[doc = "Bit 3 - The interrupt enable bit for the I2C_END_DETECT_INT interrupt."]
    #[inline(always)]
    pub fn end_detect_int_ena(&mut self) -> END_DETECT_INT_ENA_W {
        END_DETECT_INT_ENA_W { w: self }
    }
    #[doc = "Bit 4 - The interrupt enable bit for the I2C_END_DETECT_INT interrupt."]
    #[inline(always)]
    pub fn byte_trans_done_int_ena(&mut self) -> BYTE_TRANS_DONE_INT_ENA_W {
        BYTE_TRANS_DONE_INT_ENA_W { w: self }
    }
    #[doc = "Bit 5 - The interrupt enable bit for the I2C_ARBITRATION_LOST_INT interrupt."]
    #[inline(always)]
    pub fn arbitration_lost_int_ena(&mut self) -> ARBITRATION_LOST_INT_ENA_W {
        ARBITRATION_LOST_INT_ENA_W { w: self }
    }
    #[doc = "Bit 6 - The interrupt enable bit for I2C_TRANS_COMPLETE_INT interrupt."]
    #[inline(always)]
    pub fn mst_txfifo_udf_int_ena(&mut self) -> MST_TXFIFO_UDF_INT_ENA_W {
        MST_TXFIFO_UDF_INT_ENA_W { w: self }
    }
    #[doc = "Bit 7 - The interrupt enable bit for the I2C_TRANS_COMPLETE_INT interrupt."]
    #[inline(always)]
    pub fn trans_complete_int_ena(&mut self) -> TRANS_COMPLETE_INT_ENA_W {
        TRANS_COMPLETE_INT_ENA_W { w: self }
    }
    #[doc = "Bit 8 - The interrupt enable bit for the I2C_TIME_OUT_INT interrupt."]
    #[inline(always)]
    pub fn time_out_int_ena(&mut self) -> TIME_OUT_INT_ENA_W {
        TIME_OUT_INT_ENA_W { w: self }
    }
    #[doc = "Bit 9 - The interrupt enable bit for the I2C_TRANS_START_INT interrupt."]
    #[inline(always)]
    pub fn trans_start_int_ena(&mut self) -> TRANS_START_INT_ENA_W {
        TRANS_START_INT_ENA_W { w: self }
    }
    #[doc = "Bit 10 - The interrupt enable bit for I2C_SLAVE_STRETCH_INT interrupt."]
    #[inline(always)]
    pub fn nack_int_ena(&mut self) -> NACK_INT_ENA_W {
        NACK_INT_ENA_W { w: self }
    }
    #[doc = "Bit 11 - The interrupt enable bit for I2C_TXFIFO_OVF_INT interrupt."]
    #[inline(always)]
    pub fn txfifo_ovf_int_ena(&mut self) -> TXFIFO_OVF_INT_ENA_W {
        TXFIFO_OVF_INT_ENA_W { w: self }
    }
    #[doc = "Bit 12 - The interrupt enable bit for I2C_RXFIFO_UDF_INT interrupt."]
    #[inline(always)]
    pub fn rxfifo_udf_int_ena(&mut self) -> RXFIFO_UDF_INT_ENA_W {
        RXFIFO_UDF_INT_ENA_W { w: self }
    }
    #[doc = "Bit 13 - The interrupt enable bit for I2C_SCL_ST_TO_INT interrupt."]
    #[inline(always)]
    pub fn scl_st_to_int_ena(&mut self) -> SCL_ST_TO_INT_ENA_W {
        SCL_ST_TO_INT_ENA_W { w: self }
    }
    #[doc = "Bit 14 - The interrupt enable bit for I2C_SCL_MAIN_ST_TO_INT interrupt."]
    #[inline(always)]
    pub fn scl_main_st_to_int_ena(&mut self) -> SCL_MAIN_ST_TO_INT_ENA_W {
        SCL_MAIN_ST_TO_INT_ENA_W { w: self }
    }
    #[doc = "Bit 15 - The interrupt enable bit for I2C_DET_START_INT interrupt."]
    #[inline(always)]
    pub fn det_start_int_ena(&mut self) -> DET_START_INT_ENA_W {
        DET_START_INT_ENA_W { w: self }
    }
    #[doc = "Bit 16 - The interrupt enable bit for I2C_SLAVE_STRETCH_INT interrupt."]
    #[inline(always)]
    pub fn slave_stretch_int_ena(&mut self) -> SLAVE_STRETCH_INT_ENA_W {
        SLAVE_STRETCH_INT_ENA_W { w: self }
    }
    #[doc = "Bit 17 - The interrupt enable bit for I2C_GENARAL_CALL_INT interrupt."]
    #[inline(always)]
    pub fn general_call_int_ena(&mut self) -> GENERAL_CALL_INT_ENA_W {
        GENERAL_CALL_INT_ENA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt enable bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_ena](index.html) module"]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_ena::R](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_ena::W](W) writer structure"]
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
