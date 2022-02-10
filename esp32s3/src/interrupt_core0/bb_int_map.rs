#[doc = "Register `BB_INT_MAP` reader"]
pub struct R(crate::R<BB_INT_MAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BB_INT_MAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BB_INT_MAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BB_INT_MAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BB_INT_MAP` writer"]
pub struct W(crate::W<BB_INT_MAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BB_INT_MAP_SPEC>;
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
impl From<crate::W<BB_INT_MAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BB_INT_MAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BB_INT_MAP` reader - this register used to map bb interrupt to one of core0's external interrupt"]
pub struct BB_INT_MAP_R(crate::FieldReader<u8, u8>);
impl BB_INT_MAP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BB_INT_MAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BB_INT_MAP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BB_INT_MAP` writer - this register used to map bb interrupt to one of core0's external interrupt"]
pub struct BB_INT_MAP_W<'a> {
    w: &'a mut W,
}
impl<'a> BB_INT_MAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - this register used to map bb interrupt to one of core0's external interrupt"]
    #[inline(always)]
    pub fn bb_int_map(&self) -> BB_INT_MAP_R {
        BB_INT_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - this register used to map bb interrupt to one of core0's external interrupt"]
    #[inline(always)]
    pub fn bb_int_map(&mut self) -> BB_INT_MAP_W {
        BB_INT_MAP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "bb interrupt configuration register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_int_map]
(index.html) module"]
pub struct BB_INT_MAP_SPEC;
impl crate::RegisterSpec for BB_INT_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bb_int_map::R]
(R) reader structure"]
impl crate::Readable for BB_INT_MAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bb_int_map::W]
(W) writer structure"]
impl crate::Writable for BB_INT_MAP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BB_INT_MAP to value 0x10"]
impl crate::Resettable for BB_INT_MAP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
