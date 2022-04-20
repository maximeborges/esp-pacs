#[doc = "Register `UPDATE_CFG` reader"]
pub struct R(crate::R<UPDATE_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UPDATE_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UPDATE_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UPDATE_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UPDATE_CFG` writer"]
pub struct W(crate::W<UPDATE_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UPDATE_CFG_SPEC>;
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
impl From<crate::W<UPDATE_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UPDATE_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GLOBAL_UP_EN` reader - The global enable of update of all active registers in MCPWM module"]
pub struct GLOBAL_UP_EN_R(crate::FieldReader<bool, bool>);
impl GLOBAL_UP_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GLOBAL_UP_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GLOBAL_UP_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GLOBAL_UP_EN` writer - The global enable of update of all active registers in MCPWM module"]
pub struct GLOBAL_UP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GLOBAL_UP_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
#[doc = "Field `GLOBAL_FORCE_UP` reader - a toggle (software invert its value) will trigger a forced update of all active registers in MCPWM module"]
pub struct GLOBAL_FORCE_UP_R(crate::FieldReader<bool, bool>);
impl GLOBAL_FORCE_UP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GLOBAL_FORCE_UP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GLOBAL_FORCE_UP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GLOBAL_FORCE_UP` writer - a toggle (software invert its value) will trigger a forced update of all active registers in MCPWM module"]
pub struct GLOBAL_FORCE_UP_W<'a> {
    w: &'a mut W,
}
impl<'a> GLOBAL_FORCE_UP_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u32 & 1) << 1);
        self.w
    }
}
#[doc = "Field `OP0_UP_EN` reader - When set and PWM_GLOBAL_UP_EN is set, update of active registers in PWM operator 0 are enabled"]
pub struct OP0_UP_EN_R(crate::FieldReader<bool, bool>);
impl OP0_UP_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OP0_UP_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OP0_UP_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OP0_UP_EN` writer - When set and PWM_GLOBAL_UP_EN is set, update of active registers in PWM operator 0 are enabled"]
pub struct OP0_UP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OP0_UP_EN_W<'a> {
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
#[doc = "Field `OP0_FORCE_UP` reader - a toggle (software invert its value) will trigger a forced update of active registers in PWM operator 0"]
pub struct OP0_FORCE_UP_R(crate::FieldReader<bool, bool>);
impl OP0_FORCE_UP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OP0_FORCE_UP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OP0_FORCE_UP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OP0_FORCE_UP` writer - a toggle (software invert its value) will trigger a forced update of active registers in PWM operator 0"]
pub struct OP0_FORCE_UP_W<'a> {
    w: &'a mut W,
}
impl<'a> OP0_FORCE_UP_W<'a> {
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
#[doc = "Field `OP1_UP_EN` reader - When set and PWM_GLOBAL_UP_EN is set, update of active registers in PWM operator 1 are enabled"]
pub struct OP1_UP_EN_R(crate::FieldReader<bool, bool>);
impl OP1_UP_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OP1_UP_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OP1_UP_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OP1_UP_EN` writer - When set and PWM_GLOBAL_UP_EN is set, update of active registers in PWM operator 1 are enabled"]
pub struct OP1_UP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OP1_UP_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u32 & 1) << 4);
        self.w
    }
}
#[doc = "Field `OP1_FORCE_UP` reader - a toggle (software invert its value) will trigger a forced update of active registers in PWM operator 1"]
pub struct OP1_FORCE_UP_R(crate::FieldReader<bool, bool>);
impl OP1_FORCE_UP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OP1_FORCE_UP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OP1_FORCE_UP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OP1_FORCE_UP` writer - a toggle (software invert its value) will trigger a forced update of active registers in PWM operator 1"]
pub struct OP1_FORCE_UP_W<'a> {
    w: &'a mut W,
}
impl<'a> OP1_FORCE_UP_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u32 & 1) << 5);
        self.w
    }
}
#[doc = "Field `OP2_UP_EN` reader - When set and PWM_GLOBAL_UP_EN is set, update of active registers in PWM operator 2 are enabled"]
pub struct OP2_UP_EN_R(crate::FieldReader<bool, bool>);
impl OP2_UP_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OP2_UP_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OP2_UP_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OP2_UP_EN` writer - When set and PWM_GLOBAL_UP_EN is set, update of active registers in PWM operator 2 are enabled"]
pub struct OP2_UP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OP2_UP_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 6)) | ((value as u32 & 1) << 6);
        self.w
    }
}
#[doc = "Field `OP2_FORCE_UP` reader - a toggle (software invert its value) will trigger a forced update of active registers in PWM operator 2"]
pub struct OP2_FORCE_UP_R(crate::FieldReader<bool, bool>);
impl OP2_FORCE_UP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OP2_FORCE_UP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OP2_FORCE_UP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OP2_FORCE_UP` writer - a toggle (software invert its value) will trigger a forced update of active registers in PWM operator 2"]
pub struct OP2_FORCE_UP_W<'a> {
    w: &'a mut W,
}
impl<'a> OP2_FORCE_UP_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 7)) | ((value as u32 & 1) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - The global enable of update of all active registers in MCPWM module"]
    #[inline(always)]
    pub fn global_up_en(&self) -> GLOBAL_UP_EN_R {
        GLOBAL_UP_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - a toggle (software invert its value) will trigger a forced update of all active registers in MCPWM module"]
    #[inline(always)]
    pub fn global_force_up(&self) -> GLOBAL_FORCE_UP_R {
        GLOBAL_FORCE_UP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When set and PWM_GLOBAL_UP_EN is set, update of active registers in PWM operator 0 are enabled"]
    #[inline(always)]
    pub fn op0_up_en(&self) -> OP0_UP_EN_R {
        OP0_UP_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - a toggle (software invert its value) will trigger a forced update of active registers in PWM operator 0"]
    #[inline(always)]
    pub fn op0_force_up(&self) -> OP0_FORCE_UP_R {
        OP0_FORCE_UP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When set and PWM_GLOBAL_UP_EN is set, update of active registers in PWM operator 1 are enabled"]
    #[inline(always)]
    pub fn op1_up_en(&self) -> OP1_UP_EN_R {
        OP1_UP_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - a toggle (software invert its value) will trigger a forced update of active registers in PWM operator 1"]
    #[inline(always)]
    pub fn op1_force_up(&self) -> OP1_FORCE_UP_R {
        OP1_FORCE_UP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - When set and PWM_GLOBAL_UP_EN is set, update of active registers in PWM operator 2 are enabled"]
    #[inline(always)]
    pub fn op2_up_en(&self) -> OP2_UP_EN_R {
        OP2_UP_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - a toggle (software invert its value) will trigger a forced update of active registers in PWM operator 2"]
    #[inline(always)]
    pub fn op2_force_up(&self) -> OP2_FORCE_UP_R {
        OP2_FORCE_UP_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The global enable of update of all active registers in MCPWM module"]
    #[inline(always)]
    pub fn global_up_en(&mut self) -> GLOBAL_UP_EN_W {
        GLOBAL_UP_EN_W { w: self }
    }
    #[doc = "Bit 1 - a toggle (software invert its value) will trigger a forced update of all active registers in MCPWM module"]
    #[inline(always)]
    pub fn global_force_up(&mut self) -> GLOBAL_FORCE_UP_W {
        GLOBAL_FORCE_UP_W { w: self }
    }
    #[doc = "Bit 2 - When set and PWM_GLOBAL_UP_EN is set, update of active registers in PWM operator 0 are enabled"]
    #[inline(always)]
    pub fn op0_up_en(&mut self) -> OP0_UP_EN_W {
        OP0_UP_EN_W { w: self }
    }
    #[doc = "Bit 3 - a toggle (software invert its value) will trigger a forced update of active registers in PWM operator 0"]
    #[inline(always)]
    pub fn op0_force_up(&mut self) -> OP0_FORCE_UP_W {
        OP0_FORCE_UP_W { w: self }
    }
    #[doc = "Bit 4 - When set and PWM_GLOBAL_UP_EN is set, update of active registers in PWM operator 1 are enabled"]
    #[inline(always)]
    pub fn op1_up_en(&mut self) -> OP1_UP_EN_W {
        OP1_UP_EN_W { w: self }
    }
    #[doc = "Bit 5 - a toggle (software invert its value) will trigger a forced update of active registers in PWM operator 1"]
    #[inline(always)]
    pub fn op1_force_up(&mut self) -> OP1_FORCE_UP_W {
        OP1_FORCE_UP_W { w: self }
    }
    #[doc = "Bit 6 - When set and PWM_GLOBAL_UP_EN is set, update of active registers in PWM operator 2 are enabled"]
    #[inline(always)]
    pub fn op2_up_en(&mut self) -> OP2_UP_EN_W {
        OP2_UP_EN_W { w: self }
    }
    #[doc = "Bit 7 - a toggle (software invert its value) will trigger a forced update of active registers in PWM operator 2"]
    #[inline(always)]
    pub fn op2_force_up(&mut self) -> OP2_FORCE_UP_W {
        OP2_FORCE_UP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enable update.\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [update_cfg]
(index.html) module"]
pub struct UPDATE_CFG_SPEC;
impl crate::RegisterSpec for UPDATE_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [update_cfg::R]
(R) reader structure"]
impl crate::Readable for UPDATE_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [update_cfg::W]
(W) writer structure"]
impl crate::Writable for UPDATE_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UPDATE_CFG to value 0x55"]
impl crate::Resettable for UPDATE_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x55
    }
}
