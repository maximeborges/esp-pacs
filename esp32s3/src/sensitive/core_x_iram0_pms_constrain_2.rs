#[doc = "Register `CORE_X_IRAM0_PMS_CONSTRAIN_2` reader"]
pub struct R(crate::R<CORE_X_IRAM0_PMS_CONSTRAIN_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_X_IRAM0_PMS_CONSTRAIN_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_X_IRAM0_PMS_CONSTRAIN_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_X_IRAM0_PMS_CONSTRAIN_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_X_IRAM0_PMS_CONSTRAIN_2` writer"]
pub struct W(crate::W<CORE_X_IRAM0_PMS_CONSTRAIN_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_X_IRAM0_PMS_CONSTRAIN_2_SPEC>;
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
impl From<crate::W<CORE_X_IRAM0_PMS_CONSTRAIN_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_X_IRAM0_PMS_CONSTRAIN_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0` reader - core0/core1's permission of instruction region0 of SRAM in world1"]
pub struct CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0_R(crate::FieldReader<u8>);
impl CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0` writer - core0/core1's permission of instruction region0 of SRAM in world1"]
pub struct CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !7) | (value as u32 & 7);
        self.w
    }
}
#[doc = "Field `CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1` reader - core0/core1's permission of instruction region1 of SRAM in world1"]
pub struct CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1_R(crate::FieldReader<u8>);
impl CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1` writer - core0/core1's permission of instruction region1 of SRAM in world1"]
pub struct CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 3)) | ((value as u32 & 7) << 3);
        self.w
    }
}
#[doc = "Field `CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2` reader - core0/core1's permission of instruction region2 of SRAM in world1"]
pub struct CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2_R(crate::FieldReader<u8>);
impl CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2` writer - core0/core1's permission of instruction region2 of SRAM in world1"]
pub struct CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 6)) | ((value as u32 & 7) << 6);
        self.w
    }
}
#[doc = "Field `CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3` reader - core0/core1's permission of instruction region3 of SRAM in world1"]
pub struct CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3_R(crate::FieldReader<u8>);
impl CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3` writer - core0/core1's permission of instruction region3 of SRAM in world1"]
pub struct CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 9)) | ((value as u32 & 7) << 9);
        self.w
    }
}
#[doc = "Field `CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_CACHEDATAARRAY_PMS_0` reader - core0/core1's permission of icache data sram block0 in world1"]
pub struct CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_CACHEDATAARRAY_PMS_0_R(crate::FieldReader<u8>);
impl CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_CACHEDATAARRAY_PMS_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_CACHEDATAARRAY_PMS_0_R(crate::FieldReader::new(
            bits,
        ))
    }
}
impl core::ops::Deref for CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_CACHEDATAARRAY_PMS_0_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_CACHEDATAARRAY_PMS_0` writer - core0/core1's permission of icache data sram block0 in world1"]
pub struct CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_CACHEDATAARRAY_PMS_0_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_CACHEDATAARRAY_PMS_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 12)) | ((value as u32 & 7) << 12);
        self.w
    }
}
#[doc = "Field `CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_CACHEDATAARRAY_PMS_1` reader - core0/core1's permission of icache data sram block1 in world1"]
pub struct CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_CACHEDATAARRAY_PMS_1_R(crate::FieldReader<u8>);
impl CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_CACHEDATAARRAY_PMS_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_CACHEDATAARRAY_PMS_1_R(crate::FieldReader::new(
            bits,
        ))
    }
}
impl core::ops::Deref for CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_CACHEDATAARRAY_PMS_1_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_CACHEDATAARRAY_PMS_1` writer - core0/core1's permission of icache data sram block1 in world1"]
pub struct CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_CACHEDATAARRAY_PMS_1_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_CACHEDATAARRAY_PMS_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 15)) | ((value as u32 & 7) << 15);
        self.w
    }
}
#[doc = "Field `CORE_X_IRAM0_PMS_CONSTRAIN_ROM_WORLD_0_PMS` reader - core0/core1's permission of rom in world1"]
pub struct CORE_X_IRAM0_PMS_CONSTRAIN_ROM_WORLD_0_PMS_R(crate::FieldReader<u8>);
impl CORE_X_IRAM0_PMS_CONSTRAIN_ROM_WORLD_0_PMS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_X_IRAM0_PMS_CONSTRAIN_ROM_WORLD_0_PMS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_X_IRAM0_PMS_CONSTRAIN_ROM_WORLD_0_PMS_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_X_IRAM0_PMS_CONSTRAIN_ROM_WORLD_0_PMS` writer - core0/core1's permission of rom in world1"]
pub struct CORE_X_IRAM0_PMS_CONSTRAIN_ROM_WORLD_0_PMS_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_X_IRAM0_PMS_CONSTRAIN_ROM_WORLD_0_PMS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 18)) | ((value as u32 & 7) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - core0/core1's permission of instruction region0 of SRAM in world1"]
    #[inline(always)]
    pub fn core_x_iram0_pms_constrain_sram_world_0_pms_0(
        &self,
    ) -> CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0_R {
        CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - core0/core1's permission of instruction region1 of SRAM in world1"]
    #[inline(always)]
    pub fn core_x_iram0_pms_constrain_sram_world_0_pms_1(
        &self,
    ) -> CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1_R {
        CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - core0/core1's permission of instruction region2 of SRAM in world1"]
    #[inline(always)]
    pub fn core_x_iram0_pms_constrain_sram_world_0_pms_2(
        &self,
    ) -> CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2_R {
        CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - core0/core1's permission of instruction region3 of SRAM in world1"]
    #[inline(always)]
    pub fn core_x_iram0_pms_constrain_sram_world_0_pms_3(
        &self,
    ) -> CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3_R {
        CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - core0/core1's permission of icache data sram block0 in world1"]
    #[inline(always)]
    pub fn core_x_iram0_pms_constrain_sram_world_0_cachedataarray_pms_0(
        &self,
    ) -> CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_CACHEDATAARRAY_PMS_0_R {
        CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_CACHEDATAARRAY_PMS_0_R::new(
            ((self.bits >> 12) & 7) as u8,
        )
    }
    #[doc = "Bits 15:17 - core0/core1's permission of icache data sram block1 in world1"]
    #[inline(always)]
    pub fn core_x_iram0_pms_constrain_sram_world_0_cachedataarray_pms_1(
        &self,
    ) -> CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_CACHEDATAARRAY_PMS_1_R {
        CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_CACHEDATAARRAY_PMS_1_R::new(
            ((self.bits >> 15) & 7) as u8,
        )
    }
    #[doc = "Bits 18:20 - core0/core1's permission of rom in world1"]
    #[inline(always)]
    pub fn core_x_iram0_pms_constrain_rom_world_0_pms(
        &self,
    ) -> CORE_X_IRAM0_PMS_CONSTRAIN_ROM_WORLD_0_PMS_R {
        CORE_X_IRAM0_PMS_CONSTRAIN_ROM_WORLD_0_PMS_R::new(((self.bits >> 18) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - core0/core1's permission of instruction region0 of SRAM in world1"]
    #[inline(always)]
    pub fn core_x_iram0_pms_constrain_sram_world_0_pms_0(
        &mut self,
    ) -> CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0_W {
        CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0_W { w: self }
    }
    #[doc = "Bits 3:5 - core0/core1's permission of instruction region1 of SRAM in world1"]
    #[inline(always)]
    pub fn core_x_iram0_pms_constrain_sram_world_0_pms_1(
        &mut self,
    ) -> CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1_W {
        CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1_W { w: self }
    }
    #[doc = "Bits 6:8 - core0/core1's permission of instruction region2 of SRAM in world1"]
    #[inline(always)]
    pub fn core_x_iram0_pms_constrain_sram_world_0_pms_2(
        &mut self,
    ) -> CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2_W {
        CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2_W { w: self }
    }
    #[doc = "Bits 9:11 - core0/core1's permission of instruction region3 of SRAM in world1"]
    #[inline(always)]
    pub fn core_x_iram0_pms_constrain_sram_world_0_pms_3(
        &mut self,
    ) -> CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3_W {
        CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3_W { w: self }
    }
    #[doc = "Bits 12:14 - core0/core1's permission of icache data sram block0 in world1"]
    #[inline(always)]
    pub fn core_x_iram0_pms_constrain_sram_world_0_cachedataarray_pms_0(
        &mut self,
    ) -> CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_CACHEDATAARRAY_PMS_0_W {
        CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_CACHEDATAARRAY_PMS_0_W { w: self }
    }
    #[doc = "Bits 15:17 - core0/core1's permission of icache data sram block1 in world1"]
    #[inline(always)]
    pub fn core_x_iram0_pms_constrain_sram_world_0_cachedataarray_pms_1(
        &mut self,
    ) -> CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_CACHEDATAARRAY_PMS_1_W {
        CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_CACHEDATAARRAY_PMS_1_W { w: self }
    }
    #[doc = "Bits 18:20 - core0/core1's permission of rom in world1"]
    #[inline(always)]
    pub fn core_x_iram0_pms_constrain_rom_world_0_pms(
        &mut self,
    ) -> CORE_X_IRAM0_PMS_CONSTRAIN_ROM_WORLD_0_PMS_W {
        CORE_X_IRAM0_PMS_CONSTRAIN_ROM_WORLD_0_PMS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "corex iram0 permission configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_x_iram0_pms_constrain_2](index.html) module"]
pub struct CORE_X_IRAM0_PMS_CONSTRAIN_2_SPEC;
impl crate::RegisterSpec for CORE_X_IRAM0_PMS_CONSTRAIN_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_x_iram0_pms_constrain_2::R](R) reader structure"]
impl crate::Readable for CORE_X_IRAM0_PMS_CONSTRAIN_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_x_iram0_pms_constrain_2::W](W) writer structure"]
impl crate::Writable for CORE_X_IRAM0_PMS_CONSTRAIN_2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CORE_X_IRAM0_PMS_CONSTRAIN_2 to value 0x001f_ffff"]
impl crate::Resettable for CORE_X_IRAM0_PMS_CONSTRAIN_2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x001f_ffff
    }
}
