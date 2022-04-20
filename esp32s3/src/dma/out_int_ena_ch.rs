#[doc = "Register `OUT_INT_ENA_CH%s` reader"]
pub struct R(crate::R<OUT_INT_ENA_CH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUT_INT_ENA_CH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUT_INT_ENA_CH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUT_INT_ENA_CH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUT_INT_ENA_CH%s` writer"]
pub struct W(crate::W<OUT_INT_ENA_CH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUT_INT_ENA_CH_SPEC>;
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
impl From<crate::W<OUT_INT_ENA_CH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUT_INT_ENA_CH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUT_DONE_CH_INT_ENA` reader - The interrupt enable bit for the OUT_DONE_CH_INT interrupt."]
pub struct OUT_DONE_CH_INT_ENA_R(crate::FieldReader<bool, bool>);
impl OUT_DONE_CH_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUT_DONE_CH_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_DONE_CH_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_DONE_CH_INT_ENA` writer - The interrupt enable bit for the OUT_DONE_CH_INT interrupt."]
pub struct OUT_DONE_CH_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_DONE_CH_INT_ENA_W<'a> {
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
#[doc = "Field `OUT_EOF_CH_INT_ENA` reader - The interrupt enable bit for the OUT_EOF_CH_INT interrupt."]
pub struct OUT_EOF_CH_INT_ENA_R(crate::FieldReader<bool, bool>);
impl OUT_EOF_CH_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUT_EOF_CH_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_EOF_CH_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_EOF_CH_INT_ENA` writer - The interrupt enable bit for the OUT_EOF_CH_INT interrupt."]
pub struct OUT_EOF_CH_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_EOF_CH_INT_ENA_W<'a> {
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
#[doc = "Field `OUT_DSCR_ERR_CH_INT_ENA` reader - The interrupt enable bit for the OUT_DSCR_ERR_CH_INT interrupt."]
pub struct OUT_DSCR_ERR_CH_INT_ENA_R(crate::FieldReader<bool, bool>);
impl OUT_DSCR_ERR_CH_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUT_DSCR_ERR_CH_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_DSCR_ERR_CH_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_DSCR_ERR_CH_INT_ENA` writer - The interrupt enable bit for the OUT_DSCR_ERR_CH_INT interrupt."]
pub struct OUT_DSCR_ERR_CH_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_DSCR_ERR_CH_INT_ENA_W<'a> {
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
#[doc = "Field `OUT_TOTAL_EOF_CH_INT_ENA` reader - The interrupt enable bit for the OUT_TOTAL_EOF_CH_INT interrupt."]
pub struct OUT_TOTAL_EOF_CH_INT_ENA_R(crate::FieldReader<bool, bool>);
impl OUT_TOTAL_EOF_CH_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUT_TOTAL_EOF_CH_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_TOTAL_EOF_CH_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_TOTAL_EOF_CH_INT_ENA` writer - The interrupt enable bit for the OUT_TOTAL_EOF_CH_INT interrupt."]
pub struct OUT_TOTAL_EOF_CH_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_TOTAL_EOF_CH_INT_ENA_W<'a> {
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
#[doc = "Field `OUTFIFO_OVF_L1_CH_INT_ENA` reader - The interrupt enable bit for the OUTFIFO_OVF_L1_CH_INT interrupt."]
pub struct OUTFIFO_OVF_L1_CH_INT_ENA_R(crate::FieldReader<bool, bool>);
impl OUTFIFO_OVF_L1_CH_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUTFIFO_OVF_L1_CH_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTFIFO_OVF_L1_CH_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTFIFO_OVF_L1_CH_INT_ENA` writer - The interrupt enable bit for the OUTFIFO_OVF_L1_CH_INT interrupt."]
pub struct OUTFIFO_OVF_L1_CH_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTFIFO_OVF_L1_CH_INT_ENA_W<'a> {
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
#[doc = "Field `OUTFIFO_UDF_L1_CH_INT_ENA` reader - The interrupt enable bit for the OUTFIFO_UDF_L1_CH_INT interrupt."]
pub struct OUTFIFO_UDF_L1_CH_INT_ENA_R(crate::FieldReader<bool, bool>);
impl OUTFIFO_UDF_L1_CH_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUTFIFO_UDF_L1_CH_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTFIFO_UDF_L1_CH_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTFIFO_UDF_L1_CH_INT_ENA` writer - The interrupt enable bit for the OUTFIFO_UDF_L1_CH_INT interrupt."]
pub struct OUTFIFO_UDF_L1_CH_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTFIFO_UDF_L1_CH_INT_ENA_W<'a> {
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
#[doc = "Field `OUTFIFO_OVF_L3_CH_INT_ENA` reader - The interrupt enable bit for the OUTFIFO_OVF_L3_CH_INT interrupt."]
pub struct OUTFIFO_OVF_L3_CH_INT_ENA_R(crate::FieldReader<bool, bool>);
impl OUTFIFO_OVF_L3_CH_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUTFIFO_OVF_L3_CH_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTFIFO_OVF_L3_CH_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTFIFO_OVF_L3_CH_INT_ENA` writer - The interrupt enable bit for the OUTFIFO_OVF_L3_CH_INT interrupt."]
pub struct OUTFIFO_OVF_L3_CH_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTFIFO_OVF_L3_CH_INT_ENA_W<'a> {
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
#[doc = "Field `OUTFIFO_UDF_L3_CH_INT_ENA` reader - The interrupt enable bit for the OUTFIFO_UDF_L3_CH_INT interrupt."]
pub struct OUTFIFO_UDF_L3_CH_INT_ENA_R(crate::FieldReader<bool, bool>);
impl OUTFIFO_UDF_L3_CH_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUTFIFO_UDF_L3_CH_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTFIFO_UDF_L3_CH_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTFIFO_UDF_L3_CH_INT_ENA` writer - The interrupt enable bit for the OUTFIFO_UDF_L3_CH_INT interrupt."]
pub struct OUTFIFO_UDF_L3_CH_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTFIFO_UDF_L3_CH_INT_ENA_W<'a> {
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
impl R {
    #[doc = "Bit 0 - The interrupt enable bit for the OUT_DONE_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_done_ch_int_ena(&self) -> OUT_DONE_CH_INT_ENA_R {
        OUT_DONE_CH_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for the OUT_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_eof_ch_int_ena(&self) -> OUT_EOF_CH_INT_ENA_R {
        OUT_EOF_CH_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The interrupt enable bit for the OUT_DSCR_ERR_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_dscr_err_ch_int_ena(&self) -> OUT_DSCR_ERR_CH_INT_ENA_R {
        OUT_DSCR_ERR_CH_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The interrupt enable bit for the OUT_TOTAL_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_total_eof_ch_int_ena(&self) -> OUT_TOTAL_EOF_CH_INT_ENA_R {
        OUT_TOTAL_EOF_CH_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The interrupt enable bit for the OUTFIFO_OVF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_ovf_l1_ch_int_ena(&self) -> OUTFIFO_OVF_L1_CH_INT_ENA_R {
        OUTFIFO_OVF_L1_CH_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The interrupt enable bit for the OUTFIFO_UDF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_udf_l1_ch_int_ena(&self) -> OUTFIFO_UDF_L1_CH_INT_ENA_R {
        OUTFIFO_UDF_L1_CH_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The interrupt enable bit for the OUTFIFO_OVF_L3_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_ovf_l3_ch_int_ena(&self) -> OUTFIFO_OVF_L3_CH_INT_ENA_R {
        OUTFIFO_OVF_L3_CH_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The interrupt enable bit for the OUTFIFO_UDF_L3_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_udf_l3_ch_int_ena(&self) -> OUTFIFO_UDF_L3_CH_INT_ENA_R {
        OUTFIFO_UDF_L3_CH_INT_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The interrupt enable bit for the OUT_DONE_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_done_ch_int_ena(&mut self) -> OUT_DONE_CH_INT_ENA_W {
        OUT_DONE_CH_INT_ENA_W { w: self }
    }
    #[doc = "Bit 1 - The interrupt enable bit for the OUT_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_eof_ch_int_ena(&mut self) -> OUT_EOF_CH_INT_ENA_W {
        OUT_EOF_CH_INT_ENA_W { w: self }
    }
    #[doc = "Bit 2 - The interrupt enable bit for the OUT_DSCR_ERR_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_dscr_err_ch_int_ena(&mut self) -> OUT_DSCR_ERR_CH_INT_ENA_W {
        OUT_DSCR_ERR_CH_INT_ENA_W { w: self }
    }
    #[doc = "Bit 3 - The interrupt enable bit for the OUT_TOTAL_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_total_eof_ch_int_ena(&mut self) -> OUT_TOTAL_EOF_CH_INT_ENA_W {
        OUT_TOTAL_EOF_CH_INT_ENA_W { w: self }
    }
    #[doc = "Bit 4 - The interrupt enable bit for the OUTFIFO_OVF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_ovf_l1_ch_int_ena(&mut self) -> OUTFIFO_OVF_L1_CH_INT_ENA_W {
        OUTFIFO_OVF_L1_CH_INT_ENA_W { w: self }
    }
    #[doc = "Bit 5 - The interrupt enable bit for the OUTFIFO_UDF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_udf_l1_ch_int_ena(&mut self) -> OUTFIFO_UDF_L1_CH_INT_ENA_W {
        OUTFIFO_UDF_L1_CH_INT_ENA_W { w: self }
    }
    #[doc = "Bit 6 - The interrupt enable bit for the OUTFIFO_OVF_L3_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_ovf_l3_ch_int_ena(&mut self) -> OUTFIFO_OVF_L3_CH_INT_ENA_W {
        OUTFIFO_OVF_L3_CH_INT_ENA_W { w: self }
    }
    #[doc = "Bit 7 - The interrupt enable bit for the OUTFIFO_UDF_L3_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_udf_l3_ch_int_ena(&mut self) -> OUTFIFO_UDF_L3_CH_INT_ENA_W {
        OUTFIFO_UDF_L3_CH_INT_ENA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt enable bits of Tx channel 0\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_int_ena_ch]
(index.html) module"]
pub struct OUT_INT_ENA_CH_SPEC;
impl crate::RegisterSpec for OUT_INT_ENA_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [out_int_ena_ch::R]
(R) reader structure"]
impl crate::Readable for OUT_INT_ENA_CH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [out_int_ena_ch::W]
(W) writer structure"]
impl crate::Writable for OUT_INT_ENA_CH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OUT_INT_ENA_CH%s to value 0"]
impl crate::Resettable for OUT_INT_ENA_CH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
