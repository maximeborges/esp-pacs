#[doc = "Register `HSCH3_CONF0` reader"]
pub struct R(crate::R<HSCH3_CONF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSCH3_CONF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSCH3_CONF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSCH3_CONF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSCH3_CONF0` writer"]
pub struct W(crate::W<HSCH3_CONF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSCH3_CONF0_SPEC>;
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
impl From<crate::W<HSCH3_CONF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSCH3_CONF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER_SEL_HSCH3` reader - There are four high speed timers the two bits are used to select one of them for high speed channel3. 2'b00: seletc hstimer0. 2'b01: select hstimer1. 2'b10: select hstimer2. 2'b11: select hstimer3."]
pub type TIMER_SEL_HSCH3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIMER_SEL_HSCH3` writer - There are four high speed timers the two bits are used to select one of them for high speed channel3. 2'b00: seletc hstimer0. 2'b01: select hstimer1. 2'b10: select hstimer2. 2'b11: select hstimer3."]
pub type TIMER_SEL_HSCH3_W<'a> = crate::FieldWriter<'a, u32, HSCH3_CONF0_SPEC, u8, u8, 2, 0>;
#[doc = "Field `SIG_OUT_EN_HSCH3` reader - This is the output enable control bit for high speed channel3"]
pub type SIG_OUT_EN_HSCH3_R = crate::BitReader<bool>;
#[doc = "Field `SIG_OUT_EN_HSCH3` writer - This is the output enable control bit for high speed channel3"]
pub type SIG_OUT_EN_HSCH3_W<'a> = crate::BitWriter<'a, u32, HSCH3_CONF0_SPEC, bool, 2>;
#[doc = "Field `IDLE_LV_HSCH3` reader - This bit is used to control the output value when high speed channel3 is off."]
pub type IDLE_LV_HSCH3_R = crate::BitReader<bool>;
#[doc = "Field `IDLE_LV_HSCH3` writer - This bit is used to control the output value when high speed channel3 is off."]
pub type IDLE_LV_HSCH3_W<'a> = crate::BitWriter<'a, u32, HSCH3_CONF0_SPEC, bool, 3>;
impl R {
    #[doc = "Bits 0:1 - There are four high speed timers the two bits are used to select one of them for high speed channel3. 2'b00: seletc hstimer0. 2'b01: select hstimer1. 2'b10: select hstimer2. 2'b11: select hstimer3."]
    #[inline(always)]
    pub fn timer_sel_hsch3(&self) -> TIMER_SEL_HSCH3_R {
        TIMER_SEL_HSCH3_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - This is the output enable control bit for high speed channel3"]
    #[inline(always)]
    pub fn sig_out_en_hsch3(&self) -> SIG_OUT_EN_HSCH3_R {
        SIG_OUT_EN_HSCH3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit is used to control the output value when high speed channel3 is off."]
    #[inline(always)]
    pub fn idle_lv_hsch3(&self) -> IDLE_LV_HSCH3_R {
        IDLE_LV_HSCH3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - There are four high speed timers the two bits are used to select one of them for high speed channel3. 2'b00: seletc hstimer0. 2'b01: select hstimer1. 2'b10: select hstimer2. 2'b11: select hstimer3."]
    #[inline(always)]
    pub fn timer_sel_hsch3(&mut self) -> TIMER_SEL_HSCH3_W {
        TIMER_SEL_HSCH3_W::new(self)
    }
    #[doc = "Bit 2 - This is the output enable control bit for high speed channel3"]
    #[inline(always)]
    pub fn sig_out_en_hsch3(&mut self) -> SIG_OUT_EN_HSCH3_W {
        SIG_OUT_EN_HSCH3_W::new(self)
    }
    #[doc = "Bit 3 - This bit is used to control the output value when high speed channel3 is off."]
    #[inline(always)]
    pub fn idle_lv_hsch3(&mut self) -> IDLE_LV_HSCH3_W {
        IDLE_LV_HSCH3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsch3_conf0](index.html) module"]
pub struct HSCH3_CONF0_SPEC;
impl crate::RegisterSpec for HSCH3_CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hsch3_conf0::R](R) reader structure"]
impl crate::Readable for HSCH3_CONF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hsch3_conf0::W](W) writer structure"]
impl crate::Writable for HSCH3_CONF0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HSCH3_CONF0 to value 0"]
impl crate::Resettable for HSCH3_CONF0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
