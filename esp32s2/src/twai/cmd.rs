#[doc = "Register `CMD` writer"]
pub struct W(crate::W<CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD_SPEC>;
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
impl From<crate::W<CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_REQ` writer - Set the bit to 1 to allow the driving nodes start transmission."]
pub struct TX_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_REQ_W<'a> {
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
#[doc = "Field `ABORT_TX` writer - Set the bit to 1 to cancel a pending transmission request."]
pub struct ABORT_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> ABORT_TX_W<'a> {
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
#[doc = "Field `RELEASE_BUF` writer - Set the bit to 1 to release the RX buffer."]
pub struct RELEASE_BUF_W<'a> {
    w: &'a mut W,
}
impl<'a> RELEASE_BUF_W<'a> {
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
#[doc = "Field `CLR_OVERRUN` writer - Set the bit to 1 to clear the data overrun status bit."]
pub struct CLR_OVERRUN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_OVERRUN_W<'a> {
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
#[doc = "Field `SELF_RX_REQ` writer - Self reception request command. Set the bit to 1 to allow a message be transmitted and received simultaneously."]
pub struct SELF_RX_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SELF_RX_REQ_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Set the bit to 1 to allow the driving nodes start transmission."]
    #[inline(always)]
    pub fn tx_req(&mut self) -> TX_REQ_W {
        TX_REQ_W { w: self }
    }
    #[doc = "Bit 1 - Set the bit to 1 to cancel a pending transmission request."]
    #[inline(always)]
    pub fn abort_tx(&mut self) -> ABORT_TX_W {
        ABORT_TX_W { w: self }
    }
    #[doc = "Bit 2 - Set the bit to 1 to release the RX buffer."]
    #[inline(always)]
    pub fn release_buf(&mut self) -> RELEASE_BUF_W {
        RELEASE_BUF_W { w: self }
    }
    #[doc = "Bit 3 - Set the bit to 1 to clear the data overrun status bit."]
    #[inline(always)]
    pub fn clr_overrun(&mut self) -> CLR_OVERRUN_W {
        CLR_OVERRUN_W { w: self }
    }
    #[doc = "Bit 4 - Self reception request command. Set the bit to 1 to allow a message be transmitted and received simultaneously."]
    #[inline(always)]
    pub fn self_rx_req(&mut self) -> SELF_RX_REQ_W {
        SELF_RX_REQ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Command Register\n\nThis register you can [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd]
(index.html) module"]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cmd::W]
(W) writer structure"]
impl crate::Writable for CMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
