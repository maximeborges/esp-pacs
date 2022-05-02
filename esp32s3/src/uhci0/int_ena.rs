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
#[doc = "Field `RX_START_INT_ENA` reader - This is the interrupt enable bit for UHCI_RX_START_INT interrupt."]
pub struct RX_START_INT_ENA_R(crate::FieldReader<bool>);
impl RX_START_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_START_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_START_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_START_INT_ENA` writer - This is the interrupt enable bit for UHCI_RX_START_INT interrupt."]
pub struct RX_START_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_START_INT_ENA_W<'a> {
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
#[doc = "Field `TX_START_INT_ENA` reader - This is the interrupt enable bit for UHCI_TX_START_INT interrupt."]
pub struct TX_START_INT_ENA_R(crate::FieldReader<bool>);
impl TX_START_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_START_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_START_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_START_INT_ENA` writer - This is the interrupt enable bit for UHCI_TX_START_INT interrupt."]
pub struct TX_START_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_START_INT_ENA_W<'a> {
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
#[doc = "Field `RX_HUNG_INT_ENA` reader - This is the interrupt enable bit for UHCI_RX_HUNG_INT interrupt."]
pub struct RX_HUNG_INT_ENA_R(crate::FieldReader<bool>);
impl RX_HUNG_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_HUNG_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_HUNG_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_HUNG_INT_ENA` writer - This is the interrupt enable bit for UHCI_RX_HUNG_INT interrupt."]
pub struct RX_HUNG_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_HUNG_INT_ENA_W<'a> {
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
#[doc = "Field `TX_HUNG_INT_ENA` reader - This is the interrupt enable bit for UHCI_TX_HUNG_INT interrupt."]
pub struct TX_HUNG_INT_ENA_R(crate::FieldReader<bool>);
impl TX_HUNG_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_HUNG_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_HUNG_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_HUNG_INT_ENA` writer - This is the interrupt enable bit for UHCI_TX_HUNG_INT interrupt."]
pub struct TX_HUNG_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_HUNG_INT_ENA_W<'a> {
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
#[doc = "Field `SEND_S_REG_Q_INT_ENA` reader - This is the interrupt enable bit for UHCI_SEND_S_REQ_Q_INT interrupt."]
pub struct SEND_S_REG_Q_INT_ENA_R(crate::FieldReader<bool>);
impl SEND_S_REG_Q_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SEND_S_REG_Q_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEND_S_REG_Q_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEND_S_REG_Q_INT_ENA` writer - This is the interrupt enable bit for UHCI_SEND_S_REQ_Q_INT interrupt."]
pub struct SEND_S_REG_Q_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SEND_S_REG_Q_INT_ENA_W<'a> {
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
#[doc = "Field `SEND_A_REG_Q_INT_ENA` reader - This is the interrupt enable bit for UHCI_SEND_A_REQ_Q_INT interrupt."]
pub struct SEND_A_REG_Q_INT_ENA_R(crate::FieldReader<bool>);
impl SEND_A_REG_Q_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SEND_A_REG_Q_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEND_A_REG_Q_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEND_A_REG_Q_INT_ENA` writer - This is the interrupt enable bit for UHCI_SEND_A_REQ_Q_INT interrupt."]
pub struct SEND_A_REG_Q_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SEND_A_REG_Q_INT_ENA_W<'a> {
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
#[doc = "Field `OUTLINK_EOF_ERR_INT_ENA` reader - This is the interrupt enable bit for UHCI_OUTLINK_EOF_ERR_INT interrupt."]
pub struct OUTLINK_EOF_ERR_INT_ENA_R(crate::FieldReader<bool>);
impl OUTLINK_EOF_ERR_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUTLINK_EOF_ERR_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTLINK_EOF_ERR_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTLINK_EOF_ERR_INT_ENA` writer - This is the interrupt enable bit for UHCI_OUTLINK_EOF_ERR_INT interrupt."]
pub struct OUTLINK_EOF_ERR_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTLINK_EOF_ERR_INT_ENA_W<'a> {
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
#[doc = "Field `APP_CTRL0_INT_ENA` reader - This is the interrupt enable bit for UHCI_APP_CTRL0_INT interrupt."]
pub struct APP_CTRL0_INT_ENA_R(crate::FieldReader<bool>);
impl APP_CTRL0_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APP_CTRL0_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APP_CTRL0_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APP_CTRL0_INT_ENA` writer - This is the interrupt enable bit for UHCI_APP_CTRL0_INT interrupt."]
pub struct APP_CTRL0_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_CTRL0_INT_ENA_W<'a> {
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
#[doc = "Field `APP_CTRL1_INT_ENA` reader - This is the interrupt enable bit for UHCI_APP_CTRL1_INT interrupt."]
pub struct APP_CTRL1_INT_ENA_R(crate::FieldReader<bool>);
impl APP_CTRL1_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APP_CTRL1_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APP_CTRL1_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APP_CTRL1_INT_ENA` writer - This is the interrupt enable bit for UHCI_APP_CTRL1_INT interrupt."]
pub struct APP_CTRL1_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_CTRL1_INT_ENA_W<'a> {
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
    #[doc = "Bit 0 - This is the interrupt enable bit for UHCI_RX_START_INT interrupt."]
    #[inline(always)]
    pub fn rx_start_int_ena(&self) -> RX_START_INT_ENA_R {
        RX_START_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This is the interrupt enable bit for UHCI_TX_START_INT interrupt."]
    #[inline(always)]
    pub fn tx_start_int_ena(&self) -> TX_START_INT_ENA_R {
        TX_START_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This is the interrupt enable bit for UHCI_RX_HUNG_INT interrupt."]
    #[inline(always)]
    pub fn rx_hung_int_ena(&self) -> RX_HUNG_INT_ENA_R {
        RX_HUNG_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This is the interrupt enable bit for UHCI_TX_HUNG_INT interrupt."]
    #[inline(always)]
    pub fn tx_hung_int_ena(&self) -> TX_HUNG_INT_ENA_R {
        TX_HUNG_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This is the interrupt enable bit for UHCI_SEND_S_REQ_Q_INT interrupt."]
    #[inline(always)]
    pub fn send_s_reg_q_int_ena(&self) -> SEND_S_REG_Q_INT_ENA_R {
        SEND_S_REG_Q_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This is the interrupt enable bit for UHCI_SEND_A_REQ_Q_INT interrupt."]
    #[inline(always)]
    pub fn send_a_reg_q_int_ena(&self) -> SEND_A_REG_Q_INT_ENA_R {
        SEND_A_REG_Q_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This is the interrupt enable bit for UHCI_OUTLINK_EOF_ERR_INT interrupt."]
    #[inline(always)]
    pub fn outlink_eof_err_int_ena(&self) -> OUTLINK_EOF_ERR_INT_ENA_R {
        OUTLINK_EOF_ERR_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This is the interrupt enable bit for UHCI_APP_CTRL0_INT interrupt."]
    #[inline(always)]
    pub fn app_ctrl0_int_ena(&self) -> APP_CTRL0_INT_ENA_R {
        APP_CTRL0_INT_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This is the interrupt enable bit for UHCI_APP_CTRL1_INT interrupt."]
    #[inline(always)]
    pub fn app_ctrl1_int_ena(&self) -> APP_CTRL1_INT_ENA_R {
        APP_CTRL1_INT_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This is the interrupt enable bit for UHCI_RX_START_INT interrupt."]
    #[inline(always)]
    pub fn rx_start_int_ena(&mut self) -> RX_START_INT_ENA_W {
        RX_START_INT_ENA_W { w: self }
    }
    #[doc = "Bit 1 - This is the interrupt enable bit for UHCI_TX_START_INT interrupt."]
    #[inline(always)]
    pub fn tx_start_int_ena(&mut self) -> TX_START_INT_ENA_W {
        TX_START_INT_ENA_W { w: self }
    }
    #[doc = "Bit 2 - This is the interrupt enable bit for UHCI_RX_HUNG_INT interrupt."]
    #[inline(always)]
    pub fn rx_hung_int_ena(&mut self) -> RX_HUNG_INT_ENA_W {
        RX_HUNG_INT_ENA_W { w: self }
    }
    #[doc = "Bit 3 - This is the interrupt enable bit for UHCI_TX_HUNG_INT interrupt."]
    #[inline(always)]
    pub fn tx_hung_int_ena(&mut self) -> TX_HUNG_INT_ENA_W {
        TX_HUNG_INT_ENA_W { w: self }
    }
    #[doc = "Bit 4 - This is the interrupt enable bit for UHCI_SEND_S_REQ_Q_INT interrupt."]
    #[inline(always)]
    pub fn send_s_reg_q_int_ena(&mut self) -> SEND_S_REG_Q_INT_ENA_W {
        SEND_S_REG_Q_INT_ENA_W { w: self }
    }
    #[doc = "Bit 5 - This is the interrupt enable bit for UHCI_SEND_A_REQ_Q_INT interrupt."]
    #[inline(always)]
    pub fn send_a_reg_q_int_ena(&mut self) -> SEND_A_REG_Q_INT_ENA_W {
        SEND_A_REG_Q_INT_ENA_W { w: self }
    }
    #[doc = "Bit 6 - This is the interrupt enable bit for UHCI_OUTLINK_EOF_ERR_INT interrupt."]
    #[inline(always)]
    pub fn outlink_eof_err_int_ena(&mut self) -> OUTLINK_EOF_ERR_INT_ENA_W {
        OUTLINK_EOF_ERR_INT_ENA_W { w: self }
    }
    #[doc = "Bit 7 - This is the interrupt enable bit for UHCI_APP_CTRL0_INT interrupt."]
    #[inline(always)]
    pub fn app_ctrl0_int_ena(&mut self) -> APP_CTRL0_INT_ENA_W {
        APP_CTRL0_INT_ENA_W { w: self }
    }
    #[doc = "Bit 8 - This is the interrupt enable bit for UHCI_APP_CTRL1_INT interrupt."]
    #[inline(always)]
    pub fn app_ctrl1_int_ena(&mut self) -> APP_CTRL1_INT_ENA_W {
        APP_CTRL1_INT_ENA_W { w: self }
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
