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
#[doc = "Fields `CH(0-1)_TX_END_INT_CLR` writer - reg_ch%s_tx_end_int_clr."]
pub type CH_TX_END_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Fields `CH(2-3)_RX_END_INT_CLR` writer - reg_ch2_rx_end_int_clr."]
pub type CH_RX_END_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Fields `CH(0-1)_TX_ERR_INT_CLR` writer - reg_ch%s_err_int_clr."]
pub type CH_TX_ERR_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Fields `CH(2-3)_RX_ERR_INT_CLR` writer - reg_ch2_err_int_clr."]
pub type CH_RX_ERR_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Fields `CH(0-1)_TX_THR_EVENT_INT_CLR` writer - reg_ch%s_tx_thr_event_int_clr."]
pub type CH_TX_THR_EVENT_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Field `CH2_RX_THR_EVENT_INT_CLR` writer - reg_ch2_rx_thr_event_int_clr."]
pub type CH2_RX_THR_EVENT_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 10>;
#[doc = "Field `CH3_RX_THR_EVENT_INT_CLR` writer - reg_ch3_rx_thr_event_int_clr."]
pub type CH3_RX_THR_EVENT_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 11>;
#[doc = "Fields `CH(0-1)_TX_LOOP_INT_CLR` writer - reg_ch%s_tx_loop_int_clr."]
pub type CH_TX_LOOP_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
impl W {
    #[doc = "reg_ch(0-1)_tx_end_int_clr."]
    #[inline(always)]
    pub unsafe fn ch_tx_end_int_clr<const O: u8>(&mut self) -> CH_TX_END_INT_CLR_W<O> {
        CH_TX_END_INT_CLR_W::new(self)
    }
    #[doc = "Bit 0 - reg_ch0_tx_end_int_clr."]
    #[inline(always)]
    pub fn ch0_tx_end_int_clr(&mut self) -> CH_TX_END_INT_CLR_W<0> {
        CH_TX_END_INT_CLR_W::new(self)
    }
    #[doc = "Bit 1 - reg_ch1_tx_end_int_clr."]
    #[inline(always)]
    pub fn ch1_tx_end_int_clr(&mut self) -> CH_TX_END_INT_CLR_W<1> {
        CH_TX_END_INT_CLR_W::new(self)
    }
    #[doc = "reg_ch2_rx_end_int_clr."]
    #[inline(always)]
    pub unsafe fn ch_rx_end_int_clr<const O: u8>(&mut self) -> CH_RX_END_INT_CLR_W<O> {
        CH_RX_END_INT_CLR_W::new(self)
    }
    #[doc = "Bit 2 - reg_ch2_rx_end_int_clr."]
    #[inline(always)]
    pub fn ch2_rx_end_int_clr(&mut self) -> CH_RX_END_INT_CLR_W<2> {
        CH_RX_END_INT_CLR_W::new(self)
    }
    #[doc = "Bit 3 - reg_ch2_rx_end_int_clr."]
    #[inline(always)]
    pub fn ch3_rx_end_int_clr(&mut self) -> CH_RX_END_INT_CLR_W<3> {
        CH_RX_END_INT_CLR_W::new(self)
    }
    #[doc = "reg_ch(0-1)_err_int_clr."]
    #[inline(always)]
    pub unsafe fn ch_tx_err_int_clr<const O: u8>(&mut self) -> CH_TX_ERR_INT_CLR_W<O> {
        CH_TX_ERR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 4 - reg_ch0_err_int_clr."]
    #[inline(always)]
    pub fn ch0_tx_err_int_clr(&mut self) -> CH_TX_ERR_INT_CLR_W<4> {
        CH_TX_ERR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 5 - reg_ch1_err_int_clr."]
    #[inline(always)]
    pub fn ch1_tx_err_int_clr(&mut self) -> CH_TX_ERR_INT_CLR_W<5> {
        CH_TX_ERR_INT_CLR_W::new(self)
    }
    #[doc = "reg_ch2_err_int_clr."]
    #[inline(always)]
    pub unsafe fn ch_rx_err_int_clr<const O: u8>(&mut self) -> CH_RX_ERR_INT_CLR_W<O> {
        CH_RX_ERR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 6 - reg_ch2_err_int_clr."]
    #[inline(always)]
    pub fn ch2_rx_err_int_clr(&mut self) -> CH_RX_ERR_INT_CLR_W<6> {
        CH_RX_ERR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 7 - reg_ch2_err_int_clr."]
    #[inline(always)]
    pub fn ch3_rx_err_int_clr(&mut self) -> CH_RX_ERR_INT_CLR_W<7> {
        CH_RX_ERR_INT_CLR_W::new(self)
    }
    #[doc = "reg_ch(0-1)_tx_thr_event_int_clr."]
    #[inline(always)]
    pub unsafe fn ch_tx_thr_event_int_clr<const O: u8>(&mut self) -> CH_TX_THR_EVENT_INT_CLR_W<O> {
        CH_TX_THR_EVENT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 8 - reg_ch0_tx_thr_event_int_clr."]
    #[inline(always)]
    pub fn ch0_tx_thr_event_int_clr(&mut self) -> CH_TX_THR_EVENT_INT_CLR_W<8> {
        CH_TX_THR_EVENT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 9 - reg_ch1_tx_thr_event_int_clr."]
    #[inline(always)]
    pub fn ch1_tx_thr_event_int_clr(&mut self) -> CH_TX_THR_EVENT_INT_CLR_W<9> {
        CH_TX_THR_EVENT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 10 - reg_ch2_rx_thr_event_int_clr."]
    #[inline(always)]
    pub fn ch2_rx_thr_event_int_clr(&mut self) -> CH2_RX_THR_EVENT_INT_CLR_W {
        CH2_RX_THR_EVENT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 11 - reg_ch3_rx_thr_event_int_clr."]
    #[inline(always)]
    pub fn ch3_rx_thr_event_int_clr(&mut self) -> CH3_RX_THR_EVENT_INT_CLR_W {
        CH3_RX_THR_EVENT_INT_CLR_W::new(self)
    }
    #[doc = "reg_ch(0-1)_tx_loop_int_clr."]
    #[inline(always)]
    pub unsafe fn ch_tx_loop_int_clr<const O: u8>(&mut self) -> CH_TX_LOOP_INT_CLR_W<O> {
        CH_TX_LOOP_INT_CLR_W::new(self)
    }
    #[doc = "Bit 12 - reg_ch0_tx_loop_int_clr."]
    #[inline(always)]
    pub fn ch0_tx_loop_int_clr(&mut self) -> CH_TX_LOOP_INT_CLR_W<12> {
        CH_TX_LOOP_INT_CLR_W::new(self)
    }
    #[doc = "Bit 13 - reg_ch1_tx_loop_int_clr."]
    #[inline(always)]
    pub fn ch1_tx_loop_int_clr(&mut self) -> CH_TX_LOOP_INT_CLR_W<13> {
        CH_TX_LOOP_INT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RMT_INT_CLR_REG.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_clr](index.html) module"]
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
