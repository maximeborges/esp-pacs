#[doc = "Register `REGCLK` reader"]
pub struct R(crate::R<REGCLK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REGCLK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REGCLK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REGCLK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REGCLK` writer"]
pub struct W(crate::W<REGCLK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REGCLK_SPEC>;
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
impl From<crate::W<REGCLK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REGCLK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDT_CLK_IS_ACTIVE` reader - reg_wdt_clk_is_active."]
pub struct WDT_CLK_IS_ACTIVE_R(crate::FieldReader<bool>);
impl WDT_CLK_IS_ACTIVE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WDT_CLK_IS_ACTIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDT_CLK_IS_ACTIVE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDT_CLK_IS_ACTIVE` writer - reg_wdt_clk_is_active."]
pub struct WDT_CLK_IS_ACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_CLK_IS_ACTIVE_W<'a> {
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
#[doc = "Field `TIMER_CLK_IS_ACTIVE` reader - reg_timer_clk_is_active."]
pub struct TIMER_CLK_IS_ACTIVE_R(crate::FieldReader<bool>);
impl TIMER_CLK_IS_ACTIVE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER_CLK_IS_ACTIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER_CLK_IS_ACTIVE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER_CLK_IS_ACTIVE` writer - reg_timer_clk_is_active."]
pub struct TIMER_CLK_IS_ACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_CLK_IS_ACTIVE_W<'a> {
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
#[doc = "Field `CLK_EN` reader - reg_clk_en."]
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
#[doc = "Field `CLK_EN` writer - reg_clk_en."]
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
    #[doc = "Bit 29 - reg_wdt_clk_is_active."]
    #[inline(always)]
    pub fn wdt_clk_is_active(&self) -> WDT_CLK_IS_ACTIVE_R {
        WDT_CLK_IS_ACTIVE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - reg_timer_clk_is_active."]
    #[inline(always)]
    pub fn timer_clk_is_active(&self) -> TIMER_CLK_IS_ACTIVE_R {
        TIMER_CLK_IS_ACTIVE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - reg_clk_en."]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 29 - reg_wdt_clk_is_active."]
    #[inline(always)]
    pub fn wdt_clk_is_active(&mut self) -> WDT_CLK_IS_ACTIVE_W {
        WDT_CLK_IS_ACTIVE_W { w: self }
    }
    #[doc = "Bit 30 - reg_timer_clk_is_active."]
    #[inline(always)]
    pub fn timer_clk_is_active(&mut self) -> TIMER_CLK_IS_ACTIVE_W {
        TIMER_CLK_IS_ACTIVE_W { w: self }
    }
    #[doc = "Bit 31 - reg_clk_en."]
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
#[doc = "TIMG_REGCLK_REG.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [regclk](index.html) module"]
pub struct REGCLK_SPEC;
impl crate::RegisterSpec for REGCLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [regclk::R](R) reader structure"]
impl crate::Readable for REGCLK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [regclk::W](W) writer structure"]
impl crate::Writable for REGCLK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REGCLK to value 0x6000_0000"]
impl crate::Resettable for REGCLK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x6000_0000
    }
}
