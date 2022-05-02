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
#[doc = "Register `INT_RAW` writer"]
pub struct W(crate::W<INT_RAW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_RAW_SPEC>;
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
impl From<crate::W<INT_RAW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_RAW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_START_INT_RAW` reader - This is the interrupt raw bit. Triggered when a separator char has been sent."]
pub struct RX_START_INT_RAW_R(crate::FieldReader<bool>);
impl RX_START_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_START_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_START_INT_RAW_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_START_INT_RAW` reader - This is the interrupt raw bit. Triggered when UHCI detects a separator char."]
pub struct TX_START_INT_RAW_R(crate::FieldReader<bool>);
impl TX_START_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_START_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_START_INT_RAW_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_HUNG_INT_RAW` reader - This is the interrupt raw bit. Triggered when UHCI takes more time to receive data than configure value."]
pub struct RX_HUNG_INT_RAW_R(crate::FieldReader<bool>);
impl RX_HUNG_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_HUNG_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_HUNG_INT_RAW_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_HUNG_INT_RAW` reader - This is the interrupt raw bit. Triggered when UHCI takes more time to read data from RAM than the configured value."]
pub struct TX_HUNG_INT_RAW_R(crate::FieldReader<bool>);
impl TX_HUNG_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_HUNG_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_HUNG_INT_RAW_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEND_S_REG_Q_INT_RAW` reader - This is the interrupt raw bit. Triggered when UHCI has sent out a short packet using single_send registers."]
pub struct SEND_S_REG_Q_INT_RAW_R(crate::FieldReader<bool>);
impl SEND_S_REG_Q_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SEND_S_REG_Q_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEND_S_REG_Q_INT_RAW_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEND_A_REG_Q_INT_RAW` reader - This is the interrupt raw bit. Triggered when UHCI has sent out a short packet using always_send registers."]
pub struct SEND_A_REG_Q_INT_RAW_R(crate::FieldReader<bool>);
impl SEND_A_REG_Q_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SEND_A_REG_Q_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEND_A_REG_Q_INT_RAW_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_EOF_INT_RAW` reader - This is the interrupt raw bit. Triggered when there are some errors in EOF in the transmit data."]
pub struct OUT_EOF_INT_RAW_R(crate::FieldReader<bool>);
impl OUT_EOF_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUT_EOF_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_EOF_INT_RAW_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APP_CTRL0_INT_RAW` reader - This is the interrupt raw bit. Triggered when set UHCI_APP_CTRL0_IN_SET."]
pub struct APP_CTRL0_INT_RAW_R(crate::FieldReader<bool>);
impl APP_CTRL0_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APP_CTRL0_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APP_CTRL0_INT_RAW_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APP_CTRL0_INT_RAW` writer - This is the interrupt raw bit. Triggered when set UHCI_APP_CTRL0_IN_SET."]
pub struct APP_CTRL0_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_CTRL0_INT_RAW_W<'a> {
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
#[doc = "Field `APP_CTRL1_INT_RAW` reader - This is the interrupt raw bit. Triggered when set UHCI_APP_CTRL1_IN_SET."]
pub struct APP_CTRL1_INT_RAW_R(crate::FieldReader<bool>);
impl APP_CTRL1_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APP_CTRL1_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APP_CTRL1_INT_RAW_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APP_CTRL1_INT_RAW` writer - This is the interrupt raw bit. Triggered when set UHCI_APP_CTRL1_IN_SET."]
pub struct APP_CTRL1_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_CTRL1_INT_RAW_W<'a> {
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
    #[doc = "Bit 0 - This is the interrupt raw bit. Triggered when a separator char has been sent."]
    #[inline(always)]
    pub fn rx_start_int_raw(&self) -> RX_START_INT_RAW_R {
        RX_START_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This is the interrupt raw bit. Triggered when UHCI detects a separator char."]
    #[inline(always)]
    pub fn tx_start_int_raw(&self) -> TX_START_INT_RAW_R {
        TX_START_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This is the interrupt raw bit. Triggered when UHCI takes more time to receive data than configure value."]
    #[inline(always)]
    pub fn rx_hung_int_raw(&self) -> RX_HUNG_INT_RAW_R {
        RX_HUNG_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This is the interrupt raw bit. Triggered when UHCI takes more time to read data from RAM than the configured value."]
    #[inline(always)]
    pub fn tx_hung_int_raw(&self) -> TX_HUNG_INT_RAW_R {
        TX_HUNG_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This is the interrupt raw bit. Triggered when UHCI has sent out a short packet using single_send registers."]
    #[inline(always)]
    pub fn send_s_reg_q_int_raw(&self) -> SEND_S_REG_Q_INT_RAW_R {
        SEND_S_REG_Q_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This is the interrupt raw bit. Triggered when UHCI has sent out a short packet using always_send registers."]
    #[inline(always)]
    pub fn send_a_reg_q_int_raw(&self) -> SEND_A_REG_Q_INT_RAW_R {
        SEND_A_REG_Q_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This is the interrupt raw bit. Triggered when there are some errors in EOF in the transmit data."]
    #[inline(always)]
    pub fn out_eof_int_raw(&self) -> OUT_EOF_INT_RAW_R {
        OUT_EOF_INT_RAW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This is the interrupt raw bit. Triggered when set UHCI_APP_CTRL0_IN_SET."]
    #[inline(always)]
    pub fn app_ctrl0_int_raw(&self) -> APP_CTRL0_INT_RAW_R {
        APP_CTRL0_INT_RAW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This is the interrupt raw bit. Triggered when set UHCI_APP_CTRL1_IN_SET."]
    #[inline(always)]
    pub fn app_ctrl1_int_raw(&self) -> APP_CTRL1_INT_RAW_R {
        APP_CTRL1_INT_RAW_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - This is the interrupt raw bit. Triggered when set UHCI_APP_CTRL0_IN_SET."]
    #[inline(always)]
    pub fn app_ctrl0_int_raw(&mut self) -> APP_CTRL0_INT_RAW_W {
        APP_CTRL0_INT_RAW_W { w: self }
    }
    #[doc = "Bit 8 - This is the interrupt raw bit. Triggered when set UHCI_APP_CTRL1_IN_SET."]
    #[inline(always)]
    pub fn app_ctrl1_int_raw(&mut self) -> APP_CTRL1_INT_RAW_W {
        APP_CTRL1_INT_RAW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Raw interrupt status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_raw](index.html) module"]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_raw::R](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_raw::W](W) writer structure"]
impl crate::Writable for INT_RAW_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
