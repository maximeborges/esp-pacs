#[doc = "Register `TIMER2_CFG0` reader"]
pub struct R(crate::R<TIMER2_CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER2_CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER2_CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER2_CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER2_CFG0` writer"]
pub struct W(crate::W<TIMER2_CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER2_CFG0_SPEC>;
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
impl From<crate::W<TIMER2_CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER2_CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER2_PRESCALE` reader - period of PT0_clk = Period of PWM_clk * (PWM_timer2_PRESCALE + 1)"]
pub struct TIMER2_PRESCALE_R(crate::FieldReader<u8>);
impl TIMER2_PRESCALE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TIMER2_PRESCALE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER2_PRESCALE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER2_PRESCALE` writer - period of PT0_clk = Period of PWM_clk * (PWM_timer2_PRESCALE + 1)"]
pub struct TIMER2_PRESCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER2_PRESCALE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `TIMER2_PERIOD` reader - period shadow register of PWM timer2"]
pub struct TIMER2_PERIOD_R(crate::FieldReader<u16>);
impl TIMER2_PERIOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TIMER2_PERIOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER2_PERIOD_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER2_PERIOD` writer - period shadow register of PWM timer2"]
pub struct TIMER2_PERIOD_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER2_PERIOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 8)) | ((value as u32 & 0xffff) << 8);
        self.w
    }
}
#[doc = "Field `TIMER2_PERIOD_UPMETHOD` reader - Update method for active register of PWM timer2 period, 0: immediate, 1: TEZ, 2: sync, 3: TEZ | sync. TEZ here and below means timer equal zero event"]
pub struct TIMER2_PERIOD_UPMETHOD_R(crate::FieldReader<u8>);
impl TIMER2_PERIOD_UPMETHOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TIMER2_PERIOD_UPMETHOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER2_PERIOD_UPMETHOD_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER2_PERIOD_UPMETHOD` writer - Update method for active register of PWM timer2 period, 0: immediate, 1: TEZ, 2: sync, 3: TEZ | sync. TEZ here and below means timer equal zero event"]
pub struct TIMER2_PERIOD_UPMETHOD_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER2_PERIOD_UPMETHOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 24)) | ((value as u32 & 3) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - period of PT0_clk = Period of PWM_clk * (PWM_timer2_PRESCALE + 1)"]
    #[inline(always)]
    pub fn timer2_prescale(&self) -> TIMER2_PRESCALE_R {
        TIMER2_PRESCALE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:23 - period shadow register of PWM timer2"]
    #[inline(always)]
    pub fn timer2_period(&self) -> TIMER2_PERIOD_R {
        TIMER2_PERIOD_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
    #[doc = "Bits 24:25 - Update method for active register of PWM timer2 period, 0: immediate, 1: TEZ, 2: sync, 3: TEZ | sync. TEZ here and below means timer equal zero event"]
    #[inline(always)]
    pub fn timer2_period_upmethod(&self) -> TIMER2_PERIOD_UPMETHOD_R {
        TIMER2_PERIOD_UPMETHOD_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - period of PT0_clk = Period of PWM_clk * (PWM_timer2_PRESCALE + 1)"]
    #[inline(always)]
    pub fn timer2_prescale(&mut self) -> TIMER2_PRESCALE_W {
        TIMER2_PRESCALE_W { w: self }
    }
    #[doc = "Bits 8:23 - period shadow register of PWM timer2"]
    #[inline(always)]
    pub fn timer2_period(&mut self) -> TIMER2_PERIOD_W {
        TIMER2_PERIOD_W { w: self }
    }
    #[doc = "Bits 24:25 - Update method for active register of PWM timer2 period, 0: immediate, 1: TEZ, 2: sync, 3: TEZ | sync. TEZ here and below means timer equal zero event"]
    #[inline(always)]
    pub fn timer2_period_upmethod(&mut self) -> TIMER2_PERIOD_UPMETHOD_W {
        TIMER2_PERIOD_UPMETHOD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM timer2 period and update method configuration register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer2_cfg0](index.html) module"]
pub struct TIMER2_CFG0_SPEC;
impl crate::RegisterSpec for TIMER2_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer2_cfg0::R](R) reader structure"]
impl crate::Readable for TIMER2_CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer2_cfg0::W](W) writer structure"]
impl crate::Writable for TIMER2_CFG0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMER2_CFG0 to value 0xff00"]
impl crate::Resettable for TIMER2_CFG0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff00
    }
}
