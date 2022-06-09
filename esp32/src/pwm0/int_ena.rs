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
#[doc = "Field `TIMER0_STOP_INT_ENA` reader - "]
pub type TIMER0_STOP_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `TIMER0_STOP_INT_ENA` writer - "]
pub type TIMER0_STOP_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 0>;
#[doc = "Field `TIMER1_STOP_INT_ENA` reader - "]
pub type TIMER1_STOP_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `TIMER1_STOP_INT_ENA` writer - "]
pub type TIMER1_STOP_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 1>;
#[doc = "Field `TIMER2_STOP_INT_ENA` reader - "]
pub type TIMER2_STOP_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `TIMER2_STOP_INT_ENA` writer - "]
pub type TIMER2_STOP_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 2>;
#[doc = "Field `TIMER0_TEZ_INT_ENA` reader - "]
pub type TIMER0_TEZ_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `TIMER0_TEZ_INT_ENA` writer - "]
pub type TIMER0_TEZ_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 3>;
#[doc = "Field `TIMER1_TEZ_INT_ENA` reader - "]
pub type TIMER1_TEZ_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `TIMER1_TEZ_INT_ENA` writer - "]
pub type TIMER1_TEZ_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 4>;
#[doc = "Field `TIMER2_TEZ_INT_ENA` reader - "]
pub type TIMER2_TEZ_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `TIMER2_TEZ_INT_ENA` writer - "]
pub type TIMER2_TEZ_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 5>;
#[doc = "Field `TIMER0_TEP_INT_ENA` reader - "]
pub type TIMER0_TEP_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `TIMER0_TEP_INT_ENA` writer - "]
pub type TIMER0_TEP_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 6>;
#[doc = "Field `TIMER1_TEP_INT_ENA` reader - "]
pub type TIMER1_TEP_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `TIMER1_TEP_INT_ENA` writer - "]
pub type TIMER1_TEP_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 7>;
#[doc = "Field `TIMER2_TEP_INT_ENA` reader - "]
pub type TIMER2_TEP_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `TIMER2_TEP_INT_ENA` writer - "]
pub type TIMER2_TEP_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 8>;
#[doc = "Field `FAULT0_INT_ENA` reader - "]
pub type FAULT0_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `FAULT0_INT_ENA` writer - "]
pub type FAULT0_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 9>;
#[doc = "Field `FAULT1_INT_ENA` reader - "]
pub type FAULT1_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `FAULT1_INT_ENA` writer - "]
pub type FAULT1_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 10>;
#[doc = "Field `FAULT2_INT_ENA` reader - "]
pub type FAULT2_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `FAULT2_INT_ENA` writer - "]
pub type FAULT2_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 11>;
#[doc = "Field `FAULT0_CLR_INT_ENA` reader - "]
pub type FAULT0_CLR_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `FAULT0_CLR_INT_ENA` writer - "]
pub type FAULT0_CLR_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 12>;
#[doc = "Field `FAULT1_CLR_INT_ENA` reader - "]
pub type FAULT1_CLR_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `FAULT1_CLR_INT_ENA` writer - "]
pub type FAULT1_CLR_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 13>;
#[doc = "Field `FAULT2_CLR_INT_ENA` reader - "]
pub type FAULT2_CLR_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `FAULT2_CLR_INT_ENA` writer - "]
pub type FAULT2_CLR_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 14>;
#[doc = "Field `OP0_TEA_INT_ENA` reader - "]
pub type OP0_TEA_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `OP0_TEA_INT_ENA` writer - "]
pub type OP0_TEA_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 15>;
#[doc = "Field `OP1_TEA_INT_ENA` reader - "]
pub type OP1_TEA_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `OP1_TEA_INT_ENA` writer - "]
pub type OP1_TEA_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 16>;
#[doc = "Field `OP2_TEA_INT_ENA` reader - "]
pub type OP2_TEA_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `OP2_TEA_INT_ENA` writer - "]
pub type OP2_TEA_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 17>;
#[doc = "Field `OP0_TEB_INT_ENA` reader - "]
pub type OP0_TEB_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `OP0_TEB_INT_ENA` writer - "]
pub type OP0_TEB_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 18>;
#[doc = "Field `OP1_TEB_INT_ENA` reader - "]
pub type OP1_TEB_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `OP1_TEB_INT_ENA` writer - "]
pub type OP1_TEB_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 19>;
#[doc = "Field `OP2_TEB_INT_ENA` reader - "]
pub type OP2_TEB_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `OP2_TEB_INT_ENA` writer - "]
pub type OP2_TEB_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 20>;
#[doc = "Field `FH0_CBC_INT_ENA` reader - "]
pub type FH0_CBC_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `FH0_CBC_INT_ENA` writer - "]
pub type FH0_CBC_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 21>;
#[doc = "Field `FH1_CBC_INT_ENA` reader - "]
pub type FH1_CBC_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `FH1_CBC_INT_ENA` writer - "]
pub type FH1_CBC_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 22>;
#[doc = "Field `FH2_CBC_INT_ENA` reader - "]
pub type FH2_CBC_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `FH2_CBC_INT_ENA` writer - "]
pub type FH2_CBC_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 23>;
#[doc = "Field `FH0_OST_INT_ENA` reader - "]
pub type FH0_OST_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `FH0_OST_INT_ENA` writer - "]
pub type FH0_OST_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 24>;
#[doc = "Field `FH1_OST_INT_ENA` reader - "]
pub type FH1_OST_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `FH1_OST_INT_ENA` writer - "]
pub type FH1_OST_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 25>;
#[doc = "Field `FH2_OST_INT_ENA` reader - "]
pub type FH2_OST_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `FH2_OST_INT_ENA` writer - "]
pub type FH2_OST_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 26>;
#[doc = "Field `CAP0_INT_ENA` reader - "]
pub type CAP0_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `CAP0_INT_ENA` writer - "]
pub type CAP0_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 27>;
#[doc = "Field `CAP1_INT_ENA` reader - "]
pub type CAP1_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `CAP1_INT_ENA` writer - "]
pub type CAP1_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 28>;
#[doc = "Field `CAP2_INT_ENA` reader - "]
pub type CAP2_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `CAP2_INT_ENA` writer - "]
pub type CAP2_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 29>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn timer0_stop_int_ena(&self) -> TIMER0_STOP_INT_ENA_R {
        TIMER0_STOP_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn timer1_stop_int_ena(&self) -> TIMER1_STOP_INT_ENA_R {
        TIMER1_STOP_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn timer2_stop_int_ena(&self) -> TIMER2_STOP_INT_ENA_R {
        TIMER2_STOP_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn timer0_tez_int_ena(&self) -> TIMER0_TEZ_INT_ENA_R {
        TIMER0_TEZ_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn timer1_tez_int_ena(&self) -> TIMER1_TEZ_INT_ENA_R {
        TIMER1_TEZ_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn timer2_tez_int_ena(&self) -> TIMER2_TEZ_INT_ENA_R {
        TIMER2_TEZ_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn timer0_tep_int_ena(&self) -> TIMER0_TEP_INT_ENA_R {
        TIMER0_TEP_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn timer1_tep_int_ena(&self) -> TIMER1_TEP_INT_ENA_R {
        TIMER1_TEP_INT_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn timer2_tep_int_ena(&self) -> TIMER2_TEP_INT_ENA_R {
        TIMER2_TEP_INT_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn fault0_int_ena(&self) -> FAULT0_INT_ENA_R {
        FAULT0_INT_ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn fault1_int_ena(&self) -> FAULT1_INT_ENA_R {
        FAULT1_INT_ENA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn fault2_int_ena(&self) -> FAULT2_INT_ENA_R {
        FAULT2_INT_ENA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn fault0_clr_int_ena(&self) -> FAULT0_CLR_INT_ENA_R {
        FAULT0_CLR_INT_ENA_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn fault1_clr_int_ena(&self) -> FAULT1_CLR_INT_ENA_R {
        FAULT1_CLR_INT_ENA_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn fault2_clr_int_ena(&self) -> FAULT2_CLR_INT_ENA_R {
        FAULT2_CLR_INT_ENA_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn op0_tea_int_ena(&self) -> OP0_TEA_INT_ENA_R {
        OP0_TEA_INT_ENA_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn op1_tea_int_ena(&self) -> OP1_TEA_INT_ENA_R {
        OP1_TEA_INT_ENA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn op2_tea_int_ena(&self) -> OP2_TEA_INT_ENA_R {
        OP2_TEA_INT_ENA_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn op0_teb_int_ena(&self) -> OP0_TEB_INT_ENA_R {
        OP0_TEB_INT_ENA_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn op1_teb_int_ena(&self) -> OP1_TEB_INT_ENA_R {
        OP1_TEB_INT_ENA_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn op2_teb_int_ena(&self) -> OP2_TEB_INT_ENA_R {
        OP2_TEB_INT_ENA_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn fh0_cbc_int_ena(&self) -> FH0_CBC_INT_ENA_R {
        FH0_CBC_INT_ENA_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn fh1_cbc_int_ena(&self) -> FH1_CBC_INT_ENA_R {
        FH1_CBC_INT_ENA_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn fh2_cbc_int_ena(&self) -> FH2_CBC_INT_ENA_R {
        FH2_CBC_INT_ENA_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn fh0_ost_int_ena(&self) -> FH0_OST_INT_ENA_R {
        FH0_OST_INT_ENA_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn fh1_ost_int_ena(&self) -> FH1_OST_INT_ENA_R {
        FH1_OST_INT_ENA_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn fh2_ost_int_ena(&self) -> FH2_OST_INT_ENA_R {
        FH2_OST_INT_ENA_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn cap0_int_ena(&self) -> CAP0_INT_ENA_R {
        CAP0_INT_ENA_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn cap1_int_ena(&self) -> CAP1_INT_ENA_R {
        CAP1_INT_ENA_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn cap2_int_ena(&self) -> CAP2_INT_ENA_R {
        CAP2_INT_ENA_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn timer0_stop_int_ena(&mut self) -> TIMER0_STOP_INT_ENA_W {
        TIMER0_STOP_INT_ENA_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn timer1_stop_int_ena(&mut self) -> TIMER1_STOP_INT_ENA_W {
        TIMER1_STOP_INT_ENA_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn timer2_stop_int_ena(&mut self) -> TIMER2_STOP_INT_ENA_W {
        TIMER2_STOP_INT_ENA_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn timer0_tez_int_ena(&mut self) -> TIMER0_TEZ_INT_ENA_W {
        TIMER0_TEZ_INT_ENA_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn timer1_tez_int_ena(&mut self) -> TIMER1_TEZ_INT_ENA_W {
        TIMER1_TEZ_INT_ENA_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn timer2_tez_int_ena(&mut self) -> TIMER2_TEZ_INT_ENA_W {
        TIMER2_TEZ_INT_ENA_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn timer0_tep_int_ena(&mut self) -> TIMER0_TEP_INT_ENA_W {
        TIMER0_TEP_INT_ENA_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn timer1_tep_int_ena(&mut self) -> TIMER1_TEP_INT_ENA_W {
        TIMER1_TEP_INT_ENA_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn timer2_tep_int_ena(&mut self) -> TIMER2_TEP_INT_ENA_W {
        TIMER2_TEP_INT_ENA_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn fault0_int_ena(&mut self) -> FAULT0_INT_ENA_W {
        FAULT0_INT_ENA_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn fault1_int_ena(&mut self) -> FAULT1_INT_ENA_W {
        FAULT1_INT_ENA_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn fault2_int_ena(&mut self) -> FAULT2_INT_ENA_W {
        FAULT2_INT_ENA_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn fault0_clr_int_ena(&mut self) -> FAULT0_CLR_INT_ENA_W {
        FAULT0_CLR_INT_ENA_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn fault1_clr_int_ena(&mut self) -> FAULT1_CLR_INT_ENA_W {
        FAULT1_CLR_INT_ENA_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn fault2_clr_int_ena(&mut self) -> FAULT2_CLR_INT_ENA_W {
        FAULT2_CLR_INT_ENA_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn op0_tea_int_ena(&mut self) -> OP0_TEA_INT_ENA_W {
        OP0_TEA_INT_ENA_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn op1_tea_int_ena(&mut self) -> OP1_TEA_INT_ENA_W {
        OP1_TEA_INT_ENA_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn op2_tea_int_ena(&mut self) -> OP2_TEA_INT_ENA_W {
        OP2_TEA_INT_ENA_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn op0_teb_int_ena(&mut self) -> OP0_TEB_INT_ENA_W {
        OP0_TEB_INT_ENA_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn op1_teb_int_ena(&mut self) -> OP1_TEB_INT_ENA_W {
        OP1_TEB_INT_ENA_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn op2_teb_int_ena(&mut self) -> OP2_TEB_INT_ENA_W {
        OP2_TEB_INT_ENA_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn fh0_cbc_int_ena(&mut self) -> FH0_CBC_INT_ENA_W {
        FH0_CBC_INT_ENA_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn fh1_cbc_int_ena(&mut self) -> FH1_CBC_INT_ENA_W {
        FH1_CBC_INT_ENA_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn fh2_cbc_int_ena(&mut self) -> FH2_CBC_INT_ENA_W {
        FH2_CBC_INT_ENA_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn fh0_ost_int_ena(&mut self) -> FH0_OST_INT_ENA_W {
        FH0_OST_INT_ENA_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn fh1_ost_int_ena(&mut self) -> FH1_OST_INT_ENA_W {
        FH1_OST_INT_ENA_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn fh2_ost_int_ena(&mut self) -> FH2_OST_INT_ENA_W {
        FH2_OST_INT_ENA_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn cap0_int_ena(&mut self) -> CAP0_INT_ENA_W {
        CAP0_INT_ENA_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn cap1_int_ena(&mut self) -> CAP1_INT_ENA_W {
        CAP1_INT_ENA_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn cap2_int_ena(&mut self) -> CAP2_INT_ENA_W {
        CAP2_INT_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_ena](index.html) module"]
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
