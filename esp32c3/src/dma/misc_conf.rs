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
#[doc = "Field `CLK_EN` reader - reg_clk_en"]
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
#[doc = "Field `CLK_EN` writer - reg_clk_en"]
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
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u32 & 1) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Set this bit, then clear this bit to reset the internal ahb FSM."]
    #[inline(always)]
    pub fn ahbm_rst_inter(&self) -> AHBM_RST_INTER_R {
        AHBM_RST_INTER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to disable priority arbitration function."]
    #[inline(always)]
    pub fn arb_pri_dis(&self) -> ARB_PRI_DIS_R {
        ARB_PRI_DIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reg_clk_en"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit, then clear this bit to reset the internal ahb FSM."]
    #[inline(always)]
    pub fn ahbm_rst_inter(&mut self) -> AHBM_RST_INTER_W {
        AHBM_RST_INTER_W { w: self }
    }
    #[doc = "Bit 2 - Set this bit to disable priority arbitration function."]
    #[inline(always)]
    pub fn arb_pri_dis(&mut self) -> ARB_PRI_DIS_W {
        ARB_PRI_DIS_W { w: self }
    }
    #[doc = "Bit 3 - reg_clk_en"]
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
#[doc = "DMA_MISC_CONF_REG.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc_conf](index.html) module"]
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
