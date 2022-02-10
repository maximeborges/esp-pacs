#[doc = "Register `CORE_X_DRAM0_PMS_CONSTRAIN_1` reader"]
pub struct R(crate::R<CORE_X_DRAM0_PMS_CONSTRAIN_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_X_DRAM0_PMS_CONSTRAIN_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_X_DRAM0_PMS_CONSTRAIN_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_X_DRAM0_PMS_CONSTRAIN_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_X_DRAM0_PMS_CONSTRAIN_1` writer"]
pub struct W(crate::W<CORE_X_DRAM0_PMS_CONSTRAIN_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_X_DRAM0_PMS_CONSTRAIN_1_SPEC>;
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
impl From<crate::W<CORE_X_DRAM0_PMS_CONSTRAIN_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_X_DRAM0_PMS_CONSTRAIN_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0` reader - core_x_dram0_pms_constrain_sram_world_0_pms_0"]
pub struct CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0_R(crate::FieldReader<u8, u8>);
impl CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0` writer - core_x_dram0_pms_constrain_sram_world_0_pms_0"]
pub struct CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1` reader - core_x_dram0_pms_constrain_sram_world_0_pms_1"]
pub struct CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1_R(crate::FieldReader<u8, u8>);
impl CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1` writer - core_x_dram0_pms_constrain_sram_world_0_pms_1"]
pub struct CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2` reader - core_x_dram0_pms_constrain_sram_world_0_pms_2"]
pub struct CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2_R(crate::FieldReader<u8, u8>);
impl CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2` writer - core_x_dram0_pms_constrain_sram_world_0_pms_2"]
pub struct CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3` reader - core_x_dram0_pms_constrain_sram_world_0_pms_3"]
pub struct CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3_R(crate::FieldReader<u8, u8>);
impl CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3` writer - core_x_dram0_pms_constrain_sram_world_0_pms_3"]
pub struct CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_0` reader - core_x_dram0_pms_constrain_sram_world_1_pms_0"]
pub struct CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_0_R(crate::FieldReader<u8, u8>);
impl CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_0` writer - core_x_dram0_pms_constrain_sram_world_1_pms_0"]
pub struct CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_0_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_1` reader - core_x_dram0_pms_constrain_sram_world_1_pms_1"]
pub struct CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_1_R(crate::FieldReader<u8, u8>);
impl CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_1` writer - core_x_dram0_pms_constrain_sram_world_1_pms_1"]
pub struct CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_1_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Field `CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_2` reader - core_x_dram0_pms_constrain_sram_world_1_pms_2"]
pub struct CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_2_R(crate::FieldReader<u8, u8>);
impl CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_2` writer - core_x_dram0_pms_constrain_sram_world_1_pms_2"]
pub struct CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_2_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_3` reader - core_x_dram0_pms_constrain_sram_world_1_pms_3"]
pub struct CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_3_R(crate::FieldReader<u8, u8>);
impl CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_3` writer - core_x_dram0_pms_constrain_sram_world_1_pms_3"]
pub struct CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_3_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "Field `CORE_X_DRAM0_PMS_CONSTRAIN_ROM_WORLD_0_PMS` reader - core_x_dram0_pms_constrain_rom_world_0_pms"]
pub struct CORE_X_DRAM0_PMS_CONSTRAIN_ROM_WORLD_0_PMS_R(crate::FieldReader<u8, u8>);
impl CORE_X_DRAM0_PMS_CONSTRAIN_ROM_WORLD_0_PMS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_X_DRAM0_PMS_CONSTRAIN_ROM_WORLD_0_PMS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_X_DRAM0_PMS_CONSTRAIN_ROM_WORLD_0_PMS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_X_DRAM0_PMS_CONSTRAIN_ROM_WORLD_0_PMS` writer - core_x_dram0_pms_constrain_rom_world_0_pms"]
pub struct CORE_X_DRAM0_PMS_CONSTRAIN_ROM_WORLD_0_PMS_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_X_DRAM0_PMS_CONSTRAIN_ROM_WORLD_0_PMS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `CORE_X_DRAM0_PMS_CONSTRAIN_ROM_WORLD_1_PMS` reader - core_x_dram0_pms_constrain_rom_world_1_pms"]
pub struct CORE_X_DRAM0_PMS_CONSTRAIN_ROM_WORLD_1_PMS_R(crate::FieldReader<u8, u8>);
impl CORE_X_DRAM0_PMS_CONSTRAIN_ROM_WORLD_1_PMS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_X_DRAM0_PMS_CONSTRAIN_ROM_WORLD_1_PMS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_X_DRAM0_PMS_CONSTRAIN_ROM_WORLD_1_PMS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_X_DRAM0_PMS_CONSTRAIN_ROM_WORLD_1_PMS` writer - core_x_dram0_pms_constrain_rom_world_1_pms"]
pub struct CORE_X_DRAM0_PMS_CONSTRAIN_ROM_WORLD_1_PMS_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_X_DRAM0_PMS_CONSTRAIN_ROM_WORLD_1_PMS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | ((value as u32 & 0x03) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - core_x_dram0_pms_constrain_sram_world_0_pms_0"]
    #[inline(always)]
    pub fn core_x_dram0_pms_constrain_sram_world_0_pms_0(
        &self,
    ) -> CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0_R {
        CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - core_x_dram0_pms_constrain_sram_world_0_pms_1"]
    #[inline(always)]
    pub fn core_x_dram0_pms_constrain_sram_world_0_pms_1(
        &self,
    ) -> CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1_R {
        CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - core_x_dram0_pms_constrain_sram_world_0_pms_2"]
    #[inline(always)]
    pub fn core_x_dram0_pms_constrain_sram_world_0_pms_2(
        &self,
    ) -> CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2_R {
        CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - core_x_dram0_pms_constrain_sram_world_0_pms_3"]
    #[inline(always)]
    pub fn core_x_dram0_pms_constrain_sram_world_0_pms_3(
        &self,
    ) -> CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3_R {
        CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - core_x_dram0_pms_constrain_sram_world_1_pms_0"]
    #[inline(always)]
    pub fn core_x_dram0_pms_constrain_sram_world_1_pms_0(
        &self,
    ) -> CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_0_R {
        CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_0_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - core_x_dram0_pms_constrain_sram_world_1_pms_1"]
    #[inline(always)]
    pub fn core_x_dram0_pms_constrain_sram_world_1_pms_1(
        &self,
    ) -> CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_1_R {
        CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_1_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - core_x_dram0_pms_constrain_sram_world_1_pms_2"]
    #[inline(always)]
    pub fn core_x_dram0_pms_constrain_sram_world_1_pms_2(
        &self,
    ) -> CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_2_R {
        CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_2_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - core_x_dram0_pms_constrain_sram_world_1_pms_3"]
    #[inline(always)]
    pub fn core_x_dram0_pms_constrain_sram_world_1_pms_3(
        &self,
    ) -> CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_3_R {
        CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_3_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - core_x_dram0_pms_constrain_rom_world_0_pms"]
    #[inline(always)]
    pub fn core_x_dram0_pms_constrain_rom_world_0_pms(
        &self,
    ) -> CORE_X_DRAM0_PMS_CONSTRAIN_ROM_WORLD_0_PMS_R {
        CORE_X_DRAM0_PMS_CONSTRAIN_ROM_WORLD_0_PMS_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - core_x_dram0_pms_constrain_rom_world_1_pms"]
    #[inline(always)]
    pub fn core_x_dram0_pms_constrain_rom_world_1_pms(
        &self,
    ) -> CORE_X_DRAM0_PMS_CONSTRAIN_ROM_WORLD_1_PMS_R {
        CORE_X_DRAM0_PMS_CONSTRAIN_ROM_WORLD_1_PMS_R::new(((self.bits >> 26) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - core_x_dram0_pms_constrain_sram_world_0_pms_0"]
    #[inline(always)]
    pub fn core_x_dram0_pms_constrain_sram_world_0_pms_0(
        &mut self,
    ) -> CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0_W {
        CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0_W { w: self }
    }
    #[doc = "Bits 2:3 - core_x_dram0_pms_constrain_sram_world_0_pms_1"]
    #[inline(always)]
    pub fn core_x_dram0_pms_constrain_sram_world_0_pms_1(
        &mut self,
    ) -> CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1_W {
        CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1_W { w: self }
    }
    #[doc = "Bits 4:5 - core_x_dram0_pms_constrain_sram_world_0_pms_2"]
    #[inline(always)]
    pub fn core_x_dram0_pms_constrain_sram_world_0_pms_2(
        &mut self,
    ) -> CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2_W {
        CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2_W { w: self }
    }
    #[doc = "Bits 6:7 - core_x_dram0_pms_constrain_sram_world_0_pms_3"]
    #[inline(always)]
    pub fn core_x_dram0_pms_constrain_sram_world_0_pms_3(
        &mut self,
    ) -> CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3_W {
        CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3_W { w: self }
    }
    #[doc = "Bits 12:13 - core_x_dram0_pms_constrain_sram_world_1_pms_0"]
    #[inline(always)]
    pub fn core_x_dram0_pms_constrain_sram_world_1_pms_0(
        &mut self,
    ) -> CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_0_W {
        CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_0_W { w: self }
    }
    #[doc = "Bits 14:15 - core_x_dram0_pms_constrain_sram_world_1_pms_1"]
    #[inline(always)]
    pub fn core_x_dram0_pms_constrain_sram_world_1_pms_1(
        &mut self,
    ) -> CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_1_W {
        CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_1_W { w: self }
    }
    #[doc = "Bits 16:17 - core_x_dram0_pms_constrain_sram_world_1_pms_2"]
    #[inline(always)]
    pub fn core_x_dram0_pms_constrain_sram_world_1_pms_2(
        &mut self,
    ) -> CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_2_W {
        CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_2_W { w: self }
    }
    #[doc = "Bits 18:19 - core_x_dram0_pms_constrain_sram_world_1_pms_3"]
    #[inline(always)]
    pub fn core_x_dram0_pms_constrain_sram_world_1_pms_3(
        &mut self,
    ) -> CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_3_W {
        CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_3_W { w: self }
    }
    #[doc = "Bits 24:25 - core_x_dram0_pms_constrain_rom_world_0_pms"]
    #[inline(always)]
    pub fn core_x_dram0_pms_constrain_rom_world_0_pms(
        &mut self,
    ) -> CORE_X_DRAM0_PMS_CONSTRAIN_ROM_WORLD_0_PMS_W {
        CORE_X_DRAM0_PMS_CONSTRAIN_ROM_WORLD_0_PMS_W { w: self }
    }
    #[doc = "Bits 26:27 - core_x_dram0_pms_constrain_rom_world_1_pms"]
    #[inline(always)]
    pub fn core_x_dram0_pms_constrain_rom_world_1_pms(
        &mut self,
    ) -> CORE_X_DRAM0_PMS_CONSTRAIN_ROM_WORLD_1_PMS_W {
        CORE_X_DRAM0_PMS_CONSTRAIN_ROM_WORLD_1_PMS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SENSITIVE_CORE_X_DRAM0_PMS_CONSTRAIN_1_REG\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_x_dram0_pms_constrain_1]
(index.html) module"]
pub struct CORE_X_DRAM0_PMS_CONSTRAIN_1_SPEC;
impl crate::RegisterSpec for CORE_X_DRAM0_PMS_CONSTRAIN_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_x_dram0_pms_constrain_1::R]
(R) reader structure"]
impl crate::Readable for CORE_X_DRAM0_PMS_CONSTRAIN_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_x_dram0_pms_constrain_1::W]
(W) writer structure"]
impl crate::Writable for CORE_X_DRAM0_PMS_CONSTRAIN_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CORE_X_DRAM0_PMS_CONSTRAIN_1 to value 0x0f0f_f0ff"]
impl crate::Resettable for CORE_X_DRAM0_PMS_CONSTRAIN_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f0f_f0ff
    }
}
