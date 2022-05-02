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
#[doc = "Field `JTAG_IN_FLUSH_INT_RAW` reader - The raw interrupt bit turns to high level when flush cmd is received for IN endpoint 2 of JTAG."]
pub struct JTAG_IN_FLUSH_INT_RAW_R(crate::FieldReader<bool>);
impl JTAG_IN_FLUSH_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        JTAG_IN_FLUSH_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JTAG_IN_FLUSH_INT_RAW_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOF_INT_RAW` reader - The raw interrupt bit turns to high level when SOF frame is received."]
pub struct SOF_INT_RAW_R(crate::FieldReader<bool>);
impl SOF_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SOF_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOF_INT_RAW_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SERIAL_OUT_RECV_PKT_INT_RAW` reader - The raw interrupt bit turns to high level when Serial Port OUT Endpoint received one packet."]
pub struct SERIAL_OUT_RECV_PKT_INT_RAW_R(crate::FieldReader<bool>);
impl SERIAL_OUT_RECV_PKT_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SERIAL_OUT_RECV_PKT_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SERIAL_OUT_RECV_PKT_INT_RAW_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SERIAL_IN_EMPTY_INT_RAW` reader - The raw interrupt bit turns to high level when Serial Port IN Endpoint is empty."]
pub struct SERIAL_IN_EMPTY_INT_RAW_R(crate::FieldReader<bool>);
impl SERIAL_IN_EMPTY_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SERIAL_IN_EMPTY_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SERIAL_IN_EMPTY_INT_RAW_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID_ERR_INT_RAW` reader - The raw interrupt bit turns to high level when pid error is detected."]
pub struct PID_ERR_INT_RAW_R(crate::FieldReader<bool>);
impl PID_ERR_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PID_ERR_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PID_ERR_INT_RAW_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRC5_ERR_INT_RAW` reader - The raw interrupt bit turns to high level when CRC5 error is detected."]
pub struct CRC5_ERR_INT_RAW_R(crate::FieldReader<bool>);
impl CRC5_ERR_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CRC5_ERR_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRC5_ERR_INT_RAW_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRC16_ERR_INT_RAW` reader - The raw interrupt bit turns to high level when CRC16 error is detected."]
pub struct CRC16_ERR_INT_RAW_R(crate::FieldReader<bool>);
impl CRC16_ERR_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CRC16_ERR_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRC16_ERR_INT_RAW_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STUFF_ERR_INT_RAW` reader - The raw interrupt bit turns to high level when stuff error is detected."]
pub struct STUFF_ERR_INT_RAW_R(crate::FieldReader<bool>);
impl STUFF_ERR_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STUFF_ERR_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STUFF_ERR_INT_RAW_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN_TOKEN_REC_IN_EP1_INT_RAW` reader - The raw interrupt bit turns to high level when IN token for IN endpoint 1 is received."]
pub struct IN_TOKEN_REC_IN_EP1_INT_RAW_R(crate::FieldReader<bool>);
impl IN_TOKEN_REC_IN_EP1_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IN_TOKEN_REC_IN_EP1_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN_TOKEN_REC_IN_EP1_INT_RAW_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB_BUS_RESET_INT_RAW` reader - The raw interrupt bit turns to high level when usb bus reset is detected."]
pub struct USB_BUS_RESET_INT_RAW_R(crate::FieldReader<bool>);
impl USB_BUS_RESET_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USB_BUS_RESET_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_BUS_RESET_INT_RAW_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_EP1_ZERO_PAYLOAD_INT_RAW` reader - The raw interrupt bit turns to high level when OUT endpoint 1 received packet with zero palyload."]
pub struct OUT_EP1_ZERO_PAYLOAD_INT_RAW_R(crate::FieldReader<bool>);
impl OUT_EP1_ZERO_PAYLOAD_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUT_EP1_ZERO_PAYLOAD_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_EP1_ZERO_PAYLOAD_INT_RAW_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_EP2_ZERO_PAYLOAD_INT_RAW` reader - The raw interrupt bit turns to high level when OUT endpoint 2 received packet with zero palyload."]
pub struct OUT_EP2_ZERO_PAYLOAD_INT_RAW_R(crate::FieldReader<bool>);
impl OUT_EP2_ZERO_PAYLOAD_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUT_EP2_ZERO_PAYLOAD_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_EP2_ZERO_PAYLOAD_INT_RAW_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - The raw interrupt bit turns to high level when flush cmd is received for IN endpoint 2 of JTAG."]
    #[inline(always)]
    pub fn jtag_in_flush_int_raw(&self) -> JTAG_IN_FLUSH_INT_RAW_R {
        JTAG_IN_FLUSH_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt bit turns to high level when SOF frame is received."]
    #[inline(always)]
    pub fn sof_int_raw(&self) -> SOF_INT_RAW_R {
        SOF_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt bit turns to high level when Serial Port OUT Endpoint received one packet."]
    #[inline(always)]
    pub fn serial_out_recv_pkt_int_raw(&self) -> SERIAL_OUT_RECV_PKT_INT_RAW_R {
        SERIAL_OUT_RECV_PKT_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw interrupt bit turns to high level when Serial Port IN Endpoint is empty."]
    #[inline(always)]
    pub fn serial_in_empty_int_raw(&self) -> SERIAL_IN_EMPTY_INT_RAW_R {
        SERIAL_IN_EMPTY_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The raw interrupt bit turns to high level when pid error is detected."]
    #[inline(always)]
    pub fn pid_err_int_raw(&self) -> PID_ERR_INT_RAW_R {
        PID_ERR_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The raw interrupt bit turns to high level when CRC5 error is detected."]
    #[inline(always)]
    pub fn crc5_err_int_raw(&self) -> CRC5_ERR_INT_RAW_R {
        CRC5_ERR_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The raw interrupt bit turns to high level when CRC16 error is detected."]
    #[inline(always)]
    pub fn crc16_err_int_raw(&self) -> CRC16_ERR_INT_RAW_R {
        CRC16_ERR_INT_RAW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The raw interrupt bit turns to high level when stuff error is detected."]
    #[inline(always)]
    pub fn stuff_err_int_raw(&self) -> STUFF_ERR_INT_RAW_R {
        STUFF_ERR_INT_RAW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The raw interrupt bit turns to high level when IN token for IN endpoint 1 is received."]
    #[inline(always)]
    pub fn in_token_rec_in_ep1_int_raw(&self) -> IN_TOKEN_REC_IN_EP1_INT_RAW_R {
        IN_TOKEN_REC_IN_EP1_INT_RAW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The raw interrupt bit turns to high level when usb bus reset is detected."]
    #[inline(always)]
    pub fn usb_bus_reset_int_raw(&self) -> USB_BUS_RESET_INT_RAW_R {
        USB_BUS_RESET_INT_RAW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The raw interrupt bit turns to high level when OUT endpoint 1 received packet with zero palyload."]
    #[inline(always)]
    pub fn out_ep1_zero_payload_int_raw(&self) -> OUT_EP1_ZERO_PAYLOAD_INT_RAW_R {
        OUT_EP1_ZERO_PAYLOAD_INT_RAW_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The raw interrupt bit turns to high level when OUT endpoint 2 received packet with zero palyload."]
    #[inline(always)]
    pub fn out_ep2_zero_payload_int_raw(&self) -> OUT_EP2_ZERO_PAYLOAD_INT_RAW_R {
        OUT_EP2_ZERO_PAYLOAD_INT_RAW_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "Raw status interrupt\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_raw](index.html) module"]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_raw::R](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INT_RAW to value 0x08"]
impl crate::Resettable for INT_RAW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x08
    }
}
