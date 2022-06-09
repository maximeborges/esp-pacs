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
#[doc = "Field `SLAVE_TRAN_COMP_INT_CLR` writer - clear slave transit complete interrupt"]
pub type SLAVE_TRAN_COMP_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 0>;
#[doc = "Field `ARBITRATION_LOST_INT_CLR` writer - clear arbitration lost interrupt"]
pub type ARBITRATION_LOST_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 1>;
#[doc = "Field `MASTER_TRAN_COMP_INT_CLR` writer - clear master transit complete interrupt"]
pub type MASTER_TRAN_COMP_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 2>;
#[doc = "Field `TRANS_COMPLETE_INT_CLR` writer - clear transit complete interrupt"]
pub type TRANS_COMPLETE_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 3>;
#[doc = "Field `TIME_OUT_INT_CLR` writer - clear time out interrupt"]
pub type TIME_OUT_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 4>;
#[doc = "Field `ACK_ERR_INT_CLR` writer - clear ack error interrupt"]
pub type ACK_ERR_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 5>;
#[doc = "Field `RX_DATA_INT_CLR` writer - clear receive data interrupt"]
pub type RX_DATA_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 6>;
#[doc = "Field `TX_DATA_INT_CLR` writer - clear transit load data complete interrupt"]
pub type TX_DATA_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 7>;
#[doc = "Field `DETECT_START_INT_CLR` writer - clear detect start interrupt"]
pub type DETECT_START_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 8>;
impl W {
    #[doc = "Bit 0 - clear slave transit complete interrupt"]
    #[inline(always)]
    pub fn slave_tran_comp_int_clr(&mut self) -> SLAVE_TRAN_COMP_INT_CLR_W {
        SLAVE_TRAN_COMP_INT_CLR_W::new(self)
    }
    #[doc = "Bit 1 - clear arbitration lost interrupt"]
    #[inline(always)]
    pub fn arbitration_lost_int_clr(&mut self) -> ARBITRATION_LOST_INT_CLR_W {
        ARBITRATION_LOST_INT_CLR_W::new(self)
    }
    #[doc = "Bit 2 - clear master transit complete interrupt"]
    #[inline(always)]
    pub fn master_tran_comp_int_clr(&mut self) -> MASTER_TRAN_COMP_INT_CLR_W {
        MASTER_TRAN_COMP_INT_CLR_W::new(self)
    }
    #[doc = "Bit 3 - clear transit complete interrupt"]
    #[inline(always)]
    pub fn trans_complete_int_clr(&mut self) -> TRANS_COMPLETE_INT_CLR_W {
        TRANS_COMPLETE_INT_CLR_W::new(self)
    }
    #[doc = "Bit 4 - clear time out interrupt"]
    #[inline(always)]
    pub fn time_out_int_clr(&mut self) -> TIME_OUT_INT_CLR_W {
        TIME_OUT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 5 - clear ack error interrupt"]
    #[inline(always)]
    pub fn ack_err_int_clr(&mut self) -> ACK_ERR_INT_CLR_W {
        ACK_ERR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 6 - clear receive data interrupt"]
    #[inline(always)]
    pub fn rx_data_int_clr(&mut self) -> RX_DATA_INT_CLR_W {
        RX_DATA_INT_CLR_W::new(self)
    }
    #[doc = "Bit 7 - clear transit load data complete interrupt"]
    #[inline(always)]
    pub fn tx_data_int_clr(&mut self) -> TX_DATA_INT_CLR_W {
        TX_DATA_INT_CLR_W::new(self)
    }
    #[doc = "Bit 8 - clear detect start interrupt"]
    #[inline(always)]
    pub fn detect_start_int_clr(&mut self) -> DETECT_START_INT_CLR_W {
        DETECT_START_INT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "interrupt clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_clr](index.html) module"]
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
