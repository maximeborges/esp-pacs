#[doc = "Register `I2S_INT_ENA` reader"]
pub struct R(crate::R<I2S_INT_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2S_INT_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2S_INT_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2S_INT_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2S_INT_ENA` writer"]
pub struct W(crate::W<I2S_INT_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2S_INT_ENA_SPEC>;
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
impl From<crate::W<I2S_INT_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2S_INT_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2S_RX_DONE_INT_ENA` reader - The interrupt enable bit for the i2s_rx_done_int interrupt"]
pub struct I2S_RX_DONE_INT_ENA_R(crate::FieldReader<bool>);
impl I2S_RX_DONE_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2S_RX_DONE_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2S_RX_DONE_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2S_RX_DONE_INT_ENA` writer - The interrupt enable bit for the i2s_rx_done_int interrupt"]
pub struct I2S_RX_DONE_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_RX_DONE_INT_ENA_W<'a> {
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
#[doc = "Field `I2S_TX_DONE_INT_ENA` reader - The interrupt enable bit for the i2s_tx_done_int interrupt"]
pub struct I2S_TX_DONE_INT_ENA_R(crate::FieldReader<bool>);
impl I2S_TX_DONE_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2S_TX_DONE_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2S_TX_DONE_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2S_TX_DONE_INT_ENA` writer - The interrupt enable bit for the i2s_tx_done_int interrupt"]
pub struct I2S_TX_DONE_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_DONE_INT_ENA_W<'a> {
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
#[doc = "Field `I2S_RX_HUNG_INT_ENA` reader - The interrupt enable bit for the i2s_rx_hung_int interrupt"]
pub struct I2S_RX_HUNG_INT_ENA_R(crate::FieldReader<bool>);
impl I2S_RX_HUNG_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2S_RX_HUNG_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2S_RX_HUNG_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2S_RX_HUNG_INT_ENA` writer - The interrupt enable bit for the i2s_rx_hung_int interrupt"]
pub struct I2S_RX_HUNG_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_RX_HUNG_INT_ENA_W<'a> {
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
#[doc = "Field `I2S_TX_HUNG_INT_ENA` reader - The interrupt enable bit for the i2s_tx_hung_int interrupt"]
pub struct I2S_TX_HUNG_INT_ENA_R(crate::FieldReader<bool>);
impl I2S_TX_HUNG_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2S_TX_HUNG_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2S_TX_HUNG_INT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2S_TX_HUNG_INT_ENA` writer - The interrupt enable bit for the i2s_tx_hung_int interrupt"]
pub struct I2S_TX_HUNG_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_HUNG_INT_ENA_W<'a> {
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
impl R {
    #[doc = "Bit 0 - The interrupt enable bit for the i2s_rx_done_int interrupt"]
    #[inline(always)]
    pub fn i2s_rx_done_int_ena(&self) -> I2S_RX_DONE_INT_ENA_R {
        I2S_RX_DONE_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for the i2s_tx_done_int interrupt"]
    #[inline(always)]
    pub fn i2s_tx_done_int_ena(&self) -> I2S_TX_DONE_INT_ENA_R {
        I2S_TX_DONE_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The interrupt enable bit for the i2s_rx_hung_int interrupt"]
    #[inline(always)]
    pub fn i2s_rx_hung_int_ena(&self) -> I2S_RX_HUNG_INT_ENA_R {
        I2S_RX_HUNG_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The interrupt enable bit for the i2s_tx_hung_int interrupt"]
    #[inline(always)]
    pub fn i2s_tx_hung_int_ena(&self) -> I2S_TX_HUNG_INT_ENA_R {
        I2S_TX_HUNG_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The interrupt enable bit for the i2s_rx_done_int interrupt"]
    #[inline(always)]
    pub fn i2s_rx_done_int_ena(&mut self) -> I2S_RX_DONE_INT_ENA_W {
        I2S_RX_DONE_INT_ENA_W { w: self }
    }
    #[doc = "Bit 1 - The interrupt enable bit for the i2s_tx_done_int interrupt"]
    #[inline(always)]
    pub fn i2s_tx_done_int_ena(&mut self) -> I2S_TX_DONE_INT_ENA_W {
        I2S_TX_DONE_INT_ENA_W { w: self }
    }
    #[doc = "Bit 2 - The interrupt enable bit for the i2s_rx_hung_int interrupt"]
    #[inline(always)]
    pub fn i2s_rx_hung_int_ena(&mut self) -> I2S_RX_HUNG_INT_ENA_W {
        I2S_RX_HUNG_INT_ENA_W { w: self }
    }
    #[doc = "Bit 3 - The interrupt enable bit for the i2s_tx_hung_int interrupt"]
    #[inline(always)]
    pub fn i2s_tx_hung_int_ena(&mut self) -> I2S_TX_HUNG_INT_ENA_W {
        I2S_TX_HUNG_INT_ENA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S interrupt enable register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_int_ena](index.html) module"]
pub struct I2S_INT_ENA_SPEC;
impl crate::RegisterSpec for I2S_INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2s_int_ena::R](R) reader structure"]
impl crate::Readable for I2S_INT_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2s_int_ena::W](W) writer structure"]
impl crate::Writable for I2S_INT_ENA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2S_INT_ENA to value 0"]
impl crate::Resettable for I2S_INT_ENA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
