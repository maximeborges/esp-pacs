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
#[doc = "Field `TIMER0_STOP_INT_CLR` writer - "]
pub type TIMER0_STOP_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 0>;
#[doc = "Field `TIMER1_STOP_INT_CLR` writer - "]
pub type TIMER1_STOP_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 1>;
#[doc = "Field `TIMER2_STOP_INT_CLR` writer - "]
pub type TIMER2_STOP_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 2>;
#[doc = "Field `TIMER0_TEZ_INT_CLR` writer - "]
pub type TIMER0_TEZ_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 3>;
#[doc = "Field `TIMER1_TEZ_INT_CLR` writer - "]
pub type TIMER1_TEZ_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 4>;
#[doc = "Field `TIMER2_TEZ_INT_CLR` writer - "]
pub type TIMER2_TEZ_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 5>;
#[doc = "Field `TIMER0_TEP_INT_CLR` writer - "]
pub type TIMER0_TEP_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 6>;
#[doc = "Field `TIMER1_TEP_INT_CLR` writer - "]
pub type TIMER1_TEP_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 7>;
#[doc = "Field `TIMER2_TEP_INT_CLR` writer - "]
pub type TIMER2_TEP_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 8>;
#[doc = "Field `FAULT0_INT_CLR` writer - "]
pub type FAULT0_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 9>;
#[doc = "Field `FAULT1_INT_CLR` writer - "]
pub type FAULT1_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 10>;
#[doc = "Field `FAULT2_INT_CLR` writer - "]
pub type FAULT2_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 11>;
#[doc = "Field `FAULT0_CLR_INT_CLR` writer - "]
pub type FAULT0_CLR_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 12>;
#[doc = "Field `FAULT1_CLR_INT_CLR` writer - "]
pub type FAULT1_CLR_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 13>;
#[doc = "Field `FAULT2_CLR_INT_CLR` writer - "]
pub type FAULT2_CLR_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 14>;
#[doc = "Field `OP0_TEA_INT_CLR` writer - "]
pub type OP0_TEA_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 15>;
#[doc = "Field `OP1_TEA_INT_CLR` writer - "]
pub type OP1_TEA_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 16>;
#[doc = "Field `OP2_TEA_INT_CLR` writer - "]
pub type OP2_TEA_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 17>;
#[doc = "Field `OP0_TEB_INT_CLR` writer - "]
pub type OP0_TEB_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 18>;
#[doc = "Field `OP1_TEB_INT_CLR` writer - "]
pub type OP1_TEB_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 19>;
#[doc = "Field `OP2_TEB_INT_CLR` writer - "]
pub type OP2_TEB_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 20>;
#[doc = "Field `FH0_CBC_INT_CLR` writer - "]
pub type FH0_CBC_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 21>;
#[doc = "Field `FH1_CBC_INT_CLR` writer - "]
pub type FH1_CBC_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 22>;
#[doc = "Field `FH2_CBC_INT_CLR` writer - "]
pub type FH2_CBC_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 23>;
#[doc = "Field `FH0_OST_INT_CLR` writer - "]
pub type FH0_OST_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 24>;
#[doc = "Field `FH1_OST_INT_CLR` writer - "]
pub type FH1_OST_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 25>;
#[doc = "Field `FH2_OST_INT_CLR` writer - "]
pub type FH2_OST_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 26>;
#[doc = "Field `CAP0_INT_CLR` writer - "]
pub type CAP0_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 27>;
#[doc = "Field `CAP1_INT_CLR` writer - "]
pub type CAP1_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 28>;
#[doc = "Field `CAP2_INT_CLR` writer - "]
pub type CAP2_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 29>;
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn timer0_stop_int_clr(&mut self) -> TIMER0_STOP_INT_CLR_W {
        TIMER0_STOP_INT_CLR_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn timer1_stop_int_clr(&mut self) -> TIMER1_STOP_INT_CLR_W {
        TIMER1_STOP_INT_CLR_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn timer2_stop_int_clr(&mut self) -> TIMER2_STOP_INT_CLR_W {
        TIMER2_STOP_INT_CLR_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn timer0_tez_int_clr(&mut self) -> TIMER0_TEZ_INT_CLR_W {
        TIMER0_TEZ_INT_CLR_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn timer1_tez_int_clr(&mut self) -> TIMER1_TEZ_INT_CLR_W {
        TIMER1_TEZ_INT_CLR_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn timer2_tez_int_clr(&mut self) -> TIMER2_TEZ_INT_CLR_W {
        TIMER2_TEZ_INT_CLR_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn timer0_tep_int_clr(&mut self) -> TIMER0_TEP_INT_CLR_W {
        TIMER0_TEP_INT_CLR_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn timer1_tep_int_clr(&mut self) -> TIMER1_TEP_INT_CLR_W {
        TIMER1_TEP_INT_CLR_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn timer2_tep_int_clr(&mut self) -> TIMER2_TEP_INT_CLR_W {
        TIMER2_TEP_INT_CLR_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn fault0_int_clr(&mut self) -> FAULT0_INT_CLR_W {
        FAULT0_INT_CLR_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn fault1_int_clr(&mut self) -> FAULT1_INT_CLR_W {
        FAULT1_INT_CLR_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn fault2_int_clr(&mut self) -> FAULT2_INT_CLR_W {
        FAULT2_INT_CLR_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn fault0_clr_int_clr(&mut self) -> FAULT0_CLR_INT_CLR_W {
        FAULT0_CLR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn fault1_clr_int_clr(&mut self) -> FAULT1_CLR_INT_CLR_W {
        FAULT1_CLR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn fault2_clr_int_clr(&mut self) -> FAULT2_CLR_INT_CLR_W {
        FAULT2_CLR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn op0_tea_int_clr(&mut self) -> OP0_TEA_INT_CLR_W {
        OP0_TEA_INT_CLR_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn op1_tea_int_clr(&mut self) -> OP1_TEA_INT_CLR_W {
        OP1_TEA_INT_CLR_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn op2_tea_int_clr(&mut self) -> OP2_TEA_INT_CLR_W {
        OP2_TEA_INT_CLR_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn op0_teb_int_clr(&mut self) -> OP0_TEB_INT_CLR_W {
        OP0_TEB_INT_CLR_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn op1_teb_int_clr(&mut self) -> OP1_TEB_INT_CLR_W {
        OP1_TEB_INT_CLR_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn op2_teb_int_clr(&mut self) -> OP2_TEB_INT_CLR_W {
        OP2_TEB_INT_CLR_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn fh0_cbc_int_clr(&mut self) -> FH0_CBC_INT_CLR_W {
        FH0_CBC_INT_CLR_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn fh1_cbc_int_clr(&mut self) -> FH1_CBC_INT_CLR_W {
        FH1_CBC_INT_CLR_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn fh2_cbc_int_clr(&mut self) -> FH2_CBC_INT_CLR_W {
        FH2_CBC_INT_CLR_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn fh0_ost_int_clr(&mut self) -> FH0_OST_INT_CLR_W {
        FH0_OST_INT_CLR_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn fh1_ost_int_clr(&mut self) -> FH1_OST_INT_CLR_W {
        FH1_OST_INT_CLR_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn fh2_ost_int_clr(&mut self) -> FH2_OST_INT_CLR_W {
        FH2_OST_INT_CLR_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn cap0_int_clr(&mut self) -> CAP0_INT_CLR_W {
        CAP0_INT_CLR_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn cap1_int_clr(&mut self) -> CAP1_INT_CLR_W {
        CAP1_INT_CLR_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn cap2_int_clr(&mut self) -> CAP2_INT_CLR_W {
        CAP2_INT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_clr](index.html) module"]
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
