#[doc = "Register `CLOCK_GATE` reader"]
pub struct R(crate::R<CLOCK_GATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLOCK_GATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLOCK_GATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLOCK_GATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLOCK_GATE` writer"]
pub struct W(crate::W<CLOCK_GATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLOCK_GATE_SPEC>;
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
impl From<crate::W<CLOCK_GATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLOCK_GATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REG_CLK_EN` reader - this register uesd to control clock-gating interupt martrix"]
pub struct REG_CLK_EN_R(crate::FieldReader<bool>);
impl REG_CLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REG_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_CLK_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REG_CLK_EN` writer - this register uesd to control clock-gating interupt martrix"]
pub struct REG_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_CLK_EN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - this register uesd to control clock-gating interupt martrix"]
    #[inline(always)]
    pub fn reg_clk_en(&self) -> REG_CLK_EN_R {
        REG_CLK_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - this register uesd to control clock-gating interupt martrix"]
    #[inline(always)]
    pub fn reg_clk_en(&mut self) -> REG_CLK_EN_W {
        REG_CLK_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "clock gate register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clock_gate](index.html) module"]
pub struct CLOCK_GATE_SPEC;
impl crate::RegisterSpec for CLOCK_GATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clock_gate::R](R) reader structure"]
impl crate::Readable for CLOCK_GATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clock_gate::W](W) writer structure"]
impl crate::Writable for CLOCK_GATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLOCK_GATE to value 0x01"]
impl crate::Resettable for CLOCK_GATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
