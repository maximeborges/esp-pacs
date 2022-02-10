#[doc = "Register `EP1_CONF` reader"]
pub struct R(crate::R<EP1_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EP1_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EP1_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EP1_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EP1_CONF` writer"]
pub struct W(crate::W<EP1_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EP1_CONF_SPEC>;
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
impl From<crate::W<EP1_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EP1_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WR_DONE` writer - Set this bit to indicate writing byte data to UART Tx FIFO is done."]
pub struct WR_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_DONE_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `SERIAL_IN_EP_DATA_FREE` reader - 1'b1: Indicate UART Tx FIFO is not full and can write data into in. After writing USB_DEVICE_WR_DONE, this bit would be 0 until data in UART Tx FIFO is read by USB Host."]
pub struct SERIAL_IN_EP_DATA_FREE_R(crate::FieldReader<bool, bool>);
impl SERIAL_IN_EP_DATA_FREE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SERIAL_IN_EP_DATA_FREE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SERIAL_IN_EP_DATA_FREE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SERIAL_OUT_EP_DATA_AVAIL` reader - 1'b1: Indicate there is data in UART Rx FIFO."]
pub struct SERIAL_OUT_EP_DATA_AVAIL_R(crate::FieldReader<bool, bool>);
impl SERIAL_OUT_EP_DATA_AVAIL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SERIAL_OUT_EP_DATA_AVAIL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SERIAL_OUT_EP_DATA_AVAIL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 1 - 1'b1: Indicate UART Tx FIFO is not full and can write data into in. After writing USB_DEVICE_WR_DONE, this bit would be 0 until data in UART Tx FIFO is read by USB Host."]
    #[inline(always)]
    pub fn serial_in_ep_data_free(&self) -> SERIAL_IN_EP_DATA_FREE_R {
        SERIAL_IN_EP_DATA_FREE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 1'b1: Indicate there is data in UART Rx FIFO."]
    #[inline(always)]
    pub fn serial_out_ep_data_avail(&self) -> SERIAL_OUT_EP_DATA_AVAIL_R {
        SERIAL_OUT_EP_DATA_AVAIL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to indicate writing byte data to UART Tx FIFO is done."]
    #[inline(always)]
    pub fn wr_done(&mut self) -> WR_DONE_W {
        WR_DONE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB_DEVICE_EP1_CONF_REG.\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep1_conf]
(index.html) module"]
pub struct EP1_CONF_SPEC;
impl crate::RegisterSpec for EP1_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ep1_conf::R]
(R) reader structure"]
impl crate::Readable for EP1_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ep1_conf::W]
(W) writer structure"]
impl crate::Writable for EP1_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EP1_CONF to value 0x02"]
impl crate::Resettable for EP1_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
