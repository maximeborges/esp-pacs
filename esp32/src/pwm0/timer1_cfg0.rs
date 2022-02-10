#[doc = "Register `TIMER1_CFG0` reader"]
pub struct R(crate::R<TIMER1_CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER1_CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER1_CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER1_CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER1_CFG0` writer"]
pub struct W(crate::W<TIMER1_CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER1_CFG0_SPEC>;
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
impl From<crate::W<TIMER1_CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER1_CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER1_PRESCALE` reader - "]
pub struct TIMER1_PRESCALE_R(crate::FieldReader<u8, u8>);
impl TIMER1_PRESCALE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TIMER1_PRESCALE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER1_PRESCALE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER1_PRESCALE` writer - "]
pub struct TIMER1_PRESCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER1_PRESCALE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `TIMER1_PERIOD` reader - "]
pub struct TIMER1_PERIOD_R(crate::FieldReader<u16, u16>);
impl TIMER1_PERIOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TIMER1_PERIOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER1_PERIOD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER1_PERIOD` writer - "]
pub struct TIMER1_PERIOD_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER1_PERIOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 8)) | ((value as u32 & 0xffff) << 8);
        self.w
    }
}
#[doc = "Field `TIMER1_PERIOD_UPMETHOD` reader - "]
pub struct TIMER1_PERIOD_UPMETHOD_R(crate::FieldReader<u8, u8>);
impl TIMER1_PERIOD_UPMETHOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TIMER1_PERIOD_UPMETHOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER1_PERIOD_UPMETHOD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER1_PERIOD_UPMETHOD` writer - "]
pub struct TIMER1_PERIOD_UPMETHOD_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER1_PERIOD_UPMETHOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn timer1_prescale(&self) -> TIMER1_PRESCALE_R {
        TIMER1_PRESCALE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:23"]
    #[inline(always)]
    pub fn timer1_period(&self) -> TIMER1_PERIOD_R {
        TIMER1_PERIOD_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn timer1_period_upmethod(&self) -> TIMER1_PERIOD_UPMETHOD_R {
        TIMER1_PERIOD_UPMETHOD_R::new(((self.bits >> 24) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn timer1_prescale(&mut self) -> TIMER1_PRESCALE_W {
        TIMER1_PRESCALE_W { w: self }
    }
    #[doc = "Bits 8:23"]
    #[inline(always)]
    pub fn timer1_period(&mut self) -> TIMER1_PERIOD_W {
        TIMER1_PERIOD_W { w: self }
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn timer1_period_upmethod(&mut self) -> TIMER1_PERIOD_UPMETHOD_W {
        TIMER1_PERIOD_UPMETHOD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer1_cfg0]
(index.html) module"]
pub struct TIMER1_CFG0_SPEC;
impl crate::RegisterSpec for TIMER1_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer1_cfg0::R]
(R) reader structure"]
impl crate::Readable for TIMER1_CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer1_cfg0::W]
(W) writer structure"]
impl crate::Writable for TIMER1_CFG0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMER1_CFG0 to value 0xff00"]
impl crate::Resettable for TIMER1_CFG0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff00
    }
}
