#[doc = "Register `INT_CLR` writer"]
pub struct W(crate::W<INT_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_CLR_SPEC>;
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
impl From<crate::W<INT_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `JTAG_IN_FLUSH_INT_CLR` writer - Set this bit to clear the USB_DEVICE_JTAG_IN_FLUSH_INT interrupt."]
pub struct JTAG_IN_FLUSH_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> JTAG_IN_FLUSH_INT_CLR_W<'a> {
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
#[doc = "Field `SOF_INT_CLR` writer - Set this bit to clear the USB_DEVICE_JTAG_SOF_INT interrupt."]
pub struct SOF_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SOF_INT_CLR_W<'a> {
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
#[doc = "Field `SERIAL_OUT_RECV_PKT_INT_CLR` writer - Set this bit to clear the USB_DEVICE_SERIAL_OUT_RECV_PKT_INT interrupt."]
pub struct SERIAL_OUT_RECV_PKT_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SERIAL_OUT_RECV_PKT_INT_CLR_W<'a> {
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
#[doc = "Field `SERIAL_IN_EMPTY_INT_CLR` writer - Set this bit to clear the USB_DEVICE_SERIAL_IN_EMPTY_INT interrupt."]
pub struct SERIAL_IN_EMPTY_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SERIAL_IN_EMPTY_INT_CLR_W<'a> {
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
#[doc = "Field `PID_ERR_INT_CLR` writer - Set this bit to clear the USB_DEVICE_PID_ERR_INT interrupt."]
pub struct PID_ERR_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> PID_ERR_INT_CLR_W<'a> {
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
#[doc = "Field `CRC5_ERR_INT_CLR` writer - Set this bit to clear the USB_DEVICE_CRC5_ERR_INT interrupt."]
pub struct CRC5_ERR_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC5_ERR_INT_CLR_W<'a> {
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
#[doc = "Field `CRC16_ERR_INT_CLR` writer - Set this bit to clear the USB_DEVICE_CRC16_ERR_INT interrupt."]
pub struct CRC16_ERR_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC16_ERR_INT_CLR_W<'a> {
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
#[doc = "Field `STUFF_ERR_INT_CLR` writer - Set this bit to clear the USB_DEVICE_STUFF_ERR_INT interrupt."]
pub struct STUFF_ERR_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> STUFF_ERR_INT_CLR_W<'a> {
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
#[doc = "Field `IN_TOKEN_REC_IN_EP1_INT_CLR` writer - Set this bit to clear the USB_DEVICE_IN_TOKEN_IN_EP1_INT interrupt."]
pub struct IN_TOKEN_REC_IN_EP1_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_TOKEN_REC_IN_EP1_INT_CLR_W<'a> {
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
#[doc = "Field `USB_BUS_RESET_INT_CLR` writer - Set this bit to clear the USB_DEVICE_USB_BUS_RESET_INT interrupt."]
pub struct USB_BUS_RESET_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_BUS_RESET_INT_CLR_W<'a> {
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
#[doc = "Field `OUT_EP1_ZERO_PAYLOAD_INT_CLR` writer - Set this bit to clear the USB_DEVICE_OUT_EP1_ZERO_PAYLOAD_INT interrupt."]
pub struct OUT_EP1_ZERO_PAYLOAD_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_EP1_ZERO_PAYLOAD_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 10)) | ((value as u32 & 1) << 10);
        self.w
    }
}
#[doc = "Field `OUT_EP2_ZERO_PAYLOAD_INT_CLR` writer - Set this bit to clear the USB_DEVICE_OUT_EP2_ZERO_PAYLOAD_INT interrupt."]
pub struct OUT_EP2_ZERO_PAYLOAD_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_EP2_ZERO_PAYLOAD_INT_CLR_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Set this bit to clear the USB_DEVICE_JTAG_IN_FLUSH_INT interrupt."]
    #[inline(always)]
    pub fn jtag_in_flush_int_clr(&mut self) -> JTAG_IN_FLUSH_INT_CLR_W {
        JTAG_IN_FLUSH_INT_CLR_W { w: self }
    }
    #[doc = "Bit 1 - Set this bit to clear the USB_DEVICE_JTAG_SOF_INT interrupt."]
    #[inline(always)]
    pub fn sof_int_clr(&mut self) -> SOF_INT_CLR_W {
        SOF_INT_CLR_W { w: self }
    }
    #[doc = "Bit 2 - Set this bit to clear the USB_DEVICE_SERIAL_OUT_RECV_PKT_INT interrupt."]
    #[inline(always)]
    pub fn serial_out_recv_pkt_int_clr(&mut self) -> SERIAL_OUT_RECV_PKT_INT_CLR_W {
        SERIAL_OUT_RECV_PKT_INT_CLR_W { w: self }
    }
    #[doc = "Bit 3 - Set this bit to clear the USB_DEVICE_SERIAL_IN_EMPTY_INT interrupt."]
    #[inline(always)]
    pub fn serial_in_empty_int_clr(&mut self) -> SERIAL_IN_EMPTY_INT_CLR_W {
        SERIAL_IN_EMPTY_INT_CLR_W { w: self }
    }
    #[doc = "Bit 4 - Set this bit to clear the USB_DEVICE_PID_ERR_INT interrupt."]
    #[inline(always)]
    pub fn pid_err_int_clr(&mut self) -> PID_ERR_INT_CLR_W {
        PID_ERR_INT_CLR_W { w: self }
    }
    #[doc = "Bit 5 - Set this bit to clear the USB_DEVICE_CRC5_ERR_INT interrupt."]
    #[inline(always)]
    pub fn crc5_err_int_clr(&mut self) -> CRC5_ERR_INT_CLR_W {
        CRC5_ERR_INT_CLR_W { w: self }
    }
    #[doc = "Bit 6 - Set this bit to clear the USB_DEVICE_CRC16_ERR_INT interrupt."]
    #[inline(always)]
    pub fn crc16_err_int_clr(&mut self) -> CRC16_ERR_INT_CLR_W {
        CRC16_ERR_INT_CLR_W { w: self }
    }
    #[doc = "Bit 7 - Set this bit to clear the USB_DEVICE_STUFF_ERR_INT interrupt."]
    #[inline(always)]
    pub fn stuff_err_int_clr(&mut self) -> STUFF_ERR_INT_CLR_W {
        STUFF_ERR_INT_CLR_W { w: self }
    }
    #[doc = "Bit 8 - Set this bit to clear the USB_DEVICE_IN_TOKEN_IN_EP1_INT interrupt."]
    #[inline(always)]
    pub fn in_token_rec_in_ep1_int_clr(&mut self) -> IN_TOKEN_REC_IN_EP1_INT_CLR_W {
        IN_TOKEN_REC_IN_EP1_INT_CLR_W { w: self }
    }
    #[doc = "Bit 9 - Set this bit to clear the USB_DEVICE_USB_BUS_RESET_INT interrupt."]
    #[inline(always)]
    pub fn usb_bus_reset_int_clr(&mut self) -> USB_BUS_RESET_INT_CLR_W {
        USB_BUS_RESET_INT_CLR_W { w: self }
    }
    #[doc = "Bit 10 - Set this bit to clear the USB_DEVICE_OUT_EP1_ZERO_PAYLOAD_INT interrupt."]
    #[inline(always)]
    pub fn out_ep1_zero_payload_int_clr(&mut self) -> OUT_EP1_ZERO_PAYLOAD_INT_CLR_W {
        OUT_EP1_ZERO_PAYLOAD_INT_CLR_W { w: self }
    }
    #[doc = "Bit 11 - Set this bit to clear the USB_DEVICE_OUT_EP2_ZERO_PAYLOAD_INT interrupt."]
    #[inline(always)]
    pub fn out_ep2_zero_payload_int_clr(&mut self) -> OUT_EP2_ZERO_PAYLOAD_INT_CLR_W {
        OUT_EP2_ZERO_PAYLOAD_INT_CLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB_DEVICE_INT_CLR_REG.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_clr](index.html) module"]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [int_clr::W](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
