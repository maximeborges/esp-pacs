#[doc = "Register `SAR_COCPU_STATE` reader"]
pub struct R(crate::R<SAR_COCPU_STATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_COCPU_STATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_COCPU_STATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_COCPU_STATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_COCPU_STATE` writer"]
pub struct W(crate::W<SAR_COCPU_STATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_COCPU_STATE_SPEC>;
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
impl From<crate::W<SAR_COCPU_STATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_COCPU_STATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COCPU_DBG_TRIGGER` writer - Trigger ULP-RISCV debug registers"]
pub struct COCPU_DBG_TRIGGER_W<'a> {
    w: &'a mut W,
}
impl<'a> COCPU_DBG_TRIGGER_W<'a> {
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
#[doc = "Field `COCPU_CLK_EN` reader - Check ULP-RISCV whether clk on"]
pub struct COCPU_CLK_EN_R(crate::FieldReader<bool, bool>);
impl COCPU_CLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COCPU_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COCPU_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COCPU_RESET_N` reader - Check ULP-RISCV whether in reset state"]
pub struct COCPU_RESET_N_R(crate::FieldReader<bool, bool>);
impl COCPU_RESET_N_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COCPU_RESET_N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COCPU_RESET_N_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COCPU_EOI` reader - Check ULP-RISCV whether in interrupt state"]
pub struct COCPU_EOI_R(crate::FieldReader<bool, bool>);
impl COCPU_EOI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COCPU_EOI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COCPU_EOI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COCPU_TRAP` reader - Check ULP-RISCV whether in trap state"]
pub struct COCPU_TRAP_R(crate::FieldReader<bool, bool>);
impl COCPU_TRAP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COCPU_TRAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COCPU_TRAP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COCPU_EBREAK` reader - Check ULP-RISCV whether in ebreak"]
pub struct COCPU_EBREAK_R(crate::FieldReader<bool, bool>);
impl COCPU_EBREAK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COCPU_EBREAK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COCPU_EBREAK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 26 - Check ULP-RISCV whether clk on"]
    #[inline(always)]
    pub fn cocpu_clk_en(&self) -> COCPU_CLK_EN_R {
        COCPU_CLK_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Check ULP-RISCV whether in reset state"]
    #[inline(always)]
    pub fn cocpu_reset_n(&self) -> COCPU_RESET_N_R {
        COCPU_RESET_N_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Check ULP-RISCV whether in interrupt state"]
    #[inline(always)]
    pub fn cocpu_eoi(&self) -> COCPU_EOI_R {
        COCPU_EOI_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Check ULP-RISCV whether in trap state"]
    #[inline(always)]
    pub fn cocpu_trap(&self) -> COCPU_TRAP_R {
        COCPU_TRAP_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Check ULP-RISCV whether in ebreak"]
    #[inline(always)]
    pub fn cocpu_ebreak(&self) -> COCPU_EBREAK_R {
        COCPU_EBREAK_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 25 - Trigger ULP-RISCV debug registers"]
    #[inline(always)]
    pub fn cocpu_dbg_trigger(&mut self) -> COCPU_DBG_TRIGGER_W {
        COCPU_DBG_TRIGGER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ULP-RISCV status\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_cocpu_state]
(index.html) module"]
pub struct SAR_COCPU_STATE_SPEC;
impl crate::RegisterSpec for SAR_COCPU_STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_cocpu_state::R]
(R) reader structure"]
impl crate::Readable for SAR_COCPU_STATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_cocpu_state::W]
(W) writer structure"]
impl crate::Writable for SAR_COCPU_STATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAR_COCPU_STATE to value 0"]
impl crate::Resettable for SAR_COCPU_STATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
