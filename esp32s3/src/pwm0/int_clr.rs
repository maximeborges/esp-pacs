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
#[doc = "Field `TIMER0_STOP_INT_CLR` writer - Set this bit to clear the interrupt triggered when the timer 0 stops."]
pub struct TIMER0_STOP_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER0_STOP_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `TIMER1_STOP_INT_CLR` writer - Set this bit to clear the interrupt triggered when the timer 1 stops."]
pub struct TIMER1_STOP_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER1_STOP_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `TIMER2_STOP_INT_CLR` writer - Set this bit to clear the interrupt triggered when the timer 2 stops."]
pub struct TIMER2_STOP_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER2_STOP_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `TIMER0_TEZ_INT_CLR` writer - Set this bit to clear the interrupt triggered by a PWM timer 0 TEZ event."]
pub struct TIMER0_TEZ_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER0_TEZ_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `TIMER1_TEZ_INT_CLR` writer - Set this bit to clear the interrupt triggered by a PWM timer 1 TEZ event."]
pub struct TIMER1_TEZ_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER1_TEZ_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `TIMER2_TEZ_INT_CLR` writer - Set this bit to clear the interrupt triggered by a PWM timer 2 TEZ event."]
pub struct TIMER2_TEZ_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER2_TEZ_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `TIMER0_TEP_INT_CLR` writer - Set this bit to clear the interrupt triggered by a PWM timer 0 TEP event."]
pub struct TIMER0_TEP_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER0_TEP_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `TIMER1_TEP_INT_CLR` writer - Set this bit to clear the interrupt triggered by a PWM timer 1 TEP event."]
pub struct TIMER1_TEP_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER1_TEP_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `TIMER2_TEP_INT_CLR` writer - Set this bit to clear the interrupt triggered by a PWM timer 2 TEP event."]
pub struct TIMER2_TEP_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER2_TEP_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `FAULT0_INT_CLR` writer - Set this bit to clear the interrupt triggered when event_f0 starts."]
pub struct FAULT0_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULT0_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `FAULT1_INT_CLR` writer - Set this bit to clear the interrupt triggered when event_f1 starts."]
pub struct FAULT1_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULT1_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `FAULT2_INT_CLR` writer - Set this bit to clear the interrupt triggered when event_f2 starts."]
pub struct FAULT2_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULT2_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `FAULT0_CLR_INT_CLR` writer - Set this bit to clear the interrupt triggered when event_f0 ends."]
pub struct FAULT0_CLR_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULT0_CLR_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `FAULT1_CLR_INT_CLR` writer - Set this bit to clear the interrupt triggered when event_f1 ends."]
pub struct FAULT1_CLR_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULT1_CLR_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `FAULT2_CLR_INT_CLR` writer - Set this bit to clear the interrupt triggered when event_f2 ends."]
pub struct FAULT2_CLR_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULT2_CLR_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `CMPR0_TEA_INT_CLR` writer - Set this bit to clear the interrupt triggered by a PWM operator 0 TEA event"]
pub struct CMPR0_TEA_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPR0_TEA_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `CMPR1_TEA_INT_CLR` writer - Set this bit to clear the interrupt triggered by a PWM operator 1 TEA event"]
pub struct CMPR1_TEA_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPR1_TEA_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `CMPR2_TEA_INT_CLR` writer - Set this bit to clear the interrupt triggered by a PWM operator 2 TEA event"]
pub struct CMPR2_TEA_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPR2_TEA_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `CMPR0_TEB_INT_CLR` writer - Set this bit to clear the interrupt triggered by a PWM operator 0 TEB event"]
pub struct CMPR0_TEB_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPR0_TEB_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `CMPR1_TEB_INT_CLR` writer - Set this bit to clear the interrupt triggered by a PWM operator 1 TEB event"]
pub struct CMPR1_TEB_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPR1_TEB_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `CMPR2_TEB_INT_CLR` writer - Set this bit to clear the interrupt triggered by a PWM operator 2 TEB event"]
pub struct CMPR2_TEB_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPR2_TEB_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `TZ0_CBC_INT_CLR` writer - Set this bit to clear the interrupt triggered by a cycle-by-cycle mode action on PWM0."]
pub struct TZ0_CBC_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TZ0_CBC_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `TZ1_CBC_INT_CLR` writer - Set this bit to clear the interrupt triggered by a cycle-by-cycle mode action on PWM1."]
pub struct TZ1_CBC_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TZ1_CBC_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `TZ2_CBC_INT_CLR` writer - Set this bit to clear the interrupt triggered by a cycle-by-cycle mode action on PWM2."]
pub struct TZ2_CBC_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TZ2_CBC_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `TZ0_OST_INT_CLR` writer - Set this bit to clear the interrupt triggered by a one-shot mode action on PWM0."]
pub struct TZ0_OST_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TZ0_OST_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `TZ1_OST_INT_CLR` writer - Set this bit to clear the interrupt triggered by a one-shot mode action on PWM1."]
pub struct TZ1_OST_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TZ1_OST_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `TZ2_OST_INT_CLR` writer - Set this bit to clear the interrupt triggered by a one-shot mode action on PWM2."]
pub struct TZ2_OST_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TZ2_OST_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `CAP0_INT_CLR` writer - Set this bit to clear the interrupt triggered by capture on channel 0."]
pub struct CAP0_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP0_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `CAP1_INT_CLR` writer - Set this bit to clear the interrupt triggered by capture on channel 1."]
pub struct CAP1_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP1_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `CAP2_INT_CLR` writer - Set this bit to clear the interrupt triggered by capture on channel 2."]
pub struct CAP2_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP2_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to clear the interrupt triggered when the timer 0 stops."]
    #[inline(always)]
    pub fn timer0_stop_int_clr(&mut self) -> TIMER0_STOP_INT_CLR_W {
        TIMER0_STOP_INT_CLR_W { w: self }
    }
    #[doc = "Bit 1 - Set this bit to clear the interrupt triggered when the timer 1 stops."]
    #[inline(always)]
    pub fn timer1_stop_int_clr(&mut self) -> TIMER1_STOP_INT_CLR_W {
        TIMER1_STOP_INT_CLR_W { w: self }
    }
    #[doc = "Bit 2 - Set this bit to clear the interrupt triggered when the timer 2 stops."]
    #[inline(always)]
    pub fn timer2_stop_int_clr(&mut self) -> TIMER2_STOP_INT_CLR_W {
        TIMER2_STOP_INT_CLR_W { w: self }
    }
    #[doc = "Bit 3 - Set this bit to clear the interrupt triggered by a PWM timer 0 TEZ event."]
    #[inline(always)]
    pub fn timer0_tez_int_clr(&mut self) -> TIMER0_TEZ_INT_CLR_W {
        TIMER0_TEZ_INT_CLR_W { w: self }
    }
    #[doc = "Bit 4 - Set this bit to clear the interrupt triggered by a PWM timer 1 TEZ event."]
    #[inline(always)]
    pub fn timer1_tez_int_clr(&mut self) -> TIMER1_TEZ_INT_CLR_W {
        TIMER1_TEZ_INT_CLR_W { w: self }
    }
    #[doc = "Bit 5 - Set this bit to clear the interrupt triggered by a PWM timer 2 TEZ event."]
    #[inline(always)]
    pub fn timer2_tez_int_clr(&mut self) -> TIMER2_TEZ_INT_CLR_W {
        TIMER2_TEZ_INT_CLR_W { w: self }
    }
    #[doc = "Bit 6 - Set this bit to clear the interrupt triggered by a PWM timer 0 TEP event."]
    #[inline(always)]
    pub fn timer0_tep_int_clr(&mut self) -> TIMER0_TEP_INT_CLR_W {
        TIMER0_TEP_INT_CLR_W { w: self }
    }
    #[doc = "Bit 7 - Set this bit to clear the interrupt triggered by a PWM timer 1 TEP event."]
    #[inline(always)]
    pub fn timer1_tep_int_clr(&mut self) -> TIMER1_TEP_INT_CLR_W {
        TIMER1_TEP_INT_CLR_W { w: self }
    }
    #[doc = "Bit 8 - Set this bit to clear the interrupt triggered by a PWM timer 2 TEP event."]
    #[inline(always)]
    pub fn timer2_tep_int_clr(&mut self) -> TIMER2_TEP_INT_CLR_W {
        TIMER2_TEP_INT_CLR_W { w: self }
    }
    #[doc = "Bit 9 - Set this bit to clear the interrupt triggered when event_f0 starts."]
    #[inline(always)]
    pub fn fault0_int_clr(&mut self) -> FAULT0_INT_CLR_W {
        FAULT0_INT_CLR_W { w: self }
    }
    #[doc = "Bit 10 - Set this bit to clear the interrupt triggered when event_f1 starts."]
    #[inline(always)]
    pub fn fault1_int_clr(&mut self) -> FAULT1_INT_CLR_W {
        FAULT1_INT_CLR_W { w: self }
    }
    #[doc = "Bit 11 - Set this bit to clear the interrupt triggered when event_f2 starts."]
    #[inline(always)]
    pub fn fault2_int_clr(&mut self) -> FAULT2_INT_CLR_W {
        FAULT2_INT_CLR_W { w: self }
    }
    #[doc = "Bit 12 - Set this bit to clear the interrupt triggered when event_f0 ends."]
    #[inline(always)]
    pub fn fault0_clr_int_clr(&mut self) -> FAULT0_CLR_INT_CLR_W {
        FAULT0_CLR_INT_CLR_W { w: self }
    }
    #[doc = "Bit 13 - Set this bit to clear the interrupt triggered when event_f1 ends."]
    #[inline(always)]
    pub fn fault1_clr_int_clr(&mut self) -> FAULT1_CLR_INT_CLR_W {
        FAULT1_CLR_INT_CLR_W { w: self }
    }
    #[doc = "Bit 14 - Set this bit to clear the interrupt triggered when event_f2 ends."]
    #[inline(always)]
    pub fn fault2_clr_int_clr(&mut self) -> FAULT2_CLR_INT_CLR_W {
        FAULT2_CLR_INT_CLR_W { w: self }
    }
    #[doc = "Bit 15 - Set this bit to clear the interrupt triggered by a PWM operator 0 TEA event"]
    #[inline(always)]
    pub fn cmpr0_tea_int_clr(&mut self) -> CMPR0_TEA_INT_CLR_W {
        CMPR0_TEA_INT_CLR_W { w: self }
    }
    #[doc = "Bit 16 - Set this bit to clear the interrupt triggered by a PWM operator 1 TEA event"]
    #[inline(always)]
    pub fn cmpr1_tea_int_clr(&mut self) -> CMPR1_TEA_INT_CLR_W {
        CMPR1_TEA_INT_CLR_W { w: self }
    }
    #[doc = "Bit 17 - Set this bit to clear the interrupt triggered by a PWM operator 2 TEA event"]
    #[inline(always)]
    pub fn cmpr2_tea_int_clr(&mut self) -> CMPR2_TEA_INT_CLR_W {
        CMPR2_TEA_INT_CLR_W { w: self }
    }
    #[doc = "Bit 18 - Set this bit to clear the interrupt triggered by a PWM operator 0 TEB event"]
    #[inline(always)]
    pub fn cmpr0_teb_int_clr(&mut self) -> CMPR0_TEB_INT_CLR_W {
        CMPR0_TEB_INT_CLR_W { w: self }
    }
    #[doc = "Bit 19 - Set this bit to clear the interrupt triggered by a PWM operator 1 TEB event"]
    #[inline(always)]
    pub fn cmpr1_teb_int_clr(&mut self) -> CMPR1_TEB_INT_CLR_W {
        CMPR1_TEB_INT_CLR_W { w: self }
    }
    #[doc = "Bit 20 - Set this bit to clear the interrupt triggered by a PWM operator 2 TEB event"]
    #[inline(always)]
    pub fn cmpr2_teb_int_clr(&mut self) -> CMPR2_TEB_INT_CLR_W {
        CMPR2_TEB_INT_CLR_W { w: self }
    }
    #[doc = "Bit 21 - Set this bit to clear the interrupt triggered by a cycle-by-cycle mode action on PWM0."]
    #[inline(always)]
    pub fn tz0_cbc_int_clr(&mut self) -> TZ0_CBC_INT_CLR_W {
        TZ0_CBC_INT_CLR_W { w: self }
    }
    #[doc = "Bit 22 - Set this bit to clear the interrupt triggered by a cycle-by-cycle mode action on PWM1."]
    #[inline(always)]
    pub fn tz1_cbc_int_clr(&mut self) -> TZ1_CBC_INT_CLR_W {
        TZ1_CBC_INT_CLR_W { w: self }
    }
    #[doc = "Bit 23 - Set this bit to clear the interrupt triggered by a cycle-by-cycle mode action on PWM2."]
    #[inline(always)]
    pub fn tz2_cbc_int_clr(&mut self) -> TZ2_CBC_INT_CLR_W {
        TZ2_CBC_INT_CLR_W { w: self }
    }
    #[doc = "Bit 24 - Set this bit to clear the interrupt triggered by a one-shot mode action on PWM0."]
    #[inline(always)]
    pub fn tz0_ost_int_clr(&mut self) -> TZ0_OST_INT_CLR_W {
        TZ0_OST_INT_CLR_W { w: self }
    }
    #[doc = "Bit 25 - Set this bit to clear the interrupt triggered by a one-shot mode action on PWM1."]
    #[inline(always)]
    pub fn tz1_ost_int_clr(&mut self) -> TZ1_OST_INT_CLR_W {
        TZ1_OST_INT_CLR_W { w: self }
    }
    #[doc = "Bit 26 - Set this bit to clear the interrupt triggered by a one-shot mode action on PWM2."]
    #[inline(always)]
    pub fn tz2_ost_int_clr(&mut self) -> TZ2_OST_INT_CLR_W {
        TZ2_OST_INT_CLR_W { w: self }
    }
    #[doc = "Bit 27 - Set this bit to clear the interrupt triggered by capture on channel 0."]
    #[inline(always)]
    pub fn cap0_int_clr(&mut self) -> CAP0_INT_CLR_W {
        CAP0_INT_CLR_W { w: self }
    }
    #[doc = "Bit 28 - Set this bit to clear the interrupt triggered by capture on channel 1."]
    #[inline(always)]
    pub fn cap1_int_clr(&mut self) -> CAP1_INT_CLR_W {
        CAP1_INT_CLR_W { w: self }
    }
    #[doc = "Bit 29 - Set this bit to clear the interrupt triggered by capture on channel 2."]
    #[inline(always)]
    pub fn cap2_int_clr(&mut self) -> CAP2_INT_CLR_W {
        CAP2_INT_CLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt clear bits\n\nThis register you can [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_clr]
(index.html) module"]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [int_clr::W]
(W) writer structure"]
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
