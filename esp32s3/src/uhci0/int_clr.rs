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
#[doc = "Field `RX_START_INT_CLR` writer - Set this bit to clear UHCI_RX_START_INT interrupt."]
pub type RX_START_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 0>;
#[doc = "Field `TX_START_INT_CLR` writer - Set this bit to clear UHCI_TX_START_INT interrupt."]
pub type TX_START_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 1>;
#[doc = "Field `RX_HUNG_INT_CLR` writer - Set this bit to clear UHCI_RX_HUNG_INT interrupt."]
pub type RX_HUNG_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 2>;
#[doc = "Field `TX_HUNG_INT_CLR` writer - Set this bit to clear UHCI_TX_HUNG_INT interrupt."]
pub type TX_HUNG_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 3>;
#[doc = "Field `SEND_S_REG_Q_INT_CLR` writer - Set this bit to clear UHCI_SEND_S_REQ_Q_INT interrupt."]
pub type SEND_S_REG_Q_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 4>;
#[doc = "Field `SEND_A_REG_Q_INT_CLR` writer - Set this bit to clear UHCI_SEND_A_REQ_Q_INT interrupt."]
pub type SEND_A_REG_Q_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 5>;
#[doc = "Field `OUTLINK_EOF_ERR_INT_CLR` writer - Set this bit to clear UHCI_OUTLINK_EOF_ERR_INT interrupt."]
pub type OUTLINK_EOF_ERR_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 6>;
#[doc = "Field `APP_CTRL0_INT_CLR` writer - Set this bit to clear UHCI_APP_CTRL0_INT interrupt."]
pub type APP_CTRL0_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 7>;
#[doc = "Field `APP_CTRL1_INT_CLR` writer - Set this bit to clear UHCI_APP_CTRL1_INT interrupt."]
pub type APP_CTRL1_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 8>;
impl W {
    #[doc = "Bit 0 - Set this bit to clear UHCI_RX_START_INT interrupt."]
    #[inline(always)]
    pub fn rx_start_int_clr(&mut self) -> RX_START_INT_CLR_W {
        RX_START_INT_CLR_W::new(self)
    }
    #[doc = "Bit 1 - Set this bit to clear UHCI_TX_START_INT interrupt."]
    #[inline(always)]
    pub fn tx_start_int_clr(&mut self) -> TX_START_INT_CLR_W {
        TX_START_INT_CLR_W::new(self)
    }
    #[doc = "Bit 2 - Set this bit to clear UHCI_RX_HUNG_INT interrupt."]
    #[inline(always)]
    pub fn rx_hung_int_clr(&mut self) -> RX_HUNG_INT_CLR_W {
        RX_HUNG_INT_CLR_W::new(self)
    }
    #[doc = "Bit 3 - Set this bit to clear UHCI_TX_HUNG_INT interrupt."]
    #[inline(always)]
    pub fn tx_hung_int_clr(&mut self) -> TX_HUNG_INT_CLR_W {
        TX_HUNG_INT_CLR_W::new(self)
    }
    #[doc = "Bit 4 - Set this bit to clear UHCI_SEND_S_REQ_Q_INT interrupt."]
    #[inline(always)]
    pub fn send_s_reg_q_int_clr(&mut self) -> SEND_S_REG_Q_INT_CLR_W {
        SEND_S_REG_Q_INT_CLR_W::new(self)
    }
    #[doc = "Bit 5 - Set this bit to clear UHCI_SEND_A_REQ_Q_INT interrupt."]
    #[inline(always)]
    pub fn send_a_reg_q_int_clr(&mut self) -> SEND_A_REG_Q_INT_CLR_W {
        SEND_A_REG_Q_INT_CLR_W::new(self)
    }
    #[doc = "Bit 6 - Set this bit to clear UHCI_OUTLINK_EOF_ERR_INT interrupt."]
    #[inline(always)]
    pub fn outlink_eof_err_int_clr(&mut self) -> OUTLINK_EOF_ERR_INT_CLR_W {
        OUTLINK_EOF_ERR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 7 - Set this bit to clear UHCI_APP_CTRL0_INT interrupt."]
    #[inline(always)]
    pub fn app_ctrl0_int_clr(&mut self) -> APP_CTRL0_INT_CLR_W {
        APP_CTRL0_INT_CLR_W::new(self)
    }
    #[doc = "Bit 8 - Set this bit to clear UHCI_APP_CTRL1_INT interrupt."]
    #[inline(always)]
    pub fn app_ctrl1_int_clr(&mut self) -> APP_CTRL1_INT_CLR_W {
        APP_CTRL1_INT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt clear bits\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_clr](index.html) module"]
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
