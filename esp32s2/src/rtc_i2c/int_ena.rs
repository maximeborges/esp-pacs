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
#[doc = "Field `SLAVE_TRAN_COMP_INT_ENA` reader - RTC_I2C_SLAVE_TRAN_COMP_INT interrupt enable bit"]
pub struct SLAVE_TRAN_COMP_INT_ENA_R(crate::FieldReader<bool, bool>);
impl SLAVE_TRAN_COMP_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLAVE_TRAN_COMP_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLAVE_TRAN_COMP_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLAVE_TRAN_COMP_INT_ENA` writer - RTC_I2C_SLAVE_TRAN_COMP_INT interrupt enable bit"]
pub struct SLAVE_TRAN_COMP_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_TRAN_COMP_INT_ENA_W<'a> {
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
#[doc = "Field `ARBITRATION_LOST_INT_ENA` reader - RTC_I2C_ARBITRATION_LOST_INT interrupt enable bit"]
pub struct ARBITRATION_LOST_INT_ENA_R(crate::FieldReader<bool, bool>);
impl ARBITRATION_LOST_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ARBITRATION_LOST_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARBITRATION_LOST_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARBITRATION_LOST_INT_ENA` writer - RTC_I2C_ARBITRATION_LOST_INT interrupt enable bit"]
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
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u32 & 1) << 1);
        self.w
    }
}
#[doc = "Field `MASTER_TRAN_COMP_INT_ENA` reader - RTC_I2C_MASTER_TRAN_COMP_INT interrupt enable bit"]
pub struct MASTER_TRAN_COMP_INT_ENA_R(crate::FieldReader<bool, bool>);
impl MASTER_TRAN_COMP_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MASTER_TRAN_COMP_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASTER_TRAN_COMP_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASTER_TRAN_COMP_INT_ENA` writer - RTC_I2C_MASTER_TRAN_COMP_INT interrupt enable bit"]
pub struct MASTER_TRAN_COMP_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> MASTER_TRAN_COMP_INT_ENA_W<'a> {
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
#[doc = "Field `TRANS_COMPLETE_INT_ENA` reader - RTC_I2C_TRANS_COMPLETE_INT interrupt enable bit"]
pub struct TRANS_COMPLETE_INT_ENA_R(crate::FieldReader<bool, bool>);
impl TRANS_COMPLETE_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRANS_COMPLETE_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRANS_COMPLETE_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRANS_COMPLETE_INT_ENA` writer - RTC_I2C_TRANS_COMPLETE_INT interrupt enable bit"]
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
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u32 & 1) << 3);
        self.w
    }
}
#[doc = "Field `TIME_OUT_INT_ENA` reader - RTC_I2C_TIME_OUT_INT interrupt enable bit"]
pub struct TIME_OUT_INT_ENA_R(crate::FieldReader<bool, bool>);
impl TIME_OUT_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIME_OUT_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIME_OUT_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIME_OUT_INT_ENA` writer - RTC_I2C_TIME_OUT_INT interrupt enable bit"]
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
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u32 & 1) << 4);
        self.w
    }
}
#[doc = "Field `ACK_ERR_INT_ENA` reader - RTC_I2C_ACK_ERR_INT interrupt enable bit"]
pub struct ACK_ERR_INT_ENA_R(crate::FieldReader<bool, bool>);
impl ACK_ERR_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ACK_ERR_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACK_ERR_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACK_ERR_INT_ENA` writer - RTC_I2C_ACK_ERR_INT interrupt enable bit"]
pub struct ACK_ERR_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> ACK_ERR_INT_ENA_W<'a> {
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
#[doc = "Field `RX_DATA_INT_ENA` reader - RTC_I2C_RX_DATA_INT interrupt enable bit"]
pub struct RX_DATA_INT_ENA_R(crate::FieldReader<bool, bool>);
impl RX_DATA_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_DATA_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_DATA_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_DATA_INT_ENA` writer - RTC_I2C_RX_DATA_INT interrupt enable bit"]
pub struct RX_DATA_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_DATA_INT_ENA_W<'a> {
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
#[doc = "Field `TX_DATA_INT_ENA` reader - RTC_I2C_TX_DATA_INT interrupt enable bit"]
pub struct TX_DATA_INT_ENA_R(crate::FieldReader<bool, bool>);
impl TX_DATA_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_DATA_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_DATA_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_DATA_INT_ENA` writer - RTC_I2C_TX_DATA_INT interrupt enable bit"]
pub struct TX_DATA_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DATA_INT_ENA_W<'a> {
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
#[doc = "Field `DETECT_START_INT_ENA` reader - RTC_I2C_DETECT_START_INT interrupt enable bit"]
pub struct DETECT_START_INT_ENA_R(crate::FieldReader<bool, bool>);
impl DETECT_START_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DETECT_START_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DETECT_START_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DETECT_START_INT_ENA` writer - RTC_I2C_DETECT_START_INT interrupt enable bit"]
pub struct DETECT_START_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DETECT_START_INT_ENA_W<'a> {
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
    #[doc = "Bit 0 - RTC_I2C_SLAVE_TRAN_COMP_INT interrupt enable bit"]
    #[inline(always)]
    pub fn slave_tran_comp_int_ena(&self) -> SLAVE_TRAN_COMP_INT_ENA_R {
        SLAVE_TRAN_COMP_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RTC_I2C_ARBITRATION_LOST_INT interrupt enable bit"]
    #[inline(always)]
    pub fn arbitration_lost_int_ena(&self) -> ARBITRATION_LOST_INT_ENA_R {
        ARBITRATION_LOST_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RTC_I2C_MASTER_TRAN_COMP_INT interrupt enable bit"]
    #[inline(always)]
    pub fn master_tran_comp_int_ena(&self) -> MASTER_TRAN_COMP_INT_ENA_R {
        MASTER_TRAN_COMP_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RTC_I2C_TRANS_COMPLETE_INT interrupt enable bit"]
    #[inline(always)]
    pub fn trans_complete_int_ena(&self) -> TRANS_COMPLETE_INT_ENA_R {
        TRANS_COMPLETE_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RTC_I2C_TIME_OUT_INT interrupt enable bit"]
    #[inline(always)]
    pub fn time_out_int_ena(&self) -> TIME_OUT_INT_ENA_R {
        TIME_OUT_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RTC_I2C_ACK_ERR_INT interrupt enable bit"]
    #[inline(always)]
    pub fn ack_err_int_ena(&self) -> ACK_ERR_INT_ENA_R {
        ACK_ERR_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RTC_I2C_RX_DATA_INT interrupt enable bit"]
    #[inline(always)]
    pub fn rx_data_int_ena(&self) -> RX_DATA_INT_ENA_R {
        RX_DATA_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RTC_I2C_TX_DATA_INT interrupt enable bit"]
    #[inline(always)]
    pub fn tx_data_int_ena(&self) -> TX_DATA_INT_ENA_R {
        TX_DATA_INT_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RTC_I2C_DETECT_START_INT interrupt enable bit"]
    #[inline(always)]
    pub fn detect_start_int_ena(&self) -> DETECT_START_INT_ENA_R {
        DETECT_START_INT_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTC_I2C_SLAVE_TRAN_COMP_INT interrupt enable bit"]
    #[inline(always)]
    pub fn slave_tran_comp_int_ena(&mut self) -> SLAVE_TRAN_COMP_INT_ENA_W {
        SLAVE_TRAN_COMP_INT_ENA_W { w: self }
    }
    #[doc = "Bit 1 - RTC_I2C_ARBITRATION_LOST_INT interrupt enable bit"]
    #[inline(always)]
    pub fn arbitration_lost_int_ena(&mut self) -> ARBITRATION_LOST_INT_ENA_W {
        ARBITRATION_LOST_INT_ENA_W { w: self }
    }
    #[doc = "Bit 2 - RTC_I2C_MASTER_TRAN_COMP_INT interrupt enable bit"]
    #[inline(always)]
    pub fn master_tran_comp_int_ena(&mut self) -> MASTER_TRAN_COMP_INT_ENA_W {
        MASTER_TRAN_COMP_INT_ENA_W { w: self }
    }
    #[doc = "Bit 3 - RTC_I2C_TRANS_COMPLETE_INT interrupt enable bit"]
    #[inline(always)]
    pub fn trans_complete_int_ena(&mut self) -> TRANS_COMPLETE_INT_ENA_W {
        TRANS_COMPLETE_INT_ENA_W { w: self }
    }
    #[doc = "Bit 4 - RTC_I2C_TIME_OUT_INT interrupt enable bit"]
    #[inline(always)]
    pub fn time_out_int_ena(&mut self) -> TIME_OUT_INT_ENA_W {
        TIME_OUT_INT_ENA_W { w: self }
    }
    #[doc = "Bit 5 - RTC_I2C_ACK_ERR_INT interrupt enable bit"]
    #[inline(always)]
    pub fn ack_err_int_ena(&mut self) -> ACK_ERR_INT_ENA_W {
        ACK_ERR_INT_ENA_W { w: self }
    }
    #[doc = "Bit 6 - RTC_I2C_RX_DATA_INT interrupt enable bit"]
    #[inline(always)]
    pub fn rx_data_int_ena(&mut self) -> RX_DATA_INT_ENA_W {
        RX_DATA_INT_ENA_W { w: self }
    }
    #[doc = "Bit 7 - RTC_I2C_TX_DATA_INT interrupt enable bit"]
    #[inline(always)]
    pub fn tx_data_int_ena(&mut self) -> TX_DATA_INT_ENA_W {
        TX_DATA_INT_ENA_W { w: self }
    }
    #[doc = "Bit 8 - RTC_I2C_DETECT_START_INT interrupt enable bit"]
    #[inline(always)]
    pub fn detect_start_int_ena(&mut self) -> DETECT_START_INT_ENA_W {
        DETECT_START_INT_ENA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enable RTC I2C interrupt\n\nThis register you can [`read`]
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
