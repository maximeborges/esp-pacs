#[doc = "Register `INT_ENA_CH2` reader"]
pub struct R(crate::R<INT_ENA_CH2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ENA_CH2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_ENA_CH2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_ENA_CH2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_ENA_CH2` writer"]
pub struct W(crate::W<INT_ENA_CH2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_ENA_CH2_SPEC>;
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
impl From<crate::W<INT_ENA_CH2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_ENA_CH2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IN_DONE_CH2_INT_ENA` reader - The interrupt enable bit for the IN_DONE_CH_INT interrupt."]
pub struct IN_DONE_CH2_INT_ENA_R(crate::FieldReader<bool>);
impl IN_DONE_CH2_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IN_DONE_CH2_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN_DONE_CH2_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN_DONE_CH2_INT_ENA` writer - The interrupt enable bit for the IN_DONE_CH_INT interrupt."]
pub struct IN_DONE_CH2_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_DONE_CH2_INT_ENA_W<'a> {
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
#[doc = "Field `IN_SUC_EOF_CH2_INT_ENA` reader - The interrupt enable bit for the IN_SUC_EOF_CH_INT interrupt."]
pub struct IN_SUC_EOF_CH2_INT_ENA_R(crate::FieldReader<bool>);
impl IN_SUC_EOF_CH2_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IN_SUC_EOF_CH2_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN_SUC_EOF_CH2_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN_SUC_EOF_CH2_INT_ENA` writer - The interrupt enable bit for the IN_SUC_EOF_CH_INT interrupt."]
pub struct IN_SUC_EOF_CH2_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_SUC_EOF_CH2_INT_ENA_W<'a> {
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
#[doc = "Field `IN_ERR_EOF_CH2_INT_ENA` reader - The interrupt enable bit for the IN_ERR_EOF_CH_INT interrupt."]
pub struct IN_ERR_EOF_CH2_INT_ENA_R(crate::FieldReader<bool>);
impl IN_ERR_EOF_CH2_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IN_ERR_EOF_CH2_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN_ERR_EOF_CH2_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN_ERR_EOF_CH2_INT_ENA` writer - The interrupt enable bit for the IN_ERR_EOF_CH_INT interrupt."]
pub struct IN_ERR_EOF_CH2_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_ERR_EOF_CH2_INT_ENA_W<'a> {
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
#[doc = "Field `OUT_DONE_CH2_INT_ENA` reader - The interrupt enable bit for the OUT_DONE_CH_INT interrupt."]
pub struct OUT_DONE_CH2_INT_ENA_R(crate::FieldReader<bool>);
impl OUT_DONE_CH2_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUT_DONE_CH2_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_DONE_CH2_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_DONE_CH2_INT_ENA` writer - The interrupt enable bit for the OUT_DONE_CH_INT interrupt."]
pub struct OUT_DONE_CH2_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_DONE_CH2_INT_ENA_W<'a> {
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
#[doc = "Field `OUT_EOF_CH2_INT_ENA` reader - The interrupt enable bit for the OUT_EOF_CH_INT interrupt."]
pub struct OUT_EOF_CH2_INT_ENA_R(crate::FieldReader<bool>);
impl OUT_EOF_CH2_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUT_EOF_CH2_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_EOF_CH2_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_EOF_CH2_INT_ENA` writer - The interrupt enable bit for the OUT_EOF_CH_INT interrupt."]
pub struct OUT_EOF_CH2_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_EOF_CH2_INT_ENA_W<'a> {
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
#[doc = "Field `IN_DSCR_ERR_CH2_INT_ENA` reader - The interrupt enable bit for the IN_DSCR_ERR_CH_INT interrupt."]
pub struct IN_DSCR_ERR_CH2_INT_ENA_R(crate::FieldReader<bool>);
impl IN_DSCR_ERR_CH2_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IN_DSCR_ERR_CH2_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN_DSCR_ERR_CH2_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN_DSCR_ERR_CH2_INT_ENA` writer - The interrupt enable bit for the IN_DSCR_ERR_CH_INT interrupt."]
pub struct IN_DSCR_ERR_CH2_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_DSCR_ERR_CH2_INT_ENA_W<'a> {
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
#[doc = "Field `OUT_DSCR_ERR_CH2_INT_ENA` reader - The interrupt enable bit for the OUT_DSCR_ERR_CH_INT interrupt."]
pub struct OUT_DSCR_ERR_CH2_INT_ENA_R(crate::FieldReader<bool>);
impl OUT_DSCR_ERR_CH2_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUT_DSCR_ERR_CH2_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_DSCR_ERR_CH2_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_DSCR_ERR_CH2_INT_ENA` writer - The interrupt enable bit for the OUT_DSCR_ERR_CH_INT interrupt."]
pub struct OUT_DSCR_ERR_CH2_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_DSCR_ERR_CH2_INT_ENA_W<'a> {
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
#[doc = "Field `IN_DSCR_EMPTY_CH2_INT_ENA` reader - The interrupt enable bit for the IN_DSCR_EMPTY_CH_INT interrupt."]
pub struct IN_DSCR_EMPTY_CH2_INT_ENA_R(crate::FieldReader<bool>);
impl IN_DSCR_EMPTY_CH2_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IN_DSCR_EMPTY_CH2_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN_DSCR_EMPTY_CH2_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN_DSCR_EMPTY_CH2_INT_ENA` writer - The interrupt enable bit for the IN_DSCR_EMPTY_CH_INT interrupt."]
pub struct IN_DSCR_EMPTY_CH2_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_DSCR_EMPTY_CH2_INT_ENA_W<'a> {
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
#[doc = "Field `OUT_TOTAL_EOF_CH2_INT_ENA` reader - The interrupt enable bit for the OUT_TOTAL_EOF_CH_INT interrupt."]
pub struct OUT_TOTAL_EOF_CH2_INT_ENA_R(crate::FieldReader<bool>);
impl OUT_TOTAL_EOF_CH2_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUT_TOTAL_EOF_CH2_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_TOTAL_EOF_CH2_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_TOTAL_EOF_CH2_INT_ENA` writer - The interrupt enable bit for the OUT_TOTAL_EOF_CH_INT interrupt."]
pub struct OUT_TOTAL_EOF_CH2_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_TOTAL_EOF_CH2_INT_ENA_W<'a> {
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
#[doc = "Field `INFIFO_OVF_CH2_INT_ENA` reader - The interrupt enable bit for the INFIFO_OVF_L1_CH_INT interrupt."]
pub struct INFIFO_OVF_CH2_INT_ENA_R(crate::FieldReader<bool>);
impl INFIFO_OVF_CH2_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INFIFO_OVF_CH2_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INFIFO_OVF_CH2_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INFIFO_OVF_CH2_INT_ENA` writer - The interrupt enable bit for the INFIFO_OVF_L1_CH_INT interrupt."]
pub struct INFIFO_OVF_CH2_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> INFIFO_OVF_CH2_INT_ENA_W<'a> {
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
#[doc = "Field `INFIFO_UDF_CH2_INT_ENA` reader - The interrupt enable bit for the INFIFO_UDF_L1_CH_INT interrupt."]
pub struct INFIFO_UDF_CH2_INT_ENA_R(crate::FieldReader<bool>);
impl INFIFO_UDF_CH2_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INFIFO_UDF_CH2_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INFIFO_UDF_CH2_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INFIFO_UDF_CH2_INT_ENA` writer - The interrupt enable bit for the INFIFO_UDF_L1_CH_INT interrupt."]
pub struct INFIFO_UDF_CH2_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> INFIFO_UDF_CH2_INT_ENA_W<'a> {
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
#[doc = "Field `OUTFIFO_OVF_CH2_INT_ENA` reader - The interrupt enable bit for the OUTFIFO_OVF_L1_CH_INT interrupt."]
pub struct OUTFIFO_OVF_CH2_INT_ENA_R(crate::FieldReader<bool>);
impl OUTFIFO_OVF_CH2_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUTFIFO_OVF_CH2_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTFIFO_OVF_CH2_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTFIFO_OVF_CH2_INT_ENA` writer - The interrupt enable bit for the OUTFIFO_OVF_L1_CH_INT interrupt."]
pub struct OUTFIFO_OVF_CH2_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTFIFO_OVF_CH2_INT_ENA_W<'a> {
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
#[doc = "Field `OUTFIFO_UDF_CH2_INT_ENA` reader - The interrupt enable bit for the OUTFIFO_UDF_L1_CH_INT interrupt."]
pub struct OUTFIFO_UDF_CH2_INT_ENA_R(crate::FieldReader<bool>);
impl OUTFIFO_UDF_CH2_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUTFIFO_UDF_CH2_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTFIFO_UDF_CH2_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTFIFO_UDF_CH2_INT_ENA` writer - The interrupt enable bit for the OUTFIFO_UDF_L1_CH_INT interrupt."]
pub struct OUTFIFO_UDF_CH2_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTFIFO_UDF_CH2_INT_ENA_W<'a> {
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
impl R {
    #[doc = "Bit 0 - The interrupt enable bit for the IN_DONE_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_done_ch2_int_ena(&self) -> IN_DONE_CH2_INT_ENA_R {
        IN_DONE_CH2_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for the IN_SUC_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_suc_eof_ch2_int_ena(&self) -> IN_SUC_EOF_CH2_INT_ENA_R {
        IN_SUC_EOF_CH2_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The interrupt enable bit for the IN_ERR_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_err_eof_ch2_int_ena(&self) -> IN_ERR_EOF_CH2_INT_ENA_R {
        IN_ERR_EOF_CH2_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The interrupt enable bit for the OUT_DONE_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_done_ch2_int_ena(&self) -> OUT_DONE_CH2_INT_ENA_R {
        OUT_DONE_CH2_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The interrupt enable bit for the OUT_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_eof_ch2_int_ena(&self) -> OUT_EOF_CH2_INT_ENA_R {
        OUT_EOF_CH2_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The interrupt enable bit for the IN_DSCR_ERR_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_dscr_err_ch2_int_ena(&self) -> IN_DSCR_ERR_CH2_INT_ENA_R {
        IN_DSCR_ERR_CH2_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The interrupt enable bit for the OUT_DSCR_ERR_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_dscr_err_ch2_int_ena(&self) -> OUT_DSCR_ERR_CH2_INT_ENA_R {
        OUT_DSCR_ERR_CH2_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The interrupt enable bit for the IN_DSCR_EMPTY_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_dscr_empty_ch2_int_ena(&self) -> IN_DSCR_EMPTY_CH2_INT_ENA_R {
        IN_DSCR_EMPTY_CH2_INT_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The interrupt enable bit for the OUT_TOTAL_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_total_eof_ch2_int_ena(&self) -> OUT_TOTAL_EOF_CH2_INT_ENA_R {
        OUT_TOTAL_EOF_CH2_INT_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The interrupt enable bit for the INFIFO_OVF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_ovf_ch2_int_ena(&self) -> INFIFO_OVF_CH2_INT_ENA_R {
        INFIFO_OVF_CH2_INT_ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The interrupt enable bit for the INFIFO_UDF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_udf_ch2_int_ena(&self) -> INFIFO_UDF_CH2_INT_ENA_R {
        INFIFO_UDF_CH2_INT_ENA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The interrupt enable bit for the OUTFIFO_OVF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_ovf_ch2_int_ena(&self) -> OUTFIFO_OVF_CH2_INT_ENA_R {
        OUTFIFO_OVF_CH2_INT_ENA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The interrupt enable bit for the OUTFIFO_UDF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_udf_ch2_int_ena(&self) -> OUTFIFO_UDF_CH2_INT_ENA_R {
        OUTFIFO_UDF_CH2_INT_ENA_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The interrupt enable bit for the IN_DONE_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_done_ch2_int_ena(&mut self) -> IN_DONE_CH2_INT_ENA_W {
        IN_DONE_CH2_INT_ENA_W { w: self }
    }
    #[doc = "Bit 1 - The interrupt enable bit for the IN_SUC_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_suc_eof_ch2_int_ena(&mut self) -> IN_SUC_EOF_CH2_INT_ENA_W {
        IN_SUC_EOF_CH2_INT_ENA_W { w: self }
    }
    #[doc = "Bit 2 - The interrupt enable bit for the IN_ERR_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_err_eof_ch2_int_ena(&mut self) -> IN_ERR_EOF_CH2_INT_ENA_W {
        IN_ERR_EOF_CH2_INT_ENA_W { w: self }
    }
    #[doc = "Bit 3 - The interrupt enable bit for the OUT_DONE_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_done_ch2_int_ena(&mut self) -> OUT_DONE_CH2_INT_ENA_W {
        OUT_DONE_CH2_INT_ENA_W { w: self }
    }
    #[doc = "Bit 4 - The interrupt enable bit for the OUT_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_eof_ch2_int_ena(&mut self) -> OUT_EOF_CH2_INT_ENA_W {
        OUT_EOF_CH2_INT_ENA_W { w: self }
    }
    #[doc = "Bit 5 - The interrupt enable bit for the IN_DSCR_ERR_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_dscr_err_ch2_int_ena(&mut self) -> IN_DSCR_ERR_CH2_INT_ENA_W {
        IN_DSCR_ERR_CH2_INT_ENA_W { w: self }
    }
    #[doc = "Bit 6 - The interrupt enable bit for the OUT_DSCR_ERR_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_dscr_err_ch2_int_ena(&mut self) -> OUT_DSCR_ERR_CH2_INT_ENA_W {
        OUT_DSCR_ERR_CH2_INT_ENA_W { w: self }
    }
    #[doc = "Bit 7 - The interrupt enable bit for the IN_DSCR_EMPTY_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_dscr_empty_ch2_int_ena(&mut self) -> IN_DSCR_EMPTY_CH2_INT_ENA_W {
        IN_DSCR_EMPTY_CH2_INT_ENA_W { w: self }
    }
    #[doc = "Bit 8 - The interrupt enable bit for the OUT_TOTAL_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_total_eof_ch2_int_ena(&mut self) -> OUT_TOTAL_EOF_CH2_INT_ENA_W {
        OUT_TOTAL_EOF_CH2_INT_ENA_W { w: self }
    }
    #[doc = "Bit 9 - The interrupt enable bit for the INFIFO_OVF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_ovf_ch2_int_ena(&mut self) -> INFIFO_OVF_CH2_INT_ENA_W {
        INFIFO_OVF_CH2_INT_ENA_W { w: self }
    }
    #[doc = "Bit 10 - The interrupt enable bit for the INFIFO_UDF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_udf_ch2_int_ena(&mut self) -> INFIFO_UDF_CH2_INT_ENA_W {
        INFIFO_UDF_CH2_INT_ENA_W { w: self }
    }
    #[doc = "Bit 11 - The interrupt enable bit for the OUTFIFO_OVF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_ovf_ch2_int_ena(&mut self) -> OUTFIFO_OVF_CH2_INT_ENA_W {
        OUTFIFO_OVF_CH2_INT_ENA_W { w: self }
    }
    #[doc = "Bit 12 - The interrupt enable bit for the OUTFIFO_UDF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_udf_ch2_int_ena(&mut self) -> OUTFIFO_UDF_CH2_INT_ENA_W {
        OUTFIFO_UDF_CH2_INT_ENA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA_INT_ENA_CH2_REG.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_ena_ch2](index.html) module"]
pub struct INT_ENA_CH2_SPEC;
impl crate::RegisterSpec for INT_ENA_CH2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_ena_ch2::R](R) reader structure"]
impl crate::Readable for INT_ENA_CH2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_ena_ch2::W](W) writer structure"]
impl crate::Writable for INT_ENA_CH2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_ENA_CH2 to value 0"]
impl crate::Resettable for INT_ENA_CH2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
