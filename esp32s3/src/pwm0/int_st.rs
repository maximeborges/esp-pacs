#[doc = "Register `INT_ST` reader"]
pub struct R(crate::R<INT_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TIMER0_STOP_INT_ST` reader - The masked status bit for the interrupt triggered when the timer 0 stops."]
pub struct TIMER0_STOP_INT_ST_R(crate::FieldReader<bool, bool>);
impl TIMER0_STOP_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER0_STOP_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER0_STOP_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER1_STOP_INT_ST` reader - The masked status bit for the interrupt triggered when the timer 1 stops."]
pub struct TIMER1_STOP_INT_ST_R(crate::FieldReader<bool, bool>);
impl TIMER1_STOP_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER1_STOP_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER1_STOP_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER2_STOP_INT_ST` reader - The masked status bit for the interrupt triggered when the timer 2 stops."]
pub struct TIMER2_STOP_INT_ST_R(crate::FieldReader<bool, bool>);
impl TIMER2_STOP_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER2_STOP_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER2_STOP_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER0_TEZ_INT_ST` reader - The masked status bit for the interrupt triggered by a PWM timer 0 TEZ event."]
pub struct TIMER0_TEZ_INT_ST_R(crate::FieldReader<bool, bool>);
impl TIMER0_TEZ_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER0_TEZ_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER0_TEZ_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER1_TEZ_INT_ST` reader - The masked status bit for the interrupt triggered by a PWM timer 1 TEZ event."]
pub struct TIMER1_TEZ_INT_ST_R(crate::FieldReader<bool, bool>);
impl TIMER1_TEZ_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER1_TEZ_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER1_TEZ_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER2_TEZ_INT_ST` reader - The masked status bit for the interrupt triggered by a PWM timer 2 TEZ event."]
pub struct TIMER2_TEZ_INT_ST_R(crate::FieldReader<bool, bool>);
impl TIMER2_TEZ_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER2_TEZ_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER2_TEZ_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER0_TEP_INT_ST` reader - The masked status bit for the interrupt triggered by a PWM timer 0 TEP event."]
pub struct TIMER0_TEP_INT_ST_R(crate::FieldReader<bool, bool>);
impl TIMER0_TEP_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER0_TEP_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER0_TEP_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER1_TEP_INT_ST` reader - The masked status bit for the interrupt triggered by a PWM timer 1 TEP event."]
pub struct TIMER1_TEP_INT_ST_R(crate::FieldReader<bool, bool>);
impl TIMER1_TEP_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER1_TEP_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER1_TEP_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER2_TEP_INT_ST` reader - The masked status bit for the interrupt triggered by a PWM timer 2 TEP event."]
pub struct TIMER2_TEP_INT_ST_R(crate::FieldReader<bool, bool>);
impl TIMER2_TEP_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER2_TEP_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER2_TEP_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAULT0_INT_ST` reader - The masked status bit for the interrupt triggered when event_f0 starts."]
pub struct FAULT0_INT_ST_R(crate::FieldReader<bool, bool>);
impl FAULT0_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FAULT0_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FAULT0_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAULT1_INT_ST` reader - The masked status bit for the interrupt triggered when event_f1 starts."]
pub struct FAULT1_INT_ST_R(crate::FieldReader<bool, bool>);
impl FAULT1_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FAULT1_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FAULT1_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAULT2_INT_ST` reader - The masked status bit for the interrupt triggered when event_f2 starts."]
pub struct FAULT2_INT_ST_R(crate::FieldReader<bool, bool>);
impl FAULT2_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FAULT2_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FAULT2_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAULT0_CLR_INT_ST` reader - The masked status bit for the interrupt triggered when event_f0 ends."]
pub struct FAULT0_CLR_INT_ST_R(crate::FieldReader<bool, bool>);
impl FAULT0_CLR_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FAULT0_CLR_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FAULT0_CLR_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAULT1_CLR_INT_ST` reader - The masked status bit for the interrupt triggered when event_f1 ends."]
pub struct FAULT1_CLR_INT_ST_R(crate::FieldReader<bool, bool>);
impl FAULT1_CLR_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FAULT1_CLR_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FAULT1_CLR_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAULT2_CLR_INT_ST` reader - The masked status bit for the interrupt triggered when event_f2 ends."]
pub struct FAULT2_CLR_INT_ST_R(crate::FieldReader<bool, bool>);
impl FAULT2_CLR_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FAULT2_CLR_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FAULT2_CLR_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPR0_TEA_INT_ST` reader - The masked status bit for the interrupt triggered by a PWM operator 0 TEA event"]
pub struct CMPR0_TEA_INT_ST_R(crate::FieldReader<bool, bool>);
impl CMPR0_TEA_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMPR0_TEA_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPR0_TEA_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPR1_TEA_INT_ST` reader - The masked status bit for the interrupt triggered by a PWM operator 1 TEA event"]
pub struct CMPR1_TEA_INT_ST_R(crate::FieldReader<bool, bool>);
impl CMPR1_TEA_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMPR1_TEA_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPR1_TEA_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPR2_TEA_INT_ST` reader - The masked status bit for the interrupt triggered by a PWM operator 2 TEA event"]
pub struct CMPR2_TEA_INT_ST_R(crate::FieldReader<bool, bool>);
impl CMPR2_TEA_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMPR2_TEA_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPR2_TEA_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPR0_TEB_INT_ST` reader - The masked status bit for the interrupt triggered by a PWM operator 0 TEB event"]
pub struct CMPR0_TEB_INT_ST_R(crate::FieldReader<bool, bool>);
impl CMPR0_TEB_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMPR0_TEB_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPR0_TEB_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPR1_TEB_INT_ST` reader - The masked status bit for the interrupt triggered by a PWM operator 1 TEB event"]
pub struct CMPR1_TEB_INT_ST_R(crate::FieldReader<bool, bool>);
impl CMPR1_TEB_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMPR1_TEB_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPR1_TEB_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPR2_TEB_INT_ST` reader - The masked status bit for the interrupt triggered by a PWM operator 2 TEB event"]
pub struct CMPR2_TEB_INT_ST_R(crate::FieldReader<bool, bool>);
impl CMPR2_TEB_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMPR2_TEB_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPR2_TEB_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZ0_CBC_INT_ST` reader - The masked status bit for the interrupt triggered by a cycle-by-cycle mode action on PWM0."]
pub struct TZ0_CBC_INT_ST_R(crate::FieldReader<bool, bool>);
impl TZ0_CBC_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TZ0_CBC_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZ0_CBC_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZ1_CBC_INT_ST` reader - The masked status bit for the interrupt triggered by a cycle-by-cycle mode action on PWM1."]
pub struct TZ1_CBC_INT_ST_R(crate::FieldReader<bool, bool>);
impl TZ1_CBC_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TZ1_CBC_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZ1_CBC_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZ2_CBC_INT_ST` reader - The masked status bit for the interrupt triggered by a cycle-by-cycle mode action on PWM2."]
pub struct TZ2_CBC_INT_ST_R(crate::FieldReader<bool, bool>);
impl TZ2_CBC_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TZ2_CBC_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZ2_CBC_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZ0_OST_INT_ST` reader - The masked status bit for the interrupt triggered by a one-shot mode action on PWM0."]
pub struct TZ0_OST_INT_ST_R(crate::FieldReader<bool, bool>);
impl TZ0_OST_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TZ0_OST_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZ0_OST_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZ1_OST_INT_ST` reader - The masked status bit for the interrupt triggered by a one-shot mode action on PWM1."]
pub struct TZ1_OST_INT_ST_R(crate::FieldReader<bool, bool>);
impl TZ1_OST_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TZ1_OST_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZ1_OST_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZ2_OST_INT_ST` reader - The masked status bit for the interrupt triggered by a one-shot mode action on PWM2."]
pub struct TZ2_OST_INT_ST_R(crate::FieldReader<bool, bool>);
impl TZ2_OST_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TZ2_OST_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZ2_OST_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAP0_INT_ST` reader - The masked status bit for the interrupt triggered by capture on channel 0."]
pub struct CAP0_INT_ST_R(crate::FieldReader<bool, bool>);
impl CAP0_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CAP0_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAP0_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAP1_INT_ST` reader - The masked status bit for the interrupt triggered by capture on channel 1."]
pub struct CAP1_INT_ST_R(crate::FieldReader<bool, bool>);
impl CAP1_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CAP1_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAP1_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAP2_INT_ST` reader - The masked status bit for the interrupt triggered by capture on channel 2."]
pub struct CAP2_INT_ST_R(crate::FieldReader<bool, bool>);
impl CAP2_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CAP2_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAP2_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - The masked status bit for the interrupt triggered when the timer 0 stops."]
    #[inline(always)]
    pub fn timer0_stop_int_st(&self) -> TIMER0_STOP_INT_ST_R {
        TIMER0_STOP_INT_ST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - The masked status bit for the interrupt triggered when the timer 1 stops."]
    #[inline(always)]
    pub fn timer1_stop_int_st(&self) -> TIMER1_STOP_INT_ST_R {
        TIMER1_STOP_INT_ST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - The masked status bit for the interrupt triggered when the timer 2 stops."]
    #[inline(always)]
    pub fn timer2_stop_int_st(&self) -> TIMER2_STOP_INT_ST_R {
        TIMER2_STOP_INT_ST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - The masked status bit for the interrupt triggered by a PWM timer 0 TEZ event."]
    #[inline(always)]
    pub fn timer0_tez_int_st(&self) -> TIMER0_TEZ_INT_ST_R {
        TIMER0_TEZ_INT_ST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - The masked status bit for the interrupt triggered by a PWM timer 1 TEZ event."]
    #[inline(always)]
    pub fn timer1_tez_int_st(&self) -> TIMER1_TEZ_INT_ST_R {
        TIMER1_TEZ_INT_ST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - The masked status bit for the interrupt triggered by a PWM timer 2 TEZ event."]
    #[inline(always)]
    pub fn timer2_tez_int_st(&self) -> TIMER2_TEZ_INT_ST_R {
        TIMER2_TEZ_INT_ST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - The masked status bit for the interrupt triggered by a PWM timer 0 TEP event."]
    #[inline(always)]
    pub fn timer0_tep_int_st(&self) -> TIMER0_TEP_INT_ST_R {
        TIMER0_TEP_INT_ST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - The masked status bit for the interrupt triggered by a PWM timer 1 TEP event."]
    #[inline(always)]
    pub fn timer1_tep_int_st(&self) -> TIMER1_TEP_INT_ST_R {
        TIMER1_TEP_INT_ST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - The masked status bit for the interrupt triggered by a PWM timer 2 TEP event."]
    #[inline(always)]
    pub fn timer2_tep_int_st(&self) -> TIMER2_TEP_INT_ST_R {
        TIMER2_TEP_INT_ST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - The masked status bit for the interrupt triggered when event_f0 starts."]
    #[inline(always)]
    pub fn fault0_int_st(&self) -> FAULT0_INT_ST_R {
        FAULT0_INT_ST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - The masked status bit for the interrupt triggered when event_f1 starts."]
    #[inline(always)]
    pub fn fault1_int_st(&self) -> FAULT1_INT_ST_R {
        FAULT1_INT_ST_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - The masked status bit for the interrupt triggered when event_f2 starts."]
    #[inline(always)]
    pub fn fault2_int_st(&self) -> FAULT2_INT_ST_R {
        FAULT2_INT_ST_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - The masked status bit for the interrupt triggered when event_f0 ends."]
    #[inline(always)]
    pub fn fault0_clr_int_st(&self) -> FAULT0_CLR_INT_ST_R {
        FAULT0_CLR_INT_ST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - The masked status bit for the interrupt triggered when event_f1 ends."]
    #[inline(always)]
    pub fn fault1_clr_int_st(&self) -> FAULT1_CLR_INT_ST_R {
        FAULT1_CLR_INT_ST_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - The masked status bit for the interrupt triggered when event_f2 ends."]
    #[inline(always)]
    pub fn fault2_clr_int_st(&self) -> FAULT2_CLR_INT_ST_R {
        FAULT2_CLR_INT_ST_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - The masked status bit for the interrupt triggered by a PWM operator 0 TEA event"]
    #[inline(always)]
    pub fn cmpr0_tea_int_st(&self) -> CMPR0_TEA_INT_ST_R {
        CMPR0_TEA_INT_ST_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - The masked status bit for the interrupt triggered by a PWM operator 1 TEA event"]
    #[inline(always)]
    pub fn cmpr1_tea_int_st(&self) -> CMPR1_TEA_INT_ST_R {
        CMPR1_TEA_INT_ST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - The masked status bit for the interrupt triggered by a PWM operator 2 TEA event"]
    #[inline(always)]
    pub fn cmpr2_tea_int_st(&self) -> CMPR2_TEA_INT_ST_R {
        CMPR2_TEA_INT_ST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - The masked status bit for the interrupt triggered by a PWM operator 0 TEB event"]
    #[inline(always)]
    pub fn cmpr0_teb_int_st(&self) -> CMPR0_TEB_INT_ST_R {
        CMPR0_TEB_INT_ST_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - The masked status bit for the interrupt triggered by a PWM operator 1 TEB event"]
    #[inline(always)]
    pub fn cmpr1_teb_int_st(&self) -> CMPR1_TEB_INT_ST_R {
        CMPR1_TEB_INT_ST_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - The masked status bit for the interrupt triggered by a PWM operator 2 TEB event"]
    #[inline(always)]
    pub fn cmpr2_teb_int_st(&self) -> CMPR2_TEB_INT_ST_R {
        CMPR2_TEB_INT_ST_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - The masked status bit for the interrupt triggered by a cycle-by-cycle mode action on PWM0."]
    #[inline(always)]
    pub fn tz0_cbc_int_st(&self) -> TZ0_CBC_INT_ST_R {
        TZ0_CBC_INT_ST_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - The masked status bit for the interrupt triggered by a cycle-by-cycle mode action on PWM1."]
    #[inline(always)]
    pub fn tz1_cbc_int_st(&self) -> TZ1_CBC_INT_ST_R {
        TZ1_CBC_INT_ST_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - The masked status bit for the interrupt triggered by a cycle-by-cycle mode action on PWM2."]
    #[inline(always)]
    pub fn tz2_cbc_int_st(&self) -> TZ2_CBC_INT_ST_R {
        TZ2_CBC_INT_ST_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - The masked status bit for the interrupt triggered by a one-shot mode action on PWM0."]
    #[inline(always)]
    pub fn tz0_ost_int_st(&self) -> TZ0_OST_INT_ST_R {
        TZ0_OST_INT_ST_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - The masked status bit for the interrupt triggered by a one-shot mode action on PWM1."]
    #[inline(always)]
    pub fn tz1_ost_int_st(&self) -> TZ1_OST_INT_ST_R {
        TZ1_OST_INT_ST_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - The masked status bit for the interrupt triggered by a one-shot mode action on PWM2."]
    #[inline(always)]
    pub fn tz2_ost_int_st(&self) -> TZ2_OST_INT_ST_R {
        TZ2_OST_INT_ST_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - The masked status bit for the interrupt triggered by capture on channel 0."]
    #[inline(always)]
    pub fn cap0_int_st(&self) -> CAP0_INT_ST_R {
        CAP0_INT_ST_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - The masked status bit for the interrupt triggered by capture on channel 1."]
    #[inline(always)]
    pub fn cap1_int_st(&self) -> CAP1_INT_ST_R {
        CAP1_INT_ST_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - The masked status bit for the interrupt triggered by capture on channel 2."]
    #[inline(always)]
    pub fn cap2_int_st(&self) -> CAP2_INT_ST_R {
        CAP2_INT_ST_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
#[doc = "Masked interrupt status\n\nThis register you can [`read`]
(crate::generic::Reg::read). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_st]
(index.html) module"]
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_st::R]
(R) reader structure"]
impl crate::Readable for INT_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for INT_ST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
