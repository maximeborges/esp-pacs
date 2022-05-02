#[doc = "Register `SRAM_CMD` reader"]
pub struct R(crate::R<SRAM_CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRAM_CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRAM_CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRAM_CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRAM_CMD` writer"]
pub struct W(crate::W<SRAM_CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRAM_CMD_SPEC>;
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
impl From<crate::W<SRAM_CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRAM_CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCLK_MODE` reader - SPI_CLK mode bits when SPI0 accesses to Ext_RAM. 0: SPI_CLK is off when CS inactive 1: SPI_CLK is delayed one cycle after CS inactive 2: SPI_CLK is delayed two cycles after CS inactive 3: SPI_CLK is always on."]
pub struct SCLK_MODE_R(crate::FieldReader<u8>);
impl SCLK_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SCLK_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCLK_MODE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCLK_MODE` writer - SPI_CLK mode bits when SPI0 accesses to Ext_RAM. 0: SPI_CLK is off when CS inactive 1: SPI_CLK is delayed one cycle after CS inactive 2: SPI_CLK is delayed two cycles after CS inactive 3: SPI_CLK is always on."]
pub struct SCLK_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLK_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u32 & 3);
        self.w
    }
}
#[doc = "Field `SWB_MODE` reader - Mode bits when SPI0 accesses to Ext_RAM."]
pub struct SWB_MODE_R(crate::FieldReader<u8>);
impl SWB_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SWB_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWB_MODE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWB_MODE` writer - Mode bits when SPI0 accesses to Ext_RAM."]
pub struct SWB_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SWB_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 2)) | ((value as u32 & 0xff) << 2);
        self.w
    }
}
#[doc = "Field `SDIN_DUAL` reader - When SPI0 accesses to Ext_RAM, set this bit to enable 2-bm in DIN phase."]
pub struct SDIN_DUAL_R(crate::FieldReader<bool>);
impl SDIN_DUAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDIN_DUAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIN_DUAL_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIN_DUAL` writer - When SPI0 accesses to Ext_RAM, set this bit to enable 2-bm in DIN phase."]
pub struct SDIN_DUAL_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIN_DUAL_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 10)) | ((value as u32 & 1) << 10);
        self.w
    }
}
#[doc = "Field `SDOUT_DUAL` reader - When SPI0 accesses to Ext_RAM, set this bit to enable 2-bm in DOUT phase."]
pub struct SDOUT_DUAL_R(crate::FieldReader<bool>);
impl SDOUT_DUAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDOUT_DUAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDOUT_DUAL_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDOUT_DUAL` writer - When SPI0 accesses to Ext_RAM, set this bit to enable 2-bm in DOUT phase."]
pub struct SDOUT_DUAL_W<'a> {
    w: &'a mut W,
}
impl<'a> SDOUT_DUAL_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 11)) | ((value as u32 & 1) << 11);
        self.w
    }
}
#[doc = "Field `SADDR_DUAL` reader - When SPI0 accesses to Ext_RAM, set this bit to enable 2-bm in ADDR phase."]
pub struct SADDR_DUAL_R(crate::FieldReader<bool>);
impl SADDR_DUAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SADDR_DUAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SADDR_DUAL_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SADDR_DUAL` writer - When SPI0 accesses to Ext_RAM, set this bit to enable 2-bm in ADDR phase."]
pub struct SADDR_DUAL_W<'a> {
    w: &'a mut W,
}
impl<'a> SADDR_DUAL_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 12)) | ((value as u32 & 1) << 12);
        self.w
    }
}
#[doc = "Field `SCMD_DUAL` reader - When SPI0 accesses to Ext_RAM, set this bit to enable 2-bm in CMD phase."]
pub struct SCMD_DUAL_R(crate::FieldReader<bool>);
impl SCMD_DUAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCMD_DUAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCMD_DUAL_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCMD_DUAL` writer - When SPI0 accesses to Ext_RAM, set this bit to enable 2-bm in CMD phase."]
pub struct SCMD_DUAL_W<'a> {
    w: &'a mut W,
}
impl<'a> SCMD_DUAL_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 13)) | ((value as u32 & 1) << 13);
        self.w
    }
}
#[doc = "Field `SDIN_QUAD` reader - When SPI0 accesses to Ext_RAM, set this bit to enable 4-bm in DIN phase."]
pub struct SDIN_QUAD_R(crate::FieldReader<bool>);
impl SDIN_QUAD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDIN_QUAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIN_QUAD_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIN_QUAD` writer - When SPI0 accesses to Ext_RAM, set this bit to enable 4-bm in DIN phase."]
pub struct SDIN_QUAD_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIN_QUAD_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 14)) | ((value as u32 & 1) << 14);
        self.w
    }
}
#[doc = "Field `SDOUT_QUAD` reader - When SPI0 accesses to Ext_RAM, set this bit to enable 4-bm in DOUT phase."]
pub struct SDOUT_QUAD_R(crate::FieldReader<bool>);
impl SDOUT_QUAD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDOUT_QUAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDOUT_QUAD_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDOUT_QUAD` writer - When SPI0 accesses to Ext_RAM, set this bit to enable 4-bm in DOUT phase."]
pub struct SDOUT_QUAD_W<'a> {
    w: &'a mut W,
}
impl<'a> SDOUT_QUAD_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 15)) | ((value as u32 & 1) << 15);
        self.w
    }
}
#[doc = "Field `SADDR_QUAD` reader - When SPI0 accesses to Ext_RAM, set this bit to enable 4-bm in ADDR phase."]
pub struct SADDR_QUAD_R(crate::FieldReader<bool>);
impl SADDR_QUAD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SADDR_QUAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SADDR_QUAD_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SADDR_QUAD` writer - When SPI0 accesses to Ext_RAM, set this bit to enable 4-bm in ADDR phase."]
pub struct SADDR_QUAD_W<'a> {
    w: &'a mut W,
}
impl<'a> SADDR_QUAD_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 16)) | ((value as u32 & 1) << 16);
        self.w
    }
}
#[doc = "Field `SCMD_QUAD` reader - When SPI0 accesses to Ext_RAM, set this bit to enable 4-bm in CMD phase."]
pub struct SCMD_QUAD_R(crate::FieldReader<bool>);
impl SCMD_QUAD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCMD_QUAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCMD_QUAD_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCMD_QUAD` writer - When SPI0 accesses to Ext_RAM, set this bit to enable 4-bm in CMD phase."]
pub struct SCMD_QUAD_W<'a> {
    w: &'a mut W,
}
impl<'a> SCMD_QUAD_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 17)) | ((value as u32 & 1) << 17);
        self.w
    }
}
#[doc = "Field `SDIN_OCT` reader - When SPI0 accesses to Ext_RAM, set this bit to enable 8-bm in DIN phase."]
pub struct SDIN_OCT_R(crate::FieldReader<bool>);
impl SDIN_OCT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDIN_OCT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIN_OCT_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIN_OCT` writer - When SPI0 accesses to Ext_RAM, set this bit to enable 8-bm in DIN phase."]
pub struct SDIN_OCT_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIN_OCT_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 18)) | ((value as u32 & 1) << 18);
        self.w
    }
}
#[doc = "Field `SDOUT_OCT` reader - When SPI0 accesses to Ext_RAM, set this bit to enable 8-bm in DOUT phase."]
pub struct SDOUT_OCT_R(crate::FieldReader<bool>);
impl SDOUT_OCT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDOUT_OCT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDOUT_OCT_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDOUT_OCT` writer - When SPI0 accesses to Ext_RAM, set this bit to enable 8-bm in DOUT phase."]
pub struct SDOUT_OCT_W<'a> {
    w: &'a mut W,
}
impl<'a> SDOUT_OCT_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 19)) | ((value as u32 & 1) << 19);
        self.w
    }
}
#[doc = "Field `SADDR_OCT` reader - When SPI0 accesses to Ext_RAM, set this bit to enable 8-bm in ADDR phase."]
pub struct SADDR_OCT_R(crate::FieldReader<bool>);
impl SADDR_OCT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SADDR_OCT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SADDR_OCT_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SADDR_OCT` writer - When SPI0 accesses to Ext_RAM, set this bit to enable 8-bm in ADDR phase."]
pub struct SADDR_OCT_W<'a> {
    w: &'a mut W,
}
impl<'a> SADDR_OCT_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 20)) | ((value as u32 & 1) << 20);
        self.w
    }
}
#[doc = "Field `SCMD_OCT` reader - When SPI0 accesses to Ext_RAM, set this bit to enable 8-bm in CMD phase."]
pub struct SCMD_OCT_R(crate::FieldReader<bool>);
impl SCMD_OCT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCMD_OCT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCMD_OCT_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCMD_OCT` writer - When SPI0 accesses to Ext_RAM, set this bit to enable 8-bm in CMD phase."]
pub struct SCMD_OCT_W<'a> {
    w: &'a mut W,
}
impl<'a> SCMD_OCT_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 21)) | ((value as u32 & 1) << 21);
        self.w
    }
}
#[doc = "Field `SDUMMY_OUT` reader - When SPI0 accesses to Ext_RAM, in the DUMMY phase the signal level of SPI bus is output by the SPI0 controller."]
pub struct SDUMMY_OUT_R(crate::FieldReader<bool>);
impl SDUMMY_OUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDUMMY_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDUMMY_OUT_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDUMMY_OUT` writer - When SPI0 accesses to Ext_RAM, in the DUMMY phase the signal level of SPI bus is output by the SPI0 controller."]
pub struct SDUMMY_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> SDUMMY_OUT_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 22)) | ((value as u32 & 1) << 22);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - SPI_CLK mode bits when SPI0 accesses to Ext_RAM. 0: SPI_CLK is off when CS inactive 1: SPI_CLK is delayed one cycle after CS inactive 2: SPI_CLK is delayed two cycles after CS inactive 3: SPI_CLK is always on."]
    #[inline(always)]
    pub fn sclk_mode(&self) -> SCLK_MODE_R {
        SCLK_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:9 - Mode bits when SPI0 accesses to Ext_RAM."]
    #[inline(always)]
    pub fn swb_mode(&self) -> SWB_MODE_R {
        SWB_MODE_R::new(((self.bits >> 2) & 0xff) as u8)
    }
    #[doc = "Bit 10 - When SPI0 accesses to Ext_RAM, set this bit to enable 2-bm in DIN phase."]
    #[inline(always)]
    pub fn sdin_dual(&self) -> SDIN_DUAL_R {
        SDIN_DUAL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - When SPI0 accesses to Ext_RAM, set this bit to enable 2-bm in DOUT phase."]
    #[inline(always)]
    pub fn sdout_dual(&self) -> SDOUT_DUAL_R {
        SDOUT_DUAL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - When SPI0 accesses to Ext_RAM, set this bit to enable 2-bm in ADDR phase."]
    #[inline(always)]
    pub fn saddr_dual(&self) -> SADDR_DUAL_R {
        SADDR_DUAL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - When SPI0 accesses to Ext_RAM, set this bit to enable 2-bm in CMD phase."]
    #[inline(always)]
    pub fn scmd_dual(&self) -> SCMD_DUAL_R {
        SCMD_DUAL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - When SPI0 accesses to Ext_RAM, set this bit to enable 4-bm in DIN phase."]
    #[inline(always)]
    pub fn sdin_quad(&self) -> SDIN_QUAD_R {
        SDIN_QUAD_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - When SPI0 accesses to Ext_RAM, set this bit to enable 4-bm in DOUT phase."]
    #[inline(always)]
    pub fn sdout_quad(&self) -> SDOUT_QUAD_R {
        SDOUT_QUAD_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - When SPI0 accesses to Ext_RAM, set this bit to enable 4-bm in ADDR phase."]
    #[inline(always)]
    pub fn saddr_quad(&self) -> SADDR_QUAD_R {
        SADDR_QUAD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - When SPI0 accesses to Ext_RAM, set this bit to enable 4-bm in CMD phase."]
    #[inline(always)]
    pub fn scmd_quad(&self) -> SCMD_QUAD_R {
        SCMD_QUAD_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - When SPI0 accesses to Ext_RAM, set this bit to enable 8-bm in DIN phase."]
    #[inline(always)]
    pub fn sdin_oct(&self) -> SDIN_OCT_R {
        SDIN_OCT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - When SPI0 accesses to Ext_RAM, set this bit to enable 8-bm in DOUT phase."]
    #[inline(always)]
    pub fn sdout_oct(&self) -> SDOUT_OCT_R {
        SDOUT_OCT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - When SPI0 accesses to Ext_RAM, set this bit to enable 8-bm in ADDR phase."]
    #[inline(always)]
    pub fn saddr_oct(&self) -> SADDR_OCT_R {
        SADDR_OCT_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - When SPI0 accesses to Ext_RAM, set this bit to enable 8-bm in CMD phase."]
    #[inline(always)]
    pub fn scmd_oct(&self) -> SCMD_OCT_R {
        SCMD_OCT_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - When SPI0 accesses to Ext_RAM, in the DUMMY phase the signal level of SPI bus is output by the SPI0 controller."]
    #[inline(always)]
    pub fn sdummy_out(&self) -> SDUMMY_OUT_R {
        SDUMMY_OUT_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - SPI_CLK mode bits when SPI0 accesses to Ext_RAM. 0: SPI_CLK is off when CS inactive 1: SPI_CLK is delayed one cycle after CS inactive 2: SPI_CLK is delayed two cycles after CS inactive 3: SPI_CLK is always on."]
    #[inline(always)]
    pub fn sclk_mode(&mut self) -> SCLK_MODE_W {
        SCLK_MODE_W { w: self }
    }
    #[doc = "Bits 2:9 - Mode bits when SPI0 accesses to Ext_RAM."]
    #[inline(always)]
    pub fn swb_mode(&mut self) -> SWB_MODE_W {
        SWB_MODE_W { w: self }
    }
    #[doc = "Bit 10 - When SPI0 accesses to Ext_RAM, set this bit to enable 2-bm in DIN phase."]
    #[inline(always)]
    pub fn sdin_dual(&mut self) -> SDIN_DUAL_W {
        SDIN_DUAL_W { w: self }
    }
    #[doc = "Bit 11 - When SPI0 accesses to Ext_RAM, set this bit to enable 2-bm in DOUT phase."]
    #[inline(always)]
    pub fn sdout_dual(&mut self) -> SDOUT_DUAL_W {
        SDOUT_DUAL_W { w: self }
    }
    #[doc = "Bit 12 - When SPI0 accesses to Ext_RAM, set this bit to enable 2-bm in ADDR phase."]
    #[inline(always)]
    pub fn saddr_dual(&mut self) -> SADDR_DUAL_W {
        SADDR_DUAL_W { w: self }
    }
    #[doc = "Bit 13 - When SPI0 accesses to Ext_RAM, set this bit to enable 2-bm in CMD phase."]
    #[inline(always)]
    pub fn scmd_dual(&mut self) -> SCMD_DUAL_W {
        SCMD_DUAL_W { w: self }
    }
    #[doc = "Bit 14 - When SPI0 accesses to Ext_RAM, set this bit to enable 4-bm in DIN phase."]
    #[inline(always)]
    pub fn sdin_quad(&mut self) -> SDIN_QUAD_W {
        SDIN_QUAD_W { w: self }
    }
    #[doc = "Bit 15 - When SPI0 accesses to Ext_RAM, set this bit to enable 4-bm in DOUT phase."]
    #[inline(always)]
    pub fn sdout_quad(&mut self) -> SDOUT_QUAD_W {
        SDOUT_QUAD_W { w: self }
    }
    #[doc = "Bit 16 - When SPI0 accesses to Ext_RAM, set this bit to enable 4-bm in ADDR phase."]
    #[inline(always)]
    pub fn saddr_quad(&mut self) -> SADDR_QUAD_W {
        SADDR_QUAD_W { w: self }
    }
    #[doc = "Bit 17 - When SPI0 accesses to Ext_RAM, set this bit to enable 4-bm in CMD phase."]
    #[inline(always)]
    pub fn scmd_quad(&mut self) -> SCMD_QUAD_W {
        SCMD_QUAD_W { w: self }
    }
    #[doc = "Bit 18 - When SPI0 accesses to Ext_RAM, set this bit to enable 8-bm in DIN phase."]
    #[inline(always)]
    pub fn sdin_oct(&mut self) -> SDIN_OCT_W {
        SDIN_OCT_W { w: self }
    }
    #[doc = "Bit 19 - When SPI0 accesses to Ext_RAM, set this bit to enable 8-bm in DOUT phase."]
    #[inline(always)]
    pub fn sdout_oct(&mut self) -> SDOUT_OCT_W {
        SDOUT_OCT_W { w: self }
    }
    #[doc = "Bit 20 - When SPI0 accesses to Ext_RAM, set this bit to enable 8-bm in ADDR phase."]
    #[inline(always)]
    pub fn saddr_oct(&mut self) -> SADDR_OCT_W {
        SADDR_OCT_W { w: self }
    }
    #[doc = "Bit 21 - When SPI0 accesses to Ext_RAM, set this bit to enable 8-bm in CMD phase."]
    #[inline(always)]
    pub fn scmd_oct(&mut self) -> SCMD_OCT_W {
        SCMD_OCT_W { w: self }
    }
    #[doc = "Bit 22 - When SPI0 accesses to Ext_RAM, in the DUMMY phase the signal level of SPI bus is output by the SPI0 controller."]
    #[inline(always)]
    pub fn sdummy_out(&mut self) -> SDUMMY_OUT_W {
        SDUMMY_OUT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI0 external RAM mode control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sram_cmd](index.html) module"]
pub struct SRAM_CMD_SPEC;
impl crate::RegisterSpec for SRAM_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sram_cmd::R](R) reader structure"]
impl crate::Readable for SRAM_CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sram_cmd::W](W) writer structure"]
impl crate::Writable for SRAM_CMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRAM_CMD to value 0"]
impl crate::Resettable for SRAM_CMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
