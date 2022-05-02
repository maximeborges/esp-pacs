#[doc = "Register `SAR_COCPU_INT_ENA` reader"]
pub struct R(crate::R<SAR_COCPU_INT_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_COCPU_INT_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_COCPU_INT_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_COCPU_INT_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_COCPU_INT_ENA` writer"]
pub struct W(crate::W<SAR_COCPU_INT_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_COCPU_INT_ENA_SPEC>;
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
impl From<crate::W<SAR_COCPU_INT_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_COCPU_INT_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COCPU_TOUCH_DONE_INT_ENA` reader - TOUCH_DONE_INT interrupt enable bit"]
pub struct COCPU_TOUCH_DONE_INT_ENA_R(crate::FieldReader<bool>);
impl COCPU_TOUCH_DONE_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COCPU_TOUCH_DONE_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COCPU_TOUCH_DONE_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COCPU_TOUCH_DONE_INT_ENA` writer - TOUCH_DONE_INT interrupt enable bit"]
pub struct COCPU_TOUCH_DONE_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> COCPU_TOUCH_DONE_INT_ENA_W<'a> {
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
#[doc = "Field `COCPU_TOUCH_INACTIVE_INT_ENA` reader - TOUCH_INACTIVE_INT interrupt enable bit"]
pub struct COCPU_TOUCH_INACTIVE_INT_ENA_R(crate::FieldReader<bool>);
impl COCPU_TOUCH_INACTIVE_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COCPU_TOUCH_INACTIVE_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COCPU_TOUCH_INACTIVE_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COCPU_TOUCH_INACTIVE_INT_ENA` writer - TOUCH_INACTIVE_INT interrupt enable bit"]
pub struct COCPU_TOUCH_INACTIVE_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> COCPU_TOUCH_INACTIVE_INT_ENA_W<'a> {
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
#[doc = "Field `COCPU_TOUCH_ACTIVE_INT_ENA` reader - TOUCH_ACTIVE_INT interrupt enable bit"]
pub struct COCPU_TOUCH_ACTIVE_INT_ENA_R(crate::FieldReader<bool>);
impl COCPU_TOUCH_ACTIVE_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COCPU_TOUCH_ACTIVE_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COCPU_TOUCH_ACTIVE_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COCPU_TOUCH_ACTIVE_INT_ENA` writer - TOUCH_ACTIVE_INT interrupt enable bit"]
pub struct COCPU_TOUCH_ACTIVE_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> COCPU_TOUCH_ACTIVE_INT_ENA_W<'a> {
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
#[doc = "Field `COCPU_SARADC1_INT_ENA` reader - SARADC1_DONE_INT interrupt enable bit"]
pub struct COCPU_SARADC1_INT_ENA_R(crate::FieldReader<bool>);
impl COCPU_SARADC1_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COCPU_SARADC1_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COCPU_SARADC1_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COCPU_SARADC1_INT_ENA` writer - SARADC1_DONE_INT interrupt enable bit"]
pub struct COCPU_SARADC1_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> COCPU_SARADC1_INT_ENA_W<'a> {
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
#[doc = "Field `COCPU_SARADC2_INT_ENA` reader - SARADC2_DONE_INT interrupt enable bit"]
pub struct COCPU_SARADC2_INT_ENA_R(crate::FieldReader<bool>);
impl COCPU_SARADC2_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COCPU_SARADC2_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COCPU_SARADC2_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COCPU_SARADC2_INT_ENA` writer - SARADC2_DONE_INT interrupt enable bit"]
pub struct COCPU_SARADC2_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> COCPU_SARADC2_INT_ENA_W<'a> {
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
#[doc = "Field `COCPU_TSENS_INT_ENA` reader - TSENS_DONE_INT interrupt enable bit"]
pub struct COCPU_TSENS_INT_ENA_R(crate::FieldReader<bool>);
impl COCPU_TSENS_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COCPU_TSENS_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COCPU_TSENS_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COCPU_TSENS_INT_ENA` writer - TSENS_DONE_INT interrupt enable bit"]
pub struct COCPU_TSENS_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> COCPU_TSENS_INT_ENA_W<'a> {
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
#[doc = "Field `COCPU_START_INT_ENA` reader - RISCV_START_INT interrupt enable bit"]
pub struct COCPU_START_INT_ENA_R(crate::FieldReader<bool>);
impl COCPU_START_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COCPU_START_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COCPU_START_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COCPU_START_INT_ENA` writer - RISCV_START_INT interrupt enable bit"]
pub struct COCPU_START_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> COCPU_START_INT_ENA_W<'a> {
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
#[doc = "Field `COCPU_SW_INT_ENA` reader - SW_INT interrupt enable bit"]
pub struct COCPU_SW_INT_ENA_R(crate::FieldReader<bool>);
impl COCPU_SW_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COCPU_SW_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COCPU_SW_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COCPU_SW_INT_ENA` writer - SW_INT interrupt enable bit"]
pub struct COCPU_SW_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> COCPU_SW_INT_ENA_W<'a> {
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
#[doc = "Field `COCPU_SWD_INT_ENA` reader - SWD_INT interrupt enable bit"]
pub struct COCPU_SWD_INT_ENA_R(crate::FieldReader<bool>);
impl COCPU_SWD_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COCPU_SWD_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COCPU_SWD_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COCPU_SWD_INT_ENA` writer - SWD_INT interrupt enable bit"]
pub struct COCPU_SWD_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> COCPU_SWD_INT_ENA_W<'a> {
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
impl R {
    #[doc = "Bit 0 - TOUCH_DONE_INT interrupt enable bit"]
    #[inline(always)]
    pub fn cocpu_touch_done_int_ena(&self) -> COCPU_TOUCH_DONE_INT_ENA_R {
        COCPU_TOUCH_DONE_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TOUCH_INACTIVE_INT interrupt enable bit"]
    #[inline(always)]
    pub fn cocpu_touch_inactive_int_ena(&self) -> COCPU_TOUCH_INACTIVE_INT_ENA_R {
        COCPU_TOUCH_INACTIVE_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TOUCH_ACTIVE_INT interrupt enable bit"]
    #[inline(always)]
    pub fn cocpu_touch_active_int_ena(&self) -> COCPU_TOUCH_ACTIVE_INT_ENA_R {
        COCPU_TOUCH_ACTIVE_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SARADC1_DONE_INT interrupt enable bit"]
    #[inline(always)]
    pub fn cocpu_saradc1_int_ena(&self) -> COCPU_SARADC1_INT_ENA_R {
        COCPU_SARADC1_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SARADC2_DONE_INT interrupt enable bit"]
    #[inline(always)]
    pub fn cocpu_saradc2_int_ena(&self) -> COCPU_SARADC2_INT_ENA_R {
        COCPU_SARADC2_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TSENS_DONE_INT interrupt enable bit"]
    #[inline(always)]
    pub fn cocpu_tsens_int_ena(&self) -> COCPU_TSENS_INT_ENA_R {
        COCPU_TSENS_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RISCV_START_INT interrupt enable bit"]
    #[inline(always)]
    pub fn cocpu_start_int_ena(&self) -> COCPU_START_INT_ENA_R {
        COCPU_START_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SW_INT interrupt enable bit"]
    #[inline(always)]
    pub fn cocpu_sw_int_ena(&self) -> COCPU_SW_INT_ENA_R {
        COCPU_SW_INT_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SWD_INT interrupt enable bit"]
    #[inline(always)]
    pub fn cocpu_swd_int_ena(&self) -> COCPU_SWD_INT_ENA_R {
        COCPU_SWD_INT_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TOUCH_DONE_INT interrupt enable bit"]
    #[inline(always)]
    pub fn cocpu_touch_done_int_ena(&mut self) -> COCPU_TOUCH_DONE_INT_ENA_W {
        COCPU_TOUCH_DONE_INT_ENA_W { w: self }
    }
    #[doc = "Bit 1 - TOUCH_INACTIVE_INT interrupt enable bit"]
    #[inline(always)]
    pub fn cocpu_touch_inactive_int_ena(&mut self) -> COCPU_TOUCH_INACTIVE_INT_ENA_W {
        COCPU_TOUCH_INACTIVE_INT_ENA_W { w: self }
    }
    #[doc = "Bit 2 - TOUCH_ACTIVE_INT interrupt enable bit"]
    #[inline(always)]
    pub fn cocpu_touch_active_int_ena(&mut self) -> COCPU_TOUCH_ACTIVE_INT_ENA_W {
        COCPU_TOUCH_ACTIVE_INT_ENA_W { w: self }
    }
    #[doc = "Bit 3 - SARADC1_DONE_INT interrupt enable bit"]
    #[inline(always)]
    pub fn cocpu_saradc1_int_ena(&mut self) -> COCPU_SARADC1_INT_ENA_W {
        COCPU_SARADC1_INT_ENA_W { w: self }
    }
    #[doc = "Bit 4 - SARADC2_DONE_INT interrupt enable bit"]
    #[inline(always)]
    pub fn cocpu_saradc2_int_ena(&mut self) -> COCPU_SARADC2_INT_ENA_W {
        COCPU_SARADC2_INT_ENA_W { w: self }
    }
    #[doc = "Bit 5 - TSENS_DONE_INT interrupt enable bit"]
    #[inline(always)]
    pub fn cocpu_tsens_int_ena(&mut self) -> COCPU_TSENS_INT_ENA_W {
        COCPU_TSENS_INT_ENA_W { w: self }
    }
    #[doc = "Bit 6 - RISCV_START_INT interrupt enable bit"]
    #[inline(always)]
    pub fn cocpu_start_int_ena(&mut self) -> COCPU_START_INT_ENA_W {
        COCPU_START_INT_ENA_W { w: self }
    }
    #[doc = "Bit 7 - SW_INT interrupt enable bit"]
    #[inline(always)]
    pub fn cocpu_sw_int_ena(&mut self) -> COCPU_SW_INT_ENA_W {
        COCPU_SW_INT_ENA_W { w: self }
    }
    #[doc = "Bit 8 - SWD_INT interrupt enable bit"]
    #[inline(always)]
    pub fn cocpu_swd_int_ena(&mut self) -> COCPU_SWD_INT_ENA_W {
        COCPU_SWD_INT_ENA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt enable bit of ULP-RISCV\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_cocpu_int_ena](index.html) module"]
pub struct SAR_COCPU_INT_ENA_SPEC;
impl crate::RegisterSpec for SAR_COCPU_INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_cocpu_int_ena::R](R) reader structure"]
impl crate::Readable for SAR_COCPU_INT_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_cocpu_int_ena::W](W) writer structure"]
impl crate::Writable for SAR_COCPU_INT_ENA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAR_COCPU_INT_ENA to value 0"]
impl crate::Resettable for SAR_COCPU_INT_ENA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
