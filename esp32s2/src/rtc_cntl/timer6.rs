#[doc = "Register `TIMER6` reader"]
pub struct R(crate::R<TIMER6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER6` writer"]
pub struct W(crate::W<TIMER6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER6_SPEC>;
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
impl From<crate::W<TIMER6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DG_DCDC_WAIT_TIMER` reader - "]
pub struct DG_DCDC_WAIT_TIMER_R(crate::FieldReader<u16, u16>);
impl DG_DCDC_WAIT_TIMER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        DG_DCDC_WAIT_TIMER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DG_DCDC_WAIT_TIMER_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DG_DCDC_WAIT_TIMER` writer - "]
pub struct DG_DCDC_WAIT_TIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> DG_DCDC_WAIT_TIMER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 16)) | ((value as u32 & 0x01ff) << 16);
        self.w
    }
}
#[doc = "Field `DG_DCDC_POWERUP_TIMER` reader - "]
pub struct DG_DCDC_POWERUP_TIMER_R(crate::FieldReader<u8, u8>);
impl DG_DCDC_POWERUP_TIMER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DG_DCDC_POWERUP_TIMER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DG_DCDC_POWERUP_TIMER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DG_DCDC_POWERUP_TIMER` writer - "]
pub struct DG_DCDC_POWERUP_TIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> DG_DCDC_POWERUP_TIMER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 25)) | ((value as u32 & 0x7f) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:24"]
    #[inline(always)]
    pub fn dg_dcdc_wait_timer(&self) -> DG_DCDC_WAIT_TIMER_R {
        DG_DCDC_WAIT_TIMER_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bits 25:31"]
    #[inline(always)]
    pub fn dg_dcdc_powerup_timer(&self) -> DG_DCDC_POWERUP_TIMER_R {
        DG_DCDC_POWERUP_TIMER_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:24"]
    #[inline(always)]
    pub fn dg_dcdc_wait_timer(&mut self) -> DG_DCDC_WAIT_TIMER_W {
        DG_DCDC_WAIT_TIMER_W { w: self }
    }
    #[doc = "Bits 25:31"]
    #[inline(always)]
    pub fn dg_dcdc_powerup_timer(&mut self) -> DG_DCDC_POWERUP_TIMER_W {
        DG_DCDC_POWERUP_TIMER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configure minimal sleep cycles register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer6]
(index.html) module"]
pub struct TIMER6_SPEC;
impl crate::RegisterSpec for TIMER6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer6::R]
(R) reader structure"]
impl crate::Readable for TIMER6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer6::W]
(W) writer structure"]
impl crate::Writable for TIMER6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMER6 to value 0x1020_0000"]
impl crate::Resettable for TIMER6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1020_0000
    }
}
