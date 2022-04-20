#[doc = "Register `CPU_PER_CONF` reader"]
pub struct R(crate::R<CPU_PER_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU_PER_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPU_PER_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPU_PER_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPU_PER_CONF` writer"]
pub struct W(crate::W<CPU_PER_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPU_PER_CONF_SPEC>;
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
impl From<crate::W<CPU_PER_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPU_PER_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPUPERIOD_SEL` reader - This field used to sel cpu clock frequent."]
pub struct CPUPERIOD_SEL_R(crate::FieldReader<u8, u8>);
impl CPUPERIOD_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CPUPERIOD_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPUPERIOD_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPUPERIOD_SEL` writer - This field used to sel cpu clock frequent."]
pub struct CPUPERIOD_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CPUPERIOD_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u32 & 3);
        self.w
    }
}
#[doc = "Field `PLL_FREQ_SEL` reader - This field used to sel pll frequent."]
pub struct PLL_FREQ_SEL_R(crate::FieldReader<bool, bool>);
impl PLL_FREQ_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PLL_FREQ_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL_FREQ_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL_FREQ_SEL` writer - This field used to sel pll frequent."]
pub struct PLL_FREQ_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_FREQ_SEL_W<'a> {
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
#[doc = "Field `CPU_WAIT_MODE_FORCE_ON` reader - Set 1 to force cpu_waiti_clk enable."]
pub struct CPU_WAIT_MODE_FORCE_ON_R(crate::FieldReader<bool, bool>);
impl CPU_WAIT_MODE_FORCE_ON_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CPU_WAIT_MODE_FORCE_ON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPU_WAIT_MODE_FORCE_ON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPU_WAIT_MODE_FORCE_ON` writer - Set 1 to force cpu_waiti_clk enable."]
pub struct CPU_WAIT_MODE_FORCE_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_WAIT_MODE_FORCE_ON_W<'a> {
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
#[doc = "Field `CPU_WAITI_DELAY_NUM` reader - This field used to set delay cycle when cpu enter waiti mode, after delay waiti_clk will close"]
pub struct CPU_WAITI_DELAY_NUM_R(crate::FieldReader<u8, u8>);
impl CPU_WAITI_DELAY_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CPU_WAITI_DELAY_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPU_WAITI_DELAY_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPU_WAITI_DELAY_NUM` writer - This field used to set delay cycle when cpu enter waiti mode, after delay waiti_clk will close"]
pub struct CPU_WAITI_DELAY_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_WAITI_DELAY_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - This field used to sel cpu clock frequent."]
    #[inline(always)]
    pub fn cpuperiod_sel(&self) -> CPUPERIOD_SEL_R {
        CPUPERIOD_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - This field used to sel pll frequent."]
    #[inline(always)]
    pub fn pll_freq_sel(&self) -> PLL_FREQ_SEL_R {
        PLL_FREQ_SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set 1 to force cpu_waiti_clk enable."]
    #[inline(always)]
    pub fn cpu_wait_mode_force_on(&self) -> CPU_WAIT_MODE_FORCE_ON_R {
        CPU_WAIT_MODE_FORCE_ON_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - This field used to set delay cycle when cpu enter waiti mode, after delay waiti_clk will close"]
    #[inline(always)]
    pub fn cpu_waiti_delay_num(&self) -> CPU_WAITI_DELAY_NUM_R {
        CPU_WAITI_DELAY_NUM_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - This field used to sel cpu clock frequent."]
    #[inline(always)]
    pub fn cpuperiod_sel(&mut self) -> CPUPERIOD_SEL_W {
        CPUPERIOD_SEL_W { w: self }
    }
    #[doc = "Bit 2 - This field used to sel pll frequent."]
    #[inline(always)]
    pub fn pll_freq_sel(&mut self) -> PLL_FREQ_SEL_W {
        PLL_FREQ_SEL_W { w: self }
    }
    #[doc = "Bit 3 - Set 1 to force cpu_waiti_clk enable."]
    #[inline(always)]
    pub fn cpu_wait_mode_force_on(&mut self) -> CPU_WAIT_MODE_FORCE_ON_W {
        CPU_WAIT_MODE_FORCE_ON_W { w: self }
    }
    #[doc = "Bits 4:7 - This field used to set delay cycle when cpu enter waiti mode, after delay waiti_clk will close"]
    #[inline(always)]
    pub fn cpu_waiti_delay_num(&mut self) -> CPU_WAITI_DELAY_NUM_W {
        CPU_WAITI_DELAY_NUM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "cpu peripheral clock configuration register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_per_conf]
(index.html) module"]
pub struct CPU_PER_CONF_SPEC;
impl crate::RegisterSpec for CPU_PER_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu_per_conf::R]
(R) reader structure"]
impl crate::Readable for CPU_PER_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpu_per_conf::W]
(W) writer structure"]
impl crate::Writable for CPU_PER_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CPU_PER_CONF to value 0x0c"]
impl crate::Resettable for CPU_PER_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0c
    }
}
