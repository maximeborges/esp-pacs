#[doc = "Register `TEST_CONF` reader"]
pub struct R(crate::R<TEST_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TEST_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TEST_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TEST_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TEST_CONF` writer"]
pub struct W(crate::W<TEST_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TEST_CONF_SPEC>;
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
impl From<crate::W<TEST_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TEST_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TEST_ENABLE` reader - Enable test of the USB pad"]
pub struct TEST_ENABLE_R(crate::FieldReader<bool, bool>);
impl TEST_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TEST_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEST_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEST_ENABLE` writer - Enable test of the USB pad"]
pub struct TEST_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> TEST_ENABLE_W<'a> {
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
#[doc = "Field `TEST_USB_OE` reader - USB pad oen in test"]
pub struct TEST_USB_OE_R(crate::FieldReader<bool, bool>);
impl TEST_USB_OE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TEST_USB_OE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEST_USB_OE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEST_USB_OE` writer - USB pad oen in test"]
pub struct TEST_USB_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> TEST_USB_OE_W<'a> {
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
#[doc = "Field `TEST_TX_DP` reader - USB D+ tx value in test"]
pub struct TEST_TX_DP_R(crate::FieldReader<bool, bool>);
impl TEST_TX_DP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TEST_TX_DP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEST_TX_DP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEST_TX_DP` writer - USB D+ tx value in test"]
pub struct TEST_TX_DP_W<'a> {
    w: &'a mut W,
}
impl<'a> TEST_TX_DP_W<'a> {
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
#[doc = "Field `TEST_TX_DM` reader - USB D- tx value in test"]
pub struct TEST_TX_DM_R(crate::FieldReader<bool, bool>);
impl TEST_TX_DM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TEST_TX_DM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEST_TX_DM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEST_TX_DM` writer - USB D- tx value in test"]
pub struct TEST_TX_DM_W<'a> {
    w: &'a mut W,
}
impl<'a> TEST_TX_DM_W<'a> {
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
#[doc = "Field `TEST_RX_RCV` reader - USB differential rx value in test"]
pub struct TEST_RX_RCV_R(crate::FieldReader<bool, bool>);
impl TEST_RX_RCV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TEST_RX_RCV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEST_RX_RCV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEST_RX_DP` reader - USB D+ rx value in test"]
pub struct TEST_RX_DP_R(crate::FieldReader<bool, bool>);
impl TEST_RX_DP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TEST_RX_DP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEST_RX_DP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEST_RX_DM` reader - USB D- rx value in test"]
pub struct TEST_RX_DM_R(crate::FieldReader<bool, bool>);
impl TEST_RX_DM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TEST_RX_DM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEST_RX_DM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Enable test of the USB pad"]
    #[inline(always)]
    pub fn test_enable(&self) -> TEST_ENABLE_R {
        TEST_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USB pad oen in test"]
    #[inline(always)]
    pub fn test_usb_oe(&self) -> TEST_USB_OE_R {
        TEST_USB_OE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USB D+ tx value in test"]
    #[inline(always)]
    pub fn test_tx_dp(&self) -> TEST_TX_DP_R {
        TEST_TX_DP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - USB D- tx value in test"]
    #[inline(always)]
    pub fn test_tx_dm(&self) -> TEST_TX_DM_R {
        TEST_TX_DM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - USB differential rx value in test"]
    #[inline(always)]
    pub fn test_rx_rcv(&self) -> TEST_RX_RCV_R {
        TEST_RX_RCV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USB D+ rx value in test"]
    #[inline(always)]
    pub fn test_rx_dp(&self) -> TEST_RX_DP_R {
        TEST_RX_DP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - USB D- rx value in test"]
    #[inline(always)]
    pub fn test_rx_dm(&self) -> TEST_RX_DM_R {
        TEST_RX_DM_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable test of the USB pad"]
    #[inline(always)]
    pub fn test_enable(&mut self) -> TEST_ENABLE_W {
        TEST_ENABLE_W { w: self }
    }
    #[doc = "Bit 1 - USB pad oen in test"]
    #[inline(always)]
    pub fn test_usb_oe(&mut self) -> TEST_USB_OE_W {
        TEST_USB_OE_W { w: self }
    }
    #[doc = "Bit 2 - USB D+ tx value in test"]
    #[inline(always)]
    pub fn test_tx_dp(&mut self) -> TEST_TX_DP_W {
        TEST_TX_DP_W { w: self }
    }
    #[doc = "Bit 3 - USB D- tx value in test"]
    #[inline(always)]
    pub fn test_tx_dm(&mut self) -> TEST_TX_DM_W {
        TEST_TX_DM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Internal PHY Testing Register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [test_conf]
(index.html) module"]
pub struct TEST_CONF_SPEC;
impl crate::RegisterSpec for TEST_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [test_conf::R]
(R) reader structure"]
impl crate::Readable for TEST_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [test_conf::W]
(W) writer structure"]
impl crate::Writable for TEST_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TEST_CONF to value 0"]
impl crate::Resettable for TEST_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
