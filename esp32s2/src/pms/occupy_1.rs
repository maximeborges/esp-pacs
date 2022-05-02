#[doc = "Register `OCCUPY_1` reader"]
pub struct R(crate::R<OCCUPY_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OCCUPY_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OCCUPY_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OCCUPY_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OCCUPY_1` writer"]
pub struct W(crate::W<OCCUPY_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OCCUPY_1_SPEC>;
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
impl From<crate::W<OCCUPY_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OCCUPY_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OCCUPY_CACHE` reader - Configure whether SRAM Block 0-3 is used as cache memory."]
pub struct OCCUPY_CACHE_R(crate::FieldReader<u8>);
impl OCCUPY_CACHE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OCCUPY_CACHE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OCCUPY_CACHE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OCCUPY_CACHE` writer - Configure whether SRAM Block 0-3 is used as cache memory."]
pub struct OCCUPY_CACHE_W<'a> {
    w: &'a mut W,
}
impl<'a> OCCUPY_CACHE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Configure whether SRAM Block 0-3 is used as cache memory."]
    #[inline(always)]
    pub fn occupy_cache(&self) -> OCCUPY_CACHE_R {
        OCCUPY_CACHE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Configure whether SRAM Block 0-3 is used as cache memory."]
    #[inline(always)]
    pub fn occupy_cache(&mut self) -> OCCUPY_CACHE_W {
        OCCUPY_CACHE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Occupy permission control register 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [occupy_1](index.html) module"]
pub struct OCCUPY_1_SPEC;
impl crate::RegisterSpec for OCCUPY_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [occupy_1::R](R) reader structure"]
impl crate::Readable for OCCUPY_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [occupy_1::W](W) writer structure"]
impl crate::Writable for OCCUPY_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OCCUPY_1 to value 0"]
impl crate::Resettable for OCCUPY_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
