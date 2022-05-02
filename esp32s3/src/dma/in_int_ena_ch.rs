#[doc = "Register `IN_INT_ENA_CH%s` reader"]
pub struct R(crate::R<IN_INT_ENA_CH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IN_INT_ENA_CH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IN_INT_ENA_CH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IN_INT_ENA_CH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IN_INT_ENA_CH%s` writer"]
pub struct W(crate::W<IN_INT_ENA_CH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IN_INT_ENA_CH_SPEC>;
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
impl From<crate::W<IN_INT_ENA_CH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IN_INT_ENA_CH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IN_DONE_CH_INT_ENA` reader - The interrupt enable bit for the IN_DONE_CH_INT interrupt."]
pub struct IN_DONE_CH_INT_ENA_R(crate::FieldReader<bool>);
impl IN_DONE_CH_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IN_DONE_CH_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN_DONE_CH_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN_DONE_CH_INT_ENA` writer - The interrupt enable bit for the IN_DONE_CH_INT interrupt."]
pub struct IN_DONE_CH_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_DONE_CH_INT_ENA_W<'a> {
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
#[doc = "Field `IN_SUC_EOF_CH_INT_ENA` reader - The interrupt enable bit for the IN_SUC_EOF_CH_INT interrupt."]
pub struct IN_SUC_EOF_CH_INT_ENA_R(crate::FieldReader<bool>);
impl IN_SUC_EOF_CH_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IN_SUC_EOF_CH_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN_SUC_EOF_CH_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN_SUC_EOF_CH_INT_ENA` writer - The interrupt enable bit for the IN_SUC_EOF_CH_INT interrupt."]
pub struct IN_SUC_EOF_CH_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_SUC_EOF_CH_INT_ENA_W<'a> {
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
#[doc = "Field `IN_ERR_EOF_CH_INT_ENA` reader - The interrupt enable bit for the IN_ERR_EOF_CH_INT interrupt."]
pub struct IN_ERR_EOF_CH_INT_ENA_R(crate::FieldReader<bool>);
impl IN_ERR_EOF_CH_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IN_ERR_EOF_CH_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN_ERR_EOF_CH_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN_ERR_EOF_CH_INT_ENA` writer - The interrupt enable bit for the IN_ERR_EOF_CH_INT interrupt."]
pub struct IN_ERR_EOF_CH_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_ERR_EOF_CH_INT_ENA_W<'a> {
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
#[doc = "Field `IN_DSCR_ERR_CH_INT_ENA` reader - The interrupt enable bit for the IN_DSCR_ERR_CH_INT interrupt."]
pub struct IN_DSCR_ERR_CH_INT_ENA_R(crate::FieldReader<bool>);
impl IN_DSCR_ERR_CH_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IN_DSCR_ERR_CH_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN_DSCR_ERR_CH_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN_DSCR_ERR_CH_INT_ENA` writer - The interrupt enable bit for the IN_DSCR_ERR_CH_INT interrupt."]
pub struct IN_DSCR_ERR_CH_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_DSCR_ERR_CH_INT_ENA_W<'a> {
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
#[doc = "Field `IN_DSCR_EMPTY_CH_INT_ENA` reader - The interrupt enable bit for the IN_DSCR_EMPTY_CH_INT interrupt."]
pub struct IN_DSCR_EMPTY_CH_INT_ENA_R(crate::FieldReader<bool>);
impl IN_DSCR_EMPTY_CH_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IN_DSCR_EMPTY_CH_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN_DSCR_EMPTY_CH_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN_DSCR_EMPTY_CH_INT_ENA` writer - The interrupt enable bit for the IN_DSCR_EMPTY_CH_INT interrupt."]
pub struct IN_DSCR_EMPTY_CH_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_DSCR_EMPTY_CH_INT_ENA_W<'a> {
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
#[doc = "Field `INFIFO_FULL_WM_CH_INT_ENA` reader - The interrupt enable bit for the INFIFO_FULL_WM_CH_INT interrupt."]
pub struct INFIFO_FULL_WM_CH_INT_ENA_R(crate::FieldReader<bool>);
impl INFIFO_FULL_WM_CH_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INFIFO_FULL_WM_CH_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INFIFO_FULL_WM_CH_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INFIFO_FULL_WM_CH_INT_ENA` writer - The interrupt enable bit for the INFIFO_FULL_WM_CH_INT interrupt."]
pub struct INFIFO_FULL_WM_CH_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> INFIFO_FULL_WM_CH_INT_ENA_W<'a> {
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
#[doc = "Field `INFIFO_OVF_L1_CH_INT_ENA` reader - The interrupt enable bit for the INFIFO_OVF_L1_CH_INT interrupt."]
pub struct INFIFO_OVF_L1_CH_INT_ENA_R(crate::FieldReader<bool>);
impl INFIFO_OVF_L1_CH_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INFIFO_OVF_L1_CH_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INFIFO_OVF_L1_CH_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INFIFO_OVF_L1_CH_INT_ENA` writer - The interrupt enable bit for the INFIFO_OVF_L1_CH_INT interrupt."]
pub struct INFIFO_OVF_L1_CH_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> INFIFO_OVF_L1_CH_INT_ENA_W<'a> {
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
#[doc = "Field `INFIFO_UDF_L1_CH_INT_ENA` reader - The interrupt enable bit for the INFIFO_UDF_L1_CH_INT interrupt."]
pub struct INFIFO_UDF_L1_CH_INT_ENA_R(crate::FieldReader<bool>);
impl INFIFO_UDF_L1_CH_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INFIFO_UDF_L1_CH_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INFIFO_UDF_L1_CH_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INFIFO_UDF_L1_CH_INT_ENA` writer - The interrupt enable bit for the INFIFO_UDF_L1_CH_INT interrupt."]
pub struct INFIFO_UDF_L1_CH_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> INFIFO_UDF_L1_CH_INT_ENA_W<'a> {
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
#[doc = "Field `INFIFO_OVF_L3_CH_INT_ENA` reader - The interrupt enable bit for the INFIFO_OVF_L3_CH_INT interrupt."]
pub struct INFIFO_OVF_L3_CH_INT_ENA_R(crate::FieldReader<bool>);
impl INFIFO_OVF_L3_CH_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INFIFO_OVF_L3_CH_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INFIFO_OVF_L3_CH_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INFIFO_OVF_L3_CH_INT_ENA` writer - The interrupt enable bit for the INFIFO_OVF_L3_CH_INT interrupt."]
pub struct INFIFO_OVF_L3_CH_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> INFIFO_OVF_L3_CH_INT_ENA_W<'a> {
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
#[doc = "Field `INFIFO_UDF_L3_CH_INT_ENA` reader - The interrupt enable bit for the INFIFO_UDF_L3_CH_INT interrupt."]
pub struct INFIFO_UDF_L3_CH_INT_ENA_R(crate::FieldReader<bool>);
impl INFIFO_UDF_L3_CH_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INFIFO_UDF_L3_CH_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INFIFO_UDF_L3_CH_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INFIFO_UDF_L3_CH_INT_ENA` writer - The interrupt enable bit for the INFIFO_UDF_L3_CH_INT interrupt."]
pub struct INFIFO_UDF_L3_CH_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> INFIFO_UDF_L3_CH_INT_ENA_W<'a> {
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
impl R {
    #[doc = "Bit 0 - The interrupt enable bit for the IN_DONE_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_done_ch_int_ena(&self) -> IN_DONE_CH_INT_ENA_R {
        IN_DONE_CH_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for the IN_SUC_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_suc_eof_ch_int_ena(&self) -> IN_SUC_EOF_CH_INT_ENA_R {
        IN_SUC_EOF_CH_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The interrupt enable bit for the IN_ERR_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_err_eof_ch_int_ena(&self) -> IN_ERR_EOF_CH_INT_ENA_R {
        IN_ERR_EOF_CH_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The interrupt enable bit for the IN_DSCR_ERR_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_dscr_err_ch_int_ena(&self) -> IN_DSCR_ERR_CH_INT_ENA_R {
        IN_DSCR_ERR_CH_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The interrupt enable bit for the IN_DSCR_EMPTY_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_dscr_empty_ch_int_ena(&self) -> IN_DSCR_EMPTY_CH_INT_ENA_R {
        IN_DSCR_EMPTY_CH_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The interrupt enable bit for the INFIFO_FULL_WM_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_full_wm_ch_int_ena(&self) -> INFIFO_FULL_WM_CH_INT_ENA_R {
        INFIFO_FULL_WM_CH_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The interrupt enable bit for the INFIFO_OVF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_ovf_l1_ch_int_ena(&self) -> INFIFO_OVF_L1_CH_INT_ENA_R {
        INFIFO_OVF_L1_CH_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The interrupt enable bit for the INFIFO_UDF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_udf_l1_ch_int_ena(&self) -> INFIFO_UDF_L1_CH_INT_ENA_R {
        INFIFO_UDF_L1_CH_INT_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The interrupt enable bit for the INFIFO_OVF_L3_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_ovf_l3_ch_int_ena(&self) -> INFIFO_OVF_L3_CH_INT_ENA_R {
        INFIFO_OVF_L3_CH_INT_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The interrupt enable bit for the INFIFO_UDF_L3_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_udf_l3_ch_int_ena(&self) -> INFIFO_UDF_L3_CH_INT_ENA_R {
        INFIFO_UDF_L3_CH_INT_ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The interrupt enable bit for the IN_DONE_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_done_ch_int_ena(&mut self) -> IN_DONE_CH_INT_ENA_W {
        IN_DONE_CH_INT_ENA_W { w: self }
    }
    #[doc = "Bit 1 - The interrupt enable bit for the IN_SUC_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_suc_eof_ch_int_ena(&mut self) -> IN_SUC_EOF_CH_INT_ENA_W {
        IN_SUC_EOF_CH_INT_ENA_W { w: self }
    }
    #[doc = "Bit 2 - The interrupt enable bit for the IN_ERR_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_err_eof_ch_int_ena(&mut self) -> IN_ERR_EOF_CH_INT_ENA_W {
        IN_ERR_EOF_CH_INT_ENA_W { w: self }
    }
    #[doc = "Bit 3 - The interrupt enable bit for the IN_DSCR_ERR_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_dscr_err_ch_int_ena(&mut self) -> IN_DSCR_ERR_CH_INT_ENA_W {
        IN_DSCR_ERR_CH_INT_ENA_W { w: self }
    }
    #[doc = "Bit 4 - The interrupt enable bit for the IN_DSCR_EMPTY_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_dscr_empty_ch_int_ena(&mut self) -> IN_DSCR_EMPTY_CH_INT_ENA_W {
        IN_DSCR_EMPTY_CH_INT_ENA_W { w: self }
    }
    #[doc = "Bit 5 - The interrupt enable bit for the INFIFO_FULL_WM_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_full_wm_ch_int_ena(&mut self) -> INFIFO_FULL_WM_CH_INT_ENA_W {
        INFIFO_FULL_WM_CH_INT_ENA_W { w: self }
    }
    #[doc = "Bit 6 - The interrupt enable bit for the INFIFO_OVF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_ovf_l1_ch_int_ena(&mut self) -> INFIFO_OVF_L1_CH_INT_ENA_W {
        INFIFO_OVF_L1_CH_INT_ENA_W { w: self }
    }
    #[doc = "Bit 7 - The interrupt enable bit for the INFIFO_UDF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_udf_l1_ch_int_ena(&mut self) -> INFIFO_UDF_L1_CH_INT_ENA_W {
        INFIFO_UDF_L1_CH_INT_ENA_W { w: self }
    }
    #[doc = "Bit 8 - The interrupt enable bit for the INFIFO_OVF_L3_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_ovf_l3_ch_int_ena(&mut self) -> INFIFO_OVF_L3_CH_INT_ENA_W {
        INFIFO_OVF_L3_CH_INT_ENA_W { w: self }
    }
    #[doc = "Bit 9 - The interrupt enable bit for the INFIFO_UDF_L3_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_udf_l3_ch_int_ena(&mut self) -> INFIFO_UDF_L3_CH_INT_ENA_W {
        INFIFO_UDF_L3_CH_INT_ENA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt enable bits of Rx channel 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [in_int_ena_ch](index.html) module"]
pub struct IN_INT_ENA_CH_SPEC;
impl crate::RegisterSpec for IN_INT_ENA_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [in_int_ena_ch::R](R) reader structure"]
impl crate::Readable for IN_INT_ENA_CH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [in_int_ena_ch::W](W) writer structure"]
impl crate::Writable for IN_INT_ENA_CH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IN_INT_ENA_CH%s to value 0"]
impl crate::Resettable for IN_INT_ENA_CH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
