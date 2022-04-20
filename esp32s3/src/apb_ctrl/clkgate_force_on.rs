#[doc = "Register `CLKGATE_FORCE_ON` reader"]
pub struct R(crate::R<CLKGATE_FORCE_ON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKGATE_FORCE_ON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKGATE_FORCE_ON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKGATE_FORCE_ON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKGATE_FORCE_ON` writer"]
pub struct W(crate::W<CLKGATE_FORCE_ON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKGATE_FORCE_ON_SPEC>;
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
impl From<crate::W<CLKGATE_FORCE_ON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKGATE_FORCE_ON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ROM_CLKGATE_FORCE_ON` reader - ******* Description ***********"]
pub struct ROM_CLKGATE_FORCE_ON_R(crate::FieldReader<u8, u8>);
impl ROM_CLKGATE_FORCE_ON_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ROM_CLKGATE_FORCE_ON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROM_CLKGATE_FORCE_ON_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROM_CLKGATE_FORCE_ON` writer - ******* Description ***********"]
pub struct ROM_CLKGATE_FORCE_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> ROM_CLKGATE_FORCE_ON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !7) | (value as u32 & 7);
        self.w
    }
}
#[doc = "Field `SRAM_CLKGATE_FORCE_ON` reader - ******* Description ***********"]
pub struct SRAM_CLKGATE_FORCE_ON_R(crate::FieldReader<u16, u16>);
impl SRAM_CLKGATE_FORCE_ON_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        SRAM_CLKGATE_FORCE_ON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM_CLKGATE_FORCE_ON_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM_CLKGATE_FORCE_ON` writer - ******* Description ***********"]
pub struct SRAM_CLKGATE_FORCE_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_CLKGATE_FORCE_ON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 3)) | ((value as u32 & 0x07ff) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - ******* Description ***********"]
    #[inline(always)]
    pub fn rom_clkgate_force_on(&self) -> ROM_CLKGATE_FORCE_ON_R {
        ROM_CLKGATE_FORCE_ON_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:13 - ******* Description ***********"]
    #[inline(always)]
    pub fn sram_clkgate_force_on(&self) -> SRAM_CLKGATE_FORCE_ON_R {
        SRAM_CLKGATE_FORCE_ON_R::new(((self.bits >> 3) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2 - ******* Description ***********"]
    #[inline(always)]
    pub fn rom_clkgate_force_on(&mut self) -> ROM_CLKGATE_FORCE_ON_W {
        ROM_CLKGATE_FORCE_ON_W { w: self }
    }
    #[doc = "Bits 3:13 - ******* Description ***********"]
    #[inline(always)]
    pub fn sram_clkgate_force_on(&mut self) -> SRAM_CLKGATE_FORCE_ON_W {
        SRAM_CLKGATE_FORCE_ON_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "******* Description ***********\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkgate_force_on]
(index.html) module"]
pub struct CLKGATE_FORCE_ON_SPEC;
impl crate::RegisterSpec for CLKGATE_FORCE_ON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkgate_force_on::R]
(R) reader structure"]
impl crate::Readable for CLKGATE_FORCE_ON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkgate_force_on::W]
(W) writer structure"]
impl crate::Writable for CLKGATE_FORCE_ON_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLKGATE_FORCE_ON to value 0x3fff"]
impl crate::Resettable for CLKGATE_FORCE_ON_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3fff
    }
}
