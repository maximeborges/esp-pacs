#[doc = "Register `HSCH6_CONF0` reader"]
pub struct R(crate::R<HSCH6_CONF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSCH6_CONF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSCH6_CONF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSCH6_CONF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSCH6_CONF0` writer"]
pub struct W(crate::W<HSCH6_CONF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSCH6_CONF0_SPEC>;
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
impl From<crate::W<HSCH6_CONF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSCH6_CONF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER_SEL_HSCH6` reader - There are four high speed timers the two bits are used to select one of them for high speed channel6. 2'b00: seletc hstimer0. 2'b01: select hstimer1. 2'b10: select hstimer2. 2'b11: select hstimer3."]
pub struct TIMER_SEL_HSCH6_R(crate::FieldReader<u8>);
impl TIMER_SEL_HSCH6_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TIMER_SEL_HSCH6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER_SEL_HSCH6_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER_SEL_HSCH6` writer - There are four high speed timers the two bits are used to select one of them for high speed channel6. 2'b00: seletc hstimer0. 2'b01: select hstimer1. 2'b10: select hstimer2. 2'b11: select hstimer3."]
pub struct TIMER_SEL_HSCH6_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_SEL_HSCH6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u32 & 3);
        self.w
    }
}
#[doc = "Field `SIG_OUT_EN_HSCH6` reader - This is the output enable control bit for high speed channel6"]
pub struct SIG_OUT_EN_HSCH6_R(crate::FieldReader<bool>);
impl SIG_OUT_EN_HSCH6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SIG_OUT_EN_HSCH6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIG_OUT_EN_HSCH6_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIG_OUT_EN_HSCH6` writer - This is the output enable control bit for high speed channel6"]
pub struct SIG_OUT_EN_HSCH6_W<'a> {
    w: &'a mut W,
}
impl<'a> SIG_OUT_EN_HSCH6_W<'a> {
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
#[doc = "Field `IDLE_LV_HSCH6` reader - This bit is used to control the output value when high speed channel6 is off."]
pub struct IDLE_LV_HSCH6_R(crate::FieldReader<bool>);
impl IDLE_LV_HSCH6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IDLE_LV_HSCH6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDLE_LV_HSCH6_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDLE_LV_HSCH6` writer - This bit is used to control the output value when high speed channel6 is off."]
pub struct IDLE_LV_HSCH6_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLE_LV_HSCH6_W<'a> {
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
impl R {
    #[doc = "Bits 0:1 - There are four high speed timers the two bits are used to select one of them for high speed channel6. 2'b00: seletc hstimer0. 2'b01: select hstimer1. 2'b10: select hstimer2. 2'b11: select hstimer3."]
    #[inline(always)]
    pub fn timer_sel_hsch6(&self) -> TIMER_SEL_HSCH6_R {
        TIMER_SEL_HSCH6_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - This is the output enable control bit for high speed channel6"]
    #[inline(always)]
    pub fn sig_out_en_hsch6(&self) -> SIG_OUT_EN_HSCH6_R {
        SIG_OUT_EN_HSCH6_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit is used to control the output value when high speed channel6 is off."]
    #[inline(always)]
    pub fn idle_lv_hsch6(&self) -> IDLE_LV_HSCH6_R {
        IDLE_LV_HSCH6_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - There are four high speed timers the two bits are used to select one of them for high speed channel6. 2'b00: seletc hstimer0. 2'b01: select hstimer1. 2'b10: select hstimer2. 2'b11: select hstimer3."]
    #[inline(always)]
    pub fn timer_sel_hsch6(&mut self) -> TIMER_SEL_HSCH6_W {
        TIMER_SEL_HSCH6_W { w: self }
    }
    #[doc = "Bit 2 - This is the output enable control bit for high speed channel6"]
    #[inline(always)]
    pub fn sig_out_en_hsch6(&mut self) -> SIG_OUT_EN_HSCH6_W {
        SIG_OUT_EN_HSCH6_W { w: self }
    }
    #[doc = "Bit 3 - This bit is used to control the output value when high speed channel6 is off."]
    #[inline(always)]
    pub fn idle_lv_hsch6(&mut self) -> IDLE_LV_HSCH6_W {
        IDLE_LV_HSCH6_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsch6_conf0](index.html) module"]
pub struct HSCH6_CONF0_SPEC;
impl crate::RegisterSpec for HSCH6_CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hsch6_conf0::R](R) reader structure"]
impl crate::Readable for HSCH6_CONF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hsch6_conf0::W](W) writer structure"]
impl crate::Writable for HSCH6_CONF0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HSCH6_CONF0 to value 0"]
impl crate::Resettable for HSCH6_CONF0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
