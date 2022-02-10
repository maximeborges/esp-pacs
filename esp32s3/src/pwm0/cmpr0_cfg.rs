#[doc = "Register `CMPR0_CFG` reader"]
pub struct R(crate::R<CMPR0_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMPR0_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMPR0_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMPR0_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMPR0_CFG` writer"]
pub struct W(crate::W<CMPR0_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMPR0_CFG_SPEC>;
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
impl From<crate::W<CMPR0_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMPR0_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMPR0_A_UPMETHOD` reader - Update method for PWM generator 0 time stamp A's active register. When all bits are set to 0: immediately, when bit0 is set to 1: TEZ, when bit1 is set to 1: TEP,when bit2 is set to 1: sync, when bit3 is set to 1: disable the update."]
pub struct CMPR0_A_UPMETHOD_R(crate::FieldReader<u8, u8>);
impl CMPR0_A_UPMETHOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CMPR0_A_UPMETHOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPR0_A_UPMETHOD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPR0_A_UPMETHOD` writer - Update method for PWM generator 0 time stamp A's active register. When all bits are set to 0: immediately, when bit0 is set to 1: TEZ, when bit1 is set to 1: TEP,when bit2 is set to 1: sync, when bit3 is set to 1: disable the update."]
pub struct CMPR0_A_UPMETHOD_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPR0_A_UPMETHOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `CMPR0_B_UPMETHOD` reader - Update method for PWM generator 0 time stamp B's active register. When all bits are set to 0: immediately, when bit0 is set to 1: TEZ, when bit1 is set to 1: TEP,when bit2 is set to 1: sync, when bit3 is set to 1: disable the update."]
pub struct CMPR0_B_UPMETHOD_R(crate::FieldReader<u8, u8>);
impl CMPR0_B_UPMETHOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CMPR0_B_UPMETHOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPR0_B_UPMETHOD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPR0_B_UPMETHOD` writer - Update method for PWM generator 0 time stamp B's active register. When all bits are set to 0: immediately, when bit0 is set to 1: TEZ, when bit1 is set to 1: TEP,when bit2 is set to 1: sync, when bit3 is set to 1: disable the update."]
pub struct CMPR0_B_UPMETHOD_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPR0_B_UPMETHOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `CMPR0_A_SHDW_FULL` reader - Set and reset by hardware. If set, PWM generator 0 time stamp A's shadow reg is filled and waiting to be transferred to A's active reg. If cleared, A's active reg has been updated with shadow register latest value"]
pub struct CMPR0_A_SHDW_FULL_R(crate::FieldReader<bool, bool>);
impl CMPR0_A_SHDW_FULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMPR0_A_SHDW_FULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPR0_A_SHDW_FULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPR0_B_SHDW_FULL` reader - Set and reset by hardware. If set, PWM generator 0 time stamp B's shadow reg is filled and waiting to be transferred to B's active reg. If cleared, B's active reg has been updated with shadow register latest value"]
pub struct CMPR0_B_SHDW_FULL_R(crate::FieldReader<bool, bool>);
impl CMPR0_B_SHDW_FULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMPR0_B_SHDW_FULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPR0_B_SHDW_FULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - Update method for PWM generator 0 time stamp A's active register. When all bits are set to 0: immediately, when bit0 is set to 1: TEZ, when bit1 is set to 1: TEP,when bit2 is set to 1: sync, when bit3 is set to 1: disable the update."]
    #[inline(always)]
    pub fn cmpr0_a_upmethod(&self) -> CMPR0_A_UPMETHOD_R {
        CMPR0_A_UPMETHOD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Update method for PWM generator 0 time stamp B's active register. When all bits are set to 0: immediately, when bit0 is set to 1: TEZ, when bit1 is set to 1: TEP,when bit2 is set to 1: sync, when bit3 is set to 1: disable the update."]
    #[inline(always)]
    pub fn cmpr0_b_upmethod(&self) -> CMPR0_B_UPMETHOD_R {
        CMPR0_B_UPMETHOD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Set and reset by hardware. If set, PWM generator 0 time stamp A's shadow reg is filled and waiting to be transferred to A's active reg. If cleared, A's active reg has been updated with shadow register latest value"]
    #[inline(always)]
    pub fn cmpr0_a_shdw_full(&self) -> CMPR0_A_SHDW_FULL_R {
        CMPR0_A_SHDW_FULL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Set and reset by hardware. If set, PWM generator 0 time stamp B's shadow reg is filled and waiting to be transferred to B's active reg. If cleared, B's active reg has been updated with shadow register latest value"]
    #[inline(always)]
    pub fn cmpr0_b_shdw_full(&self) -> CMPR0_B_SHDW_FULL_R {
        CMPR0_B_SHDW_FULL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Update method for PWM generator 0 time stamp A's active register. When all bits are set to 0: immediately, when bit0 is set to 1: TEZ, when bit1 is set to 1: TEP,when bit2 is set to 1: sync, when bit3 is set to 1: disable the update."]
    #[inline(always)]
    pub fn cmpr0_a_upmethod(&mut self) -> CMPR0_A_UPMETHOD_W {
        CMPR0_A_UPMETHOD_W { w: self }
    }
    #[doc = "Bits 4:7 - Update method for PWM generator 0 time stamp B's active register. When all bits are set to 0: immediately, when bit0 is set to 1: TEZ, when bit1 is set to 1: TEP,when bit2 is set to 1: sync, when bit3 is set to 1: disable the update."]
    #[inline(always)]
    pub fn cmpr0_b_upmethod(&mut self) -> CMPR0_B_UPMETHOD_W {
        CMPR0_B_UPMETHOD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transfer status and update method for time stamp registers A and B\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpr0_cfg]
(index.html) module"]
pub struct CMPR0_CFG_SPEC;
impl crate::RegisterSpec for CMPR0_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmpr0_cfg::R]
(R) reader structure"]
impl crate::Readable for CMPR0_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmpr0_cfg::W]
(W) writer structure"]
impl crate::Writable for CMPR0_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMPR0_CFG to value 0"]
impl crate::Resettable for CMPR0_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
