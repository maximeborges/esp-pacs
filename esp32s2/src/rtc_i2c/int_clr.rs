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
#[doc = "Field `SLAVE_TRAN_COMP_INT_CLR` writer - RTC_I2C_SLAVE_TRAN_COMP_INT interrupt clear bit"]
pub struct SLAVE_TRAN_COMP_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_TRAN_COMP_INT_CLR_W<'a> {
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
#[doc = "Field `ARBITRATION_LOST_INT_CLR` writer - RTC_I2C_ARBITRATION_LOST_INT interrupt clear bit"]
pub struct ARBITRATION_LOST_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> ARBITRATION_LOST_INT_CLR_W<'a> {
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
#[doc = "Field `MASTER_TRAN_COMP_INT_CLR` writer - RTC_I2C_MASTER_TRAN_COMP_INT interrupt clear bit"]
pub struct MASTER_TRAN_COMP_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> MASTER_TRAN_COMP_INT_CLR_W<'a> {
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
#[doc = "Field `TRANS_COMPLETE_INT_CLR` writer - RTC_I2C_TRANS_COMPLETE_INT interrupt clear bit"]
pub struct TRANS_COMPLETE_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TRANS_COMPLETE_INT_CLR_W<'a> {
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
#[doc = "Field `TIME_OUT_INT_CLR` writer - RTC_I2C_TIME_OUT_INT interrupt clear bit"]
pub struct TIME_OUT_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TIME_OUT_INT_CLR_W<'a> {
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
#[doc = "Field `ACK_ERR_INT_CLR` writer - RTC_I2C_ACK_ERR_INT interrupt clear bit"]
pub struct ACK_ERR_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> ACK_ERR_INT_CLR_W<'a> {
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
#[doc = "Field `RX_DATA_INT_CLR` writer - RTC_I2C_RX_DATA_INT interrupt clear bit"]
pub struct RX_DATA_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_DATA_INT_CLR_W<'a> {
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
#[doc = "Field `TX_DATA_INT_CLR` writer - RTC_I2C_TX_DATA_INT interrupt clear bit"]
pub struct TX_DATA_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DATA_INT_CLR_W<'a> {
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
#[doc = "Field `DETECT_START_INT_CLR` writer - RTC_I2C_DETECT_START_INT interrupt clear bit"]
pub struct DETECT_START_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DETECT_START_INT_CLR_W<'a> {
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
    #[doc = "Bit 0 - RTC_I2C_SLAVE_TRAN_COMP_INT interrupt clear bit"]
    #[inline(always)]
    pub fn slave_tran_comp_int_clr(&mut self) -> SLAVE_TRAN_COMP_INT_CLR_W {
        SLAVE_TRAN_COMP_INT_CLR_W { w: self }
    }
    #[doc = "Bit 1 - RTC_I2C_ARBITRATION_LOST_INT interrupt clear bit"]
    #[inline(always)]
    pub fn arbitration_lost_int_clr(&mut self) -> ARBITRATION_LOST_INT_CLR_W {
        ARBITRATION_LOST_INT_CLR_W { w: self }
    }
    #[doc = "Bit 2 - RTC_I2C_MASTER_TRAN_COMP_INT interrupt clear bit"]
    #[inline(always)]
    pub fn master_tran_comp_int_clr(&mut self) -> MASTER_TRAN_COMP_INT_CLR_W {
        MASTER_TRAN_COMP_INT_CLR_W { w: self }
    }
    #[doc = "Bit 3 - RTC_I2C_TRANS_COMPLETE_INT interrupt clear bit"]
    #[inline(always)]
    pub fn trans_complete_int_clr(&mut self) -> TRANS_COMPLETE_INT_CLR_W {
        TRANS_COMPLETE_INT_CLR_W { w: self }
    }
    #[doc = "Bit 4 - RTC_I2C_TIME_OUT_INT interrupt clear bit"]
    #[inline(always)]
    pub fn time_out_int_clr(&mut self) -> TIME_OUT_INT_CLR_W {
        TIME_OUT_INT_CLR_W { w: self }
    }
    #[doc = "Bit 5 - RTC_I2C_ACK_ERR_INT interrupt clear bit"]
    #[inline(always)]
    pub fn ack_err_int_clr(&mut self) -> ACK_ERR_INT_CLR_W {
        ACK_ERR_INT_CLR_W { w: self }
    }
    #[doc = "Bit 6 - RTC_I2C_RX_DATA_INT interrupt clear bit"]
    #[inline(always)]
    pub fn rx_data_int_clr(&mut self) -> RX_DATA_INT_CLR_W {
        RX_DATA_INT_CLR_W { w: self }
    }
    #[doc = "Bit 7 - RTC_I2C_TX_DATA_INT interrupt clear bit"]
    #[inline(always)]
    pub fn tx_data_int_clr(&mut self) -> TX_DATA_INT_CLR_W {
        TX_DATA_INT_CLR_W { w: self }
    }
    #[doc = "Bit 8 - RTC_I2C_DETECT_START_INT interrupt clear bit"]
    #[inline(always)]
    pub fn detect_start_int_clr(&mut self) -> DETECT_START_INT_CLR_W {
        DETECT_START_INT_CLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clear RTC I2C interrupt\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_clr](index.html) module"]
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
