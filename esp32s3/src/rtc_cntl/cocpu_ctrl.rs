#[doc = "Register `COCPU_CTRL` reader"]
pub struct R(crate::R<COCPU_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COCPU_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COCPU_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COCPU_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COCPU_CTRL` writer"]
pub struct W(crate::W<COCPU_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COCPU_CTRL_SPEC>;
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
impl From<crate::W<COCPU_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COCPU_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COCPU_CLK_FO` reader - cocpu clk force on"]
pub struct COCPU_CLK_FO_R(crate::FieldReader<bool, bool>);
impl COCPU_CLK_FO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COCPU_CLK_FO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COCPU_CLK_FO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COCPU_CLK_FO` writer - cocpu clk force on"]
pub struct COCPU_CLK_FO_W<'a> {
    w: &'a mut W,
}
impl<'a> COCPU_CLK_FO_W<'a> {
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
#[doc = "Field `COCPU_START_2_RESET_DIS` reader - time from start cocpu to pull down reset"]
pub struct COCPU_START_2_RESET_DIS_R(crate::FieldReader<u8, u8>);
impl COCPU_START_2_RESET_DIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        COCPU_START_2_RESET_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COCPU_START_2_RESET_DIS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COCPU_START_2_RESET_DIS` writer - time from start cocpu to pull down reset"]
pub struct COCPU_START_2_RESET_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> COCPU_START_2_RESET_DIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 1)) | ((value as u32 & 0x3f) << 1);
        self.w
    }
}
#[doc = "Field `COCPU_START_2_INTR_EN` reader - time from start cocpu to give start interrupt"]
pub struct COCPU_START_2_INTR_EN_R(crate::FieldReader<u8, u8>);
impl COCPU_START_2_INTR_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        COCPU_START_2_INTR_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COCPU_START_2_INTR_EN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COCPU_START_2_INTR_EN` writer - time from start cocpu to give start interrupt"]
pub struct COCPU_START_2_INTR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> COCPU_START_2_INTR_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 7)) | ((value as u32 & 0x3f) << 7);
        self.w
    }
}
#[doc = "Field `COCPU_SHUT` reader - to shut cocpu"]
pub struct COCPU_SHUT_R(crate::FieldReader<bool, bool>);
impl COCPU_SHUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COCPU_SHUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COCPU_SHUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COCPU_SHUT` writer - to shut cocpu"]
pub struct COCPU_SHUT_W<'a> {
    w: &'a mut W,
}
impl<'a> COCPU_SHUT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `COCPU_SHUT_2_CLK_DIS` reader - time from shut cocpu to disable clk"]
pub struct COCPU_SHUT_2_CLK_DIS_R(crate::FieldReader<u8, u8>);
impl COCPU_SHUT_2_CLK_DIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        COCPU_SHUT_2_CLK_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COCPU_SHUT_2_CLK_DIS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COCPU_SHUT_2_CLK_DIS` writer - time from shut cocpu to disable clk"]
pub struct COCPU_SHUT_2_CLK_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> COCPU_SHUT_2_CLK_DIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 14)) | ((value as u32 & 0xff) << 14);
        self.w
    }
}
#[doc = "Field `COCPU_SHUT_RESET_EN` reader - to reset cocpu"]
pub struct COCPU_SHUT_RESET_EN_R(crate::FieldReader<bool, bool>);
impl COCPU_SHUT_RESET_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COCPU_SHUT_RESET_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COCPU_SHUT_RESET_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COCPU_SHUT_RESET_EN` writer - to reset cocpu"]
pub struct COCPU_SHUT_RESET_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> COCPU_SHUT_RESET_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `COCPU_SEL` reader - 1: old ULP 0: new riscV"]
pub struct COCPU_SEL_R(crate::FieldReader<bool, bool>);
impl COCPU_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COCPU_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COCPU_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COCPU_SEL` writer - 1: old ULP 0: new riscV"]
pub struct COCPU_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COCPU_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `COCPU_DONE_FORCE` reader - 1: select riscv done 0: select ulp done"]
pub struct COCPU_DONE_FORCE_R(crate::FieldReader<bool, bool>);
impl COCPU_DONE_FORCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COCPU_DONE_FORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COCPU_DONE_FORCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COCPU_DONE_FORCE` writer - 1: select riscv done 0: select ulp done"]
pub struct COCPU_DONE_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> COCPU_DONE_FORCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `COCPU_DONE` reader - done signal used by riscv to control timer."]
pub struct COCPU_DONE_R(crate::FieldReader<bool, bool>);
impl COCPU_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COCPU_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COCPU_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COCPU_DONE` writer - done signal used by riscv to control timer."]
pub struct COCPU_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> COCPU_DONE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `COCPU_SW_INT_TRIGGER` writer - trigger cocpu register interrupt"]
pub struct COCPU_SW_INT_TRIGGER_W<'a> {
    w: &'a mut W,
}
impl<'a> COCPU_SW_INT_TRIGGER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `COCPU_CLKGATE_EN` reader - open ulp-riscv clk gate"]
pub struct COCPU_CLKGATE_EN_R(crate::FieldReader<bool, bool>);
impl COCPU_CLKGATE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COCPU_CLKGATE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COCPU_CLKGATE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COCPU_CLKGATE_EN` writer - open ulp-riscv clk gate"]
pub struct COCPU_CLKGATE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> COCPU_CLKGATE_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - cocpu clk force on"]
    #[inline(always)]
    pub fn cocpu_clk_fo(&self) -> COCPU_CLK_FO_R {
        COCPU_CLK_FO_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:6 - time from start cocpu to pull down reset"]
    #[inline(always)]
    pub fn cocpu_start_2_reset_dis(&self) -> COCPU_START_2_RESET_DIS_R {
        COCPU_START_2_RESET_DIS_R::new(((self.bits >> 1) & 0x3f) as u8)
    }
    #[doc = "Bits 7:12 - time from start cocpu to give start interrupt"]
    #[inline(always)]
    pub fn cocpu_start_2_intr_en(&self) -> COCPU_START_2_INTR_EN_R {
        COCPU_START_2_INTR_EN_R::new(((self.bits >> 7) & 0x3f) as u8)
    }
    #[doc = "Bit 13 - to shut cocpu"]
    #[inline(always)]
    pub fn cocpu_shut(&self) -> COCPU_SHUT_R {
        COCPU_SHUT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 14:21 - time from shut cocpu to disable clk"]
    #[inline(always)]
    pub fn cocpu_shut_2_clk_dis(&self) -> COCPU_SHUT_2_CLK_DIS_R {
        COCPU_SHUT_2_CLK_DIS_R::new(((self.bits >> 14) & 0xff) as u8)
    }
    #[doc = "Bit 22 - to reset cocpu"]
    #[inline(always)]
    pub fn cocpu_shut_reset_en(&self) -> COCPU_SHUT_RESET_EN_R {
        COCPU_SHUT_RESET_EN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - 1: old ULP 0: new riscV"]
    #[inline(always)]
    pub fn cocpu_sel(&self) -> COCPU_SEL_R {
        COCPU_SEL_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - 1: select riscv done 0: select ulp done"]
    #[inline(always)]
    pub fn cocpu_done_force(&self) -> COCPU_DONE_FORCE_R {
        COCPU_DONE_FORCE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - done signal used by riscv to control timer."]
    #[inline(always)]
    pub fn cocpu_done(&self) -> COCPU_DONE_R {
        COCPU_DONE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 27 - open ulp-riscv clk gate"]
    #[inline(always)]
    pub fn cocpu_clkgate_en(&self) -> COCPU_CLKGATE_EN_R {
        COCPU_CLKGATE_EN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - cocpu clk force on"]
    #[inline(always)]
    pub fn cocpu_clk_fo(&mut self) -> COCPU_CLK_FO_W {
        COCPU_CLK_FO_W { w: self }
    }
    #[doc = "Bits 1:6 - time from start cocpu to pull down reset"]
    #[inline(always)]
    pub fn cocpu_start_2_reset_dis(&mut self) -> COCPU_START_2_RESET_DIS_W {
        COCPU_START_2_RESET_DIS_W { w: self }
    }
    #[doc = "Bits 7:12 - time from start cocpu to give start interrupt"]
    #[inline(always)]
    pub fn cocpu_start_2_intr_en(&mut self) -> COCPU_START_2_INTR_EN_W {
        COCPU_START_2_INTR_EN_W { w: self }
    }
    #[doc = "Bit 13 - to shut cocpu"]
    #[inline(always)]
    pub fn cocpu_shut(&mut self) -> COCPU_SHUT_W {
        COCPU_SHUT_W { w: self }
    }
    #[doc = "Bits 14:21 - time from shut cocpu to disable clk"]
    #[inline(always)]
    pub fn cocpu_shut_2_clk_dis(&mut self) -> COCPU_SHUT_2_CLK_DIS_W {
        COCPU_SHUT_2_CLK_DIS_W { w: self }
    }
    #[doc = "Bit 22 - to reset cocpu"]
    #[inline(always)]
    pub fn cocpu_shut_reset_en(&mut self) -> COCPU_SHUT_RESET_EN_W {
        COCPU_SHUT_RESET_EN_W { w: self }
    }
    #[doc = "Bit 23 - 1: old ULP 0: new riscV"]
    #[inline(always)]
    pub fn cocpu_sel(&mut self) -> COCPU_SEL_W {
        COCPU_SEL_W { w: self }
    }
    #[doc = "Bit 24 - 1: select riscv done 0: select ulp done"]
    #[inline(always)]
    pub fn cocpu_done_force(&mut self) -> COCPU_DONE_FORCE_W {
        COCPU_DONE_FORCE_W { w: self }
    }
    #[doc = "Bit 25 - done signal used by riscv to control timer."]
    #[inline(always)]
    pub fn cocpu_done(&mut self) -> COCPU_DONE_W {
        COCPU_DONE_W { w: self }
    }
    #[doc = "Bit 26 - trigger cocpu register interrupt"]
    #[inline(always)]
    pub fn cocpu_sw_int_trigger(&mut self) -> COCPU_SW_INT_TRIGGER_W {
        COCPU_SW_INT_TRIGGER_W { w: self }
    }
    #[doc = "Bit 27 - open ulp-riscv clk gate"]
    #[inline(always)]
    pub fn cocpu_clkgate_en(&mut self) -> COCPU_CLKGATE_EN_W {
        COCPU_CLKGATE_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure ulp-riscv\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cocpu_ctrl]
(index.html) module"]
pub struct COCPU_CTRL_SPEC;
impl crate::RegisterSpec for COCPU_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cocpu_ctrl::R]
(R) reader structure"]
impl crate::Readable for COCPU_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cocpu_ctrl::W]
(W) writer structure"]
impl crate::Writable for COCPU_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COCPU_CTRL to value 0x008a_0810"]
impl crate::Resettable for COCPU_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x008a_0810
    }
}
