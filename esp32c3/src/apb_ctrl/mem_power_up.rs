#[doc = "Register `MEM_POWER_UP` reader"]
pub struct R(crate::R<MEM_POWER_UP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEM_POWER_UP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEM_POWER_UP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEM_POWER_UP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MEM_POWER_UP` writer"]
pub struct W(crate::W<MEM_POWER_UP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MEM_POWER_UP_SPEC>;
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
impl From<crate::W<MEM_POWER_UP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MEM_POWER_UP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ROM_POWER_UP` reader - reg_rom_power_up"]
pub struct ROM_POWER_UP_R(crate::FieldReader<u8, u8>);
impl ROM_POWER_UP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ROM_POWER_UP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROM_POWER_UP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROM_POWER_UP` writer - reg_rom_power_up"]
pub struct ROM_POWER_UP_W<'a> {
    w: &'a mut W,
}
impl<'a> ROM_POWER_UP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `SRAM_POWER_UP` reader - reg_sram_power_up"]
pub struct SRAM_POWER_UP_R(crate::FieldReader<u8, u8>);
impl SRAM_POWER_UP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SRAM_POWER_UP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM_POWER_UP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM_POWER_UP` writer - reg_sram_power_up"]
pub struct SRAM_POWER_UP_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_POWER_UP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 2)) | ((value as u32 & 0x0f) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - reg_rom_power_up"]
    #[inline(always)]
    pub fn rom_power_up(&self) -> ROM_POWER_UP_R {
        ROM_POWER_UP_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:5 - reg_sram_power_up"]
    #[inline(always)]
    pub fn sram_power_up(&self) -> SRAM_POWER_UP_R {
        SRAM_POWER_UP_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - reg_rom_power_up"]
    #[inline(always)]
    pub fn rom_power_up(&mut self) -> ROM_POWER_UP_W {
        ROM_POWER_UP_W { w: self }
    }
    #[doc = "Bits 2:5 - reg_sram_power_up"]
    #[inline(always)]
    pub fn sram_power_up(&mut self) -> SRAM_POWER_UP_W {
        SRAM_POWER_UP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB_CTRL_MEM_POWER_UP_REG\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_power_up]
(index.html) module"]
pub struct MEM_POWER_UP_SPEC;
impl crate::RegisterSpec for MEM_POWER_UP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mem_power_up::R]
(R) reader structure"]
impl crate::Readable for MEM_POWER_UP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mem_power_up::W]
(W) writer structure"]
impl crate::Writable for MEM_POWER_UP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MEM_POWER_UP to value 0x3f"]
impl crate::Resettable for MEM_POWER_UP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3f
    }
}
