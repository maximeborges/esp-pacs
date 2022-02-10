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
#[doc = "Field `PRO_ICACHE_CLK_ON` reader - Set this bit to enable clock of i-cache."]
pub struct PRO_ICACHE_CLK_ON_R(crate::FieldReader<bool, bool>);
impl PRO_ICACHE_CLK_ON_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_ICACHE_CLK_ON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_ICACHE_CLK_ON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_ICACHE_CLK_ON` writer - Set this bit to enable clock of i-cache."]
pub struct PRO_ICACHE_CLK_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_ICACHE_CLK_ON_W<'a> {
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
#[doc = "Field `PRO_DCACHE_CLK_ON` reader - Set this bit to enable clock of d-cache."]
pub struct PRO_DCACHE_CLK_ON_R(crate::FieldReader<bool, bool>);
impl PRO_DCACHE_CLK_ON_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_DCACHE_CLK_ON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_DCACHE_CLK_ON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_DCACHE_CLK_ON` writer - Set this bit to enable clock of d-cache."]
pub struct PRO_DCACHE_CLK_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_DCACHE_CLK_ON_W<'a> {
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
#[doc = "Field `PRO_CACHE_RESET` reader - Set this bit to reset cache."]
pub struct PRO_CACHE_RESET_R(crate::FieldReader<bool, bool>);
impl PRO_CACHE_RESET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_CACHE_RESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_CACHE_RESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_CACHE_RESET` writer - Set this bit to reset cache."]
pub struct PRO_CACHE_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_CACHE_RESET_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Set this bit to enable clock of i-cache."]
    #[inline(always)]
    pub fn pro_icache_clk_on(&self) -> PRO_ICACHE_CLK_ON_R {
        PRO_ICACHE_CLK_ON_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Set this bit to enable clock of d-cache."]
    #[inline(always)]
    pub fn pro_dcache_clk_on(&self) -> PRO_DCACHE_CLK_ON_R {
        PRO_DCACHE_CLK_ON_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Set this bit to reset cache."]
    #[inline(always)]
    pub fn pro_cache_reset(&self) -> PRO_CACHE_RESET_R {
        PRO_CACHE_RESET_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to enable clock of i-cache."]
    #[inline(always)]
    pub fn pro_icache_clk_on(&mut self) -> PRO_ICACHE_CLK_ON_W {
        PRO_ICACHE_CLK_ON_W { w: self }
    }
    #[doc = "Bit 1 - Set this bit to enable clock of d-cache."]
    #[inline(always)]
    pub fn pro_dcache_clk_on(&mut self) -> PRO_DCACHE_CLK_ON_W {
        PRO_DCACHE_CLK_ON_W { w: self }
    }
    #[doc = "Bit 2 - Set this bit to reset cache."]
    #[inline(always)]
    pub fn pro_cache_reset(&mut self) -> PRO_CACHE_RESET_W {
        PRO_CACHE_RESET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cache control register\n\nThis register you can [`read`]
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
#[doc = "`reset()` method sets CACHE_CONTROL to value 0x03"]
impl crate::Resettable for CACHE_CONTROL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03
    }
}
