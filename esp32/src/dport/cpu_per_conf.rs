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
#[doc = "Field `CPUPERIOD_SEL` reader - "]
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
#[doc = "Field `CPUPERIOD_SEL` writer - "]
pub struct CPUPERIOD_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CPUPERIOD_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `LOWSPEED_CLK_SEL` reader - "]
pub struct LOWSPEED_CLK_SEL_R(crate::FieldReader<bool, bool>);
impl LOWSPEED_CLK_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOWSPEED_CLK_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOWSPEED_CLK_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOWSPEED_CLK_SEL` writer - "]
pub struct LOWSPEED_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LOWSPEED_CLK_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `FAST_CLK_RTC_SEL` reader - "]
pub struct FAST_CLK_RTC_SEL_R(crate::FieldReader<bool, bool>);
impl FAST_CLK_RTC_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FAST_CLK_RTC_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FAST_CLK_RTC_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAST_CLK_RTC_SEL` writer - "]
pub struct FAST_CLK_RTC_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FAST_CLK_RTC_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn cpuperiod_sel(&self) -> CPUPERIOD_SEL_R {
        CPUPERIOD_SEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn lowspeed_clk_sel(&self) -> LOWSPEED_CLK_SEL_R {
        LOWSPEED_CLK_SEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn fast_clk_rtc_sel(&self) -> FAST_CLK_RTC_SEL_R {
        FAST_CLK_RTC_SEL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn cpuperiod_sel(&mut self) -> CPUPERIOD_SEL_W {
        CPUPERIOD_SEL_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn lowspeed_clk_sel(&mut self) -> LOWSPEED_CLK_SEL_W {
        LOWSPEED_CLK_SEL_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn fast_clk_rtc_sel(&mut self) -> FAST_CLK_RTC_SEL_W {
        FAST_CLK_RTC_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`]
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
#[doc = "`reset()` method sets CPU_PER_CONF to value 0"]
impl crate::Resettable for CPU_PER_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
