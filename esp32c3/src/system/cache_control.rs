#[doc = "Register `CACHE_CONTROL` reader"]
pub struct R(crate::R<CACHE_CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACHE_CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACHE_CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACHE_CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CACHE_CONTROL` writer"]
pub struct W(crate::W<CACHE_CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CACHE_CONTROL_SPEC>;
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
impl From<crate::W<CACHE_CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CACHE_CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ICACHE_CLK_ON` reader - reg_icache_clk_on"]
pub struct ICACHE_CLK_ON_R(crate::FieldReader<bool, bool>);
impl ICACHE_CLK_ON_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ICACHE_CLK_ON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICACHE_CLK_ON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICACHE_CLK_ON` writer - reg_icache_clk_on"]
pub struct ICACHE_CLK_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> ICACHE_CLK_ON_W<'a> {
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
#[doc = "Field `ICACHE_RESET` reader - reg_icache_reset"]
pub struct ICACHE_RESET_R(crate::FieldReader<bool, bool>);
impl ICACHE_RESET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ICACHE_RESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICACHE_RESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICACHE_RESET` writer - reg_icache_reset"]
pub struct ICACHE_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> ICACHE_RESET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `DCACHE_CLK_ON` reader - reg_dcache_clk_on"]
pub struct DCACHE_CLK_ON_R(crate::FieldReader<bool, bool>);
impl DCACHE_CLK_ON_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCACHE_CLK_ON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCACHE_CLK_ON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCACHE_CLK_ON` writer - reg_dcache_clk_on"]
pub struct DCACHE_CLK_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> DCACHE_CLK_ON_W<'a> {
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
#[doc = "Field `DCACHE_RESET` reader - reg_dcache_reset"]
pub struct DCACHE_RESET_R(crate::FieldReader<bool, bool>);
impl DCACHE_RESET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCACHE_RESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCACHE_RESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCACHE_RESET` writer - reg_dcache_reset"]
pub struct DCACHE_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> DCACHE_RESET_W<'a> {
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
    #[doc = "Bit 0 - reg_icache_clk_on"]
    #[inline(always)]
    pub fn icache_clk_on(&self) -> ICACHE_CLK_ON_R {
        ICACHE_CLK_ON_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - reg_icache_reset"]
    #[inline(always)]
    pub fn icache_reset(&self) -> ICACHE_RESET_R {
        ICACHE_RESET_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - reg_dcache_clk_on"]
    #[inline(always)]
    pub fn dcache_clk_on(&self) -> DCACHE_CLK_ON_R {
        DCACHE_CLK_ON_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - reg_dcache_reset"]
    #[inline(always)]
    pub fn dcache_reset(&self) -> DCACHE_RESET_R {
        DCACHE_RESET_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - reg_icache_clk_on"]
    #[inline(always)]
    pub fn icache_clk_on(&mut self) -> ICACHE_CLK_ON_W {
        ICACHE_CLK_ON_W { w: self }
    }
    #[doc = "Bit 1 - reg_icache_reset"]
    #[inline(always)]
    pub fn icache_reset(&mut self) -> ICACHE_RESET_W {
        ICACHE_RESET_W { w: self }
    }
    #[doc = "Bit 2 - reg_dcache_clk_on"]
    #[inline(always)]
    pub fn dcache_clk_on(&mut self) -> DCACHE_CLK_ON_W {
        DCACHE_CLK_ON_W { w: self }
    }
    #[doc = "Bit 3 - reg_dcache_reset"]
    #[inline(always)]
    pub fn dcache_reset(&mut self) -> DCACHE_RESET_W {
        DCACHE_RESET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "cache control register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cache_control]
(index.html) module"]
pub struct CACHE_CONTROL_SPEC;
impl crate::RegisterSpec for CACHE_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cache_control::R]
(R) reader structure"]
impl crate::Readable for CACHE_CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cache_control::W]
(W) writer structure"]
impl crate::Writable for CACHE_CONTROL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CACHE_CONTROL to value 0x05"]
impl crate::Resettable for CACHE_CONTROL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x05
    }
}
