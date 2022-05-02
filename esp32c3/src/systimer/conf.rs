#[doc = "Register `CONF` reader"]
pub struct R(crate::R<CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONF` writer"]
pub struct W(crate::W<CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONF_SPEC>;
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
impl From<crate::W<CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSTIMER_CLK_FO` reader - systimer clock force on"]
pub struct SYSTIMER_CLK_FO_R(crate::FieldReader<bool>);
impl SYSTIMER_CLK_FO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SYSTIMER_CLK_FO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYSTIMER_CLK_FO_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSTIMER_CLK_FO` writer - systimer clock force on"]
pub struct SYSTIMER_CLK_FO_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTIMER_CLK_FO_W<'a> {
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
#[doc = "Field `TARGET2_WORK_EN` reader - target2 work enable"]
pub struct TARGET2_WORK_EN_R(crate::FieldReader<bool>);
impl TARGET2_WORK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TARGET2_WORK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TARGET2_WORK_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TARGET2_WORK_EN` writer - target2 work enable"]
pub struct TARGET2_WORK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TARGET2_WORK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 22)) | ((value as u32 & 1) << 22);
        self.w
    }
}
#[doc = "Field `TARGET1_WORK_EN` reader - target1 work enable"]
pub struct TARGET1_WORK_EN_R(crate::FieldReader<bool>);
impl TARGET1_WORK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TARGET1_WORK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TARGET1_WORK_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TARGET1_WORK_EN` writer - target1 work enable"]
pub struct TARGET1_WORK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TARGET1_WORK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 23)) | ((value as u32 & 1) << 23);
        self.w
    }
}
#[doc = "Field `TARGET0_WORK_EN` reader - target0 work enable"]
pub struct TARGET0_WORK_EN_R(crate::FieldReader<bool>);
impl TARGET0_WORK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TARGET0_WORK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TARGET0_WORK_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TARGET0_WORK_EN` writer - target0 work enable"]
pub struct TARGET0_WORK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TARGET0_WORK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 24)) | ((value as u32 & 1) << 24);
        self.w
    }
}
#[doc = "Field `TIMER_UNIT1_CORE1_STALL_EN` reader - If timer unit1 is stalled when core1 stalled"]
pub struct TIMER_UNIT1_CORE1_STALL_EN_R(crate::FieldReader<bool>);
impl TIMER_UNIT1_CORE1_STALL_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER_UNIT1_CORE1_STALL_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER_UNIT1_CORE1_STALL_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER_UNIT1_CORE1_STALL_EN` writer - If timer unit1 is stalled when core1 stalled"]
pub struct TIMER_UNIT1_CORE1_STALL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_UNIT1_CORE1_STALL_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 25)) | ((value as u32 & 1) << 25);
        self.w
    }
}
#[doc = "Field `TIMER_UNIT1_CORE0_STALL_EN` reader - If timer unit1 is stalled when core0 stalled"]
pub struct TIMER_UNIT1_CORE0_STALL_EN_R(crate::FieldReader<bool>);
impl TIMER_UNIT1_CORE0_STALL_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER_UNIT1_CORE0_STALL_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER_UNIT1_CORE0_STALL_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER_UNIT1_CORE0_STALL_EN` writer - If timer unit1 is stalled when core0 stalled"]
pub struct TIMER_UNIT1_CORE0_STALL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_UNIT1_CORE0_STALL_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 26)) | ((value as u32 & 1) << 26);
        self.w
    }
}
#[doc = "Field `TIMER_UNIT0_CORE1_STALL_EN` reader - If timer unit0 is stalled when core1 stalled"]
pub struct TIMER_UNIT0_CORE1_STALL_EN_R(crate::FieldReader<bool>);
impl TIMER_UNIT0_CORE1_STALL_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER_UNIT0_CORE1_STALL_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER_UNIT0_CORE1_STALL_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER_UNIT0_CORE1_STALL_EN` writer - If timer unit0 is stalled when core1 stalled"]
pub struct TIMER_UNIT0_CORE1_STALL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_UNIT0_CORE1_STALL_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 27)) | ((value as u32 & 1) << 27);
        self.w
    }
}
#[doc = "Field `TIMER_UNIT0_CORE0_STALL_EN` reader - If timer unit0 is stalled when core0 stalled"]
pub struct TIMER_UNIT0_CORE0_STALL_EN_R(crate::FieldReader<bool>);
impl TIMER_UNIT0_CORE0_STALL_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER_UNIT0_CORE0_STALL_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER_UNIT0_CORE0_STALL_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER_UNIT0_CORE0_STALL_EN` writer - If timer unit0 is stalled when core0 stalled"]
pub struct TIMER_UNIT0_CORE0_STALL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_UNIT0_CORE0_STALL_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 28)) | ((value as u32 & 1) << 28);
        self.w
    }
}
#[doc = "Field `TIMER_UNIT1_WORK_EN` reader - timer unit1 work enable"]
pub struct TIMER_UNIT1_WORK_EN_R(crate::FieldReader<bool>);
impl TIMER_UNIT1_WORK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER_UNIT1_WORK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER_UNIT1_WORK_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER_UNIT1_WORK_EN` writer - timer unit1 work enable"]
pub struct TIMER_UNIT1_WORK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_UNIT1_WORK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 29)) | ((value as u32 & 1) << 29);
        self.w
    }
}
#[doc = "Field `TIMER_UNIT0_WORK_EN` reader - timer unit0 work enable"]
pub struct TIMER_UNIT0_WORK_EN_R(crate::FieldReader<bool>);
impl TIMER_UNIT0_WORK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER_UNIT0_WORK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER_UNIT0_WORK_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER_UNIT0_WORK_EN` writer - timer unit0 work enable"]
pub struct TIMER_UNIT0_WORK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_UNIT0_WORK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 30)) | ((value as u32 & 1) << 30);
        self.w
    }
}
#[doc = "Field `CLK_EN` reader - register file clk gating"]
pub struct CLK_EN_R(crate::FieldReader<bool>);
impl CLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK_EN` writer - register file clk gating"]
pub struct CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 31)) | ((value as u32 & 1) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - systimer clock force on"]
    #[inline(always)]
    pub fn systimer_clk_fo(&self) -> SYSTIMER_CLK_FO_R {
        SYSTIMER_CLK_FO_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 22 - target2 work enable"]
    #[inline(always)]
    pub fn target2_work_en(&self) -> TARGET2_WORK_EN_R {
        TARGET2_WORK_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - target1 work enable"]
    #[inline(always)]
    pub fn target1_work_en(&self) -> TARGET1_WORK_EN_R {
        TARGET1_WORK_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - target0 work enable"]
    #[inline(always)]
    pub fn target0_work_en(&self) -> TARGET0_WORK_EN_R {
        TARGET0_WORK_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - If timer unit1 is stalled when core1 stalled"]
    #[inline(always)]
    pub fn timer_unit1_core1_stall_en(&self) -> TIMER_UNIT1_CORE1_STALL_EN_R {
        TIMER_UNIT1_CORE1_STALL_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - If timer unit1 is stalled when core0 stalled"]
    #[inline(always)]
    pub fn timer_unit1_core0_stall_en(&self) -> TIMER_UNIT1_CORE0_STALL_EN_R {
        TIMER_UNIT1_CORE0_STALL_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - If timer unit0 is stalled when core1 stalled"]
    #[inline(always)]
    pub fn timer_unit0_core1_stall_en(&self) -> TIMER_UNIT0_CORE1_STALL_EN_R {
        TIMER_UNIT0_CORE1_STALL_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - If timer unit0 is stalled when core0 stalled"]
    #[inline(always)]
    pub fn timer_unit0_core0_stall_en(&self) -> TIMER_UNIT0_CORE0_STALL_EN_R {
        TIMER_UNIT0_CORE0_STALL_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - timer unit1 work enable"]
    #[inline(always)]
    pub fn timer_unit1_work_en(&self) -> TIMER_UNIT1_WORK_EN_R {
        TIMER_UNIT1_WORK_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - timer unit0 work enable"]
    #[inline(always)]
    pub fn timer_unit0_work_en(&self) -> TIMER_UNIT0_WORK_EN_R {
        TIMER_UNIT0_WORK_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - register file clk gating"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - systimer clock force on"]
    #[inline(always)]
    pub fn systimer_clk_fo(&mut self) -> SYSTIMER_CLK_FO_W {
        SYSTIMER_CLK_FO_W { w: self }
    }
    #[doc = "Bit 22 - target2 work enable"]
    #[inline(always)]
    pub fn target2_work_en(&mut self) -> TARGET2_WORK_EN_W {
        TARGET2_WORK_EN_W { w: self }
    }
    #[doc = "Bit 23 - target1 work enable"]
    #[inline(always)]
    pub fn target1_work_en(&mut self) -> TARGET1_WORK_EN_W {
        TARGET1_WORK_EN_W { w: self }
    }
    #[doc = "Bit 24 - target0 work enable"]
    #[inline(always)]
    pub fn target0_work_en(&mut self) -> TARGET0_WORK_EN_W {
        TARGET0_WORK_EN_W { w: self }
    }
    #[doc = "Bit 25 - If timer unit1 is stalled when core1 stalled"]
    #[inline(always)]
    pub fn timer_unit1_core1_stall_en(&mut self) -> TIMER_UNIT1_CORE1_STALL_EN_W {
        TIMER_UNIT1_CORE1_STALL_EN_W { w: self }
    }
    #[doc = "Bit 26 - If timer unit1 is stalled when core0 stalled"]
    #[inline(always)]
    pub fn timer_unit1_core0_stall_en(&mut self) -> TIMER_UNIT1_CORE0_STALL_EN_W {
        TIMER_UNIT1_CORE0_STALL_EN_W { w: self }
    }
    #[doc = "Bit 27 - If timer unit0 is stalled when core1 stalled"]
    #[inline(always)]
    pub fn timer_unit0_core1_stall_en(&mut self) -> TIMER_UNIT0_CORE1_STALL_EN_W {
        TIMER_UNIT0_CORE1_STALL_EN_W { w: self }
    }
    #[doc = "Bit 28 - If timer unit0 is stalled when core0 stalled"]
    #[inline(always)]
    pub fn timer_unit0_core0_stall_en(&mut self) -> TIMER_UNIT0_CORE0_STALL_EN_W {
        TIMER_UNIT0_CORE0_STALL_EN_W { w: self }
    }
    #[doc = "Bit 29 - timer unit1 work enable"]
    #[inline(always)]
    pub fn timer_unit1_work_en(&mut self) -> TIMER_UNIT1_WORK_EN_W {
        TIMER_UNIT1_WORK_EN_W { w: self }
    }
    #[doc = "Bit 30 - timer unit0 work enable"]
    #[inline(always)]
    pub fn timer_unit0_work_en(&mut self) -> TIMER_UNIT0_WORK_EN_W {
        TIMER_UNIT0_WORK_EN_W { w: self }
    }
    #[doc = "Bit 31 - register file clk gating"]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W {
        CLK_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYSTIMER_CONF.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conf](index.html) module"]
pub struct CONF_SPEC;
impl crate::RegisterSpec for CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conf::R](R) reader structure"]
impl crate::Readable for CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conf::W](W) writer structure"]
impl crate::Writable for CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONF to value 0x4600_0000"]
impl crate::Resettable for CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4600_0000
    }
}
