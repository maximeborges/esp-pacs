#[doc = "Register `TIMER0_SYNC` reader"]
pub struct R(crate::R<TIMER0_SYNC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER0_SYNC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER0_SYNC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER0_SYNC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER0_SYNC` writer"]
pub struct W(crate::W<TIMER0_SYNC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER0_SYNC_SPEC>;
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
impl From<crate::W<TIMER0_SYNC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER0_SYNC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER0_SYNCI_EN` reader - When set, timer reloading with phase on sync input event is enabled."]
pub struct TIMER0_SYNCI_EN_R(crate::FieldReader<bool, bool>);
impl TIMER0_SYNCI_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER0_SYNCI_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER0_SYNCI_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER0_SYNCI_EN` writer - When set, timer reloading with phase on sync input event is enabled."]
pub struct TIMER0_SYNCI_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER0_SYNCI_EN_W<'a> {
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
#[doc = "Field `SW` reader - Toggling this bit will trigger a software sync."]
pub struct SW_R(crate::FieldReader<bool, bool>);
impl SW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW` writer - Toggling this bit will trigger a software sync."]
pub struct SW_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_W<'a> {
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
#[doc = "Field `TIMER0_SYNCO_SEL` reader - PWM timer0 sync_out selection, 0: synci, 1: TEZ, 2: TEP, otherwise:sync out is software sync"]
pub struct TIMER0_SYNCO_SEL_R(crate::FieldReader<u8, u8>);
impl TIMER0_SYNCO_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TIMER0_SYNCO_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER0_SYNCO_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER0_SYNCO_SEL` writer - PWM timer0 sync_out selection, 0: synci, 1: TEZ, 2: TEP, otherwise:sync out is software sync"]
pub struct TIMER0_SYNCO_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER0_SYNCO_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `TIMER0_PHASE` reader - phase for timer reload on sync event"]
pub struct TIMER0_PHASE_R(crate::FieldReader<u32, u32>);
impl TIMER0_PHASE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        TIMER0_PHASE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER0_PHASE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER0_PHASE` writer - phase for timer reload on sync event"]
pub struct TIMER0_PHASE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER0_PHASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0001_ffff << 4)) | ((value as u32 & 0x0001_ffff) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - When set, timer reloading with phase on sync input event is enabled."]
    #[inline(always)]
    pub fn timer0_synci_en(&self) -> TIMER0_SYNCI_EN_R {
        TIMER0_SYNCI_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Toggling this bit will trigger a software sync."]
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - PWM timer0 sync_out selection, 0: synci, 1: TEZ, 2: TEP, otherwise:sync out is software sync"]
    #[inline(always)]
    pub fn timer0_synco_sel(&self) -> TIMER0_SYNCO_SEL_R {
        TIMER0_SYNCO_SEL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:20 - phase for timer reload on sync event"]
    #[inline(always)]
    pub fn timer0_phase(&self) -> TIMER0_PHASE_R {
        TIMER0_PHASE_R::new(((self.bits >> 4) & 0x0001_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 0 - When set, timer reloading with phase on sync input event is enabled."]
    #[inline(always)]
    pub fn timer0_synci_en(&mut self) -> TIMER0_SYNCI_EN_W {
        TIMER0_SYNCI_EN_W { w: self }
    }
    #[doc = "Bit 1 - Toggling this bit will trigger a software sync."]
    #[inline(always)]
    pub fn sw(&mut self) -> SW_W {
        SW_W { w: self }
    }
    #[doc = "Bits 2:3 - PWM timer0 sync_out selection, 0: synci, 1: TEZ, 2: TEP, otherwise:sync out is software sync"]
    #[inline(always)]
    pub fn timer0_synco_sel(&mut self) -> TIMER0_SYNCO_SEL_W {
        TIMER0_SYNCO_SEL_W { w: self }
    }
    #[doc = "Bits 4:20 - phase for timer reload on sync event"]
    #[inline(always)]
    pub fn timer0_phase(&mut self) -> TIMER0_PHASE_W {
        TIMER0_PHASE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM timer0 sync function configuration register.\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer0_sync]
(index.html) module"]
pub struct TIMER0_SYNC_SPEC;
impl crate::RegisterSpec for TIMER0_SYNC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer0_sync::R]
(R) reader structure"]
impl crate::Readable for TIMER0_SYNC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer0_sync::W]
(W) writer structure"]
impl crate::Writable for TIMER0_SYNC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMER0_SYNC to value 0"]
impl crate::Resettable for TIMER0_SYNC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
