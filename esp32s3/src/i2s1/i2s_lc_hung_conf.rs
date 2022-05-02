#[doc = "Register `I2S_LC_HUNG_CONF` reader"]
pub struct R(crate::R<I2S_LC_HUNG_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2S_LC_HUNG_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2S_LC_HUNG_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2S_LC_HUNG_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2S_LC_HUNG_CONF` writer"]
pub struct W(crate::W<I2S_LC_HUNG_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2S_LC_HUNG_CONF_SPEC>;
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
impl From<crate::W<I2S_LC_HUNG_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2S_LC_HUNG_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2S_LC_FIFO_TIMEOUT` reader - the i2s_tx_hung_int interrupt or the i2s_rx_hung_int interrupt will be triggered when fifo hung counter is equal to this value"]
pub struct I2S_LC_FIFO_TIMEOUT_R(crate::FieldReader<u8>);
impl I2S_LC_FIFO_TIMEOUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        I2S_LC_FIFO_TIMEOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2S_LC_FIFO_TIMEOUT_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2S_LC_FIFO_TIMEOUT` writer - the i2s_tx_hung_int interrupt or the i2s_rx_hung_int interrupt will be triggered when fifo hung counter is equal to this value"]
pub struct I2S_LC_FIFO_TIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_LC_FIFO_TIMEOUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `I2S_LC_FIFO_TIMEOUT_SHIFT` reader - The bits are used to scale tick counter threshold. The tick counter is reset when counter value >= 88000/2^i2s_lc_fifo_timeout_shift"]
pub struct I2S_LC_FIFO_TIMEOUT_SHIFT_R(crate::FieldReader<u8>);
impl I2S_LC_FIFO_TIMEOUT_SHIFT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        I2S_LC_FIFO_TIMEOUT_SHIFT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2S_LC_FIFO_TIMEOUT_SHIFT_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2S_LC_FIFO_TIMEOUT_SHIFT` writer - The bits are used to scale tick counter threshold. The tick counter is reset when counter value >= 88000/2^i2s_lc_fifo_timeout_shift"]
pub struct I2S_LC_FIFO_TIMEOUT_SHIFT_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_LC_FIFO_TIMEOUT_SHIFT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 8)) | ((value as u32 & 7) << 8);
        self.w
    }
}
#[doc = "Field `I2S_LC_FIFO_TIMEOUT_ENA` reader - The enable bit for FIFO timeout"]
pub struct I2S_LC_FIFO_TIMEOUT_ENA_R(crate::FieldReader<bool>);
impl I2S_LC_FIFO_TIMEOUT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2S_LC_FIFO_TIMEOUT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2S_LC_FIFO_TIMEOUT_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2S_LC_FIFO_TIMEOUT_ENA` writer - The enable bit for FIFO timeout"]
pub struct I2S_LC_FIFO_TIMEOUT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_LC_FIFO_TIMEOUT_ENA_W<'a> {
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
impl R {
    #[doc = "Bits 0:7 - the i2s_tx_hung_int interrupt or the i2s_rx_hung_int interrupt will be triggered when fifo hung counter is equal to this value"]
    #[inline(always)]
    pub fn i2s_lc_fifo_timeout(&self) -> I2S_LC_FIFO_TIMEOUT_R {
        I2S_LC_FIFO_TIMEOUT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - The bits are used to scale tick counter threshold. The tick counter is reset when counter value >= 88000/2^i2s_lc_fifo_timeout_shift"]
    #[inline(always)]
    pub fn i2s_lc_fifo_timeout_shift(&self) -> I2S_LC_FIFO_TIMEOUT_SHIFT_R {
        I2S_LC_FIFO_TIMEOUT_SHIFT_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - The enable bit for FIFO timeout"]
    #[inline(always)]
    pub fn i2s_lc_fifo_timeout_ena(&self) -> I2S_LC_FIFO_TIMEOUT_ENA_R {
        I2S_LC_FIFO_TIMEOUT_ENA_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - the i2s_tx_hung_int interrupt or the i2s_rx_hung_int interrupt will be triggered when fifo hung counter is equal to this value"]
    #[inline(always)]
    pub fn i2s_lc_fifo_timeout(&mut self) -> I2S_LC_FIFO_TIMEOUT_W {
        I2S_LC_FIFO_TIMEOUT_W { w: self }
    }
    #[doc = "Bits 8:10 - The bits are used to scale tick counter threshold. The tick counter is reset when counter value >= 88000/2^i2s_lc_fifo_timeout_shift"]
    #[inline(always)]
    pub fn i2s_lc_fifo_timeout_shift(&mut self) -> I2S_LC_FIFO_TIMEOUT_SHIFT_W {
        I2S_LC_FIFO_TIMEOUT_SHIFT_W { w: self }
    }
    #[doc = "Bit 11 - The enable bit for FIFO timeout"]
    #[inline(always)]
    pub fn i2s_lc_fifo_timeout_ena(&mut self) -> I2S_LC_FIFO_TIMEOUT_ENA_W {
        I2S_LC_FIFO_TIMEOUT_ENA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S HUNG configure register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_lc_hung_conf](index.html) module"]
pub struct I2S_LC_HUNG_CONF_SPEC;
impl crate::RegisterSpec for I2S_LC_HUNG_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2s_lc_hung_conf::R](R) reader structure"]
impl crate::Readable for I2S_LC_HUNG_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2s_lc_hung_conf::W](W) writer structure"]
impl crate::Writable for I2S_LC_HUNG_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2S_LC_HUNG_CONF to value 0x0810"]
impl crate::Resettable for I2S_LC_HUNG_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0810
    }
}
