#[doc = "Register `INT_ENA` reader"]
pub struct R(crate::R<INT_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_ENA` writer"]
pub struct W(crate::W<INT_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_ENA_SPEC>;
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
impl From<crate::W<INT_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXFIFO_FULL_INT_ENA` reader - This is the enable bit for UART_RXFIFO_FULL_INT."]
pub type RXFIFO_FULL_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `RXFIFO_FULL_INT_ENA` writer - This is the enable bit for UART_RXFIFO_FULL_INT."]
pub type RXFIFO_FULL_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 0>;
#[doc = "Field `TXFIFO_EMPTY_INT_ENA` reader - This is the enable bit for UART_TXFIFO_EMPTY_INT."]
pub type TXFIFO_EMPTY_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `TXFIFO_EMPTY_INT_ENA` writer - This is the enable bit for UART_TXFIFO_EMPTY_INT."]
pub type TXFIFO_EMPTY_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 1>;
#[doc = "Field `PARITY_ERR_INT_ENA` reader - This is the enable bit for UART_PARITY_ERR_INT."]
pub type PARITY_ERR_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `PARITY_ERR_INT_ENA` writer - This is the enable bit for UART_PARITY_ERR_INT."]
pub type PARITY_ERR_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 2>;
#[doc = "Field `FRM_ERR_INT_ENA` reader - This is the enable bit for UART_FRM_ERR_INT."]
pub type FRM_ERR_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `FRM_ERR_INT_ENA` writer - This is the enable bit for UART_FRM_ERR_INT."]
pub type FRM_ERR_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 3>;
#[doc = "Field `RXFIFO_OVF_INT_ENA` reader - This is the enable bit for UART_RXFIFO_OVF_INT."]
pub type RXFIFO_OVF_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `RXFIFO_OVF_INT_ENA` writer - This is the enable bit for UART_RXFIFO_OVF_INT."]
pub type RXFIFO_OVF_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 4>;
#[doc = "Field `DSR_CHG_INT_ENA` reader - This is the enable bit for UART_DSR_CHG_INT."]
pub type DSR_CHG_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `DSR_CHG_INT_ENA` writer - This is the enable bit for UART_DSR_CHG_INT."]
pub type DSR_CHG_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 5>;
#[doc = "Field `CTS_CHG_INT_ENA` reader - This is the enable bit for UART_CTS_CHG_INT."]
pub type CTS_CHG_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `CTS_CHG_INT_ENA` writer - This is the enable bit for UART_CTS_CHG_INT."]
pub type CTS_CHG_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 6>;
#[doc = "Field `BRK_DET_INT_ENA` reader - This is the enable bit for UART_BRK_DET_INT."]
pub type BRK_DET_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `BRK_DET_INT_ENA` writer - This is the enable bit for UART_BRK_DET_INT."]
pub type BRK_DET_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 7>;
#[doc = "Field `RXFIFO_TOUT_INT_ENA` reader - This is the enable bit for UART_RXFIFO_TOUT_INT."]
pub type RXFIFO_TOUT_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `RXFIFO_TOUT_INT_ENA` writer - This is the enable bit for UART_RXFIFO_TOUT_INT."]
pub type RXFIFO_TOUT_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 8>;
#[doc = "Field `SW_XON_INT_ENA` reader - This is the enable bit for UART_SW_XON_INT."]
pub type SW_XON_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `SW_XON_INT_ENA` writer - This is the enable bit for UART_SW_XON_INT."]
pub type SW_XON_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 9>;
#[doc = "Field `SW_XOFF_INT_ENA` reader - This is the enable bit for UART_SW_XOFF_INT."]
pub type SW_XOFF_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `SW_XOFF_INT_ENA` writer - This is the enable bit for UART_SW_XOFF_INT."]
pub type SW_XOFF_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 10>;
#[doc = "Field `GLITCH_DET_INT_ENA` reader - This is the enable bit for UART_GLITCH_DET_INT."]
pub type GLITCH_DET_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `GLITCH_DET_INT_ENA` writer - This is the enable bit for UART_GLITCH_DET_INT."]
pub type GLITCH_DET_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 11>;
#[doc = "Field `TX_BRK_DONE_INT_ENA` reader - This is the enable bit for UART_TX_BRK_DONE_INT."]
pub type TX_BRK_DONE_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `TX_BRK_DONE_INT_ENA` writer - This is the enable bit for UART_TX_BRK_DONE_INT."]
pub type TX_BRK_DONE_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 12>;
#[doc = "Field `TX_BRK_IDLE_DONE_INT_ENA` reader - This is the enable bit for UART_TX_BRK_IDLE_DONE_INT."]
pub type TX_BRK_IDLE_DONE_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `TX_BRK_IDLE_DONE_INT_ENA` writer - This is the enable bit for UART_TX_BRK_IDLE_DONE_INT."]
pub type TX_BRK_IDLE_DONE_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 13>;
#[doc = "Field `TX_DONE_INT_ENA` reader - This is the enable bit for UART_TX_DONE_INT."]
pub type TX_DONE_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `TX_DONE_INT_ENA` writer - This is the enable bit for UART_TX_DONE_INT."]
pub type TX_DONE_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 14>;
#[doc = "Field `RS485_PARITY_ERR_INT_ENA` reader - This is the enable bit for UART_RS485_PARITY_ERR_INT."]
pub type RS485_PARITY_ERR_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `RS485_PARITY_ERR_INT_ENA` writer - This is the enable bit for UART_RS485_PARITY_ERR_INT."]
pub type RS485_PARITY_ERR_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 15>;
#[doc = "Field `RS485_FRM_ERR_INT_ENA` reader - This is the enable bit for UART_RS485_PARITY_ERR_INT."]
pub type RS485_FRM_ERR_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `RS485_FRM_ERR_INT_ENA` writer - This is the enable bit for UART_RS485_PARITY_ERR_INT."]
pub type RS485_FRM_ERR_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 16>;
#[doc = "Field `RS485_CLASH_INT_ENA` reader - This is the enable bit for UART_RS485_CLASH_INT."]
pub type RS485_CLASH_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `RS485_CLASH_INT_ENA` writer - This is the enable bit for UART_RS485_CLASH_INT."]
pub type RS485_CLASH_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 17>;
#[doc = "Field `AT_CMD_CHAR_DET_INT_ENA` reader - This is the enable bit for UART_AT_CMD_CHAR_DET_INT."]
pub type AT_CMD_CHAR_DET_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `AT_CMD_CHAR_DET_INT_ENA` writer - This is the enable bit for UART_AT_CMD_CHAR_DET_INT."]
pub type AT_CMD_CHAR_DET_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 18>;
#[doc = "Field `WAKEUP_INT_ENA` reader - This is the enable bit for UART_WAKEUP_INT."]
pub type WAKEUP_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `WAKEUP_INT_ENA` writer - This is the enable bit for UART_WAKEUP_INT."]
pub type WAKEUP_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 19>;
impl R {
    #[doc = "Bit 0 - This is the enable bit for UART_RXFIFO_FULL_INT."]
    #[inline(always)]
    pub fn rxfifo_full_int_ena(&self) -> RXFIFO_FULL_INT_ENA_R {
        RXFIFO_FULL_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This is the enable bit for UART_TXFIFO_EMPTY_INT."]
    #[inline(always)]
    pub fn txfifo_empty_int_ena(&self) -> TXFIFO_EMPTY_INT_ENA_R {
        TXFIFO_EMPTY_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This is the enable bit for UART_PARITY_ERR_INT."]
    #[inline(always)]
    pub fn parity_err_int_ena(&self) -> PARITY_ERR_INT_ENA_R {
        PARITY_ERR_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This is the enable bit for UART_FRM_ERR_INT."]
    #[inline(always)]
    pub fn frm_err_int_ena(&self) -> FRM_ERR_INT_ENA_R {
        FRM_ERR_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This is the enable bit for UART_RXFIFO_OVF_INT."]
    #[inline(always)]
    pub fn rxfifo_ovf_int_ena(&self) -> RXFIFO_OVF_INT_ENA_R {
        RXFIFO_OVF_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This is the enable bit for UART_DSR_CHG_INT."]
    #[inline(always)]
    pub fn dsr_chg_int_ena(&self) -> DSR_CHG_INT_ENA_R {
        DSR_CHG_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This is the enable bit for UART_CTS_CHG_INT."]
    #[inline(always)]
    pub fn cts_chg_int_ena(&self) -> CTS_CHG_INT_ENA_R {
        CTS_CHG_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This is the enable bit for UART_BRK_DET_INT."]
    #[inline(always)]
    pub fn brk_det_int_ena(&self) -> BRK_DET_INT_ENA_R {
        BRK_DET_INT_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This is the enable bit for UART_RXFIFO_TOUT_INT."]
    #[inline(always)]
    pub fn rxfifo_tout_int_ena(&self) -> RXFIFO_TOUT_INT_ENA_R {
        RXFIFO_TOUT_INT_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - This is the enable bit for UART_SW_XON_INT."]
    #[inline(always)]
    pub fn sw_xon_int_ena(&self) -> SW_XON_INT_ENA_R {
        SW_XON_INT_ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - This is the enable bit for UART_SW_XOFF_INT."]
    #[inline(always)]
    pub fn sw_xoff_int_ena(&self) -> SW_XOFF_INT_ENA_R {
        SW_XOFF_INT_ENA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - This is the enable bit for UART_GLITCH_DET_INT."]
    #[inline(always)]
    pub fn glitch_det_int_ena(&self) -> GLITCH_DET_INT_ENA_R {
        GLITCH_DET_INT_ENA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - This is the enable bit for UART_TX_BRK_DONE_INT."]
    #[inline(always)]
    pub fn tx_brk_done_int_ena(&self) -> TX_BRK_DONE_INT_ENA_R {
        TX_BRK_DONE_INT_ENA_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - This is the enable bit for UART_TX_BRK_IDLE_DONE_INT."]
    #[inline(always)]
    pub fn tx_brk_idle_done_int_ena(&self) -> TX_BRK_IDLE_DONE_INT_ENA_R {
        TX_BRK_IDLE_DONE_INT_ENA_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - This is the enable bit for UART_TX_DONE_INT."]
    #[inline(always)]
    pub fn tx_done_int_ena(&self) -> TX_DONE_INT_ENA_R {
        TX_DONE_INT_ENA_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - This is the enable bit for UART_RS485_PARITY_ERR_INT."]
    #[inline(always)]
    pub fn rs485_parity_err_int_ena(&self) -> RS485_PARITY_ERR_INT_ENA_R {
        RS485_PARITY_ERR_INT_ENA_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - This is the enable bit for UART_RS485_PARITY_ERR_INT."]
    #[inline(always)]
    pub fn rs485_frm_err_int_ena(&self) -> RS485_FRM_ERR_INT_ENA_R {
        RS485_FRM_ERR_INT_ENA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - This is the enable bit for UART_RS485_CLASH_INT."]
    #[inline(always)]
    pub fn rs485_clash_int_ena(&self) -> RS485_CLASH_INT_ENA_R {
        RS485_CLASH_INT_ENA_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - This is the enable bit for UART_AT_CMD_CHAR_DET_INT."]
    #[inline(always)]
    pub fn at_cmd_char_det_int_ena(&self) -> AT_CMD_CHAR_DET_INT_ENA_R {
        AT_CMD_CHAR_DET_INT_ENA_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - This is the enable bit for UART_WAKEUP_INT."]
    #[inline(always)]
    pub fn wakeup_int_ena(&self) -> WAKEUP_INT_ENA_R {
        WAKEUP_INT_ENA_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This is the enable bit for UART_RXFIFO_FULL_INT."]
    #[inline(always)]
    pub fn rxfifo_full_int_ena(&mut self) -> RXFIFO_FULL_INT_ENA_W {
        RXFIFO_FULL_INT_ENA_W::new(self)
    }
    #[doc = "Bit 1 - This is the enable bit for UART_TXFIFO_EMPTY_INT."]
    #[inline(always)]
    pub fn txfifo_empty_int_ena(&mut self) -> TXFIFO_EMPTY_INT_ENA_W {
        TXFIFO_EMPTY_INT_ENA_W::new(self)
    }
    #[doc = "Bit 2 - This is the enable bit for UART_PARITY_ERR_INT."]
    #[inline(always)]
    pub fn parity_err_int_ena(&mut self) -> PARITY_ERR_INT_ENA_W {
        PARITY_ERR_INT_ENA_W::new(self)
    }
    #[doc = "Bit 3 - This is the enable bit for UART_FRM_ERR_INT."]
    #[inline(always)]
    pub fn frm_err_int_ena(&mut self) -> FRM_ERR_INT_ENA_W {
        FRM_ERR_INT_ENA_W::new(self)
    }
    #[doc = "Bit 4 - This is the enable bit for UART_RXFIFO_OVF_INT."]
    #[inline(always)]
    pub fn rxfifo_ovf_int_ena(&mut self) -> RXFIFO_OVF_INT_ENA_W {
        RXFIFO_OVF_INT_ENA_W::new(self)
    }
    #[doc = "Bit 5 - This is the enable bit for UART_DSR_CHG_INT."]
    #[inline(always)]
    pub fn dsr_chg_int_ena(&mut self) -> DSR_CHG_INT_ENA_W {
        DSR_CHG_INT_ENA_W::new(self)
    }
    #[doc = "Bit 6 - This is the enable bit for UART_CTS_CHG_INT."]
    #[inline(always)]
    pub fn cts_chg_int_ena(&mut self) -> CTS_CHG_INT_ENA_W {
        CTS_CHG_INT_ENA_W::new(self)
    }
    #[doc = "Bit 7 - This is the enable bit for UART_BRK_DET_INT."]
    #[inline(always)]
    pub fn brk_det_int_ena(&mut self) -> BRK_DET_INT_ENA_W {
        BRK_DET_INT_ENA_W::new(self)
    }
    #[doc = "Bit 8 - This is the enable bit for UART_RXFIFO_TOUT_INT."]
    #[inline(always)]
    pub fn rxfifo_tout_int_ena(&mut self) -> RXFIFO_TOUT_INT_ENA_W {
        RXFIFO_TOUT_INT_ENA_W::new(self)
    }
    #[doc = "Bit 9 - This is the enable bit for UART_SW_XON_INT."]
    #[inline(always)]
    pub fn sw_xon_int_ena(&mut self) -> SW_XON_INT_ENA_W {
        SW_XON_INT_ENA_W::new(self)
    }
    #[doc = "Bit 10 - This is the enable bit for UART_SW_XOFF_INT."]
    #[inline(always)]
    pub fn sw_xoff_int_ena(&mut self) -> SW_XOFF_INT_ENA_W {
        SW_XOFF_INT_ENA_W::new(self)
    }
    #[doc = "Bit 11 - This is the enable bit for UART_GLITCH_DET_INT."]
    #[inline(always)]
    pub fn glitch_det_int_ena(&mut self) -> GLITCH_DET_INT_ENA_W {
        GLITCH_DET_INT_ENA_W::new(self)
    }
    #[doc = "Bit 12 - This is the enable bit for UART_TX_BRK_DONE_INT."]
    #[inline(always)]
    pub fn tx_brk_done_int_ena(&mut self) -> TX_BRK_DONE_INT_ENA_W {
        TX_BRK_DONE_INT_ENA_W::new(self)
    }
    #[doc = "Bit 13 - This is the enable bit for UART_TX_BRK_IDLE_DONE_INT."]
    #[inline(always)]
    pub fn tx_brk_idle_done_int_ena(&mut self) -> TX_BRK_IDLE_DONE_INT_ENA_W {
        TX_BRK_IDLE_DONE_INT_ENA_W::new(self)
    }
    #[doc = "Bit 14 - This is the enable bit for UART_TX_DONE_INT."]
    #[inline(always)]
    pub fn tx_done_int_ena(&mut self) -> TX_DONE_INT_ENA_W {
        TX_DONE_INT_ENA_W::new(self)
    }
    #[doc = "Bit 15 - This is the enable bit for UART_RS485_PARITY_ERR_INT."]
    #[inline(always)]
    pub fn rs485_parity_err_int_ena(&mut self) -> RS485_PARITY_ERR_INT_ENA_W {
        RS485_PARITY_ERR_INT_ENA_W::new(self)
    }
    #[doc = "Bit 16 - This is the enable bit for UART_RS485_PARITY_ERR_INT."]
    #[inline(always)]
    pub fn rs485_frm_err_int_ena(&mut self) -> RS485_FRM_ERR_INT_ENA_W {
        RS485_FRM_ERR_INT_ENA_W::new(self)
    }
    #[doc = "Bit 17 - This is the enable bit for UART_RS485_CLASH_INT."]
    #[inline(always)]
    pub fn rs485_clash_int_ena(&mut self) -> RS485_CLASH_INT_ENA_W {
        RS485_CLASH_INT_ENA_W::new(self)
    }
    #[doc = "Bit 18 - This is the enable bit for UART_AT_CMD_CHAR_DET_INT."]
    #[inline(always)]
    pub fn at_cmd_char_det_int_ena(&mut self) -> AT_CMD_CHAR_DET_INT_ENA_W {
        AT_CMD_CHAR_DET_INT_ENA_W::new(self)
    }
    #[doc = "Bit 19 - This is the enable bit for UART_WAKEUP_INT."]
    #[inline(always)]
    pub fn wakeup_int_ena(&mut self) -> WAKEUP_INT_ENA_W {
        WAKEUP_INT_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt enable bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_ena](index.html) module"]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_ena::R](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_ena::W](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
