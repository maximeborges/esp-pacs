#[doc = "Register `EP1` reader"]
pub struct R(crate::R<EP1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EP1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EP1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EP1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EP1` writer"]
pub struct W(crate::W<EP1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EP1_SPEC>;
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
impl From<crate::W<EP1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EP1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDWR_BYTE` reader - Write and read byte data to/from UART Tx/Rx FIFO through this field. When USB_DEVICE_SERIAL_IN_EMPTY_INT is set, then user can write data (up to 64 bytes) into UART Tx FIFO. When USB_DEVICE_SERIAL_OUT_RECV_PKT_INT is set, user can check USB_DEVICE_OUT_EP1_WR_ADDR USB_DEVICE_OUT_EP0_RD_ADDR to know how many data is received, then read data from UART Rx FIFO."]
pub struct RDWR_BYTE_R(crate::FieldReader<u8>);
impl RDWR_BYTE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RDWR_BYTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDWR_BYTE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDWR_BYTE` writer - Write and read byte data to/from UART Tx/Rx FIFO through this field. When USB_DEVICE_SERIAL_IN_EMPTY_INT is set, then user can write data (up to 64 bytes) into UART Tx FIFO. When USB_DEVICE_SERIAL_OUT_RECV_PKT_INT is set, user can check USB_DEVICE_OUT_EP1_WR_ADDR USB_DEVICE_OUT_EP0_RD_ADDR to know how many data is received, then read data from UART Rx FIFO."]
pub struct RDWR_BYTE_W<'a> {
    w: &'a mut W,
}
impl<'a> RDWR_BYTE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Write and read byte data to/from UART Tx/Rx FIFO through this field. When USB_DEVICE_SERIAL_IN_EMPTY_INT is set, then user can write data (up to 64 bytes) into UART Tx FIFO. When USB_DEVICE_SERIAL_OUT_RECV_PKT_INT is set, user can check USB_DEVICE_OUT_EP1_WR_ADDR USB_DEVICE_OUT_EP0_RD_ADDR to know how many data is received, then read data from UART Rx FIFO."]
    #[inline(always)]
    pub fn rdwr_byte(&self) -> RDWR_BYTE_R {
        RDWR_BYTE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Write and read byte data to/from UART Tx/Rx FIFO through this field. When USB_DEVICE_SERIAL_IN_EMPTY_INT is set, then user can write data (up to 64 bytes) into UART Tx FIFO. When USB_DEVICE_SERIAL_OUT_RECV_PKT_INT is set, user can check USB_DEVICE_OUT_EP1_WR_ADDR USB_DEVICE_OUT_EP0_RD_ADDR to know how many data is received, then read data from UART Rx FIFO."]
    #[inline(always)]
    pub fn rdwr_byte(&mut self) -> RDWR_BYTE_W {
        RDWR_BYTE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB_DEVICE_EP1_REG.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep1](index.html) module"]
pub struct EP1_SPEC;
impl crate::RegisterSpec for EP1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ep1::R](R) reader structure"]
impl crate::Readable for EP1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ep1::W](W) writer structure"]
impl crate::Writable for EP1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EP1 to value 0"]
impl crate::Resettable for EP1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
