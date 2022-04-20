#[doc = "Register `MEM_POWER_DOWN` reader"]
pub struct R(crate::R<MEM_POWER_DOWN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEM_POWER_DOWN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEM_POWER_DOWN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEM_POWER_DOWN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MEM_POWER_DOWN` writer"]
pub struct W(crate::W<MEM_POWER_DOWN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MEM_POWER_DOWN_SPEC>;
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
impl From<crate::W<MEM_POWER_DOWN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MEM_POWER_DOWN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ROM_POWER_DOWN` reader - ******* Description ***********"]
pub struct ROM_POWER_DOWN_R(crate::FieldReader<u8, u8>);
impl ROM_POWER_DOWN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ROM_POWER_DOWN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROM_POWER_DOWN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROM_POWER_DOWN` writer - ******* Description ***********"]
pub struct ROM_POWER_DOWN_W<'a> {
    w: &'a mut W,
}
impl<'a> ROM_POWER_DOWN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !7) | (value as u32 & 7);
        self.w
    }
}
#[doc = "Field `SRAM_POWER_DOWN` reader - ******* Description ***********"]
pub struct SRAM_POWER_DOWN_R(crate::FieldReader<u16, u16>);
impl SRAM_POWER_DOWN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        SRAM_POWER_DOWN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM_POWER_DOWN_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM_POWER_DOWN` writer - ******* Description ***********"]
pub struct SRAM_POWER_DOWN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_POWER_DOWN_W<'a> {
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
    pub fn rom_power_down(&self) -> ROM_POWER_DOWN_R {
        ROM_POWER_DOWN_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:13 - ******* Description ***********"]
    #[inline(always)]
    pub fn sram_power_down(&self) -> SRAM_POWER_DOWN_R {
        SRAM_POWER_DOWN_R::new(((self.bits >> 3) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2 - ******* Description ***********"]
    #[inline(always)]
    pub fn rom_power_down(&mut self) -> ROM_POWER_DOWN_W {
        ROM_POWER_DOWN_W { w: self }
    }
    #[doc = "Bits 3:13 - ******* Description ***********"]
    #[inline(always)]
    pub fn sram_power_down(&mut self) -> SRAM_POWER_DOWN_W {
        SRAM_POWER_DOWN_W { w: self }
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
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_power_down]
(index.html) module"]
pub struct MEM_POWER_DOWN_SPEC;
impl crate::RegisterSpec for MEM_POWER_DOWN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mem_power_down::R]
(R) reader structure"]
impl crate::Readable for MEM_POWER_DOWN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mem_power_down::W]
(W) writer structure"]
impl crate::Writable for MEM_POWER_DOWN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MEM_POWER_DOWN to value 0"]
impl crate::Resettable for MEM_POWER_DOWN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
