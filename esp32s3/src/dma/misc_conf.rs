#[doc = "Register `MISC_CONF` reader"]
pub struct R(crate::R<MISC_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MISC_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MISC_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MISC_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MISC_CONF` writer"]
pub struct W(crate::W<MISC_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MISC_CONF_SPEC>;
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
impl From<crate::W<MISC_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MISC_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AHBM_RST_INTER` reader - Set this bit, then clear this bit to reset the internal ahb FSM."]
pub struct AHBM_RST_INTER_R(crate::FieldReader<bool>);
impl AHBM_RST_INTER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AHBM_RST_INTER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AHBM_RST_INTER_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AHBM_RST_INTER` writer - Set this bit, then clear this bit to reset the internal ahb FSM."]
pub struct AHBM_RST_INTER_W<'a> {
    w: &'a mut W,
}
impl<'a> AHBM_RST_INTER_W<'a> {
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
#[doc = "Field `AHBM_RST_EXTER` reader - Set this bit, then clear this bit to reset the external ahb FSM."]
pub struct AHBM_RST_EXTER_R(crate::FieldReader<bool>);
impl AHBM_RST_EXTER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AHBM_RST_EXTER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AHBM_RST_EXTER_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AHBM_RST_EXTER` writer - Set this bit, then clear this bit to reset the external ahb FSM."]
pub struct AHBM_RST_EXTER_W<'a> {
    w: &'a mut W,
}
impl<'a> AHBM_RST_EXTER_W<'a> {
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
#[doc = "Field `ARB_PRI_DIS` reader - Set this bit to disable priority arbitration function."]
pub struct ARB_PRI_DIS_R(crate::FieldReader<bool>);
impl ARB_PRI_DIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ARB_PRI_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARB_PRI_DIS_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARB_PRI_DIS` writer - Set this bit to disable priority arbitration function."]
pub struct ARB_PRI_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> ARB_PRI_DIS_W<'a> {
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
#[doc = "Field `CLK_EN` reader - 1'h1: Force clock on for register. 1'h0: Support clock only when application writes registers."]
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
#[doc = "Field `CLK_EN` writer - 1'h1: Force clock on for register. 1'h0: Support clock only when application writes registers."]
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
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u32 & 1) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Set this bit, then clear this bit to reset the internal ahb FSM."]
    #[inline(always)]
    pub fn ahbm_rst_inter(&self) -> AHBM_RST_INTER_R {
        AHBM_RST_INTER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit, then clear this bit to reset the external ahb FSM."]
    #[inline(always)]
    pub fn ahbm_rst_exter(&self) -> AHBM_RST_EXTER_R {
        AHBM_RST_EXTER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to disable priority arbitration function."]
    #[inline(always)]
    pub fn arb_pri_dis(&self) -> ARB_PRI_DIS_R {
        ARB_PRI_DIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - 1'h1: Force clock on for register. 1'h0: Support clock only when application writes registers."]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit, then clear this bit to reset the internal ahb FSM."]
    #[inline(always)]
    pub fn ahbm_rst_inter(&mut self) -> AHBM_RST_INTER_W {
        AHBM_RST_INTER_W { w: self }
    }
    #[doc = "Bit 1 - Set this bit, then clear this bit to reset the external ahb FSM."]
    #[inline(always)]
    pub fn ahbm_rst_exter(&mut self) -> AHBM_RST_EXTER_W {
        AHBM_RST_EXTER_W { w: self }
    }
    #[doc = "Bit 2 - Set this bit to disable priority arbitration function."]
    #[inline(always)]
    pub fn arb_pri_dis(&mut self) -> ARB_PRI_DIS_W {
        ARB_PRI_DIS_W { w: self }
    }
    #[doc = "Bit 4 - 1'h1: Force clock on for register. 1'h0: Support clock only when application writes registers."]
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
#[doc = "MISC register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc_conf](index.html) module"]
pub struct MISC_CONF_SPEC;
impl crate::RegisterSpec for MISC_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [misc_conf::R](R) reader structure"]
impl crate::Readable for MISC_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [misc_conf::W](W) writer structure"]
impl crate::Writable for MISC_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MISC_CONF to value 0"]
impl crate::Resettable for MISC_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
