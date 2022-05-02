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
#[doc = "Field `RX_START_INT_CLR` writer - a"]
pub struct RX_START_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_START_INT_CLR_W<'a> {
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
#[doc = "Field `TX_START_INT_CLR` writer - a"]
pub struct TX_START_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_START_INT_CLR_W<'a> {
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
#[doc = "Field `RX_HUNG_INT_CLR` writer - a"]
pub struct RX_HUNG_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_HUNG_INT_CLR_W<'a> {
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
#[doc = "Field `TX_HUNG_INT_CLR` writer - a"]
pub struct TX_HUNG_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_HUNG_INT_CLR_W<'a> {
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
#[doc = "Field `SEND_S_REG_Q_INT_CLR` writer - a"]
pub struct SEND_S_REG_Q_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SEND_S_REG_Q_INT_CLR_W<'a> {
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
#[doc = "Field `SEND_A_REG_Q_INT_CLR` writer - a"]
pub struct SEND_A_REG_Q_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SEND_A_REG_Q_INT_CLR_W<'a> {
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
#[doc = "Field `OUTLINK_EOF_ERR_INT_CLR` writer - a"]
pub struct OUTLINK_EOF_ERR_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTLINK_EOF_ERR_INT_CLR_W<'a> {
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
#[doc = "Field `APP_CTRL0_INT_CLR` writer - a"]
pub struct APP_CTRL0_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_CTRL0_INT_CLR_W<'a> {
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
#[doc = "Field `APP_CTRL1_INT_CLR` writer - a"]
pub struct APP_CTRL1_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_CTRL1_INT_CLR_W<'a> {
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
impl W {
    #[doc = "Bit 0 - a"]
    #[inline(always)]
    pub fn rx_start_int_clr(&mut self) -> RX_START_INT_CLR_W {
        RX_START_INT_CLR_W { w: self }
    }
    #[doc = "Bit 1 - a"]
    #[inline(always)]
    pub fn tx_start_int_clr(&mut self) -> TX_START_INT_CLR_W {
        TX_START_INT_CLR_W { w: self }
    }
    #[doc = "Bit 2 - a"]
    #[inline(always)]
    pub fn rx_hung_int_clr(&mut self) -> RX_HUNG_INT_CLR_W {
        RX_HUNG_INT_CLR_W { w: self }
    }
    #[doc = "Bit 3 - a"]
    #[inline(always)]
    pub fn tx_hung_int_clr(&mut self) -> TX_HUNG_INT_CLR_W {
        TX_HUNG_INT_CLR_W { w: self }
    }
    #[doc = "Bit 4 - a"]
    #[inline(always)]
    pub fn send_s_reg_q_int_clr(&mut self) -> SEND_S_REG_Q_INT_CLR_W {
        SEND_S_REG_Q_INT_CLR_W { w: self }
    }
    #[doc = "Bit 5 - a"]
    #[inline(always)]
    pub fn send_a_reg_q_int_clr(&mut self) -> SEND_A_REG_Q_INT_CLR_W {
        SEND_A_REG_Q_INT_CLR_W { w: self }
    }
    #[doc = "Bit 6 - a"]
    #[inline(always)]
    pub fn outlink_eof_err_int_clr(&mut self) -> OUTLINK_EOF_ERR_INT_CLR_W {
        OUTLINK_EOF_ERR_INT_CLR_W { w: self }
    }
    #[doc = "Bit 7 - a"]
    #[inline(always)]
    pub fn app_ctrl0_int_clr(&mut self) -> APP_CTRL0_INT_CLR_W {
        APP_CTRL0_INT_CLR_W { w: self }
    }
    #[doc = "Bit 8 - a"]
    #[inline(always)]
    pub fn app_ctrl1_int_clr(&mut self) -> APP_CTRL1_INT_CLR_W {
        APP_CTRL1_INT_CLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "a\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_clr](index.html) module"]
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
